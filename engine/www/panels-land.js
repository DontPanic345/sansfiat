// The land's panels: a festival's and a bare tile's shape (spec items 16, Festival and Tile). `panels.js`
// reads these through `describe`.

import { civName, f2, reasonsOf, tileKind } from './panels.js';

/** A festival's panel (spec item 16, Festival): its age, traders and activity, its prices in the tooltip, the
 *  clans going, and its walkers drawn on the map (by the view's focus). */
export function festivalShape(ctx, m) {
  const { v, goods } = ctx;
  const going = m.clans.map((id) => ({ select: { kind: 'clan', id }, label: `Clan ${id}`, text: `${f2(v.clans[id]?.people ?? 0)} people` }));
  const prices = goods.map((g, f) => `${g} ${f2(m.prices[f])}`);
  const lo = Math.min(...m.prices), hi = Math.max(...m.prices);
  const centre = v.end && v.end.centres.includes(m.id);
  const town = v.towns?.find((t) => t.market === m.id);
  const doing = town
    ? `Town: ${town.clans.length} clans at the festival`
    : going.length ? `${going.length} clan${going.length === 1 ? '' : 's'} come` : 'No clan goes here';
  return {
    name: `Festival ${m.id}${centre ? ' (centre)' : ''}`,
    doing,
    numbers: [
      { label: 'Age', value: v.turn - m.founded, tip: 'Turns since it formed' },
      { label: 'Traders', value: m.traders, tip: 'Traders at the festival' },
      { label: 'Activity', value: f2(m.activity), tip: 'How busy the festival is' },
      { label: 'Prices', value: `${f2(lo)}–${f2(hi)}`, tip: `Prices: ${prices.join(', ')}` },
    ],
    list: going.length ? [{ label: 'Clans going', rows: going }] : [],
    reasons: reasonsOf(ctx, m.id),
    buttons: [],
  };
}

/** A bare tile's panel (spec item 16, Tile): the terrain, whose it is, and its foods. No buttons. */
export function tileShape(ctx, sel) {
  const { v, terrain, goods, rules, sim, model } = ctx;
  const i = sel.tile;
  const t = JSON.parse(sim.tile(i));
  const land = terrain[t.terrain].land;
  const civ = sim.tile_civ(i);
  const paper = Object.values(v.civs).find((c) => c.claim.includes(i) && !c.land.includes(i));
  const clan = model.clanAt(i);
  let doing;
  if (civ >= 0 && v.civs[civ]) doing = `${civName(ctx, civ)}'s ${v.civs[civ].capital === i ? 'capital' : 'land'}`;
  else if (paper) doing = `Paper of ${civName(ctx, paper.id)}`;
  else if (clan) doing = `Clan ${clan}'s territory`;
  else doing = "Nobody's";
  const numbers = [];
  if (land) {
    numbers.push({ label: 'Fertility', value: f2(t.fertility), tip: 'How much food the land grows' });
    goods.forEach((g, f) => {
      if (t.cap[f] > 0) numbers.push({ label: g, value: `${f2(t.stock[f])} of ${f2(t.cap[f])}`, tip: `${g} in the tile's stock, of its cap` });
    });
    numbers.push({ label: 'Trail', value: t.trail ? 'yes' : 'no', tip: 'A trail, worn by the walks over the tile' });
    if (t.wild_grain > 0) {
      numbers.push({ label: 'Tame', value: `${Math.round((100 * t.tame) / rules.tame_turns)}%`, tip: 'Share of the way to a hearth: the wild grain tamed here' });
    }
  }
  for (const u of model.holders(i)) numbers.push({ label: 'Held by', value: `${civName(ctx, u.civ)}'s army`, tip: 'An army whose zone holds the tile' });
  const living = Object.values(v.clans).filter((k) => k.home === i).sort((a, b) => a.id - b.id);
  const list = living.length
    ? [{ label: 'Clans living here', rows: living.map((k) => ({ select: { kind: 'clan', id: k.id }, label: `Clan ${k.id}`, text: `${f2(k.people)} people` })) }]
    : [];
  return { name: tileKind(ctx, i), doing, numbers, list, reasons: [], buttons: [] };
}

