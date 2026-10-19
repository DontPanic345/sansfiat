// The supply module: army food, supply lines, caches, porters, forage and pillage.
// Captain food lives in u.supply = {food, foodMax, auto}; troops are read from u.troops or u.army.troops.
// A new captain starts with the food it was given (army provisions it from the civ store); nothing free.
import { FOODS } from '../../data.js';
import { event, note, tileAt, stepCost, costMap, pathCost, territoryOf, cityAt, unitsAt, spawnUnit, removeUnit, civCities, civUnits, putGoods } from '../../core.js';
import { distance } from '../../hex.js';
import { CITY } from '../../data.js';
import { SUPPLY } from './data.js';

export const name = 'supply';
const key = (h) => `${h.q},${h.r}`;

export function init(s) { s.mods.supply = { starved: 0, starvedBy: {} }; }
export function onStart(s) {}

// ---- helpers ----------------------------------------------------------------------------------
const isCaptain = (u) => u.type === 'captain';
export const troopsOf = (u) => u.troops ?? u.army?.troops ?? 0;
function setTroops(u, n) { if (u.troops !== undefined || u.army?.troops === undefined) u.troops = n; else u.army.troops = n; }
const uid = (a) => a.unit ?? a.unitId;
const cid = (a) => a.city ?? a.cityId;
const alert = (s, civ, text) => { if (s.civs[civ]?.isHuman) event(s, `! ${text}`); };
export const eatRate = (u) => troopsOf(u) * SUPPLY.eatPerTroop;

export function ensure(u) {
  const cap = Math.max(1, troopsOf(u)) * SUPPLY.carryPerTroop;
  u.supply ??= { food: 0, foodMax: cap, auto: false };
  u.supply.foodMax = cap;
  u.supply.food = Math.min(u.supply.food, cap);
  return u.supply;
}
// Read-only view of a unit's food for UIs (never mutates).
export const foodOf = (u) => {
  const max = Math.max(1, troopsOf(u)) * SUPPLY.carryPerTroop;
  return { food: Math.min(u.supply?.food ?? 0, max), foodMax: max, auto: !!u.supply?.auto };
};

export const foodIn = (s, civId) => FOODS.reduce((a, f) => a + (s.civs[civId].store[f] ?? 0), 0);
function takeFood(s, civId, n, into = null) {
  const store = s.civs[civId].store;
  let got = 0;
  for (const f of FOODS) {
    const t = Math.min(n - got, store[f] ?? 0);
    if (t > 0) { store[f] -= t; got += t; if (into) into[f] = (into[f] ?? 0) + t; if (store[f] <= 1e-9) delete store[f]; }
  }
  return got;
}
// A porter loads food from the store and remembers the mix (p.supply.mix: {food: amount loaded}),
// so food it brings home goes back as the same foods.
function loadPorter(s, p, n) {
  p.supply.mix ??= {};
  const got = takeFood(s, p.owner, n, p.supply.mix);
  p.supply.food += got;
  return got;
}
// Food a porter brings home, back into the store in the proportions it was loaded.
function returnLoad(s, p) {
  const mix = p.supply.mix ?? {};
  const sum = Object.values(mix).reduce((a, v) => a + v, 0);
  if (p.supply.food > 0)
    putGoods(s, p.owner, sum > 0 ? Object.fromEntries(Object.entries(mix).map(([f, v]) => [f, p.supply.food * v / sum])) : { grain: p.supply.food });
  p.supply.food = 0;
  delete p.supply.mix;
}

// Whose land a tile is underneath any troops standing on it: a fort, else the nearest city's
// territory, else the civ of the clan claiming it. Null = neutral. Forage and pillage go by this.
export function landOwner(s, h) {
  const t = tileAt(s, h);
  if (!t) return null;
  if (t.fort?.owner != null) return t.fort.owner;
  let best = null;
  for (const c of Object.values(s.cities)) {
    const d = distance(c, h);
    if (d <= CITY.territoryRadius && (!best || d < best.d || (d === best.d && c.id < best.id))) best = { d, id: c.id, owner: c.owner };
  }
  if (best) return best.owner;
  return s.clans[t.claim]?.owner ?? null;
}
const enemyOf = (o, civ) => o != null && o !== civ;

// Lines pass through friendly or neutral land only (troops standing on a tile make it theirs).
const lineCost = (civId) => (s, h) => (enemyOf(territoryOf(s, h), civId) ? Infinity : stepCost(s, h));

