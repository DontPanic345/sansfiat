// The views table (#445 moved it here from shots.mjs, which imports it): each view's cameras and stages, and the lineups
// they stage (LINEUP, CLANS, PLACES) with their in-page state scripts. shots.mjs and this file copy together onto a main
// worktree (play-stage.mjs's grid and marks are on main too).
import { grid, marks } from './play-stage.mjs';

// The lineups, as play-stage.mjs has them (kept here too, so this file copies alone onto a main worktree).
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const forest = (q, r) => q >= 3 && q <= 5 && r >= -2 && r <= 1;
const ridge = (q, r) => r === -2 && q >= 0 && q <= 2;

// The unit lineup: the map, civs and units of LINEUP.
export const LINEUP = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : forest(q, r) ? 'f' : '.')),
  marks: marks(8, { A: [-5, 0], B: [0, -5], C: [0, 5], a: [1, 0], b: [4, -1], c: [1, 1], s: [4, 0], t: [2, -1] }),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [
    { civ: 0, at: { mark: 'a' }, people: 6 }, // red on plains
    { civ: 1, at: { mark: 'b' }, people: 6 }, // blue in the wood
    { civ: 2, at: { mark: 'c' }, people: 4 }, // gold on plains, stacked with its scout
  ],
  scouts: [
    { civ: 0, at: { mark: 's' }, pack: 0 }, // red in the wood
    { civ: 1, at: { mark: 't' }, pack: 0 }, // blue on plains
    { civ: 2, at: { mark: 'c' }, pack: 0 }, // gold on its army's tile
  ],
};

// The clan lineup, as play-stage.mjs's CLANS (kept here too, so this file copies alone onto a main worktree), and the
// in-page code that sets its states (play-stage.mjs's CLAN_STATES).
export const CLANS = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : forest(q, r) ? 'f' : '.')),
  marks: marks(8, { A: [-5, 0], B: [0, -5], C: [0, 5], c: [1, 1], h: [-1, 2], p: [0, 2], q: [1, 2], r: [2, 2], s: [3, 2], w: [3, -1], x: [4, -1], y: [5, -1], z: [4, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [{ civ: 2, at: { mark: 'c' }, people: 4 }],
  scouts: [{ civ: 2, at: { mark: 'c' }, pack: 0 }],
  clans: [
    { home: { mark: 'p' }, tiles: [{ mark: 'p' }], people: 6, civ: 0 },
    { home: { mark: 'q' }, tiles: [{ mark: 'q' }], people: 6, civ: 1 },
    { home: { mark: 'r' }, tiles: [{ mark: 'r' }], people: 6, civ: 2 },
    { home: { mark: 's' }, tiles: [{ mark: 's' }], people: 6 },
    { home: { mark: 'w' }, tiles: [{ mark: 'w' }], people: 6, civ: 0 },
    { home: { mark: 'x' }, tiles: [{ mark: 'x' }], people: 6, civ: 1 },
    { home: { mark: 'y' }, tiles: [{ mark: 'y' }], people: 6, civ: 2 },
    { home: { mark: 'z' }, tiles: [{ mark: 'z' }], people: 6 },
    { home: { mark: 'h' }, tiles: [{ mark: 'h' }], people: 9, civ: 0 },
    { home: { mark: 'h' }, tiles: [], people: 5, civ: 0 },
    { home: { mark: 'h' }, tiles: [], people: 3, civ: 0 },
    { home: { mark: 'h' }, tiles: [], people: 4, civ: 1 },
    { home: { mark: 'c' }, tiles: [], people: 4 },
  ],
};
export const CLAN_STATES = `(() => {
  const m = game.map, v = m.model.v, at = (q, r) => m.at(q, r);
  const on = (t) => Object.values(v.clans).filter((c) => c.home === t).sort((a, b) => a.id - b.id);
  const h = on(at(-1, 2));
  h[0].settled = true;
  h[2].since = 0;
  for (const c of Object.values(v.clans)) if (c.since === 0 && c !== h[2]) c.since = 5;
  on(at(2, 2))[0].starving = true;
  const civOf = (qr) => Object.values(v.civs).find((c) => c.capital === at(...qr)).id;
  on(at(3, 2))[0].leaning = { [civOf([0, -5])]: 0.6 * m.model.rules.join };
  on(at(4, 0))[0].leaning = { [civOf([0, 5])]: 0.2 * m.model.rules.join };
  m.dirty = true;
  m.drawNow();
  return 'set';
})()`;


// The place lineup (#445): PLACES, the map and civs of the unit lineup's shape with a ridge north of the wood, a capital
// (A) with its clans, a blue signpost (S) and clans (the first turn founds a settlement there), four clans of no civ on T.
// As proto/lineup.mjs has it (play-stage.mjs keeps its copy too). PLACE_STATES sets the states the stage can't, as CLAN_STATES does.
export const PLACES = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : ridge(q, r) ? '^' : forest(q, r) ? 'f' : '.')),
  marks: marks(8, { A: [-4, 1], B: [0, -6], C: [-1, 6], S: [2, -1], c: [1, -1], T: [-2, 4], a: [-4, 2], s: [2, 0], t: [-1, 4] }),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [{ civ: 0, at: { mark: 'c' }, people: 4 }],
  scouts: [{ civ: 0, at: { mark: 'c' }, pack: 0 }],
  clans: [
    { home: { mark: 'A' }, tiles: [{ mark: 'a' }], people: 6, civ: 0, market: 0 },
    { home: { mark: 'A' }, tiles: [], people: 5, civ: 0, market: 0 },
    { home: { mark: 'A' }, tiles: [], people: 4, civ: 0, market: 0 },
    { home: { mark: 'S' }, tiles: [{ mark: 'S' }, { mark: 's' }], people: 6, civ: 1 },
    { home: { mark: 'S' }, tiles: [], people: 5, civ: 1 },
    { home: { mark: 'S' }, tiles: [], people: 4, civ: 1 },
    { home: { mark: 'T' }, tiles: [{ mark: 'T' }, { mark: 't' }], people: 6 },
    { home: { mark: 'T' }, tiles: [], people: 5 },
    { home: { mark: 'T' }, tiles: [], people: 4 },
    { home: { mark: 'T' }, tiles: [], people: 4 },
  ],
  signposts: [{ civ: 1, at: { mark: 'S' } }],
  food: ['A', 'a', 'S', 's', 'T', 't'].map((m) => ({ at: { mark: m }, good: 1, amount: 40 })),
};

