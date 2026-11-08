// The selection: what is selected (a unit, a civ, a clan, a festival or a tile), what a click on a tile
// picks, and the panel's shape for each kind (spec items 15-17). `describe(ctx, sel)` gives the shape
// panel.js draws. A row of a shape that selects something carries the selection to `hooks.select`.
//
// A selection is {kind: 'unit', id}, {kind: 'civ', civ}, {kind: 'clan', id}, {kind: 'festival', id} or
// {kind: 'tile', tile}. `ctx` is what the page knows: {v (Sim.view), model (model.js), layout, terrain,
// goods, rules, sim, view (MapView), routing, outcome, painting}.

export const f2 = (x) => (Math.round(x * 100) / 100).toString();
const ZONES = ['', 'valley', 'sealed valley', 'neck'];
const pct = (x) => `${(Math.round(x * 1000) / 10).toFixed(1)}%`;
const ORD = (n) => n + (n % 100 >= 11 && n % 100 <= 13 ? 'th' : ['th', 'st', 'nd', 'rd'][n % 10] ?? 'th');
const plural = (n, word) => `${n} ${word}${n === 1 ? '' : 's'}`;
const sum = (xs) => xs.reduce((a, b) => a + b, 0);
const f1 = (x) => x.toFixed(1);
const TENTHS = ['none', 'a tenth', 'a fifth', 'three tenths', 'two fifths', 'half', 'three fifths', 'seven tenths', 'four fifths', 'nine tenths', 'all'];
/** A share of days as words: 'a fifth' for 0.2, 'three tenths' for 0.3, 'all' for 1. */
export const shareWords = (x) => TENTHS[Math.round(Math.min(1, Math.max(0, x)) * 10)];

/** An order for unit `u` (its civ gives the order), in the wire form `Sim.act` takes. */
export const unitOrder = (u, order) => ({ type: 'order', civ: u.civ, order: { ...order, unit: u.id } });
/** A civ's order (raising warriors for its camp): `civ` is the civ's id. */
export const civOrder = (civ, order) => ({ type: 'order', civ, order });

/** The key `body.dataset.selected` and the cycle use: `unit:40`, `civ:3`, `clan:140`, `festival:12`, `tile:812`. */
export function keyOf(sel) {
  if (!sel) return '';
  switch (sel.kind) {
    case 'unit': return `unit:${sel.id}`;
    case 'civ': return `civ:${sel.civ}`;
    case 'clan': return `clan:${sel.id}`;
    case 'festival': return `festival:${sel.id}`;
    default: return `tile:${sel.tile}`;
  }
}

/** The tile a selection stands on, or -1. */
export function tileOf(ctx, sel) {
  if (!sel) return -1;
  const { v } = ctx;
  switch (sel.kind) {
    case 'tile': return sel.tile;
    case 'civ': return v.civs[sel.civ]?.capital ?? -1;
    case 'clan': return v.clans[sel.id]?.home ?? -1;
    case 'festival': return v.markets[sel.id]?.tile ?? -1;
    default: return v.units[sel.id]?.tile ?? -1;
  }
}

/** Whether the selection still exists (a disbanded unit or a gone festival does not). */
export function alive(ctx, sel) {
  if (!sel) return false;
  const { v } = ctx;
  switch (sel.kind) {
    case 'unit': return sel.id in v.units;
    case 'civ': return sel.civ in v.civs;
    case 'clan': return sel.id in v.clans;
    case 'festival': return sel.id in v.markets;
    default: return true;
  }
}

export const marketOn = (ctx, i) => Object.values(ctx.v.markets).find((m) => m.tile === i);

/** Everything a left-click on tile `i` can pick, in cycle order: the player's units (by id), the capital,
 *  the clan whose home it is, the festival there, other civs' units, then the bare tile. */
