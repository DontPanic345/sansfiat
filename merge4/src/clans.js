// Clan rules: the hex-economy pre-game, which keeps running for every clan in the main game.
// Turn steps here are wired into the phase pipeline by rules.js. Modules may import the
// exported helpers (spawnClan, siteOptions, marketReach, ...) but should not edit this file.
import { CIVS, CLAN, FOOD, FOODS, FOUNDING, MARKET, NAME_SYLLABLES, PREGAME, TENDING, TERRAIN } from './data.js';
import { distance, key, neighbors } from './hex.js';
import { clear, demand, utility } from './market.js';
import { rand } from './rng.js';
import { addGoods as add, clanAt, costMap, event, newId, note, passable, tileAt, total } from './core.js';

export const footprint = (s, h) => [h, ...neighbors(h)].map((n) => tileAt(s, n)).filter(Boolean);
const pct = (x) => `${Math.round(x * 100)}%`;
const fmt = (x) => (x >= 10 ? x.toFixed(0) : x.toFixed(1));

// A food counts in full up to dietShare of need, and at excessValue beyond that. Only foods
// feed: materials and crafts in the basket count for nothing.
export function nutrition(basket, need) {
  const k = CLAN.dietShare * need;
  return FOODS.reduce((n, f) => {
    const v = basket[f] ?? 0;
    return n + Math.min(v, k) + CLAN.excessValue * Math.max(0, v - k);
  }, 0);
}

// Path cost over land from the market, out to MARKET.reach. Trails count, so a trail widens
// a market's catchment.
export const marketReach = (s, m) => costMap(s, m, MARKET.reach);

export const reaches = (s) => Object.values(s.markets).map((m) => ({ market: m, cost: marketReach(s, m) }));

export function nearestMarket(rs, h) {
  let best = null;
  for (const { market, cost } of rs) {
    const c = cost.get(key(h));
    if (c != null && (!best || c < best.cost)) best = { market, cost: c };
  }
  return best;
}

// A tile key followed by its neighbours' keys (whether or not on the map), cached: range()
// runs thousands of times a turn.
const AROUND = new Map();
function around(k) {
  let ks = AROUND.get(k);
  if (!ks) {
    const [q, r] = k.split(',').map(Number);
    AROUND.set(k, ks = [k, ...neighbors({ q, r }).map(key)]);
  }
  return ks;
}

