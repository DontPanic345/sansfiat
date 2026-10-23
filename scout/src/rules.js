import { CITY, SCOUT, SUPPLY, TERRAIN, TRAILS, UNITS } from './data.js';
import { distance, eq, key, neighbors, within } from './hex.js';
import { rand } from './rng.js';

// ---- queries ----------------------------------------------------------------

export const tileAt = (s, h) => s.map.tiles[key(h)];
export const unitsAt = (s, h) => Object.values(s.units).filter((u) => eq(u, h));
// The strongest unit on h that isn't owner's: who blocks owner's units and defends against them.
export const defenderAt = (s, h, owner) =>
  unitsAt(s, h).filter((u) => u.owner !== owner).sort((a, b) => UNITS[b.type].strength - UNITS[a.type].strength)[0];
export const unitName = (s, u) => `${s.players[u.owner].name} ${u.name}`;
export const cityAt = (s, h) => Object.values(s.cities).find((c) => eq(c, h));
export const passable = (s, h) => !!tileAt(s, h) && (TERRAIN[tileAt(s, h).terrain].passable || !!tileAt(s, h).trail);
export const moveCost = (s, h) => (tileAt(s, h).trail ? tileAt(s, h).trailCost : TERRAIN[tileAt(s, h).terrain].moveCost);
export const unitsOf = (s, owner) => Object.values(s.units).filter((u) => u.owner === owner);
export const citiesOf = (s, owner) => Object.values(s.cities).filter((c) => c.owner === owner);

export function cityYield(s, city) {
  return within(city, CITY.workRadius)
    .map((h) => tileAt(s, h))
    .filter(Boolean)
    .reduce((y, t) => ({ food: y.food + TERRAIN[t.terrain].food, prod: y.prod + TERRAIN[t.terrain].prod }), { food: 0, prod: 0 });
}

// Owner of the nearest city within territoryRadius; null when none or when two owners tie.
export function territoryOwner(s, h) {
  const near = Object.values(s.cities)
    .map((c) => ({ owner: c.owner, d: distance(c, h) }))
    .filter((x) => x.d <= CITY.territoryRadius);
  if (!near.length) return null;
  const d = Math.min(...near.map((x) => x.d));
  const owners = new Set(near.filter((x) => x.d === d).map((x) => x.owner));
  return owners.size === 1 ? [...owners][0] : null;
}

// Supply lines from a city to every tile within SUPPLY.range: Map<key, {d, prev}>.
// Lines run only through friendly or neutral territory; enemy territory can be the end tile.
function supplyDistances(s, city, territory) {
  const dist = new Map([[key(city), { d: 0, prev: null }]]);
  const frontier = [city];
  while (frontier.length) {
    const cur = frontier.shift();
    const { d } = dist.get(key(cur));
    if (d >= SUPPLY.range) continue;
    for (const n of neighbors(cur)) {
      const k = key(n);
      if (dist.has(k) || !passable(s, n)) continue;
      dist.set(k, { d: d + 1, prev: key(cur) });
      if (territory(n) === null || territory(n) === city.owner) frontier.push(n);
    }
  }
  return dist;
}

// Tiles a unit can walk to this turn: Map<key, {hex, rem, prev}>. A unit may always
// spend its last move points on a tile that costs more than it has left.
export function reachable(s, unit) {
  const best = new Map([[key(unit), { hex: { q: unit.q, r: unit.r }, rem: unit.movesLeft, prev: null }]]);
  const frontier = [key(unit)];
  while (frontier.length) {
    const cur = best.get(frontier.shift());
    if (cur.rem <= 0) continue;
    for (const n of neighbors(cur.hex)) {
      if (!passable(s, n) || defenderAt(s, n, unit.owner)) continue;
      const city = cityAt(s, n);
      if (city && city.owner !== unit.owner && !UNITS[unit.type].strength) continue;
      const rem = Math.max(0, cur.rem - moveCost(s, n));
      const k = key(n);
      if (best.has(k) && best.get(k).rem >= rem) continue;
      best.set(k, { hex: n, rem, prev: key(cur.hex) });
      frontier.push(k);
    }
  }
  best.delete(key(unit));
  return best;
}

