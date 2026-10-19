// The army module: captains, zones, detachments, skirmish, entrenchment, forts, capture.
// Captain: unit type 'captain', u.army = {troops, zone:[keys], foughtTurn?, foughtTroops?}. Deployed
// (zone non-empty) => troops live in detachments (units type 'detachment', u.army = {captainId, troops,
// entr, cutSince?}). Undeployed => no detachments, all troops ride on the captain.
// Hungry: u.hungry or u.supply.hungry (supply owns it). A hungry army cannot attack; it defends at
// ARMY.hungryPower. Moving costs labour by terrain (stepCost). Fighting costs food: the army records
// foughtTurn/foughtTroops on the captain and supply charges for it.
import { CITY, FOODS, TERRAIN } from '../../data.js';
import { note, event, tileAt, passable, stepCost, pathCost, cityAt, unitsAt, civCities, civClans, takeGoods, transferCity, spawnUnit, removeUnit } from '../../core.js';
import { key, neighbors, distance } from '../../hex.js';
import { rand } from '../../rng.js';
import { ARMY } from './data.js';

export const name = 'army';
export function init(s) { s.mods.army = { fights: 0, captures: 0, raised: {}, raisedTurn: {}, peak: {} }; }
export function onStart(s) { s.mods.army.startTurn = s.turn; }

const parse = (k) => { const [q, r] = k.split(',').map(Number); return { q, r }; };
export const captains = (s, civ) => Object.values(s.units).filter((u) => u.type === 'captain' && (civ == null || u.owner === civ));
export const dets = (s, cid) => Object.values(s.units).filter((u) => u.type === 'detachment' && u.army.captainId === cid);
const capOf = (s, u) => (u.type === 'captain' ? u : s.units[u.army?.captainId]);
export const hungry = (u, s) => {
  const c = s ? capOf(s, u) : null;
  return !!(u.hungry || u.supply?.hungry || c?.hungry || c?.supply?.hungry);
};
// Units that fight on their tile: detachments, and undeployed captains with troops.
export const fighting = (u) => u.type === 'detachment' || (u.type === 'captain' && !u.army.zone.length && u.army.troops > 0);
const uid = (a) => a.unit ?? a.unitId;
const cid = (a) => a.city ?? a.cityId;
const civName = (s, c) => s.civs[c]?.name ?? `civ ${c}`;
const alert = (s, civ, text) => { if (s.civs[civ]?.isHuman) event(s, `! ${text}`); };
const storeFood = (s, civ) => FOODS.reduce((a, f) => a + (s.civs[civ]?.store[f] ?? 0), 0);
function takeFood(s, civ, n) {
  let got = 0;
  for (const f of FOODS) {
    const t = Math.min(n - got, s.civs[civ].store[f] ?? 0);
    if (t > 0) { takeGoods(s, civ, f, t); got += t; }
  }
  return got;
}

// Entering h would attack: an enemy fort, an enemy city (its garrison), or enemy troops.
export function hostile(s, h, owner) {
  const t = tileAt(s, h);
  if (!t) return false;
  if (t.fort && t.fort.owner !== owner) return true;
  const c = cityAt(s, h);
  if (c && c.owner != null && c.owner !== owner) return true;
  return unitsAt(s, h).some((u) => u.owner !== owner && fighting(u));
}