export function candidates(ctx, i) {
  const { v } = ctx;
  const here = Object.values(v.units).filter((u) => u.tile === i).sort((a, b) => a.id - b.id);
  const out = here.filter((u) => u.civ === v.player).map((u) => ({ kind: 'unit', id: u.id }));
  for (const c of Object.values(v.civs)) if (c.capital === i) out.push({ kind: 'civ', civ: c.id });
  for (const k of Object.values(v.clans)) if (k.home === i) out.push({ kind: 'clan', id: k.id });
  const m = marketOn(ctx, i);
  if (m) out.push({ kind: 'festival', id: m.id });
  for (const u of here) if (u.civ !== v.player) out.push({ kind: 'unit', id: u.id });
  out.push({ kind: 'tile', tile: i });
  return out;
}

/** The selection a feed item stands for: its unit, its civ (a founding), its clan, its festival, else its tile. */
export function subjectOf(ctx, item) {
  const { v } = ctx;
  if (item.unit != null && item.unit in v.units) return { kind: 'unit', id: item.unit };
  if ((item.kind === 'civFounded' || item.kind === 'capitalFell') && item.civ != null && item.civ in v.civs) return { kind: 'civ', civ: item.civ };
  if (item.clan != null && item.clan in v.clans) return { kind: 'clan', id: item.clan };
  if (item.market != null && item.market in v.markets) return { kind: 'festival', id: item.market };
  return { kind: 'tile', tile: item.tile };
}

export function terrainLabel(ctx, i) {
  const l = ctx.layout;
  const t = ctx.terrain[l.terrain[i]];
  if (l.terrain[i] === 0) return l.lake[i] ? 'lake' : 'ocean';
  if (t.name === 'marsh') return 'marsh (delta)';
  return t.name;
}

/** The terrain name and what's on it: river, floodplain, pass, zone. */
export function tileKind(ctx, i) {
  const l = ctx.layout;
  const bits = [terrainLabel(ctx, i)];
  if (l.river[i] > 0) bits.push(`river (${l.river[i]} upstream)`);
  if (l.flood[i]) bits.push('floodplain');
  if (l.ground?.[i] > 0) bits.push(`field ground ${f2(l.ground[i])} days`);
  const field = ctx.model?.fieldAt(i);
  if (field) bits.push(`fields ${f1(field.days)} days (clan ${field.clan})`);
  if (l.pass[i]) bits.push('pass');
  if (l.zone[i] && ctx.country) bits.push(ZONES[l.zone[i]]);
  return bits.join(', ');
}

/** What tile `i` is to a clan: its home, in its territory, or on its festival route (can be several). */
export function clanRole(c, i) {
  const bits = [];
  if (c.home === i) bits.push('home');
  if (c.territory.includes(i)) bits.push('territory');
  if (c.routes.length && c.routes[0].path.includes(i)) bits.push('route');
  return bits;
}

const hex = (ctx, i) => `${ctx.layout.q[i]}, ${ctx.layout.r[i]}`;
const civName = (ctx, id) => ctx.v.civs[id]?.name ?? '?';
const unitName = (u) => { const k = u.kind === 'camp' ? 'army' : u.kind; return `${k[0].toUpperCase()}${k.slice(1)} ${u.id}`; };
const pack = (u) => sum(u.store);

/** The latest reasons the subject has (`Sim.notes`), newest first, for the panel's Why list. */
function reasonsOf(ctx, id) {
  if (id == null) return [];
  return JSON.parse(ctx.sim.notes(id))
    .slice(0, ctx.rules.notes_shown)
    .map((n) => ({ turn: n.turn, text: n.text }));
}

/** The other units on a unit's tile, as list rows that select them. */
function unitsHere(u, ctx) {
  return Object.values(ctx.v.units)
    .filter((o) => o.tile === u.tile && o.id !== u.id)
    .sort((a, b) => a.id - b.id)
    .map((o) => ({ select: { kind: 'unit', id: o.id }, label: unitName(o), text: '' }));
}

/** The claim's counts: claimed, held, paper and the thinnest held guard, from the model. */
function claimNumbers(ctx, civ) {
  const c = ctx.model.claims.get(civ);
  if (!c) return [];
  return [
    { label: 'Claimed', value: c.claimed, tip: 'Tiles the civ claims: its land, and the paper it has not yet held' },
    { label: 'Held', value: c.held, tip: 'Claimed tiles a guard holds' },
    { label: 'Paper', value: c.paper, tip: 'Claimed tiles no guard holds yet' },
    { label: 'Thinnest guard', value: f2(c.thinnest), tip: 'The least guard share on a held tile' },
  ];
}

