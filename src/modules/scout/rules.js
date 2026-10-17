// The scout module: scout unit, routes, revealing hidden trail costs, trails, fog, knowledge spread.
import { note, event, tileAt, pathCost, stepCost, spawnUnit, civUnits, civCities, civClans } from '../../core.js';
import { TERRAIN, TRAILS } from '../../data.js';
import { SCOUT } from './data.js';
import { distance, key, within } from '../../hex.js';
import { rand } from '../../rng.js';

export const name = 'scout';

export function init(s) {
  s.mods.scout = {};
}

const scoutsOf = (s, civId) => civUnits(s, civId).filter((u) => u.type === 'scout');
const isHex = (h) => h && Number.isInteger(h.q) && Number.isInteger(h.r);

function spawnScout(s, civId, at) {
  return spawnUnit(s, { owner: civId, type: 'scout', q: at.q, r: at.r, labourMax: SCOUT.labourMax, scout: { route: null } });
}

export function onStart(s) {
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

// What civ c believes tile h costs once a trail exists: {best, worst}.
export function plannedCost(s, civId, h) {
  const t = tileAt(s, h);
  if (!t) return { best: Infinity, worst: Infinity };
  const base = baseCost(t);
  const spec = TRAILS[t.terrain];
  if (t.trail) return { best: stepCost(s, h), worst: stepCost(s, h) };
  if (!spec) return { best: base, worst: base };
  if (s.civs[civId].revealed?.[key(h)]) {
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

function jobs(s, u, plan) {
  const reveals = plan.unknown
    .map((h) => ({ kind: 'reveal', hex: h, c: plannedCost(s, u.owner, h), d: distance(u, h) }))
    .sort((a, b) => (a.c.worst === Infinity ? 0 : 1) - (b.c.worst === Infinity ? 0 : 1)
      || (a.c.best - a.c.worst) - (b.c.best - b.c.worst) || a.d - b.d);
  const route = plan.unknown.length ? plan.possible.path : plan.known?.path ?? [];
  const trail = route.slice(1).find((h) => needsTrail(s, u.owner, h));
  return trail ? [...reveals, { kind: 'trail', hex: trail }] : reveals;
}

function stepToward(s, u, target) {
  const r = pathCost(s, u, target, { cost: (_, h) => stepCost(s, h) });
  if (!r || r.path.length < 2) return false;
  const next = r.path[1], c = stepCost(s, next);
  if (c > u.labour) return false;
  u.labour -= c;
  u.q = next.q;
  u.r = next.r;
  return true;
}

// Spends the scout's labour on its route: walk, reveal, build. Clears the route when finished.
export function scoutWork(s, u) {
  while (u.scout?.route && u.labour > 0) {
    const rt = u.scout.route;
    const plan = routePlan(s, u.owner, rt.from, rt.to);
    if (!plan.possible || plan.possible.cost === Infinity) {
      note(s, u.id, 'no route can exist');
      u.scout.route = null;
      return;
    }
    const todo = jobs(s, u, plan);
    if (!todo.length) {
      note(s, u.id, `route finished, cost ${plan.known?.cost}`);
      event(s, `scout ${u.id}: route finished`);
      u.scout.route = null;
      return;
    }
    const job = todo.find((j) => distance(u, j.hex) <= 1) ?? todo.find((j) => stepToward(s, u, j.hex));
    if (!job) return;
    if (distance(u, job.hex) > 1) continue;
    const t = tileAt(s, job.hex);
    if (job.kind === 'reveal') {
      s.civs[u.owner].revealed[key(t)] = true;
      u.labour -= SCOUT.revealLabour;
      continue;
    }
    const need = TRAILS[t.terrain].work;
    const work = Math.min(u.labour, need - (t.trailWork ?? 0));
    t.trailWork = (t.trailWork ?? 0) + work;
    u.labour -= work;
    if (t.trailWork >= need) {
      delete t.trailWork;
      t.trail = true;
      note(s, u.id, `built a trail at ${key(t)}`);
    }
  }
}

// ---- fog ------------------------------------------------------------------------------------

export function updateFog(s) {
  for (const civ of Object.values(s.civs)) {
    civ.seen ??= {};
    const mark = (h, r) => { for (const n of within(h, r)) if (tileAt(s, n)) civ.seen[key(n)] = true; };
    for (const u of civUnits(s, civ.id)) mark(u, SCOUT.sight.unit);
    for (const c of civCities(s, civ.id)) mark(c, SCOUT.sight.city);
    for (const c of civClans(s, civ.id)) mark(c, SCOUT.sight.clan);
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
      if (scoutsOf(s, a.civ).length >= SCOUT.maxPerCiv) return 'too many scouts';
      return null;
    },
    apply(s, a) {
      note(s, spawnScout(s, a.civ, civCities(s, a.civ)[0]).id, 'raised');
    },
    candidates: (s, civId) => (civCities(s, civId).length && scoutsOf(s, civId).length < SCOUT.maxPerCiv
      ? [{ type: 'buildScout', civ: civId }] : []),
  },
  assignRoute: {
    validate(s, a) {
      if (!ownScout(s, a)) return 'not your scout';
      if (!isHex(a.from) || !isHex(a.to) || !tileAt(s, a.from) || !tileAt(s, a.to)) return 'bad route ends';
      return routePlan(s, a.civ, a.from, a.to).possible ? null : 'no route can exist';
    },
    apply(s, a) {
      const u = ownScout(s, a);
      u.scout.route = { from: { q: a.from.q, r: a.from.r }, to: { q: a.to.q, r: a.to.r } };
      note(s, u.id, `route ${key(a.from)} to ${key(a.to)}`);
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

export function ai(s, civId) {
  const civ = s.civs[civId];
  if (!scoutsOf(s, civId).length) return actions.buildScout.candidates(s, civId);
  const out = [];
  for (const u of scoutsOf(s, civId).filter((x) => !x.scout.route)) {
    // Rival markets first, else the nearest unseen land.
    let to = null, best = Infinity;
    for (const m of Object.values(s.markets)) {
      if (m.id === civ.marketId || m.owner === civId || civ.seen?.[key(m)]) continue;
      const d = distance(u, m);
      if (d < best && d <= 25) { best = d; to = m; }
    }
    if (!to) {
      for (const [k, t] of Object.entries(s.map.tiles)) {
        if (civ.seen?.[k] || !TERRAIN[t.terrain].passable) continue;
        const d = distance(u, t);
        if (d < best) { best = d; to = t; }
      }
    }
    if (!to) continue;
    const a = { type: 'assignRoute', civ: civId, unit: u.id, from: { q: u.q, r: u.r }, to: { q: to.q, r: to.r } };
    if (!actions.assignRoute.validate(s, a)) out.push(a);
  }
  return out;
}
