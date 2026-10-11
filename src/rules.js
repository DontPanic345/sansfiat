import { CLAN, FOOD, FOODS, MARKET, NAME_SYLLABLES, PREGAME, TENDING, TERRAIN } from './data.js';
import { distance, key, neighbors } from './hex.js';
import { clear, demand, utility } from './market.js';
import { rand } from './rng.js';

export const tileAt = (s, h) => s.map.tiles[key(h)];
export const passable = (s, h) => !!tileAt(s, h) && TERRAIN[tileAt(s, h).terrain].passable;
export const clanAt = (s, h) => Object.values(s.clans).find((c) => c.q === h.q && c.r === h.r);
export const marketAt = (s, h) => Object.values(s.markets).find((m) => m.q === h.q && m.r === h.r);
export const footprint = (s, h) => [h, ...neighbors(h)].map((n) => tileAt(s, n)).filter(Boolean);
export const holder = (s, h) => s.clans[tileAt(s, h)?.claim];
export const total = (basket) => Object.values(basket).reduce((a, b) => a + b, 0);
export const totalPop = (s) => Object.values(s.clans).reduce((a, c) => a + c.pop, 0);

const add = (a, b) => {
  const out = { ...a };
  for (const [f, v] of Object.entries(b)) out[f] = (out[f] ?? 0) + v;
  return out;
};

// A food counts in full up to dietShare of need, and at excessValue beyond that.
export function nutrition(basket, need) {
  const k = CLAN.dietShare * need;
  return Object.values(basket).reduce((n, v) => n + Math.min(v, k) + CLAN.excessValue * Math.max(0, v - k), 0);
}

// Path cost over land from the market, out to MARKET.reach.
export function marketReach(s, m) {
  const cost = new Map([[key(m), 0]]);
  const frontier = [{ h: m, c: 0 }];
  while (frontier.length) {
    frontier.sort((a, b) => a.c - b.c);
    const { h, c } = frontier.shift();
    if (c > cost.get(key(h))) continue;
    for (const n of neighbors(h)) {
      if (!passable(s, n)) continue;
      const nc = c + TERRAIN[tileAt(s, n).terrain].moveCost;
      if (nc <= MARKET.reach && nc < (cost.get(key(n)) ?? Infinity)) {
        cost.set(key(n), nc);
        frontier.push({ h: n, c: nc });
      }
    }
  }
  return cost;
}

const reaches = (s) => Object.values(s.markets).map((m) => ({ market: m, cost: marketReach(s, m) }));

export function nearestMarket(rs, h) {
  let best = null;
  for (const { market, cost } of rs) {
    const c = cost.get(key(h));
    if (c != null && (!best || c < best.cost)) best = { market, cost: c };
  }
  return best;
}

// The tiles a clan holding `land` forages: the land and every tile next to it nobody else holds.
export function range(s, land, id) {
  const out = new Map();
  for (const k of land)
    for (const t of footprint(s, s.map.tiles[k])) if (t.claim == null || t.claim === id) out.set(key(t), t);
  return [...out.values()];
}

// A walking clan forages around wherever it stands.
export const forageTiles = (s, clan) => range(s, clan.land.length && !clan.route.length ? clan.land : [key(clan)], clan.id);

const available = (tiles) => {
  const avail = {};
  for (const t of tiles)
    for (const [f, v] of Object.entries(t.stock)) avail[f] = (avail[f] ?? 0) + v * CLAN.takeRate;
  return avail;
};

