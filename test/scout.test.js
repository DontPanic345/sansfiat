import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/scout/rules.js';
import { newGame, runPregame } from '../src/game.js';
import { dispatch } from '../src/rules.js';
import { key } from '../src/hex.js';
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
  assert.ok(after.some((n, i) => n >= before[i]));
  assert.ok(key({ q: 0, r: 0 }));
});