export function pathTo(reach, hex) {
  const path = [];
  for (let k = key(hex); reach.has(k); k = reach.get(k).prev) path.unshift(reach.get(k).hex);
  return path;
}

// A* from `from` to any hex within `reach` of `to`; entering h costs cost(h), Infinity blocks.
// Returns {path (including from), cost}; cost is Infinity when there is no path.
export function cheapestPath(s, from, to, cost, reach = 0) {
  const g = new Map([[key(from), 0]]);
  const prev = new Map();
  const est = (h) => g.get(key(h)) + Math.max(0, distance(h, to) - reach);
  const open = [from];
  const queued = new Set([key(from)]);
  while (open.length) {
    const i = open.reduce((best, h, j) => (est(h) < est(open[best]) ? j : best), 0);
    const cur = open.splice(i, 1)[0];
    queued.delete(key(cur));
    if (distance(cur, to) <= reach) {
      const path = [cur];
      for (let k = key(cur); prev.has(k); k = key(path[0])) path.unshift(prev.get(k));
      return { path, cost: g.get(key(cur)) };
    }
    for (const n of neighbors(cur)) {
      if (!tileAt(s, n)) continue;
      const d = g.get(key(cur)) + cost(n);
      if (!(d < (g.get(key(n)) ?? Infinity))) continue;
      g.set(key(n), d);
      prev.set(key(n), cur);
      if (!queued.has(key(n))) open.push(n), queued.add(key(n));
    }
  }
  return { path: [], cost: Infinity };
}

// ---- mutations (only call on a cloned state, i.e. from inside an action) ----

export function spawnUnit(s, owner, type, hex) {
  const id = s.nextId++;
  const built = s.players[owner].built;
  const n = (built[type] = (built[type] ?? 0) + 1);
  const name = `${type[0].toUpperCase()}${type.slice(1)} ${n}`;
  return (s.units[id] = { id, owner, type, name, n, q: hex.q, r: hex.r, movesLeft: UNITS[type].moves });
}

const event = (s, text) => s.events.push(text);

function resolveCombat(s, attacker, defender) {
  const att = UNITS[attacker.type].strength;
  const tile = TERRAIN[tileAt(s, defender).terrain];
  const def = UNITS[defender.type].strength * (1 + tile.defense + (cityAt(s, defender) ? CITY.defenseBonus : 0));
  const won = rand(s) < att / (att + def);
  const [winner, loser] = won ? [attacker, defender] : [defender, attacker];
  delete s.units[loser.id];
  event(s, `${unitName(s, winner)} defeated ${unitName(s, loser)}`);
  return won;
}

// ---- scouting ----------------------------------------------------------------

const trailable = (s, p, t) => t.trailCost != null && !(t.span > s.players[p].bridgeSpan);

// What player p knows about a tile's cost once any Trail is built: {best, worst}.
// Unrevealed tiles range from the cheapest Trail they could hide to their bare terrain cost.
export function plannedCost(s, p, h) {
  const t = tileAt(s, h);
  const base = TERRAIN[t.terrain].passable ? TERRAIN[t.terrain].moveCost : Infinity;
  const spec = TRAILS[t.terrain];
  if (t.trail) return { best: t.trailCost, worst: t.trailCost };
  if (!spec) return { best: base, worst: base };
  if (s.players[p].revealed[key(h)]) {
    const c = trailable(s, p, t) ? Math.min(base, t.trailCost) : base;
    return { best: c, worst: c };
  }
  const canSpan = !spec.span || spec.span.some(([span]) => span != null && span <= s.players[p].bridgeSpan);
  const best = canSpan ? Math.min(base, ...spec.cost.map(([c]) => c)) : base;
  return { best, worst: base };
}

