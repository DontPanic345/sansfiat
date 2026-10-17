import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/army/rules.js';
import { newGame, runPregame } from '../src/game.js';
import { dispatch } from '../src/rules.js';
import { key, neighbors } from '../src/hex.js';
import { passable, tileAt, spawnUnit } from '../src/core.js';

const A = mod.actions;
const run = (s, type, a) => {
  const e = A[type].validate(s, a);
  if (e) return e;
  A[type].apply(s, a);
  return null;
};
const step = (s) => mod.turnSteps[0].run(s);

// A main-game state with a chain of passable tiles and a city for civ 1.
function world() {
  let s = runPregame(newGame({ seed: 1, players: 3 }));
  const [c1, c2] = Object.keys(s.civs).map(Number);
  s = dispatch(s, { type: 'chooseCiv', civ: c1 }).state;
  const start = Object.values(s.map.tiles).find((t) => passable(s, t) && neighbors(t).every((n) => tileAt(s, n) && passable(s, n)));
  const chain = [{ q: start.q, r: start.r }];
  for (let i = 0; i < 5; i++) {
    const last = chain.at(-1);
    chain.push(neighbors(last).find((n) => tileAt(s, n) && passable(s, n) && !chain.some((c) => key(c) === key(n))));
  }
  s.cities = {};
  for (const c of Object.values(s.civs)) c.cityIds = [];
  const mk = (id, owner, h) => { s.cities[id] = { id, owner, name: 'C' + id, q: h.q, r: h.r, buildings: {}, project: null }; s.civs[owner].cityIds.push(id); };
  mk(9001, c1, chain[0]);
  return { s, c1, c2, chain, mk };
}
const cap = (s, civ, h, troops = 10) => spawnUnit(s, { owner: civ, type: 'captain', q: h.q, r: h.r, army: { troops, zone: [], movedTurn: -1 } });
const dets = (s) => Object.values(s.units).filter((u) => u.type === 'detachment');

test('army module is registered and initialises its state', () => {
  assert.equal(mod.name, 'army');
  assert.ok(newGame({ seed: 1 }).mods.army);
});

test('raiseArmy takes 10 people from clans and spawns a captain', () => {
  const { s, c1 } = world();
  const clan = Object.values(s.clans)[0];
  clan.owner = c1; clan.pop = 30;
  assert.equal(run(s, 'raiseArmy', { civ: c1, city: 9001 }), null);
  const c = Object.values(s.units).find((u) => u.type === 'captain');
  assert.equal(c.army.troops, 10);
  assert.equal(clan.pop, 20);
  assert.ok(s.notes[clan.id].length);
  assert.ok(run(s, 'raiseArmy', { civ: c1, city: 99 }));
});

test('captain moves one tile per turn and paints out of its zone; paint keeps zone connected', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  assert.equal(run(s, 'paint', { civ: c1, unit: c.id, ...chain[0] }), null);
  assert.ok(run(s, 'paint', { civ: c1, unit: c.id, ...chain[3] }), 'disconnected paint refused');
  assert.equal(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[1] }), null);
  assert.ok(c.army.zone.includes(key(chain[1])), 'stepping out paints');
  assert.ok(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[2] }), 'second move same turn refused');
  assert.equal(run(s, 'unpaint', { civ: c1, unit: c.id, ...chain[0] }), null);
  assert.ok(!c.army.zone.includes(key(chain[0])));
});

test('undeployed captain steps paint nothing; undeploy clears the zone', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[1] });
  assert.deepEqual(c.army.zone, []);
  run(s, 'paint', { civ: c1, unit: c.id, ...chain[1] });
  assert.equal(dets(s).length, 1);
  run(s, 'undeploy', { civ: c1, unit: c.id });
  assert.deepEqual(c.army.zone, []);
  assert.equal(dets(s).length, 0);
  assert.equal(c.army.troops, 10);
});

test('troops spread over the zone, one tile a turn, entrench, and hold tiles', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  run(s, 'paint', { civ: c1, unit: c.id, ...chain[0] });
  for (const h of [chain[1], chain[2]]) run(s, 'paint', { civ: c1, unit: c.id, ...h });
  step(s);
  assert.ok(dets(s).some((d) => key(d) === key(chain[1])), 'creeps out one tile');
  for (let i = 0; i < 6; i++) step(s);
  const per = {};
  for (const d of dets(s)) per[key(d)] = (per[key(d)] ?? 0) + d.army.troops;
  assert.equal(Object.keys(per).length, 3);
  assert.ok(Object.values(per).every((n) => n >= 2 && n <= 4));
  assert.equal(c.army.troops, 10);
  assert.ok(dets(s).every((d) => d.army.entr === 3));
  assert.equal(tileAt(s, chain[2]).held, c1);
});