// The tiles a clan holding `land` forages: the land and every tile next to it nobody else holds.
export function range(s, land, id) {
  const out = new Map();
  for (const k of land)
    for (const n of around(k)) {
      const t = s.map.tiles[n];
      if (t && (t.claim == null || t.claim === id)) out.set(n, t);
    }
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
export function siteOptions(s, clan, rs, { steps = CLAN.moveRange, pop = clan.pop } = {}) {
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
export function pickSite(s, options, min) {
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

// Only foods are eaten. Anything else in the basket (a clan should not hold materials, but a
// module might hand it some) is kept in store as it is, without spoiling.
function eat(clan) {
  const need = clan.pop;
  const k = CLAN.dietShare * need;
  const left = {};
  const other = {};
  let got = 0;
  for (const [f, v] of Object.entries(clan.basket)) {
    if (!FOOD[f]) { if (v > 0.01) other[f] = v; continue; }
    got += Math.min(v, k);
    left[f] = Math.max(0, v - k);
  }
  const extra = total(left);
  const eaten = Math.min(extra, Math.max(0, need - got) / CLAN.excessValue);
  got += eaten * CLAN.excessValue;
  const keep = extra ? 1 - eaten / extra : 0;
  clan.store = {};
  for (const [f, v] of Object.entries(left)) if (v * keep > 0.01) clan.store[f] = v * keep * (1 - CLAN.spoilage);
  Object.assign(clan.store, other);
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
// owner: civ id or null (unaligned). leaning: {civId: number}, built by the civ module.
// A daughter inherits her parent's owner and leaning (pass `parent`).
export function spawnClan(s, hex, pop, store = {}, land = [key(hex)], parent = null) {
  const id = newId(s);
  const clan = s.clans[id] = {
    id, name: clanName(s), q: hex.q, r: hex.r, pop, store, harvest: {}, basket: {}, fed: true, starving: false,
    fedRatio: 1, hungry: 0, route: [], dest: null, blocked: 0, trail: [], market: null, trip: 0, land: [], since: 0,
    owner: parent?.owner ?? null, leaning: { ...(parent?.leaning ?? {}) },
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

// A fallow tile (foraged or pillaged by troops) regrows nothing until its count runs out.
// hooks.yieldFactor(s, tile), if a module supplies one, scales regrowth (good and bad years).
export function regrow(s, { hooks } = {}) {
  const factor = hooks?.yieldFactor;
  for (const t of Object.values(s.map.tiles)) {
    if (t.fallow > 0) { t.fallow--; continue; }
    const k = factor ? factor(s, t) : 1;
    for (const [f, cap] of Object.entries(t.cap)) {
      const { regen, breeds, restock } = FOOD[f];
      const now = t.stock[f] ?? 0;
      const grow = breeds ? cap * restock + regen * now * (1 - now / cap) : cap * regen;
      t.stock[f] = Math.min(cap, now + grow * k);
    }
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
  if (!clan.route.length) {
    Object.assign(clan, { dest: null, hungry: 0 });
    note(s, clan.id, `settled at ${next}`);
  }
}

// Leaves its land for `dest`, which it claims at once so no one else settles there.
function depart(s, clan, prev, dest) {
  releaseAll(s, clan);
  claimTile(s, clan, dest);
  Object.assign(clan, { dest, route: routeTo(prev, dest), since: 0 });
}

// A clan that gives up on a walk stays where it stands, if no one holds that tile.
function stopWalking(s, clan) {
  note(s, clan.id, `gave up walking to ${clan.dest}: the way was blocked; stays at ${key(clan)}`);
  releaseAll(s, clan);
  Object.assign(clan, { route: [], dest: null, blocked: 0 });
}

// Share of a clan's people working its land. clan.drafted (people, 0..pop) is labour a civ
// has taken for its projects this turn; the civ module sets it before the forage phase. The
// drafted still eat (eat() uses the whole pop).
const workShare = (clan) => 1 - Math.min(clan.pop, Math.max(0, clan.drafted ?? 0)) / clan.pop;

export function forage(s, rs) {
  const occupied = new Set(Object.values(s.clans).map(key));
  for (const clan of shuffled(s, Object.values(s.clans))) {
    const moving = clan.route.length > 0;
    if (!moving && clan.hungry >= CLAN.patience) {
      const { prev, options, here } = siteOptions(s, clan, rs);
      const dest = pickSite(s, options, here + CLAN.moveMargin);
      if (dest) {
        const best = options.find((o) => o.k === dest);
        note(s, clan.id, `left ${key(clan)} for ${dest} after starving ${clan.hungry} turns: ` +
          `that site scores ${pct(best.score)} against ${pct(here)} here`);
        depart(s, clan, prev, dest);
        event(s, `${clan.name} leaves ${key(clan)} for ${dest}`);
      } else note(s, clan.id, `starving ${clan.hungry} turns but no site within ${CLAN.moveRange} steps is better`);
    }
    if (clan.route.length) walkOn(s, clan, occupied);
    else if (!moving) {
      adjustLand(s, clan);
      if (clan.trail.length) clan.trail.shift();
    }

    const near = nearestMarket(rs, clan);
    const trip = near ? near.cost * MARKET.tripCost : 0;
    const go = near && worthTrading(add(clan.store, clan.harvest), trip, near.market.prices);
    const was = clan.market;
    clan.market = go ? near.market.id : null;
    if (clan.market !== was) note(s, clan.id, go
      ? `goes to market at ${key(near.market)}: a more varied basket beats the trip (${pct(trip)} of labour)`
      : near ? `stops trading at ${key(near.market)}: the trip (${pct(trip)} of labour) costs more than trade gains`
      : 'no market in reach');
    clan.trip = go ? trip : 0;
    clan.harvest = harvest(s, clan, (1 - clan.trip) * workShare(clan));
    clan.basket = add(clan.store, clan.harvest);
    clan.store = {};
  }
}

// Clans trading at a market post their baskets. Any other trader (a civ's store) posts by
// writing m.posts[traderId] = basket before this step, and may write m.wants[traderId] = [good]
// (the goods it will take in exchange; default every good); it gets m.received[traderId] =
// basket after it, and m.posts and m.wants are cleared. Goods are conserved across all of them.
export function trade(s) {
  for (const m of Object.values(s.markets)) {
    const traders = Object.values(s.clans).filter((c) => c.market === m.id);
    const posts = m.posts ?? {};
    const baskets = Object.fromEntries(traders.map((c) => [c.id, c.basket]));
    Object.assign(baskets, posts);
    const before = m.prices;
    // Clans want only foods; another poster may limit what it buys with m.wants[traderId].
    const wants = { ...Object.fromEntries(traders.map((c) => [c.id, FOODS])), ...(m.wants ?? {}) };
    const res = clear(baskets, m.prices, { wants });
    for (const c of traders) c.basket = res.after[c.id];
    m.received = Object.fromEntries(Object.keys(posts).map((id) => [id, res.after[id]]));
    m.posts = {};
    delete m.wants;
    const moved = Object.keys(res.cleared ? res.prices : {}).filter((g) => before[g])
      .map((g) => ({ g, d: Math.log(res.prices[g] / before[g]) })).sort((a, b) => Math.abs(b.d) - Math.abs(a.d))[0];
    // Without a real clearing (fewer than 2 traders or goods) the last real ratios stand.
    if (res.cleared) Object.assign(m.prices, res.prices);
    if (Object.keys(baskets).length >= 2)
      note(s, m.id, `${Object.keys(baskets).length} traders swapped ${fmt(total(res.volume))} goods` +
        (moved && Math.abs(moved.d) > 0.02 ? `; ${moved.g} ${moved.d > 0 ? 'rose' : 'fell'} ${pct(Math.abs(Math.exp(moved.d) - 1))}` : ''));
    m.traders = traders.length + Object.keys(posts).length;
    m.lastVolume = total(res.volume);
    m.lastTrade = res.volume;
    m.lastTraders = [...traders.map((c) => c.name), ...Object.keys(posts).map((id) => s.civs[id]?.name ?? `trader ${id}`)];
    m.volume += m.lastVolume;
    m.activity = MARKET.gathering.fade * m.activity + (1 - MARKET.gathering.fade) * m.lastVolume;
    m.history.push({ turn: s.turn, prices: { ...m.prices }, volume: m.lastVolume });
    if (m.history.length > MARKET.history) m.history.shift();
  }
}

// The clan divides like a cell: its land splits down the middle, and the daughter takes the
// half away from home with half the people and store.
function divide(s, clan) {
  const [a, b] = halve(s, clan.land);
  const [keep, give] = a.includes(key(clan)) ? [a, b] : [b, a];
  const free = give.filter((k) => !clanAt(s, s.map.tiles[k]));
  const home = free.length ? centre(s, free, null) : null;
  if (!keep.length || !home) return false;
  for (const k of give) releaseTile(s, clan, k);
  clan.pop /= 2;
  clan.store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v / 2]));
  const daughter = spawnClan(s, home, clan.pop, { ...clan.store }, give, clan);
  daughter.since = clan.since;
  note(s, clan.id, `divided at ${fmt(clan.pop * 2)} people (splits at ${CLAN.splitAt}); ${daughter.name} took the far ${give.length} tiles`);
  note(s, daughter.id, `born when ${clan.name} divided its land`);
  event(s, `${clan.name} divides; ${daughter.name} takes ${give.length} of its tiles`);
  return true;
}

// A daughter takes half the clan, or half of splitAt from a big one, and walks off to a site
// of her own, starting next door. She only goes if some site in reach would feed her.
// hooks.daughterSite(s, clan, {options, pop}) may pick the site instead (a key from options),
// or return null to keep her home; undefined falls back to the clans' own choice.
function split(s, clan, rs, hooks) {
  const pop = Math.min(clan.pop, CLAN.splitAt) / 2;
  const { prev, options } = siteOptions(s, clan, rs, { steps: CLAN.scoutRange, pop });
  const feeding = options.filter((o) => o.fed >= CLAN.settleFed);
  const chosen = hooks?.daughterSite?.(s, clan, { options: feeding, pop });
  const dest = chosen !== undefined ? (feeding.some((o) => o.k === chosen) ? chosen : null) : pickSite(s, feeding, -Infinity);
  if (!dest) {
    if (clan.starving) note(s, clan.id, `starving at ${fmt(clan.pop)} people, but no site within ${CLAN.scoutRange} steps would feed a daughter`);
    return;
  }
  const route = routeTo(prev, dest);
  const share = pop / clan.pop;
  const store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v * share]));
  clan.store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v * (1 - share)]));
  clan.pop -= pop;
  clan.hungry = 0;
  const daughter = spawnClan(s, s.map.tiles[route[0]], pop, store, [dest], clan);
  daughter.trail = [key(clan)];
  if (route.length > 1) Object.assign(daughter, { route: route.slice(1), dest });
  note(s, clan.id, `sent ${daughter.name} (${fmt(pop)} people) to ${dest}: ` +
    (clan.starving ? `too many mouths while starving` : `grown past ${CLAN.splitAt} with no land to divide`));
  note(s, daughter.id, `split from ${clan.name}; heading for ${dest}, which would feed her`);
  event(s, `${clan.name} splits; ${daughter.name} heads for ${dest}`);
}