// known: the best route player p can be sure of. possible: the best route the unrevealed tiles allow.
// unknown: tiles on the possible route whose reveal could still change the answer.
export function routePlan(s, p, from, to) {
  const known = cheapestPath(s, from, to, (h) => plannedCost(s, p, h).worst);
  const possible = cheapestPath(s, from, to, (h) => plannedCost(s, p, h).best);
  const unknown = known.cost > possible.cost
    ? possible.path.slice(1).filter((h) => { const c = plannedCost(s, p, h); return c.best < c.worst; })
    : [];
  return { known, possible, unknown };
}

function needsTrail(s, p, h) {
  const t = tileAt(s, h);
  if (t.trail || !TRAILS[t.terrain] || !s.players[p].revealed[key(h)] || !trailable(s, p, t)) return false;
  return !TERRAIN[t.terrain].passable || t.trailCost < TERRAIN[t.terrain].moveCost;
}

// Reveals come first: impassable tiles (they decide whether a route exists), then the biggest
// possible saving. Trails are built once the route is settled, or when no reveal can be reached.
function jobs(s, u, plan) {
  const reveals = plan.unknown
    .map((h) => ({ kind: 'reveal', hex: h, c: plannedCost(s, u.owner, h), d: distance(u, h) }))
    .sort((a, b) => (a.c.worst === Infinity ? 0 : 1) - (b.c.worst === Infinity ? 0 : 1)
      || (a.c.best - a.c.worst) - (b.c.best - b.c.worst) || a.d - b.d);
  const route = plan.unknown.length ? plan.possible.path : plan.known.path;
  const trail = route.slice(1).find((h) => needsTrail(s, u.owner, h));
  return trail ? [...reveals, { kind: 'trail', hex: trail }] : reveals;
}

function stepToward(s, u, target) {
  const cost = (h) => {
    const city = cityAt(s, h);
    return passable(s, h) && !defenderAt(s, h, u.owner) && !(city && city.owner !== u.owner) ? moveCost(s, h) : Infinity;
  };
  const { path } = cheapestPath(s, u, target, cost, 1);
  if (path.length < 2) return false;
  u.movesLeft = Math.max(0, u.movesLeft - moveCost(s, path[1]));
  u.q = path[1].q;
  u.r = path[1].r;
  return true;
}

// Spends the scout's labor on its route: walk, reveal, then build. Clears the route when finished.
function scoutWork(s, u) {
  const who = unitName(s, u);
  while (u.route && u.movesLeft > 0) {
    const plan = routePlan(s, u.owner, u.route.from, u.route.to);
    if (plan.possible.cost === Infinity) {
      event(s, `${who}: no route can exist`);
      delete u.route;
      return;
    }
    const todo = jobs(s, u, plan);
    if (!todo.length) {
      event(s, `${who}: route finished, cost ${plan.known.cost}`);
      delete u.route;
      return;
    }
    const job = todo.find((j) => distance(u, j.hex) <= 1 || stepToward(s, u, j.hex));
    if (!job) return event(s, `${who} is blocked`);
    if (distance(u, job.hex) > 1) continue;
    const t = tileAt(s, job.hex);
    if (job.kind === 'reveal') {
      s.players[u.owner].revealed[key(t)] = true;
      u.movesLeft -= SCOUT.revealLabor;
      if (!TERRAIN[t.terrain].passable && trailable(s, u.owner, t)) event(s, `${who} found a pass at ${key(t)}`);
      continue;
    }
    const work = Math.min(u.movesLeft, TRAILS[t.terrain].work - (t.trailWork ?? 0));
    t.trailWork = (t.trailWork ?? 0) + work;
    u.movesLeft -= work;
    if (t.trailWork >= TRAILS[t.terrain].work) {
      delete t.trailWork;
      t.trail = true;
    }
  }
}

