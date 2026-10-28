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
for (const key of ['unit_labour', 'unit_food', 'hungry_turns', 'store_cap', 'territory']) assert.ok(RULES[key] > 0, `rules.${key}`);
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
const KINDS = new Set(['clanDivided', 'clanStarving', 'clanMoved', 'marketFormed', 'marketFaded', 'hearthTamed', 'centreFormed', 'pregameEnded', 'civFounded', 'unitMade', 'unitHungry', 'unitDisbanded', 'unitStarved']);
let lastTurn = 0;
for (const e of all) {
  assert.ok(KINDS.has(e.kind), `known kind ${e.kind}`);
  assert.ok(Number.isInteger(e.tile) && e.tile >= 0 && e.tile < n, `event tile ${e.tile} is a tile`);
  assert.equal(typeof e.text, 'string');
  assert.ok(e.text.length > 0);
  assert.ok(e.turn >= lastTurn, 'events are in turn order');
  lastTurn = e.turn;
}
assert.equal(JSON.parse(sim.events(all.length)).length, 0, 'nothing past the cursor');
assert.equal(all.filter((e) => e.kind === 'pregameEnded').length, 1, 'one end event');
assert.ok(all.some((e) => e.kind === 'centreFormed'), 'a centre forms');

// The per-turn view.
const v = JSON.parse(sim.view());
assert.equal(v.turn, turns);
assert.ok(v.end, 'view.end is set after the end');
assert.equal(v.end.centres.length, 3, 'three centres');
assert.deepEqual(v.summary.centres, v.end.centres, 'summary.centres is end.centres after the end');
for (const id of v.end.centres) assert.ok(v.markets[id], `centre ${id} is a market`);
for (const [i, c] of v.claims) assert.ok(i >= 0 && i < n && v.clans[c], 'claims are tile, clan');
for (const i of v.trails) assert.ok(layout.terrain[i] < TERRAIN.length);
for (const [i, tame] of v.tame) assert.ok(i < n && tame > 0);
for (const c of Object.values(v.clans)) assert.ok(c.home < n && c.territory.includes(c.home) && c.territory.every((i) => i < n) && c.routes.every((r) => r.path.every((i) => i < n)));

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
assert.match(sim.check('{"type":"makeUnit"}'), /not begun/, 'no orders before the pick');
assert.ok(sim.check('{"nonsense":1}').length > 0, 'check says why an action does not parse');
assert.equal(sim.check(JSON.stringify({ type: 'chooseCiv', civ: cards[0].civ })), '');

// The civs and units the view carries.
const civs = Object.values(v.civs);
assert.equal(civs.length, 3);
for (const c of civs) {
  for (const key of ['id', 'name', 'color', 'market', 'capital', 'store', 'lost', 'people', 'clans', 'territory', 'work']) {
    assert.ok(key in c, `civ carries ${key}`);
  }
  assert.equal(c.store.length, GOODS.length);
  assert.ok(c.territory.includes(c.capital) && c.work.includes(c.capital));
  assert.equal(sim.tile_civ(c.capital), c.id, 'a capital belongs to its civ');
  assert.equal(v.clans[Object.keys(v.clans).find((id) => v.clan_civ.some(([k, k2]) => k === Number(id) && k2 === c.id))].civ, c.id);
}
assert.equal(Object.values(v.units).length, 3, 'a warrior each');
assert.ok(Object.values(v.units).every((u) => u.kind === 'warrior' && u.tile === v.civs[u.civ].capital));
assert.equal(sim.tile_civ(-1 >>> 0), -1, 'no civ off the map');

sim.act(JSON.stringify({ type: 'chooseCiv', civ: cards[0].civ }));
assert.equal(sim.phase(), 'main');
let w = JSON.parse(sim.view());
assert.equal(w.player, cards[0].civ);
const rival = Object.values(w.civs).find((c) => c.id !== w.player);

// A warrior made, if the civ can spare a person.
const made = sim.check('{"type":"makeUnit"}');
assert.ok(made === '' || made.length > 0);
if (made === '') {
  sim.act('{"type":"makeUnit"}');
  w = JSON.parse(sim.view());
  assert.equal(Object.values(w.units).filter((u) => u.civ === w.player).length, 2, 'a warrior made');
}
const unit = Object.values(w.units).filter((u) => u.civ === w.player).pop();
assert.equal(sim.route(unit.id), 'null', 'no route without a destination');

// Order it towards a rival capital, or if no land route goes there, to land 5+ steps away.
const hexSteps = (a, b) => {
  const dq = layout.q[a] - layout.q[b], dr = layout.r[a] - layout.r[b];
  return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr));
};
let target = rival.capital;
if (JSON.parse(sim.path(unit.tile, target)) === null) {
  target = [...Array(n).keys()].find((i) => hexSteps(i, unit.tile) >= 5 && JSON.parse(sim.path(unit.tile, i)) !== null);
}
assert.ok(target !== undefined, 'some land is reachable');
const go = JSON.stringify({ type: 'moveUnit', unit: unit.id, to: target });
assert.equal(sim.check(go), '', 'the unit can go there');
const preview = JSON.parse(sim.path(unit.tile, target));
sim.act(go);
const route = JSON.parse(sim.route(unit.id));
assert.ok(route.path.length > 0 && route.path.at(-1) === target, 'the route ends at the destination');
assert.ok(route.cost > 0 && route.turns >= 1);
assert.deepEqual(route, preview, 'the preview is the route');
assert.equal(JSON.parse(sim.path(unit.tile, unit.tile + 1000000)), null);
assert.ok(sim.check(JSON.stringify({ type: 'moveUnit', unit: unit.id, to: n + 5 })).length > 0, 'off the map is refused');
for (let i = 0; i < 10; i++) {
  sim.act('{"type":"endTurn"}');
  const more = JSON.parse(sim.events(cursor));
  cursor += more.length;
  events.push(...more);
}
w = JSON.parse(sim.view());
assert.notEqual(w.units[unit.id]?.tile, unit.tile, 'the unit moved in 10 turns');
const again = JSON.parse(sim.route(unit.id));
assert.ok(again === null || again.path.length < route.path.length, 'the route shortens');

// Disband it.
assert.equal(sim.check(JSON.stringify({ type: 'disbandUnit', unit: unit.id })), '');
sim.act(JSON.stringify({ type: 'disbandUnit', unit: unit.id }));
w = JSON.parse(sim.view());
assert.ok(!(unit.id in w.units), 'the unit is gone');
assert.ok(sim.check(JSON.stringify({ type: 'disbandUnit', unit: unit.id })).length > 0, 'it cannot be disbanded twice');
const kinds = new Set(JSON.parse(sim.events(0)).map((e) => e.kind));
for (const k of ['civFounded', 'unitMade', 'unitDisbanded']) assert.ok(kinds.has(k), `${k} appears`);
for (const e of JSON.parse(sim.events(0))) assert.ok(e.text.length > 0 && KINDS.has(e.kind));

console.log(`ok: medium seed ${seed}, ${n} tiles, ended turn ${v.end.turn}${v.end.by_limit ? ' by the limit' : ''}, ${all.length} events, ${Object.keys(v.clans).length} clans`);