// How much of each food a clan would take from `tiles`. It fills each food up to its diet
// share first, then takes the rest in proportion to what is there.
export function planHarvest(tiles, pop, labourShare, store = {}) {
  const avail = available(tiles);
  let budget = Math.min(pop * CLAN.harvestPerHead * labourShare, Math.max(0, CLAN.surplusTarget * pop - total(store)));
  const k = CLAN.dietShare * pop;
  const first = Object.fromEntries(Object.entries(avail).map(([f, v]) => [f, Math.min(v, Math.max(0, k - (store[f] ?? 0)))]));
  const scale = Math.min(1, budget / (total(first) || 1));
  const plan = Object.fromEntries(Object.entries(first).map(([f, v]) => [f, v * scale]));
  budget -= total(plan);
  const rest = Object.fromEntries(Object.entries(avail).map(([f, v]) => [f, v - plan[f]]));
  const restTotal = total(rest);
  if (budget > 0 && restTotal > 0)
    for (const f of Object.keys(rest)) plan[f] += rest[f] * Math.min(1, budget / restTotal);
  return plan;
}

function harvest(s, clan, labourShare) {
  const tiles = forageTiles(s, clan);
  const plan = planHarvest(tiles, clan.pop, labourShare, clan.store);
  for (const [f, want] of Object.entries(plan)) {
    const there = tiles.reduce((a, t) => a + (t.stock[f] ?? 0), 0);
    if (there > 0) for (const t of tiles) if (t.stock[f]) t.stock[f] -= want * (t.stock[f] / there);
  }
  return plan;
}

const siteFed = (tiles, pop, store) => nutrition(add(store, planHarvest(tiles, pop, 1, store)), pop) / pop;
// How well the tiles could feed the clan if labour were no limit.
const potential = (tiles, pop) => nutrition(available(tiles), pop) / pop;

// Tiles a clan can walk to within `range` steps without passing another clan.
// Returns a map of tile key -> previous key on the way there.
function walkable(s, from, blocked, range) {
  const prev = new Map([[key(from), null]]);
  let frontier = [from];
  for (let d = 0; d < range; d++) {
    const next = [];
    for (const h of frontier)
      for (const n of neighbors(h)) {
        const k = key(n);
        if (prev.has(k) || blocked.has(k) || !passable(s, n)) continue;
        prev.set(k, key(h));
        next.push(n);
      }
    frontier = next;
  }
  return prev;
}

function routeTo(prev, k) {
  const route = [];
  for (let at = k; prev.get(at) != null; at = prev.get(at)) route.unshift(at);
  return route;
}

const accessBonus = (rs, h) => (nearestMarket(rs, h) ? MARKET.accessBonus : 0);

// Scores every unclaimed tile the clan could walk to, as a one-tile home.
function siteOptions(s, clan, rs, { steps = CLAN.moveRange, pop = clan.pop } = {}) {
  const others = Object.values(s.clans).filter((c) => c.id !== clan.id);
  const prev = walkable(s, clan, new Set(others.map(key)), steps);
  const options = [];
  for (const k of prev.keys()) {
    const t = s.map.tiles[k];
    if (k === key(clan) || t.claim != null) continue;
    const fed = siteFed(range(s, [k], clan.id), pop, clan.store);
    options.push({ k, fed, score: fed + accessBonus(rs, t) });
  }
  return { prev, options, here: siteFed(forageTiles(s, clan), pop, clan.store) + accessBonus(rs, clan) };
}

// Picks among sites scoring above `min`, favouring better ones without always taking the best.
function pickSite(s, options, min) {
  const good = options.filter((o) => o.score > min);
  if (!good.length) return null;
  const best = Math.max(...good.map((o) => o.score));
  const weights = good.map((o) => Math.exp((o.score - best) / CLAN.choiceSpread));
  let r = rand(s) * weights.reduce((a, w) => a + w, 0);
  for (let i = 0; i < good.length; i++) if ((r -= weights[i]) <= 0) return good[i].k;
  return good.at(-1).k;
}

// Trade only if the basket, less the labour lost on the trip, is worth more at last turn's ratios.
function worthTrading(basket, trip, prices) {
  const kept = Object.fromEntries(Object.entries(basket).map(([f, v]) => [f, v * (1 - trip)]));
  const goods = FOODS.filter((f) => (basket[f] ?? 0) > 0 || prices[f]);
  const p = Object.fromEntries(goods.map((f) => [f, prices[f] ?? 1]));
  return utility(demand(kept, p, goods)) > utility(basket);
}

