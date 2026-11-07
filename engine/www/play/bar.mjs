// The top bar (touch points 7-9): the pre-game's counts and the chips' Standing with a tooltip on each
// number; End turn and space each end a turn; Play runs turns and space pauses; speed changes the rate;
// each menu opens, and Escape closes it.   node www/play/bar.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-bar-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`bar: ${msg}`);
    failed = true;
  }
};
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const turn = async (page) => Number(await page.eval('game.view().turn'));

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  // Before the pick: the counts, each with its tooltip.
  const untitled = await page.eval(`[...document.querySelectorAll('#bar .stat')].filter((e) => e.offsetParent && !e.title).map((e) => e.textContent.trim())`);
  need(untitled.length === 0, `counts without a tooltip: ${untitled.join(', ')}`);
  need((await page.eval("document.getElementById('chips').hidden")) === true, 'the chips show before the pick');

  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");

  // After it: a chip per civ, the Standing summing to at most 100%, yours marked, each with a tooltip.
  const chips = await page.eval(`[...document.querySelectorAll('#chips .chip')].map((c) => ({ mine: c.classList.contains('mine'), you: !!c.querySelector('em'), pct: parseFloat(c.querySelector('span').textContent), title: c.title }))`);
  need(chips.length === 3, `${chips.length} chips, not 3`);
  const sum = chips.reduce((a, c) => a + c.pct, 0);
  need(sum <= 100.05, `the chips' Standing sums to ${sum}%`);
  need(chips.filter((c) => c.mine).length === 1 && chips.find((c) => c.mine).you, 'the led civ is not marked "you"');
  need(chips.every((c) => c.title.startsWith('Standing ')), 'a chip has no Standing tooltip');
  const untitledNow = await page.eval(`[...document.querySelectorAll('#bar .stat, #bar .chip')].filter((e) => e.offsetParent && !e.title).length`);
  need(untitledNow === 0, `${untitledNow} bar numbers without a tooltip`);
  // The turn reads against the limit: "312 of 437", the turn the limit falls on.
  const turnText = await page.eval(`document.getElementById('turn').textContent + ' ' + document.getElementById('turnlabel').textContent`);
  const shown = JSON.parse(await page.eval('JSON.stringify({ turn: game.view().turn, limit: game.view().limit })'));
  need(turnText === `${shown.turn} of ${shown.limit.at}`, `the turn reads "${turnText}", not "${shown.turn} of ${shown.limit.at}"`);
  await page.shot(join(out, 'bar.png'), 0.5);

  // End turn by button and by space.
  const t0 = await turn(page);
  await page.button('End turn');
  await page.until(`Number(game.view().turn) === ${t0 + 1}`, 10_000);
  await page.key('Space');
  await page.until(`Number(game.view().turn) === ${t0 + 2}`, 10_000);

  // Play runs turns; space pauses it.
  await page.button('Play');
  await page.until(`Number(game.view().turn) > ${t0 + 4}`, 20_000);
  await page.key('Space');
  need(await page.eval("document.getElementById('play').textContent === 'Play'"), 'space did not pause the run');
  const paused = await turn(page);
  await sleep(1200);
  need((await turn(page)) === paused, 'turns went on after the pause');

  // Speed changes the rate: a second at slow is a turn or two, at fast a good many.
  const setSpeed = (s) => page.eval(`(() => { const e = document.getElementById('speed'); e.value = '${s}'; e.dispatchEvent(new Event('change')); })()`);
  await page.button('Play');
  await setSpeed('slow');
  const a = await turn(page);
  await sleep(2000);
  const slow = (await turn(page)) - a;
  await setSpeed('fast');
  const b = await turn(page);
  await sleep(2000);
  const fast = (await turn(page)) - b;
  need(slow <= 3, `slow ran ${slow} turns in 2 s`);
  need(fast >= 10, `fast ran ${fast} turns in 2 s`);
  await page.key('Space');

  // Each menu opens, and Escape closes it.
  for (const id of ['gamemenu', 'viewmenu', 'debugmenu']) {
    await page.eval(`document.querySelector('#${id} > summary').click()`);
    need(await page.eval(`document.getElementById('${id}').open`), `${id} did not open`);
    await page.key('Escape');
    need(!(await page.eval(`document.getElementById('${id}').open`)), `Escape did not close ${id}`);
  }
  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
  console.log(JSON.stringify({ chips: chips.map((c) => c.pct), slow, fast }));
} finally {
  await page.close();
}

// Phone width: the bar is one line, its ☰ menu holds Play, speed and the menus, and the feed is a drawer
// closed by default.
const phone = await open('?size=small&seed=2&run=1', { size: 'phone' });
try {
  await phone.until("document.body.dataset.phase === 'choose'");
  await phone.button('Lead ');
  await phone.until("document.body.dataset.phase === 'main'");
  const lines = await phone.eval("Math.round(document.getElementById('bar').getBoundingClientRect().height)");
  need(lines <= 40, `the phone bar is ${lines}px tall, not one line`);
  need(await phone.eval("document.getElementById('side').classList.contains('collapsed')"), 'the feed is open at phone width');
  await phone.button('☰');
  const menu = await phone.eval("(() => { const r = document.getElementById('play').getBoundingClientRect(); return r.width > 0 && r.right <= window.innerWidth; })()");
  need(menu, 'the ☰ menu does not show Play inside the phone width');
  await phone.key('Escape');
  need(!(await phone.eval("document.getElementById('bar').classList.contains('more-open')")), 'Escape did not close the ☰ menu');
  need(phone.errors().length === 0, `page errors: ${phone.errors().join('; ')}`);
} finally {
  await phone.close();
}
process.exit(failed ? 1 : 0);
