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
  // Each card's last line gives the turn limit after the pick and the turn it falls on: the turn now plus the limit.
  const now = await page.eval('game.view().turn');
  const limits = await page.eval(`[...document.querySelectorAll('#pick .card')].map((c) => [...c.querySelectorAll('.dim')].map((d) => d.textContent).find((t) => t.startsWith('The game ends after')) ?? null)`);
  const ends = limits.map((t) => /^The game ends after (\d+) turns?, at turn (\d+)$/.exec(t ?? ''));
  need(ends.every(Boolean), `a card has no limit line: ${JSON.stringify(limits)}`);
  const [turns, at] = [Number(ends[0]?.[1]), Number(ends[0]?.[2])];
  need(ends.every((m) => m && Number(m[1]) === turns && Number(m[2]) === now + turns), `the limit lines disagree with turn ${now} + ${turns}: ${JSON.stringify(limits)}`);
  need(at === now + 200, `the small map's limit is 200 turns, the card says ${turns}`);

  // The card's Show (the map's Show feed is a button too, while the feed is closed).
  await page.eval("[...document.querySelectorAll('#pick .card button')].find((b) => b.textContent.trim() === 'Show').click()");
  const lit = await page.eval('game.map.highlight ? game.map.highlight.tiles.length : 0');
  need(lit > 0, 'Show did not highlight a civ\'s land');
  await page.shot(join(out, 'pick-show.png'));

  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  need(/^civ:\d+$/.test(await page.eval('document.body.dataset.selected')), 'Lead did not select the civ');
  const limit = await page.eval('game.view().limit');
  need(limit && limit.turns === turns && limit.at === at, `the view's limit is ${JSON.stringify(limit)}, the card said ${turns} at ${at}`);
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