// Why a clan went hungry: too little food, or too little variety (one food counts in full
// only up to CLAN.dietShare of need).
function hungerReason(clan) {
  const got = FOODS.reduce((a, f) => a + (clan.basket[f] ?? 0), 0);
  const foods = Object.entries(clan.basket).filter(([f, v]) => FOOD[f] && v > 0.05).sort((a, b) => b[1] - a[1]);
  const top = foods[0];
  if (got < clan.pop) return `had ${fmt(got)} food for ${fmt(clan.pop)} people`;
  if (top && top[1] > CLAN.dietShare * clan.pop * 1.5)
    return `${fmt(got)} food for ${fmt(clan.pop)} people, but mostly ${top[0]}; past ${pct(CLAN.dietShare)} of need one food counts at ${pct(CLAN.excessValue)}`;
  return `${fmt(got)} food for ${fmt(clan.pop)} people did not cover the diet`;
}

export function settle(s, rs, hooks) {
  for (const clan of Object.values(s.clans)) {
    eat(clan);
    clan.hungry = clan.starving ? clan.hungry + 1 : 0;
    if (clan.starving) note(s, clan.id, `starving (${pct(clan.fedRatio)} fed): ${hungerReason(clan)}`);
    if (clan.pop < 1) {
      releaseAll(s, clan);
      delete s.clans[clan.id];
      event(s, `${clan.name} dies out`);
      continue;
    }
    const settled = !clan.route.length;
    if (settled) clan.since++;
    if (!settled) continue;
    if (clan.pop >= CLAN.splitAt) {
      if (clan.land.length < 2 || !divide(s, clan)) split(s, clan, rs, hooks);
    } else if (clan.starving && clan.pop >= CLAN.hungrySplit) split(s, clan, rs, hooks);
  }
}

