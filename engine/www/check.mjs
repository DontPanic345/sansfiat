// Checks the data the page relies on, from the wasm build under Node: a medium world run to the end of
// the pre-game. Usage: node www/check.mjs [seed]. Needs www/pkg (./build-web.sh).
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';

const here = fileURLToPath(new URL('.', import.meta.url));
const { initSync, Sim, sizes, terrain, goods, rules } = await import(pathToFileURL(`${here}pkg/web.js`).href);
initSync({ module: readFileSync(`${here}pkg/web_bg.wasm`) });

const SIZES = JSON.parse(sizes());
assert.equal(SIZES.length, 3, 'sizes() has three entries');
assert.deepEqual(SIZES.map((s) => s.name), ['small', 'medium', 'large']);
const TERRAIN = JSON.parse(terrain());
const RULES = JSON.parse(rules());
assert.ok(RULES.tame_turns > 0 && RULES.players === 3);
for (const key of ['unit_labour', 'unit_food', 'hungry_turns', 'store_cap', 'floor']) assert.ok(RULES[key] > 0, `rules.${key}`);
const GOODS = JSON.parse(goods());

const medium = SIZES.find((s) => s.name === 'medium');
const seed = Number(process.argv[2] || 3);
const sim = new Sim(seed, medium.radius);

// The static layout: one column per field, one entry per tile.
const layout = JSON.parse(sim.layout());
const n = layout.q.length;
assert.equal(n, 3 * medium.radius * (medium.radius + 1) + 1, 'a hex of radius R has 3R(R+1)+1 tiles');
for (const key of ['q', 'r', 'terrain', 'fertility', 'river', 'flow', 'lake', 'flood', 'zone', 'pass', 'wild_grain', 'trail_cost', 'span']) {
  assert.ok(Array.isArray(layout[key]) && layout[key].length === n, `layout.${key} has one entry per tile`);
}
assert.equal(layout.radius, medium.radius);
const count = (key, f = (x) => x > 0) => layout[key].filter(f).length;
assert.ok(count('river') > 0, 'the map has rivers');
assert.ok(count('lake') > 0, 'the map has a lake');
assert.ok(layout.terrain.every((t) => t < TERRAIN.length));
assert.ok(layout.flow.every((f, i) => (f === -1) === (layout.river[i] === 0) && f < n), 'river > 0 iff flow, and flow is a tile');
assert.ok(layout.zone.every((z) => z >= 0 && z <= 3));
assert.ok(count('zone') > 0 && count('pass') > 0, 'closed country and passes on a medium map');

// A tile in full.
const t0 = JSON.parse(sim.tile(0));
for (const key of ['q', 'r', 'terrain', 'fertility', 'cap', 'stock', 'river', 'flow', 'lake', 'flood', 'zone', 'pass']) {
  assert.ok(key in t0, `tile carries ${key}`);
}

// Run to the end of the pre-game, pulling events by cursor as the page does.
let cursor = 0;
const events = [];
let turns = 0;
while (!sim.ended() && turns < 600) {
  sim.act('{"type":"endTurn"}');
  turns++;
  const more = JSON.parse(sim.events(cursor));
  cursor += more.length;
  events.push(...more);
}
assert.ok(sim.ended(), 'the pre-game ends');

// The cursor gave exactly what events(0) gives.
const all = JSON.parse(sim.events(0));
assert.deepEqual(all, events, 'events(0) is what the cursor collected');
assert.ok(all.length > 0);
const KINDS = new Set(['clanDivided', 'clanStarving', 'clanMoved', 'clanFarming', 'clanMerged', 'peopleLeft', 'marketFormed', 'marketFaded', 'townFormed', 'hearthTamed', 'centreFormed', 'pregameEnded', 'civFounded', 'campRaised', 'warriorsRaised', 'warriorsDisbanded', 'campResupplied', 'campCutOff', 'campHungry', 'warriorsStarved', 'campForaged', 'campPillaged', 'scoutRouteFound', 'scoutNoRoute', 'scoutTrailBuilt', 'scoutPassFound', 'scoutBlocked', 'scoutHungry', 'scoutStarved', 'clanJoined', 'deployed', 'held', 'lineCut', 'clanLeft', 'claimTaken', 'scoutRaised', 'skirmish', 'retreated', 'tileTaken', 'archersRaised', 'armySurrendered', 'capitalFell']);
let lastTurn = 0;
for (const e of all) {
  assert.ok(KINDS.has(e.kind), `known kind ${e.kind}`);
  assert.ok(Number.isInteger(e.tile) && e.tile >= 0 && e.tile < n, `event tile ${e.tile} is a tile`);
  assert.equal(typeof e.text, 'string');
  assert.ok(e.text.length > 0);
  assert.ok(e.turn >= lastTurn, 'events are in turn order');
  assert.equal(typeof e.alert, 'boolean', 'an event says whether it is an alert');
  lastTurn = e.turn;
}
for (const e of all.filter((e) => e.kind === 'peopleLeft')) {
  assert.ok(e.civ != null && e.amount >= 0.1 && e.text.endsWith('for others'), 'peopleLeft names its civ and amount');
}
assert.equal(JSON.parse(sim.events(all.length)).length, 0, 'nothing past the cursor');
assert.equal(all.filter((e) => e.kind === 'pregameEnded').length, 1, 'one end event');
assert.ok(all.some((e) => e.kind === 'centreFormed'), 'a centre forms');
for (const e of all.filter((e) => e.kind === 'clanMerged')) {
  assert.ok(e.clan != null && e.other != null && e.clan !== e.other, 'a merge names two clans');
  assert.equal(e.text, `Clan ${e.clan} merges into clan ${e.other}`, 'a merge says who joined whom');
}

