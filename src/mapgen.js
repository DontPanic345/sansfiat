import { FEATURES, FOODS, MAP, TERRAIN, TRAILS } from './data.js';
import { DIRS, fromOffset, key, neighbors, within } from './hex.js';
import { rand, randInt, weighted } from './rng.js';

export function generateMap(state, width, height) {
  const cells = [];
  for (let row = 0; row < height; row++)
    for (let col = 0; col < width; col++) {
      const edge = Math.min(col, row, width - 1 - col, height - 1 - row);
      cells.push({ ...fromOffset(col, row), edge, h: rand(state), m: rand(state), y: rand(state) });
    }
  const byKey = Object.fromEntries(cells.map((c) => [key(c), c]));

  for (let pass = 0; pass < 3; pass++)
    for (const field of ['h', 'm', 'y']) {
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
  norm('y');

  const tiles = {};
  for (const c of cells) {
    c.h -= c.edge < 2 ? 0.25 : 0;
    const terrain =
      c.edge === 0 || c.h < 0.35 ? 'ocean'
      : c.h > 0.88 ? 'mountain'
      : c.h > 0.72 ? 'hills'
      : c.m > 0.6 ? 'forest'
      : 'plains';
    const [lo, hi] = MAP.yieldRange;
    const fertility = Math.round((lo + (hi - lo) * c.y ** 1.5) * 10) / 10;
    tiles[key(c)] = { q: c.q, r: c.r, terrain, fertility, river: false, coast: false, claim: null, occ: 0, occTotal: 0 };
  }
  const setTerrain = (h, terrain, height) => {
    const t = tiles[key(h)];
    if (!t || byKey[key(h)].edge === 0) return;
    t.terrain = terrain;
    byKey[key(h)].h = height;
  };

  carveWater(state, cells, tiles, setTerrain);
  raiseRidges(state, tiles, setTerrain);
  makeDeserts(cells, tiles);

  const land = (t) => t && TERRAIN[t.terrain].passable;
  for (const t of Object.values(tiles)) {
    t.coast = land(t) && neighbors(t).some((n) => tiles[key(n)]?.terrain === 'ocean');
    t.shallows = t.terrain === 'ocean' && neighbors(t).some((n) => land(tiles[key(n)]));
  }

  carveRivers(state, tiles, byKey, MAP.rivers);

  for (const t of Object.values(tiles)) {
    t.cap = {};
    for (const f of FOODS) {
      const v = (TERRAIN[t.terrain].food[f] ?? 0)
        + (t.river ? FEATURES.river[f] ?? 0 : 0)
        + (t.coast ? FEATURES.coast[f] ?? 0 : 0)
        + (t.shallows ? FEATURES.shallows[f] ?? 0 : 0);
      if (v) t.cap[f] = Math.round(v * t.fertility * 10) / 10;
    }
    if (t.cap.grain) Object.assign(t, { wildGrain: t.cap.grain, tame: 0 });
    t.stock = { ...t.cap };
  }
  rollTrails(state, tiles);
  return { width, height, tiles };
}

// Every tile gets the shared infrastructure fields; forest, hills and mountains also roll their
// hidden trail cost (and mountains a bridge span). The rolls use their own stream seeded from
// the main one, so adding them did not change the rest of the map or the pre-game.
function rollTrails(state, tiles) {
  const sub = { rng: (state.rng ^ 0x5bd1e995) >>> 0 };
  for (const t of Object.values(tiles)) {
    Object.assign(t, { trail: false, fort: null, cache: null, fallow: 0 });
    const spec = TRAILS[t.terrain];
    if (!spec) continue;
    const span = spec.span ? weighted(sub, spec.span) : undefined;
    if (span === null) t.trailCost = null;
    else {
      t.trailCost = weighted(sub, spec.cost);
      if (span) t.span = span;
    }
  }
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

// Inlets cut in from the coast to leave peninsulas; inland seas are blobs in the interior.
function carveWater(state, cells, tiles, setTerrain) {
  const coastSea = cells.filter((c) => c.edge >= 2 && tiles[key(c)].terrain === 'ocean'
    && neighbors(c).some((n) => tiles[key(n)] && tiles[key(n)].terrain !== 'ocean'));
  for (let i = 0; i < MAP.inlets && coastSea.length; i++) {
    const start = coastSea[randInt(state, coastSea.length)];
    for (const h of walk(state, start, 6 + randInt(state, 10), 0.2)) setTerrain(h, 'ocean', 0);
  }
  const inland = cells.filter((c) => c.edge >= 5);
  for (let i = 0; i < MAP.inlandSeas; i++) {
    const start = inland[randInt(state, inland.length)];
    for (const h of walk(state, start, 5 + randInt(state, 6), 0.5))
      for (const n of within(h, 1)) setTerrain(n, 'ocean', 0);
  }
}

// Long wandering mountain chains with hills along their flanks and the odd pass (the same
// generator as the scout prototype's; hex-economy's copy was already merged with it).
function raiseRidges(state, tiles, setTerrain) {
  const land = Object.values(tiles).filter((t) => t.terrain !== 'ocean');
  const [lo, hi] = MAP.ridgeLength;
  for (let i = 0; i < MAP.ridges; i++) {
    const path = walk(state, land[randInt(state, land.length)], lo + randInt(state, hi - lo), 0.25);
    for (const h of path) {
      if (tiles[key(h)]?.terrain === 'ocean') continue;
      setTerrain(h, rand(state) < MAP.passChance ? 'hills' : 'mountain', 1);
      for (const n of neighbors(h))
        if (tiles[key(n)] && ['plains', 'forest', 'desert'].includes(tiles[key(n)].terrain) && rand(state) < MAP.flankHills)
          setTerrain(n, 'hills', 0.8);
    }
  }
}

// The driest share of flat land becomes desert, in blobs because moisture is smoothed noise.
function makeDeserts(cells, tiles) {
  const flat = cells.filter((c) => tiles[key(c)].terrain === 'plains').sort((a, b) => a.m - b.m);
  for (const c of flat.slice(0, Math.floor(flat.length * MAP.desertShare))) tiles[key(c)].terrain = 'desert';
}

// Each river runs downhill from high ground and is kept only if it reaches the sea.
// `flow` is the key of the next tile downstream.
function carveRivers(state, tiles, byKey, count) {
  const sources = Object.values(tiles).filter((t) => t.terrain === 'hills' || t.terrain === 'mountain');
  for (let i = 0; i < count && sources.length; i++) {
    const src = sources.splice(Math.floor(rand(state) * sources.length), 1)[0];
    const path = [src];
    const seen = new Set([key(src)]);
    let reached = false;
    while (path.length < 40) {
      const here = path.at(-1);
      const next = neighbors(here).map((n) => tiles[key(n)]).filter((n) => n && !seen.has(key(n)))
        .sort((a, b) => byKey[key(a)].h - byKey[key(b)].h)[0];
      if (!next) break;
      if (next.terrain === 'ocean') { reached = true; break; }
      if (next.river) { path.push(next); reached = true; break; }
      if (next.terrain === 'mountain') break;
      seen.add(key(next));
      path.push(next);
    }
    if (!reached) continue;
    path.forEach((t, j) => {
      if (t.terrain === 'mountain') return;
      t.river = true;
      if (path[j + 1] && !t.flow) t.flow = key(path[j + 1]);
    });
  }
}

export const isLand = (tile) => TERRAIN[tile.terrain].passable;
