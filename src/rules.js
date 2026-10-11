import { CLAN, FOOD, FOODS, MARKET, NAME_SYLLABLES, PREGAME, TERRAIN } from './data.js';
import { distance, key, neighbors } from './hex.js';
import { clear, demand, utility } from './market.js';
import { rand } from './rng.js';

export const tileAt = (s, h) => s.map.tiles[key(h)];
export const passable = (s, h) => !!tileAt(s, h) && TERRAIN[tileAt(s, h).terrain].passable;
export const clanAt = (s, h) => Object.values(s.clans).find((c) => c.q === h.q && c.r === h.r);
export const marketAt = (s, h) => Object.values(s.markets).find((m) => m.q === h.q && m.r === h.r);
export const footprint = (s, h) => [h, ...neighbors(h)].map((n) => tileAt(s, n)).filter(Boolean);
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

// How much of each food a clan would take from the footprint around h. It fills each
// food up to its diet share first, then takes the rest in proportion to what is there.
export function planHarvest(s, h, pop, labourShare, store = {}) {
  const avail = {};
  for (const t of footprint(s, h))
    for (const [f, v] of Object.entries(t.stock)) avail[f] = (avail[f] ?? 0) + v * CLAN.takeRate;
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
  const plan = planHarvest(s, clan, clan.pop, labourShare, clan.store);
  const tiles = footprint(s, clan);
  for (const [f, want] of Object.entries(plan)) {
    const there = tiles.reduce((a, t) => a + (t.stock[f] ?? 0), 0);
    if (there > 0) for (const t of tiles) if (t.stock[f]) t.stock[f] -= want * (t.stock[f] / there);
  }
  return plan;
}

const siteFed = (s, h, pop, store) => nutrition(add(store, planHarvest(s, h, pop, 1, store)), pop) / pop;

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

// Scores every free tile the clan could walk to, none closer than `spacing` to another
// clan or its destination.
function siteOptions(s, clan, rs, { includeHere = true, range = CLAN.moveRange, pop = clan.pop } = {}) {
  const others = Object.values(s.clans).filter((c) => c.id !== clan.id);
  const claimed = others.flatMap((c) => (c.dest ? [c, s.map.tiles[c.dest]] : [c]));
  const prev = walkable(s, clan, new Set(others.map(key)), range);
  const option = (h) => {
    const fed = siteFed(s, h, pop, clan.store);
    const score = fed + (nearestMarket(rs, h) ? MARKET.accessBonus : 0);
    return { k: key(h), fed, score };
  };
  const options = [];
  for (const k of prev.keys()) {
    const h = s.map.tiles[k];
    if (k === key(clan) ? !includeHere : claimed.some((c) => distance(c, h) < CLAN.spacing)) continue;
    options.push(option(h));
  }
  return { prev, options, here: option(clan).score };
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
  clan.pop *= clan.fed ? 1 + CLAN.growth : 1 - CLAN.starve * (1 - clan.fedRatio);
}

export function clanName(s) {
  const n = rand(s) < 0.3 ? 3 : 2;
  const name = Array.from({ length: n }, () => NAME_SYLLABLES[Math.floor(rand(s) * NAME_SYLLABLES.length)]).join('');
  return name[0].toUpperCase() + name.slice(1);
}

export function spawnClan(s, hex, pop, store = {}) {
  const id = s.nextId++;
  s.clans[id] = {
    id, name: clanName(s), q: hex.q, r: hex.r, pop, store, harvest: {}, basket: {}, fed: true, fedRatio: 1,
    hungry: 0, route: [], dest: null, blocked: 0, trail: [], market: null, trip: 0,
  };
  return s.clans[id];
}

