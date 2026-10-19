import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/civ/rules.js';
import { CIV } from '../src/modules/civ/data.js';
import { CITY, CLAN } from '../src/data.js';
import { newGame, runPregame } from '../src/game.js';
import { trade } from '../src/clans.js';
import { dispatch, runTurn } from '../src/rules.js';
import { civClans, civCities, passable, tileAt, transferCity } from '../src/core.js';
import { distance, key } from '../src/hex.js';

let base;
function mainGame(human = true) {
  base ??= runPregame(newGame({ seed: 1, players: 2 }));
  const r = dispatch(structuredClone(base), { type: 'chooseCiv', civ: human ? Number(Object.keys(base.civs)[0]) : null });
  assert.ok(!r.error, r.error);
  return r.state;
}
const turns = (s, n) => { for (let i = 0; i < n; i++) runTurn(s); return s; };
const step = (name) => mod.turnSteps.find((x) => x.name === name).run;
const unaligned = (s, civ) => {
  const c = Object.values(s.clans).find((x) => x.owner !== civ.id);
  Object.assign(c, { owner: null, leaning: {}, starving: false, fed: true });
  return c;
};

test('civ module is registered and initialises its state', () => {
  assert.equal(mod.name, 'civ');
  const m = newGame({ seed: 1 }).mods.civ;
  for (const k of ['posting', 'trade', 'labour', 'gifted', 'lost', 'citySite', 'siteChoice']) assert.ok(m[k], k);
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

test('drafting: workers follow what cities need, capped per clan, starving clans spared', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  const clans = civClans(s, civ.id);
  for (const c of clans) Object.assign(c, { starving: false, pop: 10 });
  clans[0].starving = true;
  civ.store = { wood: 99, stone: 99, clay: 99, grain: 999 };
  step('civ draft')(s);
  const idle = s.mods.civ.labour[civ.id];
  city.project = { kind: 'wall', labour: 0 };
  step('civ draft')(s);
  const busy = s.mods.civ.labour[civ.id];
  assert.ok(busy.wanted >= idle.wanted + CIV.workers.project, 'a project wants workers');
  assert.ok(busy.drafted > idle.drafted);
  assert.equal(clans[0].drafted, 0, 'starving clan not drafted');
  for (const c of clans) assert.ok(c.drafted <= c.pop * CIV.draftMax + 1e-9);
  const sum = clans.reduce((a, c) => a + c.drafted, 0);
  assert.ok(Math.abs(sum - busy.drafted) < 1e-9);
  // labour goes into the project
  step('civ labour')(s);
  assert.ok(city.project.labour > 0);
});

test('materials and pottery: workers gather wood/stone/clay; pottery needs the discovery', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  const near = Object.values(s.map.tiles).filter((t) => distance(t, city) === 1 && t.terrain !== 'ocean');
  near[0].terrain = 'forest'; near[1].terrain = 'hills'; near[2].river = true;
  for (let i = 0; i < 3; i++) {
    for (const c of civClans(s, civ.id)) c.starving = false;
    step('civ draft')(s); step('civ labour')(s);
  }
  assert.ok((civ.store.wood ?? 0) > 0 && (civ.store.stone ?? 0) > 0 && (civ.store.clay ?? 0) > 0);
  s.mods.civ.disc[civ.id].known.pottery = true;
  for (const c of civClans(s, civ.id)) c.starving = false;
  step('civ draft')(s); step('civ labour')(s);
  assert.ok((civ.store.pottery ?? 0) > 0, 'pottery crafted');
});

test('projects: queue, labour, completion with an event; costs checked', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  civ.store = { wood: 3 };
  assert.match(dispatch(s, { type: 'setProject', civ: civ.id, city: city.id, project: 'granary' }).error, /not enough in the store: wood 3.0\/4, clay 0.0\/2/);
  civ.store = { wood: 10, clay: 10, stone: 10 };
  const r = dispatch(s, { type: 'setProject', civ: civ.id, city: city.id, project: 'granary' });
  assert.ok(!r.error);
  const t = r.state;
  assert.equal(t.civs[civ.id].store.wood, 6);
  assert.equal(dispatch(t, { type: 'setProject', civ: civ.id, city: city.id, project: 'wall' }).error, 'city already has a project');
  t.cities[city.id].project.labour = CIV.projects.granary.labour - 0.01;
  t.events = [];
  for (const c of civClans(t, civ.id)) c.starving = false;
  step('civ draft')(t); step('civ labour')(t);
  assert.ok(t.cities[city.id].buildings.granary);
  assert.ok(t.events.some((e) => /! .*finishes a granary/.test(e)), 'human alert for a finished project');
});