// Cheapest source for a unit: {kind:'city'|'cache', at, cost, labour}. Null if none in range.
export function supplySource(s, u) {
  const cm = costMap(s, u, SUPPLY.range, { cost: lineCost(u.owner) });
  let best = null;
  const consider = (kind, h, avail) => {
    const c = cm.get(key(h));
    if (c === undefined || avail <= 0) return;
    if (!best || c < best.cost || (c === best.cost && kind === 'city')) best = { kind, at: { q: h.q, r: h.r }, cost: c };
  };
  const avail = foodIn(s, u.owner);
  for (const c of civCities(s, u.owner)) consider('city', c, avail);
  for (const t of Object.values(s.map.tiles)) if (t.cache && t.cache.owner === u.owner) consider('cache', t, t.cache.food);
  if (best) best.labour = 1 + Math.floor(best.cost / SUPPLY.costPerLabour);
  return best;
}

export function supplyLine(s, u) {
  const src = supplySource(s, u);
  if (!src) return null;
  return { src, path: pathCost(s, src.at, u, { cost: lineCost(u.owner), maxCost: SUPPLY.range })?.path ?? [] };
}

function doResupply(s, u) {
  ensure(u);
  const src = supplySource(s, u);
  const want = u.supply.foodMax - u.supply.food;
  if (!src || want <= 0 || u.labour < src.labour) return false;
  let got;
  if (src.kind === 'city') got = takeFood(s, u.owner, want);
  else { const t = tileAt(s, src.at); got = Math.min(want, t.cache.food); t.cache.food -= got; }
  u.supply.food += got;
  u.labour -= src.labour;
  note(s, u.id, `resupplied ${got.toFixed(1)} food from a ${src.kind} (line cost ${src.cost})`);
  return got > 0;
}

// ---- actions ----------------------------------------------------------------------------------
const mine = (s, a, type) => {
  const u = s.units[uid(a)];
  if (!u) return [null, 'no such unit'];
  if (u.owner !== a.civ) return [null, 'not your unit'];
  if (type && u.type !== type) return [null, `not a ${type}`];
  return [u, null];
};
const captains = (s, civId) => civUnits(s, civId).filter(isCaptain);
const porters = (s, civId) => civUnits(s, civId).filter((u) => u.type === 'porter');

// Why a captain cannot forage (enemy=false) or pillage (enemy=true) here, or null.
export function gatherError(s, u, enemy) {
  const labour = enemy ? SUPPLY.pillageLabour : SUPPLY.forageLabour;
  const f = foodOf(u);
  if (u.labour < labour) return `needs ${labour} labour`;
  if (f.food >= f.foodMax - 1e-9) return 'food store is full';
  if (enemy !== enemyOf(landOwner(s, u), u.owner)) return enemy ? 'not enemy land' : 'enemy land: pillage instead';
  const t = tileAt(s, u);
  if (t.fallow > 0) return `tile is stripped (${t.fallow} turns)`;
  if (!FOODS.some((g) => (t.stock?.[g] ?? 0) > 0.5)) return 'nothing to gather';
  return null;
}

function gather(kind, labour, enemy) {
  return {
    validate(s, a) {
      const [u, err] = mine(s, a, 'captain');
      return err ?? gatherError(s, u, enemy);
    },
    apply(s, a) {
      const u = s.units[uid(a)], t = tileAt(s, u);
      ensure(u);
      const need = Math.min(SUPPLY.gather, u.supply.foodMax - u.supply.food);
      let got = 0;
      for (const f of FOODS) {
        const x = Math.min(need - got, t.stock[f] ?? 0);
        if (x > 0) { t.stock[f] -= x; got += x; }
      }
      u.supply.food += got; u.labour -= labour;
      t.fallow = SUPPLY.regrowTurns;
      note(s, u.id, `${kind}d ${got.toFixed(1)} food`);
      if (enemy) {
        const o = landOwner(s, u);
        event(s, `${s.civs[u.owner]?.name ?? 'an army'} pillages ${s.civs[o]?.name ?? 'enemy'} land`);
        alert(s, o, `${s.civs[u.owner]?.name ?? 'An enemy'} pillages your land at ${key(u)}`);
      }
    },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: kind, civ, unit: u.id })),
  };
}

// Where a porter would drop food for an army: the army's tile, else the last friendly/neutral tile
// on the way there from home. Null if there is none.
export function cacheSite(s, u, home) {
  if (!enemyOf(territoryOf(s, u), u.owner)) return { q: u.q, r: u.r };
  const p = pathCost(s, home, u)?.path ?? [];
  for (let i = p.length - 1; i >= 0; i--) if (!enemyOf(territoryOf(s, p[i]), u.owner)) return { q: p[i].q, r: p[i].r };
  return null;
}

