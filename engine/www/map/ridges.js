// The mountain pieces of a map: a peak on each mountain tile, a ridge on each shared mountain edge, in
// draw order. Pure (no DOM): the layer draws them, and www/check.mjs and www/play/ridges.mjs read them.
// Directions are `EDGE_DIR`'s: 0 E, 1 SE, 2 SW, 3 W, 4 NW, 5 NE.

/** The ridge keys' direction names, by the direction from a tile to a neighbour (0, 1, 2). */
const RIDGE_DIR = ['e', 'se', 'sw'];

/** A mountain tile's class, from its mountain neighbours (`nb` is the view's six per tile, -1 off the map). */
export function ridgeClass(i, { terrain, nb, mountain }) {
  const dirs = [];
  for (let d = 0; d < 6; d++) {
    const j = nb[i * 6 + d];
    if (j >= 0 && terrain[j] === mountain) dirs.push(d);
  }
  if (dirs.length === 0) return 'lone';
  if (dirs.length === 1) return 'end';
  if (dirs.length >= 3) return 'fork';
  // Two: how far apart they are round the tile, the shorter way (3 opposite, 2 a bend, 1 a sharp bend).
  const gap = Math.min(Math.abs(dirs[1] - dirs[0]), 6 - Math.abs(dirs[1] - dirs[0]));
  return gap === 3 ? 'straight' : gap === 2 ? 'bend' : 'sharp';
}

/**
 * The ridge pieces of a map, in draw order: `{ key, x, y, tile, kind }`, `x, y` the piece's ground point in
 * world units. A peak is at its tile's centre, a ridge at the midpoint of its shared edge (from directions
 * E, SE and SW, so each shared edge once). Sorted by y, then ridges before peaks, then x, then tile.
 */
export function ridgePieces({ terrain, nb, px, py, mountain }) {
  const pieces = [];
  for (let i = 0; i < terrain.length; i++) {
    if (terrain[i] !== mountain) continue;
    const kind = ridgeClass(i, { terrain, nb, mountain });
    const key = kind === 'lone' ? `mountain.lone.${i % 2}` : `mountain.peak.${i % 3}`;
    pieces.push({ key, x: px[i], y: py[i], tile: i, kind });
    for (let d = 0; d < 3; d++) {
      const j = nb[i * 6 + d];
      if (j < 0 || terrain[j] !== mountain) continue;
      pieces.push({
        key: `mountain.ridge.${RIDGE_DIR[d]}.${Math.min(i, j) % 2}`,
        x: (px[i] + px[j]) / 2,
        y: (py[i] + py[j]) / 2,
        tile: i,
        kind: 'ridge',
      });
    }
  }
  const rank = (p) => (p.kind === 'ridge' ? 0 : 1);
  return pieces.sort((a, b) => a.y - b.y || rank(a) - rank(b) || a.x - b.x || a.tile - b.tile);
}
