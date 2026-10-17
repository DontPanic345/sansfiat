// The civ module: cities, civ clans, materials, projects, expansion, years, leanings,
// discoveries, victory. State: s.mods.civ = {regions, years, siteChoice, disc, startTurn, winKind};
// city.buildings = {granary, storehouse, wall}; city.project = {kind, labour} | null; city.founding.
// s.winner = winning civ id once decided (s.mods.civ.winKind: 'conquest' | 'prosperity').
import { CITY, CLAN, FOODS, MATERIALS } from '../../data.js';
import { civCities, civClans, event, newId, note, putGoods, takeGoods, tileAt, total } from '../../core.js';
import { distance, key } from '../../hex.js';
import { rand } from '../../rng.js';
import { marketReach } from '../../clans.js';
import { CIV } from './data.js';

export const name = 'civ';

const regionOf = (t) => `${Math.floor(t.q / CIV.region)},${Math.floor(t.r / CIV.region)}`;
const parse = (k) => { const [q, r] = k.split(',').map(Number); return { q, r }; };
const popOf = (s, civId) => civClans(s, civId).reduce((a, c) => a + c.pop, 0);

export function init(s) {
  s.mods.civ = {
    regions: [...new Set(Object.values(s.map.tiles).map(regionOf))].sort(),
    years: {}, siteChoice: {}, disc: {}, startTurn: null, winKind: null,
  };
}

function newCity(s, civ, h, founding = false) {
  const id = newId(s);
  s.cities[id] = {
    id, owner: civ.id, name: `${civ.name} ${founding ? 'capital' : 'town'} ${civ.cityIds.length + 1}`,
    q: h.q, r: h.r, founded: s.turn, founding, buildings: {}, project: null,
  };
  civ.cityIds.push(id);
  return s.cities[id];
}

export function onStart(s) {
  s.mods.civ.startTurn = s.turn;
  const civs = Object.values(s.civs);
  for (const civ of civs) {
    const m = s.markets[civ.marketId];
    newCity(s, civ, m, true);
    s.mods.civ.disc[civ.id] = { practice: { pottery: 0, herding: 0 }, known: {} };
    note(s, civ.id, `capital raised at the market ${key(m)}`);
  }
  const reach = Object.fromEntries(civs.map((c) => [c.id, marketReach(s, s.markets[c.marketId])]));
  for (const clan of Object.values(s.clans)) {
    const best = civs.map((c) => ({ c, d: reach[c.id].get(key(clan)) })).filter((x) => x.d != null)
      .sort((a, b) => a.d - b.d)[0];
    if (best) { clan.owner = best.c.id; note(s, clan.id, `part of ${best.c.name}: lives within reach of its market`); }
  }
  for (const civ of civs) {
    if (civClans(s, civ.id).length) continue;
    const m = s.markets[civ.marketId];
    const near = Object.values(s.clans).filter((c) => c.owner == null).sort((a, b) => distance(a, m) - distance(b, m))[0];
    if (near) near.owner = civ.id;
  }
}

// ---- projects and goods ---------------------------------------------------------------------

const affordable = (s, civId, kind) =>
  Object.entries(CIV.projects[kind].goods).every(([g, v]) => (s.civs[civId].store[g] ?? 0) >= v);

export const hasWall = (city) => !!city.buildings?.wall;