// ---- turn lifecycle ---------------------------------------------------------

// Run in order for the player whose turn is starting. Add systems here.
export const TURN_START = [
  function refreshMoves(s, p) {
    for (const u of unitsOf(s, p)) u.movesLeft = UNITS[u.type].moves;
  },
  allocateSupply,
  function scouting(s, p) {
    for (const u of unitsOf(s, p)) if (u.route) scoutWork(s, u);
  },
  function cityProduction(s, p) {
    for (const c of citiesOf(s, p)) {
      c.stock += c.spare;
      const cost = UNITS[c.build].cost;
      if (c.stock < cost) continue;
      c.stock -= cost;
      event(s, `${c.name} built ${spawnUnit(s, p, c.build, c).name}`);
    }
  },
];

function memoTerritory(s) {
  const memo = new Map();
  return (h) => (memo.has(key(h)) ? memo.get(key(h)) : memo.set(key(h), territoryOwner(s, h)).get(key(h)));
}

// Hexes from the unit's supplying city to the unit, inclusive; [] when unsupplied.
export function supplyLine(s, unit) {
  const city = s.cities[unit.supplier];
  if (!city || city.owner !== unit.owner) return [];
  const field = supplyDistances(s, city, memoTerritory(s));
  const line = [];
  for (let k = key(unit); k && field.has(k); k = field.get(k).prev) {
    const [q, r] = k.split(',').map(Number);
    line.unshift({ q, r });
  }
  return line;
}

// Each upkeep unit draws from the nearest city (by supply line) with spare supply, oldest unit
// first. A unit's labor is its full pool minus the cost of its line; unsupplied units get little.
// Whatever a city has left goes to production.
function allocateSupply(s, p) {
  const cities = citiesOf(s, p);
  const territory = memoTerritory(s);
  const lines = new Map(cities.map((c) => [c.id, supplyDistances(s, c, territory)]));
  for (const c of cities) c.spare = cityYield(s, c).prod;
  for (const u of unitsOf(s, p)) {
    const need = UNITS[u.type].upkeep;
    if (!need) continue;
    const [best] = cities
      .map((c) => ({ c, d: lines.get(c.id).get(key(u))?.d }))
      .filter((o) => o.d !== undefined && o.c.spare >= need)
      .sort((a, b) => a.d - b.d);
    if (!best) {
      u.supplier = null;
      u.movesLeft = Math.min(u.movesLeft, SUPPLY.unsuppliedLabor);
      continue;
    }
    best.c.spare -= need;
    u.supplier = best.c.id;
    u.movesLeft = Math.max(0, u.movesLeft - Math.floor(best.d / SUPPLY.tilesPerLabor));
  }
}

export function startTurn(s) {
  for (const system of TURN_START) system(s, s.current);
}

function checkElimination(s) {
  for (const p of s.players) {
    if (p.alive && !unitsOf(s, p.id).length && !citiesOf(s, p.id).length) {
      p.alive = false;
      event(s, `${p.name} eliminated`);
    }
  }
  const alive = s.players.filter((p) => p.alive);
  if (alive.length === 1) {
    s.winner = alive[0].id;
    event(s, `${alive[0].name} wins`);
  }
}

// ---- actions ----------------------------------------------------------------
// validate(s, a) -> error string | null. apply(s, a) mutates a clone.
// candidates(s) -> raw actions to try; legalActions filters them through validate.

function ownUnit(s, a) {
  const u = s.units[a.unitId];
  if (!u) return [null, 'no such unit'];
  if (u.owner !== s.current) return [null, 'not your unit'];
  return [u, null];
}