/** The army's people by kind and their strength, attacking and defending (hunger in, entrenchment apart), from
 *  the engine's rules (`Sim.rules`): each fighter's people times its attack or defence. */
function strengthOf(ctx, u) {
  const { rules } = ctx;
  const hungry = u.hungry > 0 ? rules.fight.hungry : 1;
  const stack = u.stack ?? [];
  const attack = rules.fighters.reduce((a, f, k) => a + (stack[k] ?? 0) * f.attack * hungry, 0);
  const defence = rules.fighters.reduce((a, f, k) => a + (stack[k] ?? 0) * f.defence * hungry, 0);
  const bonus = Math.min(u.still * rules.fight.entrench, rules.fight.entrench_cap);
  return { attack, defence, bonus, hungry };
}

/** The camp's panel (spec item 16, Camp; build-4: the army's fighters, strength, entrenchment and Raise…). */
function campShape(ctx, u) {
  const { v, sim, rules, routing, painting } = ctx;
  const mine = u.civ === v.player;
  const unitRows = unitsHere(u, ctx);
  const owner = civName(ctx, u.civ);
  const deployed = u.kind === 'camp' && u.territory.length > 1;
  const capital = v.civs[u.civ]?.capital === u.tile;
  const marching = u.dest != null;
  const r = marching ? JSON.parse(sim.route(u.id)) : null;
  let base;
  if (u.hungry > 0) base = `Hungry ${plural(u.hungry, 'turn')}: loses a warrior at ${rules.hungry_turns}`;
  else if (u.cut_at) base = `Cut off: ${civName(ctx, u.cut_at.by)}'s army holds its line at ${hex(ctx, u.cut_at.at)}`;
  else if (u.cut_off) base = "Cut off: its line can't feed it";
  else if (marching) base = `Marching to ${hex(ctx, u.dest)}${r ? `: ${plural(r.turns, 'turn')}` : ': no route'}`;
  else if (deployed) base = `Holding ${u.territory.length} tiles round ${hex(ctx, u.tile)}`;
  else if (capital) base = 'At the capital';
  else base = `At ${hex(ctx, u.tile)}`;
  const extra = [];
  if (v.contested?.includes(u.tile)) extra.push(`Fighting at (${hex(ctx, u.tile)})`);
  if (u.bows) extra.push('Making bows');
  const doing = [...extra, base].join(' · ');

  const line = u.cut_at
    ? 'cut'
    : u.line && u.line.path.length
      ? `${plural(u.line.path.length, 'step')}, line ${f2(u.line.cost)} days, carries ${u.line.carry == null ? 'without limit' : `${f2(u.line.carry)} food a turn`}`
      : u.line
        ? 'at the capital'
        : 'none';
  const st = strengthOf(ctx, u);
  const numbers = [
    { label: 'Warriors', value: f2(u.stack?.[0] ?? u.people), tip: 'The warriors in this army' },
    { label: 'Archers', value: f2(u.stack?.[1] ?? 0), tip: 'The archers in this army: weaker taking a tile, stronger holding one' },
    { label: 'Attack', value: f2(st.attack), tip: `Strength taking a tile it doesn't hold: each fighter's people times its attack${st.hungry < 1 ? `, ×${rules.fight.hungry} while hungry` : ''}` },
    { label: 'Defence', value: f2(st.defence), tip: `Strength holding a tile: each fighter's people times its defence${st.hungry < 1 ? `, ×${rules.fight.hungry} while hungry` : ''}` },
    {
      label: 'Entrenched',
      value: u.still > 0 ? `${plural(u.still, 'turn')}, +${Math.round(100 * st.bonus)}%` : 'no',
      tip: `Turns it has stood still; each turn adds ${Math.round(100 * rules.fight.entrench)}% to its defence, to ${Math.round(100 * rules.fight.entrench_cap)}%`,
    },
    { label: 'Pack', value: `${f2(pack(u))} of ${f2(u.pack_cap)}, ${plural(Math.round(u.food_turns), 'turn')} of food`, tip: 'Food it carries, and the turns that lasts' },
    { label: 'Line', value: line, tip: 'Its line home: steps, the days it costs, and the food a turn it carries with all its labour' },
  ];
  if (u.kind === 'camp') {
    const guard = u.guard.length ? u.guard.reduce((a, b) => a + b, 0) / u.guard.length : 0;
    numbers.push({
      label: 'Zone',
      value: deployed ? `${u.territory.length} of ${u.zone_cap} tiles, holds ${u.held.length}, guard ${f2(guard)} a tile` : `not deployed (up to ${u.zone_cap} tiles)`,
      tip: `Tiles of its zone, the most it may take, the tiles its warriors hold, and the guard on the least of them (${u.guard.length ? f2(Math.min(...u.guard)) : '0'})`,
    });
  }
  const buttons = [];
  if (mine) {
    buttons.push({ label: 'Move to…', arm: 'move' });
    if (marching) buttons.push({ label: 'Stop', action: unitOrder(u, { type: 'move', to: u.tile }) });
    if (u.kind === 'camp') {
      buttons.push({ label: 'Paint (P)', paint: 'paint' }, { label: 'Unpaint (U)', paint: 'unpaint' });
    }
    buttons.push({ label: 'Forage', action: unitOrder(u, { type: 'forage' }) });
    buttons.push({ label: 'Pillage', action: unitOrder(u, { type: 'pillage' }) });
    buttons.push({ label: 'Resupply', action: unitOrder(u, { type: 'resupply' }) });
    buttons.push(...switchButtons(u));
    if (deployed) buttons.push({ label: 'Undeploy', action: unitOrder(u, { type: 'zone', tiles: [] }) });
    if (u.kind === 'camp') buttons.push(raiseInto(u, rules));
    buttons.push(...disband(u, rules));
  }
  let hint = null;
  if (routing.arm === 'move') hint = 'Click the map to move the army there; Esc cancels';
  else if (painting === 'paint' || painting === 'unpaint') hint = 'Click or drag on the map; Esc stops';
  else if (mine && !marching) hint = 'Right-click a tile to move the army there';
  return {
    name: `${owner}'s ${u.kind === 'camp' ? 'army' : u.kind} ${u.id}`,
    swatch: v.civs[u.civ]?.color,
    doing,
    numbers,
    list: unitRows.length ? [{ label: 'Units here', rows: unitRows }] : [],
    reasons: reasonsOf(ctx, u.id),
    buttons,
    hint,
  };
}