function eat(clan) {
  const need = clan.pop;
  const k = CLAN.dietShare * need;
  const left = {};
  let got = 0;
  for (const [f, v] of Object.entries(clan.basket)) {
    got += Math.min(v, k);
    left[f] = Math.max(0, v - k);
  }
  const extra = total(left);
  const eaten = Math.min(extra, Math.max(0, need - got) / CLAN.excessValue);
  got += eaten * CLAN.excessValue;
  const keep = extra ? 1 - eaten / extra : 0;
  clan.store = {};
  for (const [f, v] of Object.entries(left)) if (v * keep > 0.01) clan.store[f] = v * keep * (1 - CLAN.spoilage);
  clan.fedRatio = got / need;
  clan.fed = clan.fedRatio >= 0.999;
  clan.starving = clan.fedRatio < CLAN.stableAt;
  const [mobile, settled] = CLAN.growth;
  const growth = mobile + (settled - mobile) * Math.min(1, clan.since / CLAN.settleTurns);
  if (clan.fed) clan.pop *= 1 + growth;
  else if (clan.starving) clan.pop *= 1 - CLAN.starve * (1 - clan.fedRatio / CLAN.stableAt);
}

export function clanName(s) {
  const n = rand(s) < 0.3 ? 3 : 2;
  const name = Array.from({ length: n }, () => NAME_SYLLABLES[Math.floor(rand(s) * NAME_SYLLABLES.length)]).join('');
  return name[0].toUpperCase() + name.slice(1);
}

// The clan claims `land`, centred on where it stands unless it is walking to `land[0]`.
export function spawnClan(s, hex, pop, store = {}, land = [key(hex)]) {
  const id = s.nextId++;
  const clan = s.clans[id] = {
    id, name: clanName(s), q: hex.q, r: hex.r, pop, store, harvest: {}, basket: {}, fed: true, starving: false,
    fedRatio: 1, hungry: 0, route: [], dest: null, blocked: 0, trail: [], market: null, trip: 0, land: [], since: 0,
  };
  for (const k of land) claimTile(s, clan, k);
  return clan;
}

function claimTile(s, clan, k) {
  s.map.tiles[k].claim = clan.id;
  clan.land.push(k);
}

function releaseTile(s, clan, k) {
  s.map.tiles[k].claim = null;
  clan.land = clan.land.filter((x) => x !== k);
}

const releaseAll = (s, clan) => [...clan.land].forEach((k) => releaseTile(s, clan, k));

function connected(s, land) {
  const seen = new Set([land[0]]);
  const stack = [land[0]];
  while (stack.length)
    for (const n of neighbors(s.map.tiles[stack.pop()])) {
      const k = key(n);
      if (land.includes(k) && !seen.has(k)) { seen.add(k); stack.push(k); }
    }
  return seen.size === land.length;
}

// The clan walks its whole land each turn, so its land's move cost is held to CLAN.roam.
export const roamCost = (s, land) => land.reduce((a, k) => a + TERRAIN[s.map.tiles[k].terrain].moveCost, 0);

// The tile with the least total distance to the rest of the land.
function centre(s, land, prefer) {
  const tiles = land.map((k) => s.map.tiles[k]);
  const spread = (t) => tiles.reduce((a, x) => a + distance(t, x), 0);
  return tiles.reduce((best, t) => {
    const d = spread(t) - spread(best);
    return d < 0 || (d === 0 && key(t) === prefer) ? t : best;
  });
}

// Home steps one tile a turn toward the centre of the clan's land.
function recentre(s, clan) {
  const c = centre(s, clan.land, key(clan));
  if (distance(c, clan) === 0) return;
  const step = clan.land.map((k) => s.map.tiles[k])
    .filter((t) => distance(t, clan) === 1 && distance(t, c) < distance(clan, c) && !clanAt(s, t))[0];
  if (!step) return;
  clan.trail.push(key(clan));
  Object.assign(clan, { q: step.q, r: step.r });
}

