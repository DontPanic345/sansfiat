// Shared helpers every module may use. Pure functions of state (plus the in-place mutators
// note/event/spawnUnit/removeUnit, which only touch the clone dispatch hands out).
// This file imports nothing from rules.js, clans.js or src/modules, so any module can import
// it without creating an import cycle.
import { CITY, NOTES, TERRAIN } from './data.js';
import { distance, key, neighbors } from './hex.js';

// ---- ids, events, the "why" log -------------------------------------------------------------

// Every entity (clan, market, civ, city, unit) takes its id from the one counter, so ids are
// unique across kinds and a bare id is enough to name a subject in s.notes.
export const newId = (s) => s.nextId++;

// A line in the turn's event log (shown in the UI log, reset each dispatch).
export const event = (s, text) => s.events.push(`Turn ${s.turn}: ${text}`);

// Records why something happened to an entity. s.notes[id] keeps the last NOTES.perSubject
// reasons, newest last, as {turn, text}. Repeating the latest text only bumps its turn.
// Notes of entities that no longer exist are swept at the end of each turn.
export function note(s, id, text) {
  if (id == null) return;
  const list = (s.notes[id] ??= []);
  const last = list.at(-1);
  if (last && last.text === text) { last.turn = s.turn; return; }
  list.push({ turn: s.turn, text });
  if (list.length > NOTES.perSubject) list.shift();
}

export const notesFor = (s, id) => s.notes[id] ?? [];

// Drops notes whose subject is gone. Core runs this at the end of every turn.
export function sweepNotes(s) {
  for (const id of Object.keys(s.notes))
    if (!(s.clans[id] || s.markets[id] || s.civs[id] || s.cities[id] || s.units[id])) delete s.notes[id];
}

// ---- goods baskets ---------------------------------------------------------------------------

export const total = (basket) => Object.values(basket).reduce((a, b) => a + b, 0);

export function addGoods(a, b) {
  const out = { ...a };
  for (const [g, v] of Object.entries(b)) out[g] = (out[g] ?? 0) + v;
  return out;
}

export const scaleGoods = (basket, k) => Object.fromEntries(Object.entries(basket).map(([g, v]) => [g, v * k]));

// The civ's store is the one pooled stock of goods every module draws on (supply feeds units
// from its food, civ spends it on projects). Use these rather than editing civ.store directly.
// takeGoods takes up to `amount` and returns what it actually took.
export function takeGoods(s, civId, good, amount) {
  const store = s.civs[civId].store;
  const took = Math.max(0, Math.min(amount, store[good] ?? 0));
  store[good] = (store[good] ?? 0) - took;
  if (store[good] <= 1e-9) delete store[good];
  return took;
}

export function putGoods(s, civId, basket) {
  const store = s.civs[civId].store;
  for (const [g, v] of Object.entries(basket)) if (v > 0) store[g] = (store[g] ?? 0) + v;
}

// ---- tiles and movement ----------------------------------------------------------------------

export const tileAt = (s, h) => s.map.tiles[key(h)];

// A tile can be entered if its terrain is passable, or a trail/bridge has opened it (mountains).
export function passable(s, h) {
  const t = tileAt(s, h);
  return !!t && (TERRAIN[t.terrain].passable || !!t.trail);
}

// Labour to enter tile h, as everyone sees it: a built trail lowers the terrain's move cost
// to the tile's trailCost. Infinity if impassable. Hidden trailCost of an unbuilt trail does
// not count here; what a civ *believes* a tile costs is the scout module's business.
export function stepCost(s, h) {
  const t = tileAt(s, h);
  if (!t) return Infinity;
  const base = TERRAIN[t.terrain].passable ? TERRAIN[t.terrain].moveCost : Infinity;
  return t.trail && t.trailCost != null ? Math.min(base, t.trailCost) : base;
}

// Binary heap keyed on .c, used by the path searches below.
function heap() {
  const a = [];
  const up = (i) => {
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p].c <= a[i].c) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  };
  const down = (i) => {
    for (;;) {
      const l = 2 * i + 1, r = l + 1;
      let m = i;
      if (l < a.length && a[l].c < a[m].c) m = l;
      if (r < a.length && a[r].c < a[m].c) m = r;
      if (m === i) break;
      [a[m], a[i]] = [a[i], a[m]];
      i = m;
    }
  };
  return {
    get size() { return a.length; },
    push(x) { a.push(x); up(a.length - 1); },
    pop() {
      const top = a[0], last = a.pop();
      if (a.length) { a[0] = last; down(0); }
      return top;
    },
  };
}