// Reasons: each subject's, newest first, at most the kept eight; an id with none has an empty list.
const v = JSON.parse(sim.view());
assert.equal(RULES.notes_shown, 5, 'the panel shows five reasons');
let reasoned = 0;
for (const id of Object.keys(v.clans)) {
  const list = JSON.parse(sim.notes(Number(id)));
  assert.ok(list.length <= 8, `clan ${id} keeps at most eight reasons`);
  for (const [k, note] of list.entries()) {
    assert.equal(typeof note.text, 'string');
    assert.ok(note.text.length > 0 && Number.isInteger(note.turn), 'a reason has a turn and text');
    if (k > 0) assert.ok(list[k - 1].turn >= note.turn, `clan ${id}: reasons are newest first`);
  }
  if (list.length) reasoned++;
}
assert.ok(reasoned > 0, 'some clans have reasons after the pre-game');
assert.deepEqual(JSON.parse(sim.notes(999999)), [], 'an id with no reasons has none');

// The per-turn view.
assert.equal(v.turn, turns);
assert.ok(v.end, 'view.end is set after the end');
assert.equal(v.end.centres.length, 3, 'three centres');
assert.deepEqual(v.summary.centres, v.end.clans.filter((c) => c != null), "summary.centres is the end's centre clans after the end");
for (const id of v.end.centres) assert.ok(v.markets[id], `centre ${id} is a market`);
for (const [i, c] of v.claims) assert.ok(i >= 0 && i < n && v.clans[c], 'claims are tile, clan');
for (const i of v.trails) assert.ok(layout.terrain[i] < TERRAIN.length);
for (const [i, tame] of v.tame) assert.ok(i < n && tame > 0);
for (const c of Object.values(v.clans)) assert.ok(c.home < n && c.territory.includes(c.home) && c.territory.every((i) => i < n) && c.routes.every((r) => r.path.every((i) => i < n)));

// Territories: each clan's claims are its territory, connected over its own tiles from its home.
const at = new Map(layout.q.map((q, i) => [`${q},${layout.r[i]}`, i]));
const EDGE = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
const neighbours = (i) => EDGE.map(([dq, dr]) => at.get(`${layout.q[i] + dq},${layout.r[i] + dr}`)).filter((j) => j !== undefined);
const claimOf = new Map(v.claims);
assert.equal(v.claims.length, Object.values(v.clans).reduce((a, c) => a + c.territory.length, 0), 'claims are the territories');
for (const c of Object.values(v.clans)) {
  const held = v.claims.filter(([, k]) => k === c.id).map(([i]) => i).sort((a, b) => a - b);
  assert.deepEqual(held, [...c.territory].sort((a, b) => a - b), `clan ${c.id}: its claims are its territory`);
  const seen = new Set([c.home]);
  const queue = [c.home];
  while (queue.length) {
    for (const j of neighbours(queue.pop())) {
      if (claimOf.get(j) === c.id && !seen.has(j)) {
        seen.add(j);
        queue.push(j);
      }
    }
  }
  assert.equal(seen.size, c.territory.length, `clan ${c.id}: its territory is connected`);
}

// Routes: a clan walks one route, to its festival; the path ends on the festival's tile.
for (const c of Object.values(v.clans)) {
  assert.equal(c.routes.length, c.market == null ? 0 : 1, `clan ${c.id}: one route if it has a festival`);
  for (const r of c.routes) {
    const m = v.markets[c.market];
    assert.ok(m, `clan ${c.id}: its festival is a market`);
    assert.equal(r.to, m.tile, `clan ${c.id}: the route goes to its festival`);
    assert.equal(r.path.length ? r.path.at(-1) : c.home, m.tile, `clan ${c.id}: the path ends on the festival`);
  }
}

// Festivals: each lists the clans going to it, and the civs' claims are land-first: held tiles are claimed.
for (const m of Object.values(v.markets)) {
  assert.deepEqual([...m.clans].sort((a, b) => a - b), Object.values(v.clans).filter((k) => k.market === m.id).map((k) => k.id).sort((a, b) => a - b), `market ${m.id} lists its clans`);
}
for (const c of Object.values(v.civs)) {
  assert.ok(c.land.every((t) => c.claim.includes(t)) && c.land.includes(c.capital), `civ ${c.id}: its land is held claim, capital included`);
}

