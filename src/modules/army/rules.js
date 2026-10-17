// The army module: captains, zones, detachments, skirmish, entrenchment, forts, capture.
// Captain: unit type 'captain', u.army = {troops, zone:[keys], movedTurn}. Deployed (zone non-empty)
// => troops live in detachments (units type 'detachment', u.army = {captainId, troops, entr}).
// Undeployed => no detachments, all troops ride on the captain. Hungry: u.hungry or u.supply.hungry.
import { note, event, tileAt, passable, stepCost, cityAt, unitsAt, civCities, civClans, takeGoods, transferCity, spawnUnit, removeUnit } from '../../core.js';
import { key, neighbors, distance } from '../../hex.js';
import { rand } from '../../rng.js';
import { ARMY } from './data.js';

export const name = 'army';
export function init(s) { s.mods.army = {}; }
export function onStart(s) {}

const parse = (k) => { const [q, r] = k.split(',').map(Number); return { q, r }; };
const captains = (s, civ) => Object.values(s.units).filter((u) => u.type === 'captain' && (civ == null || u.owner === civ));
const dets = (s, cid) => Object.values(s.units).filter((u) => u.type === 'detachment' && u.army.captainId === cid);
const hungry = (u) => !!(u.hungry || u.supply?.hungry);

// Tiles reachable from the captain's tile through zone tiles.
function linked(s, cap) {
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
  for (const d of dets(s, cap.id)) removeUnit(s, d.id);
  cap.army.zone = [];
}
const ownCaptain = (s, a) => { const u = s.units[a.unit]; return u && u.type === 'captain' && u.owner === a.civ ? u : null; };

function clanPop(s, civ) { return civClans(s, civ).reduce((t, c) => t + c.pop, 0); }