// Tiles other than home the clan could give up and stay in one piece, best value left first.
function drops(s, clan, land, value) {
  return land.filter((k) => {
    const rest = land.filter((x) => x !== k);
    return k !== key(clan) && connected(s, rest);
  }).map((k) => ({ k, v: value(land.filter((x) => x !== k)) })).sort((a, b) => b.v - a.v);
}

// A clan that isn't growing takes the best free tile next to its land, or once it can roam no
// further, swaps its worst tile for it, so land creeps toward better ground.
function adjustLand(s, clan) {
  if (!clan.land.length) {
    if (tileAt(s, clan).claim == null) claimTile(s, clan, key(clan));
    else clan.hungry = Math.max(clan.hungry, CLAN.patience);
    return;
  }
  if (!clan.fed) {
    const value = (land) => potential(range(s, land, clan.id), clan.pop);
    const now = value(clan.land);
    const edge = new Set(clan.land.flatMap((k) => neighbors(s.map.tiles[k]).map(key))
      .filter((k) => s.map.tiles[k]?.claim == null && passable(s, s.map.tiles[k])));
    const adds = [...edge].map((k) => ({ k, v: value([...clan.land, k]) })).filter((a) => a.v > now).sort((a, b) => b.v - a.v);
    const fits = adds.find((a) => roamCost(s, [...clan.land, a.k]) <= CLAN.roam);
    if (fits) claimTile(s, clan, fits.k);
    else if (adds.length) {
      const add = adds[0];
      const land = [...clan.land, add.k];
      const drop = drops(s, clan, land, value)
        .find((d) => d.k !== add.k && roamCost(s, land.filter((x) => x !== d.k)) <= CLAN.roam);
      if (drop && drop.v > now + CLAN.landMargin) {
        claimTile(s, clan, add.k);
        releaseTile(s, clan, drop.k);
      }
    }
  }
  recentre(s, clan);
}

// Splits land in two connected halves grown out from its two farthest tiles.
function halve(s, land) {
  const tiles = land.map((k) => s.map.tiles[k]);
  let seeds = [tiles[0], tiles[0]];
  for (const a of tiles) for (const b of tiles) if (distance(a, b) > distance(...seeds)) seeds = [a, b];
  const parts = seeds.map((t) => [key(t)]);
  const left = new Set(land.filter((k) => !parts.some((p) => p[0] === k)));
  while (left.size) {
    let grew = false;
    parts.forEach((part, i) => {
      const next = [...left].filter((k) => part.some((p) => distance(s.map.tiles[p], s.map.tiles[k]) === 1))
        .sort((a, b) => distance(s.map.tiles[a], seeds[i]) - distance(s.map.tiles[b], seeds[i]))[0];
      if (next) { part.push(next); left.delete(next); grew = true; }
    });
    if (!grew) break;
  }
  return parts;
}

function regrow(s) {
  for (const t of Object.values(s.map.tiles))
    for (const [f, cap] of Object.entries(t.cap)) {
      const { regen, breeds, restock } = FOOD[f];
      const now = t.stock[f] ?? 0;
      const grow = breeds ? cap * restock + regen * now * (1 - now / cap) : cap * regen;
      t.stock[f] = Math.min(cap, now + grow);
    }
}

