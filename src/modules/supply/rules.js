// The supply module: army food, supply lines, caches, porters, forage and pillage.
// Captain food lives in u.supply = {food, foodMax, auto}; troops are read from u.troops or u.army.troops.
import { FOODS } from '../../data.js';
import { event, note, tileAt, stepCost, costMap, pathCost, territoryOf, cityAt, unitsAt, spawnUnit, removeUnit, civCities, civUnits } from '../../core.js';
import { SUPPLY } from './data.js';

export const name = 'supply';
const key = (h) => `${h.q},${h.r}`;
const unkey = (k) => { const [q, r] = k.split(',').map(Number); return { q, r }; };

export function init(s) { s.mods.supply = {}; }
export function onStart(s) {}

// ---- helpers ----------------------------------------------------------------------------------
const isCaptain = (u) => u.type === 'captain';
export const troopsOf = (u) => u.troops ?? u.army?.troops ?? 0;
function setTroops(u, n) { if (u.troops !== undefined || u.army?.troops === undefined) u.troops = n; else u.army.troops = n; }

export function ensure(u) {
  const cap = Math.max(1, troopsOf(u)) * SUPPLY.carryPerTroop;
  u.supply ??= { food: cap, foodMax: cap, auto: false };
  u.supply.foodMax = cap;
  u.supply.food = Math.min(u.supply.food, cap);
  return u.supply;
}

export const foodIn = (s, civId) => FOODS.reduce((a, f) => a + (s.civs[civId].store[f] ?? 0), 0);
function takeFood(s, civId, n) {
  const store = s.civs[civId].store;
  let got = 0;
  for (const f of FOODS) {
    const t = Math.min(n - got, store[f] ?? 0);
    if (t > 0) { store[f] -= t; got += t; if (store[f] <= 1e-9) delete store[f]; }
  }
  return got;
}

// Lines pass through friendly or neutral land only.
const lineCost = (civId) => (s, h) => {
  const o = territoryOf(s, h);
  return o != null && o !== civId ? Infinity : stepCost(s, h);
};

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
  const u = s.units[a.unitId];
  if (!u) return [null, 'no such unit'];
  if (u.owner !== a.civ) return [null, 'not your unit'];
  if (type && u.type !== type) return [null, `not a ${type}`];
  return [u, null];
};
const captains = (s, civId) => civUnits(s, civId).filter(isCaptain);
const porters = (s, civId) => civUnits(s, civId).filter((u) => u.type === 'porter');

function gather(kind, labour, enemy) {
  return {
    validate(s, a) {
      const [u, err] = mine(s, a, 'captain');
      if (err) return err;
      ensure(u);
      if (u.labour < labour) return `needs ${labour} labour`;
      if (u.supply.food >= u.supply.foodMax) return 'food store is full';
      const o = territoryOf(s, u);
      if (enemy !== (o != null && o !== u.owner)) return enemy ? 'not enemy land' : 'enemy land';
      const t = tileAt(s, u);
      if (t.fallow > 0) return 'tile is stripped';
      if (!Object.values(t.stock ?? {}).some((v) => v > 0.5)) return 'nothing to gather';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId], t = tileAt(s, u);
      const need = Math.min(SUPPLY.gather, u.supply.foodMax - u.supply.food);
      let got = 0;
      for (const f of Object.keys(t.stock)) {
        const x = Math.min(need - got, t.stock[f]);
        t.stock[f] -= x; got += x;
      }
      u.supply.food += got; u.labour -= labour;
      t.fallow = SUPPLY.regrowTurns;
      note(s, u.id, `${kind}d ${got.toFixed(1)} food`);
      if (enemy) event(s, `${kind}d enemy land`);
    },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: kind, civ, unitId: u.id })),
  };
}

export const actions = {
  resupply: {
    validate(s, a) {
      const [u, err] = mine(s, a, 'captain');
      if (err) return err;
      ensure(u);
      if (u.supply.food >= u.supply.foodMax) return 'food store is full';
      const src = supplySource(s, u);
      if (!src) return 'no supply line';
      if (u.labour < src.labour) return `needs ${src.labour} labour`;
      return null;
    },
    apply: (s, a) => { doResupply(s, s.units[a.unitId]); },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: 'resupply', civ, unitId: u.id })),
  },
  autoSupply: {
    validate: (s, a) => mine(s, a, 'captain')[1],
    apply: (s, a) => { const u = s.units[a.unitId]; ensure(u); u.supply.auto = a.on !== false; },
    candidates: (s, civ) => captains(s, civ).map((u) => ({ type: 'autoSupply', civ, unitId: u.id, on: true })),
  },
  forage: gather('forage', SUPPLY.forageLabour, false),
  pillage: gather('pillage', SUPPLY.pillageLabour, true),
  buildPorter: {
    validate(s, a) {
      const c = s.cities[a.cityId];
      if (!c || c.owner !== a.civ) return 'not your city';
      if (foodIn(s, a.civ) < SUPPLY.porterCost) return 'not enough food';
      return null;
    },
    apply(s, a) {
      const c = s.cities[a.cityId];
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
      return c && c.owner === a.civ ? null : 'not in your city';
    },
    apply(s, a) { const u = s.units[a.unitId]; u.supply.food += takeFood(s, a.civ, u.supply.foodMax - u.supply.food); },
    candidates: () => [],
  },
  unloadPorter: {
    validate(s, a) { return mine(s, a, 'porter')[1] ?? (s.units[a.unitId].supply.food > 0 ? null : 'empty'); },
    apply(s, a) { unload(s, s.units[a.unitId], Infinity); },
    candidates: () => [],
  },
  setPorterRoute: {
    validate(s, a) {
      const [, err] = mine(s, a, 'porter');
      if (err) return err;
      const c = s.cities[a.cityId];
      if (!c || c.owner !== a.civ) return 'not your city';
      if (!a.to || !tileAt(s, a.to)) return 'no such tile';
      return null;
    },
    apply(s, a) {
      s.units[a.unitId].supply.route = { cityId: a.cityId, to: { q: a.to.q, r: a.to.r }, target: a.target ?? SUPPLY.porterCarry * 2 };
    },
    candidates: () => [],
  },
};