// Held land counts as occupied. Grain on land held long enough is tended toward domestication,
// and goes feral again when left.
export function occupy(s) {
  for (const [k, t] of Object.entries(s.map.tiles)) {
    if (t.claim != null) { t.occ++; t.occTotal++; }
    else t.occ = 0;
    if (t.wildGrain == null) continue;
    const was = t.tame;
    const step = t.claim == null ? -TENDING.rate : t.occ >= TENDING.tendAfter ? TENDING.rate : 0;
    t.tame = Math.max(0, Math.min(1, t.tame + step));
    if (t.tame === was) continue;
    t.cap.grain = Math.round(t.wildGrain * (1 + (TENDING.domestic - 1) * t.tame) * 10) / 10;
    if (t.tame === 1) event(s, `grain is domesticated at ${k}`);
  }
}

function farFromMarkets(s, h) {
  return Object.values(s.markets).every((m) => distance(m, h) >= MARKET.spacing);
}

export function foundMarket(s, h) {
  const id = newId(s);
  const temporary = s.phase === 'pregame';
  s.markets[id] = {
    id, q: h.q, r: h.r, founded: s.turn, owner: null, temporary, prices: {}, volume: 0, lastVolume: 0, lastTrade: {},
    traders: 0, activity: 2 * MARKET.gathering.minVolume, history: [], posts: {}, received: {},
  };
  event(s, `${temporary ? 'clans gather to trade' : 'a market forms'} at ${key(h)}`);
  return s.markets[id];
}

function removeMarket(s, m, why) {
  for (const c of Object.values(s.clans)) if (c.market === m.id) c.market = null;
  delete s.markets[m.id];
  event(s, `${why} at ${key(m)}`);
}

// Clans living within market reach of h, and their people.
export function catchment(s, h) {
  const reach = marketReach(s, h);
  const near = Object.values(s.clans).filter((c) => reach.has(key(c)));
  return { clans: near.length, pop: near.reduce((a, c) => a + c.pop, 0) };
}

const settledSites = (s) => Object.values(s.clans).filter((c) => !c.route.length).map((c) => tileAt(s, c))
  .filter((t) => t.occ >= MARKET.formAfter && farFromMarkets(s, t));

