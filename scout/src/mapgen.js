import { MAP, TERRAIN, TRAILS } from './data.js';
import { DIRS, fromOffset, key, neighbors } from './hex.js';
import { rand, randInt, weighted } from './rng.js';

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
  raiseRidges(state, tiles, byKey);
  for (const t of Object.values(tiles)) Object.assign(t, rollTrail(state, t.terrain));
  return { width, height, tiles };
}

// A wandering line of hexes from `start`, mostly holding one heading.
function walk(state, start, length, wiggle) {
  let dir = randInt(state, 6);
  const path = [start];
  for (let i = 1; i < length; i++) {
    if (rand(state) < wiggle) dir = (dir + (rand(state) < 0.5 ? 5 : 1)) % 6;
    const [dq, dr] = DIRS[dir];
    path.push({ q: path.at(-1).q + dq, r: path.at(-1).r + dr });
  }
  return path;
}

function raiseRidges(state, tiles, byKey) {
  const set = (h, terrain) => {
    if (tiles[key(h)] && byKey[key(h)].edge > 0) tiles[key(h)].terrain = terrain;
  };
  const land = Object.values(tiles).filter((t) => t.terrain !== 'ocean');
  const [lo, hi] = MAP.ridgeLength;
  for (let i = 0; i < MAP.ridges && land.length; i++) {
    for (const h of walk(state, land[randInt(state, land.length)], lo + randInt(state, hi - lo), 0.25)) {
      if (tiles[key(h)]?.terrain === 'ocean') continue;
      set(h, rand(state) < MAP.gap ? 'hills' : 'mountain');
      for (const n of neighbors(h))
        if (['plains', 'forest'].includes(tiles[key(n)]?.terrain) && rand(state) < MAP.flankHills) set(n, 'hills');
    }
  }
}

function rollTrail(state, terrain) {
  const spec = TRAILS[terrain];
  if (!spec) return {};
  const span = spec.span ? weighted(state, spec.span) : undefined;
  if (span === null) return { trailCost: null };
  return { trailCost: weighted(state, spec.cost), ...(span ? { span } : {}) };
}

export const isLand = (tile) => TERRAIN[tile.terrain].passable;