test('granary and pottery cut spoilage of the civ store and of member clans near it', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  const clan = civClans(s, civ.id).find((c) => distance(c, city) <= CIV.granaryReach);
  const run = (setup) => {
    const t = structuredClone(s);
    setup(t);
    for (const c of civClans(t, civ.id)) c.fed = false;
    t.civs[civ.id].store = { grain: 10 };
    Object.assign(t.clans[clan.id], { store: { grain: 10 }, fed: false });
    step('civ upkeep')(t);
    return { store: t.civs[civ.id].store.grain, clan: t.clans[clan.id].store.grain };
  };
  const plain = run(() => {});
  const granary = run((t) => { t.cities[city.id].buildings.granary = true; });
  const pottery = run((t) => { t.mods.civ.disc[civ.id].known.pottery = true; });
  assert.ok(granary.store > plain.store && pottery.store > plain.store);
  assert.ok(granary.clan > plain.clan && pottery.clan > plain.clan);
  assert.ok(Math.abs(plain.store - 10 * (1 - CLAN.spoilage)) < 1e-9);
  assert.ok(Math.abs(granary.store - 10 * (1 - CLAN.spoilage * CIV.granarySpoil)) < 1e-9);
});

test('store cap counts every good; a storehouse raises it', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  for (const c of civClans(s, civ.id)) Object.assign(c, { since: 0, fed: false });
  civ.store = { wood: 100, stone: 100 };
  step('civ upkeep')(s);
  assert.ok(Math.abs(civ.store.wood + civ.store.stone - CIV.baseCap) < 1e-9);
  assert.ok(s.mods.civ.overflow[civ.id] > 0);
  civ.store = { wood: 100, grain: 40 };
  step('civ upkeep')(s);
  assert.ok(Math.abs(civ.store.grain - 40 * (1 - CLAN.spoilage)) < 1e-9, 'food is kept, materials rot first');
  assert.ok(Math.abs(civ.store.wood + civ.store.grain - CIV.baseCap) < 1e-9);
  city.buildings.storehouse = true;
  civ.store = { wood: 100, stone: 100 };
  step('civ upkeep')(s);
  assert.ok(Math.abs(civ.store.wood + civ.store.stone - CIV.baseCap - CIV.storehouseCap) < 1e-9);
});

test('levy: fed member clans hand a share of their store to the civ', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const clan = civClans(s, civ.id)[0];
  civ.store = {};
  Object.assign(clan, { fed: true, store: { fish: 5 } });
  step('civ upkeep')(s);
  assert.ok(civ.store.fish > 0);
  assert.ok(clan.store.fish < 5 * 1.0001);
});

test('posting: surplus over the reserve is posted; setPosting changes the reserve', () => {
  const s = mainGame();
  const [a] = Object.values(s.civs);
  const m = s.markets[a.marketId];
  a.store = { grain: 30 };
  step('civ post')(s);
  const keep = mod.keepOf(s, a.id, 'grain');
  assert.ok(Math.abs(a.store.grain - keep) < 1e-9);
  assert.ok(Math.abs(s.mods.civ.trade[a.id].posted.grain - (30 - keep)) < 1e-9);
  assert.ok(m.posts[a.id].grain > 0);
  for (const bad of [{ good: 'gold', keep: 1 }, { good: 'grain', keep: -1 }, { good: 'grain', keep: NaN }, { good: 'grain', keep: '5' }])
    assert.ok(dispatch(s, { type: 'setPosting', civ: a.id, ...bad }).error, JSON.stringify(bad));
  const t = dispatch(s, { type: 'setPosting', civ: a.id, good: 'grain', keep: 25 }).state;
  t.civs[a.id].store = { grain: 30 };
  step('civ post')(t);
  assert.ok(Math.abs(t.civs[a.id].store.grain - 25) < 1e-9);
  const u = dispatch(t, { type: 'setPosting', civ: a.id, good: 'grain', keep: null }).state;
  assert.equal(u.mods.civ.posting[a.id].grain, undefined);
  // material default: keep what unbuilt projects need
  assert.equal(mod.keepOf(u, a.id, 'stone'), Math.max(...Object.values(CIV.projects).map((p) => p.goods.stone ?? 0)));
});