/** The two switches as the buttons show them (the camp and the scout share them). */
function switchButtons(u) {
  return [
    { label: `Auto-forage: ${u.auto_forage ? 'on' : 'off'}`, action: unitOrder(u, { type: 'switches', forage: !u.auto_forage, supply: u.auto_supply }) },
    { label: `Auto-supply: ${u.auto_supply ? 'on' : 'off'}`, action: unitOrder(u, { type: 'switches', forage: u.auto_forage, supply: !u.auto_supply }) },
  ];
}

/** Raise…: a count of warriors or archers into this army, each refused with the engine's reason (greyed). */
function raiseInto(u) {
  const into = (fighter) => (n) => civOrder(u.civ, { type: 'raise', kind: 'camp', fighter, into: u.id, n });
  return {
    label: 'Raise…',
    stepper: { options: [{ label: 'warriors', cap: 10, make: into('warriors') }, { label: 'archers', cap: 10, make: into('archers') }] },
  };
}

/** Disband…: a count of one kind the army has (an option per kind present), or of the people of a scout or a guard
 *  (`people`, shown from two people). Disband all is its own button and sends the whole unit, with no count. */
function disband(u, rules = null) {
  const options = [];
  if (rules) {
    rules.fighters.forEach((f, k) => {
      const people = u.stack?.[k] ?? 0;
      if (people >= 1) options.push({ label: f.name, cap: Math.floor(people), make: (n) => unitOrder(u, { type: 'disband', n, fighter: f.name }) });
    });
  } else if (Math.floor(u.people) >= 2) {
    options.push({ label: 'people', cap: Math.floor(u.people), make: (n) => unitOrder(u, { type: 'disband', n }) });
  }
  const all = { label: 'Disband all', action: unitOrder(u, { type: 'disband' }) };
  return options.length ? [{ label: 'Disband…', stepper: { options } }, all] : [all];
}

