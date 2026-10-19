// The civ module: cities, civ clans, drafted labour, materials, projects, posting at market,
// expansion, years, leanings, discoveries, victory.
// State: s.mods.civ = {
//   regions, years: {region: {factor, left}}, disc: {civId: {practice, known}}, startTurn, winKind,
//   siteChoice: {civId: {k, turn}}  next daughter site;  citySite: {civId: {k, turn}}  next city site,
//   posting: {civId: {good: keep}}  player reserves (absent: default policy),
//   trade: {civId: {turn, posted, received}}  last turn at market,
//   labour: {civId: {wanted, drafted, cities: {cityId: workers}}},  gifted: {clanId: {civId: {turn, amount}}},
//   lost: {civId: turn}, startPop: {civId: people at founding}, winTurn, cityOwners: {cityId: civId}, overflow: {civId: goods lost to the cap last turn} }
// clan.drafted = people working for the civ this turn (foraging uses pop - drafted).
// city.buildings = {granary, storehouse, wall}; city.project = {kind, labour} | null; city.founding.
// s.winner = winning civ id once decided (s.mods.civ.winKind: 'conquest' | 'prosperity').
import { CITY, CLAN, FOODS, GOODS, MATERIALS } from '../../data.js';
import {
  addGoods, civCities, civClans, event, newId, note, passable, putGoods, takeGoods, territoryOf, tileAt, total,
} from '../../core.js';
import { distance, key, within } from '../../hex.js';
import { rand } from '../../rng.js';
import { marketReach } from '../../clans.js';
import { CIV } from './data.js';

export const name = 'civ';

export const regionOf = (t) => `${Math.floor(t.q / CIV.region)},${Math.floor(t.r / CIV.region)}`;
const parse = (k) => { const [q, r] = String(k).split(',').map(Number); return { q, r }; };
export const popOf = (s, civId) => civClans(s, civId).reduce((a, c) => a + c.pop, 0);
const fmt = (x) => (x >= 10 ? x.toFixed(0) : x.toFixed(1));
const foodIn = (store) => FOODS.reduce((a, f) => a + (store[f] ?? 0), 0);
const kindOf = (g) => (FOODS.includes(g) ? 'food' : MATERIALS.includes(g) ? 'material' : 'craft');

// An event line; with a leading '!' (an alert, contract 9) when it concerns the human civ.
function tell(s, civIds, text) {
  const ids = [].concat(civIds);
  event(s, (ids.some((id) => s.civs[id]?.isHuman) ? '! ' : '') + text);
}

export function init(s) {
  s.mods.civ = {
    regions: [...new Set(Object.values(s.map.tiles).map(regionOf))].sort(),
    years: {}, siteChoice: {}, citySite: {}, disc: {}, startTurn: null, winKind: null,
    posting: {}, trade: {}, labour: {}, gifted: {}, lost: {}, cityOwners: {}, overflow: {}, startPop: {},
  };
}

function newCity(s, civ, h, founding = false) {
  const id = newId(s);
  s.cities[id] = {
    id, owner: civ.id, name: `${civ.name} ${founding ? 'capital' : 'town'} ${civ.cityIds.length + 1}`,
    q: h.q, r: h.r, founded: s.turn, founding, buildings: {}, project: null,
  };
  civ.cityIds.push(id);
  s.mods.civ.cityOwners[id] = civ.id;
  return s.cities[id];
}

export function onStart(s) {
  const m = s.mods.civ;
  m.startTurn = s.turn;
  const civs = Object.values(s.civs);
  for (const civ of civs) {
    const mk = s.markets[civ.marketId];
    newCity(s, civ, mk, true);
    m.disc[civ.id] = { practice: { pottery: 0, herding: 0 }, known: {} };
    note(s, civ.id, `capital raised at the market ${key(mk)}`);
  }
  const reach = Object.fromEntries(civs.map((c) => [c.id, marketReach(s, s.markets[c.marketId])]));
  for (const clan of Object.values(s.clans)) {
    const best = civs.map((c) => ({ c, d: reach[c.id].get(key(clan)) })).filter((x) => x.d != null)
      .sort((a, b) => a.d - b.d)[0];
    if (best) { clan.owner = best.c.id; note(s, clan.id, `part of ${best.c.name}: lives within reach of its market`); }
  }
  for (const civ of civs) {
    if (civClans(s, civ.id).length) continue;
    const mk = s.markets[civ.marketId];
    const near = Object.values(s.clans).filter((c) => c.owner == null).sort((a, b) => distance(a, mk) - distance(b, mk))[0];
    if (near) near.owner = civ.id;
  }
  m.startPop = Object.fromEntries(civs.map((c) => [c.id, popOf(s, c.id)]));
}

const MAT_TILE = { wood: (t) => t.terrain === 'forest', stone: (t) => t.terrain === 'hills', clay: (t) => !!t.river };

// ---- store ----------------------------------------------------------------------------------

