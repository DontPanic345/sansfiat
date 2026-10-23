import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/scout/rules.js';
import { newGame, runPregame } from '../src/game.js';
import { dispatch } from '../src/rules.js';
import { stepCost } from '../src/core.js';

function mainGame(seed = 1) {
  let s = newGame({ seed });
  s = runPregame(s, 400);
  const r = dispatch(s, { type: 'chooseCiv', civ: Object.keys(s.civs)[0] ?? null });
  return r.state ?? s;
}

test('scout module is registered and initialises its state', () => {
  assert.equal(mod.name, 'scout');
  assert.ok(newGame({ seed: 1 }).mods.scout);
});

// Hand-built fixture: a row of plains with a trailable forest and hills in the middle.
function fixture() {
  const s = newGame({ seed: 1 });
  s.phase = 'main';
  s.civs = { 1: { id: 1, seen: {}, revealed: {}, cityIds: [], store: {} } };
  s.cities = {};
  s.units = {};
  s.map.tiles = {};
  for (let q = 0; q < 6; q++) s.map.tiles[`${q},0`] = { q, r: 0, terrain: 'plains', trail: false };
  Object.assign(s.map.tiles['2,0'], { terrain: 'forest', trailCost: 1 });
  Object.assign(s.map.tiles['3,0'], { terrain: 'hills', trailCost: 1 });
  return s;
}

test('scout unit spawns with labour; main game gives each civ one', () => {
  const s = mainGame();
  for (const c of Object.values(s.civs)) {
    const sc = Object.values(s.units).filter((u) => u.owner === c.id && u.type === 'scout');
    assert.ok(sc.length >= 1);
    assert.equal(sc[0].labour, sc[0].labourMax);
  }
});

test('assignRoute reveals unknown tiles then builds trails', () => {
  const s = fixture();
  const u = Object.assign({ id: 9, owner: 1, type: 'scout', q: 0, r: 0, labour: 100, labourMax: 100 }, { scout: { route: null } });
  s.units[9] = u;
  const a = { type: 'assignRoute', civ: 1, unit: 9, from: { q: 0, r: 0 }, to: { q: 5, r: 0 } };
  assert.equal(mod.actions.assignRoute.validate(s, a), null);
  mod.actions.assignRoute.apply(s, a);
  mod.scoutWork(s, u);
  assert.ok(s.civs[1].revealed['2,0'] && s.civs[1].revealed['3,0']);
  assert.ok(s.map.tiles['2,0'].trail && s.map.tiles['3,0'].trail);
  assert.equal(stepCost(s, { q: 2, r: 0 }), 1);
  assert.equal(u.scout.route, null);
});

test('trail work needs labour: forest 2, hills 3', () => {
  const s = fixture();
  s.civs[1].revealed['2,0'] = true;
  const u = { id: 9, owner: 1, type: 'scout', q: 1, r: 0, labour: 2, labourMax: 2, scout: { route: { from: { q: 0, r: 0 }, to: { q: 2, r: 0 } } } };
  s.units[9] = u;
  mod.scoutWork(s, u);
  assert.ok(s.map.tiles['2,0'].trail);
  assert.equal(u.labour, 0);
});

test('fog: seen grows around units', () => {
  const s = fixture();
  s.units[9] = { id: 9, owner: 1, type: 'scout', q: 0, r: 0, labour: 1, labourMax: 1 };
  mod.updateFog(s);
  assert.ok(s.civs[1].seen['0,0'] && s.civs[1].seen['2,0']);
  assert.ok(!s.civs[1].seen['5,0']);
});

test('ai sends an idle scout on a route', () => {
  const s = fixture();
  s.markets = {};
  s.units[9] = { id: 9, owner: 1, type: 'scout', q: 0, r: 0, labour: 4, labourMax: 4, scout: { route: null } };
  const out = mod.ai(s, 1);
  assert.equal(out.length, 1);
  assert.equal(out[0].type, 'assignRoute');
});

test('knowledge spreads between civs at a shared market', () => {
  const s = fixture();
  s.civs[2] = { id: 2, seen: {}, revealed: {}, cityIds: [], store: {} };
  s.civs[1].revealed['2,0'] = true;
  s.markets = { 7: { id: 7 } };
  s.clans = { 1: { id: 1, market: 7, owner: 1 }, 2: { id: 2, market: 7, owner: 2 } };
  const step = mod.turnSteps.find((t) => t.name === 'shareKnowledge');
  for (let i = 0; i < 200 && !s.civs[2].revealed['2,0']; i++) step.run(s, {});
  assert.ok(s.civs[2].revealed['2,0']);
});

test('main game with scout AI runs and explores', () => {
  let s = mainGame(2);
  const before = Object.values(s.civs).map((c) => Object.keys(c.seen).length);
  for (let i = 0; i < 15; i++) s = dispatch(s, { type: 'endTurn' }).state;
  const after = Object.values(s.civs).map((c) => Object.keys(c.seen).length);
  assert.ok(after.every((n, i) => n >= before[i]), 'fog never shrinks');
  assert.ok(after.some((n, i) => n > before[i]), 'some civ saw new land');
});