export const actions = {
  resupply: {
    validate(s, a) {
      const [u, err] = mine(s, a, 'captain');
      if (err) return err;
      const f = foodOf(u);
      if (f.food >= f.foodMax) return 'food store is full';
      const src = supplySource(s, u);
      if (!src) return foodIn(s, u.owner) <= 0 ? 'no food in your store' : 'no supply line';
      if (u.labour < src.labour) return `needs ${src.labour} labour`;
      return null;
    },
    apply: (s, a) => { doResupply(s, s.units[uid(a)]); },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: 'resupply', civ, unit: u.id })),
  },
  autoSupply: {
    validate: (s, a) => mine(s, a, 'captain')[1],
    apply: (s, a) => { const u = s.units[uid(a)]; ensure(u); u.supply.auto = a.on !== false; },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: 'autoSupply', civ, unit: u.id, on: true })),
  },
  forage: gather('forage', SUPPLY.forageLabour, false),
  pillage: gather('pillage', SUPPLY.pillageLabour, true),
  buildPorter: {
    validate(s, a) {
      const c = s.cities[cid(a)];
      if (!c || c.owner !== a.civ) return 'not your city';
      if (foodIn(s, a.civ) < SUPPLY.porterCost) return `needs ${SUPPLY.porterCost} food in the store`;
      return null;
    },
    apply(s, a) {
      const c = s.cities[cid(a)];
      takeFood(s, a.civ, SUPPLY.porterCost);
      spawnUnit(s, { owner: a.civ, type: 'porter', q: c.q, r: c.r, supply: { food: 0, foodMax: SUPPLY.porterCarry, route: null } });
    },
    candidates: () => [],
  },
  loadPorter: {
    validate(s, a) {
      const [u, err] = mine(s, a, 'porter');
      if (err) return err;
      const c = cityAt(s, u);
      if (!c || c.owner !== a.civ) return 'not in your city';
      if (u.supply.food >= u.supply.foodMax) return 'already full';
      return foodIn(s, a.civ) > 0 ? null : 'no food in your store';
    },
    apply(s, a) { const u = s.units[uid(a)]; loadPorter(s, u, u.supply.foodMax - u.supply.food); },
    candidates: () => [],
  },
  unloadPorter: {
    validate(s, a) {
      const [u, err] = mine(s, a, 'porter');
      if (err) return err;
      if (!(u.supply.food > 0)) return 'empty';
      return unloadError(s, u);
    },
    apply(s, a) { unload(s, s.units[uid(a)], Infinity); },
    candidates: () => [],
  },
  setPorterRoute: {
    validate(s, a) {
      const [u, err] = mine(s, a, 'porter');
      if (err) return err;
      const city = cid(a) != null ? s.cities[cid(a)] : nearestCity(s, u);
      if (!city || city.owner !== a.civ) return 'not your city';
      const to = a.to ?? (a.q != null ? { q: a.q, r: a.r } : null);
      if (!to || !tileAt(s, to)) return 'no such tile';
      if (enemyOf(territoryOf(s, to), a.civ)) return 'enemy land: a porter cannot unload there';
      if (!pathCost(s, city, to)) return 'no path';
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      const city = cid(a) != null ? s.cities[cid(a)] : nearestCity(s, u);
      const to = a.to ?? { q: a.q, r: a.r };
      u.supply.route = { cityId: city.id, to: { q: to.q, r: to.r }, target: a.target ?? SUPPLY.porterCarry * 2, ...(a.for != null ? { for: a.for } : {}) };
      delete u.supply.returning;
    },
    candidates: () => [],
  },
  clearPorterRoute: {
    validate: (s, a) => mine(s, a, 'porter')[1],
    apply(s, a) { const u = s.units[uid(a)]; u.supply.route = null; },
    candidates: () => [],
  },
};

const nearestCity = (s, u) => civCities(s, u.owner).sort((a, b) => distance(a, u) - distance(b, u) || a.id - b.id)[0] ?? null;

function unloadError(s, p) {
  if (unitsAt(s, p).some((u) => isCaptain(u) && u.owner === p.owner)) return null;
  if (enemyOf(territoryOf(s, p), p.owner)) return 'enemy land: no cache here';
  const c = tileAt(s, p).cache;
  return c && c.owner !== p.owner ? 'someone else\'s cache' : null;
}