function shuffled(s, arr) {
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(rand(s) * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

// One step along the clan's route. A blocked clan waits, and gives up after two turns.
function walkOn(s, clan, occupied) {
  const next = clan.route[0];
  if (occupied.has(next)) {
    if (++clan.blocked >= 2) stopWalking(s, clan);
    return;
  }
  occupied.delete(key(clan));
  occupied.add(next);
  clan.trail.push(key(clan));
  if (clan.trail.length > CLAN.trail) clan.trail.shift();
  const t = s.map.tiles[next];
  Object.assign(clan, { q: t.q, r: t.r, blocked: 0 });
  clan.route.shift();
  if (!clan.route.length) Object.assign(clan, { dest: null, hungry: 0 });
}

// Leaves its land for `dest`, which it claims at once so no one else settles there.
function depart(s, clan, prev, dest) {
  releaseAll(s, clan);
  claimTile(s, clan, dest);
  Object.assign(clan, { dest, route: routeTo(prev, dest), since: 0 });
}

// A clan that gives up on a walk stays where it stands, if no one holds that tile.
function stopWalking(s, clan) {
  releaseAll(s, clan);
  Object.assign(clan, { route: [], dest: null, blocked: 0 });
}

function forage(s, rs) {
  const occupied = new Set(Object.values(s.clans).map(key));
  for (const clan of shuffled(s, Object.values(s.clans))) {
    const moving = clan.route.length > 0;
    if (!moving && clan.hungry >= CLAN.patience) {
      const { prev, options, here } = siteOptions(s, clan, rs);
      const dest = pickSite(s, options, here + CLAN.moveMargin);
      if (dest) {
        depart(s, clan, prev, dest);
        s.events.push(`Turn ${s.turn}: ${clan.name} leaves ${key(clan)} for ${dest}`);
      }
    }
    if (clan.route.length) walkOn(s, clan, occupied);
    else if (!moving) {
      adjustLand(s, clan);
      if (clan.trail.length) clan.trail.shift();
    }

    const near = nearestMarket(rs, clan);
    const trip = near ? near.cost * MARKET.tripCost : 0;
    const go = near && worthTrading(add(clan.store, clan.harvest), trip, near.market.prices);
    clan.market = go ? near.market.id : null;
    clan.trip = go ? trip : 0;
    clan.harvest = harvest(s, clan, 1 - clan.trip);
    clan.basket = add(clan.store, clan.harvest);
    clan.store = {};
  }
}

function trade(s) {
  for (const m of Object.values(s.markets)) {
    const traders = Object.values(s.clans).filter((c) => c.market === m.id);
    const res = clear(Object.fromEntries(traders.map((c) => [c.id, c.basket])), m.prices);
    for (const c of traders) c.basket = res.after[c.id];
    Object.assign(m.prices, res.prices);
    m.traders = traders.length;
    m.lastVolume = total(res.volume);
    m.lastTrade = res.volume;
    m.volume += m.lastVolume;
    m.activity = MARKET.gathering.fade * m.activity + (1 - MARKET.gathering.fade) * m.lastVolume;
    m.history.push({ turn: s.turn, prices: { ...res.prices }, volume: m.lastVolume });
    if (m.history.length > MARKET.history) m.history.shift();
  }
}

// The clan divides like a cell: its land splits down the middle, and the daughter takes the
// half away from home with half the people and store.
function divide(s, clan) {
  const [a, b] = halve(s, clan.land);
  const [keep, give] = a.includes(key(clan)) ? [a, b] : [b, a];
  const home = centre(s, give.filter((k) => !clanAt(s, s.map.tiles[k])), null);
  if (!keep.length || !home) return false;
  for (const k of give) releaseTile(s, clan, k);
  clan.pop /= 2;
  clan.store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v / 2]));
  const daughter = spawnClan(s, home, clan.pop, { ...clan.store }, give);
  daughter.since = clan.since;
  s.events.push(`Turn ${s.turn}: ${clan.name} divides; ${daughter.name} takes ${give.length} of its tiles`);
  return true;
}