// Tiles reachable from the captain's tile through zone tiles.
export function linked(s, cap) {
  const zone = new Set(cap.army.zone);
  const seen = new Set([key(cap)]);
  const queue = [{ q: cap.q, r: cap.r }];
  while (queue.length) {
    const h = queue.pop();
    for (const n of neighbors(h)) {
      const k = key(n);
      if (zone.has(k) && !seen.has(k)) { seen.add(k); queue.push(n); }
    }
  }
  return seen;
}
function zoneConnected(keys) {
  if (keys.length <= 1) return true;
  const set = new Set(keys), seen = new Set([keys[0]]), queue = [parse(keys[0])];
  while (queue.length) for (const n of neighbors(queue.pop())) {
    const k = key(n);
    if (set.has(k) && !seen.has(k)) { seen.add(k); queue.push(n); }
  }
  return seen.size === keys.length;
}
const syncTroops = (s, cap) => { if (cap.army.zone.length) cap.army.troops = dets(s, cap.id).reduce((a, d) => a + d.army.troops, 0); };
function deploy(s, cap) {
  if (dets(s, cap.id).length || !cap.army.troops) return;
  spawnUnit(s, { owner: cap.owner, type: 'detachment', q: cap.q, r: cap.r, army: { captainId: cap.id, troops: cap.army.troops, entr: 0 } });
}
function undeploy(s, cap) {
  const ds = dets(s, cap.id);
  if (ds.length) cap.army.troops = ds.reduce((a, d) => a + d.army.troops, 0);
  for (const d of ds) removeUnit(s, d.id);
  cap.army.zone = [];
}
const ownCaptain = (s, a) => { const u = s.units[uid(a)]; return u && u.type === 'captain' && u.owner === a.civ ? u : null; };
export const orphan = (s, d) => d.type === 'detachment' && s.units[d.army.captainId]?.type !== 'captain';
// Cut off: its captain is gone, or its tile is no longer linked to him through the zone.
export const cutOff = (s, d) => orphan(s, d) || !linked(s, s.units[d.army.captainId]).has(key(d));

function clanPop(s, civ) { return civClans(s, civ).reduce((t, c) => t + c.pop, 0); }
const nearestCity = (s, civ, h) => civCities(s, civ).sort((a, b) => distance(a, h) - distance(b, h) || a.id - b.id)[0] ?? null;

// Troops go home: they rejoin the nearest clan of their civ (lost if it has none).
function returnToClans(s, owner, at, n, why) {
  const clan = civClans(s, owner).sort((a, b) => distance(a, at) - distance(b, at) || a.id - b.id)[0];
  if (clan && n > 0) { clan.pop += n; note(s, clan.id, `${n} troops came home (${why})`); }
}