/** A scout's panel (spec items 23-26): what it does (its errand, its hunger), its route's known and possible costs,
 *  the search's mountains to check and left, its hunger, its switches, and the buttons. */
function scoutShape(ctx, u) {
  const { v, routing, outcome, rules, sim } = ctx;
  const mine = u.civ === v.player;
  const plan = u.plan;
  const search = u.search;
  const left = search?.left ?? [];
  const errand = u.route != null || u.dest != null;
  let base;
  if (u.wander != null && errand) base = `Foraging at ${hex(ctx, u.wander)} until fed, then ${u.route ? 'back to its route' : `on to ${hex(ctx, u.dest)}`}`;
  else if (u.route) base = `Searching ${hex(ctx, u.route[0])} → ${hex(ctx, u.route[1])}: known route ${plan?.known ? `${f2(plan.known.cost)} days` : 'none'}, possible ${plan?.possible ? `${f2(plan.possible.cost)} days` : 'none'}`;
  else if (u.dest != null && u.dest === u.tile) base = `At ${hex(ctx, u.tile)}, foraging`;
  else if (u.dest != null) {
    const r = JSON.parse(sim.route(u.id));
    base = `Going to ${hex(ctx, u.dest)}${r ? `: ${plural(r.turns, 'turn')}` : ': no route'}`;
  } else if (u.wander != null) base = `Waiting on rich ground at ${hex(ctx, u.wander)}`;
  else base = `At ${hex(ctx, u.tile)}, foraging`;
  const doing = [...(u.hungry > 0 ? [`Hungry ${plural(u.hungry, 'turn')}: loses a person at ${rules.hungry_turns}`] : []), base].join(' · ');
  const hunger = u.hunger;
  const numbers = [
    { label: 'People', value: f2(u.people), tip: 'The people in the scout' },
    { label: 'Pack', value: `${f2(pack(u))} of ${f2(u.pack_cap)}, ${plural(Math.round(u.food_turns), 'turn')} of food`, tip: 'Food it carries, and the turns that lasts' },
    { label: 'Known route', value: plan?.known ? `${f2(plan.known.cost)} days` : 'none', tip: 'The days of the route over the tiles the player has seen' },
    { label: 'Possible route', value: plan?.possible ? `${f2(plan.possible.cost)} days` : 'none', tip: 'The days of the cheapest route if the unrevealed tiles are as they could be' },
    {
      label: 'Unrevealed',
      value: search ? `${search.next.length} to check${left.length ? `; ${left.length} left` : ''}` : 'none',
      tip: "Mountains the search won't reveal: the cheapest route through each costs at least the route it knows",
    },
    {
      label: 'Hunger',
      value: hunger ? `foraging ${f2(hunger.forage)}, line ${f2(hunger.line)} of ${f2(hunger.eats)} it eats` : 'fed',
      tip: 'What foraging and the line brought on its last hungry turn, against what it eats',
    },
  ];
  const buttons = [];
  if (mine) {
    buttons.push({ label: 'Move to…', arm: 'move' }, { label: 'Route from…', arm: 'start' }, { label: 'Route to…', arm: 'to' });
    if (u.route) buttons.push({ label: 'Clear route', action: unitOrder(u, { type: 'clearRoute' }) });
    if (u.dest != null) buttons.push({ label: 'Stop', action: unitOrder(u, { type: 'move', to: u.tile }) });
    buttons.push({ label: 'Forage', action: unitOrder(u, { type: 'forage' }) }, { label: 'Resupply', action: unitOrder(u, { type: 'resupply' }) });
    buttons.push(...switchButtons(u), ...disband(u));
  }
  let hint = null;
  if (routing.arm === 'move') hint = 'Click the map for where the scout goes; Esc cancels';
  else if (routing.arm === 'start') hint = "Click the map for the route's start; Esc cancels";
  else if (routing.arm === 'to') hint = `Click the map for the route's end (from (${hex(ctx, routing.start ?? u.tile)})); Esc cancels`;
  else if (routing.start != null) hint = `Route from (${hex(ctx, routing.start)}): now Route to…`;
  else if (outcome === 'found') hint = 'Last route: found.';
  else if (outcome === 'nopath') hint = 'Last route: no route can exist.';
  const list = [];
  const unitRows = unitsHere(u, ctx);
  if (unitRows.length) list.push({ label: 'Units here', rows: unitRows });
  if (left.length) {
    list.push({
      label: 'Left unrevealed',
      rows: left.map((m) => ({
        centre: m.tile,
        label: `(${hex(ctx, m.tile)}):`,
        text: `${f2(m.through)} through it${search.known != null ? `, can't beat the known ${f2(search.known)}` : ''}`,
      })),
    });
  }
  return {
    name: `${civName(ctx, u.civ)}'s scout ${u.id}`,
    swatch: v.civs[u.civ]?.color,
    doing,
    numbers,
    list,
    reasons: reasonsOf(ctx, u.id),
    buttons,
    hint,
  };
}