// The most the whole store holds: baseCap a city plus storehouseCap a storehouse.
export function capOf(s, civId) {
  const cs = civCities(s, civId);
  return CIV.baseCap * cs.length + CIV.storehouseCap * cs.filter((c) => c.buildings?.storehouse).length;
}

// How much of a good the civ keeps back from market: the player's reserve, else the default policy.
// Default: a share of the cap for food and crafts; for a material, what the dearest project
// still to build in any city needs of it (none once everything is built).
export function keepOf(s, civId, good) {
  const own = s.mods.civ.posting[civId]?.[good];
  if (own != null) return own;
  if (kindOf(good) !== 'material') return CIV.keep[kindOf(good)] * capOf(s, civId);
  return civCities(s, civId).reduce((a, c) => Math.max(a, ...Object.entries(CIV.projects)
    .filter(([p]) => !c.buildings?.[p]).map(([, p]) => p.goods[good] ?? 0)), CIV.keepMat[good] ?? 0);
}

// The markets a civ posts at: its own, plus one other within tradeRange of it: the one whose
// trading clans hold the most land yielding the materials the civ is short of, else the nearest.
export function tradeMarkets(s, civId) {
  const civ = s.civs[civId];
  const own = s.markets[civ.marketId];
  if (!own) return [];
  const short = MATERIALS.filter((g) => (civ.store[g] ?? 0) < Math.max(1, keepOf(s, civId, g)));
  const yields = {};
  for (const c of Object.values(s.clans))
    if (c.market != null && !c.starving)
      for (const k of c.land) if (short.some((g) => MAT_TILE[g](s.map.tiles[k]))) yields[c.market] = (yields[c.market] ?? 0) + 1;
  const other = Object.values(s.markets).filter((m) => m.id !== own.id && distance(m, own) <= CIV.tradeRange)
    .sort((a, b) => (yields[b.id] ?? 0) - (yields[a.id] ?? 0) || distance(a, own) - distance(b, own) || a.id - b.id)[0];
  return other ? [own, other] : [own];
}

const hasGranary = (s, civId) => civCities(s, civId).some((c) => c.buildings?.granary);
const knows = (s, civId, g) => !!s.mods.civ.disc[civId]?.known[g];

// Spoilage multiplier for the civ store, and for one member clan (granary within reach, pottery).
export const storeSpoil = (s, civId) =>
  (hasGranary(s, civId) ? CIV.granarySpoil : 1) * (knows(s, civId, 'pottery') ? CIV.potterySpoil : 1);
export function clanSpoil(s, clan) {
  if (clan.owner == null) return 1;
  const near = civCities(s, clan.owner).some((c) => c.buildings?.granary && distance(c, clan) <= CIV.granaryReach);
  return (near ? CIV.granarySpoil : 1) * (knows(s, clan.owner, 'pottery') ? CIV.potterySpoil : 1);
}

// ---- city work ------------------------------------------------------------------------------


// A city works the tiles within CITY.workRadius, plus the land its civ's clans hold within its
// territory: the clans live there and know where the wood, stone and clay are.
export function workTiles(s, city) {
  const out = new Map();
  for (const h of within(city, CITY.workRadius)) { const t = tileAt(s, h); if (t) out.set(key(t), t); }
  for (const c of civClans(s, city.owner))
    if (distance(c, city) <= CITY.territoryRadius) for (const k of c.land) out.set(k, s.map.tiles[k]);
  return [...out.values()];
}

// Workers a city wants this turn: {project, craft, hunt, wood?, stone?, clay?}.
export function cityNeeds(s, city) {
  const civ = s.civs[city.owner];
  const tiles = workTiles(s, city);
  const W = CIV.workers;
  const out = {
    project: city.project ? W.project : 0,
    craft: knows(s, civ.id, 'pottery') && (civ.store.clay ?? 0) >= 1 && (civ.store.pottery ?? 0) < CIV.matTarget ? W.craft : 0,
    hunt: tiles.some((t) => (t.stock.game ?? 0) > 0.5) ? W.hunt : 0,
    farm: foodIn(civ.store) < CIV.foodTarget * capOf(s, civ.id) ? W.farm : 0,
  };
  for (const g of MATERIALS)
    if ((civ.store[g] ?? 0) < CIV.matTarget && tiles.some(MAT_TILE[g])) out[g] = W.material;
  return out;
}

// Drafts workers from the civ's clans in proportion to their people, as many as its cities
// need, at most draftMax of each clan. Starving clans are spared. Runs before foraging.
function draft(s) {
  const m = s.mods.civ;
  for (const c of Object.values(s.clans)) if (c.owner == null && c.drafted) c.drafted = 0;
  for (const civ of Object.values(s.civs)) {
    const cities = civCities(s, civ.id);
    const needs = Object.fromEntries(cities.map((c) => [c.id, cityNeeds(s, c)]));
    const wanted = Object.values(needs).reduce((a, n) => a + total(n), 0);
    const clans = civClans(s, civ.id);
    const can = clans.reduce((a, c) => a + (c.starving ? 0 : c.pop * CIV.draftMax), 0);
    const k = can > 0 ? Math.min(1, wanted / can) : 0;
    for (const c of clans) c.drafted = c.starving ? 0 : c.pop * CIV.draftMax * k;
    m.labour[civ.id] = { wanted, drafted: can * k, needs };
  }
}

