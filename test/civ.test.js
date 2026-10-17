import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/civ/rules.js';
import { CIV } from '../src/modules/civ/data.js';
import { newGame } from '../src/game.js';
import { runPregame } from '../src/game.js';
import { dispatch, runTurn } from '../src/rules.js';
import { civClans, civCities } from '../src/core.js';
import { distance } from '../src/hex.js';

let base;
function mainGame() {
  base ??= runPregame(newGame({ seed: 1, players: 2 }));
  const r = dispatch(structuredClone(base), { type: 'chooseCiv', civ: Number(Object.keys(base.civs)[0]) });
  assert.ok(!r.error, r.error);
  return r.state;
}
const turns = (s, n) => { for (let i = 0; i < n; i++) runTurn(s); return s; };

test('civ module is registered and initialises its state', () => {
  assert.equal(mod.name, 'civ');
  assert.ok(newGame({ seed: 1 }).mods.civ);
});

test('start: each civ has a capital and owned clans', () => {
  const s = mainGame();
  for (const civ of Object.values(s.civs)) {
    assert.equal(civCities(s, civ.id).length, 1);
    assert.ok(civCities(s, civ.id)[0].founding);
    assert.ok(civClans(s, civ.id).length >= 1);
  }
  turns(s, 5);
  assert.ok(Object.values(s.civs).every((c) => typeof c.store === 'object'));
});

test('materials and pottery: labour gathers wood/stone/clay; pottery needs the discovery', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  // plant forest, hills and river around the capital
  const near = Object.values(s.map.tiles).filter((t) => distance(t, city) === 1 && t.terrain !== 'ocean');
  near[0].terrain = 'forest'; near[1].terrain = 'hills'; near[2].river = true;
  turns(s, 3);
  assert.ok((civ.store.wood ?? 0) > 0 && (civ.store.stone ?? 0) > 0 && (civ.store.clay ?? 0) > 0);
  s.mods.civ.disc[civ.id].known.pottery = true;
  turns(s, 3);
  assert.ok((civ.store.pottery ?? 0) > 0, 'pottery crafted');
});

test('projects: queue, labour, completion; granary cuts spoilage', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  civ.store = { wood: 10, clay: 10, stone: 10 };
  const r = dispatch(s, { type: 'setProject', civ: civ.id, city: city.id, project: 'granary' });
  assert.ok(!r.error);
  let t = r.state;
  assert.equal(t.civs[civ.id].store.wood, 6);
  assert.equal(dispatch(t, { type: 'setProject', civ: civ.id, city: city.id, project: 'wall' }).error, 'city already has a project');
  turns(t, 15);
  assert.ok(t.cities[city.id].buildings.granary);
  // spoilage: with granary food keeps more
  const plain = mainGame();
  const c2 = Object.values(plain.civs)[0];
  const lose = (st, id) => { st.civs[id].store = { grain: 10 }; mod.turnSteps.find((x) => x.name === 'civ upkeep').run(st); return st.civs[id].store.grain; };
  assert.ok(lose(t, civ.id) > lose(plain, c2.id));
});

test('storehouse raises the store cap, wall flag is exposed', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  for (const c of civClans(s, civ.id)) c.since = 0;
  civ.store = { wood: 500 };
  mod.turnSteps.find((x) => x.name === 'civ upkeep').run(s);
  assert.equal(civ.store.wood, CIV.baseCap);
  city.buildings.storehouse = true; city.buildings.wall = true;
  civ.store = { wood: 500 };
  mod.turnSteps.find((x) => x.name === 'civ upkeep').run(s);
  assert.equal(civ.store.wood, CIV.baseCap + CIV.storehouseCap);
  assert.ok(mod.hasWall(city));
});

test('civ posts surplus at its market and receives goods back', () => {
  const s = mainGame();
  const [a] = Object.values(s.civs);
  a.store = { wood: 30 };
  const m = s.markets[a.marketId];
  m.posts.other = { stone: 20 };
  mod.turnSteps.find((x) => x.name === 'civ post').run(s);
  assert.equal(a.store.wood, CIV.reserve.material);
  assert.ok(m.posts[a.id].wood > 0);
});

test('expansion: daughter site respects city spacing and player choice', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const clan = civClans(s, civ.id)[0];
  const city = civCities(s, civ.id)[0];
  const options = [{ k: `${city.q + 1},${city.r}`, score: 5 }, { k: `${city.q + 5},${city.r}`, score: 1 }];
  assert.equal(mod.hooks.daughterSite(s, clan, { options }), options[1].k, 'auto-pick avoids near sites');
  dispatch(s, { type: 'setDaughterSite', civ: civ.id, site: options[0].k });
  s.mods.civ.siteChoice[civ.id] = options[0].k;
  assert.equal(mod.hooks.daughterSite(s, clan, { options }), options[0].k);
  assert.equal(mod.hooks.daughterSite(s, { owner: null }, { options }), undefined);
});

