// The shore and fringe pieces of a map, in draw order: the bank along each land–water edge and round each shore
// vertex, and the ragged fringe where two land kinds meet. Pure (no DOM): the layer draws them, and www/check-map.mjs
// and www/play/edges.mjs read them. Directions are `EDGE_DIR`'s: 0 E, 1 SE, 2 SW, 3 W, 4 NW, 5 NE.
import { CORNER } from './geom.js';

/** The fringe order, highest first: a kind bleeds into its lower neighbours. Plains is lowest and absent. */
export const FRINGE = ['desert', 'marsh', 'forest', 'hills', 'mountain', 'coast'];

/** A tile's kind: the terrain's name, terrain 0 split into 'sea' and 'lake'. */
export function kindOf(i, { terrain, lake, names }) {
  return terrain[i] === 0 && lake[i] ? 'lake' : names[terrain[i]];
}

/** A kind's place in the fringe order: 0 for the highest, Infinity for a kind that never bleeds. */
const rankOf = (kind) => {
  const r = FRINGE.indexOf(kind);
  return r < 0 ? Infinity : r;
};

/** The variant of a piece on the edge between tiles `i` and `j`: the lower-numbered tile's parity. */
const variantOf = (i, j) => Math.min(i, j) % 2;

/**
 * The shore and fringe pieces of a map: `{ key, x, y, tile, other, kind }`, `kind` 'fringe', 'edge' or 'corner'.
 * `x, y` are the piece's point in world units (a fringe: the lower tile's centre; an edge: the shared edge's midpoint;
 * a corner: the vertex). `tile` is the tile it is drawn for (the lower tile of a fringe, the land tile of an edge, the
 * odd tile of a corner) and `other` the neighbour (an edge's or fringe's), or -1. `nb` is the view's six neighbours a
 * tile (-1 off the map), `px, py` the tiles' centres, `names` the terrain index's names.
 */
export function edgePieces({ terrain, lake, river, flow, nb, px, py, names }) {
  const n = terrain.length;
  const kinds = new Array(n);
  for (let i = 0; i < n; i++) kinds[i] = kindOf(i, { terrain, lake, names });
  const water = (i) => kinds[i] === 'sea' || kinds[i] === 'lake';
  const fringes = [];
  const edges = [];
  const corners = [];

  for (let i = 0; i < n; i++) {
    if (water(i)) continue;
    for (let d = 0; d < 6; d++) {
      const j = nb[i * 6 + d];
      if (j < 0) continue;
      if (water(j)) {
        // A mouth: the river crosses this edge into the water, so the bank has a gap for it.
        if (river[i] > 0 && flow[i] === j) continue;
        edges.push({ key: `shore.edge.${d}.${variantOf(i, j)}`, x: (px[i] + px[j]) / 2, y: (py[i] + py[j]) / 2, tile: i, other: j, kind: 'edge' });
      } else if (j > i && kinds[j] !== kinds[i] && (rankOf(kinds[i]) !== Infinity || rankOf(kinds[j]) !== Infinity)) {
        // Two land kinds meet: the higher kind bleeds into the lower tile. Each edge once, from the lower-numbered tile.
        const iHigher = rankOf(kinds[i]) < rankOf(kinds[j]);
        const low = iHigher ? j : i;
        const high = iHigher ? i : j;
        let toHigh = 0;
        while (nb[low * 6 + toHigh] !== high) toHigh++;
        fringes.push({ key: `fringe.${kinds[high]}.${toHigh}.${variantOf(i, j)}`, x: px[low], y: py[low], tile: low, other: high, kind: 'fringe' });
      }
    }
  }

  // Each vertex once: corner 1 of a tile with its E and SE neighbours, corner 2 with its SE and SW.
  for (let i = 0; i < n; i++) {
    for (const c of [1, 2]) {
      const trio = c === 1 ? [[i, 1], [nb[i * 6], 3], [nb[i * 6 + 1], 5]] : [[i, 2], [nb[i * 6 + 1], 4], [nb[i * 6 + 2], 0]];
      if (trio.some(([t]) => t < 0)) continue;
      const wet = trio.filter(([t]) => water(t));
      if (wet.length === 1) {
        const [t, k] = wet[0];
        corners.push({ key: `shore.in.${k}`, x: px[t] + CORNER[k][0], y: py[t] + CORNER[k][1], tile: t, other: -1, kind: 'corner' });
      } else if (wet.length === 2) {
        const [t, k] = trio.find(([u]) => !water(u));
        corners.push({ key: `shore.out.${k}`, x: px[t] + CORNER[k][0], y: py[t] + CORNER[k][1], tile: t, other: -1, kind: 'corner' });
      }
    }
  }

  const byPlace = (a, b) => a.y - b.y || a.x - b.x || a.tile - b.tile || a.other - b.other;
  // The fringes draw from the lowest kind to the highest, so a strong ground lies over a weak one.
  const fringeOrder = (p) => FRINGE.length - FRINGE.indexOf(kinds[p.other]);
  fringes.sort((a, b) => fringeOrder(a) - fringeOrder(b) || byPlace(a, b));
  edges.sort(byPlace);
  corners.sort(byPlace);
  return [...fringes, ...edges, ...corners];
}