// Porter drops food on its tile: first to captains there, then into a cache (made if needed).
// Returns the food unloaded.
function unload(s, p, limit) {
  const start = p.supply.food;
  let amt = Math.min(p.supply.food, limit);
  for (const u of unitsAt(s, p)) {
    if (!isCaptain(u) || u.owner !== p.owner || amt <= 0) continue;
    ensure(u);
    const x = Math.min(amt, u.supply.foodMax - u.supply.food);
    u.supply.food += x; amt -= x; p.supply.food -= x;
  }
  if (amt > 0) {
    const t = tileAt(s, p), o = territoryOf(s, p);
    if (!enemyOf(o, p.owner) && (!t.cache || t.cache.owner === p.owner)) {
      t.cache ??= { owner: p.owner, food: 0 };
      t.cache.food += amt; p.supply.food -= amt;
    }
  }
  return start - p.supply.food;
}

function walk(s, u, to) {
  const r = pathCost(s, u, to);
  if (!r) return;
  for (const h of r.path.slice(1)) {
    const c = stepCost(s, h);
    if (u.labour <= 0 || (u.labour < c && u.labour < u.labourMax)) break;
    u.labour = Math.max(0, u.labour - c);
    u.q = h.q; u.r = h.r;
  }
}

function turnBack(s, p, why) {
  const route = p.supply.route;
  p.supply.route = null;
  p.supply.returning = route?.cityId ?? nearestCity(s, p)?.id ?? null;
  note(s, p.id, `turned back: ${why}`);
  alert(s, p.owner, `Porter ${p.id} turned back: ${why}`);
}

function runPorter(s, p) {
  const route = p.supply.route;
  const at = (h) => p.q === h.q && p.r === h.r;
  if (route) {
    const city = s.cities[route.cityId];
    if (!city || city.owner !== p.owner) { turnBack(s, p, 'its home city was lost'); return; }
    const hasCaptain = unitsAt(s, route.to).some((u) => isCaptain(u) && u.owner === p.owner);
    if (at(city) && p.supply.food < p.supply.foodMax && ((tileAt(s, route.to).cache?.food ?? 0) < route.target || hasCaptain)) {
      loadPorter(s, p, p.supply.foodMax - p.supply.food);
    }
    if (p.supply.food > 0 && !at(route.to)) {
      if (enemyOf(territoryOf(s, route.to), p.owner) && !hasCaptain) { turnBack(s, p, `${key(route.to)} is now enemy land`); }
      else walk(s, p, route.to);
    } else if (p.supply.food > 0) {
      const room = Math.max(0, route.target - (tileAt(s, route.to).cache?.food ?? 0));
      const err = unloadError(s, p);
      if (err) turnBack(s, p, err);
      else unload(s, p, hasCaptain ? Infinity : room);
    } else if (!at(city)) walk(s, p, city);
  } else if (p.supply.returning != null) {
    const city = s.cities[p.supply.returning];
    if (!city || city.owner !== p.owner) p.supply.returning = nearestCity(s, p)?.id ?? null;
    else if (at(city)) { returnLoad(s, p); delete p.supply.returning; }
    else walk(s, p, city);
  }
  if (p.supply.food > 0) p.supply.food = Math.max(0, p.supply.food - SUPPLY.porterEat);
}

// ---- turn steps -------------------------------------------------------------------------------
// Takes `loss` troops off an army, spread over its detachments in proportion to their size.
function attrit(s, u, kids, loss) {
  const n = troopsOf(u);
  if (kids.length) {
    const ks = [...kids].sort((a, b) => troopsOf(b) - troopsOf(a) || a.id - b.id);
    let left = loss;
    for (const d of ks) { const x = Math.min(troopsOf(d), Math.floor(loss * troopsOf(d) / Math.max(1, n))); setTroops(d, troopsOf(d) - x); left -= x; }
    while (left > 0 && ks.some((d) => troopsOf(d) > 0)) for (const d of ks) if (left > 0 && troopsOf(d) > 0) { setTroops(d, troopsOf(d) - 1); left--; }
    for (const d of ks) if (troopsOf(d) <= 0) removeUnit(s, d.id);
  }
  setTroops(u, Math.max(0, n - loss));
}

