import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/supply/rules.js';
import { newGame } from '../src/game.js';
import { spawnUnit } from '../src/core.js';
import { SUPPLY } from '../src/modules/supply/data.js';
SUPPLY.eatPerTroop = 1; // tests count food per troop

const step = (phase) => mod.turnSteps.find((t) => t.phase === phase);
const act = (s, a) => { const x = mod.actions[a.type]; const e = x.validate(s, a); if (!e) x.apply(s, a); return e; };

// A flat plains strip q=-2..16 on rows -1..1; civ 1 city at (0,0) with food.
function world() {
  const s = newGame({ seed: 1 });
  s.phase = 'main';
  s.clans = {}; s.cities = {}; s.units = {};
  for (const t of Object.values(s.map.tiles)) { t.cache = null; t.held = null; t.fort = null; t.fallow = 0; }
  for (let q = -2; q <= 16; q++) {
    for (const r of [-1, 0, 1]) {
      const t = s.map.tiles[`${q},${r}`];
      if (t) { t.terrain = 'plains'; t.trail = false; t.stock = { grain: 4 }; }
    }
  }
  s.civs = { 1: { id: 1, store: { grain: 100 }, cityIds: [100] }, 2: { id: 2, store: {}, cityIds: [] } };
  s.cities = { 100: { id: 100, owner: 1, q: 0, r: 0 } };
  return s;
}
const captain = (s, q, extra = {}) => spawnUnit(s, { owner: 1, type: 'captain', q, r: 0, troops: 5, zone: [], ...extra });

test('armies eat from the captain store; hungry flag and attrition; healing', () => {
  const s = world();
  const c = captain(s, 3, { troopsMax: 8 });
  const d = spawnUnit(s, { owner: 1, type: 'detachment', q: 4, r: 0, captainId: c.id, troops: 3 });
  mod.ensure(c);
  assert.equal(c.supply.food, 20);
  step('armies').run(s);
  assert.equal(c.supply.food, 15);
  assert.equal(c.troops, 6, 'fed army heals');
  c.supply.food = 2;
  step('armies').run(s);
  assert.ok(c.hungry && d.hungry);
  assert.ok(c.troops < 6);
  assert.ok(d.troops < 3);
  c.supply.food = 20;
  step('armies').run(s);
  assert.equal(c.hungry, false);
});

test('resupply draws along a line, costs labour, and reduces the city store', () => {
  const s = world();
  const c = captain(s, 6);
  mod.ensure(c); c.supply.food = 0;
  const src = mod.supplySource(s, c);
  assert.equal(src.kind, 'city');
  assert.equal(src.cost, 6);
  assert.equal(src.labour, 3);
  assert.equal(act(s, { type: 'resupply', civ: 1, unitId: c.id }), null);
  assert.equal(c.labour, 1);
  assert.equal(c.supply.food, 20);
  assert.equal(s.civs[1].store.grain, 80, 'feeding the army cuts the city store');
  const far = captain(s, 14);
  assert.equal(mod.supplySource(s, far), null, 'beyond range 10');
  assert.ok(act(s, { type: 'resupply', civ: 1, unitId: far.id }));
});

test('autoSupply tops up in the armies phase; lines cannot cross enemy land', () => {
  const s = world();
  const c = captain(s, 3);
  act(s, { type: 'autoSupply', civ: 1, unitId: c.id, on: true });
  c.supply.food = 3;
  step('armies').run(s);
  assert.equal(c.supply.food, 20 - 5);
  for (const t of Object.values(s.map.tiles)) if (t.q === 1 || t.q === 2) t.held = 2;
  c.supply.food = 0;
  assert.equal(mod.supplySource(s, c), null);
});

test('cache: nearest source, spoils 30% a turn, captured with the tile', () => {
  const s = world();
  s.map.tiles['8,0'].cache = { owner: 1, food: 100 };
  const c = captain(s, 9);
  mod.ensure(c); c.supply.food = 0;
  assert.equal(mod.supplySource(s, c).kind, 'cache');
  act(s, { type: 'resupply', civ: 1, unitId: c.id });
  assert.equal(c.supply.food, 20);
  assert.equal(s.map.tiles['8,0'].cache.food, 80);
  c.supply.food = 99;
  step('armies').run(s);
  assert.ok(Math.abs(s.map.tiles['8,0'].cache.food - 56) < 1e-9);
  s.map.tiles['8,0'].held = 2;
  step('armies').run(s);
  assert.equal(s.map.tiles['8,0'].cache.owner, 2);
});

test('forage on own/neutral land and pillage on enemy land strip the tile for 5 turns', () => {
  const s = world();
  const c = captain(s, 7);
  mod.ensure(c); c.supply.food = 0;
  assert.ok(act(s, { type: 'pillage', civ: 1, unitId: c.id }), 'not enemy land');
  assert.equal(act(s, { type: 'forage', civ: 1, unitId: c.id }), null);
  assert.equal(c.supply.food, 4);
  assert.equal(s.map.tiles['7,0'].fallow, 5);
  c.labour = 4;
  assert.ok(act(s, { type: 'forage', civ: 1, unitId: c.id }), 'stripped');
  const e = captain(s, 12);
  mod.ensure(e); e.supply.food = 0;
  s.map.tiles['12,0'].held = 2;
  assert.ok(act(s, { type: 'forage', civ: 1, unitId: e.id }), 'enemy land');
  assert.equal(act(s, { type: 'pillage', civ: 1, unitId: e.id }), null);
  assert.equal(e.labour, 3);
  assert.equal(e.supply.food, 4);
});

test('porter keeps a cache stocked and eats from its load', () => {
  const s = world();
  assert.equal(act(s, { type: 'buildPorter', civ: 1, cityId: 100 }), null);
  const p = Object.values(s.units).find((u) => u.type === 'porter');
  assert.equal(s.civs[1].store.grain, 99);
  assert.equal(act(s, { type: 'setPorterRoute', civ: 1, unitId: p.id, cityId: 100, to: { q: 4, r: 0 }, target: 10 }), null);
  for (let i = 0; i < 12; i++) { p.labour = p.labourMax; step('orders').run(s); }
  const cache = s.map.tiles['4,0'].cache;
  assert.ok(cache && cache.owner === 1 && cache.food > 0, 'cache stocked');
  assert.ok(s.civs[1].store.grain < 99, 'porter took food from the city');
  const q = spawnUnit(s, { owner: 1, type: 'porter', q: 1, r: 1, supply: { food: 5, foodMax: 12, route: null } });
  step('orders').run(s);
  assert.equal(q.supply.food, 4);
});

test('ai: auto-supply on, and a porter for a far army', () => {
  const s = world();
  const c = captain(s, 14);
  let acts = mod.ai(s, 1);
  assert.ok(acts.some((a) => a.type === 'autoSupply'));
  const b = acts.find((a) => a.type === 'buildPorter');
  assert.ok(b);
  act(s, b);
  acts = mod.ai(s, 1);
  const r = acts.find((a) => a.type === 'setPorterRoute');
  assert.ok(r && r.to.q === c.q);
  assert.equal(act(s, r), null);
});
