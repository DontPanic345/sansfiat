// The map canvas: pan, zoom, picking, and the layer list. The static layers (terrain, rivers, closed
// country and the debug views of the land) draw into an offscreen layer redrawn only when the view or a
// toggle changes; the others, which a turn changes, are drawn over it, in `LAYERS`' order. World units: a
// hex has circumradius 1, so tile (q, r) is centred at (sqrt3 (q + r/2), 1.5 r).
import * as terrain from './layers/terrain.js';
import * as fertility from './layers/fertility.js';
import * as floodplains from './layers/floodplains.js';
import * as rivers from './layers/rivers.js';
import * as country from './layers/country.js';
import * as passes from './layers/passes.js';
import * as hiddenCosts from './layers/hidden-costs.js';
import * as territories from './layers/territories.js';
import * as worn from './layers/worn.js';
import * as trails from './layers/trails.js';
import * as hearths from './layers/hearths.js';
import * as festivalRoutes from './layers/festival-routes.js';
import * as festivals from './layers/festivals.js';
import * as land from './layers/land.js';
import * as held from './layers/held.js';
import * as lines from './layers/lines.js';
import * as clans from './layers/clans.js';
import * as leanings from './layers/leanings.js';
import * as centres from './layers/centres.js';
import * as capitals from './layers/capitals.js';
import * as units from './layers/units.js';
import * as routes from './layers/routes.js';
import * as overlay from './layers/overlay.js';

import { CORNER, EDGE_DIR } from './geom.js';

/** The layers, bottom to top. */
export const LAYERS = [terrain, fertility, floodplains, rivers, country, passes, hiddenCosts, territories, worn, trails, hearths, festivalRoutes, festivals, land, held, lines, clans, leanings, centres, capitals, units, routes, overlay];
/** The layers a View or Debug toggle leaves off. */
const OFF = new Set(['fertility', 'country', 'hidden-costs']);
/** A toggle that switches several layers together. */
const GROUPS = { civs: ['land', 'held', 'lines', 'capitals', 'units', 'routes'], markets: ['festivals', 'centres'] };

const SQRT3 = Math.sqrt(3);
const MAX_K = 35; // px per world unit: a tile is ~60 px wide
const BG = '#0b0e12';
export { hue, walkOf } from './geom.js';

export class MapView {
  /**
   * @param {HTMLCanvasElement} canvas
   * @param {{terrain: object[], rules: object, atlas: object, onPick(i: number): void, onCommand?(i: number): void, onPaint?(i: number): void, onHover(i: number, x: number, y: number): void, onHoverEnd(): void}} opts
   */
  constructor(canvas, opts) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.layer = document.createElement('canvas');
    this.lctx = this.layer.getContext('2d');
    this.opts = opts;
    this.terrain = opts.terrain;
    this.atlas = opts.atlas;
    this.on = Object.fromEntries(LAYERS.map((l) => [l.name, !OFF.has(l.name)])); // each layer on or off, by name
    this.model = null;
    this.routes = []; // [{unit, tiles: [from, ...path], selected}]: the player's units' routes
    this.line = null; // {tiles, colour, cut?}: the selected camp's line home (to where it is cut, `cut` the tile), or null
    this.painting = false; // in paint mode a left press or drag paints tiles (onPaint) instead of picking or panning
    this.focus = null; // the selected clan's id: its territory and route are drawn bright, the others' dim
    this.homeLine = null; // the led civ's camp's line home as tile lists, drawn faint at all times
    this.plan = null; // {known, possible}: the selected scout's route as tile lists from its start, or null
    this.revealed = new Uint8Array(0); // 1 where the led civ has revealed the tile
    this.work = []; // [tile, labour, need]: trails under way
    this.focusMarket = null; // the selected festival's id: its walkers' routes are drawn bright
    this.cx = 0; this.cy = 0; this.k = 8;
    this.w = 1; this.h = 1; this.dpr = 1;
    this.n = 0;
    this.selected = -1;
    this.selectedUnit = -1;
    this.highlight = null;
    this.dirty = true;
    this.frame = 0;
    this.anim = null;
    this.stats = { terrainMs: 0, overlayMs: 0, atlas: false, cells: 0 };
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
    this.claim = new Int32Array(n);
    this.worn = new Float64Array(n);
    this.model = null;
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

