// The end screen (touch point 14): Standing so far shows a line per civ and "The game goes on"; Play on
// returns to play at the same turn; New game reaches start.   node www/play/end.mjs [--out DIR]
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
const turn = async (page) => Number(await page.eval('game.view().turn'));

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  for (let i = 0; i < 3; i++) await page.button('End turn');
  const n = await turn(page);

  await page.eval("document.querySelector('#gamemenu > summary').click()");
  await page.button('Standing so far');
  await page.until("document.body.dataset.screen === 'end'");
  const civs = await page.eval('Object.keys(game.view().civs).length');
  const lines = await page.eval("document.querySelectorAll('#end-chart polyline').length");
  need(lines === civs, `the chart has ${lines} lines for ${civs} civs`);
  const head = await page.eval("document.getElementById('end-head').textContent");
  need(head === `The game goes on: turn ${n}`, `the header reads ${JSON.stringify(head)}`);
  await page.shot(join(out, 'end.png'), 0.5);

  await page.button('Play on');
  await page.until("document.body.dataset.screen === 'play'");
  need((await turn(page)) === n, `Play on gives turn ${await turn(page)}, not ${n}`);

  await page.eval("document.querySelector('#gamemenu > summary').click()");
  await page.button('Standing so far');
  await page.until("document.body.dataset.screen === 'end'");
  await page.button('New game');
  await page.until("document.body.dataset.screen === 'start'");
  need(!(await page.eval("document.getElementById('screen-start').hidden")), 'New game did not show start');
  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