test('market: civ gets goods back, clans bring materials, materials never reach clan baskets', () => {
  const s = mainGame();
  const [a] = Object.values(s.civs);
  const m = s.markets[a.marketId];
  const clan = Object.values(s.clans).find((c) => c.land.length);
  Object.assign(clan, { market: m.id, starving: false, basket: { grain: 5 } });
  s.map.tiles[clan.land[0]].terrain = 'forest';
  a.store = { fish: 30 };
  step('civ post')(s);
  assert.ok(clan.basket.wood > 0, 'clan brings wood');
  trade(s);
  step('civ collect')(s);
  assert.ok(Object.keys(clan.basket).every((g) => !['wood', 'stone', 'clay'].includes(g)));
  assert.ok((a.store.wood ?? 0) > 0, 'civ bought wood with fish');
  assert.ok(Object.keys(s.mods.civ.trade[a.id].received).length);
});

test('gifts: validated, capped per turn, diminishing, and worth more to a starving clan', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const clan = unaligned(s, civ);
  clan.pop = 8;
  civ.store = { grain: 100 };
  for (const amount of [NaN, -5, 0, '5', Infinity, 25])
    assert.ok(dispatch(s, { type: 'giftFood', civ: civ.id, clan: clan.id, amount }).error, String(amount));
  const g10 = mod.giftGain(s, clan, civ.id, 10);
  const g5 = mod.giftGain(s, clan, civ.id, 5);
  assert.ok(g10 < 2 * g5, 'diminishing');
  let t = dispatch(s, { type: 'giftFood', civ: civ.id, clan: clan.id, amount: 5 }).state;
  t = dispatch(t, { type: 'giftFood', civ: civ.id, clan: clan.id, amount: 5 }).state;
  assert.ok(Math.abs(t.clans[clan.id].leaning[civ.id] - g10) < 1e-9, 'two halves buy no more than one whole');
  assert.match(dispatch(t, { type: 'giftFood', civ: civ.id, clan: clan.id, amount: 1 }).error, /all it can take/);
  t.turn++;
  assert.ok(!dispatch(t, { type: 'giftFood', civ: civ.id, clan: clan.id, amount: 1 }).error, 'room again next turn');
  const hungry = structuredClone(s);
  hungry.clans[clan.id].starving = true;
  assert.ok(mod.giftGain(hungry, hungry.clans[clan.id], civ.id, 10) > g10);
  // buying a fed clan takes many turns of food
  assert.ok(CIV.joinAt / g10 > 10);
});

test('trade leaning follows the civ share of trade, decays, and notes only on real change', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const m = s.markets[civ.marketId];
  const clan = unaligned(s, civ);
  Object.assign(clan, { market: m.id, basket: { grain: 6 }, pop: 6 });
  s.notes[clan.id] = [];
  civ.store = { fish: 40 };
  step('civ post')(s);
  trade(s);
  step('civ collect')(s);
  const l = clan.leaning[civ.id];
  assert.ok(l > 0 && l <= CIV.tradeLean + 1e-9);
  // no trade with the civ -> no leaning
  const other = structuredClone(s);
  const c2 = other.clans[clan.id];
  c2.leaning = {};
  Object.assign(c2, { market: m.id, basket: { grain: 6 } });
  other.civs[civ.id].store = {};
  step('civ post')(other); trade(other); step('civ collect')(other);
  assert.equal(c2.leaning[civ.id], undefined);
  // decay
  clan.leaning[civ.id] = 0.5;
  step('civ join')(s);
  assert.ok(Math.abs(clan.leaning[civ.id] - 0.5 * (1 - CIV.leanDecay)) < 1e-9);
  assert.ok(s.notes[clan.id].length <= 2, 'no note every turn');
});

test('a clan joins past the threshold, with an alert for the human', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const clan = unaligned(s, civ);
  clan.leaning[civ.id] = CIV.joinAt / (1 - CIV.leanDecay) + 0.01;
  s.events = [];
  step('civ join')(s);
  assert.equal(clan.owner, civ.id);
  assert.ok(s.notes[clan.id].some((n) => /joined/.test(n.text)));
  assert.ok(s.events.some((e) => e.includes('! ') && /joins/.test(e)));
});