test('skirmish: both lose troops; entrenchment helps; outnumbered 3:1 can slip away', () => {
  const { s, c1, c2, chain } = world();
  const a = cap(s, c1, chain[1], 10), b = cap(s, c2, chain[1], 10);
  step(s);
  assert.ok(a.army.troops < 10 && b.army.troops < 10);
  const x = cap(s, c1, chain[3], 12), y = cap(s, c2, chain[3], 2);
  let slipped = false;
  for (let i = 0; i < 20 && !slipped; i++) {
    y.army.troops = 2; x.army.troops = 12; y.q = chain[3].q; y.r = chain[3].r; x.q = chain[3].q; x.r = chain[3].r;
    step(s);
    slipped = y.q !== chain[3].q || y.r !== chain[3].r;
  }
  assert.ok(slipped, 'retreat happens with some chance');
  for (const u of Object.values(s.units)) delete s.units[u.id];
  const d1 = cap(s, c1, chain[4], 5), d2 = cap(s, c2, chain[4], 5);
  d1.army.entr = 3;
  step(s);
  assert.ok(d1.army.troops > d2.army.troops);
});

test('cut-off detachments can be promoted to a free captain', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  for (const h of chain.slice(0, 3)) run(s, 'paint', { civ: c1, unit: c.id, ...h });
  for (let i = 0; i < 4; i++) step(s);
  const far = dets(s).find((d) => key(d) === key(chain[2]));
  c.army.zone = c.army.zone.filter((k) => k !== key(chain[1])); // link severed
  assert.equal(A.promote.validate(s, { civ: c1, unit: far.id }), null);
  assert.ok(A.promote.validate(s, { civ: c1, unit: dets(s).find((d) => key(d) === key(chain[0])).id }), 'linked ones cannot');
  A.promote.apply(s, { civ: c1, unit: far.id });
  const caps = Object.values(s.units).filter((u) => u.type === 'captain');
  assert.equal(caps.length, 2);
  assert.ok(caps.find((u) => u.id !== c.id).army.troops > 0);
});

test('cut-off detachments without promotion walk back toward the captain', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  run(s, 'paint', { civ: c1, unit: c.id, ...chain[0] });
  const d = dets(s)[0];
  d.q = chain[3].q; d.r = chain[3].r;
  const before = Math.abs(d.q - chain[0].q) + Math.abs(d.r - chain[0].r);
  step(s);
  assert.ok(Math.abs(d.q - chain[0].q) + Math.abs(d.r - chain[0].r) < before);
});

test('buildFort costs labour, wood and stone, holds its tile and fights with its garrison', () => {
  const { s, c1, c2, chain } = world();
  const c = cap(s, c1, chain[0]);
  assert.ok(run(s, 'buildFort', { civ: c1, unit: c.id }), 'needs resources');
  s.civs[c1].store.wood = 5; s.civs[c1].store.stone = 5;
  assert.equal(run(s, 'buildFort', { civ: c1, unit: c.id }), null);
  assert.equal(tileAt(s, chain[0]).fort.owner, c1);
  assert.equal(s.civs[c1].store.wood, 3);
  tileAt(s, chain[4]).fort = { owner: c1, garrison: 3 };
  cap(s, c2, chain[4], 10);
  step(s);
  assert.ok(tileAt(s, chain[4]).fort.garrison < 3 || tileAt(s, chain[4]).fort.owner === c2);
});

test('holding an enemy city tile transfers it', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9002, c2, chain[2]);
  cap(s, c1, chain[2], 10);
  step(s);
  assert.equal(s.cities[9002].owner, c1);
  assert.ok(s.civs[c1].cityIds.includes(9002) && !s.civs[c2].cityIds.includes(9002));
});

test('hungry units attack at half power', () => {
  const { s, c1, c2, chain } = world();
  const a = cap(s, c1, chain[1], 10), b = cap(s, c2, chain[1], 10);
  a.hungry = true;
  step(s);
  assert.ok(a.army.troops < b.army.troops);
});

test('ai raises an army and heads for a rival city', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9003, c2, chain[5]);
  const clan = Object.values(s.clans)[0]; clan.owner = c1; clan.pop = 40;
  assert.ok(mod.ai(s, c1).some((a) => a.type === 'raiseArmy'));
  run(s, 'raiseArmy', { civ: c1, city: 9001 });
  assert.ok(mod.ai(s, c1).some((a) => a.type === 'paint'));
  assert.equal(dispatch(s, { type: 'endTurn' }).error, undefined);
});
