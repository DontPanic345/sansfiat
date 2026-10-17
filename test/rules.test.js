import assert from 'node:assert/strict';
import { test } from 'node:test';
import { FOODS, MARKET } from '../src/data.js';
import { distance, fromPixel, key, neighbors, toPixel } from '../src/hex.js';
import { newGame, replay, runPregame } from '../src/game.js';
import { clear, utility } from '../src/market.js';
import { dispatch, hearths, nutrition, passable, total } from '../src/rules.js';

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

// Every clan holds a connected patch of land including its home, and no tile has two holders.
function checkLand(s) {
  const held = Object.values(s.map.tiles).filter((t) => t.claim != null);
  const clans = Object.values(s.clans);
  assert.equal(held.length, clans.reduce((a, c) => a + c.land.length, 0));
  for (const c of clans) {
    for (const k of c.land) {
      assert.equal(s.map.tiles[k].claim, c.id);
      assert.ok(passable(s, s.map.tiles[k]));
    }
    if (c.route.length || !c.land.length) continue;
    assert.ok(c.land.includes(key(c)), `${c.name} is off its land`);
    const seen = new Set([c.land[0]]);
    for (const k of seen) for (const n of neighbors(s.map.tiles[k])) if (c.land.includes(key(n))) seen.add(key(n));
    assert.equal(seen.size, c.land.length, `${c.name} land is split`);
  }
}

test('the pre-game spreads clans over land of their own, and ends when grain is domesticated', () => {
  for (const seed of [1, 2]) {
    let s = newGame({ seed, players: 3 });
    let gathered = false;
    for (let i = 0; i < 500 && s.phase === 'pregame'; i++) {
      s = runPregame(s, 25);
      const clans = Object.values(s.clans);
      assert.ok(clans.every((c) => passable(s, c)), `clan on impassable tile (seed ${seed})`);
      assert.equal(new Set(clans.map(key)).size, clans.length, `clans share a tile (seed ${seed})`);
      assert.ok(clans.every((c) => c.pop >= 1));
      checkLand(s);
      if (s.phase === 'pregame') {
        assert.ok(Object.values(s.markets).every((m) => m.temporary && m.owner == null));
        gathered ||= Object.keys(s.markets).length > 0;
      }
    }
    assert.ok(gathered, 'clans should gather to trade in the pre-game');
    assert.equal(s.phase, 'game');
    assert.equal(hearths(s).length, 3);
    assert.ok(Object.keys(s.clans).length > 50, 'clans should have spread');
    const owned = Object.values(s.markets).filter((m) => m.owner != null);
    assert.equal(owned.length, 3);
    assert.ok(Object.values(s.markets).every((m) => !m.temporary));
    for (let i = 0; i < 5; i++) s = dispatch(s, { type: 'endTurn' }).state;
    assert.ok(owned.some((m) => s.markets[m.id].volume > 0), 'player markets should trade');
    const ms = Object.values(s.markets);
    for (const a of ms) for (const b of ms) if (a !== b) assert.ok(distance(a, b) >= MARKET.spacing);
  }
});

test('hunted-out game comes back slowly; held grain gets tamer', () => {
  let s = newGame({ seed: 3 });
  const tiles = Object.values(s.map.tiles);
  const t = tiles.find((x) => x.cap.game && x.cap.grain && x.claim == null);
  t.stock.game = 0.05 * t.cap.game;
  t.stock.grain = 0.05 * t.cap.grain;
  s = dispatch(s, { type: 'endTurn' }).state;
  const after = s.map.tiles[key(t)];
  assert.ok(after.stock.game / after.cap.game < after.stock.grain / after.cap.grain / 2);

  const held = Object.values(s.map.tiles).find((x) => x.claim != null);
  Object.assign(held, { wildGrain: 10, tame: 0, occ: 100 });
  held.cap.grain = 10;
  s = dispatch(s, { type: 'endTurn' }).state;
  assert.ok(s.map.tiles[key(held)].tame > 0);
  assert.ok(s.map.tiles[key(held)].cap.grain > 10);
});

test('clans walk one tile a turn over land, and daughters start next to or on their parent land', () => {
  let s = newGame({ seed: 6 });
  for (let i = 0; i < 150; i++) {
    const before = structuredClone(s.clans);
    s = dispatch(s, { type: 'endTurn' }).state;
    for (const c of Object.values(s.clans)) {
      assert.ok(passable(s, c), `${c.name} on impassable tile`);
      const was = before[c.id];
      if (was) assert.ok(distance(was, c) <= 1, `${c.name} jumped ${distance(was, c)} tiles`);
      else assert.ok(Object.values(before).some((p) => distance(p, c) === 1 || p.land.some((k) => distance(s.map.tiles[k], c) <= 1)), `${c.name} appeared away from any parent`);
    }
  }
});