function work(s) {
  const m = s.mods.civ;
  for (const civ of Object.values(s.civs)) {
    const lab = m.labour[civ.id];
    if (!lab || lab.wanted <= 0) continue;
    const d = m.disc[civ.id];
    const r = (lab.drafted / lab.wanted) * CIV.labourPerWorker; // labour per wanted worker
    let hunted = false, dug = false;
    for (const city of civCities(s, civ.id)) {
      const need = lab.needs[city.id];
      if (!need) continue;
      if (city.project && need.project) {
        const p = CIV.projects[city.project.kind];
        city.project.labour = Math.min(p.labour, city.project.labour + need.project * r);
        if (city.project.labour >= p.labour - 1e-9) {
          city.buildings[city.project.kind] = true;
          tell(s, civ.id, `${civ.name} finishes a ${city.project.kind} at ${city.name}`);
          note(s, city.id, `finished the ${city.project.kind}`);
          city.project = null;
        }
      }
      if (need.craft) {
        const n = Math.min(CIV.craft * need.craft * r, civ.store.clay ?? 0);
        takeGoods(s, civ.id, 'clay', n);
        putGoods(s, civ.id, { pottery: n });
      }
      const tiles = workTiles(s, city);
      if (need.hunt) {
        // light hunting of game tiles: the practice that leads to herding
        const game = tiles.filter((t) => (t.stock.game ?? 0) > 0.5);
        const take = CIV.huntPerWorker * need.hunt * r * (d.known.herding ? CIV.herdBonus : 1);
        let got = 0;
        for (const t of game) {
          const x = Math.min(take / game.length, t.stock.game * 0.1 + 0.2);
          t.stock.game -= x * (d.known.herding ? 0.5 : 1);
          got += x;
        }
        putGoods(s, civ.id, { game: got });
        if (got > 0) hunted = true;
      }
      if (need.farm) {
        // farm workers harvest the city's own tiles (its tamed grain first of all) for the store
        const fields = within(city, CITY.workRadius).map((h) => tileAt(s, h)).filter(Boolean);
        const avail = fields.reduce((a, t) => a + total(t.stock) * CLAN.takeRate, 0);
        const want = Math.min(CIV.farmPerWorker * need.farm * r, avail);
        const got = {};
        if (want > 0)
          for (const t of fields)
            for (const [f, v] of Object.entries(t.stock)) {
              const x = v * CLAN.takeRate * (want / avail);
              t.stock[f] -= x;
              got[f] = (got[f] ?? 0) + x;
            }
        putGoods(s, civ.id, got);
      }
      for (const g of MATERIALS) {
        if (!need[g]) continue;
        const mine = tiles.filter(MAT_TILE[g]);
        const f = mine.reduce((a, t) => a + hooks.yieldFactor(s, t), 0) / mine.length;
        const n = Math.min(need[g] * r * CIV.matPerWorker, mine.length * CIV.tileMax) * f;
        if (n <= 0) continue;
        putGoods(s, civ.id, { [g]: n });
        if (g === 'clay') dug = true;
      }
    }
    // Practice: digging clay or hunting counts in full; working clay got by trade, or member
    // clans living off game, counts at heldPractice a turn, so every civ can learn both.
    const P = CIV.heldPractice;
    if (hunted) d.practice.herding += 1;
    else if (civClans(s, civ.id).reduce((a, c) => a + (c.harvest?.game ?? 0), 0) >= CIV.herdGame) d.practice.herding += P;
    if (dug) d.practice.pottery += 1;
    else if ((civ.store.clay ?? 0) >= CIV.potteryClay) d.practice.pottery += P;
  }
}

// ---- actions --------------------------------------------------------------------------------

export function projectError(s, civId, city, kind) {
  if (!city || city.owner !== civId) return 'not your city';
  if (!CIV.projects[kind]) return `no project ${kind}`;
  if (city.buildings[kind]) return 'already built';
  if (city.project) return 'city already has a project';
  const short = Object.entries(CIV.projects[kind].goods).filter(([g, v]) => (s.civs[civId].store[g] ?? 0) < v)
    .map(([g, v]) => `${g} ${fmt(s.civs[civId].store[g] ?? 0)}/${v}`);
  return short.length ? `not enough in the store: ${short.join(', ')}` : null;
}

const giftedNow = (s, clanId, civId) => {
  const g = s.mods.civ.gifted[clanId]?.[civId];
  return g && g.turn === s.turn ? g.amount : 0;
};

