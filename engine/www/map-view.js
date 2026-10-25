// The map canvas: pan, zoom, picking, and the drawing. Terrain, rivers and closed country go to an
// offscreen layer redrawn only when the view or a toggle changes; what a turn changes (territory,
// rounds, markets, clans) is drawn over it. World units: a hex has circumradius 1, so tile (q, r)
// is centred at (sqrt3 (q + r/2), 1.5 r).

const SQRT3 = Math.sqrt(3);
const MAX_K = 35; // px per world unit: a tile is ~60 px wide
const TERRAIN_COLOR = {
  sea: '#1f4f7a',
  lake: '#3f8fc4',
  coast: '#d6c487',
  plains: '#9cbf5a',
  forest: '#3f7a3a',
  hills: '#8a7a5c',
  mountain: '#7c7a85',
  desert: '#efe3b8',
  marsh: '#5f8a6a',
};
const ZONE_COLOR = [null, '#f0c030', '#d9663f', '#b070e0']; // valley, sealed, neck
const BG = '#0b0e12';
/** The six hex corners (pointy top) and, for the edge from corner k to k + 1, the neighbour across it. */
const CORNER = [0, 1, 2, 3, 4, 5].map((i) => [Math.cos((Math.PI / 3) * i - Math.PI / 6), Math.sin((Math.PI / 3) * i - Math.PI / 6)]);
const EDGE_DIR = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];

/** A colour from a clan's id. */
export const hue = (id) => `hsl(${(id * 137.508) % 360} 70% 55%)`;

const fertilityColor = (f) => {
  const y = Math.max(0, Math.min(1, (f - 0.4) / 1.6));
  return `hsl(${30 + 90 * y} 60% ${25 + 30 * y}%)`;
};
const FERT_BUCKETS = 16;
const fertBucket = (f) => Math.max(0, Math.min(FERT_BUCKETS - 1, Math.floor(((f - 0.4) / 1.6) * FERT_BUCKETS)));