// Wear: the tiles with some food worn, each food's share and the cap it would have unworn.
const worn = [];
for (let i = 0; i < n; i++) if (JSON.parse(sim.tile(i)).wear.some((x) => x > 0)) worn.push(i);
assert.ok(worn.length > 0, 'some land wears on a medium map');
assert.deepEqual(v.wear.map(([i]) => i), worn, 'wear lists the tiles with some food worn');
for (const [i, wears, unworn] of v.wear) {
  assert.equal(wears.length, GOODS.length);
  assert.equal(unworn.length, GOODS.length);
  const t = JSON.parse(sim.tile(i));
  wears.forEach((x, f) => {
    assert.ok(x >= 0 && x <= 1, `tile ${i}: a wear is a share`);
    assert.ok(Math.abs(t.wear[f] - x) < 1e-12 && Math.abs(t.unworn[f] - unworn[f]) < 1e-12, `tile ${i}: wear matches the tile`);
    assert.ok(Math.abs(t.cap[f] - unworn[f] * (1 - x)) < 1e-9, `tile ${i}: cap is unworn × (1 − wear)`);
  });
}

// ---- the pick, and a unit's life ----------------------------------------------------------------

assert.equal(sim.phase(), 'choose', 'the pre-game ends at the pick');
assert.equal(v.phase, 'choose');
assert.equal(v.player, null);
const cards = JSON.parse(sim.pick());
assert.equal(cards.length, 3, 'three cards');
for (const c of cards) {
  for (const key of ['civ', 'name', 'color', 'capital', 'q', 'r', 'clans', 'people', 'hearths', 'terrain', 'rival_steps', 'rival_by_land']) {
    assert.ok(key in c, `card carries ${key}`);
  }
  assert.ok(c.clans > 0 && c.people > 0 && c.terrain.length > 0 && c.rival_steps > 0);
}
assert.throws(() => sim.act('{"type":"endTurn"}'), /choose/, 'endTurn is refused at the pick');
assert.match(sim.check('{"type":"endTurn"}'), /choose/, 'check names the pick');
assert.match(sim.check(JSON.stringify({ type: 'order', civ: cards[0].civ, order: { type: 'raise', n: 1 } })), /not begun/, 'no orders before the pick');
assert.ok(sim.check('{"nonsense":1}').length > 0, 'check says why an action does not parse');
assert.equal(sim.check(JSON.stringify({ type: 'chooseCiv', civ: cards[0].civ })), '');