// Leaning a gift of `amount` would add, after what this civ already gave the clan this turn.
export function giftGain(s, clan, civId, amount) {
  const hunger = clan.starving ? CIV.giftHunger.starving : clan.fed ? CIV.giftHunger.fed : CIV.giftHunger.hungry;
  const f = (x) => { const w = x / Math.max(1, clan.pop); return CIV.giftLean * hunger * w / (1 + w); };
  const had = giftedNow(s, clan.id, civId);
  return f(had + amount) - f(had);
}

export function giftError(s, civId, clanId, amount) {
  if (typeof amount !== 'number' || !Number.isFinite(amount) || amount <= 0) return 'gift must be a positive amount of food';
  const c = s.clans[clanId];
  if (!c) return 'no such clan';
  if (c.owner != null) return 'that clan already belongs to a civ';
  const room = CIV.giftMax - giftedNow(s, clanId, civId);
  if (amount > room + 1e-9) return room > 0 ? `at most ${fmt(room)} more food to this clan this turn` : 'already gave this clan all it can take this turn';
  return foodIn(s.civs[civId].store) >= amount - 1e-9 ? null : 'not enough food in the store';
}

// Where a daughter may be sent: a passable tile within a split's reach of one of the civ's
// clans, not in another civ's territory or on another civ's clan's land.
export function siteError(s, civId, h) {
  const t = tileAt(s, h);
  if (!t) return 'no such tile';
  if (!passable(s, h)) return 'impassable';
  const owner = territoryOf(s, h);
  if (owner != null && owner !== civId) return `${s.civs[owner]?.name ?? 'another civ'} holds that ground`;
  const holder = s.clans[t.claim];
  if (holder?.owner != null && holder.owner !== civId) return `land of a ${s.civs[holder.owner]?.name} clan`;
  if (!civClans(s, civId).some((c) => distance(c, h) <= CLAN.scoutRange)) return `none of your clans within ${CLAN.scoutRange} steps`;
  return null;
}

const tamedNear = (s, h) => within(h, CITY.workRadius).some((x) => (tileAt(s, x)?.tame ?? 0) >= CIV.cityTame);

// Where the next city may go: a daughter site that is also spaced from every city and has
// tamed grain next to it.
export function citySiteError(s, civId, h) {
  const e = siteError(s, civId, h);
  if (e) return e;
  if (Object.values(s.cities).some((c) => distance(c, h) < CITY.minSpacing)) return `within ${CITY.minSpacing} of a city`;
  if (!tamedNear(s, h)) return `no tamed grain within ${CITY.workRadius} (needs ${Math.round(CIV.cityTame * 100)}% tamed)`;
  if (civCities(s, civId).length >= CIV.maxCities) return `at most ${CIV.maxCities} cities`;
  return null;
}

export const hasWall = (city) => !!city.buildings?.wall;