// Cheapest path cost from `from` to every tile within `maxCost`. Returns Map(key -> cost).
// opts.cost(s, h) -> labour to enter h (default stepCost; return Infinity to block).
export function costMap(s, from, maxCost = Infinity, { cost = stepCost } = {}) {
  const best = new Map([[key(from), 0]]);
  const open = heap();
  open.push({ h: from, c: 0 });
  while (open.size) {
    const { h, c } = open.pop();
    if (c > best.get(key(h))) continue;
    for (const n of neighbors(h)) {
      const nc = c + cost(s, n);
      if (nc <= maxCost && nc < (best.get(key(n)) ?? Infinity)) {
        best.set(key(n), nc);
        open.push({ h: n, c: nc });
      }
    }
  }
  return best;
}

// Cheapest path from `from` to `to`: {cost, path: [hex...]} including both ends, or null.
// Same opts as costMap, plus maxCost.
export function pathCost(s, from, to, { cost = stepCost, maxCost = Infinity } = {}) {
  const goal = key(to);
  const best = new Map([[key(from), 0]]);
  const prev = new Map();
  const open = heap();
  open.push({ h: from, c: 0 });
  while (open.size) {
    const { h, c } = open.pop();
    const k = key(h);
    if (c > best.get(k)) continue;
    if (k === goal) {
      const path = [h];
      for (let at = k; prev.has(at); at = key(prev.get(at))) path.unshift(prev.get(at));
      return { cost: c, path };
    }
    for (const n of neighbors(h)) {
      const nc = c + cost(s, n);
      if (nc <= maxCost && nc < (best.get(key(n)) ?? Infinity)) {
        best.set(key(n), nc);
        prev.set(key(n), h);
        open.push({ h: n, c: nc });
      }
    }
  }
  return null;
}

// ---- who is where ----------------------------------------------------------------------------

const at = (h) => (x) => x.q === h.q && x.r === h.r;
export const clanAt = (s, h) => Object.values(s.clans).find(at(h));
export const marketAt = (s, h) => Object.values(s.markets).find(at(h));
export const cityAt = (s, h) => Object.values(s.cities).find(at(h));
export const unitsAt = (s, h) => Object.values(s.units).filter(at(h));
// The clan whose land the tile is (tile.claim), if any.
export const holder = (s, h) => s.clans[tileAt(s, h)?.claim];

export const totalPop = (s) => Object.values(s.clans).reduce((a, c) => a + c.pop, 0);

// The civ an entity belongs to, or null. Takes a clan, unit, city or market object, or any
// entity id. Clans, units and cities carry `owner`; markets carry `owner` too (founding markets).
export function civOf(s, x) {
  const e = typeof x === 'object' && x !== null ? x
    : s.clans[x] ?? s.units[x] ?? s.cities[x] ?? s.markets[x] ?? s.civs[x];
  if (!e) return null;
  if (s.civs[e.id] === e) return e;
  return e.owner != null ? s.civs[e.owner] ?? null : null;
}

// Who holds a tile in the main game: troops standing on it (tile.held, kept by the army
// module), else a fort's owner, else the civ whose city is within CITY.territoryRadius
// (nearest city wins; on a tie, the lower city id). null is neutral. Supply lines and forage/pillage use this.
export function territoryOf(s, h) {
  const t = tileAt(s, h);
  if (!t) return null;
  if (t.held != null) return t.held;
  if (t.fort?.owner != null) return t.fort.owner;
  let best = null;
  for (const c of Object.values(s.cities)) {
    const d = distance(c, h);
    if (d <= CITY.territoryRadius && (!best || d < best.d || (d === best.d && c.id < best.id))) best = { d, id: c.id, owner: c.owner };
  }
  return best ? best.owner : null;
}

// Moves a city to another civ (capture), keeping both civs' cityIds in step.
export function transferCity(s, cityId, civId) {
  const city = s.cities[cityId];
  const from = s.civs[city.owner];
  if (from) from.cityIds = from.cityIds.filter((id) => id !== cityId);
  city.owner = civId;
  if (civId != null && !s.civs[civId].cityIds.includes(cityId)) s.civs[civId].cityIds.push(cityId);
}

export const civClans = (s, civId) => Object.values(s.clans).filter((c) => c.owner === civId);
export const civCities = (s, civId) => Object.values(s.cities).filter((c) => c.owner === civId);
export const civUnits = (s, civId) => Object.values(s.units).filter((u) => u.owner === civId);
export const humanCiv = (s) => Object.values(s.civs).find((c) => c.isHuman) ?? null;

// ---- units -----------------------------------------------------------------------------------

// Every unit has these core fields; modules add their own under a namespace named for the
// module (u.army, u.supply, u.scout) so two modules never fight over a field.
// labour: what is left this turn; core refills it to labourMax at the end of every turn.
export function spawnUnit(s, { owner, type, q, r, labourMax = 4, ...rest }) {
  const id = newId(s);
  s.units[id] = { id, owner, type, q, r, labour: labourMax, labourMax, ...rest };
  return s.units[id];
}

export function removeUnit(s, id) {
  delete s.units[id];
}