function regrow(s) {
  for (const t of Object.values(s.map.tiles))
    for (const [f, cap] of Object.entries(t.cap)) t.stock[f] = Math.min(cap, t.stock[f] + cap * FOOD[f].regen);
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
    if (++clan.blocked >= 2) Object.assign(clan, { route: [], dest: null, blocked: 0 });
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

function forage(s, rs) {
  const occupied = new Set(Object.values(s.clans).map(key));
  for (const clan of shuffled(s, Object.values(s.clans))) {
    const moving = clan.route.length > 0;
    if (!moving && clan.hungry >= CLAN.patience) {
      const { prev, options, here } = siteOptions(s, clan, rs);
      const dest = pickSite(s, options.filter((o) => o.k !== key(clan)), here + CLAN.moveMargin);
      if (dest) {
        Object.assign(clan, { dest, route: routeTo(prev, dest) });
        s.events.push(`Turn ${s.turn}: ${clan.name} leaves ${key(clan)} for ${dest}`);
      }
    }
    if (clan.route.length) walkOn(s, clan, occupied);
    else if (!moving && clan.trail.length) clan.trail.shift();

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
    m.history.push({ turn: s.turn, prices: { ...res.prices }, volume: m.lastVolume });
    if (m.history.length > MARKET.history) m.history.shift();
  }
}

// A daughter takes half the clan and walks off to a site of her own, starting next door.
// She only goes if some site in reach would feed her.
function split(s, clan, rs) {
  const { prev, options } = siteOptions(s, clan, rs, { includeHere: false, range: CLAN.scoutRange, pop: clan.pop / 2 });
  const dest = pickSite(s, options.filter((o) => o.fed >= CLAN.settleFed), -Infinity);
  if (!dest) return;
  const route = routeTo(prev, dest);
  clan.pop /= 2;
  clan.store = Object.fromEntries(Object.entries(clan.store).map(([f, v]) => [f, v / 2]));
  clan.hungry = 0;
  const daughter = spawnClan(s, s.map.tiles[route[0]], clan.pop, { ...clan.store });
  daughter.trail = [key(clan)];
  if (route.length > 1) Object.assign(daughter, { route: route.slice(1), dest });
  s.events.push(`Turn ${s.turn}: ${clan.name} splits; ${daughter.name} heads for ${dest}`);
}

function settle(s, rs) {
  for (const clan of Object.values(s.clans)) {
    eat(clan);
    clan.hungry = clan.fed ? 0 : clan.hungry + 1;
    if (clan.pop < 1) {
      delete s.clans[clan.id];
      s.events.push(`Turn ${s.turn}: ${clan.name} dies out`);
      continue;
    }
    const settled = !clan.route.length;
    if (settled && (clan.pop >= CLAN.splitAt || (!clan.fed && clan.pop >= CLAN.hungrySplit))) split(s, clan, rs);
  }
}

function occupy(s) {
  const here = new Set(Object.values(s.clans).map(key));
  for (const [k, t] of Object.entries(s.map.tiles)) {
    if (here.has(k)) { t.occ++; t.occTotal++; }
    else t.occ = 0;
  }
}

function farFromMarkets(s, h) {
  return Object.values(s.markets).every((m) => distance(m, h) >= MARKET.spacing);
}

export function foundMarket(s, h) {
  const id = s.nextId++;
  s.markets[id] = { id, q: h.q, r: h.r, founded: s.turn, owner: null, prices: {}, volume: 0, lastVolume: 0, lastTrade: {}, traders: 0, history: [] };
  s.events.push(`Turn ${s.turn}: a market forms at ${key(h)}`);
  return s.markets[id];
}

function formMarkets(s) {
  const site = Object.values(s.clans).map((c) => tileAt(s, c))
    .filter((t) => t.occ >= MARKET.formAfter && farFromMarkets(s, t))
    .sort((a, b) => b.occ - a.occ)[0];
  if (site) foundMarket(s, site);
}

// The busiest markets, by how long their site has been lived on × goods traded, become civs.
export function startGame(s) {
  const score = (m) => tileAt(s, m).occTotal * (m.volume + 1);
  const chosen = Object.values(s.markets).sort((a, b) => score(b) - score(a)).slice(0, s.players.length);
  while (chosen.length < s.players.length) {
    const site = Object.values(s.map.tiles).filter((t) => t.occTotal > 0 && farFromMarkets(s, t))
      .sort((a, b) => b.occTotal - a.occTotal)[0];
    if (!site) break;
    chosen.push(foundMarket(s, site));
  }
  chosen.forEach((m, i) => {
    m.owner = i;
    s.players[i].market = m.id;
  });
  s.phase = 'game';
  s.events.push(`Turn ${s.turn}: the game begins`);
}

function pregameSettled(s) {
  const pop = s.stats.pop;
  if (s.turn >= PREGAME.maxTurns) return true;
  if (s.turn < PREGAME.minTurns || pop.length <= PREGAME.window) return false;
  return pop.at(-1) / pop.at(-1 - PREGAME.window) - 1 < PREGAME.minGrowth;
}

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
  (s) => { if (s.phase === 'pregame' && pregameSettled(s)) startGame(s); },
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