export const actions = {
  setProject: {
    validate: (s, a) => projectError(s, a.civ, s.cities[a.city], a.project),
    apply: (s, a) => {
      const c = s.cities[a.city];
      for (const [g, v] of Object.entries(CIV.projects[a.project].goods)) takeGoods(s, a.civ, g, v);
      c.project = { kind: a.project, labour: 0 };
      note(s, c.id, `began the ${a.project}`);
    },
    candidates: (s, civId) => civCities(s, civId).filter((c) => !c.project).flatMap((c) =>
      Object.keys(CIV.projects).filter((p) => !projectError(s, civId, c, p))
        .map((project) => ({ type: 'setProject', civ: civId, city: c.id, project }))),
  },
  // Food from the civ store to an unaligned clan: at most giftMax a turn, and each extra unit
  // buys less leaning (see giftGain), so winning a clan takes food over several turns.
  giftFood: {
    validate: (s, a) => giftError(s, a.civ, a.clan, a.amount),
    apply: (s, a) => {
      const c = s.clans[a.clan];
      const gain = giftGain(s, c, a.civ, a.amount);
      const store = s.civs[a.civ].store;
      let left = a.amount;
      for (const f of [...FOODS].sort((x, y) => (store[y] ?? 0) - (store[x] ?? 0))) {
        const t = takeGoods(s, a.civ, f, left);
        if (t) { c.store[f] = (c.store[f] ?? 0) + t; left -= t; }
      }
      const g = (s.mods.civ.gifted[c.id] ??= {});
      g[a.civ] = { turn: s.turn, amount: giftedNow(s, c.id, a.civ) + a.amount };
      c.leaning[a.civ] = (c.leaning[a.civ] ?? 0) + gain;
      note(s, c.id, `${s.civs[a.civ].name} gave ${fmt(a.amount)} food${c.starving ? ' while it starved' : ''}: ` +
        `leaning +${gain.toFixed(2)} to ${c.leaning[a.civ].toFixed(2)} (joins at ${CIV.joinAt})`);
    },
    candidates: (s, civId) => {
      const c = aiGiftTarget(s, civId);
      return c ? [{ type: 'giftFood', civ: civId, clan: c.id, amount: CIV.aiGift }] : [];
    },
  },
  // Choose where the next daughter of a civ clan goes (site "q,r", or null to clear). Used if
  // it is among her options (a site that would feed her); it lapses after siteTurns.
  setDaughterSite: {
    validate: (s, a) => (a.site == null ? null : siteError(s, a.civ, parse(a.site))),
    apply: (s, a) => {
      if (a.site == null) delete s.mods.civ.siteChoice[a.civ];
      else s.mods.civ.siteChoice[a.civ] = { k: key(parse(a.site)), turn: s.turn };
    },
    candidates: () => [],
  },
  // Choose where the next city goes (site "q,r", or null to clear). Daughters are steered
  // there, and the city is founded once a civ clan settles within cityNear of it. While a site
  // is chosen no city is founded anywhere else. It lapses after citySiteTurns.
  setCitySite: {
    validate: (s, a) => (a.site == null ? null : citySiteError(s, a.civ, parse(a.site))),
    apply: (s, a) => {
      if (a.site == null) delete s.mods.civ.citySite[a.civ];
      else s.mods.civ.citySite[a.civ] = { k: key(parse(a.site)), turn: s.turn };
    },
    candidates: () => [],
  },
  // How much of a good to keep back from market (keep: a number >= 0, or null for the default).
  // Everything above it is posted at the civ's market each turn.
  setPosting: {
    validate: (s, a) => (!GOODS.includes(a.good) ? `no good ${a.good}`
      : a.keep != null && (typeof a.keep !== 'number' || !Number.isFinite(a.keep) || a.keep < 0) ? 'keep must be a number >= 0'
        : null),
    apply: (s, a) => {
      const p = (s.mods.civ.posting[a.civ] ??= {});
      if (a.keep == null) delete p[a.good];
      else p[a.good] = a.keep;
    },
    candidates: () => [],
  },
};

// ---- hooks ----------------------------------------------------------------------------------

export const hooks = {
  daughterSite(s, clan, { options }) {
    if (clan.owner == null || !s.mods.civ) return undefined;
    const m = s.mods.civ;
    const want = m.siteChoice[clan.owner];
    if (want && options.some((o) => o.k === want.k)) {
      delete m.siteChoice[clan.owner];
      note(s, clan.id, `${s.civs[clan.owner].name} sends its daughter to ${want.k}`);
      return want.k;
    }
    const city = m.citySite[clan.owner];
    if (city) {
      const h = parse(city.k);
      const near = options.filter((o) => distance(parse(o.k), h) <= CIV.cityNear)
        .sort((a, b) => distance(parse(a.k), h) - distance(parse(b.k), h) || b.score - a.score)[0];
      if (near) { note(s, clan.id, `daughter heads for the chosen city site ${city.k}`); return near.k; }
    }
    const cities = civCities(s, clan.owner);
    const ok = options.filter((o) => cities.every((c) => distance(c, parse(o.k)) >= CITY.minSpacing));
    const pool = ok.length ? ok : options;
    if (!pool.length) return undefined;
    return pool.reduce((b, o) => (o.score > b.score ? o : b)).k;
  },
  yieldFactor: (s, t) => s.mods?.civ?.years?.[regionOf(t)]?.factor ?? 1,
};

// ---- turn steps -----------------------------------------------------------------------------

// Regions where a civ has a city or a clan.
const regionsOf = (s, civId) => new Set([...civCities(s, civId), ...civClans(s, civId)].map(regionOf));

function weather(s) {
  const m = s.mods.civ;
  for (const r of m.regions) {
    const y = m.years[r];
    if (y) { if (--y.left <= 0) delete m.years[r]; continue; }
    if (rand(s) < CIV.yearChance) {
      const bad = rand(s) < 0.5;
      const [lo, hi] = CIV.yearTurns;
      m.years[r] = { factor: bad ? CIV.drought : CIV.bumper, left: lo + Math.floor(rand(s) * (hi - lo + 1)) };
      const hit = Object.values(s.civs).filter((c) => regionsOf(s, c.id).has(r)).map((c) => c.id);
      tell(s, bad ? hit : [], `${bad ? 'drought' : 'a bumper year'} in region ${r}` +
        (hit.length ? ` (${hit.map((id) => s.civs[id].name).join(', ')} land)` : ''));
    }
  }
}

const MATERIAL_OF = Object.entries(MAT_TILE);

// What a civ posting `out` will take at market: every good it is not posting (if it posts some
// of everything, foods, so food it can always use stays tradeable).
export function wantsOf(out) {
  const w = GOODS.filter((g) => !(out[g] > 0.05));
  return w.length ? w : FOODS;
}

