import { CLAN, MAP } from './data.js';
import { key, neighbors } from './hex.js';
import { generateMap, isLand } from './mapgen.js';
import { pick } from './rng.js';
import { spawnClan } from './clans.js';
import { dispatch, initModules, runTurn } from './rules.js';

// The shared state shape is documented in CLAUDE.md ("State shape").
// players: how many civs the pre-game founds (one human, the rest AI).
export function newGame({ seed = 1, players = 2, width = MAP.width, height = MAP.height } = {}) {
  const s = {
    config: { seed, players, width, height },
    rng: seed >>> 0,
    turn: 1,
    phase: 'pregame', // 'pregame' -> 'choose' (pick a civ) -> 'main'
    nextId: 1,
    events: [],
    clans: {},
    markets: {},
    civs: {},
    cities: {},
    units: {},
    notes: {},
    mods: {},
    stats: { pop: [], clans: [], volume: [] },
  };
  s.map = generateMap(s, width, height);
  seedClans(s);
  initModules(s);
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

// Steps until the pre-game hands over to founding, or `maxTurns` more turns pass. Returns
// {state, turns} where turns is how many endTurn actions it took (record that many in a replay
// history). Headless fast path: clones once and runs the turns in place, which gives exactly
// the state that dispatching endTurn that many times would (tested).
export function runPregameCounted(s, maxTurns = Infinity) {
  s = structuredClone(s);
  let turns = 0;
  for (; turns < maxTurns && s.phase === 'pregame'; turns++) {
    s.events = [];
    runTurn(s);
  }
  return { state: s, turns };
}

export const runPregame = (s, maxTurns = Infinity) => runPregameCounted(s, maxTurns).state;
