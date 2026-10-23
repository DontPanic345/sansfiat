import { TERRAIN } from './data.js';
import { fromOffset, key, neighbors } from './hex.js';
import { rand } from './rng.js';

export function generateMap(state, width, height) {
  const cells = [];
  for (let row = 0; row < height; row++)
    for (let col = 0; col < width; col++) {
      const edge = Math.min(col, row, width - 1 - col, height - 1 - row);
      cells.push({ ...fromOffset(col, row), edge, h: rand(state), m: rand(state) });
    }
  const byKey = Object.fromEntries(cells.map((c) => [key(c), c]));

  for (let pass = 0; pass < 3; pass++)
    for (const field of ['h', 'm']) {
      const next = cells.map((c) => {
        const ns = neighbors(c).map((n) => byKey[key(n)]).filter(Boolean);
        return (c[field] * 2 + ns.reduce((s, n) => s + n[field], 0)) / (ns.length + 2);
      });
      cells.forEach((c, i) => (c[field] = next[i]));
    }

  const norm = (field) => {
    const vs = cells.map((c) => c[field]);
    const lo = Math.min(...vs), hi = Math.max(...vs);
    cells.forEach((c) => (c[field] = (c[field] - lo) / (hi - lo || 1)));
  };
  norm('h');
  norm('m');

  const tiles = {};
  for (const c of cells) {
    const h = c.h - (c.edge < 2 ? 0.25 : 0);
    const terrain =
      c.edge === 0 || h < 0.35 ? 'ocean'
      : h > 0.88 ? 'mountain'
      : h > 0.72 ? 'hills'
      : c.m > 0.6 ? 'forest'
      : 'plains';
    tiles[key(c)] = { q: c.q, r: c.r, terrain };
  }
  return { width, height, tiles };
}

export const isLand = (tile) => TERRAIN[tile.terrain].passable;
