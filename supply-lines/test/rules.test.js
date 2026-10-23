import assert from 'node:assert/strict';
import { test } from 'node:test';
import { playTurn } from '../src/ai.js';
import { SUPPLY, TERRAIN, UNITS } from '../src/data.js';
import { distance, fromPixel, key, neighbors, toPixel } from '../src/hex.js';
import { newGame, replay } from '../src/game.js';
import { cityYield, depleted, dispatch, legalActions, passable, reachable, spawnUnit, startTurn, supplySource, unitsOf } from '../src/rules.js';

const unitOfType = (s, type) => unitsOf(s, s.current).find((u) => u.type === type);

test('hex pixel round-trip', () => {
  for (const h of [{ q: 0, r: 0 }, { q: 3, r: -2 }, { q: -5, r: 7 }]) {
    const p = toPixel(h, 20);
    assert.deepEqual(fromPixel(p.x, p.y, 20), h);
  }
  assert.ok(neighbors({ q: 2, r: 2 }).every((n) => distance(n, { q: 2, r: 2 }) === 1));
});

test('same seed gives same game', () => {
  assert.deepEqual(newGame({ seed: 42 }), newGame({ seed: 42 }));
  assert.notDeepEqual(newGame({ seed: 42 }).map, newGame({ seed: 43 }).map);
});

test('every player starts with a settler and warrior on land', () => {
  const s = newGame({ seed: 7, players: 4 });
  for (const p of s.players) {
    const types = unitsOf(s, p.id).map((u) => u.type).sort();
    assert.deepEqual(types, ['settler', 'warrior']);
  }
  assert.ok(Object.values(s.units).every((u) => passable(s, u)));
});

test('dispatch does not mutate input and rejects illegal actions', () => {
  const s = newGame({ seed: 1 });
  const snapshot = structuredClone(s);
  const u = unitOfType(s, 'warrior');
  const to = [...reachable(s, u).values()].find((r) => distance(r.hex, u) === 1).hex;
  assert.equal(dispatch(s, { type: 'move', unitId: u.id, to }).error, undefined);
  assert.deepEqual(s, snapshot);
  assert.match(dispatch(s, { type: 'move', unitId: u.id, to: { q: 99, r: 99 } }).error, /not adjacent/);
  assert.match(dispatch(s, { type: 'foundCity', unitId: u.id }).error, /cannot found/);
});

test('founding a city consumes the settler and the city produces units', () => {
  let s = newGame({ seed: 3 });
  const settler = unitOfType(s, 'settler');
  ({ state: s } = dispatch(s, { type: 'foundCity', unitId: settler.id }));
  const city = Object.values(s.cities)[0];
  assert.equal(city.owner, 0);
  assert.equal(s.units[settler.id], undefined);

  for (let i = 0; i < 40 && unitsOf(s, 0).length < 2; i++) ({ state: s } = dispatch(s, { type: 'endTurn' }));
  assert.equal(unitsOf(s, 0).length, 2, 'city should have built a warrior');
  assert.ok(city.stock < UNITS.warrior.cost);
});

test('replaying the action history reproduces the state', () => {
  const config = { seed: 5 };
  let s = newGame(config);
  const history = [];
  const record = (st, a) => {
    const r = dispatch(st, a);
    if (!r.error) history.push(a);
    return r;
  };
  for (let i = 0; i < 20 && s.winner == null; i++) s = playTurn(s, record);
  assert.deepEqual(replay(config, history), s);
});

test('AI games run without illegal states', () => {
  for (const seed of [1, 2, 3]) {
    let s = newGame({ seed, players: 3 });
    while (s.turn <= 40 && s.winner == null) {
      s = playTurn(s, dispatch);
      for (const u of Object.values(s.units)) assert.ok(passable(s, u), `unit on impassable tile (seed ${seed})`);
      const occupied = Object.values(s.units).map(key);
      assert.equal(new Set(occupied).size, occupied.length, `stacked units (seed ${seed})`);
      if (s.turn % 10 === 0) assert.ok(legalActions(s).every((a) => !dispatch(s, a).error));
    }
  }
});

