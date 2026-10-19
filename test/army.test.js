import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/army/rules.js';
import { ARMY } from '../src/modules/army/data.js';
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

// A main-game state with a chain of plains tiles and a city for civ 1 (c1, the human).
let base = null;
function world() {
  if (!base) {
    let s = runPregame(newGame({ seed: 1, players: 3 }));
    const [c1] = Object.keys(s.civs).map(Number);
    base = dispatch(s, { type: 'chooseCiv', civ: c1 }).state;
  }
  const s = structuredClone(base);
  const [c1, c2] = Object.keys(s.civs).map(Number);
  const start = Object.values(s.map.tiles).find((t) => passable(s, t) && neighbors(t).every((n) => tileAt(s, n) && passable(s, n)));
  const chain = [{ q: start.q, r: start.r }];
  for (let i = 0; i < 6; i++) {
    const last = chain.at(-1);
    chain.push(neighbors(last).find((n) => tileAt(s, n) && passable(s, n) && !chain.some((c) => key(c) === key(n)) && chain.every((c) => c === last || !neighbors(c).some((x) => key(x) === key(n)))));
  }
  for (const h of chain) Object.assign(tileAt(s, h), { terrain: 'plains', trail: false, fort: null, fallow: 0 });
  s.cities = {}; s.units = {};
  for (const c of Object.values(s.civs)) { c.cityIds = []; c.store = {}; }
  for (const c of Object.values(s.clans)) c.owner = null;
  const mk = (id, owner, h) => { s.cities[id] = { id, owner, name: 'C' + id, q: h.q, r: h.r, buildings: {}, project: null }; s.civs[owner].cityIds.push(id); };
  mk(9001, c1, chain[0]);
  s.events = [];
  return { s, c1, c2, chain, mk };
}
const cap = (s, civ, h, troops = 10, extra = {}) => spawnUnit(s, { owner: civ, type: 'captain', q: h.q, r: h.r, army: { troops, zone: [] }, ...extra });
const dets = (s) => Object.values(s.units).filter((u) => u.type === 'detachment');
const caps = (s) => Object.values(s.units).filter((u) => u.type === 'captain');
const alerts = (s) => s.events.filter((e) => e.includes(': !'));

test('army module is registered and initialises its state', () => {
  assert.equal(mod.name, 'army');
  assert.ok(newGame({ seed: 1 }).mods.army);
});

test('raiseArmy takes 10 people from clans and provisions only from the store', () => {
  const { s, c1 } = world();
  const clan = Object.values(s.clans)[0];
  clan.owner = c1; clan.pop = 30;
  assert.equal(run(s, 'raiseArmy', { civ: c1, city: 9001 }), null);
  const c = caps(s)[0];
  assert.equal(c.army.troops, 10);
  assert.equal(clan.pop, 20);
  assert.equal(c.supply.food, 0, 'no free food: the store is empty');
  assert.ok(s.notes[clan.id].length);
  assert.ok(run(s, 'raiseArmy', { civ: c1, city: 99 }));
  s.civs[c1].store = { grain: 25 };
  assert.equal(run(s, 'raiseArmy', { civ: c1, cityId: 9001 }), null, 'cityId alias');
  assert.equal(caps(s)[1].supply.food, 25);
  assert.equal(s.civs[c1].store.grain ?? 0, 0);
});

test('moving costs labour by terrain; a full-labour captain can always take one step', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  assert.equal(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[1] }), null);
  assert.equal(c.labour, 3);
  tileAt(s, chain[2]).terrain = 'hills';
  assert.equal(run(s, 'moveCaptain', { civ: c1, unitId: c.id, ...chain[2] }), null, 'unitId alias');
  assert.equal(c.labour, 1);
  tileAt(s, chain[3]).terrain = 'forest';
  assert.match(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[3] }), /needs 2 labour/);
  c.labour = 0;
  assert.match(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[1] }), /labour/);
});

test('a deployed captain paints as he steps out; paint keeps the zone connected', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0]);
  assert.equal(run(s, 'paint', { civ: c1, unit: c.id, ...chain[0] }), null);
  assert.ok(run(s, 'paint', { civ: c1, unit: c.id, ...chain[3] }), 'disconnected paint refused');
  assert.equal(run(s, 'moveCaptain', { civ: c1, unit: c.id, ...chain[1] }), null);
  assert.ok(c.army.zone.includes(key(chain[1])), 'stepping out paints');
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