test('daughter site: validated, used by the hook, and lapses', () => {
  const s = mainGame();
  const [civ, rival] = Object.values(s.civs);
  const clan = civClans(s, civ.id)[0];
  const city = civCities(s, civ.id)[0];
  const send = (site) => dispatch(s, { type: 'setDaughterSite', civ: civ.id, site });
  assert.equal(send('999,999').error, 'no such tile');
  const water = Object.values(s.map.tiles).find((t) => !passable(s, t) && distance(t, clan) <= 3);
  if (water) assert.equal(send(key(water)).error, 'impassable');
  const far = Object.values(s.map.tiles).find((t) => passable(s, t) && civClans(s, civ.id).every((c) => distance(c, t) > CLAN.scoutRange) && !civCities(s, rival.id).some((c) => distance(c, t) <= CITY.territoryRadius));
  assert.match(send(key(far)).error, /none of your clans/);
  const rc = civCities(s, rival.id)[0];
  assert.match(send(key(rc)).error, /holds that ground/);
  const ok = Object.values(s.map.tiles).find((t) => passable(s, t) && distance(t, clan) === 2 && !mod.siteError(s, civ.id, t));
  const r = send(key(ok));
  assert.ok(!r.error, r.error);
  const options = [{ k: `${city.q + 1},${city.r}`, score: 5 }, { k: `${city.q + 5},${city.r}`, score: 1 }, { k: key(ok), score: 0 }];
  assert.equal(mod.hooks.daughterSite(r.state, clan, { options }), key(ok));
  assert.equal(mod.hooks.daughterSite(s, clan, { options: options.slice(0, 2) }), options[1].k, 'auto-pick avoids near sites');
  assert.equal(mod.hooks.daughterSite(s, { owner: null }, { options }), undefined);
  assert.equal(r.state.mods.civ.siteChoice[civ.id], undefined, 'used up');
  const t = send(key(ok)).state;
  t.turn += CIV.siteTurns;
  t.events = [];
  step('civ upkeep')(t);
  assert.equal(t.mods.civ.siteChoice[civ.id], undefined);
  assert.ok(t.events.some((e) => /lapses/.test(e)));
});

test('cities: founding needs tamed grain; the player can choose the next city site', () => {
  const s = mainGame();
  const civ = Object.values(s.civs)[0];
  const city = civCities(s, civ.id)[0];
  const clan = civClans(s, civ.id)[0];
  const far = Object.values(s.map.tiles).find((t) => passable(s, t) && distance(t, city) >= 4 && distance(t, city) <= 5
    && Object.values(s.cities).every((c) => distance(c, t) >= CITY.minSpacing) && !mod.siteError(s, civ.id, t));
  for (const t of Object.values(s.map.tiles)) if (distance(t, far) <= 1) t.tame = 0;
  for (const c of civClans(s, civ.id)) Object.assign(c, { since: 0 });
  Object.assign(clan, { q: far.q, r: far.r, route: [], since: 99, pop: 10, fed: false });
  step('civ upkeep')(s);
  assert.equal(civCities(s, civ.id).length, 1, 'no tamed grain, no city');
  assert.match(dispatch(s, { type: 'setCitySite', civ: civ.id, site: key(far) }).error, /no tamed grain/);
  far.tame = 1;
  // chosen site: founded there once a clan lives next to it
  const r = dispatch(s, { type: 'setCitySite', civ: civ.id, site: key(far) });
  assert.ok(!r.error, r.error);
  const t = r.state;
  t.events = [];
  step('civ upkeep')(t);
  const made = civCities(t, civ.id).find((c) => !c.founding);
  assert.ok(made && made.q === far.q && made.r === far.r);
  assert.ok(t.events.some((e) => /^Turn \d+: ! .*founds/.test(e)));
  assert.equal(t.mods.civ.citySite[civ.id], undefined);
  // auto founding where tamed
  step('civ upkeep')(s);
  assert.equal(civCities(s, civ.id).length, 2);
});

test('years: drought and bumper scale yield, deterministically, and expire', () => {
  const s = mainGame();
  const t = Object.values(s.map.tiles)[0];
  s.mods.civ.years[mod.regionOf(t)] = { factor: 0.5, left: 1 };
  assert.equal(mod.hooks.yieldFactor(s, t), 0.5);
  const a = structuredClone(s), b = structuredClone(s);
  turns(a, 30); turns(b, 30);
  assert.deepEqual(a.mods.civ.years, b.mods.civ.years);
  s.mods.civ.years = {};
  const w = step('civ weather');
  let seen = false;
  for (let i = 0; i < 300 && !seen; i++) { w(s); seen = Object.keys(s.mods.civ.years).length > 0; }
  assert.ok(seen);
});

test('discoveries: practice teaches; knowledge spreads through shared trade, not distance', () => {
  const s = mainGame();
  const [a, b] = Object.values(s.civs);
  s.mods.civ.disc[a.id].practice.pottery = CIV.practice.pottery;
  const d = step('civ discoveries');
  d(s);
  assert.ok(s.mods.civ.disc[a.id].known.pottery);
  // no shared market: nothing spreads even if markets sit side by side
  for (const c of Object.values(s.clans)) c.market = null;
  const far = structuredClone(s);
  far.markets[b.marketId].q = 900; // b's market out of a's trade range, so they share none
  for (let i = 0; i < 60; i++) d(far);
  assert.ok(!far.mods.civ.disc[b.id].known.pottery);
  // a clan of b trading at a's market puts them in contact
  const bc = civClans(s, b.id)[0];
  bc.market = a.marketId;
  assert.ok(mod.contacts(s)[b.id].has(a.id));
  for (let i = 0; i < 60; i++) d(s);
  assert.ok(s.mods.civ.disc[b.id].known.pottery, 'spread by trade');
  s.mods.civ.disc[a.id].practice.herding = 99;
  d(s);
  assert.ok(s.mods.civ.disc[a.id].known.herding);
});

