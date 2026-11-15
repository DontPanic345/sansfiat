// The end of a game (touch point 14): the end screen by conquest, by the led civ's fall and by the limit, the
// same limit watched; Play on after the end (the world runs on, the orders grey with "the game is over"); Standing
// so far again; New game. Each game is staged (`game.stage`) and played through the page's own buttons and chips.
// Screenshots: end-won.png, end-lost.png, end-limit.png.   node www/play/end.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-end-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`end: ${msg}`);
    failed = true;
  }
};

// A radius-4 map of plains inside a sea rim, with three capitals: A (led in most games), B and C.
const R = 4;
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const SPAN = (r) => ({ lo: Math.max(-R, -r - R), hi: Math.min(R, -r + R) });
function gridText(cell) {
  let text = '';
  for (let r = -R; r <= R; r++) {
    const { lo, hi } = SPAN(r);
    text += ' '.repeat(2 * lo + r + 2 * R);
    const row = [];
    for (let q = lo; q <= hi; q++) row.push(cell(q, r));
    text += `${row.join(' ')}\n`;
  }
  return text;
}
const MARK = { A: [-2, 0], B: [2, 0], C: [0, -3] };
const MAP = gridText((q, r) => (dist(q, r) === R ? '~' : '.'));
const MARKS = gridText((q, r) => Object.keys(MARK).find((k) => MARK[k][0] === q && MARK[k][1] === r) ?? '.');
// Each civ's clan is 1.5 people with grain on its capital (a clan under one person dies, and one with no food
// starves to it). A rival army on a capital with no army of its civ on it takes it (`civs::fall`'s rule).
const spec = (civs, extra = {}) => ({
  map: MAP,
  marks: MARKS,
  civs: civs.map((m) => ({ mark: m })),
  people: 1.5,
  lead: 0,
  food: civs.map((m) => ({ at: { mark: m }, good: 1, amount: 30 })),
  ...extra,
});

const page = await open('');
const screen = () => page.eval('document.body.dataset.screen');
const head = () => page.eval("document.getElementById('end-head').textContent");
const line = () => page.eval("document.getElementById('end-line').textContent");

/** Stages a game of `s`, then waits until it is on the map in its main phase (or at the pick, when unled). */
async function stage(s) {
  await page.eval(`game.stage(${JSON.stringify(s)})`);
  await page.until("document.body.dataset.screen === 'play'");
  await page.until("document.body.dataset.phase === 'main' || document.body.dataset.phase === 'choose'");
}

/** End turn, as a player presses it, until the end screen comes up (at most `turns` presses). */
async function playToEnd(turns) {
  for (let i = 0; i < turns && (await screen()) !== 'end'; i++) await page.button('End turn');
  await page.until("document.body.dataset.screen === 'end'");
}

try {
  // Conquest: A's army of five on B's capital, where B has no army. B falls in a turn or two,
  // and A is the last capital standing.
  await stage(spec(['A', 'B'], { armies: [{ civ: 0, at: { mark: 'B' }, people: 5 }] }));
  await playToEnd(10);
  need((await head()) === 'You won', `conquest: the header reads ${JSON.stringify(await head())}`);
  need((await line()).includes('last capital'), `conquest: the line reads ${JSON.stringify(await line())}`);
  const lines = await page.eval("document.querySelectorAll('#end-chart polyline').length");
  need(lines === 2, `conquest: the chart has ${lines} lines for 2 civs`);
  await page.shot(join(out, 'end-won.png'), 0.5);
  need(page.errors().length === 0, `conquest: page errors: ${page.errors().join('; ')}`);

  // Fell: B's army of five on A's capital, where the led civ has no army. A's
  // capital falls to B, and the line names the taker (the led civ's fall is told as its fall).
  await stage(spec(['A', 'B', 'C'], { armies: [{ civ: 1, at: { mark: 'A' }, people: 5 }] }));
  await playToEnd(10);
  need((await head()) === 'You lost', `fell: the header reads ${JSON.stringify(await head())}`);
  const taker = await page.eval('game.view().civs[Object.keys(game.view().civs)[1]].name');
  need((await line()).includes(`fell to ${taker}`), `fell: the line reads ${JSON.stringify(await line())}`);
  await page.shot(join(out, 'end-lost.png'), 0.5);

  // The limit: three civs, three turns, and the highest Standing at the limit. Then Play on: the map is back, the
  // turns go on, a raise greys with "the game is over", Standing so far shows the screen again, and New game
  // reaches start.
  await stage(spec(['A', 'B', 'C'], { limit: 3 }));
  await playToEnd(5);
  need((await line()).includes('turn limit'), `limit: the line reads ${JSON.stringify(await line())}`);
  await page.shot(join(out, 'end-limit.png'), 0.5);
  const over = await page.eval('game.view().turn');
  await page.button('Play on');
  await page.until("document.body.dataset.screen === 'play'");
  await page.button('End turn');
  need((await page.eval('game.view().turn')) === over + 1, 'limit: End turn after Play on does not advance the turn');
  need((await page.eval('game.view().phase')) === 'over', 'limit: the game is not over after Play on');
  const me = await page.eval('game.view().player');
  await page.eval(`document.querySelector('#chips button[data-civ="${me}"]').click()`);
  const raise = (await page.panel())?.buttons.find((b) => b.label.startsWith('Raise'));
  need(raise && raise.disabled && raise.title === 'the game is over', `limit: Raise… reads ${JSON.stringify(raise)}`);
  await page.eval("document.querySelector('#gamemenu > summary').click()");
  await page.button('Standing so far');
  await page.until("document.body.dataset.screen === 'end'");
  need((await head()) !== '', 'limit: Standing so far shows no header');
  await page.button('New game');
  await page.until("document.body.dataset.screen === 'start'");

  // Watched: the same limit with no civ led. The pick's Watch leaves none, and the header names the winner, not you.
  await stage(spec(['A', 'B', 'C'], { lead: undefined, limit: 3 }));
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Watch all');
  await page.until("document.body.dataset.phase === 'main'");
  await playToEnd(5);
  const h = await head();
  need(/ won$/.test(h) && !h.startsWith('You'), `watched: the header reads ${JSON.stringify(h)}`);
  need((await line()).includes('turn limit'), `watched: the line reads ${JSON.stringify(await line())}`);
  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
