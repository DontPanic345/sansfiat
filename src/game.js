import { CLAN, MAP, PLAYERS } from './data.js';
import { key, neighbors } from './hex.js';
import { generateMap, isLand } from './mapgen.js';
import { pick } from './rng.js';
import { dispatch, spawnClan } from './rules.js';

export function newGame({ seed = 1, players = 2, width = MAP.width, height = MAP.height } = {}) {
  const s = {
    config: { seed, players, width, height },
    rng: seed >>> 0,
    turn: 1,
    phase: 'pregame',
    nextId: 1,
    events: [],
    players: PLAYERS.slice(0, players).map((p, id) => ({ id, ...p, market: null })),
    clans: {},
    markets: {},
    stats: { pop: [], clans: [], volume: [] },
  };
  s.map = generateMap(s, width, height);
  seedClans(s);
  return s;
}

// One clan in every sizeable landmass, then the rest anywhere.
function seedClans(s) {
  const fertile = (t) => isLand(t) && Object.keys(t.cap).length;
  const regions = landRegions(s.map.tiles).filter((r) => r.length >= MAP.minRegion);
  const starts = regions.map((r) => pick(s, r.filter(fertile)));
  const land = Object.values(s.map.tiles).filter(fertile);
  while (starts.length < CLAN.seeds) starts.push(pick(s, land));
  for (const t of new Set(starts)) spawnClan(s, t, CLAN.startPop);
}

export function landRegions(tiles) {
  const seen = new Set();
  const regions = [];
  for (const t of Object.values(tiles)) {
    if (!isLand(t) || seen.has(key(t))) continue;
    const region = [];
    const stack = [t];
    seen.add(key(t));
    while (stack.length) {
      const h = stack.pop();
      region.push(h);
      for (const n of neighbors(h)) {
        const x = tiles[key(n)];
        if (x && isLand(x) && !seen.has(key(x))) { seen.add(key(x)); stack.push(x); }
      }
    }
    regions.push(region);
  }
  return regions;
}

export function replay(config, actions) {
  let s = newGame(config);
  for (const a of actions) {
    const r = dispatch(s, a);
    if (r.error) throw new Error(`replay failed at ${JSON.stringify(a)}: ${r.error}`);
    s = r.state;
  }
  return s;
}

// Steps until the pre-game hands over to the game, or `maxTurns` more turns pass.
export function runPregame(s, maxTurns = Infinity) {
  for (let i = 0; i < maxTurns && s.phase === 'pregame'; i++) s = dispatch(s, { type: 'endTurn' }).state;
  return s;
}