// The civs and units the view carries.
const civs = Object.values(v.civs);
assert.equal(civs.length, 3);
for (const c of civs) {
  for (const key of ['id', 'name', 'color', 'market', 'capital', 'store', 'lost', 'people', 'clans', 'claim', 'guard', 'land']) {
    assert.ok(key in c, `civ carries ${key}`);
  }
  assert.equal(c.store.length, GOODS.length);
  assert.ok(c.land.includes(c.capital) && c.claim.includes(c.capital) && c.guard.length === c.claim.length);
  assert.equal(sim.tile_civ(c.capital), c.id, 'a capital belongs to its civ');
  assert.equal(v.clans[Object.keys(v.clans).find((id) => v.clan_civ.some(([k, k2]) => k === Number(id) && k2 === c.id))].civ, c.id);
}
// Standing: each in [0, 1], the sum at most 1, and the parts add up.
{
  let sum = 0;
  for (const c of civs) {
    for (const key of ['standing', 'standing_clans', 'standing_land']) assert.ok(c[key] >= 0 && c[key] <= 1, `${key} is in [0, 1]`);
    assert.ok(Math.abs(c.standing - (c.standing_clans + c.standing_land)) < 1e-12, 'the parts add up');
    sum += c.standing;
  }
  assert.ok(sum <= 1 + 1e-9, 'the Standings sum to at most 1');
  assert.equal(typeof RULES.save_version, 'number', 'rules carry the save version');
}
const camps = Object.values(v.units).filter((u) => u.kind === 'camp');
// A civ starts with its scout and no army (founding.md, The start): the camps come from each civ's own Raise.
assert.equal(camps.length, 0, 'no camp at the pick');
assert.equal(Object.values(v.units).filter((u) => u.kind === 'scout').length, 3, 'a scout each');
// The fight's data (build-4): each fighter's name and strength, FIGHT's entrenchment, hunger and surrender; each
// army's stack by kind sums to its people; each civ's posture; the contested tiles of the last turn.
assert.deepEqual(RULES.fighters.map((f) => f.name), ['warriors', 'archers'], 'rules name the fighters in order');
assert.ok(RULES.fighters.every((f) => f.attack > 0 && f.defence > 0), 'every fighter has a strength');
for (const key of ['entrench', 'entrench_cap', 'hungry', 'surrender']) assert.ok(key in RULES.fight, `rules.fight.${key}`);
assert.ok(RULES.fight.hungry > 0 && RULES.fight.hungry <= 1, 'a hungry army fights at a share of its strength');
assert.ok(Array.isArray(v.contested), 'the view gives the contested tiles');
for (const c of Object.values(v.civs)) {
  assert.equal(typeof c.pressed, 'boolean', 'a civ says whether its capital is pressed');
  assert.equal(typeof c.besieged, 'boolean', 'a civ says whether it is besieged');
  assert.ok(c.fallen === null || (Array.isArray(c.fallen) && c.fallen.length === 2), 'a civ says whether it fell, and when and to whom');
}
for (const u of camps) {
  assert.equal(u.stack.length, 2, 'an army has a stack per fighter');
  assert.ok(Math.abs(u.stack[0] + u.stack[1] - u.people) < 1e-9, "an army's stack sums to its people");
  assert.ok(Number.isInteger(u.still) && typeof u.bows === 'boolean', 'an army has its entrenchment and bows');
}
assert.ok(camps.every((u) => u.line !== undefined && 'cut_off' in u), 'a camp carries its line');
// What the page reads of a camp: its pack, its food in turns, its switches, and whether it is cut off,
// which is its line's carry under its warriors' need (no line, or a line that can't feed it, is cut off).
const checkCamps = (view) => {
  for (const u of Object.values(view.units)) {
    // A guard holds its civ's claim, not a zone, and has no pack: it is checked below.
    if (u.kind === 'guard') {
      assert.deepEqual(u.territory, view.civs[u.civ].claim, 'a guard holds its civ\'s claim');
      assert.equal(u.tile, view.civs[u.civ].capital, 'a guard stands at the capital');
      assert.ok(u.people > 0 && typeof u.hungry === 'number', 'a guard carries its people and hunger');
      continue;
    }
    assert.equal(typeof u.pack_cap, 'number', 'a camp carries its pack cap');
    assert.ok(u.food_turns >= 0 && Number.isFinite(u.food_turns), 'a camp carries its food in turns');
    assert.equal(typeof u.auto_forage, 'boolean', 'a camp carries its forage switch');
    assert.equal(typeof u.auto_supply, 'boolean', 'a camp carries its supply switch');
    assert.equal(u.store.length, GOODS.length, 'a camp carries its pack, a basket');
    assert.equal(
      u.cut_off,
      u.line === null || (u.line.carry !== null && u.line.carry + 1e-9 < u.people * RULES.unit_food),
      'cut off is its line failing to carry its warriors\' food',
    );
    // Its zone and the ground it holds, which the page paints.
    assert.ok(u.territory.includes(u.tile), 'a zone holds the camp\'s tile');
    assert.ok(u.territory.every((t, k) => k === 0 || u.territory[k - 1] < t), 'a zone is sorted');
    assert.ok(u.territory.length <= Math.max(u.zone_cap, 1) + 3, 'a zone keeps near its cap');
    assert.equal(typeof u.zone_cap, 'number', 'a camp carries its zone cap');
    assert.ok(u.held.every((t) => u.territory.includes(t)), 'held tiles are in the zone');
    assert.equal(u.guard.length, u.held.length, 'a guard share for each held tile');
    assert.ok(u.cut_at === null || (u.cut_at.path.includes(u.cut_at.at) && view.civs[u.cut_at.by]), 'cut_at names a tile and a civ');
    if (u.line && u.line.path.length) {
      assert.equal(u.line.path.at(-1), view.civs[u.civ].capital, 'a line ends at its capital');
      assert.ok(u.line.cost > 0 && u.line.path.every((i) => i < n), 'a line is a path of tiles');
    }
  }
  for (const c of Object.values(view.civs)) {
    for (const key of ['raised', 'lost', 'hungry', 'cut_off', 'foraged', 'pillaged', 'resupplied', 'starved', 'held', 'cut_by_army']) {
      assert.ok(key in c.army, `civ carries army.${key}`);
    }
  }
};
checkCamps(v);
assert.equal(sim.tile_civ(-1 >>> 0), -1, 'no civ off the map');

// Joining and giving: the civ's keep and gift, and the clans' leanings. Nobody leads at the pick, so a gift is refused.
assert.ok(RULES.join > 0, 'rules.join is the leaning a clan joins at');
assert.equal(RULES.gift_max, RULES.store_cap, 'a gift is at most the store');
for (const c of civs) {
  for (const key of ['keep', 'gift', 'posted', 'given', 'gifted']) assert.equal(typeof c[key], 'number', `civ carries ${key}`);
  assert.equal(typeof c.joined, 'number', 'civ carries joined');
  assert.ok(c.keep_set === null || typeof c.keep_set === 'number', 'keep_set is a number or null');
  assert.ok(c.keep >= 0 && c.gift >= 0 && c.posted >= 0 && c.given >= 0 && c.gifted >= 0 && c.joined >= 0);
  assert.equal(c.keep_set, null, 'a civ starts on the default keep');
}
for (const k of Object.values(v.clans)) {
  assert.equal(typeof k.leaning, 'object', 'a clan carries its leanings');
  // Every clan leans, a civ's own clans included (`lean`).
  for (const [civ, x] of Object.entries(k.leaning)) {
    assert.ok(v.civs[civ], 'a leaning is to a civ');
    assert.ok(x > 0 && Number.isFinite(x), 'a leaning is a positive number');
  }
}
assert.notEqual(sim.check('{"type":"setGift","food":3}'), '', 'no gift before a civ is led');
assert.notEqual(sim.check('{"type":"setKeep","food":5}'), '', 'no keep before a civ is led');