export const actions = {
  raiseArmy: {
    validate(s, a) {
      const city = s.cities[a.city];
      if (!city || city.owner !== a.civ) return 'not your city';
      if (captains(s, a.civ).length >= ARMY.maxCaptains) return 'too many captains';
      if (clanPop(s, a.civ) < ARMY.size + 5) return 'not enough people to raise';
      return null;
    },
    apply(s, a) {
      const city = s.cities[a.city];
      let need = ARMY.size;
      for (const c of civClans(s, a.civ).sort((x, y) => y.pop - x.pop)) {
        const take = Math.min(need, Math.max(0, c.pop - 1));
        c.pop -= take; need -= take;
        if (take) note(s, c.id, `${take} people raised as troops for civ ${a.civ}`);
        if (!need) break;
      }
      const cap = spawnUnit(s, { owner: a.civ, type: 'captain', q: city.q, r: city.r, army: { troops: ARMY.size - need, zone: [], movedTurn: -1 } });
      note(s, cap.id, `raised at ${city.name ?? city.id} from the civ's clans`);
      event(s, `${s.civs[a.civ].name} raises an army`);
    },
    candidates: (s, civ) => civCities(s, civ).map((c) => ({ type: 'raiseArmy', civ, city: c.id })).slice(0, 1),
  },
  moveCaptain: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      if (u.army.movedTurn === s.turn) return 'already moved';
      if (distance(u, a) !== 1) return 'one tile at a time';
      if (!tileAt(s, a) || !passable(s, a) || stepCost(s, tileAt(s, a)) === Infinity) return 'impassable';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unit];
      const from = key(u);
      u.q = a.q; u.r = a.r; u.army.movedTurn = s.turn;
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
      const u = s.units[a.unit];
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
      const u = s.units[a.unit];
      u.army.zone = u.army.zone.filter((x) => x !== key(a));
      if (!u.army.zone.length) undeploy(s, u);
    },
    candidates: () => [],
  },
  undeploy: {
    validate: (s, a) => (ownCaptain(s, a) ? null : 'not your captain'),
    apply(s, a) { undeploy(s, s.units[a.unit]); },
    candidates: () => [],
  },
  promote: {
    validate(s, a) {
      const d = s.units[a.unit];
      if (!d || d.type !== 'detachment' || d.owner !== a.civ) return 'not your detachment';
      const cap = s.units[d.army.captainId];
      if (cap && linked(s, cap).has(key(d))) return 'still linked to its captain';
      return null;
    },
    apply(s, a) {
      const d = s.units[a.unit], old = s.units[d.army.captainId];
      const set = new Set(old ? old.army.zone : [key(d)]), comp = new Set([key(d)]), queue = [{ q: d.q, r: d.r }];
      while (queue.length) for (const n of neighbors(queue.pop())) {
        const k = key(n);
        if (set.has(k) && !comp.has(k)) { comp.add(k); queue.push(n); }
      }
      const cap = spawnUnit(s, { owner: d.owner, type: 'captain', q: d.q, r: d.r, army: { troops: d.army.troops, zone: [...comp], movedTurn: -1 } });
      if (old) {
        old.army.zone = old.army.zone.filter((k) => !comp.has(k));
        for (const o of dets(s, old.id)) if (o !== d && comp.has(key(o))) { o.army.captainId = cap.id; cap.army.troops += o.army.troops; }
      }
      d.army.captainId = cap.id;
      if (old) syncTroops(s, old);
      note(s, cap.id, 'promoted from a cut-off detachment, for free');
    },
    candidates: () => [],
  },
  buildFort: {
    validate(s, a) {
      const u = ownCaptain(s, a);
      if (!u) return 'not your captain';
      const t = tileAt(s, u);
      if (t.fort) return 'already a fort';
      if (!unitsAt(s, u).some((x) => x.owner === a.civ && (x.type === 'detachment' || x.army?.troops))) return 'needs troops on the tile';
      if (u.labour < ARMY.fort.labour) return 'not enough labour';
      const st = s.civs[a.civ].store;
      if ((st.wood ?? 0) < ARMY.fort.wood || (st.stone ?? 0) < ARMY.fort.stone) return 'needs wood and stone';
      return null;
    },
    apply(s, a) {
      const u = s.units[a.unit];
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
function step(s, d, goal, reach, cut = false) {
  if (!goal) return;
  let best = null;
  for (const n of neighbors(d)) {
    if (!tileAt(s, n) || !passable(s, n) || (!cut && !reach.has(key(n)))) continue;
    const dist = distance(n, goal);
    if (dist < distance(d, goal) && (!best || dist < best.dist)) best = { ...n, dist };
  }
  if (best) { d.q = best.q; d.r = best.r; d.army.entr = 0; }
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
      if (cut) goal = cap; // walk home
      else {
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
}

const power = (s, d, tile) => {
  const city = cityAt(s, tile);
  return d.army.troops * (1 + ARMY.entrBonus * (d.army.entr ?? 0)) * (hungry(d) ? 0.5 : 1) * (city && city.owner === d.owner ? 1.5 : 1);
};

function skirmish(s) {
  const tiles = {};
  for (const d of Object.values(s.units)) {
    if (d.type === 'detachment' || (d.type === 'captain' && !d.army.zone.length && d.army.troops)) (tiles[key(d)] ??= []).push(d);
  }
  for (const [k, us] of Object.entries(tiles)) {
    const t = s.map.tiles[k];
    const sides = {};
    for (const u of us) (sides[u.owner] ??= []).push(u);
    const owners = Object.keys(sides).map(Number);
    // a fort fights with its garrison for its owner if no own troops stand there
    const fortSide = t.fort && !sides[t.fort.owner] && owners.length ? t.fort.owner : null;
    if (fortSide != null) owners.push(fortSide);
    if (owners.length < 2) continue;
    const pw = {}, tr = {};
    for (const o of owners) {
      if (o === fortSide) { pw[o] = ARMY.garrison * (1 + ARMY.entrBonus * ARMY.maxEntr); tr[o] = ARMY.garrison; continue; }
      pw[o] = sides[o].reduce((a, u) => a + power(s, u, t), 0);
      tr[o] = sides[o].reduce((a, u) => a + u.army.troops, 0);
    }
    for (const o of owners) {
      const foe = owners.filter((x) => x !== o).reduce((a, x) => a + pw[x], 0);
      if (o === fortSide) { t.fort.garrison = Math.max(0, t.fort.garrison - Math.round(ARMY.dmg * foe / 1.75)); continue; }
      if (foe / Math.max(1e-9, pw[o]) >= ARMY.retreatRatio && rand(s) < ARMY.retreatChance) {
        for (const u of sides[o]) {
          note(s, u.id, 'outnumbered, slipped away');
          const home = s.units[u.army.captainId] ?? u;
          const n = neighbors(u).filter((h) => tileAt(s, h) && passable(s, h) && !tiles[key(h)]).sort((a, b) => distance(a, home) - distance(b, home))[0];
          if (n) { u.q = n.q; u.r = n.r; u.army.entr = 0; }
        }
        continue;
      }
      const loss = Math.max(1, Math.round(ARMY.dmg * foe));
      for (const u of sides[o]) u.army.troops -= Math.min(u.army.troops, Math.ceil(loss * u.army.troops / tr[o]));
    }
    if (fortSide != null && t.fort.garrison <= 0) {
      const winner = owners.find((o) => o !== fortSide && sides[o].some((u) => u.army.troops > 0));
      if (winner != null) { t.fort = { owner: winner, garrison: ARMY.garrison }; event(s, 'a fort falls'); } else t.fort.garrison = 1;
    }
  }
  for (const u of Object.values(s.units)) { // clear the fallen
    if (u.type === 'detachment' && u.army.troops <= 0) removeUnit(s, u.id);
    else if (u.type === 'captain' && !u.army.zone.length && u.army.troops <= 0) removeUnit(s, u.id);
  }
  for (const cap of captains(s)) {
    syncTroops(s, cap);
    if (cap.army.zone.length && cap.army.troops <= 0) removeUnit(s, cap.id);
  }
}

function hold(s) {
  for (const t of Object.values(s.map.tiles)) t.held = null;
  const best = {};
  for (const u of Object.values(s.units)) {
    const on = u.type === 'detachment' || (u.type === 'captain' && !u.army.zone.length);
    if (!on || !(u.army.troops > 0)) continue;
    const by = (best[key(u)] ??= {});
    by[u.owner] = (by[u.owner] ?? 0) + u.army.troops;
  }
  for (const [k, by] of Object.entries(best)) s.map.tiles[k].held = Number(Object.entries(by).sort((a, b) => b[1] - a[1])[0][0]);
  // troops standing on their own fort are fully dug in
  for (const d of Object.values(s.units)) if (d.type === 'detachment' && tileAt(s, d)?.fort?.owner === d.owner) d.army.entr = ARMY.maxEntr;
}

function capture(s) {
  for (const city of Object.values(s.cities)) {
    const t = tileAt(s, city);
    if (t.held != null && t.held !== city.owner && s.civs[t.held]) {
      transferCity(s, city.id, t.held);
      note(s, city.id, `captured by civ ${t.held}; its clans stay`);
      event(s, `${s.civs[t.held].name} takes ${city.name ?? 'a city'}`);
    }
  }
}

export const turnSteps = [
  { phase: 'armies', name: 'army spread, skirmish, hold, capture', run(s) { spread(s); skirmish(s); hold(s); capture(s); } },
];
export const hooks = {};

// ---- AI --------------------------------------------------------------------------------------

export function ai(s, civId) {
  const out = [];
  const caps = captains(s, civId);
  const mine = civCities(s, civId);
  if (mine.length && caps.length < ARMY.maxCaptains && clanPop(s, civId) >= ARMY.size + 5) out.push({ type: 'raiseArmy', civ: civId, city: mine[0].id });
  const rivals = Object.values(s.cities).filter((c) => c.owner !== civId);
  for (const cap of caps) {
    const threatened = mine.find((c) => Object.values(s.units).some((u) => u.owner !== civId && (u.type === 'detachment' || u.type === 'captain') && distance(u, c) <= 2));
    const goalC = threatened ?? [...rivals].sort((a, b) => distance(cap, a) - distance(cap, b))[0];
    for (const d of dets(s, cap.id)) if (d.army.troops >= 3 && !linked(s, cap).has(key(d))) { out.push({ type: 'promote', civ: civId, unit: d.id }); break; }
    if (!goalC) continue;
    if (!cap.army.zone.length && cap.army.troops) out.push({ type: 'paint', civ: civId, unit: cap.id, q: cap.q, r: cap.r });
    else if (cap.army.zone.length > 6) {
      const far = [...cap.army.zone].sort((a, b) => distance(parse(b), cap) - distance(parse(a), cap))[0];
      if (zoneConnected(cap.army.zone.filter((k) => k !== far))) out.push({ type: 'unpaint', civ: civId, unit: cap.id, ...parse(far) });
    }
    if (cap.army.movedTurn === s.turn || distance(cap, goalC) === 0) continue;
    const n = neighbors(cap).filter((h) => tileAt(s, h) && passable(s, h) && stepCost(s, tileAt(s, h)) < Infinity)
      .sort((a, b) => distance(a, goalC) - distance(b, goalC))[0];
    if (n && distance(n, goalC) < distance(cap, goalC)) out.push({ type: 'moveCaptain', civ: civId, unit: cap.id, q: n.q, r: n.r });
  }
  return out;
}
