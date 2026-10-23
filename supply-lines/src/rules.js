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
export const depleted = (s, h) => (tileAt(s, h).depletedUntil ?? 0) > s.turn;

export function cityYield(s, city) {
  return within(city, CITY.workRadius)
    .map((h) => tileAt(s, h))
    .filter((t) => t && !depleted(s, t))
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

function memoTerritory(s) {
  const memo = new Map();
  return (h) => (memo.has(key(h)) ? memo.get(key(h)) : memo.set(key(h), territoryOwner(s, h)).get(key(h)));
}

// Supply lines from a city, costed by terrain: Map<key, {d, prev}> for every tile within SUPPLY.range.
// Lines run only through friendly or neutral territory; enemy territory can be the end tile.
function supplyField(s, city, territory) {
  const field = new Map([[key(city), { d: 0, prev: null }]]);
  const buckets = [[city]];
  for (let d = 0; d < buckets.length; d++) {
    for (const cur of buckets[d] ?? []) {
      if (field.get(key(cur)).d !== d) continue;
      if (d > 0 && territory(cur) !== null && territory(cur) !== city.owner) continue;
      for (const n of neighbors(cur)) {
        if (!passable(s, n)) continue;
        const nd = d + TERRAIN[tileAt(s, n).terrain].moveCost;
        if (nd > SUPPLY.range || field.get(key(n))?.d <= nd) continue;
        field.set(key(n), { d: nd, prev: key(cur) });
        (buckets[nd] ??= []).push(n);
      }
    }
  }
  return field;
}

const resupplyLabor = (cost) => 1 + Math.floor(cost / SUPPLY.costPerLabor);

// The unit's supply source: the nearest own city by line cost, preferring one with spare supply.
// {city, cost, labor, line} where line runs city -> unit inclusive; null when no city reaches it.
export function supplySource(s, unit) {
  const territory = memoTerritory(s);
  const [best] = citiesOf(s, unit.owner)
    .map((city) => ({ city, field: supplyField(s, city, territory) }))
    .filter((o) => o.field.has(key(unit)))
    .map((o) => ({ ...o, cost: o.field.get(key(unit)).d }))
    .sort((a, b) => (b.city.spare > 0) - (a.city.spare > 0) || a.cost - b.cost);
  if (!best) return null;
  const line = [];
  for (let k = key(unit); k; k = best.field.get(k).prev) {
    const [q, r] = k.split(',').map(Number);
    line.unshift({ q, r });
  }
  return { city: best.city, cost: best.cost, labor: resupplyLabor(best.cost), line };
}

// Tiles a unit can walk to this turn: Map<key, {hex, rem, prev}>. A unit may always
// spend its last labor on a tile that costs more than it has left.
export function reachable(s, unit) {
  const best = new Map([[key(unit), { hex: { q: unit.q, r: unit.r }, rem: unit.labor, prev: null }]]);
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
  const def = UNITS[type];
  const unit = { id, owner, type, q: hex.q, r: hex.r, labor: def.labor, hp: def.hp };
  if (def.upkeep) unit.food = def.food;
  return (s.units[id] = unit);
}

const event = (s, text) => s.events.push(text);

const strength = (u) => (UNITS[u.type].strength * u.hp) / UNITS[u.type].hp;

function resolveCombat(s, attacker, defender) {
  for (const u of [attacker, defender]) if (u.food !== undefined) u.food = Math.max(0, u.food - SUPPLY.fightFood);
  const att = strength(attacker);
  const tile = TERRAIN[tileAt(s, defender).terrain];
  const def = strength(defender) * (1 + tile.defense + (cityAt(s, defender) ? CITY.defenseBonus : 0));
  const won = rand(s) < att / (att + def);
  const [winner, loser] = won ? [attacker, defender] : [defender, attacker];
  delete s.units[loser.id];
  event(s, `${s.players[winner.owner].name} ${winner.type} defeated ${s.players[loser.owner].name} ${loser.type}`);
  return won;
}

// ---- turn lifecycle ---------------------------------------------------------

// Run in order for the player whose turn is starting. Add systems here.
export const TURN_START = [
  function refreshLabor(s, p) {
    for (const u of unitsOf(s, p)) u.labor = UNITS[u.type].labor;
  },
  function eat(s, p) {
    for (const u of unitsOf(s, p)) {
      const need = UNITS[u.type].upkeep;
      if (!need) continue;
      if (u.food >= need) {
        u.food -= need;
        u.hp = Math.min(UNITS[u.type].hp, u.hp + SUPPLY.healHp);
        continue;
      }
      u.food = 0;
      u.hp -= SUPPLY.attritionHp;
      if (u.hp > 0) continue;
      delete s.units[u.id];
      event(s, `${s.players[p].name} ${u.type} starved`);
    }
  },
  // Supply a city didn't hand out last turn goes to production; then it gets this turn's supply.
  function cityProduction(s, p) {
    for (const c of citiesOf(s, p)) {
      c.stock += c.spare;
      c.spare = cityYield(s, c).prod;
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

function hungryUnit(s, a) {
  const [u, err] = ownUnit(s, a);
  if (err) return [null, err];
  if (u.food === undefined) return [null, 'unit needs no supply'];
  if (u.food >= UNITS[u.type].food) return [null, 'food store is full'];
  return [u, null];
}

// Forage (friendly/neutral land) and pillage (enemy land) take food from the unit's tile,
// which then yields nothing to anyone for SUPPLY.regrowTurns.
function gather(type, { labor, enemyLand, yieldOf }) {
  return {
    validate(s, a) {
      const [u, err] = hungryUnit(s, a);
      if (err) return err;
      if (u.labor < labor) return `needs ${labor} labor`;
      const owner = territoryOwner(s, u);
      if (enemyLand !== (owner !== null && owner !== u.owner)) return enemyLand ? 'not enemy land' : 'enemy land';
      if (depleted(s, u)) return 'tile is stripped';
      if (!yieldOf(TERRAIN[tileAt(s, u).terrain])) return 'nothing to gather';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const tile = tileAt(s, u);
      u.food = Math.min(UNITS[u.type].food, u.food + yieldOf(TERRAIN[tile.terrain]));
      u.labor -= labor;
      tile.depletedUntil = s.turn + SUPPLY.regrowTurns;
      if (enemyLand) event(s, `${s.players[u.owner].name} pillaged ${s.players[territoryOwner(s, u)].name} land`);
    },
    candidates: (s) => unitsOf(s, s.current).map((u) => ({ type, unitId: u.id })),
  };
}

export const ACTIONS = {
  move: {
    validate(s, a) {
      const [u, err] = ownUnit(s, a);
      if (err) return err;
      if (u.labor <= 0) return 'no labor left';
      if (distance(u, a.to) !== 1) return 'not adjacent';
      if (!passable(s, a.to)) return 'impassable';
      const other = unitAt(s, a.to);
      if (other?.owner === u.owner) return 'occupied';
      const city = cityAt(s, a.to);
      if ((other || (city && city.owner !== u.owner)) && !UNITS[u.type].strength) return 'unit cannot attack';
      if (other && u.food !== undefined && u.food < SUPPLY.fightFood) return 'no food to fight';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const enemy = unitAt(s, a.to);
      if (enemy) {
        u.labor = Math.max(0, u.labor - SUPPLY.fightLabor);
        resolveCombat(s, u, enemy);
        return;
      }
      u.labor = Math.max(0, u.labor - TERRAIN[tileAt(s, a.to).terrain].moveCost);
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

  resupply: {
    validate(s, a) {
      const [u, err] = hungryUnit(s, a);
      if (err) return err;
      const src = supplySource(s, u);
      if (!src) return 'no supply line';
      if (src.city.spare <= 0) return 'no spare supply';
      if (u.labor < src.labor) return `needs ${src.labor} labor`;
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unitId];
      const src = supplySource(s, u);
      const amount = Math.min(UNITS[u.type].food - u.food, src.city.spare);
      src.city.spare -= amount;
      u.food += amount;
      u.labor -= src.labor;
    },
    candidates: (s) => unitsOf(s, s.current).map((u) => ({ type: 'resupply', unitId: u.id })),
  },

  forage: gather('forage', { labor: SUPPLY.forageLabor, enemyLand: false, yieldOf: (t) => t.food }),
  pillage: gather('pillage', { labor: SUPPLY.pillageLabor, enemyLand: true, yieldOf: (t) => t.food + t.prod }),

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
      const city = { id, owner: u.owner, name, q: u.q, r: u.r, build: CITY.defaultBuild, stock: 0 };
      city.spare = cityYield(s, city).prod;
      s.cities[id] = city;
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