sim.act(JSON.stringify({ type: 'chooseCiv', civ: cards[0].civ }));
assert.equal(sim.phase(), 'main');
let w = JSON.parse(sim.view());
assert.equal(w.player, cards[0].civ);
const rival = Object.values(w.civs).find((c) => c.id !== w.player);

// The scout: one per civ, at its capital with people and a pack, and no route until one is set.
const scoutOf = (view) => Object.values(view.units).find((u) => u.civ === view.player && u.kind === 'scout');
const scout = scoutOf(w);
// The led civ's order, as the page sends it: the civ, and for a unit's order the unit.
const ordered = (order) => JSON.stringify({ type: 'order', civ: w.player, order });
assert.ok(scout && scout.tile === w.civs[w.player].capital, 'the led civ has a scout at its capital');
assert.ok(scout.people > 0 && scout.store.length === GOODS.length, 'the scout has people and a pack');
assert.equal(scout.route, null, 'the scout has no route yet');
assert.equal(scout.plan, undefined, 'a scout without a route has no plan');
assert.ok(w.revealed.length === 0, 'nothing is revealed at the pick');
const landOf = (i) => TERRAIN[layout.terrain[i]].land;
const water = layout.terrain.findIndex((_, i) => !landOf(i));
assert.ok(water >= 0 && sim.check(ordered({ type: 'route', unit: scout.id, from: scout.tile, to: water })).length > 0, 'a scout does not search water');
assert.ok(sim.check(ordered({ type: 'route', unit: scout.id, from: scout.tile, to: scout.tile })).length > 0, 'a route needs two different tiles');
assert.ok(sim.check(ordered({ type: 'route', unit: scout.id, from: scout.tile, to: n + 3 })).length > 0, 'a route ends on the map');
assert.ok(sim.check(ordered({ type: 'clearRoute', unit: scout.id })) === '', 'a scout can be told to clear its route');

// The leader sets its gift and keep; a gift is refused above the store's cap, or below nothing.
assert.equal(sim.check('{"type":"setGift","food":3}'), '', 'a leader can set its gift');
assert.equal(sim.check('{"type":"setKeep","food":5}'), '', 'a leader can set its keep');
assert.notEqual(sim.check(`{"type":"setGift","food":${RULES.gift_max + 1}}`), '', 'a gift above the cap is refused');
assert.notEqual(sim.check('{"type":"setGift","food":-1}'), '', 'a negative gift is refused');
assert.notEqual(sim.check('{"type":"setKeep","food":-1}'), '', 'a negative keep is refused');
sim.act('{"type":"setGift","food":3}');
sim.act('{"type":"setKeep","food":5}');
w = JSON.parse(sim.view());
// The keep in force is the one set plus the guard's food for `guard_keep_turns` (`civs::keep`).
const guardFood = (view) => {
  const g = Object.values(view.units).find((u) => u.civ === view.player && u.kind === 'guard');
  return g ? g.people * RULES.unit_food * RULES.guard_keep_turns : 0;
};
assert.equal(w.civs[w.player].gift, 3, 'the gift is set');
assert.equal(w.civs[w.player].keep, 5 + guardFood(w), 'the keep in force is set, and the guard\'s food');
assert.equal(w.civs[w.player].keep_set, 5, 'keep_set is the set keep');
sim.act('{"type":"setKeep","food":null}');
w = JSON.parse(sim.view());
assert.equal(w.civs[w.player].keep_set, null, 'keep returns to the default');
assert.ok(w.civs[w.player].keep > 0, 'the default keep is the camp\'s pack');

// Warriors raised: the led civ's first army, from the store's pack and its clans (a civ starts with none).
const armyOf = (view) => Object.values(view.units).filter((u) => u.civ === view.player && u.kind === 'camp');
assert.equal(sim.check(ordered({ type: 'raise', n: 1 })), '', 'the first warriors can be raised');
const before = armyOf(w).reduce((a, u) => a + u.people, 0);
sim.act(ordered({ type: 'raise', n: 1 }));
w = JSON.parse(sim.view());
assert.ok(armyOf(w).reduce((a, u) => a + u.people, 0) > before, 'warriors raised');
const armyView = w;
const unit = armyOf(w)[0];
assert.equal(sim.route(unit.id), 'null', 'no route without a destination');

