// The land's panels: a festival's and a bare tile's shape (spec items 16, Festival and Tile). `panels.js`
// reads these through `describe`.

import { civName, civOrder, f2, hex, reasonsOf, tileKind } from './panels.js';

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

/** Raise at a settlement: the people come from the clans that go to its market, the pack from its store. */
function raiseAt(s) {
  const at = (extra) => (n) => civOrder(s.civ, { type: 'raise', n, at: s.tile, ...extra });
  return {
    label: 'Raise…',
    stepper: {
      options: [
        { label: 'warriors', cap: 10, make: at({ kind: 'camp', fighter: 'warriors' }) },
        { label: 'a scout', cap: 10, make: at({ kind: 'scout' }) },
      ],
    },
  };
}

/** A bare tile's panel (spec item 16, Tile): the terrain, whose it is, and its foods. No buttons. */
export function tileShape(ctx, sel) {
  const { v, terrain, goods, rules, sim, model } = ctx;
  const i = sel.tile;
  const t = JSON.parse(sim.tile(i));
  const land = terrain[t.terrain].land;
  const civ = sim.tile_civ(i);
  const clan = model.clanAt(i);
  let doing;
  if (civ >= 0 && v.civs[civ]) doing = `${civName(ctx, civ)}'s ${v.civs[civ].capital === i ? 'capital' : 'land'}`;
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
  const buttons = [];
  const post = (v.signposts ?? []).find((p) => p.tile === i);
  if (post) {
    const ago = v.turn - post.turn;
    numbers.push({ label: 'Signpost', value: `${civName(ctx, post.civ)}'s, posted ${ago} turn${ago === 1 ? '' : 's'} ago`, tip: 'A place named for a clan to settle' });
    numbers.push({
      label: 'Worth',
      value: `${Math.round(100 * Math.min(post.worth, 1))}%`,
      tip: "The share of a daughter's need the tile feeds: 100% feeds her in full",
    });
    if (post.civ === v.player) buttons.push({ label: 'Remove signpost', action: civOrder(post.civ, { type: 'unpost', tile: i }) });
  }
  return { name: tileKind(ctx, i), doing, numbers, list, reasons: [], buttons };
}

/** A settlement's panel: its store against the cap and this turn's levy, the clans living on it, the clans that
 *  come to its market, its land (drawn on the map while it is selected), and its reasons. */
export function settlementShape(ctx, s) {
  const { v, goods } = ctx;
  const clanRows = (ids) => ids.map((id) => ({ select: { kind: 'clan', id }, label: `Clan ${id}`, text: `${f2(v.clans[id]?.people ?? 0)} people` }));
  const total = s.store.reduce((a, b) => a + b, 0);
  const stored = goods.map((g, f) => (s.store[f] > 0.005 ? `${g} ${f2(s.store[f])}` : null)).filter(Boolean);
  const people = s.clans.reduce((a, id) => a + (v.clans[id]?.people ?? 0), 0);
  const name = civName(ctx, s.civ);
  return {
    name: `${name}'s settlement at (${hex(ctx, s.tile)})`,
    swatch: v.civs[s.civ]?.color,
    doing: `Founded turn ${s.founded}: ${s.clans.length} clan${s.clans.length === 1 ? '' : 's'}, ${f2(people)} people live here`,
    numbers: [
      { label: 'Store', value: `${f2(total)} of ${f2(s.cap)}`, tip: `${stored.join(', ') || 'Empty'}: filled only by the levy at its market` },
      { label: 'Levy', value: `${f2(s.levied)} this turn`, tip: `Food the civ's clans paid into this store at its market this turn` },
      { label: 'Land', value: `${s.land.length} tiles`, tip: "The part of the civ's claim nearer this settlement than the capital or another settlement; lit on the map" },
      { label: 'Founded', value: s.founded, tip: 'The turn a clan settled here' },
    ],
    list: [
      { label: 'Clans living here', rows: clanRows(s.clans), empty: 'No clan lives here.' },
      { label: 'Clans coming to its market', rows: clanRows(s.traders), empty: 'No clan comes to its market.' },
    ],
    reasons: reasonsOf(ctx, s.id),
    buttons: s.civ === v.player ? [raiseAt(s)] : [],
  };
}