// Porter drops food on its tile: first to captains there, then into a cache (made if needed).
function unload(s, p, limit) {
  let amt = Math.min(p.supply.food, limit);
  for (const u of unitsAt(s, p)) {
    if (!isCaptain(u) || u.owner !== p.owner || amt <= 0) continue;
    ensure(u);
    const x = Math.min(amt, u.supply.foodMax - u.supply.food);
    u.supply.food += x; amt -= x; p.supply.food -= x;
  }
  if (amt > 0) {
    const t = tileAt(s, p), o = territoryOf(s, p);
    if (o != null && o !== p.owner) return;
    t.cache ??= { owner: p.owner, food: 0 };
    if (t.cache.owner !== p.owner) return;
    t.cache.food += amt; p.supply.food -= amt;
  }
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

function runPorter(s, p) {
  const route = p.supply.route;
  if (route) {
    const city = s.cities[route.cityId];
    if (!city || city.owner !== p.owner) { p.supply.route = null; return; }
    const at = (h) => p.q === h.q && p.r === h.r;
    const hasCaptain = unitsAt(s, route.to).some(isCaptain);
    if (at(city) && p.supply.food < p.supply.foodMax && ((tileAt(s, route.to).cache?.food ?? 0) < route.target || hasCaptain)) {
      p.supply.food += takeFood(s, p.owner, p.supply.foodMax - p.supply.food);
    }
    if (p.supply.food > 0 && !at(route.to)) walk(s, p, route.to);
    else if (p.supply.food > 0) {
      const room = Math.max(0, route.target - (tileAt(s, route.to).cache?.food ?? 0));
      unload(s, p, hasCaptain ? Infinity : room);
    } else if (!at(city)) walk(s, p, city);
  }
  if (p.supply.food > 0) p.supply.food = Math.max(0, p.supply.food - SUPPLY.porterEat);
}

// ---- turn steps -------------------------------------------------------------------------------
function armiesStep(s) {
  for (const u of Object.values(s.units)) {
    if (!isCaptain(u)) continue;
    ensure(u);
    if (u.supply.auto && u.supply.food < u.supply.foodMax) doResupply(s, u);
    const n = troopsOf(u);
    const kids = Object.values(s.units).filter((d) => d.type === 'detachment' && d.captainId === u.id);
    if (u.supply.food >= n) {
      u.supply.food -= n;
      u.hungry = false;
      for (const d of kids) d.hungry = false;
      const max = u.army?.troopsMax ?? u.troopsMax;
      if (max !== undefined && n < max) setTroops(u, Math.min(max, n + SUPPLY.heal));
    } else {
      u.supply.food = 0;
      u.hungry = true;
      for (const d of kids) {
        d.hungry = true;
        d.troops = Math.max(0, d.troops - Math.max(1, Math.ceil(d.troops * SUPPLY.attrition)));
        if (d.troops <= 0) removeUnit(s, d.id);
      }
      const left = n - Math.max(1, Math.ceil(n * SUPPLY.attrition));
      note(s, u.id, 'army is hungry and losing troops');
      if (left <= 0) { removeUnit(s, u.id); for (const d of kids) removeUnit(s, d.id); event(s, 'an army starved'); }
      else setTroops(u, left);
    }
  }
  for (const t of Object.values(s.map.tiles)) {
    if (!t.cache) continue;
    const o = territoryOf(s, t);
    if (o != null && o !== t.cache.owner) t.cache.owner = o; // captured with the tile
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
  for (const u of captains(s, civId)) {
    ensure(u);
    if (!u.supply.auto) out.push({ type: 'autoSupply', civ: civId, unitId: u.id, on: true });
    if (!cities.length) continue;
    const cm = costMap(s, u, SUPPLY.farFromHome, { cost: lineCost(civId) });
    const nearCity = cities.some((c) => cm.has(key(c)));
    const hasCache = [...cm.keys()].some((k) => tileAt(s, unkey(k))?.cache?.owner === civId);
    if (nearCity || hasCache || ps.some((p) => p.supply.route && key(p.supply.route.to) === key(u))) continue;
    const idle = ps.find((p) => !p.supply.route);
    const dist = (c) => Math.abs(c.q - u.q) + Math.abs(c.r - u.r);
    const home = cities.reduce((b, c) => (dist(c) < dist(b) ? c : b));
    if (idle) out.push({ type: 'setPorterRoute', civ: civId, unitId: idle.id, cityId: home.id, to: { q: u.q, r: u.r }, target: SUPPLY.porterCarry * 2 });
    else if (!ps.length) out.push({ type: 'buildPorter', civ: civId, cityId: home.id });
  }
  return out;
}
