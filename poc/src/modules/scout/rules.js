// The scout module: scout unit, routes, revealing hidden trail costs, trails, fog, knowledge spread.
import { note, event, tileAt, pathCost, costMap, stepCost, spawnUnit, civUnits, civCities, civClans } from '../../core.js';
import { TERRAIN, TRAILS } from '../../data.js';
import { SCOUT } from './data.js';
import { distance, eq, key, within } from '../../hex.js';
import { rand } from '../../rng.js';

export const name = 'scout';

// roads: {civId: [pairKey]} road routes (city to city) the AI has finished or given up on.
export function init(s) {
  s.mods.scout = { roads: {} };
}

export const scoutsOf = (s, civId) => civUnits(s, civId).filter((u) => u.type === 'scout');
const isHex = (h) => h && Number.isInteger(h.q) && Number.isInteger(h.r);
const isHuman = (s, civId) => !!s.civs[civId]?.isHuman;

function spawnScout(s, civId, at) {
  const u = spawnUnit(s, { owner: civId, type: 'scout', q: at.q, r: at.r, labourMax: SCOUT.labourMax, scout: { route: null, tried: [] } });
  markSeen(s, civId, u, SCOUT.sight.unit);
  return u;
}

export function onStart(s) {
  s.mods.scout ??= {};
  s.mods.scout.roads ??= {};
  for (const civ of Object.values(s.civs)) {
    civ.seen ??= {};
    civ.revealed ??= {};
    const home = civCities(s, civ.id)[0] ?? s.markets[civ.marketId];
    if (home && !scoutsOf(s, civ.id).length) spawnScout(s, civ.id, home);
  }
  updateFog(s);
}

// ---- route planning -------------------------------------------------------------------------

const trailable = (t) => t.trailCost != null && !(t.span > SCOUT.bridgeSpan);
const baseCost = (t) => (TERRAIN[t.terrain].passable ? TERRAIN[t.terrain].moveCost : Infinity);

// What civ c believes tile h costs once a trail exists: {best, worst}. A trail on a tile the civ
// has never seen is unknown to it.
export function plannedCost(s, civId, h) {
  const t = tileAt(s, h);
  if (!t) return { best: Infinity, worst: Infinity };
  const civ = s.civs[civId];
  const base = baseCost(t);
  const spec = TRAILS[t.terrain];
  if (t.trail && civ?.seen?.[key(h)]) return { best: stepCost(s, h), worst: stepCost(s, h) };
  if (!spec) return { best: base, worst: base };
  if (civ?.revealed?.[key(h)]) {
    const c = trailable(t) ? Math.min(base, t.trailCost) : base;
    return { best: c, worst: c };
  }
  const canSpan = !spec.span || spec.span.some(([sp]) => sp != null && sp <= SCOUT.bridgeSpan);
  return { best: canSpan ? Math.min(base, ...spec.cost.map(([c]) => c)) : base, worst: base };
}

// known: best sure route; possible: best route unrevealed tiles allow; unknown: tiles worth revealing.
export function routePlan(s, civId, from, to) {
  const known = pathCost(s, from, to, { cost: (_, h) => plannedCost(s, civId, h).worst });
  const possible = pathCost(s, from, to, { cost: (_, h) => plannedCost(s, civId, h).best });
  if (!possible) return { known, possible, unknown: [] };
  const unknown = (known?.cost ?? Infinity) > possible.cost
    ? possible.path.slice(1).filter((h) => { const c = plannedCost(s, civId, h); return c.best < c.worst; })
    : [];
  return { known, possible, unknown };
}

function needsTrail(s, civId, h) {
  const t = tileAt(s, h);
  if (t.trail || !TRAILS[t.terrain] || !s.civs[civId].revealed?.[key(h)] || !trailable(t)) return false;
  return !TERRAIN[t.terrain].passable || t.trailCost < baseCost(t);
}

