// The goods on the page (#356): a store's wares as rows that pick a good (the picked one marked), and the chains
// a picked good makes on the map, which the flow layer draws
// (map/layers/flow.js). Imported by panels.js and panels-land.js.
import { f2 } from './panels.js';

const cap = (g) => `${g[0].toUpperCase()}${g.slice(1)}`;

/** A number row for each of salt and obsidian in a store: with `always`, a row for each even at 0; else only
 *  when held. With `at` (the store's tile) each row picks its good: a click shows its chains on the map, and a
 *  second click clears them (`ctx.good`, marked). */
export function wareRows(ctx, store, always, what, at = null) {
  return ctx.goods
    .map((g, f) => ({ g, f }))
    .filter(({ f }) => f >= ctx.rules.foods && (always || store[f] > 0.005))
    .map(({ g, f }) => {
      const tip = f === ctx.rules.foods ? `Salt in ${what}: it keeps the food there from spoiling, and is used up doing it` : `Obsidian in ${what}: a warrior raised from the store takes ${ctx.rules.arms.each} of it for a blade`;
      const row = { label: cap(g), value: f2(store[f]), tip };
      if (at == null) return row;
      const picked = ctx.good?.store === at && ctx.good.name === g;
      return {
        ...row,
        tip: `${tip}. ${picked ? 'Click to stop showing where it came from' : 'Click to show where it came from, on the map'}`,
        pick: { store: at, name: g },
        picked,
      };
    });
}

/** The chains the picked good makes on the map: `{good, store, chains}`, each chain a lot of it in the store
 *  with its share and the tiles it went through, from its source to the store (`model.chainsOf`). Null with no
 *  good picked. */
export function flowOf(ctx) {
  if (!ctx.good) return null;
  const { store, name } = ctx.good;
  return { good: name, store, chains: ctx.model.chainsOf(store, ctx.goods.indexOf(name)) };
}