export const ACTIONS = {
  move: {
    validate(s, a) {
      const [u, err] = ownUnit(s, a);
      if (err) return err;
      if (u.movesLeft <= 0) return 'no moves left';
      if (distance(u, a.to) !== 1) return 'not adjacent';
      if (!passable(s, a.to)) return 'impassable';
      const enemy = defenderAt(s, a.to, u.owner);
      const city = cityAt(s, a.to);
      if ((enemy || (city && city.owner !== u.owner)) && !UNITS[u.type].strength) return 'unit cannot attack';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const enemy = defenderAt(s, a.to, u.owner);
      if (enemy) {
        if (resolveCombat(s, u, enemy)) u.movesLeft = 0;
        return;
      }
      u.movesLeft = Math.max(0, u.movesLeft - moveCost(s, a.to));
      u.q = a.to.q;
      u.r = a.to.r;
      const city = cityAt(s, u);
      if (city && city.owner !== u.owner) {
        event(s, `${s.players[u.owner].name} captured ${city.name}`);
        city.owner = u.owner;
      }
    },
    candidates: (s) =>
      unitsOf(s, s.current).flatMap((u) => neighbors(u).map((to) => ({ type: 'move', unitId: u.id, to }))),
  },

  foundCity: {
    validate(s, a) {
      const [u, err] = ownUnit(s, a);
      if (err) return err;
      if (!UNITS[u.type].canFound) return 'unit cannot found cities';
      if (Object.values(s.cities).some((c) => distance(c, u) < CITY.minSpacing)) return 'too close to another city';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const id = s.nextId++;
      const name = `${s.players[u.owner].name} ${citiesOf(s, u.owner).length + 1}`;
      s.cities[id] = { id, owner: u.owner, name, q: u.q, r: u.r, build: CITY.defaultBuild, stock: 0 };
      delete s.units[u.id];
      event(s, `${name} founded`);
    },
    candidates: (s) => unitsOf(s, s.current).map((u) => ({ type: 'foundCity', unitId: u.id })),
  },

  assignRoute: {
    validate(s, a) {
      const [u, err] = ownUnit(s, a);
      if (err) return err;
      if (!UNITS[u.type].canScout) return 'unit cannot scout';
      if (!tileAt(s, a.from) || !tileAt(s, a.to)) return 'off the map';
      if (eq(a.from, a.to)) return 'start and target are the same';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      u.route = { from: { q: a.from.q, r: a.from.r }, to: { q: a.to.q, r: a.to.r } };
      scoutWork(s, u);
    },
    candidates: () => [],
  },

  setBuild: {
    validate(s, a) {
      const c = s.cities[a.cityId];
      if (!c) return 'no such city';
      if (c.owner !== s.current) return 'not your city';
      if (!UNITS[a.build]) return 'unknown unit type';
      return null;
    },
    apply(s, a) {
      s.cities[a.cityId].build = a.build;
    },
    candidates: (s) =>
      citiesOf(s, s.current).flatMap((c) =>
        Object.keys(UNITS).filter((b) => b !== c.build).map((build) => ({ type: 'setBuild', cityId: c.id, build }))),
  },

  endTurn: {
    validate: () => null,
    apply(s) {
      let next = s.current;
      do {
        next = (next + 1) % s.players.length;
        if (next === 0) s.turn++;
      } while (!s.players[next].alive);
      s.current = next;
      startTurn(s);
    },
    candidates: () => [{ type: 'endTurn' }],
  },
};

// ---- entry points -------------------------------------------------------------

export function validate(s, a) {
  if (s.winner != null) return 'game over';
  const def = ACTIONS[a.type];
  return def ? def.validate(s, a) : `unknown action ${a.type}`;
}

// Pure: returns {state, error?}. Never mutates the input state.
export function dispatch(s, a) {
  const error = validate(s, a);
  if (error) return { state: s, error };
  const next = structuredClone(s);
  next.events = [];
  ACTIONS[a.type].apply(next, a);
  checkElimination(next);
  return { state: next };
}

export const legalActions = (s) =>
  s.winner != null ? [] : Object.values(ACTIONS).flatMap((d) => d.candidates(s)).filter((a) => !validate(s, a));