/** In the page: the town of no civ on T (the stage makes no market of no civ) is set on the drawn model, and every clan is
 *  out of walking (`since` 5) and not starving, then a redraw. As play-stage.mjs's PLACE_STATES. */
export const PLACE_STATES = `(() => {
  const m = game.map, v = m.model.v, t = m.at(-2, 4);
  const clans = Object.values(v.clans).filter((c) => c.home === t).map((c) => c.id);
  v.towns.push({ tile: t, market: null, clans });
  for (const c of Object.values(v.clans)) { c.starving = false; c.since = 5; }
  m.dirty = true;
  m.drawNow();
  return 'set';
})()`;

/**
 * The views: `size` is a map (`small`, `medium`, `large`), `seed` its seed, `window` `laptop` or `phone`, `k` the
 * zoom. `on` is the camera's tile: `plains` (the plains tile nearest world (0, 0), the default) or a mountain
 * class (`fork`, `bend`, `4+`), the tile of that class nearest world (0, 0). `tile` is the tile it must be.
 * `edge` moves the camera so the tile's centre sits `by` world units past that canvas edge.
 * The shore classes (`neck`, `strait`, `bay`, `pond`, `mouth-sea`, `mouth-lake`, `marsh`) are shores.md's, as the
 * chooser counts them: water is terrain 0, and off the map counts as water.
 */