export const actions = {
  raiseArmy: {
    validate(s, a) {
      const city = s.cities[cid(a)];
      if (!city || city.owner !== a.civ) return 'not your city';
      if (captains(s, a.civ).length >= ARMY.maxCaptains) return `at most ${ARMY.maxCaptains} captains`;
      if (clanPop(s, a.civ) < ARMY.size + 5) return 'not enough people to raise';
      return null;
    },
    apply(s, a) {
      const city = s.cities[cid(a)];
      let need = ARMY.size;
      for (const c of civClans(s, a.civ).sort((x, y) => y.pop - x.pop || x.id - y.id)) {
        const take = Math.min(need, Math.max(0, Math.floor(c.pop - 1)));
        c.pop -= take; need -= take;
        if (take) note(s, c.id, `${take} people raised as troops for ${civName(s, a.civ)}`);
        if (!need) break;
      }
      const troops = ARMY.size - need;
      const food = takeFood(s, a.civ, troops * ARMY.provision); // provisions only from what the store holds
      const cap = spawnUnit(s, { owner: a.civ, type: 'captain', q: city.q, r: city.r, army: { troops, troopsMax: troops, zone: [] }, supply: { food, foodMax: Math.max(food, 1), auto: false } });
      const m = s.mods.army;
      m.raised[a.civ] = (m.raised[a.civ] ?? 0) + 1;
      m.raisedTurn[a.civ] = s.turn;
      note(s, cap.id, `raised at ${city.name ?? city.id} from the civ's clans with ${food.toFixed(1)} food`);
      event(s, `${civName(s, a.civ)} raises an army`);
      // word reaches every human civ with a city within raiseWarn of the raising city
      for (const c of Object.values(s.civs)) {
        if (c.id === a.civ || !c.isHuman) continue;
        const near = civCities(s, c.id).sort((x, y) => distance(x, city) - distance(y, city) || x.id - y.id)[0];
        if (near && distance(near, city) <= ARMY.raiseWarn)
          alert(s, c.id, `${civName(s, a.civ)} raises an army of ${troops} at ${city.name ?? 'a city'} (${city.q},${city.r}), ${distance(near, city)} tiles from ${near.name ?? 'your city'}`);
      }
    },
    candidates: (s, civ) => civCities(s, civ).map((c) => ({ type: 'raiseArmy', civ, city: c.id })).slice(0, 1),
  },
  moveCaptain: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      if (distance(u, a) !== 1) return 'one tile at a time';
      if (!tileAt(s, a) || !passable(s, a)) return 'impassable';
      const c = stepCost(s, a);
      if (c === Infinity) return 'impassable';
      if (u.labour <= 0 || (u.labour < c && u.labour < u.labourMax)) return `needs ${c} labour (has ${u.labour})`;
      if (fighting(u) && hostile(s, a, u.owner) && hungry(u, s)) return 'a hungry army cannot attack';
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      const from = key(u);
      u.labour = Math.max(0, u.labour - stepCost(s, a));
      u.q = a.q; u.r = a.r;
      const k = key(a), z = u.army.zone;
      if (z.length && !z.includes(k) && z.includes(from)) z.push(k); // stepping out paints
    },
    candidates: () => [],
  },
  paint: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      if (!tileAt(s, a) || !passable(s, a)) return 'cannot paint there';
      const z = u.army.zone, k = key(a);
      if (z.includes(k)) return 'already painted';
      if (z.length ? !neighbors(a).some((n) => z.includes(key(n))) : distance(u, a) > 1) return 'zone must stay connected';
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      if (!u.army.zone.length) { u.army.zone.push(key(u)); if (key(u) !== key(a)) u.army.zone.push(key(a)); deploy(s, u); }
      else u.army.zone.push(key(a));
    },
    candidates: () => [],
  },
  unpaint: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      const z = u.army.zone, k = key(a);
      if (!z.includes(k)) return 'not painted';
      if (!zoneConnected(z.filter((x) => x !== k))) return 'zone must stay connected';
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      u.army.zone = u.army.zone.filter((x) => x !== key(a));
      if (!u.army.zone.length) undeploy(s, u);
    },
    candidates: () => [],
  },
  undeploy: {
    validate: (s, a) => { const u = ownCaptain(s, a); return !u ? 'not your captain' : u.army.zone.length ? null : 'not deployed'; },
    apply(s, a) { undeploy(s, s.units[uid(a)]); },
    candidates: () => [],
  },
  // Send the army home: its troops rejoin the civ's clans. Only near one of the civ's cities.
  disband: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      const c = nearestCity(s, a.civ, u);
      if (!c || distance(c, u) > CITY.territoryRadius) return `must be within ${CITY.territoryRadius} of your city`;
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      undeploy(s, u);
      returnToClans(s, u.owner, u, u.army.troops, 'army disbanded');
      removeUnit(s, u.id);
      event(s, `${civName(s, a.civ)} disbands an army`);
    },
    candidates: () => [],
  },
  promote: {
    validate(s, a) {
      const d = s.units[uid(a)];
      if (!d || d.type !== 'detachment' || d.owner !== a.civ) return 'not your detachment';
      if (!cutOff(s, d)) return 'still linked to its captain';
      if (captains(s, a.civ).length >= ARMY.maxCaptains) return `at most ${ARMY.maxCaptains} captains; it walks back or disbands`;
      return null;
    },
    apply(s, a) {
      const d = s.units[uid(a)], old = orphan(s, d) ? null : s.units[d.army.captainId];
      const set = new Set(old ? old.army.zone : [key(d)]), comp = new Set([key(d)]), queue = [{ q: d.q, r: d.r }];
      while (queue.length) for (const n of neighbors(queue.pop())) {
        const k = key(n);
        if (set.has(k) && !comp.has(k)) { comp.add(k); queue.push(n); }
      }
      const before = old ? old.army.troops : 0;
      const cap = spawnUnit(s, { owner: d.owner, type: 'captain', q: d.q, r: d.r, army: { troops: d.army.troops, troopsMax: d.army.troops, zone: [...comp] } });
      if (old) {
        old.army.zone = old.army.zone.filter((k) => !comp.has(k));
        for (const o of dets(s, old.id)) if (o !== d && comp.has(key(o))) { o.army.captainId = cap.id; cap.army.troops += o.army.troops; }
        if (old.army.troopsMax != null) old.army.troopsMax = Math.max(0, old.army.troopsMax - cap.army.troops);
      }
      cap.army.troopsMax = cap.army.troops;
      d.army.captainId = cap.id;
      for (const x of dets(s, cap.id)) delete x.army.cutSince;
      // the new army takes its share of the old army's food; an orphan has none
      const share = old?.supply ? old.supply.food * Math.min(1, cap.army.troops / Math.max(1, before)) : 0;
      if (old?.supply) old.supply.food -= share;
      cap.supply = { food: share, foodMax: Math.max(1, share), auto: old?.supply?.auto ?? false };
      if (old) { syncTroops(s, old); if (!old.army.zone.length) undeploy(s, old); }
      note(s, cap.id, `promoted from a cut-off detachment with ${share.toFixed(1)} food`);
    },
    candidates: () => [],
  },
  buildFort: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      const t = tileAt(s, u);
      if (t.fort) return 'already a fort';
      if (cityAt(s, u)) return 'a city has its own garrison';
      if (!unitsAt(s, u).some((x) => x.owner === a.civ && fighting(x))) return 'needs troops on the tile';
      if (u.labour < ARMY.fort.labour) return `needs ${ARMY.fort.labour} labour`;
      const st = s.civs[a.civ].store;
      if ((st.wood ?? 0) < ARMY.fort.wood || (st.stone ?? 0) < ARMY.fort.stone) return `needs ${ARMY.fort.wood} wood and ${ARMY.fort.stone} stone`;
      return null;
    },
    apply(s, a) {
      const u = s.units[uid(a)];
      u.labour -= ARMY.fort.labour;
      takeGoods(s, a.civ, 'wood', ARMY.fort.wood); takeGoods(s, a.civ, 'stone', ARMY.fort.stone);
      tileAt(s, u).fort = { owner: a.civ, garrison: ARMY.garrison };
      note(s, u.id, 'built a fort');
    },
    candidates: () => [],
  },
};

