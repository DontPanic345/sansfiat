import { CITY, SUPPLY, TERRAIN, UNITS } from './data.js';
import { distance, eq, key, neighbors, within } from './hex.js';
import { rand } from './rng.js';

// ---- queries ----------------------------------------------------------------

export const tileAt = (s, h) => s.map.tiles[key(h)];
export const unitAt = (s, h) => Object.values(s.units).find((u) => eq(u, h));
export const cityAt = (s, h) => Object.values(s.cities).find((c) => eq(c, h));
export const passable = (s, h) => !!tileAt(s, h) && TERRAIN[tileAt(s, h).terrain].passable;
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
      if (!passable(s, n) || unitAt(s, n)) continue;
      const city = cityAt(s, n);
      if (city && city.owner !== unit.owner && !UNITS[unit.type].strength) continue;
      const rem = Math.max(0, cur.rem - TERRAIN[tileAt(s, n).terrain].moveCost);
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

// ---- mutations (only call on a cloned state, i.e. from inside an action) ----

export function spawnUnit(s, owner, type, hex) {
  const id = s.nextId++;
  return (s.units[id] = { id, owner, type, q: hex.q, r: hex.r, movesLeft: UNITS[type].moves });
}

const event = (s, text) => s.events.push(text);

function resolveCombat(s, attacker, defender) {
  const att = UNITS[attacker.type].strength;
  const tile = TERRAIN[tileAt(s, defender).terrain];
  const def = UNITS[defender.type].strength * (1 + tile.defense + (cityAt(s, defender) ? CITY.defenseBonus : 0));
  const won = rand(s) < att / (att + def);
  const [winner, loser] = won ? [attacker, defender] : [defender, attacker];
  delete s.units[loser.id];
  event(s, `${s.players[winner.owner].name} ${winner.type} defeated ${s.players[loser.owner].name} ${loser.type}`);
  return won;
}

// ---- turn lifecycle ---------------------------------------------------------

// Run in order for the player whose turn is starting. Add systems here.
export const TURN_START = [
  function refreshMoves(s, p) {
    for (const u of unitsOf(s, p)) u.movesLeft = UNITS[u.type].moves;
  },
  allocateSupply,
  function cityProduction(s, p) {
    for (const c of citiesOf(s, p)) {
      c.stock += c.spare;
      const cost = UNITS[c.build].cost;
      if (c.stock < cost) continue;
      const spot = [c, ...neighbors(c)].find((h) => passable(s, h) && !unitAt(s, h));
      if (!spot) continue;
      c.stock -= cost;
      spawnUnit(s, p, c.build, spot);
      event(s, `${c.name} built ${c.build}`);
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
      const other = unitAt(s, a.to);
      if (other?.owner === u.owner) return 'occupied';
      const city = cityAt(s, a.to);
      if ((other || (city && city.owner !== u.owner)) && !UNITS[u.type].strength) return 'unit cannot attack';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const enemy = unitAt(s, a.to);
      if (enemy) {
        if (resolveCombat(s, u, enemy)) u.movesLeft = 0;
        return;
      }
      u.movesLeft = Math.max(0, u.movesLeft - TERRAIN[tileAt(s, a.to).terrain].moveCost);
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