// A daughter takes half the clan, or half of splitAt from a big one, and walks off to a site
// of her own, starting next door. She only goes if some site in reach would feed her.
function split(s, clan, rs) {
  const pop = Math.min(clan.pop, CLAN.splitAt) / 2;
  const { prev, options } = siteOptions(s, clan, rs, { steps: CLAN.scoutRange, pop });
  const dest = pickSite(s, options.filter((o) => o.fed >= CLAN.settleFed), -Infinity);
  if (!dest) return;
  const route = routeTo(prev, dest);
  const share = pop / clan.pop;
  const store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v * share]));
  clan.store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v * (1 - share)]));
  clan.pop -= pop;
  clan.hungry = 0;
  const daughter = spawnClan(s, s.map.tiles[route[0]], pop, store, [dest]);
  daughter.trail = [key(clan)];
  if (route.length > 1) Object.assign(daughter, { route: route.slice(1), dest });
  s.events.push(`Turn ${s.turn}: ${clan.name} splits; ${daughter.name} heads for ${dest}`);
}

function settle(s, rs) {
  for (const clan of Object.values(s.clans)) {
    eat(clan);
    clan.hungry = clan.starving ? clan.hungry + 1 : 0;
    if (clan.pop < 1) {
      releaseAll(s, clan);
      delete s.clans[clan.id];
      s.events.push(`Turn ${s.turn}: ${clan.name} dies out`);
      continue;
    }
    const settled = !clan.route.length;
    if (settled) clan.since++;
    if (!settled) continue;
    if (clan.pop >= CLAN.splitAt) {
      if (clan.land.length < 2 || !divide(s, clan)) split(s, clan, rs);
    } else if (clan.starving && clan.pop >= CLAN.hungrySplit) split(s, clan, rs);
  }
}

// Held land counts as occupied. Grain on land held long enough is tended toward domestication,
// and goes feral again when left.
function occupy(s) {
  for (const [k, t] of Object.entries(s.map.tiles)) {
    if (t.claim != null) { t.occ++; t.occTotal++; }
    else t.occ = 0;
    if (t.wildGrain == null) continue;
    const was = t.tame;
    const step = t.claim == null ? -TENDING.rate : t.occ >= TENDING.tendAfter ? TENDING.rate : 0;
    t.tame = Math.max(0, Math.min(1, t.tame + step));
    if (t.tame === was) continue;
    t.cap.grain = Math.round(t.wildGrain * (1 + (TENDING.domestic - 1) * t.tame) * 10) / 10;
    if (t.tame === 1) s.events.push(`Turn ${s.turn}: grain is domesticated at ${k}`);
  }
}

function farFromMarkets(s, h) {
  return Object.values(s.markets).every((m) => distance(m, h) >= MARKET.spacing);
}

export function foundMarket(s, h) {
  const id = s.nextId++;
  const temporary = s.phase === 'pregame';
  s.markets[id] = {
    id, q: h.q, r: h.r, founded: s.turn, owner: null, temporary, prices: {}, volume: 0, lastVolume: 0, lastTrade: {},
    traders: 0, activity: 2 * MARKET.gathering.minVolume, history: [],
  };
  s.events.push(`Turn ${s.turn}: ${temporary ? 'clans gather to trade' : 'a market forms'} at ${key(h)}`);
  return s.markets[id];
}

// Clans living within market reach of h, and their people.
function catchment(s, h) {
  const reach = marketReach(s, h);
  const near = Object.values(s.clans).filter((c) => reach.has(key(c)));
  return { clans: near.length, pop: near.reduce((a, c) => a + c.pop, 0) };
}

const settledSites = (s) => Object.values(s.clans).filter((c) => !c.route.length).map((c) => tileAt(s, c))
  .filter((t) => t.occ >= MARKET.formAfter && farFromMarkets(s, t));