// A market forms where at least gathering.minClans clans live within reach: in the pre-game where
// the most people live, in the main game on the longest-held such site. Any market no civ owns
// (a pre-game gathering, or a neutral market in the main game) fades once its traded volume,
// smoothed with weight gathering.fade on the past, falls below gathering.minVolume.
// In the main game a market that fell quiet does not re-form on (or next to) its site the same turn.
export function formMarkets(s) {
  const faded = [];
  for (const m of Object.values(s.markets))
    if (m.owner == null && m.activity < MARKET.gathering.minVolume) {
      removeMarket(s, m, m.temporary ? 'the gathering disperses' : 'the market falls quiet');
      faded.push(m);
    }
  const sites = settledSites(s).filter((t) => s.phase !== 'main' || faded.every((m) => distance(m, t) >= MARKET.spacing));
  const site = s.phase === 'pregame'
    ? sites.map((t) => ({ t, ...catchment(s, t) }))
      .filter((x) => x.clans >= MARKET.gathering.minClans).sort((a, b) => b.pop - a.pop)[0]?.t
    : sites.sort((a, b) => b.occ - a.occ).find((t) => catchment(s, t).clans >= MARKET.gathering.minClans);
  if (site) foundMarket(s, site);
}

// Picks tiles in order, each at least `spacing` from those already picked.
function spread(tiles, spacing) {
  const out = [];
  for (const t of tiles) if (out.every((o) => distance(o, t) >= spacing)) out.push(t);
  return out;
}

export const hearths = (s) => spread(Object.values(s.map.tiles).filter((t) => t.tame === 1), TENDING.hearthSpacing);

// Land that is passable, or would be once a trail or bridge is built (a mountain with a span
// has a trailCost; forest and hills are passable already). Returns {comp: Map(tile key ->
// component id), sizes: [tiles per component]}.
export function landComponents(s) {
  const comp = new Map();
  const sizes = [];
  const open = (t) => t && (passable(s, t) || t.trailCost != null);
  for (const t of Object.values(s.map.tiles)) {
    if (comp.has(key(t)) || !open(t)) continue;
    const id = sizes.length;
    const stack = [t];
    comp.set(key(t), id);
    let n = 0;
    while (stack.length) {
      const h = stack.pop();
      n++;
      for (const x of neighbors(h)) {
        const nt = tileAt(s, x);
        if (open(nt) && !comp.has(key(nt))) { comp.set(key(nt), id); stack.push(nt); }
      }
    }
    sizes.push(n);
  }
  return { comp, sizes };
}

// A market a civ could be founded on: busy enough to call thriving.
const thriving = (m) => m.owner == null && m.traders >= FOUNDING.minTraders && m.activity >= FOUNDING.minActivity;

// Greedily adds sites from `cands` ([{market|null, tiles: [hearth candidates]}]) to `picked`
// until it holds `want`: markets FOUNDING.spacing apart, hearths TENDING.hearthSpacing apart.
// A candidate without a market will get a new one on its hearth.
function addSites(picked, cands, want) {
  for (const c of cands) {
    if (picked.length >= want) return;
    if (c.market && picked.some((p) => p.market?.id === c.market.id)) continue;
    const hearth = c.tiles.find((h) => picked.every((p) => distance(p.hearth, h) >= TENDING.hearthSpacing));
    if (!hearth) continue;
    const at = c.market ?? hearth;
    if (picked.some((p) => distance(p.market ?? p.hearth, at) < FOUNDING.spacing)) continue;
    picked.push({ market: c.market ?? null, hearth });
  }
}

const byTame = (a, b) => (b.tame ?? 0) - (a.tame ?? 0) || b.occTotal - a.occTotal || a.q - b.q || a.r - b.r;

