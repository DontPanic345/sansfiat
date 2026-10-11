import assert from 'node:assert/strict';
import { test } from 'node:test';
import { FOODS, MARKET } from '../src/data.js';
import { distance, fromPixel, key, neighbors, toPixel } from '../src/hex.js';
import { newGame, replay, runPregame } from '../src/game.js';
import { clear, utility } from '../src/market.js';
import { dispatch, nutrition, passable, total } from '../src/rules.js';

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

test('rivers flow over land to the sea', () => {
  const { map } = newGame({ seed: 4 });
  const rivers = Object.values(map.tiles).filter((t) => t.river);
  assert.ok(rivers.length > 0);
  for (const t of rivers) {
    assert.ok(passable({ map }, t));
    if (t.flow) assert.equal(distance(t, map.tiles[t.flow]), 1);
  }
});

test('a varied diet feeds a clan on far less food than one food alone', () => {
  assert.ok(nutrition({ fish: 12 }, 10) < 10);
  assert.ok(nutrition({ fish: 4, grain: 4, game: 4 }, 10) >= 10);
});

test('market clearing conserves goods, keeps each trade value-neutral, and leaves both sides better off', () => {
  const baskets = { a: { fish: 10, grain: 1 }, b: { fish: 1, grain: 8 }, c: { fish: 3, fruit: 6 } };
  const { prices, after, volume } = clear(baskets);
  for (const f of FOODS) {
    const before = total(Object.fromEntries(Object.entries(baskets).map(([id, b]) => [id, b[f] ?? 0])));
    const now = total(Object.fromEntries(Object.entries(after).map(([id, b]) => [id, b[f] ?? 0])));
    assert.ok(Math.abs(before - now) < 1e-9, `${f} not conserved`);
  }
  const worth = (b) => Object.entries(b).reduce((s, [f, v]) => s + prices[f] * v, 0);
  for (const id of Object.keys(baskets)) {
    assert.ok(Math.abs(worth(after[id]) - worth(baskets[id])) < 1e-3 * worth(baskets[id]), `${id} gained value`);
    assert.ok(utility(after[id]) > utility(baskets[id]), `${id} not better off`);
  }
  assert.ok(volume.fish > 0 && volume.grain > 0);
});

test('a lone trader or a single good means no trade', () => {
  assert.deepEqual(clear({ a: { fish: 5, grain: 1 } }).volume, {});
  assert.deepEqual(clear({ a: { fish: 5 }, b: { fish: 2 } }).volume, {});
});

test('dispatch does not mutate input', () => {
  const s = newGame({ seed: 1 });
  const snapshot = structuredClone(s);
  assert.equal(dispatch(s, { type: 'endTurn' }).error, undefined);
  assert.deepEqual(s, snapshot);
  assert.match(dispatch(s, { type: 'nope' }).error, /unknown/);
});

test('replaying the action history reproduces the state', () => {
  const config = { seed: 5 };
  let s = newGame(config);
  const history = [];
  for (let i = 0; i < 30; i++) {
    s = dispatch(s, { type: 'endTurn' }).state;
    history.push({ type: 'endTurn' });
  }
  assert.deepEqual(replay(config, history), s);
});

test('the pre-game spreads clans, forms markets and hands them to players', () => {
  for (const seed of [1, 2]) {
    let s = newGame({ seed, players: 3 });
    for (let i = 0; i < 500 && s.phase === 'pregame'; i++) {
      s = runPregame(s, 25);
      const clans = Object.values(s.clans);
      assert.ok(clans.every((c) => passable(s, c)), `clan on impassable tile (seed ${seed})`);
      assert.equal(new Set(clans.map(key)).size, clans.length, `clans share a tile (seed ${seed})`);
      assert.ok(clans.every((c) => c.pop >= 1));
    }
    assert.equal(s.phase, 'game');
    assert.ok(Object.keys(s.clans).length > 50, 'clans should have spread');
    const owned = Object.values(s.markets).filter((m) => m.owner != null);
    assert.equal(owned.length, 3);
    assert.ok(owned.some((m) => m.volume > 0), 'some trade should have happened');
    const ms = Object.values(s.markets);
    for (const a of ms) for (const b of ms) if (a !== b) assert.ok(distance(a, b) >= MARKET.spacing);
  }
});

test('clans walk one tile a turn over land, and daughters start next to their parent', () => {
  let s = newGame({ seed: 6 });
  for (let i = 0; i < 150; i++) {
    const before = structuredClone(s.clans);
    s = dispatch(s, { type: 'endTurn' }).state;
    for (const c of Object.values(s.clans)) {
      assert.ok(passable(s, c), `${c.name} on impassable tile`);
      const was = before[c.id];
      if (was) assert.ok(distance(was, c) <= 1, `${c.name} jumped ${distance(was, c)} tiles`);
      else assert.ok(Object.values(before).some((p) => distance(p, c) === 1), `${c.name} appeared away from any parent`);
    }
  }
});