  /** Takes the view model and works out what the layers index by tile: claims, wear, revealed tiles,
   *  hearths and the centres' sites. */
  setModel(model) {
    const v = model.v;
    this.model = model;
    const n = this.n;
    this.claim.fill(0);
    this.worn.fill(0);
    for (const [i, c] of v.claims) this.claim[i] = c;
    this.revealed = new Uint8Array(n);
    for (const i of v.revealed) this.revealed[i] = 1;
    this.work = v.trail_work;
    // A tile's wear is its most worn food's share.
    for (const [i, worn] of v.wear) this.worn[i] = Math.max(...worn);
    const tameTurns = this.opts.rules.tame_turns;
    const tame = new Map(v.tame);
    this.hearths = [];
    for (let i = 0; i < n; i++) {
      if (this.l.wild_grain[i] > 0 && (tame.get(i) || 0) >= tameTurns) this.hearths.push(i);
    }
    // The centres' sites: the end's markets, or before it the centre clans' homes.
    this.centres = v.end
      ? v.end.centres.map((id) => v.markets[id]?.tile).filter((t) => t != null)
      : v.summary.centres.map((id) => v.clans[id]?.home).filter((t) => t != null);
    this.shapes = null;
  }

  /** Adds to `path` the edges of tile `i` that face a neighbour `same` is false for. */
  outline(path, i, same) {
    for (let d = 0; d < 6; d++) {
      const j = this.nb[i * 6 + d];
      if (j >= 0 && same(j)) continue;
      const e = (d + 1) % 6;
      path.moveTo(this.px[i] + CORNER[d][0], this.py[i] + CORNER[d][1]);
      path.lineTo(this.px[i] + CORNER[e][0], this.py[i] + CORNER[e][1]);
    }
  }

  /** The civ marks that change only with a turn, built when first drawn after one: each civ's border, the
   *  solid one round its held land and the dashed one round its paper (claimed, not held) against the
   *  ground its claim doesn't cover. */
  civShapes() {
    if (this.shapes) return this.shapes;
    this.shapes = [];
    for (const c of Object.values(this.model.v.civs || {})) {
      const landSet = new Set(c.land);
      const claim = new Set(c.claim);
      const solid = new Path2D();
      const dashed = new Path2D();
      for (const i of c.land) this.outline(solid, i, (j) => landSet.has(j));
      for (const i of c.claim) if (!landSet.has(i)) this.outline(dashed, i, (j) => claim.has(j));
      this.shapes.push({ civ: c, solid, dashed, held: c.land.length, paper: c.claim.length - c.land.length });
    }
    return this.shapes;
  }

  /** A civ's border as the page draws it: its held tiles and its paper, counted; null for a civ not
   *  in the view. */
  border(civ) {
    const s = this.civShapes().find((x) => x.civ.id === civ);
    return s ? { held: s.held, paper: s.paper } : null;
  }

  /** The routes to draw: `[{unit, tiles, selected}]`, the tiles from where the unit stands to its destination. */
  setRoutes(routes) {
    this.routes = routes;
    this.requestDraw();
  }

  /** The clan whose territory holds tile `i`, or null. */
  clanAt(i) {
    return this.claim[i] || null;
  }

  /** Switches the layer `name` (or a group: `civs`, `markets`) on or off. */
  setLayer(name, on) {
    const names = GROUPS[name] ?? [name];
    for (const n of names) {
      if (!(n in this.on)) throw new Error(`no layer ${n}`);
      this.on[n] = on;
      if (LAYERS.find((l) => l.name === n).static) this.dirty = true;
    }
    if (GROUPS[name]) this.on[name] = on;
    this.requestDraw();
  }

