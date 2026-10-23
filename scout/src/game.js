import { PLAYERS, SCOUT, TERRAIN } from './data.js';
import { distance, neighbors } from './hex.js';
import { generateMap } from './mapgen.js';
import { pick } from './rng.js';
import { dispatch, passable, spawnUnit, startTurn } from './rules.js';

export function newGame({ seed = 1, width = 22, height = 14, players = 2 } = {}) {
  const s = {
    config: { seed, width, height, players },
    rng: seed >>> 0,
    turn: 1,
    current: 0,
    nextId: 1,
    winner: null,
    events: [],
    players: PLAYERS.slice(0, players).map((p, id) => ({ id, ...p, alive: true, bridgeSpan: SCOUT.bridgeSpan, revealed: {}, built: {} })),
    units: {},
    cities: {},
  };
  s.map = generateMap(s, width, height);
  placeStarts(s);
  startTurn(s);
  return s;
}

// Each start is the candidate farthest from the starts already chosen.
function placeStarts(s) {
  const candidates = Object.values(s.map.tiles).filter(
    (t) => TERRAIN[t.terrain].passable && neighbors(t).filter((n) => passable(s, n)).length >= 3);
  const starts = [pick(s, candidates)];
  while (starts.length < s.players.length) {
    const score = (t) => Math.min(...starts.map((x) => distance(x, t)));
    starts.push(candidates.reduce((a, b) => (score(b) > score(a) ? b : a)));
  }
  starts.forEach((t, p) => {
    spawnUnit(s, p, 'settler', t);
    spawnUnit(s, p, 'warrior', neighbors(t).find((n) => passable(s, n)));
  });
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