// ---- turn ------------------------------------------------------------------------------------

function needy(z, m, target, from, reach) {
  let best = null;
  for (const k of z) {
    if (!reach.has(k) || (m[k] ?? 0) >= target - 0.001 || k === key(from)) continue;
    const d = distance(from, parse(k));
    if (!best || d < best.d) best = { d, ...parse(k) };
  }
  return best;
}
// One tile toward goal. In the zone (or anywhere when cut off); a hungry army never steps into a fight.
function step(s, d, goal, reach, cut = false) {
  if (!goal) return;
  const avoid = hungry(d, s);
  let best = null;
  for (const n of neighbors(d)) {
    if (!tileAt(s, n) || !passable(s, n) || (!cut && !reach.has(key(n)))) continue;
    if (avoid && hostile(s, n, d.owner)) continue;
    const dist = distance(n, goal);
    if (dist < distance(d, goal) && (!best || dist < best.dist)) best = { ...n, dist };
  }
  if (best) { d.q = best.q; d.r = best.r; d.army.entr = 0; }
}

function disbandDet(s, d, why) {
  returnToClans(s, d.owner, d, d.army.troops, why);
  alert(s, d.owner, `${d.army.troops} cut-off troops gave up and went home`);
  removeUnit(s, d.id);
}

function spread(s) {
  for (const cap of captains(s)) {
    const z = cap.army.zone;
    if (!z.length) continue;
    if (!dets(s, cap.id).length) deploy(s, cap);
    const reach = linked(s, cap);
    const target = cap.army.troops / z.length, cap_ = Math.max(1, Math.ceil(target));
    const per = () => { const m = {}; for (const d of dets(s, cap.id)) m[key(d)] = (m[key(d)] ?? 0) + d.army.troops; return m; };
    for (const d of dets(s, cap.id)) {
      const k = key(d), cut = !reach.has(k);
      let goal;
      if (cut) { // walk home; give up after a while
        d.army.cutSince ??= s.turn;
        if (s.turn - d.army.cutSince >= ARMY.cutTurns) { disbandDet(s, d, 'cut off too long'); continue; }
        goal = cap;
      } else {
        delete d.army.cutSince;
        const m = per();
        if (z.includes(k) && d.army.troops > cap_) { // overfull: split the excess off
          const out = d.army.troops - cap_;
          d.army.troops = cap_;
          d.army.entr = Math.min(ARMY.maxEntr, d.army.entr + 1);
          const n = spawnUnit(s, { owner: d.owner, type: 'detachment', q: d.q, r: d.r, army: { captainId: cap.id, troops: out, entr: 0 } });
          step(s, n, needy(z, per(), target, n, reach), reach);
          continue;
        }
        if (z.includes(k) && m[k] <= cap_ + 0.001) { d.army.entr = Math.min(ARMY.maxEntr, d.army.entr + 1); continue; }
        goal = needy(z, m, target, d, reach);
      }
      step(s, d, goal, reach, cut);
    }
    const by = {}; // merge detachments sharing a tile
    for (const d of dets(s, cap.id)) {
      const k = key(d);
      if (by[k]) { by[k].army.troops += d.army.troops; by[k].army.entr = Math.min(by[k].army.entr, d.army.entr); removeUnit(s, d.id); } else by[k] = d;
    }
    syncTroops(s, cap);
  }
  // Orphans (their captain is gone) walk to the nearest own city and go home, or give up.
  for (const d of Object.values(s.units)) {
    if (!orphan(s, d)) continue;
    d.army.cutSince ??= s.turn;
    const home = nearestCity(s, d.owner, d);
    if (!home || distance(home, d) === 0 || s.turn - d.army.cutSince >= ARMY.cutTurns) { disbandDet(s, d, 'its captain fell'); continue; }
    step(s, d, home, null, true);
  }
}