// Each civ posts what it holds above its reserve at its own market. Clans trading where a civ
// posts bring a little of the wood, stone and clay their land yields, so a civ without hills
// can still get stone by posting food. Baskets are noted for trade leaning.
function post(s) {
  const m = s.mods.civ;
  m.snap = {};
  for (const civ of Object.values(s.civs)) {
    m.trade[civ.id] = { turn: s.turn, posted: {}, received: {}, at: {} };
    const mks = civCities(s, civ.id).length ? tradeMarkets(s, civ.id) : [];
    if (!mks.length) continue;
    const out = {};
    for (const [g, v] of Object.entries(civ.store)) {
      const over = v - keepOf(s, civ.id, g);
      if (over > 0.05) out[g] = over;
    }
    if (total(out) <= 0) continue;
    for (const [g, v] of Object.entries(out)) takeGoods(s, civ.id, g, v);
    m.trade[civ.id].posted = out;
    // The civ takes in exchange only goods it is not posting, so it never buys back its own surplus.
    const wants = wantsOf(out);
    for (const mk of mks) {
      const part = Object.fromEntries(Object.entries(out).map(([g, v]) => [g, v / mks.length]));
      mk.posts[civ.id] = part;
      (mk.wants ??= {})[civ.id] = wants;
      m.trade[civ.id].at[mk.id] = { posted: part, received: {} };
    }
  }
  for (const mk of Object.values(s.markets)) {
    if (!Object.keys(mk.posts ?? {}).some((id) => s.civs[id])) continue;
    for (const c of Object.values(s.clans)) {
      if (c.market !== mk.id) continue;
      if (!c.starving)
        for (const k of c.land) {
          const t = s.map.tiles[k];
          for (const [g, is] of MATERIAL_OF)
            if (is(t)) c.basket[g] = (c.basket[g] ?? 0) + CIV.clanMat * hooks.yieldFactor(s, t);
        }
      m.snap[c.id] = { ...c.basket };
    }
  }
}

const moved = (a, b) => [...new Set([...Object.keys(a), ...Object.keys(b)])]
  .reduce((x, g) => x + Math.abs((b[g] ?? 0) - (a[g] ?? 0)), 0) / 2;

// Changes a clan's leaning toward a civ; notes it only when it crosses a leanNoteStep.
function lean(s, c, civId, delta, why) {
  const before = c.leaning[civId] ?? 0;
  const after = Math.max(0, before + delta);
  if (after < 1e-4) delete c.leaning[civId];
  else c.leaning[civId] = after;
  const step = (x) => Math.floor(x / CIV.leanNoteStep);
  if (step(after) !== step(before) && s.civs[civId])
    note(s, c.id, `leaning toward ${s.civs[civId].name} ${after > before ? 'rose' : 'fell'} to ${after.toFixed(2)} ` +
      `(joins at ${CIV.joinAt}): ${why}`);
}

function collect(s) {
  const m = s.mods.civ;
  for (const civ of Object.values(s.civs)) {
    const t = m.trade[civ.id];
    if (t?.turn !== s.turn) continue;
    for (const [mid, x] of Object.entries(t.at)) {
      const got = s.markets[mid]?.received?.[civ.id];
      if (!got) continue;
      putGoods(s, civ.id, got);
      x.received = got;
      t.received = addGoods(t.received, got);
    }
  }
  // Trade leaning: the goods a clan traded (per head, at most 1) times the civ's share of all
  // goods that changed hands at that market.
  for (const [id, before] of Object.entries(m.snap ?? {})) {
    const c = s.clans[id];
    if (!c) continue;
    const vol = moved(before, c.basket);
    for (const g of Object.keys(c.basket)) if (!FOODS.includes(g)) delete c.basket[g];
    if (c.owner != null) continue;
    const mk = s.markets[c.market];
    if (!mk?.lastVolume) continue;
    for (const civ of Object.values(s.civs)) {
      const x = m.trade[civ.id]?.at?.[mk.id];
      if (!x) continue;
      const share = moved(x.posted, x.received) / (2 * mk.lastVolume);
      const gain = CIV.tradeLean * Math.min(1, vol / Math.max(1, c.pop)) * share;
      if (gain > 0) lean(s, c, civ.id, gain, `trades at a market where ${civ.name} posts`);
    }
  }
  delete m.snap;
}

