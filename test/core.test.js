import assert from 'node:assert/strict';
import { test } from 'node:test';
import { TRAILS } from '../src/data.js';
import { key, neighbors } from '../src/hex.js';
import { newGame, replay, runPregame, runPregameCounted } from '../src/game.js';
import { clear } from '../src/market.js';
import {
  civOf, costMap, note, notesFor, pathCost, putGoods, spawnUnit, stepCost, sweepNotes, takeGoods, territoryOf,
  tileAt, transferCity, unitsAt,
} from '../src/core.js';
import { ACTIONS, PHASES, PIPELINE, dispatch, legalActions } from '../src/rules.js';
import { MODULES } from '../src/modules/index.js';

const founded = (seed = 1) => runPregame(newGame({ seed, players: 3 }));

test('mapgen rolls hidden trail costs: forest and hills to 1-2, mountains to 3 with a span, or never', () => {
  const { map } = newGame({ seed: 7 });
  const tiles = Object.values(map.tiles);
  for (const t of tiles) {
    assert.equal(t.trail, false);
    assert.equal(t.fort, null);
    assert.equal(t.cache, null);
    if (!TRAILS[t.terrain]) { assert.equal(t.trailCost, undefined); continue; }
    if (t.terrain === 'mountain') {
      if (t.trailCost === null) assert.equal(t.span, undefined);
      else { assert.equal(t.trailCost, 3); assert.ok([1, 2].includes(t.span)); }
    } else assert.ok([1, 2].includes(t.trailCost));
  }
  const mountains = tiles.filter((t) => t.terrain === 'mountain');
  assert.ok(mountains.some((t) => t.trailCost === null) && mountains.some((t) => t.trailCost === 3));
  assert.ok(tiles.some((t) => t.terrain === 'hills' && t.trailCost === 1));
});

test('stepCost and pathCost use trails, which also open mountain passes', () => {
  const s = newGame({ seed: 7 });
  const forest = Object.values(s.map.tiles).find((t) => t.terrain === 'forest' && t.trailCost === 1);
  assert.equal(stepCost(s, forest), 2);
  forest.trail = true;
  assert.equal(stepCost(s, forest), 1);
  const mtn = Object.values(s.map.tiles).find((t) => t.terrain === 'mountain' && t.trailCost === 3
    && neighbors(t).some((n) => stepCost(s, n) === 1));
  assert.equal(stepCost(s, mtn), Infinity);
  mtn.trail = true;
  assert.equal(stepCost(s, mtn), 3);
  const from = neighbors(mtn).find((n) => stepCost(s, n) === 1);
  const p = pathCost(s, from, mtn);
  assert.equal(p.cost, 3);
  assert.deepEqual(p.path.map(key), [key(from), key(mtn)]);
  assert.equal(costMap(s, from, 3).get(key(mtn)), 3);
  assert.equal(pathCost(s, from, { q: -99, r: -99 }), null);
});

test('the fast headless pre-game gives exactly the state that dispatching endTurn does', () => {
  const config = { seed: 2, players: 2 };
  const { state, turns } = runPregameCounted(newGame(config), 40);
  assert.equal(turns, 40);
  const slow = replay(config, Array.from({ length: 40 }, () => ({ type: 'endTurn' })));
  state.events = slow.events = [];
  assert.deepEqual(state, slow);
});

test('the why log is bounded, deduplicates, and forgets entities that are gone', () => {
  const s = newGame({ seed: 1 });
  const clan = Object.values(s.clans)[0];
  for (let i = 0; i < 20; i++) note(s, clan.id, `reason ${i}`);
  note(s, clan.id, 'reason 19');
  assert.equal(notesFor(s, clan.id).length, 8);
  assert.equal(notesFor(s, clan.id).at(-1).text, 'reason 19');
  note(s, 9999, 'nobody');
  sweepNotes(s);
  assert.equal(notesFor(s, 9999).length, 0);
  assert.equal(notesFor(s, clan.id).length, 8);
});

test('clan rules record why they act', () => {
  const s = runPregame(newGame({ seed: 1 }), 120);
  const texts = Object.values(s.notes).flat().map((n) => n.text);
  for (const re of [/^starving/, /^divided|^sent/, /^goes to market/, /traders swapped/])
    assert.ok(texts.some((t) => re.test(t)), `no note matching ${re}`);
  assert.ok(Object.keys(s.notes).every((id) => s.clans[id] || s.markets[id]));
});

