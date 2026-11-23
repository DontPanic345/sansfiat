// The wood pieces of a map: a tree on each slot of a forest tile and a reed on each slot of a marsh tile, kept or
// left clear by the rules below. Pure (no DOM): the layer draws them, and www/check-map.mjs and www/play/woods.mjs
// read them. Directions are `EDGE_DIR`'s: 0 E, 1 SE, 2 SW, 3 W, 4 NW, 5 NE.
//
// The rules, in world units (a hex has circumradius 1, apothem √3/2):
// - A tile's slots are a triangular lattice through its centre, kept where every edge is at least `inset` away (a reed's
//   inset is negative: reeds stand a little past the edge).
//   Each slot moves by up to `jitter` in x and in y; the piece stands on the moved point.
// - An edge is open when the neighbour across it is not forest (or is off the map). A forest slot's depth is its
//   distance to the nearest open edge. A slot nearer than `band` is sparse: it holds a small tree when its draw is
//   under `sparse`, else nothing. Any other slot holds a big tree. A marsh slot holds a reed.
// - A slot within `river` of the tile's river course is clear, and so is a slot in the field clearing when the tile
//   has fields this turn.
// - The pieces are sorted by y, then x, then tile, then slot, so a tree in front covers one behind, across tiles too.

/** The woods' numbers, each in world units unless it says otherwise. The one table build-3 tunes. */
export const WOODS = {
  /** The lattice's spacing: a tree tile's (`tree`) and a reed tile's (`reed`). */
  gap: { tree: 0.56, reed: 0.6 },
  /** How far a slot keeps from every edge of its hex: the lattice's margin. Reeds are negative: they stand past the hex's
   *  edge, so a marsh's rim on the water and on land is ragged, not a straight hex edge. */
  inset: { tree: 0.06, reed: -0.05 },
  /** How far a slot may move, in x and in y: the pieces don't stand in rows. */
  jitter: 0.14,
  /** A forest slot nearer than this to an open edge is in the band: sparse, and small trees only. */
  band: 0.34,
  /** The share of a sparse slot's draws that hold a small tree. */
  sparse: 0.6,
  /** A slot nearer than this to a river's course is clear, so the river reads across the wood. */
  river: 0.22,
  /** The field clearing, relative to the tile's centre: |x| < `x`, and `top` < y < `bottom` (where fields draw). */
  clearing: { x: 0.6, top: 0.0, bottom: 0.8 },
  /** The woods draw once a tile's cell is this many device px high (below it, the ground alone). Device px. */
  from: 24,
};

/** The salts of the draws: each is its own draw, so the moves, the sparse test and the variant are independent. */
export const SALT = { jx: 1, jy: 2, sparse: 3, variant: 4 };

const APOTHEM = Math.sqrt(3) / 2;
const ROW = Math.sqrt(3) / 2;
/** The outward normal of each edge: edge d faces the neighbour in EDGE_DIR[d], at 60·d degrees. */
const NORMALS = [0, 1, 2, 3, 4, 5].map((d) => [Math.cos((Math.PI / 3) * d), Math.sin((Math.PI / 3) * d)]);

/** A draw in [0, 1) from a tile, a slot and a salt: a pure integer hash, so a map always looks the same. */
export function slotDraw(tile, slot, salt) {
  let h = Math.imul(tile + 1, 0x9e3779b1) ^ Math.imul(slot + 1, 0x85ebca77) ^ Math.imul(salt + 1, 0xc2b2ae3d);
  h ^= h >>> 15;
  h = Math.imul(h, 0x2c1b3c6d);
  h ^= h >>> 12;
  h = Math.imul(h, 0x297a2d39);
  h ^= h >>> 15;
  return (h >>> 0) / 4294967296;
}

/** Whether (x, y), relative to a hex's centre, is at least `margin` inside all six edges. */
const inside = (x, y, margin) => NORMALS.every(([nx, ny]) => APOTHEM - (x * nx + y * ny) >= margin);

/** The lattice points of a triangular lattice through a hex's centre, spaced `gap` (rows `gap·√3/2` apart, every other
 *  row offset by `gap/2`), kept where every edge is at least `inset` away (`WOODS.inset`'s tree or reed). Sorted by y, then x: a slot's index is
 *  its place in this list, the same for every tile. */