export const VIEWS = [
  { name: 'small2-close', size: 'small', seed: 2, window: 'laptop', k: 35 },
  { name: 'small2-play', size: 'small', seed: 2, window: 'laptop', k: 18 },
  { name: 'small2-edge', size: 'small', seed: 2, window: 'phone', k: 6 },
  { name: 'small2-flat', size: 'small', seed: 2, window: 'phone', k: 5 },
  { name: 'small3-play', size: 'small', seed: 3, window: 'laptop', k: 18 },
  { name: 'small6-play', size: 'small', seed: 6, window: 'laptop', k: 18 },
  { name: 'small9-play', size: 'small', seed: 9, window: 'laptop', k: 18 },
  { name: 'medium8-play', size: 'medium', seed: 8, window: 'laptop', k: 18 },
  { name: 'medium9-play', size: 'medium', seed: 9, window: 'laptop', k: 18 },
  { name: 'large8-play', size: 'large', seed: 8, window: 'laptop', k: 18 },
  { name: 'small9-fork-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'fork', tile: 469 },
  { name: 'small9-fork-play', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'fork', tile: 469 },
  { name: 'small2-bend-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'bend', tile: 377 },
  { name: 'small2-bend-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'bend', tile: 377 },
  { name: 'medium9-blob-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: '4+', tile: 777 },
  { name: 'large8-bend-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'bend', tile: 1937 },
  { name: 'large8-fork-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'fork', tile: 1953 },
  { name: 'small9-edge-bottom', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'bend', tile: 474, edge: { side: 'bottom', by: 1.0 } },
  { name: 'small9-edge-right', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'bend', tile: 474, edge: { side: 'right', by: 0.8 } },
  { name: 'small9-fork-edge', size: 'small', seed: 9, window: 'phone', k: 6, on: 'fork', tile: 469 },
  { name: 'small9-fork-flat', size: 'small', seed: 9, window: 'phone', k: 5, on: 'fork', tile: 469 },
  { name: 'small2-neck-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'neck', tile: 402 },
  { name: 'small2-delta-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'mouth-sea', tile: 506 },
  { name: 'small2-delta-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'mouth-sea', tile: 506 },
  { name: 'small6-bay-close', size: 'small', seed: 6, window: 'laptop', k: 35, on: 'bay', tile: 376 },
  { name: 'small9-pond-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'pond', tile: 341 },
  { name: 'small9-lake-play', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'mouth-lake', tile: 377 },
  { name: 'medium8-strait-close', size: 'medium', seed: 8, window: 'laptop', k: 35, on: 'strait', tile: 875 },
  { name: 'medium9-neck-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: 'neck', tile: 1004 },
  { name: 'large8-delta-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'marsh', tile: 2639 },
  { name: 'large8-pond-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'pond', tile: 1289 },
  { name: 'small2-neck-edge', size: 'small', seed: 2, window: 'phone', k: 6, on: 'neck', tile: 402 },
  { name: 'small2-neck-flat', size: 'small', seed: 2, window: 'phone', k: 5, on: 'neck', tile: 402 },
  { name: 'medium9-woods-close', size: 'medium', seed: 9, window: 'laptop', k: 35, on: 'interior', tile: 827 },
  { name: 'medium9-woods-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: 'interior', tile: 827 },
  { name: 'medium9-woods-near', size: 'medium', seed: 9, window: 'laptop', k: 12, on: 'interior', tile: 827 },
  { name: 'medium9-woods-far', size: 'medium', seed: 9, window: 'laptop', k: 11, on: 'interior', tile: 827 },
  { name: 'medium9-woods-edge', size: 'medium', seed: 9, window: 'phone', k: 6, on: 'interior', tile: 827 },
  { name: 'medium9-woods-flat', size: 'medium', seed: 9, window: 'phone', k: 5, on: 'interior', tile: 827 },
  { name: 'small2-lone-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'lone', tile: 439 },
  { name: 'small2-lone-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'lone', tile: 439 },
  { name: 'small9-river-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'forest-river', tile: 258 },
  { name: 'small2-delta-near', size: 'small', seed: 2, window: 'laptop', k: 12, on: 'marsh', tile: 506 },
  { name: 'small2-delta-far', size: 'small', seed: 2, window: 'laptop', k: 11, on: 'marsh', tile: 506 },
  { name: 'small2-delta-edge', size: 'small', seed: 2, window: 'phone', k: 6, on: 'marsh', tile: 506 },
  { name: 'small2-delta-flat', size: 'small', seed: 2, window: 'phone', k: 5, on: 'marsh', tile: 506 },
  { name: 'medium9-fields-close', size: 'medium', seed: 9, window: 'laptop', k: 35, on: 'forest-fields', tile: 828, turns: 8 },
  { name: 'small2-marsh-fields-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'marsh-fields', tile: 535, turns: 8 },
  // The unit views (#443): the figures, the stacks and the lineup, each set up on main by proto/scenes.mjs and lineup.mjs.
  { name: 'medium9-units-play', size: 'medium', seed: 9, window: 'laptop', k: 18, raise: 3, on: 'at', between: [547, 780], units: true },
  { name: 'medium9-stack-close', size: 'medium', seed: 9, window: 'laptop', k: 35, raise: 3, on: 'at', at: 547, units: true },
  { name: 'medium9-wood-close', size: 'medium', seed: 9, window: 'laptop', k: 35, on: 'at', at: 780, units: true },
  { name: 'medium9-gold-play', size: 'medium', seed: 9, window: 'laptop', k: 18, lead: 'Gold', raise: 3, on: 'at', at: 780, units: true },
  { name: 'medium9-gold-close', size: 'medium', seed: 9, window: 'laptop', k: 35, lead: 'Gold', raise: 3, on: 'at', at: 780, units: true },
  { name: 'medium8-units-play', size: 'medium', seed: 8, window: 'laptop', k: 18, on: 'at', between: [1145, 639], units: true },
  { name: 'small2-camps-play', size: 'small', seed: 2, window: 'laptop', k: 18, turns: 8, on: 'at', at: 527, units: true },
  { name: 'small2-camps-close', size: 'small', seed: 2, window: 'laptop', k: 35, turns: 8, on: 'at', at: 527, units: true },
  { name: 'medium9-units-edge', size: 'medium', seed: 9, window: 'phone', k: 6, raise: 3, on: 'at', between: [547, 780], units: true },
  { name: 'medium9-units-flat', size: 'medium', seed: 9, window: 'phone', k: 5, raise: 3, on: 'at', between: [547, 780], units: true },
  { name: 'lineup-close', window: 'laptop', k: 35, stage: 'lineup', on: 'at', between: [125, 170], units: true },
  { name: 'lineup-play', window: 'laptop', k: 18, stage: 'lineup', on: 'at', between: [125, 170], units: true },
  { name: 'lineup-edge', window: 'phone', k: 6, stage: 'lineup', on: 'at', between: [125, 170], units: true },
  // The signals: a unit selected (white edge), and a hungry camp selected (white over red).
  { name: 'lineup-selected', window: 'laptop', k: 18, stage: 'lineup', on: 'at', between: [125, 170], select: { kind: 'camp', civ: 'red' }, units: true },
  { name: 'small2-camps-selected', size: 'small', seed: 2, window: 'laptop', k: 35, turns: 8, on: 'at', at: 527, select: { tile: 527 }, units: true },
  // The clan views (#444): the clans as cells, their states and kinds, each set up on main by proto/scene.mjs and lineup.mjs.
  { name: 'medium9-clans-play', size: 'medium', seed: 9, window: 'laptop', k: 18, turns: 16, on: 'at', between: [734, 874], clans: true },
  { name: 'medium9-clans-close', size: 'medium', seed: 9, window: 'laptop', k: 35, turns: 16, on: 'at', between: [734, 874], clans: true },
  { name: 'medium9-clans-edge', size: 'medium', seed: 9, window: 'phone', k: 6, turns: 16, on: 'at', between: [734, 874], clans: true },
  { name: 'medium9-clans-flat', size: 'medium', seed: 9, window: 'phone', k: 5, turns: 16, on: 'at', between: [734, 874], clans: true },
  { name: 'small2-clans-play', size: 'small', seed: 2, window: 'laptop', k: 18, turns: 16, on: 'at', between: [410, 503], clans: true },
  { name: 'small2-clans-close', size: 'small', seed: 2, window: 'laptop', k: 35, turns: 16, on: 'at', at: 503, clans: true },
  { name: 'medium8-clans-play', size: 'medium', seed: 8, window: 'laptop', k: 18, turns: 16, on: 'at', at: 401, clans: true },
  { name: 'medium8-lean-play', size: 'medium', seed: 8, window: 'laptop', k: 18, turns: 16, on: 'at', at: 1368, clans: true },
  { name: 'small9-lean-close', size: 'small', seed: 9, window: 'laptop', k: 35, turns: 4, on: 'at', at: 417, clans: true },
  { name: 'clans-lineup-close', window: 'laptop', k: 35, stage: 'clans', states: true, on: 'at', between: [93, 170], clans: true },
  { name: 'clans-lineup-play', window: 'laptop', k: 18, stage: 'clans', states: true, on: 'at', between: [93, 170], clans: true },
  { name: 'clans-lineup-edge', window: 'phone', k: 6, stage: 'clans', states: true, on: 'at', between: [93, 170], clans: true },
  // The place views (#445): the places as cells, each set up on main by proto/scenes.mjs and lineup.mjs (relay/specs/445).
  { name: 'medium9-places-play', size: 'medium', seed: 9, window: 'laptop', k: 18, turns: 40, on: 'at', between: [735, 566], places: true },
  { name: 'medium9-places-close', size: 'medium', seed: 9, window: 'laptop', k: 35, turns: 40, on: 'at', between: [735, 919], places: true },
  { name: 'medium9-places-edge', size: 'medium', seed: 9, window: 'phone', k: 6, turns: 40, on: 'at', between: [735, 566], places: true },
  { name: 'medium9-places-flat', size: 'medium', seed: 9, window: 'phone', k: 5, turns: 40, on: 'at', between: [735, 566], places: true },
  { name: 'small2-places-close', size: 'small', seed: 2, window: 'laptop', k: 35, turns: 40, on: 'at', between: [505, 506], places: true },
  { name: 'large8-places-close', size: 'large', seed: 8, window: 'laptop', k: 35, turns: 40, on: 'at', between: [2787, 2689], places: true },
  { name: 'large8-places-play', size: 'large', seed: 8, window: 'laptop', k: 18, turns: 40, on: 'at', at: 2524, places: true },
  { name: 'places-lineup-close', window: 'laptop', k: 35, stage: 'places', turns: 1, states: true, on: 'at', between: [47, 140], places: true },
  { name: 'places-lineup-play', window: 'laptop', k: 18, stage: 'places', turns: 1, states: true, on: 'at', between: [47, 140], places: true },
  { name: 'places-lineup-edge', window: 'phone', k: 6, stage: 'places', turns: 1, states: true, on: 'at', between: [47, 140], places: true },
];