test('skirmish: both lose troops and record the fight for food; entrenchment helps; 3:1 can slip away', () => {
  const { s, c1, c2, chain } = world();
  const a = cap(s, c1, chain[1], 10), b = cap(s, c2, chain[1], 10);
  step(s);
  assert.ok(a.army.troops < 10 && b.army.troops < 10);
  assert.equal(a.army.foughtTurn, s.turn);
  assert.equal(a.army.foughtTroops, 10);
  assert.ok(alerts(s).some((e) => /fight/.test(e)), 'the human is alerted');
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

test('a hungry army cannot attack, and defends at half power', () => {
  const { s, c1, c2, chain } = world();
  const a = cap(s, c1, chain[1], 10);
  cap(s, c2, chain[2], 10);
  a.hungry = true;
  assert.match(run(s, 'moveCaptain', { civ: c1, unit: a.id, ...chain[2] }), /hungry/);
  a.hungry = false;
  assert.equal(run(s, 'moveCaptain', { civ: c1, unit: a.id, ...chain[2] }), null);
  const { s: t, c1: d1, c2: d2, chain: ch } = world();
  const h = cap(t, d1, ch[1], 10), e = cap(t, d2, ch[1], 10);
  h.hungry = true;
  step(t);
  assert.ok(h.army.troops < e.army.troops);
});

function cutOff(s, c1, chain) {
  const c = cap(s, c1, chain[0]);
  for (const h of chain.slice(0, 3)) run(s, 'paint', { civ: c1, unit: c.id, ...h });
  for (let i = 0; i < 4; i++) step(s);
  const far = dets(s).find((d) => key(d) === key(chain[2]));
  c.army.zone = c.army.zone.filter((k) => k !== key(chain[1])); // link severed
  return { c, far };
}

test('cut-off detachments can be promoted (with a share of the food) while under the captain cap', () => {
  const { s, c1, chain } = world();
  const { c, far } = cutOff(s, c1, chain);
  c.supply = { food: 30, foodMax: 40, auto: true };
  assert.equal(A.promote.validate(s, { civ: c1, unit: far.id }), null);
  assert.ok(A.promote.validate(s, { civ: c1, unit: dets(s).find((d) => key(d) === key(chain[0])).id }), 'linked ones cannot');
  const before = c.army.troops;
  A.promote.apply(s, { civ: c1, unit: far.id });
  const nc = caps(s).find((u) => u.id !== c.id);
  assert.equal(caps(s).length, 2);
  assert.ok(nc.army.troops > 0);
  assert.ok(Math.abs(nc.supply.food - 30 * nc.army.troops / before) < 1e-9, 'food shared by troops, not free');
  assert.ok(Math.abs(c.supply.food + nc.supply.food - 30) < 1e-9);
});

test('promote respects the captain cap; over the cap a cut-off detachment disbands into its clans', () => {
  const { s, c1, chain } = world();
  const clan = Object.values(s.clans)[0];
  clan.owner = c1;
  const pop = clan.pop;
  const { far } = cutOff(s, c1, chain);
  cap(s, c1, chain[5], 5); cap(s, c1, chain[6], 5);
  assert.match(A.promote.validate(s, { civ: c1, unit: far.id }), /at most 3/);
  far.army.cutSince = s.turn - ARMY.cutTurns;
  const troops = far.army.troops;
  step(s);
  assert.ok(!s.units[far.id], 'gave up');
  assert.equal(clan.pop, pop + troops, 'troops went home to the clan');
});

test('the AI never cuts off or promotes its own linked detachments', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9003, c2, chain[6]);
  const c = cap(s, c1, chain[0]);
  for (const h of chain.slice(0, 4)) run(s, 'paint', { civ: c1, unit: c.id, ...h });
  for (let i = 0; i < 4; i++) step(s);
  const acts = mod.ai(s, c1);
  assert.ok(!acts.some((a) => a.type === 'promote' || a.type === 'unpaint'));
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
  const f = tileAt(s, chain[4]).fort;
  assert.equal(f.owner, c2, 'garrison beaten, fort taken');
  assert.ok(alerts(s).some((e) => /takes your fort/.test(e)));
});

