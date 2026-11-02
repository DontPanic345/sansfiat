// A scout in the page (touch point 19): Move to… sets a route from the scout's own tile; Route from… then
// Move to… sets one from a picked start; Clear route clears it. Leads the first civ on small seed 2, where the
// route from the capital to tile 58 is found (`core/src/script.rs`).   node www/play/scout.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-scout-')));
mkdirSync(out, { recursive: true });

const TARGET = 58; // seed 2, small: the first forest or hills tile seven steps from the capital
let failed = false;
const fail = (msg) => {
  console.error(`scout: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  const scout = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'scout').id`);

  // The scout is selected from the capital, where the camp and the guard stand too.
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${scout}`; n++) {
    await page.click(cap);
    await page.eval(frames);
  }
  need((await page.data()).selected === `unit:${scout}`, `clicking the capital selects the scout ${scout}`);
  let p = await page.panel();
  need(/'s scout$/.test(p.name) && !!p.doing, `the scout's panel is named "${p.name}" with a doing line`);
  const labels = p.buttons.map((b) => b.label);
  for (const want of ['Move to…', 'Route from…', 'Clear route', 'Disband…']) need(labels.includes(want), `the scout's row has ${want}`);

  // Move to… sets a route from the scout's tile.
  await page.button('Move to…');
  await page.eval(frames);
  need((await page.eval("document.body.dataset.armed")) === 'to', 'Move to… arms the route’s end');
  await page.click(TARGET);
  await page.eval(frames);
  const route1 = await page.eval(`game.view().units[${scout}].route`);
  need(JSON.stringify(route1) === JSON.stringify([cap, TARGET]), `Move to… sets a route from the scout's tile (${JSON.stringify(route1)})`);
  need((await page.data()).routeState === 'set', 'the route is set on the page');

  // Clear route clears it.
  await page.button('Clear route');
  await page.eval(frames);
  need((await page.eval(`game.view().units[${scout}].route`)) == null, 'Clear route clears the route');

  // Route from… arms the start; a click there, then Move to… sets the route from it.
  // The start: a land neighbour of the capital, not the scout's own tile.
  const start = await page.eval(`(() => { const m = game.map; for (let d = 0; d < 6; d++) { const j = m.nb[${cap} * 6 + d]; if (j >= 0 && m.land[j]) return j; } return null; })()`);
  need(start !== null, 'the capital has a land neighbour to start from');
  await page.button('Route from…');
  need((await page.eval("document.body.dataset.armed")) === 'start', 'Route from… arms the start');
  await page.click(start);
  await page.eval(frames);
  await page.button('Move to…');
  await page.click(TARGET);
  await page.eval(frames);
  const route2 = await page.eval(`game.view().units[${scout}].route`);
  need(JSON.stringify(route2) === JSON.stringify([start, TARGET]), `Route from… then Move to… sets a route from the picked start (${JSON.stringify(route2)})`);
  p = await page.panel();
  need(p.buttons.some((b) => b.label === 'Clear route' && !b.disabled), 'the route can be cleared again');
  await page.shot(join(out, 'scout.png'), 1);

  // The scout searches the route turn by turn: it is found, or no route can exist, within 80 turns.
  let state = '';
  let turns = 0;
  for (; turns < 80 && state !== 'found' && state !== 'nopath'; turns++) {
    await page.button('End turn');
    state = (await page.data()).routeState;
  }
  need(state === 'found', `the scout finds the route in ${turns} turns (${state})`);

  // Escape disarms an armed route click.
  await page.button('Route from…');
  await page.key('Escape');
  need((await page.eval("document.body.dataset.armed")) === '', 'Escape disarms Route from…');
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