// The most garrison a city holds (see ARMY.garrisonPer).
export function garrisonMax(s, city) {
  const pop = civClans(s, city.owner).filter((c) => distance(c, city) <= CITY.territoryRadius).reduce((a, c) => a + c.pop, 0);
  return ARMY.garrison + Math.min(ARMY.garrisonPop, Math.floor(pop / ARMY.garrisonPer)) + (city.buildings?.wall ? ARMY.wallGarrison : 0);
}
const cityMult = (city, owner) => (city && city.owner === owner ? (1 + CITY.defence) * (city.buildings?.wall ? ARMY.wallMult : 1) : 1);
const ground = (tile) => 1 + (TERRAIN[tile?.terrain]?.defence ?? 0);
const power = (s, d, tile) => d.army.troops * (1 + ARMY.entrBonus * (d.army.entr ?? 0)) * (d.army.entr > 0 ? ground(tile) : 1)
  * (hungry(d, s) ? ARMY.hungryPower : 1) * cityMult(cityAt(s, tile), d.owner);
const garrisonPower = (gar, city, owner, fort, tile) => gar.garrison * ground(tile) *
  (fort ? 1 + ARMY.entrBonus * ARMY.maxEntr : ARMY.garrisonPower * (city?.buildings?.wall ? ARMY.wallMult : 1));