// The founding sites: one per player, each a thriving market with a hearth (fully domesticated
// grain) in its reach, markets FOUNDING.spacing apart and hearths TENDING.hearthSpacing apart,
// all on one landmass (counting land a trail or bridge would open) so every civ can reach every
// other. Returns [{market, hearth}] for the landmass that holds the most. With `fallback` (the
// pre-game ran out) it tops that landmass up with the best-tended held tiles, on a market that
// reaches them or a new one (market: null), then any fertile tile there, and if it still falls
// short takes other landmasses.
export function foundingSites(s, { fallback = false } = {}) {
  const want = s.config.players;
  const tame = Object.values(s.map.tiles).filter((t) => t.tame === 1);
  if (!fallback && tame.length < want) return [];
  const markets = Object.values(s.markets).filter(thriving).map((m) => {
    const close = tame.filter((t) => distance(t, m) <= MARKET.reach); // a step costs at least 1
    const reach = close.length ? marketReach(s, m) : null;
    const tiles = close.filter((t) => reach.has(key(t))).sort(byTame);
    return { market: m, tiles, score: m.activity + m.traders };
  }).filter((c) => c.tiles.length).sort((a, b) => b.score - a.score || a.market.id - b.market.id);
  if (!fallback && markets.length < want) return [];
  const { comp, sizes } = landComponents(s);
  for (const c of markets) c.comp = comp.get(key(c.market));
  const groups = sizes.map((size, id) => {
    const picked = [];
    addSites(picked, markets.filter((c) => c.comp === id), want);
    return { id, size, picked };
  }).sort((a, b) => b.picked.length - a.picked.length || b.size - a.size);
  if (!fallback || !groups.length) return groups[0]?.picked ?? [];

  const free = Object.values(s.markets).filter((m) => m.owner == null).map((m) => ({ m, reach: marketReach(s, m) }));
  const held = Object.values(s.map.tiles).filter((t) => t.claim != null).sort(byTame).map((t) => {
    const near = free.filter((x) => x.reach.has(key(t))).sort((a, b) => a.reach.get(key(t)) - b.reach.get(key(t)))[0];
    return { market: near?.m ?? null, tiles: [t], comp: comp.get(key(t)) };
  });
  // Last resort: any fertile land, richest first.
  const open = Object.values(s.map.tiles).filter((t) => passable(s, t) && total(t.cap) > 0)
    .sort((a, b) => total(b.cap) - total(a.cap) || a.q - b.q || a.r - b.r)
    .map((t) => ({ market: null, tiles: [t], comp: comp.get(key(t)) }));
  const picked = groups[0].picked;
  for (const g of groups) {
    if (g !== groups[0]) addSites(picked, markets.filter((c) => c.comp === g.id), want);
    addSites(picked, held.filter((c) => c.comp === g.id), want);
    if (g === groups[0]) addSites(picked, open.filter((c) => c.comp === g.id), want);
  }
  return picked;
}

// Ends the pre-game. Each civ takes a founding site (foundingSites): its market, with a hearth in
// reach. Gatherings still going become permanent markets, and settled sites far from any market
// with gathering.minClans clans in reach get markets of their own.
// Creates s.civs (all AI until the player picks one with chooseCiv) and moves to phase 'choose'.
// What a civ *is* beyond this record (its clans, first city, store) is set up by the civ
// module's onStart once the main game begins.
export function foundCivs(s) {
  const n = hearths(s).length;
  const sites = foundingSites(s, { fallback: true });
  s.phase = 'choose';
  for (const { market, hearth } of sites) {
    let m = market;
    if (!m) {
      for (const x of Object.values(s.markets))
        if (x.owner == null && distance(x, hearth) < MARKET.spacing) removeMarket(s, x, 'the gathering moves on');
      m = foundMarket(s, hearth);
    }
    const look = CIVS[Object.keys(s.civs).length % CIVS.length];
    const id = newId(s);
    s.civs[id] = {
      id, name: look.name, color: look.color, isHuman: false, marketId: m.id, cityIds: [], store: {},
      seen: {}, revealed: {}, founded: s.turn,
    };
    m.owner = id;
    const grain = hearth.tame === 1 ? 'grain domesticated' : hearth.tame > 0 ? `grain ${pct(hearth.tame)} tamed` : 'land held';
    note(s, id, `founded at the market ${key(m)}, near ${grain} at ${key(hearth)}`);
  }
  for (const m of Object.values(s.markets)) m.temporary = false;
  const rest = settledSites(s).map((t) => ({ t, ...catchment(s, t) }))
    .filter((x) => x.clans >= MARKET.gathering.minClans).sort((a, b) => b.pop - a.pop);
  for (const { t } of rest) if (farFromMarkets(s, t)) foundMarket(s, t);
  event(s, `${n ? `grain is farmed at ${n} hearth${n > 1 ? 's' : ''}` : 'the pre-game runs out'}; ` +
    `${Object.keys(s.civs).length} civs and ${Object.keys(s.markets).length} markets; choose a civ to lead`);
}

// The pre-game ends once there is a founding site per player (see foundingSites), or at maxTurns.
export const pregameOver = (s) => s.turn >= PREGAME.maxTurns
  || (s.turn >= PREGAME.minTurns && foundingSites(s).length >= s.config.players);
