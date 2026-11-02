// The start screen (touch points 1-4): a bare URL shows it; New game with a seed starts the pre-game and
// Skip reaches the pick; 🎲 changes the seed; a save loaded from a file lands in play at its turn; a save
// of another version shows its error on start and stays there.   node www/play/start.mjs [--out DIR]
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-start-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`start: ${msg}`);
    failed = true;
  }
};
const screen = (page) => page.eval('document.body.dataset.screen');

const page = await open('');
try {
  // A bare URL opens the start screen, with Continue greyed when there is no autosave.
  need((await screen(page)) === 'start', `a bare URL shows ${await screen(page)}, not start`);
  need(await page.eval("!document.getElementById('screen-start').hidden && document.getElementById('screen-play').hidden"), 'start is not the only screen shown');
  need(await page.eval("document.getElementById('continue').disabled && document.getElementById('continue-line').textContent === 'No saved game'"), 'Continue is not greyed with no saved game');
  await page.shot(join(out, 'start.png'));

  // 🎲 changes the seed.
  const seed0 = await page.eval("document.getElementById('seed').value");
  await page.button('🎲');
  const seed1 = await page.eval("document.getElementById('seed').value");
  need(seed1 !== seed0, `🎲 left the seed at ${seed0}`);

  // New game with a seed starts the pre-game at fast; Skip reaches the pick.
  await page.eval("document.getElementById('size').value = 'small'; document.getElementById('seed').value = '2'");
  await page.button('Start');
  await page.until("document.body.dataset.screen === 'play' && document.body.dataset.phase === 'pregame'");
  need(await page.eval("document.getElementById('speed').value === 'fast'"), 'Start did not begin the pre-game at fast');
  await page.button('Skip to the pick');
  await page.until("document.body.dataset.phase === 'choose'");
  need(await page.eval("!document.getElementById('pick').hidden"), 'the pick is not shown after Skip');

  // Save the game at the pick; a bare URL reloads start, and a file loads back into play at its turn.
  const turn = Number(await page.eval('game.view().turn'));
  const digest = await page.eval('game.digest()');
  const save = await page.eval('JSON.stringify(game.save())');
  const file = join(out, 'start-game.json');
  writeFileSync(file, save);
  await page.goto('');
  need((await screen(page)) === 'start', 'a bare URL after a game did not show start');
  await page.file('#start-load', file);
  await page.until("document.body.dataset.screen === 'play'");
  need(Number(await page.eval('game.view().turn')) === turn, `the loaded file is at turn ${await page.eval('game.view().turn')}, not ${turn}`);
  need((await page.eval('game.digest()')) === digest, 'the loaded file has another digest');

  // A save of another version shows the engine's error on start and stays on start.
  const old = JSON.parse(save);
  old.version = 2;
  const oldFile = join(out, 'start-version-2.json');
  writeFileSync(oldFile, JSON.stringify(old));
  await page.goto('');
  await page.file('#start-load', oldFile);
  await page.until("document.getElementById('start-error').textContent !== ''");
  const error = await page.eval("document.getElementById('start-error').textContent");
  need(/version 2/.test(error), `the error reads ${JSON.stringify(error)}`);
  need((await screen(page)) === 'start', 'a save of another version left start');

  // Continue, once a game has been autosaved, goes back to it: the line names the turn.
  await page.goto('?size=small&seed=2&run=1');
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  await page.button('End turn');
  await page.goto('');
  need(!(await page.eval("document.getElementById('continue').disabled")), 'Continue is greyed after a game was autosaved');
  need(/turn \d+, small seed 2$/.test(await page.eval("document.getElementById('continue-line').textContent")), 'the Continue line does not name the game');

  need(page.errors().length === 0, `page errors: ${page.errors().join('; ')}`);
  console.log(JSON.stringify({ turn, digest }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
