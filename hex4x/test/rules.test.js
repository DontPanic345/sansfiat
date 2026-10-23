import assert from 'node:assert/strict';
import { test } from 'node:test';
import { playTurn } from '../src/ai.js';
import { UNITS } from '../src/data.js';
import { distance, fromPixel, key, neighbors, toPixel } from '../src/hex.js';
import { newGame, replay } from '../src/game.js';
import { cityYield, dispatch, legalActions, passable, reachable, spawnUnit, startTurn, unitsOf } from '../src/rules.js';

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
  Object.assign(warrior, { q: city.q, r: city.r });
  s = structuredClone(s);
  return { s, city: s.cities[city.id], warrior: s.units[warrior.id] };
}

test('a warrior at home keeps full labor and the city funds its upkeep', () => {
  const { s, city, warrior } = supplyScenario();
  startTurn(s);
  assert.equal(warrior.supplier, city.id);
  assert.equal(warrior.movesLeft, UNITS.warrior.moves);
  assert.equal(city.spare, cityYield(s, city).prod - UNITS.warrior.upkeep);
});

test('labor drops with supply-line length and a warrior with no line is crippled', () => {
  const { s, city, warrior } = supplyScenario();
  const far = Object.keys(s.map.tiles).map((k) => s.map.tiles[k])
    .filter((t) => passable(s, t) && distance(t, city) === 4 && !Object.values(s.units).some((u) => u.q === t.q && u.r === t.r));
  assert.ok(far.length);
  Object.assign(warrior, { q: far[0].q, r: far[0].r });
  startTurn(s);
  assert.ok(warrior.movesLeft < UNITS.warrior.moves);

  Object.assign(warrior, { q: 999, r: 999 });
  startTurn(s);
  assert.equal(warrior.supplier, null);
  assert.equal(warrior.movesLeft, 1);
});

test('a city out of supply leaves later warriors unsupplied and builds nothing', () => {
  const { s, city } = supplyScenario();
  const free = [city, ...neighbors(city)].filter((h) => passable(s, h) && !Object.values(s.units).some((u) => u.q === h.q && u.r === h.r));
  const prod = cityYield(s, city).prod;
  const extras = Array.from({ length: prod }, (_, i) => spawnUnit(s, 0, 'warrior', free[i % free.length]));
  const before = city.stock;
  startTurn(s);
  assert.ok(extras.some((u) => u.supplier === null));
  assert.equal(city.spare, 0);
  assert.equal(city.stock, before);
});