test('new city founded by a settled civ clan at least minSpacing away', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  const clan = civClans(s, civ.id)[0];
  const far = Object.values(s.map.tiles).find((t) => distance(t, city) >= 4 && Object.values(s.cities).every((c) => distance(c, t) >= 3));
  Object.assign(clan, { q: far.q, r: far.r, route: [], since: 99, pop: 10 });
  mod.turnSteps.find((x) => x.name === 'civ upkeep').run(s);
  assert.equal(civCities(s, civ.id).length, 2);
});

test('years: drought and bumper scale yield, deterministically, and expire', () => {
  const s = mainGame();
  const t = Object.values(s.map.tiles)[0];
  const r = `${Math.floor(t.q / CIV.region)},${Math.floor(t.r / CIV.region)}`;
  s.mods.civ.years[r] = { factor: 0.5, left: 1 };
  assert.equal(mod.hooks.yieldFactor(s, t), 0.5);
  const a = structuredClone(s), b = structuredClone(s);
  turns(a, 30); turns(b, 30);
  assert.deepEqual(a.mods.civ.years, b.mods.civ.years);
  s.mods.civ.years = {};
  const w = mod.turnSteps.find((x) => x.name === 'civ weather');
  let seen = false;
  for (let i = 0; i < 300 && !seen; i++) { w.run(s); seen = Object.keys(s.mods.civ.years).length > 0; }
  assert.ok(seen);
});

test('leanings: trade and gifts build leaning; clan joins past the threshold', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const clan = Object.values(s.clans).find((c) => c.owner !== civ.id) ?? Object.values(s.clans)[0];
  clan.owner = null;
  clan.market = civ.marketId;
  mod.turnSteps.find((x) => x.name === 'civ collect').run(s);
  assert.ok(clan.leaning[civ.id] > 0);
  civ.store = { grain: 40 };
  const r = dispatch(s, { type: 'giftFood', civ: civ.id, clan: clan.id, amount: 30 });
  assert.ok(!r.error);
  const c = r.state.clans[clan.id];
  assert.ok(c.leaning[civ.id] > CIV.joinAt - 0.01 + 0.0);
  mod.turnSteps.find((x) => x.name === 'civ join').run(r.state);
  assert.equal(r.state.clans[clan.id].owner, civ.id);
  assert.ok(r.state.notes[clan.id].some((n) => /joined/.test(n.text)));
});

test('discoveries: practice teaches pottery and herding, and they spread', () => {
  const s = mainGame();
  const [a, b] = Object.values(s.civs);
  s.mods.civ.disc[a.id].practice.pottery = CIV.practice.pottery;
  const d = mod.turnSteps.find((x) => x.name === 'civ discoveries');
  d.run(s);
  assert.ok(s.mods.civ.disc[a.id].known.pottery);
  s.markets[b.marketId].q = s.markets[a.marketId].q; s.markets[b.marketId].r = s.markets[a.marketId].r;
  for (let i = 0; i < 40; i++) d.run(s);
  assert.ok(s.mods.civ.disc[b.id].known.pottery, 'spread to the neighbour');
  s.mods.civ.disc[a.id].practice.herding = 99;
  d.run(s);
  assert.ok(s.mods.civ.disc[a.id].known.herding);
});

test('victory: conquest by holding every founding city; prosperity at the limit', () => {
  const s = mainGame();
  const [a, b] = Object.values(s.civs);
  const vic = mod.turnSteps.find((x) => x.name === 'civ victory');
  vic.run(s);
  assert.equal(s.winner, undefined);
  const cb = civCities(s, b.id)[0];
  cb.owner = a.id;
  assert.equal(mod.conquerer(s), a.id);
  vic.run(s);
  assert.equal(s.winner, a.id);
  const p = mainGame();
  p.turn = p.mods.civ.startTurn + CIV.turnLimit;
  vic.run(p);
  assert.ok(p.winner != null);
  assert.equal(p.mods.civ.winKind, 'prosperity');
});

test('AI civs play: projects get queued and a long run stays valid', () => {
  const s = mainGame();
  const [a] = Object.values(s.civs);
  a.store = { wood: 10, clay: 10, stone: 10 };
  assert.ok(mod.ai(s, a.id).some((x) => x.type === 'setProject'));
  turns(s, 40);
  assert.ok(Object.keys(s.clans).length > 0);
});
