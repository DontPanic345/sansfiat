import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/supply/rules.js';
import { newGame } from '../src/game.js';
import { spawnUnit } from '../src/core.js';
import { SUPPLY } from '../src/modules/supply/data.js';
SUPPLY.eatPerTroop = 1; // tests count food per troop

const step = (phase) => mod.turnSteps.find((t) => t.phase === phase);
const act = (s, a) => { const x = mod.actions[a.type]; const e = x.validate(s, a); if (!e) x.apply(s, a); return e; };

// A flat plains strip q=-2..16 on rows -1..1; civ 1 (human) city at (0,0) with food.
function world() {
  const s = newGame({ seed: 1 });
  s.phase = 'main';
  s.clans = {}; s.cities = {}; s.units = {};
  for (const t of Object.values(s.map.tiles)) { t.cache = null; t.held = null; t.fort = null; t.fallow = 0; t.claim = null; }
  for (let q = -2; q <= 16; q++) {
    for (const r of [-1, 0, 1]) {
      const t = s.map.tiles[`${q},${r}`];
      if (t) { t.terrain = 'plains'; t.trail = false; t.stock = { grain: 4 }; }
    }
  }
  s.civs = { 1: { id: 1, name: 'Red', isHuman: true, store: { grain: 100 }, cityIds: [100] }, 2: { id: 2, name: 'Blue', store: {}, cityIds: [] } };
  s.cities = { 100: { id: 100, owner: 1, q: 0, r: 0 } };
  s.events = [];
  return s;
}
const captain = (s, q, extra = {}) => spawnUnit(s, { owner: 1, type: 'captain', q, r: 0, troops: 5, zone: [], ...extra });
const alerts = (s) => s.events.filter((e) => e.includes(': !'));

test('a new captain gets no free food', () => {
  const s = world();
  const c = captain(s, 3);
  assert.equal(mod.ensure(c).food, 0);
  assert.equal(c.supply.foodMax, 20);
});

test('armies eat from the captain store; hungry flag; healing', () => {
  const s = world();
  const c = captain(s, 3, { troopsMax: 8, supply: { food: 20, foodMax: 20, auto: false } });
  const d = spawnUnit(s, { owner: 1, type: 'detachment', q: 4, r: 0, captainId: c.id, troops: 3 });
  step('armies').run(s);
  assert.equal(c.supply.food, 15);
  assert.equal(c.troops, 6, 'fed army heals');
  c.supply.food = 2;
  step('armies').run(s);
  assert.ok(c.hungry && d.hungry);
  assert.ok(alerts(s).some((e) => /hungry/.test(e)));
  c.supply.food = 20;
  step('armies').run(s);
  assert.equal(c.hungry, false);
});

test('fighting costs food', () => {
  const s = world();
  const c = captain(s, 3, { supply: { food: 20, foodMax: 20, auto: false }, army: { foughtTurn: s.turn, foughtTroops: 4 } });
  c.troops = 5;
  step('armies').run(s);
  assert.equal(c.supply.food, 20 - 5 - 4 * SUPPLY.fightEat);
});

test('starvation takes ~10% of the whole army a turn, spread over its detachments', () => {
  const s = world();
  const c = captain(s, 3, { troops: 20 });
  const ds = [5, 5, 5, 5].map((n, i) => spawnUnit(s, { owner: 1, type: 'detachment', q: 3 + i, r: 0, captainId: c.id, troops: n }));
  step('armies').run(s);
  assert.equal(c.troops, 18, 'lost 2 of 20, not half');
  assert.equal(ds.reduce((a, d) => a + (s.units[d.id] ? d.troops : 0), 0), 18);
  assert.ok(ds.every((d) => d.troops >= 4));
  assert.equal(s.mods.supply.starved, 2);
  const small = captain(s, 8, { troops: 3 });
  step('armies').run(s);
  assert.equal(small.troops, 2, 'at least one a turn');
  assert.ok(alerts(s).some((e) => /lost 1 troops to hunger/.test(e)));
});

test('resupply draws along a line, costs labour, and reduces the city store', () => {
  const s = world();
  const c = captain(s, 6);
  const src = mod.supplySource(s, c);
  assert.equal(src.kind, 'city');
  assert.equal(src.cost, 6);
  assert.equal(src.labour, 3);
  assert.equal(act(s, { type: 'resupply', civ: 1, unit: c.id }), null);
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
  mod.ensure(c);
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
  assert.ok(alerts(s).some((e) => /cache .* lost/.test(e)));
});