export const actions = {
  setProject: {
    validate: (s, a) => {
      const c = s.cities[a.city];
      if (!c || c.owner !== a.civ) return 'not your city';
      if (!CIV.projects[a.project]) return `no project ${a.project}`;
      if (c.buildings[a.project]) return 'already built';
      if (c.project) return 'city already has a project';
      return affordable(s, a.civ, a.project) ? null : 'not enough goods in the store';
    },
    apply: (s, a) => {
      const c = s.cities[a.city];
      for (const [g, v] of Object.entries(CIV.projects[a.project].goods)) takeGoods(s, a.civ, g, v);
      c.project = { kind: a.project, labour: 0 };
      note(s, c.id, `began the ${a.project}`);
    },
    candidates: (s, civId) => civCities(s, civId).filter((c) => !c.project).flatMap((c) =>
      Object.keys(CIV.projects).filter((p) => !c.buildings[p] && affordable(s, civId, p))
        .map((project) => ({ type: 'setProject', civ: civId, city: c.id, project }))),
  },
  giftFood: {
    validate: (s, a) => {
      const c = s.clans[a.clan];
      if (!c) return 'no such clan';
      if (c.owner != null) return 'that clan already belongs to a civ';
      const have = FOODS.reduce((n, f) => n + (s.civs[a.civ].store[f] ?? 0), 0);
      return a.amount > 0 && have >= a.amount ? null : 'not enough food in the store';
    },
    apply: (s, a) => {
      const c = s.clans[a.clan];
      let left = a.amount;
      for (const f of FOODS) {
        const t = takeGoods(s, a.civ, f, left);
        if (t) { c.store[f] = (c.store[f] ?? 0) + t; left -= t; }
      }
      c.leaning[a.civ] = (c.leaning[a.civ] ?? 0) + a.amount * CIV.giftLean;
      note(s, c.id, `${s.civs[a.civ].name} gave ${a.amount} food: leaning ${c.leaning[a.civ].toFixed(2)}`);
    },
    candidates: (s, civId) => {
      const m = s.markets[s.civs[civId].marketId];
      const c = Object.values(s.clans).filter((x) => x.owner == null).sort((a, b) => distance(a, m) - distance(b, m))[0];
      return c ? [{ type: 'giftFood', civ: civId, clan: c.id, amount: 5 }] : [];
    },
  },
  // Choose where the next daughter of a civ clan goes (a tile key "q,r"); used if it is one of her options.
  setDaughterSite: {
    validate: (s, a) => (tileAt(s, parse(String(a.site))) ? null : 'no such tile'),
    apply: (s, a) => { s.mods.civ.siteChoice[a.civ] = a.site; },
    candidates: () => [],
  },
};

// ---- hooks ----------------------------------------------------------------------------------

export const hooks = {
  daughterSite(s, clan, { options }) {
    if (clan.owner == null || !s.mods.civ) return undefined;
    const want = s.mods.civ.siteChoice[clan.owner];
    if (want && options.some((o) => o.k === want)) { delete s.mods.civ.siteChoice[clan.owner]; return want; }
    const cities = civCities(s, clan.owner);
    const ok = options.filter((o) => cities.every((c) => distance(c, parse(o.k)) >= CITY.minSpacing));
    const pool = ok.length ? ok : options;
    if (!pool.length) return undefined;
    return pool.reduce((b, o) => (o.score > b.score ? o : b)).k;
  },
  yieldFactor: (s, t) => s.mods?.civ?.years?.[regionOf(t)]?.factor ?? 1,
};

// ---- turn steps -----------------------------------------------------------------------------

function weather(s) {
  const m = s.mods.civ;
  for (const r of m.regions) {
    const y = m.years[r];
    if (y) { if (--y.left <= 0) delete m.years[r]; continue; }
    if (rand(s) < CIV.yearChance) {
      const bad = rand(s) < 0.5;
      const [lo, hi] = CIV.yearTurns;
      m.years[r] = { factor: bad ? CIV.drought : CIV.bumper, left: lo + Math.floor(rand(s) * (hi - lo + 1)) };
      event(s, `${bad ? 'drought' : 'a bumper year'} in region ${r}`);
    }
  }
}

const capOf = (s, civ) => {
  const cs = civCities(s, civ.id);
  return CIV.baseCap * cs.length + CIV.storehouseCap * cs.filter((c) => c.buildings.storehouse).length;
};

const MAT_TILE = { wood: (t) => t.terrain === 'forest', stone: (t) => t.terrain === 'hills', clay: (t) => !!t.river };