function skirmish(s) {
  const m = s.mods.army;
  const tiles = {};
  for (const d of Object.values(s.units)) if (fighting(d)) (tiles[key(d)] ??= []).push(d);
  const report = {}; // civ -> {tiles, lost, dealt}
  for (const [k, us] of Object.entries(tiles)) {
    const t = s.map.tiles[k];
    const sides = {};
    for (const u of us) (sides[u.owner] ??= []).push(u);
    const owners = Object.keys(sides).map(Number);
    // a fort or city fights with its garrison for its owner if no own troops stand there
    const city = cityAt(s, t);
    // a city's own garrison defends its tile (even if a fort stands there too)
    const cityGar = city && city.owner != null ? city : null;
    const gar = cityGar ?? t.fort ?? null;
    const gOwner = cityGar ? city.owner : t.fort ? t.fort.owner : null;
    const isFort = !cityGar && !!t.fort;
    if (gar && gar.garrison === undefined) gar.garrison = isFort ? ARMY.garrison : garrisonMax(s, city);
    const fortSide = gar && gar.garrison > 0 && !sides[gOwner] && owners.length && owners.some((o) => o !== gOwner) ? gOwner : null;
    if (fortSide != null) owners.push(fortSide);
    if (owners.length < 2) continue;
    m.fights = (m.fights ?? 0) + 1;
    const pw = {}, tr = {};
    for (const o of owners) {
      if (o === fortSide) { pw[o] = garrisonPower(gar, city, o, isFort, t); tr[o] = gar.garrison; continue; }
      pw[o] = sides[o].reduce((a, u) => a + power(s, u, t), 0);
      tr[o] = sides[o].reduce((a, u) => a + u.army.troops, 0);
      for (const u of sides[o]) { // fighting costs food: supply charges the captain
        const c = capOf(s, u);
        if (!c) continue;
        if (c.army.foughtTurn !== s.turn) { c.army.foughtTurn = s.turn; c.army.foughtTroops = 0; }
        c.army.foughtTroops += u.army.troops;
      }
    }
    const lost = {};
    for (const o of owners) {
      const foe = owners.filter((x) => x !== o).reduce((a, x) => a + pw[x], 0);
      if (o === fortSide) {
        const x = Math.min(gar.garrison, Math.round(ARMY.dmg * foe / 1.75), isFort ? Infinity : ARMY.siegeLoss);
        gar.garrison -= x; lost[o] = x;
        if (!isFort) {
          note(s, city.id, `besieged: the garrison lost ${x}, ${gar.garrison} left`);
          alert(s, o, `${city.name ?? 'Your city'} is besieged: its garrison lost ${x}, ${gar.garrison} left`);
        }
        continue;
      }
      if (foe / Math.max(1e-9, pw[o]) >= ARMY.retreatRatio && rand(s) < ARMY.retreatChance) {
        for (const u of sides[o]) {
          note(s, u.id, 'outnumbered, slipped away');
          const home = capOf(s, u) ?? u;
          const n = neighbors(u).filter((h) => tileAt(s, h) && passable(s, h) && !tiles[key(h)]).sort((a, b) => distance(a, home) - distance(b, home))[0];
          if (n) { u.q = n.q; u.r = n.r; u.army.entr = 0; }
        }
        lost[o] = 0;
        continue;
      }
      const loss = Math.max(1, Math.round(ARMY.dmg * foe));
      lost[o] = 0;
      for (const u of sides[o]) { const x = Math.min(u.army.troops, Math.ceil(loss * u.army.troops / tr[o])); u.army.troops -= x; lost[o] += x; }
    }
    for (const o of owners) {
      if (!s.civs[o]?.isHuman) continue;
      const r = (report[o] ??= { tiles: 0, lost: 0, dealt: 0 });
      r.tiles++; r.lost += lost[o];
      for (const x of owners) if (x !== o) r.dealt += lost[x];
    }
    if (fortSide != null && gar.garrison <= 0) {
      const winner = owners.find((o) => o !== fortSide && sides[o].some((u) => u.army.troops > 0));
      if (winner != null && isFort) {
        t.fort = { owner: winner, garrison: ARMY.garrison };
        event(s, `${civName(s, winner)} takes a fort from ${civName(s, fortSide)}`);
        alert(s, winner, `You take a fort at ${k}`); alert(s, fortSide, `${civName(s, winner)} takes your fort at ${k}`);
      } else if (winner == null) gar.garrison = 1;
      // a city with garrison 0 and a winner on the tile falls in capture()
    }
  }
  for (const [o, r] of Object.entries(report)) alert(s, Number(o), `Your troops fight on ${r.tiles} tile${r.tiles > 1 ? 's' : ''}: lost ${r.lost}, enemy lost ${r.dealt}`);
  for (const u of Object.values(s.units)) { // clear the fallen
    if (u.type === 'detachment' && u.army.troops <= 0) removeUnit(s, u.id);
    else if (u.type === 'captain' && !u.army.zone.length && u.army.troops <= 0) { alert(s, u.owner, `Your army ${u.id} is destroyed`); removeUnit(s, u.id); }
  }
  for (const cap of captains(s)) {
    syncTroops(s, cap);
    if (cap.army.zone.length && cap.army.troops <= 0) { alert(s, cap.owner, `Your army ${cap.id} is destroyed`); removeUnit(s, cap.id); }
  }
}

function hold(s) {
  for (const t of Object.values(s.map.tiles)) t.held = null;
  const best = {};
  for (const u of Object.values(s.units)) {
    if (!fighting(u) || !(u.army.troops > 0)) continue;
    const by = (best[key(u)] ??= {});
    by[u.owner] = (by[u.owner] ?? 0) + u.army.troops;
  }
  for (const [k, by] of Object.entries(best)) s.map.tiles[k].held = Number(Object.entries(by).sort((a, b) => b[1] - a[1] || a[0] - b[0])[0][0]);
  // troops standing on their own fort are fully dug in
  for (const d of Object.values(s.units)) if (d.type === 'detachment' && tileAt(s, d)?.fort?.owner === d.owner) d.army.entr = ARMY.maxEntr;
}

