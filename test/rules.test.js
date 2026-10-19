import assert from 'node:assert/strict';
import { test } from 'node:test';
import { FOODS, FOUNDING, MARKET } from '../src/data.js';
import {
  catchment, forage, formMarkets, foundCivs, foundMarket, foundingSites, landComponents, marketReach, reaches, settle, trade,
} from '../src/clans.js';
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
    assert.equal(s.phase, 'choose');
    assert.ok(hearths(s).length >= 3);
    assert.ok(Object.keys(s.clans).length > 50, 'clans should have spread');
    const owned = Object.values(s.markets).filter((m) => m.owner != null);
    assert.equal(owned.length, 3);
    assert.deepEqual(owned.map((m) => s.civs[m.owner].marketId).sort(), owned.map((m) => m.id).sort());
    assert.ok(Object.values(s.markets).every((m) => !m.temporary));
    assert.match(dispatch(s, { type: 'endTurn' }).error, /choose/);
    s = dispatch(s, { type: 'chooseCiv', civ: null }).state;
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

// ---- finishing-pass fixes --------------------------------------------------------------------

test('only foods feed: materials count for nothing, and clans do not buy them', () => {
  assert.equal(nutrition({ stone: 10, clay: 5, pottery: 3 }, 10), 0);
  assert.equal(nutrition({ fish: 4, grain: 4, stone: 9 }, 10), nutrition({ fish: 4, grain: 4 }, 10));
  // A civ posts stone; the clans want only foods, so the stone stays with the civ.
  const baskets = { civ: { stone: 10, grain: 2 }, a: { fish: 6, grain: 1 }, b: { grain: 6, fruit: 2 } };
  const res = clear(baskets, {}, { wants: { a: FOODS, b: FOODS } });
  assert.ok((res.after.a.stone ?? 0) < 1e-9 && (res.after.b.stone ?? 0) < 1e-9);
  assert.ok(Math.abs(res.after.civ.stone - 10) < 1e-9);
  // Two posters can still swap stone for food.
  const two = clear({ civ: { stone: 10 }, other: { grain: 10 }, a: { fish: 4, grain: 4 } }, {}, { wants: { a: FOODS } });
  assert.ok(two.after.other.stone > 0 && two.after.civ.grain > 0);
});

test('in the main game no clan ends up holding or eating non-food goods', () => {
  let s = dispatch(runPregame(newGame({ seed: 3, players: 3 })), { type: 'chooseCiv', civ: null }).state;
  for (let i = 0; i < 40; i++) {
    s = dispatch(s, { type: 'endTurn' }).state;
    for (const c of Object.values(s.clans))
      for (const b of [c.basket, c.store]) for (const g of Object.keys(b)) assert.ok(FOODS.includes(g), `${c.name} holds ${g}`);
  }
});

test('founding: thriving markets 8+ apart, each reaching a hearth, all on one landmass (seeds 1-5)', () => {
  for (const seed of [1, 2, 3, 4, 5]) {
    const s = runPregame(newGame({ seed, players: 3 }));
    assert.equal(s.phase, 'choose');
    const civs = Object.values(s.civs);
    assert.equal(civs.length, 3, `seed ${seed}`);
    const { comp } = landComponents(s);
    const ms = civs.map((c) => s.markets[c.marketId]);
    assert.equal(new Set(ms.map((m) => comp.get(key(m)))).size, 1, `seed ${seed}: founding markets on separate landmasses`);
    for (const m of ms) {
      assert.ok(m.traders >= FOUNDING.minTraders, `seed ${seed}: market ${key(m)} has ${m.traders} traders`);
      const reach = marketReach(s, m);
      assert.ok(Object.values(s.map.tiles).some((t) => t.tame === 1 && reach.has(key(t))), `seed ${seed}: no hearth in reach of ${key(m)}`);
      for (const o of ms) if (o !== m) assert.ok(distance(m, o) >= FOUNDING.spacing, `seed ${seed}: markets ${distance(m, o)} apart`);
    }
    for (const c of civs) assert.match(s.notes[c.id].at(-1).text, /grain domesticated/);
    // Markets made at founding have enough clans in reach.
    for (const m of Object.values(s.markets))
      if (m.owner == null && m.founded === s.turn - 1) assert.ok(catchment(s, m).clans >= MARKET.gathering.minClans);
  }
});