test('founding creates civs, and chooseCiv starts the main game', () => {
  const s = founded(1);
  assert.equal(s.phase, 'choose');
  const civs = Object.values(s.civs);
  assert.equal(civs.length, 3);
  for (const c of civs) {
    assert.deepEqual(Object.keys(c).sort(),
      ['cityIds', 'color', 'founded', 'id', 'isHuman', 'marketId', 'name', 'revealed', 'seen', 'store'].sort());
    assert.equal(s.markets[c.marketId].owner, c.id);
    assert.equal(civOf(s, s.markets[c.marketId]), c);
  }
  assert.ok(Object.values(s.clans).every((c) => c.owner === null && typeof c.leaning === 'object'));
  assert.deepEqual(legalActions(s).map((a) => a.type).sort(), ['chooseCiv', 'chooseCiv', 'chooseCiv', 'chooseCiv']);
  assert.match(dispatch(s, { type: 'chooseCiv', civ: 12345 }).error, /no civ/);
  const main = dispatch(s, { type: 'chooseCiv', civ: civs[1].id }).state;
  assert.equal(main.phase, 'main');
  assert.equal(main.civs[civs[1].id].isHuman, true);
  assert.equal(dispatch(main, { type: 'endTurn' }).error, undefined);
});

test('the registry wires four modules into known phases with no clashing actions', () => {
  assert.deepEqual(MODULES.map((m) => m.name), ['civ', 'scout', 'army', 'supply']);
  for (const st of PIPELINE) assert.ok(PHASES.includes(st.phase));
  assert.deepEqual([...new Set(PIPELINE.map((st) => st.phase))], PHASES.filter((p) => PIPELINE.some((st) => st.phase === p)));
  assert.ok(ACTIONS.endTurn && ACTIONS.chooseCiv);
  const s = newGame({ seed: 1 });
  assert.deepEqual(Object.keys(s.mods).sort(), ['army', 'civ', 'scout', 'supply']);
});

test('civ stores trade at markets through posts, and goods are conserved', () => {
  let s = dispatch(founded(1), { type: 'chooseCiv', civ: null }).state;
  const civ = Object.values(s.civs)[0];
  const m = s.markets[civ.marketId];
  // A trader that only posts wood and pottery: the market clears non-food goods too.
  m.posts[civ.id] = { wood: 10, pottery: 4 };
  s = dispatch(s, { type: 'endTurn' }).state;
  const got = s.markets[m.id].received[civ.id];
  assert.ok(got, 'the civ gets its basket back');
  assert.deepEqual(s.markets[m.id].posts, {});
  const res = clear({ a: { wood: 10, grain: 1 }, b: { grain: 8, clay: 2 } });
  for (const g of ['wood', 'grain', 'clay']) {
    const after = (res.after.a[g] ?? 0) + (res.after.b[g] ?? 0);
    const before = ({ wood: 10, grain: 9, clay: 2 })[g];
    assert.ok(Math.abs(after - before) < 1e-9);
  }
  assert.ok(res.volume.wood > 0);
});

test('units: spawn, find, own, and have labour refilled each turn', () => {
  let s = dispatch(founded(1), { type: 'chooseCiv', civ: null }).state;
  const civ = Object.values(s.civs)[0];
  const m = s.markets[civ.marketId];
  const u = spawnUnit(s, { owner: civ.id, type: 'test', q: m.q, r: m.r, labourMax: 4, test: { x: 1 } });
  u.labour = 0;
  assert.deepEqual(unitsAt(s, m).map((x) => x.id), [u.id]);
  assert.equal(civOf(s, u.id), civ);
  s = dispatch(s, { type: 'endTurn' }).state;
  assert.equal(s.units[u.id].labour, 4);
  assert.deepEqual(s.units[u.id].test, { x: 1 });
  assert.ok(tileAt(s, m));
});

test('a seed plus the action list replays exactly through founding and into the main game', () => {
  const config = { seed: 3, players: 2 };
  const { state, turns } = runPregameCounted(newGame(config));
  const history = [...Array.from({ length: turns }, () => ({ type: 'endTurn' })),
    { type: 'chooseCiv', civ: Number(Object.keys(state.civs)[0]) }, { type: 'endTurn' }, { type: 'endTurn' }];
  let s = state;
  for (const a of history.slice(turns)) s = dispatch(s, a).state;
  const again = replay(config, history);
  s.events = again.events = [];
  assert.deepEqual(again, s);
});

test('territory comes from troops, forts, then city radius; cities transfer; stores give what they have', () => {
  const s = dispatch(founded(1), { type: 'chooseCiv', civ: null }).state;
  const [a, b] = Object.values(s.civs);
  const m = s.markets[a.marketId];
  const id = s.nextId++;
  s.cities[id] = { id, owner: a.id, name: 'Test', q: m.q, r: m.r };
  a.cityIds.push(id);
  const near = neighbors(m)[0];
  assert.equal(territoryOf(s, near), a.id);
  tileAt(s, near).fort = { owner: b.id };
  assert.equal(territoryOf(s, near), b.id);
  tileAt(s, near).held = a.id;
  assert.equal(territoryOf(s, near), a.id);
  assert.equal(territoryOf(s, { q: m.q + 9, r: m.r }), null);
  transferCity(s, id, b.id);
  assert.deepEqual([a.cityIds, b.cityIds], [[], [id]]);
  putGoods(s, a.id, { grain: 3, wood: 1 });
  assert.equal(takeGoods(s, a.id, 'grain', 5), 3);
  assert.deepEqual(a.store, { wood: 1 });
});