  /** Paint mode on or off: while on, a left press and drag call `onPaint` for each tile they cross. */
  setPainting(on) {
    this.painting = on;
    this.canvas.style.cursor = on ? 'crosshair' : '';
  }

  /** The clan to show bright (its territory and festival route), the rest dimmed; null for none. */
  setFocus(clanId) {
    this.focus = clanId ?? null;
    this.requestDraw();
  }

  /** The festival whose walkers (the clans going to it) are drawn bright, the rest dimmed; null for none. */
  setFocusMarket(id) {
    this.focusMarket = id ?? null;
    this.requestDraw();
  }

  /** The led civ's camp's line home as tiles from the camp, drawn thin and faint; null for none. */
  setHomeLine(tiles) {
    this.homeLine = tiles;
    this.requestDraw();
  }

  /** The selected camp's line home: `{tiles: [its tile, ...path], colour}`, or null. */
  setLine(line) {
    this.line = line;
    this.requestDraw();
  }

  /** The selected scout's route: `{known, possible}`, each the tiles from its start, or null for none. */
  setPlan(plan) {
    this.plan = plan;
    this.requestDraw();
  }

  setSelection(i, unit = -1) {
    this.selected = i;
    this.selectedUnit = unit;
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

  /** The client coordinates of tile `i`'s centre, for headless scripts that click on it. */
  screenOf(i) {
    const rect = this.canvas.getBoundingClientRect();
    return {
      x: rect.left + this.w / 2 + (this.px[i] - this.cx) * this.k,
      y: rect.top + this.h / 2 + (this.py[i] - this.cy) * this.k,
    };
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
      if (this.painting && e.button === 0 && this.pointers.size === 1) {
        const rect = c.getBoundingClientRect();
        const i = this.tileAt(e.clientX - rect.left, e.clientY - rect.top);
        this.drag.paint = i;
        if (i >= 0) this.opts.onPaint?.(i);
      }
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
      if (this.drag.paint !== undefined) {
        const i = this.tileAt(e.clientX - rect.left, e.clientY - rect.top);
        if (i >= 0 && i !== this.drag.paint) this.opts.onPaint?.(i);
        this.drag.paint = i;
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
        if (this.drag.moved <= 4 && e.type === 'pointerup' && this.drag.paint === undefined) {
          const rect = c.getBoundingClientRect();
          const i = this.tileAt(e.clientX - rect.left, e.clientY - rect.top);
          // Left picks, right commands; other buttons do neither.
          if (this.drag.button === 0) this.opts.onPick(i);
          else if (this.drag.button === 2) this.opts.onCommand?.(i);
        }
        this.drag = null;
      } else {
        this.drag.pinch = null;
      }
    };
    c.addEventListener('contextmenu', (e) => e.preventDefault());
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
      this.drawStatic();
      this.stats.terrainMs = performance.now() - t0;
      this.dirty = false;
    }
    const t1 = performance.now();
    const ctx = this.ctx;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.drawImage(this.layer, 0, 0);
    this.setWorld(ctx);
    if (this.model) {
      ctx.lineJoin = 'round';
      ctx.lineCap = 'round';
      for (const l of LAYERS) if (!l.static && this.on[l.name]) l.draw(ctx, this.model, this);
    }
    this.stats.overlayMs = performance.now() - t1;
  }

  /** The static layers, drawn into the offscreen layer. */
  drawStatic() {
    const ctx = this.lctx;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = BG;
    ctx.fillRect(0, 0, this.layer.width, this.layer.height);
    this.setWorld(ctx);
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';
    for (const l of LAYERS) {
      if (!l.static || !this.on[l.name]) continue;
      ctx.save();
      l.draw(ctx, this.model, this);
      ctx.restore();
      this.setWorld(ctx);
    }
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

  /** Whether tile `i` is within 80 px of the canvas. */
  onScreen(i) {
    const x = (this.px[i] - this.cx) * this.k + this.w / 2, y = (this.py[i] - this.cy) * this.k + this.h / 2;
    return x > -80 && x < this.w + 80 && y > -80 && y < this.h + 80;
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
}
