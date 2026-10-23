import assert from 'node:assert/strict';
import { test } from 'node:test';
import { playTurn } from '../src/ai.js';
import { UNITS } from '../src/data.js';
import { distance, fromPixel, key, neighbors, toPixel } from '../src/hex.js';
import { newGame, replay } from '../src/game.js';
import { cityYield, defenderAt, dispatch, legalActions, passable, reachable, routePlan, spawnUnit, startTurn, unitsOf } from '../src/rules.js';

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

test('own units stack, rival units block or fight', () => {
  const s = newGame({ seed: 1 });
  const [warrior, settler] = [unitOfType(s, 'warrior'), unitOfType(s, 'settler')];
  const { state, error } = dispatch(s, { type: 'move', unitId: warrior.id, to: { q: settler.q, r: settler.r } });
  assert.equal(error, undefined);
  assert.equal(Object.values(state.units).filter((u) => u.q === settler.q && u.r === settler.r).length, 2);

  const rival = spawnUnit(s, 1, 'warrior', neighbors(settler).find((n) => passable(s, n) && !defenderAt(s, n, 1)));
  assert.equal(defenderAt(s, rival, 0).id, rival.id);
  assert.ok(!reachable(s, settler).has(key(rival)));
  assert.match(dispatch(s, { type: 'move', unitId: settler.id, to: { q: rival.q, r: rival.r } }).error, /cannot attack/);
});

test('units are named by type and number per player', () => {
  const s = newGame({ seed: 1, players: 2 });
  const extra = spawnUnit(s, 0, 'warrior', unitOfType(s, 'warrior'));
  assert.deepEqual(unitsOf(s, 0).map((u) => u.name).sort(), ['Settler 1', 'Warrior 1', 'Warrior 2']);
  assert.equal(extra.name, 'Warrior 2');
  assert.deepEqual(unitsOf(s, 1).map((u) => u.name).sort(), ['Settler 1', 'Warrior 1']);
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
      const owners = new Map();
      for (const u of Object.values(s.units)) {
        assert.ok((owners.get(key(u)) ?? u.owner) === u.owner, `rival units share a tile (seed ${seed})`);
        owners.set(key(u), u.owner);
      }
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

// An 8x3 plains strip, Red city at the west end and Blue city at the east, and a Red scout.
// Column q=4 is the wall: `wall` gives its three tiles' extra fields.
function strip(wall) {
  const s = newGame({ seed: 1 });
  s.map.tiles = {};
  for (let q = 0; q < 8; q++)
    for (let r = 0; r < 3; r++) s.map.tiles[`${q},${r}`] = { q, r, terrain: 'plains' };
  wall.forEach((t, r) => Object.assign(s.map.tiles[`4,${r}`], t));
  s.units = {};
  s.cities = {
    1: { id: 1, owner: 0, name: 'Red 1', q: 0, r: 1, build: 'warrior', stock: -1e9 },
    2: { id: 2, owner: 1, name: 'Blue 1', q: 7, r: 1, build: 'warrior', stock: -1e9 },
  };
  const scout = spawnUnit(s, 0, 'scout', { q: 1, r: 1 });
  return { s, scout };
}

const shut = { terrain: 'mountain', trailCost: null };
const pass = { terrain: 'mountain', trailCost: 3, span: 1 };

function runRoute({ s, scout }) {
  ({ state: s } = dispatch(s, { type: 'assignRoute', unitId: scout.id, from: { q: 0, r: 1 }, to: { q: 7, r: 1 } }));
  const events = [...s.events];
  for (let i = 0; i < 40 && s.units[scout.id].route; i++) {
    ({ state: s } = dispatch(s, { type: 'endTurn' }));
    events.push(...s.events);
  }
  return { s, events, scout: s.units[scout.id] };
}

test('a scout finds the pass through a mountain wall and trails it', () => {
  const start = strip([shut, pass, shut]);
  assert.equal(routePlan(start.s, 0, { q: 0, r: 1 }, { q: 7, r: 1 }).known.cost, Infinity);

  const { s, events, scout } = runRoute(start);
  assert.equal(scout.route, undefined);
  assert.ok(s.map.tiles['4,1'].trail);
  assert.ok(events.some((e) => /found a pass at 4,1/.test(e)));
  assert.ok(events.some((e) => /route finished, cost 9/.test(e)));
  assert.ok(passable(s, { q: 4, r: 1 }));
  assert.equal(routePlan(s, 1, { q: 0, r: 1 }, { q: 7, r: 1 }).known.cost, 9, 'trails serve every player');
});

test('a sealed wall ends the search once every tile is revealed', () => {
  const { s, events, scout } = runRoute(strip([shut, shut, shut]));
  assert.equal(scout.route, undefined);
  assert.ok(events.some((e) => /no route can exist/.test(e)));
  assert.deepEqual(Object.keys(s.players[0].revealed).sort(), ['4,0', '4,1', '4,2']);
  assert.ok(!Object.values(s.map.tiles).some((t) => t.trail));
});

test('a pass beyond the bridge span stays shut until tech allows it', () => {
  const span2 = { ...pass, span: 2 };
  assert.ok(runRoute(strip([shut, span2, shut])).events.some((e) => /no route can exist/.test(e)));

  const teched = strip([shut, span2, shut]);
  teched.s.players[0].bridgeSpan = 2;
  const { s } = runRoute(teched);
  assert.ok(s.map.tiles['4,1'].trail);
});

test('nothing is revealed when no unknown tile could improve the route', () => {
  const { s, events } = runRoute(strip([{ terrain: 'forest', trailCost: 1 }, {}, {}]));
  assert.deepEqual(s.players[0].revealed, {});
  assert.ok(events.some((e) => /route finished, cost 7/.test(e)));
});
