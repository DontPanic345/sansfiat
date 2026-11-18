// The Tests screen (#339, build-2): the stage tests core lists, one of them watched on its map and held to its
// holds. Start's Tests button opens the list; the list has the four tests, the first named scout-errand with its
// claim. Choosing scout-errand draws its map (not blank), turn 0 of 40, every hold pending; Step moves the scout to
// another tile; Again and Play run it to the end: Passed, every hold held, and the page's state says passed. A copy
// of it with "by": 2 on its built-trails hold fails at turn 2 and the result names that hold. Writes
// tests-list.png, tests-errand-passed.png and tests-errand-failed.png to --out.
//   node www/play/tests.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-tests-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`tests: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
// The scout's tile in the test's view: the led civ's scout, else civ 0's.
const SCOUT = `(() => { const v = game.testView(); const c = v.player ?? 0; const u = Object.values(v.units).find((u) => u.kind === 'scout' && u.civ === c); return u ? u.tile : null; })()`;
// The share of the map's canvas that differs from its background corner, sampled.
const DRAWN = `(() => { const c = document.getElementById('tests-map'); const d = c.getContext('2d').getImageData(0, 0, c.width, c.height).data;
  const bg = [d[0], d[1], d[2]]; let diff = 0, n = 0;
  for (let i = 0; i < d.length; i += 4 * 9) { n++; if (d[i] !== bg[0] || d[i + 1] !== bg[1] || d[i + 2] !== bg[2]) diff++; }
  return diff / n; })()`;
const HOLDS = `[...document.querySelectorAll('#tests-holds li')].map((li) => li.dataset.state)`;
const NAMES = ['scout-errand', 'scout-killed', 'scout-trapped', 'scout-walks-round'];

const page = await open('');
try {
  // Start's Tests button opens the list.
  await page.button('Tests');
  await page.until("document.body.dataset.screen === 'tests'");
  need((await page.eval('game.tests().map((t) => t.name)')).join() === NAMES.join(), 'the list holds the four tests in order');

  // ?tests&pace: the list, and the pace a played turn takes (20 ms, so a whole test plays in seconds).
  await page.goto('?tests&pace=20');
  await page.until("document.body.dataset.screen === 'tests'");
  const rows = await page.eval("[...document.querySelectorAll('#tests-rows .test-row')].map((b) => b.dataset.test)");
  need(rows.join() === NAMES.join(), `the list shows four rows, the first scout-errand (${rows.join(', ')})`);
  const claim = await page.eval("document.querySelector('#tests-rows .test-row[data-test=\"scout-errand\"] span').textContent");
  need(claim === (await page.eval("game.tests()[0].claim")) && claim.length > 0, `the first row has its claim: "${claim}"`);
  await page.shot(join(out, 'tests-list.png'), 1);

  // Choosing scout-errand: its map is drawn, turn 0, every hold pending.
  await page.eval("document.querySelector('[data-test=\"scout-errand\"]').click()");
  await page.eval(frames);
  need(await page.eval("!document.getElementById('tests-open').hidden && document.getElementById('tests-name').textContent === 'scout-errand'"), 'choosing scout-errand opens it');
  need((await page.eval(DRAWN)) > 0.2, 'its map is drawn (its canvas is not blank)');
  need((await page.eval("document.getElementById('tests-turn').textContent")) === '0 of 40', 'it reads 0 of 40 at the start');
  const holds = await page.eval(HOLDS);
  need(holds.length === 8 && holds.every((s) => s === 'pending'), `all ${holds.length} holds are pending at the start`);

  // Step: one turn, and the scout is on another tile.
  const t0 = await page.eval(SCOUT);
  await page.button('Step');
  await page.eval(frames);
  const t1 = await page.eval(SCOUT);
  const s1 = await page.eval('game.testState()');
  need(s1.turn === 1 && t0 != null && t1 != null && t1 !== t0, `after Step the scout moved, from tile ${t0} to ${t1}, at turn ${s1.turn}`);

  // Again starts it afresh; Play runs it to the end.
  await page.button('Again');
  await page.eval(frames);
  need((await page.eval('game.testState().turn')) === 0, 'Again starts the test at turn 0');
  await page.button('Play');
  await page.until("document.body.dataset.testDone === 'true'", 60_000);
  await page.eval(frames);
  const passed = await page.eval('game.testState()');
  need(passed.passed === true && passed.holds.every((h) => h.state === 'held'), 'played through, scout-errand passes: every hold is held');
  need((await page.eval("document.getElementById('tests-result').textContent")) === 'Passed', 'the result reads Passed');
  need((await page.eval(HOLDS)).every((s) => s === 'held'), 'every hold on the screen is held');
  need((await page.eval("document.getElementById('tests-play').textContent")) === 'Play', 'Play is Play again once the test is done');
  await page.shot(join(out, 'tests-errand-passed.png'), 1);

  // A copy with "by": 2 on the built-trails hold (holds[3]) fails at turn 2 and names the hold.
  await page.eval(`(() => { const t = game.tests().find((x) => x.name === 'scout-errand'); const c = JSON.parse(t.json);
    c.holds[3] = { by: 2, fact: { scoutBuilt: { civ: 0 } } }; game.test(c); })()`);
  await page.eval(frames);
  await page.button('Play');
  await page.until("document.body.dataset.testDone === 'true'", 60_000);
  await page.eval(frames);
  const failedState = await page.eval('game.testState()');
  const h3 = failedState.holds[3];
  need(failedState.passed === false && h3.state === 'failed' && h3.turn === 2, `the copy fails: its built hold at turn ${h3.turn}, ${h3.state}`);
  need(/^by turn 2: /.test(h3.words), `the failed hold names its words: "${h3.words}"`);
  const result = await page.eval("document.getElementById('tests-result').textContent");
  need(result === `Failed: ${h3.words} at turn 2`, `the result reads the hold and its turn: "${result}"`);
  need((await page.eval(HOLDS))[3] === 'failed', 'the failed hold is marked failed on the screen');
  await page.shot(join(out, 'tests-errand-failed.png'), 1);

  need(page.errors().length === 0, `the page has no errors${page.errors().length ? `: ${page.errors().join('; ')}` : ''}`);
} finally {
  await page.close();
}
if (failed) process.exit(1);
console.log(`ok: screenshots in ${out}`);