// The reveal and trail jobs left on a route, in the order the scout should do them.
function jobs(s, civId, at, plan) {
  const reveals = plan.unknown
    .map((h) => ({ kind: 'reveal', hex: h, c: plannedCost(s, civId, h), d: distance(at, h) }))
    .sort((a, b) => (a.c.worst === Infinity ? 0 : 1) - (b.c.worst === Infinity ? 0 : 1)
      || (a.c.best - a.c.worst) - (b.c.best - b.c.worst) || a.d - b.d);
  const route = plan.unknown.length ? plan.possible.path : plan.known?.path ?? [];
  const trail = route.slice(1).find((h) => needsTrail(s, civId, h));
  return trail ? [...reveals, { kind: 'trail', hex: trail }] : reveals;
}

// Whether a route from -> to has anything for a scout to reveal or build.
export function routeHasWork(s, civId, from, to) {
  const plan = routePlan(s, civId, from, to);
  return !!plan.possible && plan.possible.cost < Infinity && jobs(s, civId, from, plan).length > 0;
}

function markSeen(s, civId, h, r) {
  const civ = s.civs[civId];
  if (!civ) return;
  civ.seen ??= {};
  for (const n of within(h, r)) if (tileAt(s, n)) civ.seen[key(n)] = true;
}

// One step toward `target` over real costs. An impassable target (an unrevealed mountain) is
// reached by standing next to it, so it costs nothing to "enter" in the search.
// Returns 'there' (adjacent, or on a passable target), 'moved', 'tired' (no labour) or 'blocked'.
function stepToward(s, u, target) {
  const open = stepCost(s, target) < Infinity;
  if (open ? eq(u, target) : distance(u, target) <= 1) return 'there';
  const r = pathCost(s, u, target, { cost: (_, h) => (eq(h, target) && !open ? 0 : stepCost(s, h)) });
  if (!r || r.path.length < 2) return 'blocked';
  const next = r.path[1];
  if (!open && eq(next, target)) return 'there';
  const c = stepCost(s, next);
  if (c > u.labour && u.labour < u.labourMax) return 'tired';
  u.labour = Math.max(0, u.labour - c);
  u.q = next.q;
  u.r = next.r;
  markSeen(s, u.owner, u, SCOUT.sight.unit);
  return 'moved';
}

function endRoute(s, u, text, { alert = false } = {}) {
  const rt = u.scout.route;
  note(s, u.id, text);
  if (isHuman(s, u.owner)) event(s, `${alert ? '! ' : ''}scout ${u.id}: ${text}`);
  if (rt?.pair) {
    const done = (s.mods.scout.roads[u.owner] ??= []);
    if (!done.includes(rt.pair)) done.push(rt.pair);
  }
  u.scout.route = null;
}

// Spends the scout's labour on its route: reveal and build along it, then walk to its end.
// Clears the route when finished, or when the scout cannot get anywhere.
export function scoutWork(s, u) {
  let guard = 64, plan = null;
  while (u.scout?.route && u.labour > 0 && guard-- > 0) {
    const rt = u.scout.route;
    if (rt.walking) {
      const res = stepToward(s, u, rt.to);
      if (res === 'there') { endRoute(s, u, `route finished at ${key(rt.to)}, cost ${rt.cost}`); return; }
      if (res === 'blocked') { endRoute(s, u, `stuck: cannot walk to ${key(rt.to)}`, { alert: true }); return; }
      if (res === 'tired') return;
      continue;
    }
    // The plan changes only when a tile is revealed or a trail finished, so it is kept until then.
    plan ??= routePlan(s, u.owner, rt.from, rt.to);
    if (!plan.possible || plan.possible.cost === Infinity) {
      endRoute(s, u, `no route can exist to ${key(rt.to)}`);
      return;
    }
    const todo = jobs(s, u.owner, u, plan);
    if (!todo.length) {
      if (rt.walk === false) { endRoute(s, u, `route finished, cost ${plan.known?.cost}`); return; }
      rt.walking = true;
      rt.cost = plan.known?.cost ?? null;
      continue;
    }
    let job = todo.find((j) => distance(u, j.hex) <= 1);
    if (!job) {
      for (const j of todo) {
        const res = stepToward(s, u, j.hex);
        if (res === 'blocked') continue;
        if (res === 'tired') return;
        job = j;
        break;
      }
      if (!job) {
        endRoute(s, u, `stuck at ${key(u)}: cannot reach ${key(todo[0].hex)}`, { alert: true });
        return;
      }
      if (distance(u, job.hex) > 1) continue;
      if (u.labour <= 0) return;
    }
    const t = tileAt(s, job.hex);
    if (job.kind === 'reveal') {
      s.civs[u.owner].revealed ??= {};
      s.civs[u.owner].revealed[key(t)] = true;
      u.labour -= SCOUT.revealLabour;
      plan = null;
      continue;
    }
    const need = TRAILS[t.terrain].work;
    const work = Math.min(u.labour, need - (t.trailWork ?? 0));
    t.trailWork = (t.trailWork ?? 0) + work;
    u.labour -= work;
    if (t.trailWork >= need) {
      delete t.trailWork;
      t.trail = true;
      plan = null;
      note(s, u.id, `built a ${TERRAIN[t.terrain].passable ? 'trail' : 'bridge'} at ${key(t)}`);
    }
  }
}