test('the founding fallback still founds every civ when the pre-game runs out', () => {
  const s = runPregame(newGame({ seed: 2, players: 4 }), 40);
  assert.equal(s.phase, 'pregame');
  assert.deepEqual(foundingSites(s), []);
  foundCivs(s);
  assert.equal(Object.keys(s.civs).length, 4);
  const ms = Object.values(s.civs).map((c) => s.markets[c.marketId]);
  for (const a of ms) for (const b of ms) if (a !== b) assert.ok(distance(a, b) >= FOUNDING.spacing);
});

test('neutral markets fade when unused in the main game; founding markets do not', () => {
  const s = dispatch(runPregame(newGame({ seed: 1, players: 3 })), { type: 'chooseCiv', civ: null }).state;
  const neutral = Object.values(s.markets).find((m) => m.owner == null);
  const owned = Object.values(s.markets).find((m) => m.owner != null);
  neutral.activity = 0;
  owned.activity = 0;
  formMarkets(s);
  assert.equal(s.markets[neutral.id], undefined);
  assert.ok(s.markets[owned.id]);
  assert.ok(Object.values(s.clans).every((c) => c.market == null || s.markets[c.market]));
});

test('a market with nothing to clear keeps its last real ratios', () => {
  assert.equal(clear({ a: { fish: 5, grain: 1 } }).cleared, false);
  const s = newGame({ seed: 1 });
  const m = foundMarket(s, Object.values(s.clans)[0]);
  m.posts = { 999: { fish: 5, grain: 2 } };
  trade(s);
  assert.deepEqual(m.prices, {});
  m.prices = { fish: 2, grain: 0.5 };
  m.posts = { 999: { fish: 5, grain: 2, fruit: 1 } };
  trade(s);
  assert.deepEqual(m.prices, { fish: 2, grain: 0.5 });
  assert.deepEqual(m.history.at(-1).prices, { fish: 2, grain: 0.5 });
});

test('deserts are about a fifth of the land', () => {
  for (const seed of [1, 2, 3, 4, 5]) {
    const land = Object.values(newGame({ seed }).map.tiles).filter((t) => t.terrain !== 'ocean');
    const share = land.filter((t) => t.terrain === 'desert').length / land.length;
    assert.ok(share > 0.17 && share < 0.23, `seed ${seed}: deserts ${share.toFixed(2)} of land`);
  }
});

test('players are clamped to 2..4', () => {
  assert.equal(newGame({ players: 9 }).config.players, 4);
  assert.equal(newGame({ players: 1 }).config.players, 2);
  assert.equal(newGame({ players: 3 }).config.players, 3);
});

test('endTurn is rejected once there is a winner', () => {
  const s = dispatch(runPregame(newGame({ seed: 1, players: 2 })), { type: 'chooseCiv', civ: null }).state;
  s.winner = Number(Object.keys(s.civs)[0]);
  assert.equal(dispatch(s, { type: 'endTurn' }).error, 'game over');
});

test('drafted people do not forage but still eat', () => {
  const base = runPregame(newGame({ seed: 1 }), 60);
  const run = (drafted) => {
    const s = structuredClone(base);
    if (drafted) for (const c of Object.values(s.clans)) c.drafted = drafted(c);
    forage(s, reaches(s));
    return s;
  };
  const harvests = (s) => Object.values(s.clans).map((c) => c.harvest);
  const harvested = (s) => Object.values(s.clans).reduce((a, c) => a + total(c.harvest), 0);
  const free = run(null);
  assert.deepEqual(harvests(run(() => 0)), harvests(free));
  assert.ok(harvested(run((c) => c.pop / 2)) < harvested(free));
  const all = run((c) => c.pop);
  assert.equal(harvested(all), 0);
  // Eating uses the whole pop: with nothing harvested, clans living off a thin store go hungry.
  const fed = (s) => Object.values(s.clans).filter((c) => c.fed).length;
  const freeFed = structuredClone(free);
  settle(all, reaches(all));
  settle(freeFed, reaches(freeFed));
  assert.ok(fed(all) < fed(freeFed));
});
