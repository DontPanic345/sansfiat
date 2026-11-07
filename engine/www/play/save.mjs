// The autosave and the files (touch points 2, 10, 13): after five turns the autosave holds turn N;
// Continue after a reload gives turn N with the same digest; Save to file, then Load a file, gives the
// same digest; with storage refused, the game plays on and the Game menu says autosave is off.
//   node www/play/save.mjs [--out DIR]
import { mkdirSync, mkdtempSync, readdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-save-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`save: ${msg}`);
    failed = true;
  }
};
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const turn = async (page) => Number(await page.eval('game.view().turn'));

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  for (let i = 0; i < 5; i++) await page.button('End turn');
  const n = await turn(page);

  // The autosave holds turn N.
  const entry = JSON.parse(await page.eval("localStorage.getItem('sansfiat.autosave')"));
  need(entry && entry.turn === n, `the autosave holds turn ${entry?.turn}, not ${n}`);
  need(entry?.civ != null, 'the autosave does not name the led civ');
  const digest = await page.eval('game.digest()');
  const lines = await page.eval('game.view().standing.length');

  // Continue, after a reload of start, gives turn N with the same digest.
  await page.goto('');
  await page.until("document.body.dataset.screen === 'start'");
  await page.button('Continue');
  await page.until("document.body.dataset.screen === 'play'");
  need((await turn(page)) === n, `Continue gives turn ${await turn(page)}, not ${n}`);
  need((await page.eval('game.digest()')) === digest, 'Continue gives another digest');
  need((await page.eval('game.view().standing.length')) === lines, 'Continue loses the Standing history');

  // Save to file, end a turn, then Load a file: back to turn N, with the same digest.
  await page.download(out);
  await page.eval("document.querySelector('#gamemenu > summary').click()");
  await page.button('Save to file');
  let saved;
  for (let i = 0; i < 50 && !saved; i++) {
    saved = readdirSync(out).find((f) => /^sansfiat-seed.*\.json$/.test(f));
    if (!saved) await sleep(100);
  }
  need(saved, 'Save to file wrote no file');
  if (saved) {
    await page.button('End turn');
    await page.file('#load', join(out, saved));
    await page.until(`Number(game.view().turn) === ${n}`, 10_000);
    need((await page.eval('game.digest()')) === digest, 'Load a file gives another digest');
  }

  // Storage refused: the game plays on, and the Game menu says autosave is off.
  await page.eval("Storage.prototype.setItem = function () { throw new Error('storage refused'); }");
  const before = await turn(page);
  await page.button('End turn');
  await page.button('End turn');
  need((await turn(page)) === before + 2, 'the game did not play on with storage refused');
  need(/Autosave off/.test(await page.eval("document.getElementById('autosave-line').textContent")), 'the Game menu does not say autosave is off');
  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
  console.log(JSON.stringify({ turn: n, digest }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
