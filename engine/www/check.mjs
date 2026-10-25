// Checks the data the page relies on, from the wasm build under Node: a medium world run to the end of
// the pre-game. Usage: node www/check.mjs [seed]. Needs www/pkg (./build-web.sh).
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';

const here = fileURLToPath(new URL('.', import.meta.url));
const { initSync, Sim, sizes, terrain, rules } = await import(pathToFileURL(`${here}pkg/web.js`).href);
initSync({ module: readFileSync(`${here}pkg/web_bg.wasm`) });

const SIZES = JSON.parse(sizes());
assert.equal(SIZES.length, 3, 'sizes() has three entries');
assert.deepEqual(SIZES.map((s) => s.name), ['small', 'medium', 'large']);
const TERRAIN = JSON.parse(terrain());
const RULES = JSON.parse(rules());
assert.ok(RULES.tame_turns > 0 && RULES.players === 3);

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
for (const c of Object.values(v.clans)) assert.ok(c.home < n && c.round.every((i) => i < n));

console.log(`ok: medium seed ${seed}, ${n} tiles, ended turn ${v.end.turn}${v.end.by_limit ? ' by the limit' : ''}, ${all.length} events, ${Object.keys(v.clans).length} clans`);