// ---- fog ------------------------------------------------------------------------------------

// Tiles a civ can see right now: around its units, cities and clans, and its clans' land.
export function visibleNow(s, civId) {
  const out = new Set();
  const mark = (h, r) => { for (const n of within(h, r)) out.add(key(n)); };
  for (const u of civUnits(s, civId)) mark(u, SCOUT.sight.unit);
  for (const c of civCities(s, civId)) mark(c, SCOUT.sight.city);
  for (const c of civClans(s, civId)) {
    mark(c, SCOUT.sight.clan);
    for (const k of c.land ?? []) if (typeof k === 'string') out.add(k);
  }
  return out;
}

export function updateFog(s) {
  for (const civ of Object.values(s.civs)) {
    civ.seen ??= {};
    for (const u of civUnits(s, civ.id)) markSeen(s, civ.id, u, SCOUT.sight.unit);
    for (const c of civCities(s, civ.id)) markSeen(s, civ.id, c, SCOUT.sight.city);
    for (const c of civClans(s, civ.id)) markSeen(s, civ.id, c, SCOUT.sight.clan);
  }
  for (const t of Object.values(s.map.tiles)) {
    const o = t.claim != null ? s.clans[t.claim]?.owner : null;
    if (o != null && s.civs[o]) s.civs[o].seen[key(t)] = true;
  }
}

// ---- knowledge spread -----------------------------------------------------------------------

function shareKnowledge(s) {
  for (const m of Object.values(s.markets)) {
    const civs = [...new Set(Object.values(s.clans).filter((c) => c.market === m.id && c.owner != null).map((c) => c.owner))]
      .filter((id) => s.civs[id]);
    for (const a of civs) for (const b of civs) {
      if (a === b || rand(s) >= SCOUT.shareChance) continue;
      const mine = Object.keys(s.civs[a].revealed ?? {}).filter((k) => !s.civs[b].revealed?.[k]);
      if (!mine.length) continue;
      s.civs[b].revealed ??= {};
      s.civs[b].revealed[mine[Math.floor(rand(s) * mine.length)]] = true;
    }
  }
}

// ---- actions --------------------------------------------------------------------------------

const ownScout = (s, a) => {
  const u = s.units[a.unit];
  return u && u.type === 'scout' && u.owner === a.civ ? u : null;
};

export const actions = {
  buildScout: {
    validate(s, a) {
      if (!civCities(s, a.civ).length) return 'no city to build a scout in';
      if (scoutsOf(s, a.civ).length >= SCOUT.maxPerCiv) return `at most ${SCOUT.maxPerCiv} scouts`;
      return null;
    },
    apply(s, a) {
      const city = (a.city != null && s.cities[a.city]?.owner === a.civ) ? s.cities[a.city] : civCities(s, a.civ)[0];
      note(s, spawnScout(s, a.civ, city).id, `raised at ${city.name ?? key(city)}`);
    },
    candidates: (s, civId) => (civCities(s, civId).length && scoutsOf(s, civId).length < SCOUT.maxPerCiv
      ? [{ type: 'buildScout', civ: civId }] : []),
  },
  // {civ, unit, from, to, walk?}: reveal and trail the best route from -> to, then walk to `to`
  // (unless walk is false).
  assignRoute: {
    validate(s, a) {
      if (!ownScout(s, a)) return 'not your scout';
      if (!isHex(a.from) || !isHex(a.to) || !tileAt(s, a.from) || !tileAt(s, a.to)) return 'bad route ends';
      return routePlan(s, a.civ, a.from, a.to).possible ? null : 'no route can exist';
    },
    apply(s, a) {
      const u = ownScout(s, a);
      u.scout.route = { from: { q: a.from.q, r: a.from.r }, to: { q: a.to.q, r: a.to.r } };
      if (a.walk === false) u.scout.route.walk = false;
      if (a.pair) u.scout.route.pair = a.pair;
      note(s, u.id, `route ${key(a.from)} to ${key(a.to)}`);
    },
    candidates: () => [],
  },
  cancelRoute: {
    validate(s, a) {
      const u = ownScout(s, a);
      if (!u) return 'not your scout';
      return u.scout?.route ? null : 'scout has no route';
    },
    apply(s, a) {
      const u = ownScout(s, a);
      u.scout.route = null;
      note(s, u.id, 'route cancelled');
    },
    candidates: () => [],
  },
};