function capture(s) {
  for (const city of Object.values(s.cities)) {
    const t = tileAt(s, city);
    const max = garrisonMax(s, city);
    if (city.garrison === undefined) city.garrison = max;
    if (t.held == null || t.held === city.owner) {
      if (city.garrison < max && !t.held) city.garrison++;
      else if (city.garrison > max) city.garrison = max;
      continue;
    }
    if (city.garrison <= 0 && s.civs[t.held]) {
      const from = city.owner, nm = city.name ?? `city ${city.id}`;
      s.mods.army.captures = (s.mods.army.captures ?? 0) + 1;
      transferCity(s, city.id, t.held);
      city.garrison = Math.min(ARMY.garrison, garrisonMax(s, city));
      if (t.fort) t.fort.owner = t.held; // a fort in the city goes with it
      note(s, city.id, `captured by ${civName(s, t.held)}; its clans stay`);
      event(s, `${civName(s, t.held)} takes ${nm} from ${civName(s, from)}`);
      alert(s, t.held, `You take ${nm} from ${civName(s, from)}`);
      alert(s, from, `${civName(s, t.held)} takes your ${nm}`);
    }
  }
}

// Warns the human once when enemy troops come within ARMY.warnRange of one of its cities.
function warn(s) {
  const m = s.mods.army;
  const near = {};
  for (const city of Object.values(s.cities)) {
    if (!s.civs[city.owner]?.isHuman) continue;
    const foe = Object.values(s.units).find((u) => u.owner !== city.owner && fighting(u) && distance(u, city) <= ARMY.warnRange);
    if (!foe) continue;
    near[city.id] = true;
    if (!m.near?.[city.id]) alert(s, city.owner, `${civName(s, foe.owner)} troops (${Math.round(foe.army.troops)}) near ${city.name ?? 'your city'} at ${foe.q},${foe.r}`);
  }
  m.near = near;
}

function peaks(s) {
  const m = s.mods.army;
  m.peak ??= {};
  for (const c of Object.keys(s.civs)) m.peak[c] = Math.max(m.peak[c] ?? 0, captains(s, Number(c)).length);
}

export const turnSteps = [
  { phase: 'armies', name: 'army spread, skirmish, hold, capture', run(s) { spread(s); skirmish(s); hold(s); capture(s); warn(s); peaks(s); } },
];
export const hooks = {};

// ---- AI --------------------------------------------------------------------------------------

// Defence of a city as an attacker would face it: garrison (if no troops) or troops on the tile.
export function defence(s, city) {
  const t = tileAt(s, city);
  const troops = unitsAt(s, city).filter((u) => u.owner === city.owner && fighting(u));
  if (troops.length) return troops.reduce((a, u) => a + power(s, u, t), 0);
  return garrisonPower({ garrison: city.garrison ?? garrisonMax(s, city) }, city, city.owner, false, t);
}

// Whether an AI captain should build a fort where he stands: near home, facing a rival city,
// away from other forts and cities, with materials to spare (see ARMY.aiFort*).
function fortHere(s, cap, civId) {
  const t = tileAt(s, cap);
  if (t.fort || cityAt(s, cap) || actions.buildFort.validate(s, { type: 'buildFort', civ: civId, unit: cap.id })) return false;
  const st = s.civs[civId].store;
  if ((st.wood ?? 0) < ARMY.fort.wood + ARMY.aiFortSpare || (st.stone ?? 0) < ARMY.fort.stone + ARMY.aiFortSpare) return false;
  const forts = Object.values(s.map.tiles).filter((x) => x.fort);
  if (forts.filter((x) => x.fort.owner === civId).length >= ARMY.aiForts) return false;
  if (forts.some((x) => distance(x, cap) < ARMY.aiFortGap)) return false;
  if (!civCities(s, civId).some((c) => distance(c, cap) <= ARMY.aiFortHome)) return false;
  return Object.values(s.cities).some((c) => c.owner != null && c.owner !== civId && distance(c, cap) <= ARMY.aiFortRival);
}
const wildFood = (s, at, r) => Object.values(s.map.tiles)
  .filter((t) => distance(t, at) <= r && !(t.fallow > 0))
  .reduce((a, t) => a + FOODS.reduce((b, f) => b + (t.stock?.[f] ?? 0), 0), 0);

