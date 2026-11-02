// The pick (touch point 5): three cards; Show highlights a civ's land; Lead selects its capital; Watch
// leaves no civ led.   node www/play/pick.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-pick-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`pick: ${msg}`);
    failed = true;
  }
};

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  const cards = await page.eval("document.querySelectorAll('#pick .card').length");
  need(cards === 3, `the pick shows ${cards} cards, not 3`);

  await page.button('Show');
  const lit = await page.eval('game.map.highlight ? game.map.highlight.tiles.length : 0');
  need(lit > 0, 'Show did not highlight a civ\'s land');
  await page.shot(join(out, 'pick-show.png'));

  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  need(/^civ:\d+$/.test(await page.eval('document.body.dataset.selected')), 'Lead did not select the civ');
  need((await page.eval('document.body.dataset.civ')) !== '', 'Lead did not make a civ the player\'s');
  need(!(await page.eval("document.getElementById('pick').hidden === false")), 'the pick is still up after Lead');
  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
} finally {
  await page.close();
}

// Watch leaves no civ led.
const watch = await open('?size=small&seed=2&run=1');
try {
  await watch.until("document.body.dataset.phase === 'choose'");
  await watch.button('Watch all');
  await watch.until("document.body.dataset.phase === 'main'");
  need((await watch.eval('document.body.dataset.civ')) === '', 'Watch left a civ led');
  need((await watch.eval('document.body.dataset.selected')) === '', 'Watch selected a capital');
  need(watch.errors().length === 0, `page errors: ${watch.errors().join('; ')}`);
} finally {
  await watch.close();
}
process.exit(failed ? 1 : 0);