/** A guard's panel (spec item 16, Guard): its people, its hunger and its civ's claim, and Disband….  */
function guardShape(ctx, u) {
  const { v, rules } = ctx;
  const mine = u.civ === v.player;
  const c = ctx.model.claims.get(u.civ);
  const numbers = [
    { label: 'People', value: f2(u.people), tip: 'The people on the claim' },
    { label: 'Hunger', value: u.hungry > 0 ? plural(u.hungry, 'turn') : 'fed', tip: `Turns without food; a person is lost at ${rules.hungry_turns}` },
    ...claimNumbers(ctx, u.civ),
  ];
  const doing = c ? `Guarding ${c.claimed} claimed tiles, holding ${c.held}` : 'Guarding';
  return {
    name: `${civName(ctx, u.civ)}'s guard`,
    swatch: v.civs[u.civ]?.color,
    doing,
    numbers,
    reasons: reasonsOf(ctx, u.id),
    buttons: mine ? disband(u) : [],
  };
}

/** The unaligned clans leaning towards civ `id`, strongest first, up to ten: rows that select their clan. */
function leaningRows(ctx, id) {
  const { v, rules } = ctx;
  return Object.values(v.clans)
    .filter((k) => k.civ == null && (k.leaning[id] ?? 0) > 0)
    .sort((a, b) => b.leaning[id] - a.leaning[id] || a.id - b.id)
    .slice(0, 10)
    .map((k) => {
      const [top, lean] = Object.entries(k.leaning).sort(([, a], [, b]) => b - a)[0];
      const leads = lean > k.leaning[id] ? ` · ${civName(ctx, +top)} leads` : '';
      const starving = k.starving ? ' · starving' : '';
      const share = Math.round((100 * k.leaning[id]) / rules.join);
      return { select: { kind: 'clan', id: k.id }, label: `Clan ${k.id}`, text: `${f2(k.people)} people${starving} · leaning ${share}%${leads}` };
    });
}

/** A civ's panel (spec item 16, Civ): its Standing with the parts in the tooltip, its store, its gifts and
 *  keep, its claim, the leaning clans and its units. Only the player's civ has buttons. Raise… makes a new army
 *  at the capital (warriors or archers), a guard or a scout (build-4). A fallen civ has no buttons. */