// The cheapest rival city this army (with friends nearby) can beat, or null.
function target(s, cap, civId, friends) {
  const strength = friends.filter((c) => distance(c, cap) <= 3 && fighting(c)).reduce((a, c) => a + c.army.troops, 0);
  let best = null;
  for (const c of Object.values(s.cities)) {
    if (c.owner == null || c.owner === civId) continue;
    if (strength < defence(s, c) * ARMY.aiOdds) continue;
    const p = pathCost(s, cap, c, { maxCost: ARMY.aiReach });
    if (p && (!best || p.cost < best.cost)) best = { city: c, cost: p.cost, path: p.path };
  }
  return best;
}
const reachableRival = (s, from, civId) => Object.values(s.cities).some((c) => c.owner != null && c.owner !== civId && pathCost(s, from, c, { maxCost: ARMY.aiReach }));

export function ai(s, civId) {
  const out = [];
  const m = s.mods.army;
  const caps = captains(s, civId);
  const mine = civCities(s, civId);
  let n = caps.length;
  for (const d of Object.values(s.units)) {
    if (d.owner === civId && orphan(s, d) && d.army.troops >= 3 && n < ARMY.maxCaptains) { out.push({ type: 'promote', civ: civId, unit: d.id }); n++; }
  }
  // Raise only with people to spare, food to feed them (store or wild land), and a rival in reach.
  // No AI army in the first aiGrace turns of the main game, so a new player can find its feet.
  const gap = s.turn - (m.raisedTurn?.[civId] ?? -Infinity);
  const settled = s.turn - (m.startTurn ?? -Infinity) >= ARMY.aiGrace;
  if (mine.length && settled && n < ARMY.maxCaptains && gap >= ARMY.aiRaiseGap && clanPop(s, civId) >= ARMY.aiRaisePop) {
    const site = mine.find((c) => (storeFood(s, civId) >= ARMY.size || wildFood(s, c, 2) >= ARMY.size * 2) && reachableRival(s, c, civId));
    if (site) out.push({ type: 'raiseArmy', civ: civId, city: site.id });
  }
  for (const cap of caps) {
    if (cap.army.zone.length) out.push({ type: 'undeploy', civ: civId, unit: cap.id }); // AI marches as one body
    const food = cap.supply?.food ?? 0;
    const starving = hungry(cap, s) && food <= 0;
    const threatened = mine.find((c) => Object.values(s.units).some((u) => u.owner !== civId && fighting(u) && distance(u, c) <= 2));
    let goal = null;
    if (starving) {
      const home = nearestCity(s, civId, cap);
      if (home && distance(home, cap) <= CITY.territoryRadius) { out.push({ type: 'disband', civ: civId, unit: cap.id }); continue; }
      goal = home;
    } else if (threatened) goal = threatened;
    else {
      const t = target(s, cap, civId, caps);
      goal = t ? t.city : (cityAt(s, cap)?.owner === civId ? null : nearestCity(s, civId, cap));
    }
    if (!starving && fortHere(s, cap, civId)) out.push({ type: 'buildFort', civ: civId, unit: cap.id });
    if (!goal || key(goal) === key(cap)) continue;
    const pc = pathCost(s, cap, goal, { maxCost: ARMY.aiReach * 2 });
    if (!pc) continue;
    let spent = 0;
    for (const h of pc.path.slice(1)) {
      const c = stepCost(s, h);
      if (spent > 0 && spent + c > ARMY.aiMoveLabour) break;
      if (hungry(cap, s) && hostile(s, h, civId)) break;
      out.push({ type: 'moveCaptain', civ: civId, unit: cap.id, q: h.q, r: h.r });
      spent += c;
      if (hostile(s, h, civId) || spent >= ARMY.aiMoveLabour) break;
    }
  }
  return out;
}