test('victory: conquest, prosperity, winTurn, lost civs and alerts', () => {
  const s = mainGame();
  const [a, b] = Object.values(s.civs);
  const vic = step('civ victory');
  vic(s);
  assert.equal(s.winner, undefined);
  const cb = civCities(s, b.id)[0];
  s.events = [];
  transferCity(s, cb.id, a.id);
  assert.equal(mod.conquerer(s), a.id);
  vic(s);
  assert.equal(s.winner, a.id);
  assert.equal(s.mods.civ.winTurn, s.turn);
  assert.equal(s.mods.civ.lost[b.id], s.turn);
  assert.ok(s.events.some((e) => /! .*takes/.test(e)));
  assert.ok(s.events.some((e) => /! .*wins by conquest: you win/.test(e)));
  const p = mainGame();
  p.turn = p.mods.civ.startTurn + CIV.turnLimit;
  vic(p);
  assert.ok(p.winner != null);
  assert.equal(p.mods.civ.winKind, 'prosperity');
});

test('prosperity rewards growth since founding, not the founding head start', () => {
  const s = mainGame();
  const [a, b] = Object.values(s.civs);
  s.mods.civ.startPop = { [a.id]: 100, [b.id]: 10 };
  const set = (civ, n) => { const cs = civClans(s, civ.id); for (const c of cs) c.pop = n / cs.length; };
  set(a, 90); set(b, 15);
  assert.equal(mod.prosperityWinner(s), b.id, 'the smaller civ that grew wins');
  set(b, 9);
  assert.equal(mod.prosperityWinner(s), a.id);
});

test('alerts only when the human civ is concerned', () => {
  const s = mainGame(false);
  const civ = Object.values(s.civs)[0];
  const clan = unaligned(s, civ);
  clan.leaning[civ.id] = 5;
  s.events = [];
  step('civ join')(s);
  assert.ok(s.events.some((e) => /joins/.test(e)) && !s.events.some((e) => e.includes('! ')));
});

test('AI civs play: projects and gifts go through the same rules; a long run stays valid', () => {
  const s = mainGame(false);
  const [a] = Object.values(s.civs);
  a.store = { wood: 10, clay: 10, stone: 10, grain: 60 };
  const acts = mod.ai(s, a.id);
  assert.ok(acts.some((x) => x.type === 'setProject'));
  const gift = acts.find((x) => x.type === 'giftFood');
  assert.ok(gift && !mod.giftError(s, a.id, gift.clan, gift.amount));
  turns(s, 40);
  assert.ok(Object.keys(s.clans).length > 0);
  for (const g of Object.values(s.mods.civ.gifted)) for (const x of Object.values(g)) assert.ok(x.amount <= CIV.giftMax);
});

test('ui: panel shows store, labour, gifts with gain, and site buttons; drawLayer draws', async () => {
  const ui = await import('../src/modules/civ/ui.js');
  const s = mainGame();
  const civ = Object.values(s.civs).find((c) => c.isHuman);
  const city = civCities(s, civ.id)[0];
  turns(s, 2);
  const html = ui.panel(s, { q: city.q, r: city.r });
  for (const want of ['Store', 'Labour', 'setPosting', 'Discoveries', 'Prosperity', 'join at']) assert.ok(html.includes(want), want);
  const clan = Object.values(s.clans).find((c) => c.owner == null);
  civ.seen = undefined;
  const h2 = ui.panel(s, { q: clan.q, r: clan.r });
  assert.match(h2, /Gift 5 food \(\+\d/);
  assert.match(h2, /Found next city here/);
  assert.equal(ui.panel(mainGame(false), null), '', 'no human, no panel');
  const calls = [];
  const ctx = new Proxy({}, { get: (t, k) => (k in t ? t[k] : () => calls.push(k)), set: (t, k, v) => { t[k] = v; return true; } });
  ui.drawLayer(ctx, s, { size: 10, viewer: civ.id, at: (h) => ({ x: h.q * 10, y: h.r * 10 }), hexPath: () => {} });
  assert.ok(calls.includes('fillText'));
});