function work(s) {
  const m = s.mods.civ;
  for (const civ of Object.values(s.civs)) {
    const cities = civCities(s, civ.id);
    if (!cities.length) continue;
    const d = m.disc[civ.id];
    const labour = popOf(s, civ.id) * CIV.labourShare / cities.length;
    for (const city of cities) {
      let l = labour;
      if (city.project) {
        const p = CIV.projects[city.project.kind];
        const put = Math.min(l, p.labour - city.project.labour);
        city.project.labour += put;
        l -= put;
        if (city.project.labour >= p.labour - 1e-9) {
          city.buildings[city.project.kind] = true;
          event(s, `${civ.name} finishes a ${city.project.kind}`);
          note(s, city.id, `finished the ${city.project.kind}`);
          city.project = null;
        }
      }
      if (d.known.pottery && (civ.store.clay ?? 0) >= 1 && l > 0 && (civ.store.pottery ?? 0) < CIV.matTarget) {
        const n = Math.min(CIV.craft, l, civ.store.clay);
        takeGoods(s, civ.id, 'clay', n);
        putGoods(s, civ.id, { pottery: n });
        l -= n;
      }
      const tiles = Object.values(s.map.tiles).filter((t) => distance(city, t) <= CIV.workRadius);
      // light hunting of game tiles: the practice that leads to herding
      const game = tiles.filter((t) => (t.stock.game ?? 0) > 1);
      if (game.length && l > 0) {
        const take = Math.min(CIV.hunt * (d.known.herding ? CIV.herdBonus : 1), l);
        let got = 0;
        for (const t of game) {
          const x = Math.min(take / game.length, t.stock.game * 0.1 + 0.2);
          t.stock.game -= x * (d.known.herding ? 0.5 : 1);
          got += x;
        }
        putGoods(s, civ.id, { game: got });
        l -= take * 0.5;
        d.practice.herding += 1;
      }
      const want = MATERIALS.filter((g) => (civ.store[g] ?? 0) < CIV.matTarget && tiles.some(MAT_TILE[g]));
      for (const g of want) {
        const mine = tiles.filter(MAT_TILE[g]);
        const f = mine.reduce((a, t) => a + hooks.yieldFactor(s, t), 0) / mine.length;
        const n = Math.max(0, Math.min(l / want.length, mine.length * CIV.tileMax)) * f;
        if (n <= 0) continue;
        putGoods(s, civ.id, { [g]: n });
        if (g === 'clay') d.practice.pottery += 1;
      }
    }
  }
}

function post(s) {
  for (const civ of Object.values(s.civs)) {
    const m = s.markets[civ.marketId];
    if (!m) continue;
    const out = {};
    for (const [g, v] of Object.entries(civ.store)) {
      const keep = FOODS.includes(g) ? CIV.reserve.food : MATERIALS.includes(g) ? CIV.reserve.material : CIV.reserve.craft;
      if (v > keep) out[g] = v - keep;
    }
    if (total(out) <= 0) continue;
    for (const [g, v] of Object.entries(out)) takeGoods(s, civ.id, g, v);
    m.posts[civ.id] = out;
  }
}

function collect(s) {
  for (const civ of Object.values(s.civs)) {
    const m = s.markets[civ.marketId];
    if (!m) continue;
    const got = m.received?.[civ.id];
    if (got) putGoods(s, civ.id, got);
    // trading at a civ's market builds unaligned clans' leaning toward it
    for (const c of Object.values(s.clans))
      if (c.owner == null && c.market === m.id) {
        c.leaning[civ.id] = (c.leaning[civ.id] ?? 0) + CIV.tradeLean;
        note(s, c.id, `trades at ${civ.name}'s market: leaning ${c.leaning[civ.id].toFixed(2)}`);
      }
  }
}