const hexSteps = (a, b) => {
  const dq = layout.q[a] - layout.q[b], dr = layout.r[a] - layout.r[b];
  return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr));
};
// Watching (all three on the AI, as the page's "Watch all three" sends): a gift is refused while nobody leads.
// Its save is also the control's: the same log under `claims: none`.
const watch = new Sim(seed, medium.radius);
while (!watch.ended()) watch.act('{"type":"endTurn"}');
watch.act('{"type":"chooseCiv","civ":null}');
assert.equal(watch.phase(), 'main');
assert.notEqual(watch.check('{"type":"setGift","food":3}'), '', 'a gift is refused while watching');

// The claim and the guard. A claim is connected land holding the capital, and a refused one says why.
const claimOrder = (civ, tiles) => JSON.stringify({ type: 'order', civ, order: { type: 'claim', tiles } });
const guardOrder = (civ) => JSON.stringify({ type: 'order', civ, order: { type: 'raise', n: 1, kind: 'guard' } });
{
  const c0 = w.civs[w.player];
  assert.ok(c0.claim.every((t, k) => k === 0 || c0.claim[k - 1] < t), 'a claim is sorted');
  assert.ok(c0.land.every((t) => c0.claim.includes(t)), 'land is in the claim');
  const beside = [...Array(n).keys()].find((i) => landOf(i) && !c0.claim.includes(i) && c0.claim.some((t) => hexSteps(i, t) === 1));
  assert.ok(beside !== undefined, 'some land lies beside the claim');
  const wider = [...c0.claim, beside];
  assert.equal(sim.check(claimOrder(w.player, wider)), '', 'a claim one tile wider, still connected, is accepted');
  assert.match(sim.check(claimOrder(w.player, [...wider, water])), /land/, 'a claim over water is refused, with its reason');
  assert.match(sim.check(claimOrder(w.player, [...wider, n + 4])), /not a tile/, 'a claim off the map is refused');
  // A land tile no walk from the capital reaches: a claim that is not connected is refused.
  const cut = [...Array(n).keys()].find((i) => landOf(i) && !wider.includes(i) && sim.check(claimOrder(w.player, [...wider, i])).includes('connected'));
  assert.ok(cut !== undefined, 'some land is cut off from the capital');
  assert.match(sim.check(claimOrder(w.player, [...wider, cut])), /connected to the capital/, 'a claim not connected to the capital is refused');
  sim.act(claimOrder(w.player, wider));
  w = JSON.parse(sim.view());
  assert.deepEqual(w.civs[w.player].claim, [...new Set([...wider, c0.capital])].sort((a, b) => a - b), 'the claim is the order\'s');
  assert.equal(w.civs[w.player].guard.length, w.civs[w.player].claim.length, 'the guard is divided over the claim');
  assert.ok(w.civs[w.player].land.includes(c0.capital), 'the capital is held');
  // The guard: raised from the civ's clans onto its claim, at the capital.
  const before = Object.values(w.units).find((u) => u.civ === w.player && u.kind === 'guard')?.people ?? 0;
  assert.equal(sim.check(guardOrder(w.player)), '', 'a guard is raised from the clans');
  sim.act(guardOrder(w.player));
  w = JSON.parse(sim.view());
  const guard = Object.values(w.units).find((u) => u.civ === w.player && u.kind === 'guard');
  assert.ok(guard && guard.people > before && guard.tile === c0.capital, 'the guard grows at the capital');
  assert.deepEqual(guard.territory, w.civs[w.player].claim, 'the guard holds the claim');
  // The control: with Config.claims none, a claim and a guard are refused with the reason. The watch
  // game's log, without its watching, replays under that config to the pick, and a civ is led.
  const off = JSON.parse(watch.save());
  off.config.claims = 'none';
  off.log = off.log.filter((a) => a.type !== 'chooseCiv');
  const none = Sim.load(JSON.stringify(off));
  const led = Number(Object.keys(JSON.parse(none.view()).civs)[0]);
  none.act(JSON.stringify({ type: 'chooseCiv', civ: led }));
  assert.match(none.check(claimOrder(led, [])), /claims control is off/, 'a claim is refused when the control is off');
  assert.match(none.check(guardOrder(led)), /may not claim/, 'a guard is refused when the control is off');
}
// Order it towards a rival capital, or if no land route goes there, to land 5+ steps away.
let target = rival.capital;
if (JSON.parse(sim.path(unit.tile, target)) === null) {
  target = [...Array(n).keys()].find((i) => hexSteps(i, unit.tile) >= 5 && JSON.parse(sim.path(unit.tile, i)) !== null);
}
assert.ok(target !== undefined, 'some land is reachable');
const go = ordered({ type: 'move', unit: unit.id, to: target });
assert.equal(sim.check(go), '', 'the camp can go there');
const preview = JSON.parse(sim.path(unit.tile, target));
sim.act(go);
const route = JSON.parse(sim.route(unit.id));
assert.ok(route.path.length > 0 && route.path.at(-1) === target, 'the route ends at the destination');
assert.ok(route.cost > 0 && route.turns >= 1);
assert.deepEqual(route, preview, 'the preview is the route');
assert.equal(JSON.parse(sim.path(unit.tile, unit.tile + 1000000)), null);
// The ring: the civ's capital is 0, every share is a number, and a civ with no ring (none, or no civ) is empty.
const ring = JSON.parse(sim.ring(unit.civ));
assert.ok(ring.length > 0, 'the army\'s civ has a ring');
assert.deepEqual(ring.find(([i]) => i === w.civs[unit.civ].capital), [w.civs[unit.civ].capital, 0], 'the capital is 0');
assert.ok(ring.every(([i, share]) => Number.isInteger(i) && share >= 0), 'every ring share is a whole tile and not negative');
assert.deepEqual(JSON.parse(sim.ring(9999)), [], 'no civ has no ring');
assert.ok(sim.check(ordered({ type: 'move', unit: unit.id, to: n + 5 })).length > 0, 'off the map is refused');
// Another civ's unit, and another civ's order, are refused; so is the old action shape, which no longer parses.
const rivalUnit = Object.values(w.units).find((u) => u.civ === rival.id);
assert.ok(rivalUnit, 'the rival has a unit at the pick');
assert.match(sim.check(ordered({ type: 'move', unit: rivalUnit.id, to: target })), /that unit is not yours/, 'a move for a unit that is not yours is refused');
assert.match(sim.check(JSON.stringify({ type: 'order', civ: rival.id, order: { type: 'move', unit: unit.id, to: target } })), /that civ is not yours/, 'an order for a civ the player does not lead is refused');
assert.notEqual(sim.check(JSON.stringify({ type: 'moveCamp', to: target })), '', 'the old moveCamp shape is refused');
assert.throws(() => sim.act(JSON.stringify({ type: 'moveCamp', to: target })), /moveCamp|variant|unknown/i, 'the old moveCamp shape fails to parse');
// A route from the capital three steps out on land, set and read back as the page reads it.
const to = [...Array(n).keys()].find((i) => landOf(i) && hexSteps(i, scout.tile) === 3);
assert.ok(to !== undefined, 'some land lies three steps out');
const set = ordered({ type: 'route', unit: scout.id, from: scout.tile, to });
assert.equal(sim.check(set), '', 'the route is accepted');
sim.act(set);
const routed = scoutOf(JSON.parse(sim.view()));
assert.deepEqual(routed.route, [scout.tile, to], 'the route is the scout\'s');
assert.ok(routed.plan.known && routed.plan.known.path.at(-1) === to, 'the known path ends at the route\'s end');
assert.ok(routed.plan.possible && routed.plan.possible.cost <= routed.plan.known.cost, 'the possible path costs no more than the known');
assert.ok(routed.plan.candidates.every((i) => i < n), 'candidates are tiles');
sim.act(ordered({ type: 'clearRoute', unit: scout.id }));
assert.equal(scoutOf(JSON.parse(sim.view())).route, null, 'the route is cleared');
sim.act(set);
for (let i = 0; i < 10; i++) {
  sim.act('{"type":"endTurn"}');
  const more = JSON.parse(sim.events(cursor));
  cursor += more.length;
  events.push(...more);
}
w = JSON.parse(sim.view());
assert.ok(w.revealed.every((i) => Number.isInteger(i) && i >= 0 && i < n), 'revealed is a list of tiles');
assert.ok(w.trail_work.every(([i, done, need]) => i < n && done > 0 && need > 0), 'trail work is a tile, the labour in and the labour needed');
const onRoute = scoutOf(w);
if (onRoute && onRoute.route) assert.ok(onRoute.plan && onRoute.plan.known, 'a scout with a route carries its plan');
const moved = Object.values(w.units).find((u) => u.civ === w.player && u.kind === 'camp');
checkCamps(w);
assert.ok(!moved || moved.tile !== unit.tile, 'the camp moved in 10 turns (or starved out)');
const again = JSON.parse(sim.route(unit.id));
assert.ok(again === null || again.path.length < route.path.length, 'the route shortens');