// A row of plains q 0..n-1 with one civ, for movement tests.
function row(n, extra = {}) {
  const s = fixture();
  s.map.tiles = {};
  for (let q = 0; q < n; q++) s.map.tiles[`${q},0`] = { q, r: 0, terrain: 'plains', trail: false };
  for (const [k, t] of Object.entries(extra)) Object.assign(s.map.tiles[k], t);
  return s;
}
const scoutAt = (s, q, labour = 100) => (s.units[9] = { id: 9, owner: 1, type: 'scout', q, r: 0, labour, labourMax: labour, scout: { route: null, tried: [] } });
const send = (s, from, to, extra = {}) => mod.actions.assignRoute.apply(s, { type: 'assignRoute', civ: 1, unit: 9, from: { q: from, r: 0 }, to: { q: to, r: 0 }, ...extra });

test('scout reveals and bridges an unrevealed mountain from the tile beside it (no stall)', () => {
  const s = row(6, { '3,0': { terrain: 'mountain', trailCost: 3, span: 1 } });
  const u = scoutAt(s, 0);
  send(s, 0, 5);
  mod.scoutWork(s, u);
  assert.ok(s.civs[1].revealed['3,0']);
  assert.ok(s.map.tiles['3,0'].trail, 'bridge built');
  assert.deepEqual([u.q, u.r], [5, 0]);
  assert.equal(u.scout.route, null);
});

test('a scout that cannot reach its work stops and notes that it is stuck', () => {
  const s = row(6, { '1,0': { terrain: 'ocean' }, '3,0': { terrain: 'mountain', trailCost: 3, span: 1 } });
  const u = scoutAt(s, 0);
  send(s, 2, 5);
  mod.scoutWork(s, u);
  assert.equal(u.scout.route, null);
  assert.match(s.notes[9].at(-1).text, /stuck/);
});

test('with nothing to reveal or build, the scout walks to the destination over turns', () => {
  const s = row(8);
  const u = scoutAt(s, 0, 4);
  send(s, 0, 7);
  mod.scoutWork(s, u);
  assert.deepEqual([u.q, u.labour], [4, 0]);
  assert.ok(u.scout.route, 'still walking');
  assert.ok(s.civs[1].seen['6,0'], 'fog lifts as it walks');
  u.labour = u.labourMax;
  mod.scoutWork(s, u);
  assert.equal(u.q, 7);
  assert.equal(u.scout.route, null);
  assert.match(s.notes[9].at(-1).text, /route finished at 7,0/);
});

test('cancelRoute stops a scout', () => {
  const s = row(4);
  const u = scoutAt(s, 0);
  send(s, 0, 3);
  const a = { type: 'cancelRoute', civ: 1, unit: 9 };
  assert.equal(mod.actions.cancelRoute.validate(s, a), null);
  mod.actions.cancelRoute.apply(s, a);
  assert.equal(u.scout.route, null);
  assert.ok(mod.actions.cancelRoute.validate(s, a));
  assert.ok(mod.actions.cancelRoute.validate(s, { ...a, civ: 2 }));
});

test('plannedCost ignores a trail on a tile the civ has not seen', () => {
  const s = row(4, { '2,0': { terrain: 'forest', trailCost: 1, trail: true } });
  assert.equal(mod.plannedCost(s, 1, { q: 2, r: 0 }).worst, 2);
  s.civs[1].seen['2,0'] = true;
  assert.equal(mod.plannedCost(s, 1, { q: 2, r: 0 }).worst, 1);
});

test('ai explores: its scout moves into unseen land without repeating or flooding the log', () => {
  const s = row(30);
  s.markets = {};
  const u = scoutAt(s, 0, 4);
  mod.updateFog(s);
  const seen = [Object.keys(s.civs[1].seen).length];
  for (let turn = 0; turn < 12; turn++) {
    s.events = [];
    for (const a of mod.ai(s, 1)) if (!mod.actions[a.type].validate(s, a)) mod.actions[a.type].apply(s, a);
    mod.scoutWork(s, u);
    mod.updateFog(s);
    u.labour = u.labourMax;
    seen.push(Object.keys(s.civs[1].seen).length);
    assert.ok(!s.events.some((e) => /scout/.test(e)), 'AI scouts do not write events');
  }
  assert.ok(u.q >= 20, `scout walked east, at ${u.q}`);
  assert.equal(seen.at(-1), 30, 'whole row seen');
  assert.equal(new Set(u.scout.tried).size, u.scout.tried.length, 'no target tried twice');
});

test('ai builds a trail toward a rival city', () => {
  const s = row(6, { '2,0': { terrain: 'forest', trailCost: 1 }, '3,0': { terrain: 'hills', trailCost: 1 } });
  s.civs[2] = { id: 2, seen: {}, revealed: {}, cityIds: [11], store: {} };
  s.cities = { 10: { id: 10, owner: 1, q: 0, r: 0 }, 11: { id: 11, owner: 2, q: 5, r: 0 } };
  s.civs[1].cityIds = [10];
  const u = scoutAt(s, 0);
  const out = mod.ai(s, 1);
  assert.equal(out.length, 1);
  assert.equal(out[0].walk, false);
  mod.actions.assignRoute.apply(s, out[0]);
  mod.scoutWork(s, u);
  assert.ok(s.map.tiles['2,0'].trail && s.map.tiles['3,0'].trail);
  assert.ok(s.mods.scout.roads[1].includes('10-11'));
  assert.equal(mod.ai(s, 1)[0]?.pair, undefined, 'road not rebuilt');
});