// In the pre-game a gathering forms where the most people live within reach, and disperses once
// trade there dies down. In the game a market forms on the longest-held site far from the rest.
function formMarkets(s) {
  if (s.phase === 'pregame') {
    for (const m of Object.values(s.markets))
      if (m.temporary && m.activity < MARKET.gathering.minVolume) {
        delete s.markets[m.id];
        s.events.push(`Turn ${s.turn}: the gathering at ${key(m)} disperses`);
      }
    const site = settledSites(s).map((t) => ({ t, ...catchment(s, t) }))
      .filter((x) => x.clans >= MARKET.gathering.minClans).sort((a, b) => b.pop - a.pop)[0];
    if (site) foundMarket(s, site.t);
    return;
  }
  const site = settledSites(s).sort((a, b) => b.occ - a.occ)[0];
  if (site) foundMarket(s, site);
}

// Picks tiles in order, each at least `spacing` from those already picked.
function spread(tiles, spacing) {
  const out = [];
  for (const t of tiles) if (out.every((o) => distance(o, t) >= spacing)) out.push(t);
  return out;
}

export const hearths = (s) => spread(Object.values(s.map.tiles).filter((t) => t.tame === 1), TENDING.hearthSpacing);

// Each player takes a hearth: the most tamed grain first, then the longest-held land.
// Their market is a gathering already near it, or a new one on it. Gatherings still going
// become permanent, and the busiest settled sites left get markets of their own.
export function startGame(s) {
  const n = hearths(s).length;
  s.phase = 'game';
  const sites = spread(Object.values(s.map.tiles).filter((t) => t.claim != null)
    .sort((a, b) => (b.tame ?? 0) - (a.tame ?? 0) || b.occTotal - a.occTotal), TENDING.hearthSpacing);
  for (const h of sites) {
    const player = s.players.find((p) => p.market == null);
    if (!player) break;
    const near = Object.values(s.markets).filter((m) => distance(m, h) < MARKET.spacing);
    if (near.some((m) => m.owner != null)) continue;
    const m = near.sort((a, b) => distance(a, h) - distance(b, h))[0] ?? foundMarket(s, h);
    m.owner = player.id;
    player.market = m.id;
  }
  for (const m of Object.values(s.markets)) m.temporary = false;
  const rest = settledSites(s).map((t) => ({ t, ...catchment(s, t) })).sort((a, b) => b.pop - a.pop);
  for (const { t } of rest) if (farFromMarkets(s, t)) foundMarket(s, t);
  s.events.push(`Turn ${s.turn}: ${n ? `grain is farmed at ${n} hearth${n > 1 ? 's' : ''}` : 'the pre-game runs out'}; ` +
    `${Object.keys(s.markets).length} markets and the game begins`);
}

const pregameOver = (s) => s.turn >= PREGAME.maxTurns || (s.turn >= PREGAME.minTurns && hearths(s).length >= s.players.length);

// Order matters: per-turn steps run top to bottom.
export const TURN = [
  regrow,
  (s) => forage(s, reaches(s)),
  trade,
  (s) => settle(s, reaches(s)),
  occupy,
  formMarkets,
  (s) => {
    s.stats.pop.push(totalPop(s));
    s.stats.clans.push(Object.keys(s.clans).length);
    s.stats.volume.push(Object.values(s.markets).reduce((a, m) => a + m.lastVolume, 0));
  },
  (s) => { if (s.phase === 'pregame' && pregameOver(s)) startGame(s); },
];

export const ACTIONS = {
  endTurn: {
    validate: () => null,
    apply: (s) => {
      for (const step of TURN) step(s);
      s.turn++;
    },
    candidates: () => [{ type: 'endTurn' }],
  },
};

export function validate(s, a) {
  const def = ACTIONS[a.type];
  return def ? def.validate(s, a) : `unknown action ${a.type}`;
}

export function dispatch(s, a) {
  const error = validate(s, a);
  if (error) return { state: s, error };
  const next = structuredClone(s);
  next.events = [];
  ACTIONS[a.type].apply(next, a);
  return { state: next };
}

export const legalActions = (s) =>
  Object.entries(ACTIONS).flatMap(([, def]) => def.candidates(s)).filter((a) => !validate(s, a));