function supplyScenario() {
  let s = newGame({ seed: 3 });
  ({ state: s } = dispatch(s, { type: 'foundCity', unitId: unitOfType(s, 'settler').id }));
  const city = Object.values(s.cities)[0];
  const warrior = unitOfType(s, 'warrior');
  s = structuredClone(s);
  Object.assign(s.units[warrior.id], { q: city.q, r: city.r });
  return { s, city: s.cities[city.id], warrior: s.units[warrior.id] };
}

const freeTiles = (s, pred) => Object.values(s.map.tiles)
  .filter((t) => passable(s, t) && !Object.values(s.units).some((u) => u.q === t.q && u.r === t.r) && pred(t));

test('units eat each turn and starve without food', () => {
  const { s, warrior } = supplyScenario();
  const before = warrior.food;
  startTurn(s);
  assert.equal(warrior.food, before - UNITS.warrior.upkeep);
  warrior.food = 0;
  startTurn(s);
  assert.equal(warrior.hp, UNITS.warrior.hp - SUPPLY.attritionHp);
  for (let i = 0; i < 5; i++) startTurn(s);
  assert.equal(s.units[warrior.id], undefined);
});

test('resupply at home costs 1 labor and draws on the city spare', () => {
  const { s, city, warrior } = supplyScenario();
  warrior.food = 1;
  const spare = city.spare;
  const { state, error } = dispatch(s, { type: 'resupply', unitId: warrior.id });
  assert.equal(error, undefined);
  const got = Math.min(UNITS.warrior.food - 1, spare);
  assert.equal(state.units[warrior.id].food, 1 + got);
  assert.equal(state.units[warrior.id].labor, warrior.labor - 1);
  assert.equal(state.cities[city.id].spare, spare - got);
  assert.match(dispatch(state, { type: 'resupply', unitId: warrior.id }).error ?? 'full', /full|spare/);
});

test('a longer supply line costs more labor, and none means cut off', () => {
  const { s, city, warrior } = supplyScenario();
  const far = freeTiles(s, (t) => distance(t, city) === 3)
    .find((t) => supplySource(s, Object.assign(warrior, { q: t.q, r: t.r })));
  assert.ok(far);
  const src = supplySource(s, warrior);
  assert.ok(src.cost >= 3 && src.labor > 1);
  assert.deepEqual(src.line[0], { q: city.q, r: city.r });
  assert.deepEqual(src.line.at(-1), { q: warrior.q, r: warrior.r });

  Object.assign(warrior, { q: 999, r: 999 });
  assert.equal(supplySource(s, warrior), null);
});

test('foraging strips the tile and fighting costs food', () => {
  const { s, city, warrior } = supplyScenario();
  const plains = freeTiles(s, (t) => t.terrain === 'plains' && distance(t, city) === 1)[0];
  Object.assign(warrior, { q: plains.q, r: plains.r, food: 0 });
  let { state, error } = dispatch(s, { type: 'forage', unitId: warrior.id });
  assert.equal(error, undefined);
  assert.equal(state.units[warrior.id].food, TERRAIN.plains.food);
  assert.ok(depleted(state, plains));
  assert.ok(cityYield(state, city).food < cityYield(s, city).food);
  assert.match(dispatch(state, { type: 'forage', unitId: warrior.id }).error, /stripped|labor/);
  assert.match(dispatch(state, { type: 'pillage', unitId: warrior.id }).error, /not enemy land/);

  const foe = spawnUnit(state, 1, 'warrior', freeTiles(state, (t) => distance(t, plains) === 1)[0]);
  state.units[warrior.id].food = 0;
  assert.match(dispatch(state, { type: 'move', unitId: warrior.id, to: { q: foe.q, r: foe.r } }).error, /no food/);
});