test('city capture: the garrison must be beaten, a wall multiplies it, transferCity moves the city', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9002, c2, chain[3]);
  const a = cap(s, c1, chain[3], 10);
  step(s);
  // garrison 3 x 1.75 (dug in) x 1.5 (city) = 7.875 power: the attacker loses round(3.9) = 4
  assert.equal(a.army.troops, 6);
  assert.equal(s.cities[9002].owner, c1, 'garrison 3 - round(10*0.5/1.75)=3 -> 0, tile held: taken');
  assert.ok(s.civs[c1].cityIds.includes(9002) && !s.civs[c2].cityIds.includes(9002));
  assert.equal(s.cities[9002].garrison, ARMY.garrison, 'new owner garrisons it');
  assert.ok(alerts(s).some((e) => /You take C9002/.test(e)));
  assert.equal(s.mods.army.captures, 1);

  const w = world();
  w.mk(9002, w.c2, w.chain[3]);
  w.s.cities[9002].buildings.wall = true;
  const b = cap(w.s, w.c1, w.chain[3], 10);
  step(w.s);
  // with a wall: 7.875 x 1.5 = 11.8 power: the attacker loses round(5.9) = 6
  assert.equal(b.army.troops, 4);

  const v = world();
  v.mk(9002, v.c2, v.chain[3]);
  const weak = cap(v.s, v.c1, v.chain[3], 4);
  step(v.s);
  assert.equal(v.s.cities[9002].owner, v.c2, 'a small army does not beat the garrison');
  assert.ok(v.s.cities[9002].garrison > 0);
  assert.ok(weak.army.troops < 4 || !v.s.units[weak.id]);
});

test('losing a city alerts the human', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9004, c1, chain[3]);
  s.cities[9004].garrison = 0;
  cap(s, c2, chain[3], 10);
  step(s);
  assert.equal(s.cities[9004].owner, c2);
  assert.ok(alerts(s).some((e) => /takes your C9004/.test(e)));
});

test('disband returns troops to the clans, only near home', () => {
  const { s, c1, chain } = world();
  const clan = Object.values(s.clans)[0];
  clan.owner = c1;
  const pop = clan.pop;
  const c = cap(s, c1, chain[0], 7);
  const far = cap(s, c1, chain[6], 7);
  assert.ok(run(s, 'disband', { civ: c1, unit: far.id }));
  assert.equal(run(s, 'disband', { civ: c1, unit: c.id }), null);
  assert.equal(clan.pop, pop + 7);
  assert.ok(!s.units[c.id]);
});

test('ai raises only when it can feed and reach a rival, then marches toward the rival city', () => {
  const { s, c1, c2, chain, mk } = world();
  mk(9003, c2, chain[6]);
  const clan = Object.values(s.clans)[0]; clan.owner = c1; clan.pop = 40;
  for (const t of Object.values(s.map.tiles)) t.stock = {};
  assert.ok(!mod.ai(s, c1).some((a) => a.type === 'raiseArmy'), 'no food anywhere: no army');
  s.civs[c1].store = { grain: 40 };
  const r = mod.ai(s, c1).find((a) => a.type === 'raiseArmy');
  assert.ok(r);
  run(s, 'raiseArmy', r);
  s.mods.army.raisedTurn[c1] = s.turn;
  assert.ok(!mod.ai(s, c1).some((a) => a.type === 'raiseArmy'), 'waits between raisings');
  const c = caps(s)[0];
  const moves = mod.ai(s, c1).filter((a) => a.type === 'moveCaptain');
  assert.ok(moves.length >= 1);
  assert.equal(key(moves[0]), key(chain[1]), 'heads along the path to the rival city');
  for (const m of moves) assert.equal(run(s, 'moveCaptain', m), null);
  assert.ok(c.labour >= 4 - ARMY.aiMoveLabour, 'keeps labour for food');
  assert.equal(dispatch(s, { type: 'endTurn' }).error, undefined);
});

test('ai disbands a starving army at home instead of letting it die', () => {
  const { s, c1, chain } = world();
  const c = cap(s, c1, chain[0], 10, { hungry: true, supply: { food: 0, foodMax: 40, auto: true } });
  assert.ok(mod.ai(s, c1).some((a) => a.type === 'disband' && a.unit === c.id));
});