function upkeep(s) {
  const m = s.mods.civ;
  for (const civ of Object.values(s.civs)) {
    const cities = civCities(s, civ.id);
    // member clans near a granary, or knowing pottery, lose less of their store (clans.js
    // spoiled it at CLAN.spoilage; give back the difference)
    for (const c of civClans(s, civ.id)) {
      const k = clanSpoil(s, c);
      if (k < 1) for (const f of Object.keys(c.store)) c.store[f] *= (1 - CLAN.spoilage * k) / (1 - CLAN.spoilage);
    }
    const spoil = CLAN.spoilage * storeSpoil(s, civ.id);
    for (const g of Object.keys(civ.store)) if (FOODS.includes(g)) civ.store[g] *= 1 - spoil;
    // fed member clans hand a share of their leftover food to the civ
    for (const c of civClans(s, civ.id)) {
      if (!c.fed) continue;
      const give = {};
      for (const f of Object.keys(c.store)) { give[f] = c.store[f] * CIV.levy; c.store[f] -= give[f]; }
      putGoods(s, civ.id, give);
    }
    const cap = capOf(s, civ.id), held = total(civ.store);
    m.overflow[civ.id] = Math.max(0, held - cap);
    // past the cap, materials and crafts are left to rot first, then food
    let over = held - cap;
    for (const isFood of [false, true]) {
      if (over <= 0) break;
      const gs = Object.keys(civ.store).filter((g) => FOODS.includes(g) === isFood);
      const sum = gs.reduce((a, g) => a + civ.store[g], 0);
      const cut = Math.min(1, over / (sum || 1));
      for (const g of gs) civ.store[g] *= 1 - cut;
      over -= sum * cut;
    }
    for (const g of Object.keys(civ.store)) if (civ.store[g] < 0.01) delete civ.store[g];
    expireSites(s, civ);
    if (cities.length && cities.length < CIV.maxCities) found(s, civ);
  }
}

function expireSites(s, civ) {
  const m = s.mods.civ;
  const d = m.siteChoice[civ.id];
  if (d && s.turn - d.turn >= CIV.siteTurns) {
    delete m.siteChoice[civ.id];
    tell(s, civ.id, `${civ.name}: no daughter went to ${d.k} in ${CIV.siteTurns} turns; the choice lapses`);
  }
  const c = m.citySite[civ.id];
  if (!c) return;
  const err = citySiteError(s, civ.id, parse(c.k));
  if (err || s.turn - c.turn >= CIV.citySiteTurns) {
    delete m.citySite[civ.id];
    tell(s, civ.id, `${civ.name}: the city site ${c.k} lapses (${err ?? `no clan settled there in ${CIV.citySiteTurns} turns`})`);
  }
}

// A settled civ clan founds a city: at the chosen site once it lives within cityNear of it,
// else (no site chosen) where it stands, if spaced from every city and next to tamed grain.
function found(s, civ) {
  const site = s.mods.civ.citySite[civ.id];
  const settled = civClans(s, civ.id).filter((c) => !c.route.length);
  let at = null, by = null;
  if (site) {
    const h = parse(site.k);
    by = settled.find((c) => distance(c, h) <= CIV.cityNear);
    if (by) { at = h; delete s.mods.civ.citySite[civ.id]; }
  } else {
    by = settled.find((c) => c.since >= CIV.newCitySince && c.pop >= CIV.newCityPop
      && Object.values(s.cities).every((x) => distance(x, c) >= CITY.minSpacing) && tamedNear(s, c));
    at = by;
  }
  if (!at) return;
  const city = newCity(s, civ, at);
  note(s, city.id, `founded by ${by.name} at ${key(at)}`);
  tell(s, civ.id, `${civ.name} founds ${city.name} at ${key(at)}`);
}

function join(s) {
  for (const c of Object.values(s.clans)) {
    // a civ with no cities left cannot hold its clans: they go back to living on their own
    if (c.owner != null && !civCities(s, c.owner).length) {
      const was = s.civs[c.owner]?.name ?? 'its civ';
      c.owner = null; c.drafted = 0;
      note(s, c.id, `unaligned again: ${was} has no cities left`);
    }
    if (c.owner != null) continue;
    for (const id of Object.keys(c.leaning)) {
      if (!s.civs[id]) { delete c.leaning[id]; continue; }
      lean(s, c, Number(id), -c.leaning[id] * CIV.leanDecay, 'trade and gifts fade');
    }
    const best = Object.entries(c.leaning).sort((a, b) => b[1] - a[1])[0];
    if (best && best[1] >= CIV.joinAt) {
      const civ = s.civs[best[0]];
      c.owner = civ.id;
      c.drafted = 0;
      note(s, c.id, `joined ${civ.name}: leaning reached ${best[1].toFixed(2)} through trade and gifts`);
      tell(s, civ.id, `${c.name} joins ${civ.name}`);
    }
  }
}

// Civs in trading contact: at some market both post (see tradeMarkets) or have clans trading.
export function contacts(s) {
  const at = {};
  const add = (mid, civId) => (at[mid] ??= new Set()).add(civId);
  for (const civ of Object.values(s.civs)) if (civCities(s, civ.id).length) for (const mk of tradeMarkets(s, civ.id)) add(mk.id, civ.id);
  for (const c of Object.values(s.clans)) if (c.market != null && c.owner != null) add(c.market, c.owner);
  const out = Object.fromEntries(Object.keys(s.civs).map((id) => [id, new Set()]));
  for (const set of Object.values(at))
    for (const a of set) for (const b of set) if (a !== b) out[a]?.add(b);
  return out;
}