function civShape(ctx, c) {
  const { v, goods, rules, painting } = ctx;
  const mine = c.id === v.player;
  if (c.fallen) {
    const [turn, by] = c.fallen;
    return {
      name: c.name,
      swatch: c.color,
      doing: `Fallen to ${civName(ctx, by)} on turn ${turn}`,
      numbers: [],
      list: [],
      reasons: reasonsOf(ctx, c.id),
      buttons: [],
    };
  }
  const s = ctx.model.standing.get(c.id);
  const civs = Object.values(v.civs).length;
  const posture = c.besieged ? '; besieged' : c.pressed ? '; its capital is pressed' : '';
  const doing = `${mine ? `You lead ${c.name}: ` : ''}Standing ${pct(s.total)}, ${ORD(s.rank)} of ${civs}${posture}`;
  const stored = goods.map((g, f) => (c.store[f] > 0.005 ? `${g} ${f2(c.store[f])}` : null)).filter(Boolean);
  const numbers = [
    { label: 'Standing', value: pct(s.total), tip: `Clans and units ${pct(s.clans)}, unaligned clans on its land ${pct(s.land)}` },
    { label: 'People', value: f2(c.people), tip: 'The people in its clans' },
    {
      label: 'Living',
      value: `gathering ${Math.round(c.living.gathering)} · farming ${Math.round(c.living.farming)} · in the army ${Math.round(c.living.army)}`,
      tip: 'Its people by what they do: gathering over their land, working fields, or in its units',
    },
    { label: 'Clans', value: c.clans, tip: 'Clans it holds' },
    {
      label: 'Store',
      value: `${f2(sum(c.store))} of ${rules.store_cap}`,
      tip: `${stored.join(', ') || 'Empty'}${c.lost > 0.005 ? `; lost ${f2(c.lost)} last turn` : ''}`,
    },
    {
      label: 'Last turn',
      value: `kept ${f2(c.keep)} · posted ${f2(c.posted)} · given ${f2(c.given)}`,
      tip: 'Food kept in its store, posted to festivals and given to clans last turn',
    },
    { label: 'Joined', value: c.joined, tip: 'Clans joined since founding' },
    ...claimNumbers(ctx, c.id),
    { label: 'Gift', value: `${f2(c.gift)} a turn`, tip: 'Food a turn given to clans, which draws them to lean to the civ' },
    { label: 'Keep', value: `${f2(c.keep)}${c.keep_set == null ? ' (the default)' : ''}`, tip: 'The food the civ keeps in its store out of posting and gifts' },
  ];
  const buttons = [];
  if (mine) {
    buttons.push({
      label: 'Raise…',
      stepper: {
        options: [
          { label: 'warriors', cap: 10, make: (n) => civOrder(c.id, { type: 'raise', kind: 'camp', fighter: 'warriors', n }) },
          { label: 'archers', cap: 10, make: (n) => civOrder(c.id, { type: 'raise', kind: 'camp', fighter: 'archers', n }) },
          { label: 'a guard', cap: 10, make: (n) => civOrder(c.id, { type: 'raise', n, kind: 'guard' }) },
          { label: 'a scout', cap: 10, make: (n) => civOrder(c.id, { type: 'raise', n, kind: 'scout' }) },
        ],
      },
    });
    buttons.push({ label: 'Gift −1', action: { type: 'setGift', food: Math.max(0, c.gift - 1) } });
    buttons.push({ label: 'Gift +1', action: { type: 'setGift', food: Math.min(rules.gift_max, c.gift + 1) } });
    buttons.push({ label: 'Keep −1', action: { type: 'setKeep', food: Math.max(0, c.keep - 1) } });
    buttons.push({ label: 'Keep +1', action: { type: 'setKeep', food: c.keep + 1 } });
    if (c.keep_set != null) buttons.push({ label: 'Keep default', action: { type: 'setKeep', food: null } });
    buttons.push({ label: 'Claim (P)', paint: 'claim' }, { label: 'Unclaim (U)', paint: 'unclaim' });
  }
  const units = Object.values(v.units).filter((u) => u.civ === c.id).sort((a, b) => a.id - b.id);
  const unitRows = units.map((u) => ({ select: { kind: 'unit', id: u.id }, label: unitName(u), text: `(${hex(ctx, u.tile)})` }));
  let hint = null;
  if (painting === 'claim' || painting === 'unclaim') hint = 'Click or drag on the map; Esc stops';
  return {
    name: c.name,
    swatch: c.color,
    doing,
    numbers,
    list: [
      { label: 'Leaning clans', rows: leaningRows(ctx, c.id), empty: `No clan leans towards ${c.name} yet.` },
      { label: `Units (${units.length})`, rows: unitRows, empty: 'No units.' },
    ],
    reasons: reasonsOf(ctx, c.id),
    buttons,
    hint,
  };
}