export function slotsOf(gap, inset) {
  const out = [];
  const rows = Math.ceil(1 / (gap * ROW)) + 1;
  const cols = Math.ceil(1 / gap) + 2;
  for (let r = -rows; r <= rows; r++) {
    for (let c = -cols; c <= cols; c++) {
      const x = (c + ((r & 1) ? 0.5 : 0)) * gap;
      const y = r * gap * ROW;
      if (inside(x, y, inset)) out.push({ x, y });
    }
  }
  return out.sort((a, b) => a.y - b.y || a.x - b.x);
}

/** Distance from (x, y) to the segment `[x0, y0, x1, y1]` is under `r`. */
const near = (x, y, [x0, y0, x1, y1], r) => {
  const dx = x1 - x0, dy = y1 - y0;
  const t = Math.max(0, Math.min(1, ((x - x0) * dx + (y - y0) * dy) / (dx * dx + dy * dy)));
  const ex = x0 + t * dx - x, ey = y0 + t * dy - y;
  return ex * ex + ey * ey < r * r;
};

/** The river course through tile `i`, as segments `[x0, y0, x1, y1]` in world units: from its centre toward its flow
 *  tile, when it is a river (`river > 0`, `flow >= 0`), and from each neighbour that flows into it, from that
 *  neighbour's edge with `i` to `i`'s centre. A flow tile is always a neighbour, so each segment ends at their edge's
 *  midpoint. */
export function riverSegments(i, { river, flow, nb, px, py }) {
  const out = [];
  if (river[i] > 0 && flow[i] >= 0) {
    const f = flow[i];
    out.push([px[i], py[i], (px[i] + px[f]) / 2, (py[i] + py[f]) / 2]);
  }
  for (let d = 0; d < 6; d++) {
    const j = nb[i * 6 + d];
    if (j >= 0 && river[j] > 0 && flow[j] === i) out.push([(px[i] + px[j]) / 2, (py[i] + py[j]) / 2, px[i], py[i]]);
  }
  return out;
}

/**
 * The wood pieces of a map, in draw order: `{ key, x, y, tile, slot, kind }`, `kind` 'tree' or 'reed', `x, y` the
 * moved point the piece stands on (its anchor). `terrain`, `river`, `flow`, `nb` (six per tile, -1 off the map), `px`
 * and `py` are the map's columns; `forest` and `marsh` are the terrain indices; `fields` is a Set of the tiles that have
 * a plot this turn.
 */
export function woodPieces({ terrain, river, flow, nb, px, py, forest, marsh, fields }) {
  const TREE = slotsOf(WOODS.gap.tree, WOODS.inset.tree), REED = slotsOf(WOODS.gap.reed, WOODS.inset.reed);
  const { jitter, band, sparse, river: riverR, clearing } = WOODS;
  const pieces = [];
  for (let i = 0; i < terrain.length; i++) {
    const t = terrain[i];
    if (t !== forest && t !== marsh) continue;
    const isTree = t === forest;
    const slots = isTree ? TREE : REED;
    const course = riverSegments(i, { river, flow, nb, px, py });
    const open = [];
    if (isTree) for (let d = 0; d < 6; d++) { const j = nb[i * 6 + d]; if (j < 0 || terrain[j] !== forest) open.push(d); }
    const clear = fields.has(i);
    slots.forEach((s, slot) => {
      const x = px[i] + s.x + (slotDraw(i, slot, SALT.jx) * 2 - 1) * jitter;
      const y = py[i] + s.y + (slotDraw(i, slot, SALT.jy) * 2 - 1) * jitter;
      if (course.some((seg) => near(x, y, seg, riverR))) return;
      const lx = x - px[i], ly = y - py[i];
      if (clear && Math.abs(lx) < clearing.x && ly > clearing.top && ly < clearing.bottom) return;
      const variant = Math.floor(slotDraw(i, slot, SALT.variant) * 3);
      if (!isTree) {
        pieces.push({ key: `reed.${variant}`, x, y, tile: i, slot, kind: 'reed' });
        return;
      }
      const depth = Math.min(Infinity, ...open.map((d) => APOTHEM - (lx * NORMALS[d][0] + ly * NORMALS[d][1])));
      if (depth < band) {
        if (slotDraw(i, slot, SALT.sparse) < sparse) pieces.push({ key: `tree.small.${variant}`, x, y, tile: i, slot, kind: 'tree' });
      } else pieces.push({ key: `tree.big.${variant}`, x, y, tile: i, slot, kind: 'tree' });
    });
  }
  return pieces.sort((a, b) => a.y - b.y || a.x - b.x || a.tile - b.tile || a.slot - b.slot);
}