function discover(s) {
  const m = s.mods.civ;
  const touch = contacts(s);
  for (const civ of Object.values(s.civs)) {
    const d = m.disc[civ.id];
    if (!d) continue;
    for (const g of Object.keys(CIV.practice)) {
      if (d.known[g]) continue;
      const from = [...touch[civ.id]].filter((o) => m.disc[o]?.known[g]);
      d.practice[g] += CIV.spread * from.length;
      if (d.practice[g] >= CIV.practice[g]) {
        d.known[g] = true;
        tell(s, civ.id, `${civ.name} learns ${g}${from.length ? ` (helped by trade with ${from.map((o) => s.civs[o].name).join(', ')})` : ''}`);
        note(s, civ.id, `learned ${g}${from.length ? ' by practice and trade' : ' by practice'}`);
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

// Prosperity score: the civ's people now over its people at founding. Founding hands the
// richest hearth the most people, so counting heads alone hands it the win; growth (division,
// clans joining, cities that keep people fed) is what the player can actually change.
export function prosperity(s, civId) {
  const pop = popOf(s, civId), start = s.mods.civ.startPop?.[civId] || 1;
  return { pop, start, score: pop / start };
}

// Prosperity: the highest score among civs still holding a city; ties go to more people, then
// the lower id.
export function prosperityWinner(s) {
  const alive = Object.values(s.civs).filter((c) => civCities(s, c.id).length);
  const pool = alive.length ? alive : Object.values(s.civs);
  const rank = pool.map((c) => ({ id: c.id, ...prosperity(s, c.id) }))
    .sort((a, b) => b.score - a.score || b.pop - a.pop || a.id - b.id);
  return rank[0].id;
}

// Alerts for cities changing hands (army captures with transferCity), and civs left with none.
function watchCities(s) {
  const m = s.mods.civ;
  for (const city of Object.values(s.cities)) {
    const was = m.cityOwners[city.id];
    if (was != null && was !== city.owner)
      tell(s, [was, city.owner], `${s.civs[city.owner]?.name ?? 'someone'} takes ${city.name} from ${s.civs[was]?.name ?? 'its owner'}`);
    m.cityOwners[city.id] = city.owner;
  }
  for (const civ of Object.values(s.civs))
    if (m.lost[civ.id] == null && !civCities(s, civ.id).length) {
      m.lost[civ.id] = s.turn;
      tell(s, civ.id, `${civ.name} has lost its last city`);
    }
}

export function victory(s) {
  const m = s.mods.civ;
  watchCities(s);
  if (s.winner != null) return;
  const c = conquerer(s);
  if (c != null) { s.winner = c; m.winKind = 'conquest'; }
  else if (s.turn - m.startTurn >= CIV.turnLimit) { s.winner = prosperityWinner(s); m.winKind = 'prosperity'; }
  else return;
  m.winTurn = s.turn;
  const human = Object.values(s.civs).find((x) => x.isHuman);
  const tail = !human ? '' : human.id === s.winner ? ': you win' : `: ${human.name} loses`;
  event(s, `${human ? '! ' : ''}${s.civs[s.winner].name} wins by ${m.winKind}${tail}`);
}

export const turnSteps = [
  { phase: 'regrow', name: 'civ weather', order: 'before', run: weather },
  { phase: 'forage', name: 'civ draft', order: 'before', run: draft },
  { phase: 'forage', name: 'civ labour', run: work },
  { phase: 'trade', name: 'civ post', order: 'before', run: post },
  { phase: 'trade', name: 'civ collect', run: collect },
  { phase: 'settle', name: 'civ upkeep', run: upkeep },
  { phase: 'markets', name: 'civ join', run: join },
  { phase: 'discoveries', name: 'civ discoveries', run: discover },
  { phase: 'discoveries', name: 'civ victory', run: victory },
];

// ---- AI -------------------------------------------------------------------------------------

// The unaligned clan an AI civ courts: within aiGiftRange of one of its cities, starving first,
// then the one already leaning most toward it.
function aiGiftTarget(s, civId) {
  const cities = civCities(s, civId);
  const near = Object.values(s.clans).filter((c) => c.owner == null && cities.some((x) => distance(x, c) <= CIV.aiGiftRange));
  const rank = (c) => (c.starving ? 10 : 0) + (c.leaning[civId] ?? 0);
  return near.sort((a, b) => rank(b) - rank(a) || a.id - b.id)[0] ?? null;
}

export function ai(s, civId) {
  const civ = s.civs[civId];
  const out = [];
  const proj = actions.setProject.candidates(s, civId)[0];
  if (proj) out.push(proj);
  const reserve = FOODS.reduce((a, f) => a + keepOf(s, civId, f), 0);
  if (foodIn(civ.store) > reserve + CIV.aiGift) out.push(...actions.giftFood.candidates(s, civId));
  return out;
}
