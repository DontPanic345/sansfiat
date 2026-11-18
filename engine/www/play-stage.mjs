// The staged games the page scripts play on: a map's text on a radius-R hex (`grid`), its plains with sea on the
// rim (`plains`), its marks by letter (`marks`), and `stage`, which builds the game and waits for its main phase.
// Shared by the scripts in www/play/; it lives here, not there, because all.mjs runs every script in www/play/.
//   import { grid, plains, marks, stage } from '../play-stage.mjs';

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
