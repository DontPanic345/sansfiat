// Cross-module seams: supply <-> army <-> civ.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as army from '../src/modules/army/rules.js';
import * as supply from '../src/modules/supply/rules.js';
import * as civ from '../src/modules/civ/rules.js';
import { newGame, runPregame } from '../src/game.js';
import { dispatch } from '../src/rules.js';
import { key, neighbors } from '../src/hex.js';
import { passable, tileAt, spawnUnit } from '../src/core.js';

function world() {
  let s = runPregame(newGame({ seed: 1, players: 3 }));
  const [c1, c2] = Object.keys(s.civs).map(Number);
  s = dispatch(s, { type: 'chooseCiv', civ: c1 }).state;
  const start = Object.values(s.map.tiles).find((t) => passable(s, t) && neighbors(t).every((n) => tileAt(s, n) && passable(s, n)));
  const chain = [{ q: start.q, r: start.r }];
  for (let i = 0; i < 5; i++) chain.push(neighbors(chain.at(-1)).find((n) => tileAt(s, n) && passable(s, n) && !chain.some((c) => key(c) === key(n))));
  s.cities = {};
  for (const c of Object.values(s.civs)) c.cityIds = [];
  const mk = (id, owner, h, founding = false) => {
    s.cities[id] = { id, owner, name: 'C' + id, q: h.q, r: h.r, buildings: {}, project: null, founding };
    s.civs[owner].cityIds.push(id);
  };
  return { s, c1, c2, chain, mk };
}
const armies = (s) => army.turnSteps[0].run(s);
const supplyStep = (s) => supply.turnSteps.find((x) => x.phase === 'armies').run(s);
const cap = (s, civId, h, troops) => spawnUnit(s, { owner: civId, type: 'captain', q: h.q, r: h.r, army: { troops, zone: [], movedTurn: -1 } });
const det = (s, civId, h, c, troops) => spawnUnit(s, { owner: civId, type: 'detachment', q: h.q, r: h.r, army: { captainId: c.id, troops, entr: 0 } });

test('supply hunger reaches detachments through u.army.captainId', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0], 6);
  const d = det(s, c1, chain[1], c, 4);
  supply.ensure(c); c.supply.food = 0;
  supplyStep(s);
  assert.ok(c.hungry && d.hungry);
  assert.ok(d.army.troops < 4, 'detachment troops suffer attrition');
});

test('a city wall strengthens its defenders in a fight', () => {
  const lost = (wall) => {
    const { s, c1, c2, chain, mk } = world();
    mk(9001, c1, chain[1]);
    if (wall) s.cities[9001].buildings.wall = true;
    cap(s, c1, chain[1], 5);
    const ac = cap(s, c2, chain[1], 5);
    armies(s);
    return 5 - ac.army.troops; // the attacker bleeds more against a wall
  };
  assert.ok(lost(true) > lost(false));
});

test('city garrison of 3 defends; capture needs it beaten', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9001, c1, chain[1]);
  cap(s, c2, chain[1], 1);
  armies(s);
  assert.equal(s.cities[9001].owner, c1, 'one soldier cannot walk in');
  cap(s, c2, chain[1], 10);
  for (let i = 0; i < 4 && s.cities[9001].owner === c1; i++) armies(s);
  assert.equal(s.cities[9001].owner, c2);
});

test('conquest victory fires after army captures every founding city', () => {
  const { s, c1, c2, chain, mk } = world();
  for (const c of Object.values(s.civs)) if (c.id !== c1 && c.id !== c2) delete s.civs[c.id];
  mk(9001, c1, chain[0], true);
  mk(9002, c2, chain[3], true);
  cap(s, c1, chain[3], 20);
  for (let i = 0; i < 4 && s.cities[9002].owner === c2; i++) armies(s);
  assert.equal(s.cities[9002].owner, c1);
  civ.victory(s);
  assert.equal(s.winner, c1);
  assert.equal(s.mods.civ.winKind, 'conquest');
});

test('a supply cache is captured with the tile held by army', () => {
  const { s, c1, c2, chain } = world();
  const t = tileAt(s, chain[2]);
  t.cache = { owner: c1, food: 10 };
  cap(s, c2, chain[2], 5);
  armies(s);
  assert.equal(t.held, c2);
  supplyStep(s);
  assert.equal(t.cache.owner, c2);
});