function armiesStep(s) {
  const m = s.mods.supply;
  for (const u of Object.values(s.units)) {
    if (!isCaptain(u) || !s.units[u.id]) continue;
    ensure(u);
    if (u.supply.auto && u.supply.food < u.supply.foodMax) doResupply(s, u);
    const n = troopsOf(u);
    const fought = u.army?.foughtTurn === s.turn ? u.army.foughtTroops ?? 0 : 0;
    const need = n * SUPPLY.eatPerTroop + fought * SUPPLY.fightEat;
    const kids = Object.values(s.units).filter((d) => d.type === 'detachment' && (d.army?.captainId ?? d.captainId) === u.id);
    if (u.supply.food >= need) {
      u.supply.food -= need;
      u.hungry = false;
      for (const d of kids) d.hungry = false;
      const max = u.army?.troopsMax ?? u.troopsMax;
      if (max !== undefined && n < max && !fought) { // heal when resting: the captain, or (deployed) his smallest detachment
        const add = Math.min(max, n + SUPPLY.heal) - n;
        const d = [...kids].sort((a, b) => troopsOf(a) - troopsOf(b) || a.id - b.id)[0];
        if (d) setTroops(d, troopsOf(d) + add);
        setTroops(u, n + add);
      }
    } else {
      const was = u.hungry;
      u.supply.food = 0;
      u.hungry = true;
      for (const d of kids) d.hungry = true;
      const loss = Math.min(n, Math.max(1, Math.round(n * SUPPLY.attrition)));
      attrit(s, u, kids, loss);
      m.starved = (m.starved ?? 0) + loss;
      (m.starvedBy ??= {})[u.owner] = (m.starvedBy[u.owner] ?? 0) + loss;
      note(s, u.id, `army is hungry and lost ${loss} troops`);
      if (!was) alert(s, u.owner, `Army ${u.id} is hungry: it cannot attack and loses troops each turn`);
      if (troopsOf(u) <= 0) {
        removeUnit(s, u.id);
        for (const d of kids) removeUnit(s, d.id);
        event(s, `an army of ${s.civs[u.owner]?.name ?? 'someone'} starved`);
        alert(s, u.owner, `Army ${u.id} starved to nothing`);
      } else alert(s, u.owner, `Army ${u.id} lost ${loss} troops to hunger (${troopsOf(u)} left)`);
    }
  }
  for (const t of Object.values(s.map.tiles)) {
    if (!t.cache) continue;
    const o = territoryOf(s, t);
    if (o != null && o !== t.cache.owner) { // captured with the tile
      alert(s, o, `You capture a supply cache at ${key(t)}`);
      alert(s, t.cache.owner, `Your supply cache at ${key(t)} is lost`);
      t.cache.owner = o;
    }
    t.cache.food *= 1 - SUPPLY.cacheSpoil;
    if (t.cache.food < 0.1 && !unitsAt(s, t).some((u) => u.type === 'porter')) t.cache = null;
  }
}

export const turnSteps = [
  { phase: 'orders', name: 'porters', order: 'after', run(s) { for (const u of Object.values(s.units)) if (u.type === 'porter') runPorter(s, u); } },
  { phase: 'armies', name: 'army food', order: 'after', run: armiesStep },
];

export const hooks = {};

export function ai(s, civId) {
  const out = [];
  const cities = civCities(s, civId);
  const ps = porters(s, civId);
  const caps = captains(s, civId);
  const store = foodIn(s, civId);
  const claimed = new Set();
  for (const u of caps) {
    const f = foodOf(u);
    if (!f.auto) out.push({ type: 'autoSupply', civ: civId, unit: u.id, on: true });
    // live off the land when low: pillage enemy land, else forage
    if (f.food < f.foodMax * SUPPLY.aiForageBelow) {
      if (!gatherError(s, u, true)) out.push({ type: 'pillage', civ: civId, unit: u.id });
      else if (!gatherError(s, u, false)) out.push({ type: 'forage', civ: civId, unit: u.id });
    }
    if (!cities.length || store < SUPPLY.porterCarry) continue;
    const src = supplySource(s, u);
    if (src && src.cost <= SUPPLY.farFromHome) continue;
    const home = nearestCity(s, u);
    const to = home && cacheSite(s, u, home);
    if (!to) continue;
    const mineP = ps.find((p) => p.supply.route?.for === u.id);
    const route = (p) => ({ type: 'setPorterRoute', civ: civId, unit: p.id, city: home.id, to, target: SUPPLY.porterCarry * 2, for: u.id });
    if (mineP) { if (!src && key(mineP.supply.route.to) !== key(to)) out.push(route(mineP)); continue; }
    const idle = ps.find((p) => !p.supply.route && p.supply.returning == null && !claimed.has(p.id));
    if (idle) { out.push(route(idle)); claimed.add(idle.id); }
    else if (ps.length < caps.length) out.push({ type: 'buildPorter', civ: civId, city: home.id });
  }
  // free porters whose army is gone
  for (const p of ps) if (p.supply.route?.for != null && !s.units[p.supply.route.for]) out.push({ type: 'clearPorterRoute', civ: civId, unit: p.id });
  return out;
}