function upkeep(s) {
  for (const civ of Object.values(s.civs)) {
    const cities = civCities(s, civ.id);
    const spoil = CLAN.spoilage * (cities.some((c) => c.buildings.granary) ? CIV.granarySpoil : 1);
    const cap = capOf(s, civ);
    for (const g of Object.keys(civ.store)) {
      if (FOODS.includes(g)) civ.store[g] *= 1 - spoil;
      if (civ.store[g] > cap) civ.store[g] = cap;
      if (civ.store[g] < 0.01) delete civ.store[g];
    }
    // a settled civ clan far from every city founds a new one
    if (cities.length >= CIV.maxCities) continue;
    for (const c of civClans(s, civ.id)) {
      if (c.route.length || c.since < CIV.newCitySince || c.pop < CIV.newCityPop) continue;
      if (Object.values(s.cities).some((x) => distance(x, c) < CITY.minSpacing)) continue;
      const city = newCity(s, civ, c);
      note(s, city.id, `founded by ${c.name} at ${key(c)}`);
      event(s, `${civ.name} founds ${city.name}`);
      break;
    }
  }
}

function join(s) {
  for (const c of Object.values(s.clans)) {
    if (c.owner != null) continue;
    const best = Object.entries(c.leaning).sort((a, b) => b[1] - a[1])[0];
    if (best && best[1] >= CIV.joinAt && s.civs[best[0]]) {
      c.owner = Number(best[0]);
      note(s, c.id, `joined ${s.civs[best[0]].name}: leaning reached ${best[1].toFixed(2)} through trade and gifts`);
      event(s, `${c.name} joins ${s.civs[best[0]].name}`);
    }
  }
}

function discover(s) {
  const m = s.mods.civ;
  const civs = Object.values(s.civs);
  for (const civ of civs) {
    const d = m.disc[civ.id];
    for (const g of Object.keys(CIV.practice)) {
      if (d.known[g]) continue;
      for (const o of civs)
        if (o.id !== civ.id && m.disc[o.id].known[g]
          && distance(s.markets[o.marketId], s.markets[civ.marketId]) <= CIV.spreadRange) d.practice[g] += CIV.spread;
      if (d.practice[g] >= CIV.practice[g]) {
        d.known[g] = true;
        event(s, `${civ.name} learns ${g}`);
        note(s, civ.id, `learned ${g} by practice`);
      }
    }
  }
}

// Conquest: one civ holds every founding city. The army module captures cities with transferCity;
// this just reads city ownership.
export function conquerer(s) {
  const owners = new Set(Object.values(s.cities).filter((c) => c.founding).map((c) => c.owner));
  return owners.size === 1 && Object.keys(s.civs).length > 1 ? [...owners][0] : null;
}

export function prosperityWinner(s) {
  return Object.values(s.civs).reduce((b, c) => (popOf(s, c.id) > popOf(s, b.id) ? c : b)).id;
}

export function victory(s) {
  if (s.winner != null) return;
  const m = s.mods.civ;
  const c = conquerer(s);
  if (c != null) { s.winner = c; m.winKind = 'conquest'; }
  else if (s.turn - m.startTurn >= CIV.turnLimit) { s.winner = prosperityWinner(s); m.winKind = 'prosperity'; }
  else return;
  event(s, `${s.civs[s.winner].name} wins by ${m.winKind}`);
}

export const turnSteps = [
  { phase: 'regrow', name: 'civ weather', order: 'before', run: weather },
  { phase: 'forage', name: 'civ labour', run: work },
  { phase: 'trade', name: 'civ post', order: 'before', run: post },
  { phase: 'trade', name: 'civ collect', run: collect },
  { phase: 'settle', name: 'civ upkeep', run: upkeep },
  { phase: 'markets', name: 'civ join', run: join },
  { phase: 'discoveries', name: 'civ discoveries', run: discover },
  { phase: 'discoveries', name: 'civ victory', run: victory },
];

export function ai(s, civId) {
  const civ = s.civs[civId];
  const out = [];
  const proj = actions.setProject.candidates(s, civId)[0];
  if (proj) out.push(proj);
  const food = FOODS.reduce((a, f) => a + (civ.store[f] ?? 0), 0);
  if (food > 2 * CIV.reserve.food) out.push(...actions.giftFood.candidates(s, civId));
  return out;
}