// The switches, the zone and the one-shot orders are checked while there is a camp to give them to.
if (moved) {
  sim.act(ordered({ type: 'move', unit: moved.id, to: moved.tile }));
  const here = Object.values(JSON.parse(sim.view()).units).find((u) => u.civ === w.player && u.kind === 'camp');
  assert.notEqual(sim.check(ordered({ type: 'zone', unit: here.id, tiles: [here.tile + 1000000] })), '', 'a zone off the map is refused');
  const capitals = Object.values(w.civs).map((c) => c.capital);
  const first = (here.line?.path ?? []).filter((t) => !capitals.includes(t)).slice(0, 1);
  if (first.length && here.people >= 2) {
    const zone = ordered({ type: 'zone', unit: here.id, tiles: first });
    assert.equal(sim.check(zone), '', 'a zone over the line\'s first tile');
    sim.act(zone);
    sim.act('{"type":"endTurn"}');
    const deployed = Object.values(JSON.parse(sim.view()).units).find((u) => u.civ === w.player);
    if (deployed) {
      assert.deepEqual(deployed.territory, [...new Set([here.tile, first[0]])].sort((a, b) => a - b), 'the zone is set');
      assert.ok(deployed.held.includes(here.tile), 'it holds its own tile');
      checkCamps(JSON.parse(sim.view()));
    }
  }
  assert.equal(sim.check(ordered({ type: 'switches', unit: moved.id, forage: false, supply: true })), '');
  sim.act(ordered({ type: 'switches', unit: moved.id, forage: false, supply: true }));
  const switched = Object.values(JSON.parse(sim.view()).units).find((u) => u.civ === w.player && u.kind === 'camp');
  assert.ok(switched && switched.auto_forage === false && switched.auto_supply === true, 'the switches are set');
  assert.equal(sim.check(ordered({ type: 'resupply', unit: moved.id })), '');
  // Disband it.
  assert.equal(sim.check(ordered({ type: 'disband', unit: moved.id, n: null })), '');
  sim.act(ordered({ type: 'disband', unit: moved.id, n: null }));
  w = JSON.parse(sim.view());
  assert.ok(!Object.values(w.units).some((u) => u.civ === w.player && u.kind === 'camp'), 'the camp is gone');
}
assert.ok(sim.check(ordered({ type: 'disband', unit: unit.id, n: null })).length > 0, 'it cannot be disbanded twice');
const kinds = new Set(JSON.parse(sim.events(0)).map((e) => e.kind));
for (const k of ['civFounded', 'campRaised']) assert.ok(kinds.has(k), `${k} appears`);
for (const e of JSON.parse(sim.events(0))) assert.ok(e.text.length > 0 && KINDS.has(e.kind));