test('forage on own/neutral land, pillage on enemy land, decided by the land under the troops', () => {
  const s = world();
  const c = captain(s, 7);
  assert.match(act(s, { type: 'pillage', civ: 1, unit: c.id }), /not enemy land/);
  assert.equal(act(s, { type: 'forage', civ: 1, unit: c.id }), null);
  assert.equal(c.supply.food, 4);
  assert.equal(s.map.tiles['7,0'].fallow, 5);
  c.labour = 4;
  assert.match(act(s, { type: 'forage', civ: 1, unit: c.id }), /stripped/);
  // an enemy city's territory, with our troops standing on it (held by us)
  s.cities[200] = { id: 200, owner: 2, q: 13, r: 0 };
  const e = captain(s, 12);
  s.map.tiles['12,0'].held = 1;
  assert.equal(mod.landOwner(s, e), 2);
  assert.match(act(s, { type: 'forage', civ: 1, unitId: e.id }), /enemy land/);
  assert.equal(act(s, { type: 'pillage', civ: 1, unitId: e.id }), null);
  assert.equal(e.labour, 3);
  assert.equal(e.supply.food, 4);
  // a clan's claim makes the land its civ's
  const k = spawnUnit(s, { owner: 1, type: 'captain', q: 5, r: 1, troops: 5, zone: [] });
  s.clans[300] = { id: 300, owner: 2, q: 5, r: 1, pop: 5 };
  s.map.tiles['5,1'].claim = 300;
  delete s.cities[200];
  assert.equal(mod.landOwner(s, k), 2);
  assert.equal(act(s, { type: 'pillage', civ: 1, unit: k.id }), null);
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

test('porter: no route into enemy land; if the target turns hostile it turns back and returns the food', () => {
  const s = world();
  act(s, { type: 'buildPorter', civ: 1, city: 100 });
  const p = Object.values(s.units).find((u) => u.type === 'porter');
  s.map.tiles['6,0'].held = 2;
  assert.match(act(s, { type: 'setPorterRoute', civ: 1, unit: p.id, to: { q: 6, r: 0 } }), /enemy land/);
  s.map.tiles['6,0'].held = null;
  assert.equal(act(s, { type: 'setPorterRoute', civ: 1, unit: p.id, q: 6, r: 0 }), null, 'city defaults to the nearest; q,r accepted');
  p.labour = p.labourMax; step('orders').run(s);
  assert.ok(p.supply.food > 0 && p.q > 0, 'loaded and walking');
  s.map.tiles['6,0'].held = 2;
  const grain = s.civs[1].store.grain;
  for (let i = 0; i < 6; i++) { p.labour = p.labourMax; step('orders').run(s); }
  assert.equal(p.supply.route, null);
  assert.ok(p.q === 0 && p.r === 0, 'back home');
  assert.ok(s.civs[1].store.grain > grain, 'unspent food returned to the store');
  assert.ok(alerts(s).some((e) => /turned back/.test(e)));
});

test('unloadPorter refuses to cache on enemy land but feeds a captain there', () => {
  const s = world();
  const p = spawnUnit(s, { owner: 1, type: 'porter', q: 6, r: 0, supply: { food: 10, foodMax: 12, route: null } });
  s.cities[200] = { id: 200, owner: 2, q: 7, r: 0 };
  assert.match(act(s, { type: 'unloadPorter', civ: 1, unit: p.id }), /enemy land/);
  const c = captain(s, 6);
  assert.equal(act(s, { type: 'unloadPorter', civ: 1, unit: p.id }), null);
  assert.equal(c.supply.food, 10);
});

test('ai: auto-supply on, forage when low, and a porter for a far army', () => {
  const s = world();
  const c = captain(s, 14);
  let acts = mod.ai(s, 1);
  assert.ok(acts.some((a) => a.type === 'autoSupply'));
  assert.ok(acts.some((a) => a.type === 'forage'));
  const b = acts.find((a) => a.type === 'buildPorter');
  assert.ok(b);
  act(s, b);
  acts = mod.ai(s, 1);
  const r = acts.find((a) => a.type === 'setPorterRoute');
  assert.ok(r && r.to.q === c.q && r.for === c.id);
  assert.equal(act(s, r), null);
});
