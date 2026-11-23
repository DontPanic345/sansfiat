// The staged games the page scripts play on: a map's text on a radius-R hex (`grid`), its plains with sea on the
// rim (`plains`), its marks by letter (`marks`), and `stage`, which builds the game and waits for its main phase.
// `LINEUP` is the unit lineup. Shared by the scripts in www/play/; it lives here, not there, because all.mjs runs
// every script in www/play/. shots.mjs keeps its own copy of LINEUP, so it runs alone on a main worktree.
//   import { grid, plains, marks, stage, LINEUP } from '../play-stage.mjs';

const FRAMES = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

/** The hex distance of (q, r) from the centre. */
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));

/** A radius-R map's text (as `map::ascii` draws one): `cell(q, r)` gives each tile's character. */
export const grid = (R, cell) => {
  let text = '';
  for (let r = -R; r <= R; r++) {
    const lo = Math.max(-R, -r - R);
    const hi = Math.min(R, -r + R);
    text += ' '.repeat(2 * lo + r + 2 * R);
    const row = [];
    for (let q = lo; q <= hi; q++) row.push(cell(q, r));
    text += `${row.join(' ')}\n`;
  }
  return text;
};

/** Sea on the rim of radius R, plains inside. */
export const plains = (R) => grid(R, (q, r) => (dist(q, r) === R ? '~' : '.'));

/** The marks' text on radius R: `at` maps a letter to its tile's [q, r]. */
export const marks = (R, at) =>
  grid(R, (q, r) => Object.keys(at).find((k) => at[k][0] === q && at[k][1] === r) ?? '.');

/** Stages `spec` (a StageSpec, as an object) on the page, which must be open, and waits for its main phase and two
 *  frames. The game is a staged game, not a save. */
export const stage = async (page, spec) => {
  await page.eval(`game.stage(${JSON.stringify(spec)})`);
  await page.until("document.body.dataset.phase === 'main'");
  await page.eval(FRAMES);
};

/** The unit lineup: radius 8, sea on the rim, plains inside with a forest block on the east. Three civs (red led, blue,
 *  gold), each with one army and one scout: red's army on plains and its scout in the wood, blue's army in the wood and
 *  its scout on plains, gold's army and scout stacked on one plains tile. */
const forest8 = (q, r) => q >= 3 && q <= 5 && r >= -2 && r <= 1;
export const LINEUP = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : forest8(q, r) ? 'f' : '.')),
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