// A save carries its version, and one without it is refused with a reason that names the version.
const save = JSON.parse(sim.save());
assert.equal(save.version, 18, 'a save carries its version');
assert.doesNotThrow(() => Sim.load(JSON.stringify(save)), 'a save of this build loads');
const { version, ...unversioned } = save;
assert.throws(() => Sim.load(JSON.stringify(unversioned)), /version/, 'a save without a version fails, naming it');
assert.throws(() => Sim.load(JSON.stringify({ ...save, version: 1 })), /version 1/, 'a save of another version fails, naming it');


// The panels (build-4): a fallen civ reads its fall with no buttons, and an army's shape names its fighters.
const { describe, subjectOf } = await import(pathToFileURL(`${here}panels.js`).href);
const { panelCtxFor } = await import(pathToFileURL(`${here}check-ctx.mjs`).href);
const panelCtx = panelCtxFor(sim, { rules: RULES, terrain: TERRAIN, goods: GOODS });
const fallenView = { ...v, civs: { ...v.civs } };
const victim = Object.values(v.civs).find((c) => c.id !== v.player);
const other = Object.values(v.civs).find((c) => c.id !== victim.id);
fallenView.civs[victim.id] = { ...victim, fallen: [12, other.id] };
const fallenShape = describe(panelCtx(fallenView), { kind: 'civ', civ: victim.id });
assert.equal(fallenShape.doing, `Fallen to ${other.name} on turn 12`, 'a fallen civ reads its fall');
assert.equal(fallenShape.buttons.length, 0, 'a fallen civ has no buttons');
// The led civ's view after its first warriors were raised above (a civ starts with no army).
const ledId = armyView.player;
const ledView = armyView;
const armyUnit = armyOf(armyView)[0];
const armyShape = describe(panelCtx(ledView), { kind: 'unit', id: armyUnit.id });
assert.match(armyShape.name, /'s army \d+$/, 'an army is named by its civ and id');
assert.ok(armyShape.numbers.some((n) => n.label === 'Archers') && armyShape.numbers.some((n) => n.label === 'Attack'), 'an army gives its archers and attack');
const raise = armyShape.buttons.find((b) => b.stepper && b.label === 'Raise…');
assert.deepEqual(raise.stepper.options.map((o) => o.label), ['warriors', 'archers'], 'an army raises warriors or archers into itself');
const civShp = describe(panelCtx(ledView), { kind: 'civ', civ: ledId });
const civRaise = civShp.buttons.find((b) => b.stepper && b.label === 'Raise…');
assert.deepEqual(civRaise.stepper.options.map((o) => o.label), ['warriors', 'archers', 'a guard', 'a scout'], 'a civ raises warriors, archers, a guard or a scout');
// A feed item of a clan's fields selects the clan, as a clan's move does.
const someClan = Object.values(v.clans)[0];
for (const kind of ['clanMoved', 'clanFarming']) {
  assert.deepEqual(subjectOf({ v }, { kind, clan: someClan.id, tile: someClan.home }), { kind: 'clan', id: someClan.id }, `a ${kind} item selects its clan`);
}

await import('./check-stage.mjs');

console.log(`ok: medium seed ${seed}, ${n} tiles, ended turn ${v.end.turn}${v.end.by_limit ? ' by the limit' : ''}, ${all.length} events, ${Object.keys(v.clans).length} clans`);
