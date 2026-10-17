import { FOODS, TERRAIN } from './data.js';
import { key } from './hex.js';
import { footprint } from './clans.js';
import { total } from './core.js';

// Debug map views. Each `values(s)` returns a Map of tile key -> number; tiles left out
// are drawn as `none`. Hot keys follow VIEW_KEYS in list order.
export const VIEW_KEYS = '1234567890-=';
export const VIEWS = {
  terrain: { label: 'Terrain' },
  population: {
    label: 'Population',
    note: 'People per tile of clan land',
    values: (s) => {
      const out = new Map();
      for (const c of Object.values(s.clans)) {
        const land = c.route.length || !c.land.length ? [key(c)] : c.land;
        for (const k of land) out.set(k, (out.get(k) ?? 0) + c.pop / land.length);
      }
      return out;
    },
  },
  fertility: {
    label: 'Fertility',
    note: 'Multiplier on every food cap',
    values: (s) => landMap(s, (t) => t.fertility),
  },
  yield: {
    label: 'Gross yield',
    note: 'Food cap of the tile, all foods',
    values: (s) => tileMap(s, (t) => total(t.cap) || null),
  },
  site: {
    label: 'Site yield',
    note: 'Food cap a clan here could draw on',
    values: (s) => landMap(s, (t) => footprint(s, t).reduce((a, x) => a + total(x.cap), 0)),
  },
  variety: {
    label: 'Diet variety',
    note: 'Foods within a clan\'s reach of this tile',
    values: (s) => landMap(s, (t) => FOODS.filter((f) => footprint(s, t).some((x) => x.cap[f])).length),
  },
  stock: {
    label: 'Stock',
    note: 'Food left as a share of cap',
    values: (s) => tileMap(s, (t) => (total(t.cap) ? total(t.stock) / total(t.cap) : null)),
  },
  occupation: {
    label: 'Occupation',
    note: 'Turns any clan has held the tile',
    values: (s) => landMap(s, (t) => t.occTotal || null),
  },
  settled: {
    label: 'Settled',
    note: 'Turns since the clan holding the tile last moved',
    values: (s) => new Map(Object.values(s.clans).flatMap((c) => c.land.map((k) => [k, c.since]))),
  },
  tame: {
    label: 'Tended grain',
    note: 'Grain tiles from wild (0) to domesticated (1)',
    values: (s) => tileMap(s, (t) => t.tame ?? null),
  },
  trailCost: {
    label: 'Trail cost (hidden)',
    note: 'Move cost a trail would bring the tile to; blank where none can be built. Debug: players see this only once scouted',
    values: (s) => tileMap(s, (t) => t.trailCost ?? null),
  },
  passable: { label: 'Impassable', note: 'White walkable, red blocked, black sea' },
};

const tileMap = (s, f) => {
  const out = new Map();
  for (const t of Object.values(s.map.tiles)) {
    const v = f(t);
    if (v != null) out.set(key(t), v);
  }
  return out;
};
const landMap = (s, f) => tileMap(s, (t) => (TERRAIN[t.terrain].passable ? f(t) : null));

// Dark blue -> teal -> yellow, roughly viridis.
const RAMP = [[68, 1, 84], [59, 82, 139], [33, 145, 140], [94, 201, 98], [253, 231, 37]];
export function ramp(x) {
  const p = Math.max(0, Math.min(1, x)) * (RAMP.length - 1);
  const i = Math.min(RAMP.length - 2, Math.floor(p)), f = p - i;
  const c = RAMP[i].map((v, j) => Math.round(v + (RAMP[i + 1][j] - v) * f));
  return `rgb(${c.join(',')})`;
}
export const rampCss = `linear-gradient(to right, ${RAMP.map((_, i) => ramp(i / (RAMP.length - 1))).join(', ')})`;