export class MapView {
  /**
   * @param {HTMLCanvasElement} canvas
   * @param {{terrain: object[], rules: object, onPick(i: number): void, onHover(i: number, x: number, y: number): void, onHoverEnd(): void}} opts
   */
  constructor(canvas, opts) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.layer = document.createElement('canvas');
    this.lctx = this.layer.getContext('2d');
    this.opts = opts;
    this.terrain = opts.terrain;
    this.options = { fertility: false, territory: true, rounds: true, markets: true, hidden: false };
    this.cx = 0; this.cy = 0; this.k = 8;
    this.w = 1; this.h = 1; this.dpr = 1;
    this.n = 0;
    this.selected = -1;
    this.highlight = null;
    this.dirty = true;
    this.frame = 0;
    this.anim = null;
    this.stats = { terrainMs: 0, overlayMs: 0 };
    this.pointers = new Map();
    this.drag = null;
    this.attach();
  }

  // ---- layout ----------------------------------------------------------------------------

  /** Takes the static columns from `Sim.layout()` and builds what picking and drawing use. */
  setLayout(l) {
    this.l = l;
    const n = (this.n = l.q.length);
    this.radius = l.radius;
    this.px = new Float64Array(n);
    this.py = new Float64Array(n);
    const side = 2 * l.radius + 1;
    this.side = side;
    this.grid = new Int32Array(side * side).fill(-1);
    for (let i = 0; i < n; i++) {
      this.px[i] = SQRT3 * (l.q[i] + l.r[i] / 2);
      this.py[i] = 1.5 * l.r[i];
      this.grid[(l.q[i] + l.radius) * side + (l.r[i] + l.radius)] = i;
    }
    this.nb = new Int32Array(n * 6).fill(-1);
    for (let i = 0; i < n; i++) {
      for (let d = 0; d < 6; d++) this.nb[i * 6 + d] = this.at(l.q[i] + EDGE_DIR[d][0], l.r[i] + EDGE_DIR[d][1]);
    }
    this.land = new Uint8Array(n);
    for (let i = 0; i < n; i++) this.land[i] = this.terrain[l.terrain[i]].land ? 1 : 0;
    this.names = this.terrain.map((t) => t.name);
    this.byName = Object.fromEntries(this.names.map((nm, i) => [nm, i]));
    this.vis = new Int32Array(n);
    this.nvis = 0;
    this.owner = new Int32Array(n);
    this.claim = new Int32Array(n);
    this.reach = new Uint8Array(n);
    this.v = null;
    this.selected = -1;
    this.highlight = null;
    this.dirty = true;
    this.resize();
    this.fit();
  }

  /** The tile at axial (q, r), or -1. */
  at(q, r) {
    const R = this.radius;
    if (q < -R || q > R || r < -R || r > R) return -1;
    return this.grid[(q + R) * this.side + (r + R)];
  }

  /** Takes `Sim.view()` (already parsed) and works out territory, claims, hearths and reach. */
  setDynamic(v) {
    this.v = v;
    const n = this.n;
    this.owner.fill(0);
    this.claim.fill(0);
    this.reach.fill(0);
    for (const [i, c] of v.claims) this.claim[i] = c;
    for (const c of Object.values(v.clans)) {
      for (const i of c.round) {
        if (this.owner[i] === 0) this.owner[i] = c.id;
        for (let d = 0; d < 6; d++) {
          const j = this.nb[i * 6 + d];
          if (j >= 0 && this.land[j] && this.owner[j] === 0) this.owner[j] = c.id;
        }
      }
    }
    const tameTurns = this.opts.rules.tame_turns;
    const tame = new Map(v.tame);
    this.hearths = [];
    for (let i = 0; i < n; i++) {
      if (this.l.wild_grain[i] > 0 && (tame.get(i) || 0) >= tameTurns) this.hearths.push(i);
    }
    this.centres = new Set(v.end ? v.end.centres : v.summary.centres);
    // A market's reach, approximated by hex steps over land (as the page always has).
    for (const m of Object.values(v.markets)) {
      const q0 = this.l.q[m.tile], r0 = this.l.r[m.tile];
      for (let dq = -3; dq <= 3; dq++) {
        for (let dr = Math.max(-3, -dq - 3); dr <= Math.min(3, -dq + 3); dr++) {
          const j = this.at(q0 + dq, r0 + dr);
          if (j >= 0 && this.land[j]) this.reach[j] = 1;
        }
      }
    }
  }

  /** The clan holding tile `i` through its round, or its territory, as `[clan, how]`. */
  holder(i) {
    if (this.claim[i]) return [this.claim[i], 'round'];
    if (this.owner[i]) return [this.owner[i], 'territory'];
    return null;
  }

  setOptions(o) {
    const terrainChanged = o.fertility !== this.options.fertility || o.hidden !== this.options.hidden;
    Object.assign(this.options, o);
    if (terrainChanged) this.dirty = true;
    this.requestDraw();
  }

  setSelection(i) {
    this.selected = i;
    this.requestDraw();
  }

  setHighlight(h) {
    this.highlight = h;
    this.requestDraw();
  }

  // ---- view ------------------------------------------------------------------------------

  /** Size of the world's bounding box for a map of radius `R`, in world units. */
  static extent(R) {
    return [SQRT3 * (2 * R + 1) + 0.6, 3 * R + 2 + 0.6];
  }

  fitK(R) {
    const [ww, hh] = MapView.extent(R);
    return Math.min(this.w / ww, this.h / hh);
  }

  get minK() {
    return Math.min(this.fitK(33), this.fitK(this.radius || 33));
  }

  fit() {
    this.anim = null;
    this.cx = 0;
    this.cy = 0;
    this.k = Math.min(MAX_K, Math.max(this.minK, this.fitK(this.radius)));
    this.viewChanged();
  }

  clampView() {
    const [ww, hh] = MapView.extent(this.radius);
    const mx = ww / 2, my = hh / 2;
    this.cx = Math.max(-mx, Math.min(mx, this.cx));
    this.cy = Math.max(-my, Math.min(my, this.cy));
    this.k = Math.max(this.minK, Math.min(MAX_K, this.k));
  }

  viewChanged() {
    this.clampView();
    this.dirty = true;
    this.requestDraw();
    this.opts.onView?.();
  }

  panBy(dx, dy) {
    this.anim = null;
    this.cx -= dx / this.k;
    this.cy -= dy / this.k;
    this.viewChanged();
  }

  /** Zooms by `f` about the screen point (x, y) (css px in the canvas), default the centre. */
  zoomBy(f, x = this.w / 2, y = this.h / 2) {
    this.anim = null;
    const k = Math.max(this.minK, Math.min(MAX_K, this.k * f));
    const wx = this.cx + (x - this.w / 2) / this.k;
    const wy = this.cy + (y - this.h / 2) / this.k;
    this.k = k;
    this.cx = wx - (x - this.w / 2) / k;
    this.cy = wy - (y - this.h / 2) / k;
    this.viewChanged();
  }

  /** Centres on tile `i`, zooming in to at least `minK`; eases over a third of a second when `smooth`. */
  centreOn(i, { minK = 0, smooth = true } = {}) {
    const to = { cx: this.px[i], cy: this.py[i], k: Math.max(this.k, Math.min(MAX_K, minK)) };
    if (!smooth) {
      Object.assign(this, to);
      this.anim = null;
      this.viewChanged();
      return;
    }
    this.anim = { from: { cx: this.cx, cy: this.cy, k: this.k }, to, t0: performance.now() };
    this.requestDraw();
  }

  stepAnim() {
    if (!this.anim) return;
    const { from, to, t0 } = this.anim;
    const t = Math.min(1, (performance.now() - t0) / 350);
    const e = t * t * (3 - 2 * t);
    this.cx = from.cx + (to.cx - from.cx) * e;
    this.cy = from.cy + (to.cy - from.cy) * e;
    // Zoom geometrically so it feels even.
    this.k = from.k * Math.pow(to.k / from.k, e);
    if (t >= 1) this.anim = null;
    this.clampView();
    this.dirty = true;
    this.opts.onView?.();
    if (this.anim) this.requestDraw();
  }

  resize() {
    const box = this.canvas.parentElement.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    const w = Math.max(1, Math.floor(box.width)), h = Math.max(1, Math.floor(box.height));
    if (w === this.w && h === this.h && dpr === this.dpr && this.canvas.width) return;
    const grew = this.n > 0;
    const oldFit = this.n ? this.fitK(this.radius) : 0;
    const wasFit = grew && Math.abs(this.k - oldFit) < 1e-6;
    this.w = w; this.h = h; this.dpr = dpr;
    this.canvas.width = this.layer.width = Math.floor(w * dpr);
    this.canvas.height = this.layer.height = Math.floor(h * dpr);
    this.canvas.style.width = `${w}px`;
    this.canvas.style.height = `${h}px`;
    if (this.n && wasFit) this.fit();
    else if (this.n) this.clampView();
    this.dirty = true;
    this.drawNow(); // resizing clears the canvas: draw before the next paint
  }

  /** The tile under the css-pixel point (x, y) of the canvas, or -1. */
  tileAt(x, y) {
    const wx = this.cx + (x - this.w / 2) / this.k;
    const wy = this.cy + (y - this.h / 2) / this.k;
    const fr = wy / 1.5;
    const fq = wx / SQRT3 - fr / 2;
    // Cube rounding.
    let q = Math.round(fq), r = Math.round(fr), s = Math.round(-fq - fr);
    const dq = Math.abs(q - fq), dr = Math.abs(r - fr), ds = Math.abs(s + fq + fr);
    if (dq > dr && dq > ds) q = -r - s;
    else if (dr > ds) r = -q - s;
    return this.at(q, r);
  }

  // ---- input -----------------------------------------------------------------------------

  attach() {
    const c = this.canvas;
    c.style.touchAction = 'none';
    c.addEventListener('pointerdown', (e) => {
      c.setPointerCapture(e.pointerId);
      this.pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
      this.opts.onHoverEnd();
      this.drag = { moved: 0, pinch: this.pointers.size > 1 ? this.pinchState() : null, button: e.button };
    });
    c.addEventListener('pointermove', (e) => {
      const p = this.pointers.get(e.pointerId);
      const rect = c.getBoundingClientRect();
      if (!p) {
        const i = this.tileAt(e.clientX - rect.left, e.clientY - rect.top);
        this.opts.onHover(i, e.clientX, e.clientY);
        return;
      }
      const dx = e.clientX - p.x, dy = e.clientY - p.y;
      p.x = e.clientX;
      p.y = e.clientY;
      if (this.pointers.size >= 2) {
        const st = this.pinchState();
        if (this.drag.pinch) {
          this.zoomBy(st.d / this.drag.pinch.d, st.x - rect.left, st.y - rect.top);
          this.panBy(st.x - this.drag.pinch.x, st.y - this.drag.pinch.y);
        }
        this.drag.pinch = st;
        this.drag.moved += 100;
        return;
      }
      this.drag.moved += Math.abs(dx) + Math.abs(dy);
      if (this.drag.moved > 4) {
        this.opts.onHoverEnd();
        this.panBy(dx, dy);
      }
    });
    const up = (e) => {
      const was = this.pointers.delete(e.pointerId);
      if (!was || !this.drag) return;
      if (this.pointers.size === 0) {
        if (this.drag.moved <= 4 && e.type === 'pointerup') {
          const rect = c.getBoundingClientRect();
          this.opts.onPick(this.tileAt(e.clientX - rect.left, e.clientY - rect.top));
        }
        this.drag = null;
      } else {
        this.drag.pinch = null;
      }
    };
    c.addEventListener('pointerup', up);
    c.addEventListener('pointercancel', up);
    c.addEventListener('pointerleave', (e) => {
      if (!this.pointers.has(e.pointerId)) this.opts.onHoverEnd();
    });
    c.addEventListener(
      'wheel',
      (e) => {
        e.preventDefault();
        this.opts.onHoverEnd();
        const rect = c.getBoundingClientRect();
        const unit = e.deltaMode === 1 ? 16 : e.deltaMode === 2 ? 400 : 1;
        this.zoomBy(Math.exp(-e.deltaY * unit * 0.0015), e.clientX - rect.left, e.clientY - rect.top);
      },
      { passive: false },
    );
  }

  pinchState() {
    const [a, b] = [...this.pointers.values()];
    return { d: Math.max(1, Math.hypot(a.x - b.x, a.y - b.y)), x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 };
  }

  // ---- drawing ---------------------------------------------------------------------------

  requestDraw() {
    if (this.frame) return;
    this.frame = requestAnimationFrame(() => {
      this.frame = 0;
      this.drawNow();
    });
  }

  drawNow() {
    if (!this.n) return;
    if (this.frame) {
      cancelAnimationFrame(this.frame);
      this.frame = 0;
    }
    this.stepAnim();
    this.collectVisible();
    if (this.dirty) {
      const t0 = performance.now();
      this.drawTerrain();
      this.stats.terrainMs = performance.now() - t0;
      this.dirty = false;
    }
    const t1 = performance.now();
    const ctx = this.ctx;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.drawImage(this.layer, 0, 0);
    this.setWorld(ctx);
    this.drawOverlay(ctx);
    this.stats.overlayMs = performance.now() - t1;
  }

  setWorld(ctx) {
    const s = this.k * this.dpr;
    ctx.setTransform(s, 0, 0, s, (this.w / 2 - this.cx * this.k) * this.dpr, (this.h / 2 - this.cy * this.k) * this.dpr);
  }

  collectVisible() {
    const m = 1.3;
    const x0 = this.cx - this.w / 2 / this.k - m, x1 = this.cx + this.w / 2 / this.k + m;
    const y0 = this.cy - this.h / 2 / this.k - m, y1 = this.cy + this.h / 2 / this.k + m;
    let c = 0;
    for (let i = 0; i < this.n; i++) {
      const x = this.px[i], y = this.py[i];
      if (x >= x0 && x <= x1 && y >= y0 && y <= y1) this.vis[c++] = i;
    }
    this.nvis = c;
  }

  hexAt(ctx, i, r = 1) {
    const x = this.px[i], y = this.py[i];
    ctx.moveTo(x + CORNER[0][0] * r, y + CORNER[0][1] * r);
    for (let c = 1; c < 6; c++) ctx.lineTo(x + CORNER[c][0] * r, y + CORNER[c][1] * r);
    ctx.closePath();
  }

  /** Strokes, in one path, the edges of the tiles `inSet` that face a tile that isn't `same` as them. */
  edges(ctx, inSet, same) {
    ctx.beginPath();
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      if (!inSet(i)) continue;
      const x = this.px[i], y = this.py[i];
      for (let d = 0; d < 6; d++) {
        const j = this.nb[i * 6 + d];
        if (j >= 0 && same(i, j)) continue;
        const e = (d + 1) % 6;
        ctx.moveTo(x + CORNER[d][0], y + CORNER[d][1]);
        ctx.lineTo(x + CORNER[e][0], y + CORNER[e][1]);
      }
    }
    ctx.stroke();
  }

  drawTerrain() {
    const ctx = this.lctx;
    const l = this.l;
    const k = this.k;
    const px = 1 / k; // one device-independent pixel in world units
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = BG;
    ctx.fillRect(0, 0, this.layer.width, this.layer.height);
    this.setWorld(ctx);
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';

    // Fill, one path per colour.
    const fert = this.options.fertility;
    const buckets = new Map();
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      const t = l.terrain[i];
      let key;
      if (fert && this.land[i]) key = `f${fertBucket(l.fertility[i])}`;
      else key = t === 0 && l.lake[i] ? 'lake' : this.names[t];
      let b = buckets.get(key);
      if (!b) buckets.set(key, (b = []));
      b.push(i);
    }
    for (const [key, list] of buckets) {
      ctx.fillStyle = /^f\d+$/.test(key) ? fertilityColor(0.4 + ((Number(key.slice(1)) + 0.5) / FERT_BUCKETS) * 1.6) : TERRAIN_COLOR[key] || '#f0f';
      ctx.beginPath();
      for (const i of list) this.hexAt(ctx, i, 1.03);
      ctx.fill();
    }
    if (k >= 14) {
      ctx.strokeStyle = 'rgba(0,0,0,0.22)';
      ctx.lineWidth = px;
      ctx.beginPath();
      for (let a = 0; a < this.nvis; a++) this.hexAt(ctx, this.vis[a], 1);
      ctx.stroke();
    }

    // Floodplains: a bright green wash with a thin darker border (the delta's marsh is its own colour).
    const marsh = this.byName.marsh;
    ctx.fillStyle = 'rgba(96,235,70,0.42)';
    ctx.beginPath();
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      if (l.flood[i] && l.terrain[i] !== marsh) this.hexAt(ctx, i, 1.02);
    }
    ctx.fill();
    ctx.strokeStyle = 'rgba(28,110,24,0.9)';
    ctx.lineWidth = Math.max(px * 1.2, 0.05);
    this.edges(ctx, (i) => l.flood[i] === 1, (i, j) => l.flood[j] === 1);

    this.drawMarks(ctx);
    this.drawRivers(ctx);

    // Closed country: an outline round each zone's outer edge.
    for (let z = 1; z <= 3; z++) {
      ctx.strokeStyle = ZONE_COLOR[z];
      ctx.lineWidth = Math.max(px * 2.2, 0.13);
      ctx.setLineDash(z === 2 ? [0.5, 0.3] : []);
      ctx.lineCap = z === 2 ? 'butt' : 'round';
      this.edges(ctx, (i) => l.zone[i] === z, (i, j) => l.zone[j] === z);
    }
    ctx.setLineDash([]);
    ctx.lineCap = 'round';

    // Passes: a pale notch on the ridge.
    if (k >= 5) {
      ctx.fillStyle = '#fff2b0';
      ctx.strokeStyle = '#3a2c00';
      ctx.lineWidth = Math.max(px, 0.05);
      ctx.beginPath();
      for (let a = 0; a < this.nvis; a++) {
        const i = this.vis[a];
        if (l.pass[i]) {
          ctx.moveTo(this.px[i] + 0.3, this.py[i]);
          ctx.arc(this.px[i], this.py[i], 0.3, 0, 2 * Math.PI);
        }
      }
      ctx.fill();
      ctx.stroke();
    }

    if (this.options.hidden) this.drawHidden(ctx);
  }

  /** Peaks on mountains, stipple on desert, reeds on marsh. */
  drawMarks(ctx) {
    const l = this.l;
    const k = this.k;
    const px = 1 / k;
    const { mountain, desert, marsh } = this.byName;
    if (k >= 6) {
      ctx.beginPath();
      for (let a = 0; a < this.nvis; a++) {
        const i = this.vis[a];
        if (l.terrain[i] !== mountain) continue;
        const x = this.px[i], y = this.py[i];
        ctx.moveTo(x, y - 0.62);
        ctx.lineTo(x + 0.58, y + 0.38);
        ctx.lineTo(x - 0.58, y + 0.38);
        ctx.closePath();
      }
      ctx.fillStyle = '#c4c2cc';
      ctx.fill();
      ctx.strokeStyle = '#3c3a46';
      ctx.lineWidth = Math.max(px, 0.06);
      ctx.stroke();
    }
    if (k >= 9) {
      ctx.fillStyle = '#b39a55';
      ctx.beginPath();
      for (let a = 0; a < this.nvis; a++) {
        const i = this.vis[a];
        if (l.terrain[i] !== desert) continue;
        let s = (i * 2654435761) >>> 0;
        for (let d = 0; d < 7; d++) {
          s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
          const ax = ((s >>> 8) % 1000) / 1000 - 0.5;
          s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
          const ay = ((s >>> 8) % 1000) / 1000 - 0.5;
          if (Math.abs(ax) * 2 + Math.abs(ay) * 1.6 < 1.3) ctx.rect(this.px[i] + ax * 1.4, this.py[i] + ay * 1.4, Math.max(px * 1.4, 0.06), Math.max(px * 1.4, 0.06));
        }
      }
      ctx.fill();
      ctx.strokeStyle = '#2f5a46';
      ctx.lineWidth = Math.max(px * 1.2, 0.07);
      ctx.beginPath();
      for (let a = 0; a < this.nvis; a++) {
        const i = this.vis[a];
        if (l.terrain[i] !== marsh) continue;
        const x = this.px[i], y = this.py[i];
        for (const [dx, dy] of [[-0.4, 0.2], [0, -0.05], [0.4, 0.25]]) {
          ctx.moveTo(x + dx, y + dy + 0.3);
          ctx.lineTo(x + dx, y + dy - 0.3);
          ctx.moveTo(x + dx - 0.12, y + dy - 0.12);
          ctx.lineTo(x + dx, y + dy - 0.3);
          ctx.lineTo(x + dx + 0.12, y + dy - 0.12);
        }
      }
      ctx.stroke();
    }
  }

  /** A line from each river tile's centre to its flow tile's centre; wider downstream. */
  drawRivers(ctx) {
    const l = this.l;
    const k = this.k;
    const px = 1 / k;
    const classes = [[], [], [], [], [], [], []];
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      if (l.river[i] > 0 && l.flow[i] >= 0) classes[Math.min(6, Math.floor(Math.sqrt(l.river[i])))].push(i);
    }
    const width = (c) => Math.max(px * 1.3, 0.14 + 0.06 * c);
    for (const pass of [0, 1]) {
      if (pass === 0 && k < 9) continue;
      ctx.strokeStyle = pass === 0 ? '#1d5f96' : '#58b8f2';
      classes.forEach((list, c) => {
        if (!list.length) return;
        ctx.lineWidth = width(c) + (pass === 0 ? px * 1.6 : 0);
        ctx.beginPath();
        for (const i of list) {
          const j = l.flow[i];
          const x = this.px[i], y = this.py[i];
          // Into the sea a little past the shore, so a mouth opens into the water.
          const f = l.terrain[j] === 0 ? 0.62 : 1;
          ctx.moveTo(x, y);
          ctx.lineTo(x + (this.px[j] - x) * f, y + (this.py[j] - y) * f);
        }
        ctx.stroke();
      });
    }
  }

  /** The hidden-costs view: trail cost / span on forest, hills and mountain, bridgeable mountains tinted. */
  drawHidden(ctx) {
    const l = this.l;
    const { mountain } = this.byName;
    ctx.fillStyle = 'rgba(255,140,0,0.5)';
    ctx.beginPath();
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      if (l.terrain[i] === mountain && l.span[i] != null) this.hexAt(ctx, i, 1);
    }
    ctx.fill();
    if (this.k < 13) return;
    ctx.font = '0.55px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    for (let a = 0; a < this.nvis; a++) {
      const i = this.vis[a];
      const c = l.trail_cost[i];
      if (c == null) continue;
      const t = l.terrain[i];
      if (t !== mountain && this.names[t] !== 'forest' && this.names[t] !== 'hills') continue;
      const text = l.span[i] != null ? `${c}/${l.span[i]}` : `${c}`;
      ctx.fillStyle = 'rgba(0,0,0,0.65)';
      ctx.fillText(text, this.px[i] + 0.03, this.py[i] + 0.82 * 0.5 + 0.03);
      ctx.fillStyle = '#fff';
      ctx.fillText(text, this.px[i], this.py[i] + 0.82 * 0.5);
    }
  }

  drawOverlay(ctx) {
    const v = this.v;
    if (!v) return;
    const k = this.k;
    const px = 1 / k;
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';
    const onScreen = (i) => {
      const x = (this.px[i] - this.cx) * k + this.w / 2, y = (this.py[i] - this.cy) * k + this.h / 2;
      return x > -80 && x < this.w + 80 && y > -80 && y < this.h + 80;
    };

    // Territory: a faint fill in the clan's colour, a firmer line where it meets other ground.
    if (this.options.territory) {
      const byClan = new Map();
      for (let a = 0; a < this.nvis; a++) {
        const i = this.vis[a];
        const o = this.owner[i];
        if (!o) continue;
        let list = byClan.get(o);
        if (!list) byClan.set(o, (list = []));
        list.push(i);
      }
      ctx.globalAlpha = 0.3;
      for (const [id, list] of byClan) {
        ctx.fillStyle = hue(id);
        ctx.beginPath();
        for (const i of list) this.hexAt(ctx, i, 1.02);
        ctx.fill();
      }
      ctx.globalAlpha = 0.85;
      ctx.lineWidth = Math.max(px * 1.2, 0.04);
      for (const [id, list] of byClan) {
        ctx.strokeStyle = hue(id);
        ctx.beginPath();
        for (const i of list) {
          for (let d = 0; d < 6; d++) {
            const j = this.nb[i * 6 + d];
            if (j >= 0 && this.owner[j] === id) continue;
            const e = (d + 1) % 6;
            ctx.moveTo(this.px[i] + CORNER[d][0], this.py[i] + CORNER[d][1]);
            ctx.lineTo(this.px[i] + CORNER[e][0], this.py[i] + CORNER[e][1]);
          }
        }
        ctx.stroke();
      }
      ctx.globalAlpha = 1;
    }

    // Trails: a short line across the tile.
    const trailW = Math.max(px * 1.6, 0.14);
    for (const [col, extra] of [['rgba(60,40,10,0.6)', px * 1.6], ['#f2dca4', 0]]) {
      ctx.strokeStyle = col;
      ctx.lineWidth = trailW + extra;
      ctx.beginPath();
      for (const i of v.trails) {
        if (!onScreen(i)) continue;
        ctx.moveTo(this.px[i] - 0.45, this.py[i] + 0.22);
        ctx.lineTo(this.px[i] + 0.45, this.py[i] - 0.22);
      }
      ctx.stroke();
    }

    // Hearths: a small yellow square.
    ctx.fillStyle = '#ffd400';
    ctx.strokeStyle = '#3a2c00';
    ctx.lineWidth = Math.max(px, 0.05);
    ctx.beginPath();
    for (const i of this.hearths) {
      if (onScreen(i)) ctx.rect(this.px[i] - 0.2, this.py[i] - 0.2, 0.4, 0.4);
    }
    ctx.fill();
    ctx.stroke();

    // Rounds: a closed line through the tiles' centres, back to home.
    if (this.options.rounds) {
      const clans = Object.values(v.clans).filter((c) => c.round.length >= 2);
      for (const casing of [true, false]) {
        ctx.lineWidth = casing ? Math.max(px * 3.4, 0.2) : Math.max(px * 1.8, 0.12);
        for (const c of clans) {
          ctx.strokeStyle = casing ? 'rgba(0,0,0,0.5)' : hue(c.id);
          ctx.beginPath();
          c.round.forEach((i, n) => {
            if (n === 0) ctx.moveTo(this.px[i], this.py[i]);
            else ctx.lineTo(this.px[i], this.py[i]);
          });
          ctx.closePath();
          ctx.stroke();
        }
      }
    }

    // Markets: a ring with the reach outlined; centres picked out.
    if (this.options.markets) {
      ctx.strokeStyle = 'rgba(255,255,255,0.55)';
      ctx.lineWidth = Math.max(px, 0.05);
      this.edges(ctx, (i) => this.reach[i] === 1, (i, j) => this.reach[j] === 1);
      for (const m of Object.values(v.markets)) {
        const x = this.px[m.tile], y = this.py[m.tile];
        if (!onScreen(m.tile)) continue;
        for (const [col, extra] of [['rgba(0,0,0,0.6)', px * 2], ['#fff', 0]]) {
          ctx.strokeStyle = col;
          ctx.lineWidth = Math.max(px * 1.6, 0.15) + extra;
          ctx.beginPath();
          ctx.arc(x, y, 0.9, 0, 2 * Math.PI);
          ctx.stroke();
        }
      }
      ctx.strokeStyle = '#ffc800';
      ctx.lineWidth = Math.max(px * 3.2, 0.3);
      ctx.beginPath();
      for (const id of this.centres) {
        const m = v.markets[id];
        if (!m || !onScreen(m.tile)) continue;
        ctx.moveTo(this.px[m.tile] + 1.5, this.py[m.tile]);
        ctx.arc(this.px[m.tile], this.py[m.tile], 1.5, 0, 2 * Math.PI);
      }
      ctx.stroke();
    }

    // Clans: a circle at home, its area by people; a red ring when starving.
    for (const c of Object.values(v.clans)) {
      if (!onScreen(c.home)) continue;
      const x = this.px[c.home], y = this.py[c.home];
      const r = Math.min(1.4, Math.max(px * 2.2, 0.22 * Math.sqrt(c.pop)));
      ctx.beginPath();
      ctx.arc(x, y, r, 0, 2 * Math.PI);
      ctx.fillStyle = hue(c.id);
      ctx.fill();
      ctx.lineWidth = Math.max(px, 0.04);
      ctx.strokeStyle = 'rgba(0,0,0,0.75)';
      ctx.stroke();
      if (c.starving) {
        ctx.beginPath();
        ctx.arc(x, y, r + px * 1.6, 0, 2 * Math.PI);
        ctx.lineWidth = Math.max(px * 2.2, 0.1);
        ctx.strokeStyle = '#ff2a2a';
        ctx.stroke();
      }
    }

    // What an event or the panel points at.
    const hl = this.highlight;
    if (hl) {
      let tile = -1;
      if (hl.clan != null && v.clans[hl.clan]) {
        const c = v.clans[hl.clan];
        tile = c.home;
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = Math.max(px * 2.6, 0.2);
        ctx.beginPath();
        c.round.forEach((i, n) => (n === 0 ? ctx.moveTo(this.px[i], this.py[i]) : ctx.lineTo(this.px[i], this.py[i])));
        ctx.closePath();
        ctx.stroke();
      } else if (hl.market != null && v.markets[hl.market]) {
        tile = v.markets[hl.market].tile;
      }
      if (tile >= 0) {
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = Math.max(px * 3, 0.25);
        ctx.beginPath();
        ctx.arc(this.px[tile], this.py[tile], 2.1, 0, 2 * Math.PI);
        ctx.stroke();
        ctx.strokeStyle = '#000';
        ctx.lineWidth = Math.max(px, 0.06);
        ctx.beginPath();
        ctx.arc(this.px[tile], this.py[tile], 2.1 + Math.max(px * 1.6, 0.12), 0, 2 * Math.PI);
        ctx.stroke();
      }
    }

    if (this.selected >= 0) {
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = Math.max(px * 2.5, 0.12);
      ctx.beginPath();
      this.hexAt(ctx, this.selected, 1);
      ctx.stroke();
      ctx.strokeStyle = '#000';
      ctx.lineWidth = Math.max(px, 0.04);
      ctx.beginPath();
      this.hexAt(ctx, this.selected, 1 + Math.max(px * 1.6, 0.06));
      ctx.stroke();
    }
  }
}