/** A clan's panel (spec item 16, Clan): nothing to order, so no buttons. */
function clanShape(ctx, k) {
  const { v, rules } = ctx;
  const civ = k.civ != null ? v.civs[k.civ] : null;
  const cost = k.routes.reduce((a, r) => a + r.cost, 0);
  const now = k.starving
    ? 'Starving'
    : k.market != null
      ? `Walks to festival ${k.market}`
      : 'No festival in reach';
  const farmer = k.fields >= rules.field.farmer - 1e-9;
  const doing = farmer ? `Farms ${shareWords(k.fields)} of its days; ${now.charAt(0).toLowerCase()}${now.slice(1)}` : now;
  const leans = Object.entries(k.leaning)
    .sort(([, a], [, b]) => b - a)
    .map(([civ, x]) => `${civName(ctx, +civ)} ${Math.round((100 * x) / rules.join)}%`);
  const numbers = [
    { label: 'People', value: f2(k.people), tip: 'The people in the clan' },
    { label: 'Fed', value: `${Math.round(100 * k.fed_ratio)}%`, tip: 'The share of its food need the clan met last turn' },
    { label: 'Territory', value: `${k.territory.length} tiles`, tip: 'Tiles of the clan’s territory' },
  ];
  if (k.fields > 0) {
    const days = sum(k.plots.map((p) => p[1]));
    const grain = sum(k.plots.map((p) => p[2]));
    const each = k.plots.map(([t, d, g]) => `(${hex(ctx, t).replace('-', '−')}): ${f1(d)} days, ${f1(g)} grain`);
    numbers.push({
      label: 'Fields',
      value: `${plural(k.plots.length, 'tile')} · ${f1(days)} days · ${f1(grain)} grain`,
      tip: `Days a turn on fields, and the grain they yielded${each.length ? `: ${each.join('; ')}` : ''}`,
    });
  }
  if (k.market != null) numbers.push({ label: 'Route', value: `${f2(cost)} days a turn`, tip: `Days a turn to walk to festival ${k.market}` });
  numbers.push({ label: 'Leanings', value: leans.length ? leans.join(', ') : 'none', tip: `Shares of the joining line (${rules.join}) for each civ` });
  return {
    name: `Clan ${k.id}`,
    swatch: civ?.color,
    doing,
    numbers,
    reasons: reasonsOf(ctx, k.id),
    buttons: [],
  };
}

/** A festival's panel (spec item 16, Festival): its age, traders and activity, its prices in the tooltip, the
 *  clans going, and its walkers drawn on the map (by the view's focus). */
function festivalShape(ctx, m) {
  const { v, goods } = ctx;
  const going = m.clans.map((id) => ({ select: { kind: 'clan', id }, label: `Clan ${id}`, text: `${f2(v.clans[id]?.people ?? 0)} people` }));
  const prices = goods.map((g, f) => `${g} ${f2(m.prices[f])}`);
  const lo = Math.min(...m.prices), hi = Math.max(...m.prices);
  const centre = v.end && v.end.centres.includes(m.id);
  return {
    name: `Festival ${m.id}${centre ? ' (centre)' : ''}`,
    doing: going.length ? `${going.length} clan${going.length === 1 ? '' : 's'} come` : 'No clan goes here',
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
function tileShape(ctx, sel) {
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
  return { name: tileKind(ctx, i), doing, numbers, reasons: [], buttons: [] };
}

/** The panel's shape for a selection, or null if it is gone. */
export function describe(ctx, sel) {
  if (!sel || !alive(ctx, sel)) return null;
  const { v } = ctx;
  const key = keyOf(sel);
  let shape;
  switch (sel.kind) {
    case 'unit': {
      const u = v.units[sel.id];
      shape = u.kind === 'scout' ? scoutShape(ctx, u) : u.kind === 'guard' ? guardShape(ctx, u) : campShape(ctx, u);
      break;
    }
    case 'civ': shape = civShape(ctx, v.civs[sel.civ]); break;
    case 'clan': shape = clanShape(ctx, v.clans[sel.id]); break;
    case 'festival': shape = festivalShape(ctx, v.markets[sel.id]); break;
    default: shape = tileShape(ctx, sel);
  }
  return { ...shape, key };
}
