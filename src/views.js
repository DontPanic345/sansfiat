import { FOODS, TERRAIN } from './data.js';
import { key } from './hex.js';
import { footprint, total } from './rules.js';

// Debug map views. Each `values(s)` returns a Map of tile key -> number; tiles left out
// are drawn as `none`. Hot keys are the number of the view in this list.
export const VIEWS = {
  terrain: { label: 'Terrain' },
  population: {
    label: 'Population',
    note: 'People drawing food from each tile',
    values: (s) => {
      const out = new Map();
      for (const c of Object.values(s.clans))
        for (const t of footprint(s, c)) if (t.terrain !== 'mountain') out.set(key(t), (out.get(key(t)) ?? 0) + c.pop);
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
    note: 'Turns any clan has lived on the tile',
    values: (s) => landMap(s, (t) => t.occTotal || null),
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