export const turnSteps = [
  { phase: 'trade', name: 'shareKnowledge', run: shareKnowledge },
  { phase: 'orders', name: 'scoutWork', order: 'after', run: (s) => {
    for (const u of Object.values(s.units)) if (u.type === 'scout') scoutWork(s, u);
  } },
  { phase: 'discoveries', name: 'fog', run: updateFog },
];

export const hooks = {};

// ---- AI -------------------------------------------------------------------------------------

// City pairs worth a trail: the capital to each rival city (nearest first), then to its own cities.
function roadPairs(s, civId) {
  const mine = civCities(s, civId);
  const cap = mine[0];
  if (!cap) return [];
  const rivals = Object.values(s.cities).filter((c) => c.owner !== civId && c.owner != null)
    .sort((a, b) => distance(cap, a) - distance(cap, b) || a.id - b.id);
  return [...rivals, ...mine.slice(1)].map((c) => ({ from: cap, to: c, pair: `${cap.id}-${c.id}` }));
}

function nextRoad(s, civId, taken) {
  const done = s.mods.scout.roads?.[civId] ?? [];
  for (const p of roadPairs(s, civId)) {
    if (done.includes(p.pair) || taken.has(p.pair)) continue;
    if (routeHasWork(s, civId, p.from, p.to)) return p;
    (s.mods.scout.roads[civId] ??= []).push(p.pair);
  }
  return null;
}

// The cheapest unseen tile the scout believes it can reach, skipping targets it already tried.
function exploreTarget(s, u) {
  const civ = s.civs[u.owner];
  const tried = new Set(u.scout.tried ?? []);
  const reach = costMap(s, u, 80, { cost: (_, h) => plannedCost(s, u.owner, h).best });
  let to = null, best = Infinity;
  for (const [k, c] of reach) {
    if (civ.seen?.[k] || tried.has(k)) continue;
    const t = s.map.tiles[k];
    if (!t || !TERRAIN[t.terrain].passable) continue;
    if (c < best) { best = c; to = t; }
  }
  return to;
}

export function ai(s, civId) {
  s.mods.scout.roads ??= {};
  const scouts = scoutsOf(s, civId);
  const want = civCities(s, civId).length >= 2 ? 2 : 1;
  if (scouts.length < Math.min(want, SCOUT.maxPerCiv)) return actions.buildScout.candidates(s, civId);
  const out = [];
  const taken = new Set(scouts.map((u) => u.scout.route?.pair).filter(Boolean));
  scouts.forEach((u, i) => {
    if (u.scout.route) return;
    // The first scout builds roads toward rivals and between cities; the others explore.
    const road = i === 0 ? nextRoad(s, civId, taken) : null;
    if (road) {
      taken.add(road.pair);
      out.push({ type: 'assignRoute', civ: civId, unit: u.id, from: { q: road.from.q, r: road.from.r },
        to: { q: road.to.q, r: road.to.r }, walk: false, pair: road.pair });
      return;
    }
    const to = exploreTarget(s, u);
    if (!to) return;
    u.scout.tried = [...(u.scout.tried ?? []), key(to)].slice(-40);
    const a = { type: 'assignRoute', civ: civId, unit: u.id, from: { q: u.q, r: u.r }, to: { q: to.q, r: to.r } };
    if (!actions.assignRoute.validate(s, a)) out.push(a);
  });
  return out;
}
