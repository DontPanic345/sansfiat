// The placeholder art: every atlas cell drawn from the page's flat colours and shapes. `draw-atlas.mjs`
// renders these into `atlas.png`; paid art replaces the PNG and keeps the cells `layout()` names.

export const TERRAINS = ['sea', 'lake', 'coast', 'plains', 'forest', 'hills', 'mountain', 'desert', 'marsh'];
export const TERRAIN_COLOR = {
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
export const WIDTH = 1024;
/** The circumradius of a terrain cell's hex, in cell pixels (the cell is 2 × this tall). */
export const HEX_R = 74;
/** A terrain cell's size, in cell pixels: the hex sits centred in it. */
export const TERRAIN_CELL = { w: 128, h: 148 };
/** A fringe's strip along a hex edge: its depth from the edge, in cell px, before the noise. */
export const DEPTH = 16;
/** The strip's depth varies by up to this many px either way, along the edge (`noise`). */
export const AMP = 6;
/** The strip's mask falls from 1 to 0 over this many px past its depth. */
export const FEATHER = 4;
/** The strip's mask tapers to 0 over this many px at each end of the edge, so it stops short of the corners. */
export const TAPER = 12;
/** The bank's shore cells: an edge piece is 112 px square, a corner piece 48 px (its anchor is the vertex). */
export const SHORE = { edge: { w: 112, h: 112, ax: 56, ay: 56 }, corner: { w: 48, h: 48, ax: 24, ay: 24 } };
/** The kinds that bleed a fringe into a neighbour of a lower kind, highest first: a shared land edge draws the higher. */
export const FRINGE = ['desert', 'marsh', 'forest', 'hills', 'mountain', 'coast'];
/** The wood and marsh pieces: a cell's size, and its ground point (the anchor), in cell px. Build-2 draws them at a slot. */
export const TREE = {
  big: { w: 80, h: 100, ax: 40, ay: 95 },
  small: { w: 48, h: 64, ax: 24, ay: 60 },
  reed: { w: 48, h: 60, ax: 24, ay: 56 },
};
/** How many world units (a hex has circumradius 1) a 64 px icon cell spans. */
export const ICON_WORLD = { 'icon.festival': 2.5, 'icon.centre': 4, 'icon.hearth': 1.6, 'icon.pass': 1, 'icon.town': 1.1 };

const UNITS = ['camp', 'scout'];
/** A unit cell: a figure 96 px square, standing with its feet at the anchor (48, 92), which is the tile's ground. */
export const UNIT = { w: 96, h: 96, ax: 48, ay: 92 };
const ICONS = ['capital', 'festival', 'centre', 'hearth', 'pass'];
/** A mountain piece's cell: 224 × 144, its ground point (the anchor) at (112, 128). */
const MOUNTAIN = { w: 224, h: 144, ax: 112, ay: 128 };

// The fringe geometry, in a terrain cell: a pointy-top hex of circumradius HEX_R, its apothem A, and edge d
// facing the screen angle 60 d (EDGE_DIR's order: E, SE, SW, W, NW, NE).
const APOTHEM = (HEX_R * Math.sqrt(3)) / 2;
const EDGE_NORMAL = [0, 1, 2, 3, 4, 5].map((d) => [Math.cos((Math.PI / 3) * d), Math.sin((Math.PI / 3) * d)]);

/** A point's place for edge `d` of a terrain cell: `s` its depth in px inward from the edge, `t` its place along it
 *  (0 at the middle), or null when it is outside the hex or past the strip's reach (`DEPTH + AMP + FEATHER`). */
function stripAt(d, x, y) {
  const px = x - TERRAIN_CELL.w / 2, py = y - TERRAIN_CELL.h / 2;
  const u = px / APOTHEM, v = py / HEX_R;
  if (Math.abs(u) > 1 || Math.abs(v) + Math.abs(u) / 2 > 1) return null;
  const [nx, ny] = EDGE_NORMAL[d];
  const s = APOTHEM - (px * nx + py * ny);
  const t = -px * ny + py * nx;
  if (s < 0 || s > DEPTH + AMP + FEATHER || Math.abs(t) > HEX_R / 2) return null;
  return { s, t };
}

/** The fringe mask of edge `d`, variant `v`, at cell px (x, y) of a terrain cell: 1 inside the strip, 0 outside. */
export function fringeMask(d, v, x, y) {
  const p = stripAt(d, x, y);
  if (!p) return 0;
  // A smooth noise along the edge: two or three sines, phased by the edge and the variant, so the strip is ragged.
  const ph = 2.1 * d + 3.7 * v;
  const n = (Math.sin(p.t * 0.118 + ph) + 0.5 * Math.sin(p.t * 0.33 + 1.7 * ph + 1) + 0.25 * Math.sin(p.t * 0.9 + 2.3 * ph + 2)) / 1.75;
  const depth = DEPTH + AMP * n;
  const feather = Math.min(1, Math.max(0, 1 - (p.s - depth) / FEATHER));
  const taper = Math.min(1, (HEX_R / 2 - Math.abs(p.t)) / TAPER);
  return feather * taper;
}

const boxes = new Map();
/** The rectangle of a terrain cell that holds every pixel of edge `d`'s fringe: `{ x, y, w, h }`, whole px. */
export function fringeBox(d) {
  if (!boxes.has(d)) {
    let x0 = Infinity, y0 = Infinity, x1 = -1, y1 = -1;
    for (let y = 0; y < TERRAIN_CELL.h; y++) {
      for (let x = 0; x < TERRAIN_CELL.w; x++) {
        if (!stripAt(d, x + 0.5, y + 0.5)) continue;
        x0 = Math.min(x0, x);
        y0 = Math.min(y0, y);
        x1 = Math.max(x1, x);
        y1 = Math.max(y1, y);
      }
    }
    boxes.set(d, { x: x0, y: y0, w: x1 - x0 + 1, h: y1 - y0 + 1 });
  }
  return boxes.get(d);
}

/** Every cell with its place in the sheet, its size and a note: `{width, height, cells}`. */
export function layout() {
  const cells = {};
  let x = 0, y = 0, rowH = 0;
  const put = (name, w, h, note, anchor) => {
    if (x + w > WIDTH) {
      x = 0;
      y += rowH;
      rowH = 0;
    }
    cells[name] = { x, y, w, h, note, ...anchor };
    x += w;
    rowH = Math.max(rowH, h);
  };
  const row = () => {
    if (x > 0) {
      x = 0;
      y += rowH;
      rowH = 0;
    }
  };
  const hexNote = (what) => `${what}: pointy-top hex, circumradius ${HEX_R}, centred, transparent outside`;
  for (const t of TERRAINS) {
    for (let v = 0; v < 3; v++) put(`terrain.${t}.${v}`, 128, 148, hexNote(`${t}, variant ${v}`));
  }
  row();
  put('overlay.floodplain', 128, 148, hexNote('floodplain wash, translucent, drawn over the tile'));
  put('overlay.desert-dots', 128, 148, hexNote('desert stipple, translucent (no longer drawn: the desert ground is sand)'));
  put('overlay.fields', 128, 148, hexNote('field furrows: three short dark ochre strokes in the lower half, translucent, drawn over a tile with fields'));
  row();
  for (const u of UNITS) {
    const at = { ax: UNIT.ax, ay: UNIT.ay };
    put(`unit.${u}`, UNIT.w, UNIT.h, `${u} figure, standing, its feet at the anchor, transparent outside: cut from the actor study`, at);
    put(`unit.${u}.mask`, UNIT.w, UNIT.h, `${u} mask: its cloth in grey (the civ's colour shows through when it is tinted by multiply), transparent elsewhere, drawn over the figure`, at);
  }
  row();
  put('icon.capital', 64, 64, "capital: a house with a door, casing and white edge, transparent inside (the civ's colour shows through from the mask)");
  put('icon.capital.mask', 64, 64, "capital silhouette in opaque white: tinted in the civ's colour");
  put('icon.festival', 64, 64, 'festival: a white ring with a dark casing');
  put('icon.centre', 64, 64, 'centre: a gold ring with a dark casing, round the festival that is a centre');
  put('icon.hearth', 64, 64, 'hearth: a small yellow square with a dark edge');
  put('icon.pass', 64, 64, 'pass: a pale dot with a dark edge');
  put('icon.town', 64, 64, 'town: a white diamond with a dark casing, at the top of a festival tile that several clans live on');
  row();
  const piece = (name, note) => put(name, MOUNTAIN.w, MOUNTAIN.h, note, { ax: MOUNTAIN.ax, ay: MOUNTAIN.ay });
  for (let v = 0; v < 2; v++) piece(`mountain.lone.${v}`, `a lone mountain's peak, variant ${v}: its ground point at the anchor, transparent outside`);
  for (let v = 0; v < 3; v++) piece(`mountain.peak.${v}`, `a mountain's peak in a range, variant ${v}: its ground point at the anchor, transparent outside`);
  for (const dir of ['e', 'se', 'sw']) {
    for (let v = 0; v < 2; v++) piece(`mountain.ridge.${dir}.${v}`, `the ridge across a shared edge in direction ${dir}, variant ${v}: its midpoint at the anchor, transparent outside`);
  }
  row();
  for (let d = 0; d < 6; d++) {
    for (let v = 0; v < 2; v++) {
      put(`shore.edge.${d}.${v}`, SHORE.edge.w, SHORE.edge.h, `the bank across hex edge ${d} (EDGE_DIR order), variant ${v}, on the land side: its midpoint at the anchor, transparent outside`, { ax: SHORE.edge.ax, ay: SHORE.edge.ay });
    }
  }
  row();
  for (let c = 0; c < 6; c++) put(`shore.in.${c}`, SHORE.corner.w, SHORE.corner.h, `the bank at hex vertex ${c}, where water pokes into land: the bank fanned round the vertex, transparent outside`, { ax: SHORE.corner.ax, ay: SHORE.corner.ay });
  row();
  for (let c = 0; c < 6; c++) put(`shore.out.${c}`, SHORE.corner.w, SHORE.corner.h, `the bank at hex vertex ${c}, where land pokes into water: the bank of its two edges mitred at the vertex, transparent outside`, { ax: SHORE.corner.ax, ay: SHORE.corner.ay });
  row();
  for (const kind of FRINGE) {
    for (let d = 0; d < 6; d++) {
      const b = fringeBox(d);
      for (let v = 0; v < 2; v++) {
        put(`fringe.${kind}.${d}.${v}`, b.w, b.h, `${kind} along hex edge ${d}, variant ${v}: a ragged strip inside the tile, cut from its ${kind} cell, the hex centre at the anchor, transparent outside`, { ax: TERRAIN_CELL.w / 2 - b.x, ay: TERRAIN_CELL.h / 2 - b.y });
      }
    }
    row();
  }
  // The wood and the marsh: a tree stands on its trunk's foot, a reed clump on its base; each cell's ground point is its anchor.
  const at = (t) => ({ ax: TREE[t].ax, ay: TREE[t].ay });
  for (let v = 0; v < 3; v++) put(`tree.big.${v}`, TREE.big.w, TREE.big.h, `a full tree in a wood, variant ${v}: its trunk's foot is the ground point, at the anchor, transparent outside`, at('big'));
  row();
  for (let v = 0; v < 3; v++) put(`tree.small.${v}`, TREE.small.w, TREE.small.h, `a sapling or small tree at a wood's margin, variant ${v}: its trunk's foot is the ground point, at the anchor, transparent outside`, at('small'));
  row();
  for (let v = 0; v < 3; v++) put(`reed.${v}`, TREE.reed.w, TREE.reed.h, `a reed clump on a marsh, variant ${v}: its base is the ground point, at the anchor, transparent outside`, at('reed'));
  row();
  return { width: WIDTH, height: y, cells };
}

const corners = (cx, cy, r) =>
  [0, 1, 2, 3, 4, 5].map((i) => [cx + r * Math.cos((Math.PI / 3) * i - Math.PI / 6), cy + r * Math.sin((Math.PI / 3) * i - Math.PI / 6)]);

const hexPath = (ctx, w, h, r = HEX_R) => {
  ctx.beginPath();
  corners(w / 2, h / 2, r).forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
  ctx.closePath();
};

const SHIELD_S = 22; // half the shield's height, in cell px
const shield = (ctx, w, h) => {
  const s = SHIELD_S, x = w / 2, y = h / 2;
  ctx.beginPath();
  ctx.moveTo(x - s * 0.8, y - s);
  ctx.lineTo(x + s * 0.8, y - s);
  ctx.lineTo(x + s * 0.8, y + s * 0.1);
  ctx.lineTo(x, y + s);
  ctx.lineTo(x - s * 0.8, y + s * 0.1);
  ctx.closePath();
};

const HOUSE_S = 22;
const house = (ctx, w, h) => {
  const s = HOUSE_S, x = w / 2, y = h / 2 + s * 0.075;
  ctx.beginPath();
  ctx.moveTo(x - s, y + s * 0.8);
  ctx.lineTo(x - s, y - s * 0.1);
  ctx.lineTo(x, y - s * 0.95);
  ctx.lineTo(x + s, y - s * 0.1);
  ctx.lineTo(x + s, y + s * 0.8);
  ctx.closePath();
};

const ring = (ctx, w, h, r, colour, line, casing) => {
  for (const [col, lw] of [['rgba(0,0,0,0.7)', line + casing], [colour, line]]) {
    ctx.strokeStyle = col;
    ctx.lineWidth = lw;
    ctx.beginPath();
    ctx.arc(w / 2, h / 2, r, 0, 2 * Math.PI);
    ctx.stroke();
  }
};

/**
 * A mountain piece, standing on its anchor (the ground point): a peak is a tall triangle, a ridge two low
 * triangles along its direction (e: east, se: down to the right, sw: down to the left).
 */
const mountainPiece = (ctx, [, kind, dir]) => {
  const ax = MOUNTAIN.ax, ay = MOUNTAIN.ay;
  const triangles = [];
  if (kind === 'ridge') {
    const [ux, uy] = { e: [1, 0], se: [0.5, Math.sqrt(3) / 2], sw: [-0.5, Math.sqrt(3) / 2] }[dir];
    for (const s of [-1, 1]) {
      const bx = ax + s * 60 * ux, by = ay + s * 60 * uy;
      triangles.push([[bx, by], [ax, ay], [(bx + ax) / 2, (by + ay) / 2 - 50]]);
    }
  } else {
    triangles.push([[ax - 55, ay], [ax + 55, ay], [ax, ay - 90]]);
  }
  for (const tri of triangles) {
    ctx.beginPath();
    tri.forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
    ctx.closePath();
    ctx.fillStyle = TERRAIN_COLOR.mountain;
    ctx.fill();
    ctx.strokeStyle = '#3c3a46';
    ctx.lineWidth = 4;
    ctx.stroke();
  }
};

/** A tree placeholder standing on its anchor: a dark green cone on a brown trunk. */
const treePiece = (ctx, w, h, { ax, ay }) => {
  const trunkW = 0.1 * w, base = ay - 0.22 * h;
  ctx.fillStyle = '#5e4128';
  ctx.fillRect(ax - trunkW / 2, base, trunkW, ay - base);
  ctx.beginPath();
  ctx.moveTo(ax, 0.08 * h);
  ctx.lineTo(ax + 0.4 * w, base);
  ctx.lineTo(ax - 0.4 * w, base);
  ctx.closePath();
  ctx.fillStyle = '#2f6b3a';
  ctx.fill();
  ctx.strokeStyle = '#1d3d24';
  ctx.lineWidth = 2;
  ctx.stroke();
};

/** A reed placeholder standing on its anchor: a few upright straw strokes on a dark green base. */
const reedPiece = (ctx, w, h, { ax, ay }) => {
  ctx.strokeStyle = '#8c9a3a';
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  for (const [dx, lean, len] of [[-0.3, -0.05, 0.85], [-0.1, 0.04, 1], [0.08, -0.03, 0.9], [0.28, 0.06, 0.7]]) {
    const x = ax + dx * w;
    ctx.moveTo(x, ay);
    ctx.lineTo(x + lean * w, ay - len * h * 0.9);
  }
  ctx.stroke();
  ctx.fillStyle = '#3d5a2a';
  ctx.beginPath();
  ctx.ellipse(ax, ay - 1, 0.42 * w, 0.06 * h, 0, 0, 2 * Math.PI);
  ctx.fill();
};

/** Draws cell `name` into `ctx`, filling (0, 0, w, h); the cell is clear before. */
export function draw(name, ctx, w, h) {
  ctx.lineJoin = 'round';
  ctx.lineCap = 'round';
  const [kind, id] = name.split('.');
  if (kind === 'shore' || kind === 'fringe') return; // cut from the source: no placeholder
  if (kind === 'terrain') {
    ctx.fillStyle = TERRAIN_COLOR[id];
    hexPath(ctx, w, h, HEX_R * 1.0);
    ctx.fill();
    const u = HEX_R, cx = w / 2, cy = h / 2;
    if (id === 'mountain') {
      ctx.beginPath();
      ctx.moveTo(cx, cy - 0.62 * u);
      ctx.lineTo(cx + 0.58 * u, cy + 0.38 * u);
      ctx.lineTo(cx - 0.58 * u, cy + 0.38 * u);
      ctx.closePath();
      ctx.fillStyle = '#c4c2cc';
      ctx.fill();
      ctx.strokeStyle = '#3c3a46';
      ctx.lineWidth = 4;
      ctx.stroke();
    } else if (id === 'marsh') {
      ctx.strokeStyle = '#2f5a46';
      ctx.lineWidth = 5;
      ctx.beginPath();
      for (const [dx, dy] of [[-0.4, 0.2], [0, -0.05], [0.4, 0.25]]) {
        const x = cx + dx * u, y = cy + dy * u;
        ctx.moveTo(x, y + 0.3 * u);
        ctx.lineTo(x, y - 0.3 * u);
        ctx.moveTo(x - 0.12 * u, y - 0.12 * u);
        ctx.lineTo(x, y - 0.3 * u);
        ctx.lineTo(x + 0.12 * u, y - 0.12 * u);
      }
      ctx.stroke();
    }
  } else if (kind === 'mountain') {
    mountainPiece(ctx, name.split('.'));
  } else if (name === 'overlay.floodplain') {
    ctx.fillStyle = 'rgba(96,235,70,0.42)';
    hexPath(ctx, w, h, HEX_R * 1.02);
    ctx.fill();
  } else if (name === 'overlay.fields') {
    ctx.strokeStyle = '#7a5a1e';
    ctx.lineWidth = 6;
    ctx.lineCap = 'round';
    ctx.beginPath();
    for (const y of [0.18, 0.4, 0.62]) {
      ctx.moveTo(w / 2 - 0.5 * HEX_R, h / 2 + y * HEX_R);
      ctx.lineTo(w / 2 + 0.5 * HEX_R, h / 2 + y * HEX_R);
    }
    ctx.stroke();
  } else if (name === 'overlay.desert-dots') {
    ctx.fillStyle = '#b39a55';
    let s = 12345;
    const next = () => {
      s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
      return ((s >>> 8) % 1000) / 1000 - 0.5;
    };
    for (let d = 0; d < 7; d++) {
      const ax = next(), ay = next();
      if (Math.abs(ax) * 2 + Math.abs(ay) * 1.6 < 1.3) ctx.fillRect(w / 2 + ax * 1.4 * HEX_R - 3, h / 2 + ay * 1.4 * HEX_R - 3, 6, 6);
    }
  } else if (kind === 'tree') {
    treePiece(ctx, w, h, TREE[id]);
  } else if (kind === 'reed') {
    reedPiece(ctx, w, h, TREE.reed);
  } else if (kind === 'unit') {
    shield(ctx, w, h);
    if (name.endsWith('.mask')) {
      ctx.fillStyle = '#fff';
      ctx.fill();
    } else {
      ctx.strokeStyle = '#000';
      ctx.lineWidth = 5;
      ctx.stroke();
      ctx.strokeStyle = 'rgba(0,0,0,0.9)';
      ctx.lineWidth = 1.5;
      ctx.stroke();
    }
  } else if (name === 'icon.capital.mask') {
    house(ctx, w, h);
    ctx.fillStyle = '#fff';
    ctx.fill();
  } else if (name === 'icon.capital') {
    house(ctx, w, h);
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 5;
    ctx.stroke();
    ctx.strokeStyle = '#fff';
    ctx.lineWidth = 1.8;
    ctx.stroke();
    const s = HOUSE_S, x = w / 2, y = h / 2 + s * 0.075;
    ctx.fillStyle = 'rgba(0,0,0,0.75)';
    ctx.fillRect(x - s * 0.22, y + s * 0.1, s * 0.44, s * 0.7);
  } else if (name === 'icon.festival') {
    ring(ctx, w, h, 0.9 * (w / ICON_WORLD[name]), '#fff', 4, 4);
  } else if (name === 'icon.centre') {
    ring(ctx, w, h, 1.7 * (w / ICON_WORLD[name]), '#ffc800', 5, 4);
  } else if (name === 'icon.hearth') {
    const q = 0.4 * (w / ICON_WORLD[name]);
    ctx.fillStyle = '#ffd400';
    ctx.strokeStyle = '#3a2c00';
    ctx.lineWidth = 2;
    ctx.fillRect(w / 2 - q / 2, h / 2 - q / 2, q, q);
    ctx.strokeRect(w / 2 - q / 2, h / 2 - q / 2, q, q);
  } else if (name === 'icon.town') {
    const q = 0.55 * (w / ICON_WORLD[name]);
    ctx.beginPath();
    ctx.moveTo(w / 2, h / 2 - q);
    ctx.lineTo(w / 2 + q, h / 2);
    ctx.lineTo(w / 2, h / 2 + q);
    ctx.lineTo(w / 2 - q, h / 2);
    ctx.closePath();
    ctx.fillStyle = '#fff';
    ctx.strokeStyle = '#1a1a1a';
    ctx.lineWidth = 3;
    ctx.fill();
    ctx.stroke();
  } else if (name === 'icon.pass') {
    ctx.fillStyle = '#fff2b0';
    ctx.strokeStyle = '#3a2c00';
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.arc(w / 2, h / 2, 0.3 * (w / ICON_WORLD[name]), 0, 2 * Math.PI);
    ctx.fill();
    ctx.stroke();
  } else {
    throw new Error(`no placeholder for ${name}`);
  }
}
