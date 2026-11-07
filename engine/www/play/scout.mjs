// A scout in the page (touch point 19). On small seed 2: Move to… sends it to a tile (its march is drawn, and it
// stands there after a few turns); Route from… rings the start on the capital; Route to… sets the route from it;
// Clear route clears it; a tile ten steps out with Move to… armed says it is past the line's reach; and a scout
// sent out with its switches off goes hungry, its panel saying why. On small seed 12 the scout's route to the rival's
// capital leaves a mountain unrevealed, and the panel lists it. Four screenshots go into --out: the march, the start ring,
// the unrevealed list and the hungry panel.
//   node www/play/scout.mjs [--out DIR]
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
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

/** The tile tooltip's text after hovering tile `i` (the page's #tip, once it shows). */
async function tipAt(page, i) {
  await page.hover(i);
  await sleep(600);
  return page.eval("(() => { const t = document.getElementById('tip'); return t && !t.hidden ? t.textContent : ''; })()");
}

/** Leads the first civ of `seed` into its main phase and selects its scout (on its capital, where the camp and the
 *  guard stand too). Returns the ids the script needs. */
async function leadAndSelectScout(page) {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  const scout = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'scout').id`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${scout}`; n++) {
    await page.click(cap);
    await page.eval(frames);
  }
  return { me, cap, scout };
}

/** The tiles `steps` land steps from `from`, nearest first (a breadth-first walk over the land). */
const ringAt = (page, from, steps) =>
  page.eval(`(() => { const m = game.map; const seen = new Map([[${from}, 0]]); let frontier = [${from}];
    for (let d = 0; d < ${steps}; d++) { const next = []; for (const i of frontier) for (let k = 0; k < 6; k++) { const j = m.nb[i * 6 + k];
      if (j >= 0 && m.land[j] && !seen.has(j)) { seen.set(j, d + 1); next.push(j); } } frontier = next; }
    return frontier; })()`);

const scoutOf = (page, id) => page.eval(`game.view().units[${id}]`);

// ---- small seed 2: the errands, the start ring, the reach warning and the hunger -----------------------------------

const page = await open('?size=small&seed=2&run=1');
try {
  const { cap, scout } = await leadAndSelectScout(page);
  need((await page.data()).selected === `unit:${scout}`, `clicking the capital selects the scout ${scout}`);
  let p = await page.panel();
  need(/'s scout \d+$/.test(p.name) && !!p.doing, `the scout's panel is named "${p.name}" with a doing line`);
  const labels = p.buttons.map((b) => b.label);
  for (const want of ['Move to…', 'Route from…', 'Route to…', 'Disband…', 'Disband all']) need(labels.includes(want), `the scout's row has ${want}`);

  // Move to… sends the scout to TARGET: a dest, no route, a line that says so, and the march drawn.
  await page.button('Move to…');
  await page.eval(frames);
  need((await page.eval('document.body.dataset.armed')) === 'move', 'Move to… arms the scout’s move');
  await page.click(TARGET);
  await page.eval(frames);
  let s = await scoutOf(page, scout);
  need(s.dest === TARGET && s.route == null, `Move to… sets the scout's dest to ${TARGET} and no route (dest ${s.dest}, route ${JSON.stringify(s.route)})`);
  p = await page.panel();
  need(p.doing.includes('Going to'), `the doing line says "Going to" (${p.doing})`);
  need(await page.eval(`game.map.routes.some((r) => r.unit === ${scout} && r.tiles.length > 1)`), 'the march path is drawn');
  await page.shot(join(out, 'march.png'), 1);

  // The scout stands at TARGET after a few turns, and its line says "At".
  let turns = 0;
  for (; turns < 15 && (await scoutOf(page, scout)).tile !== TARGET; turns++) await page.button('End turn');
  s = await scoutOf(page, scout);
  need(s.tile === TARGET, `the scout stands at ${TARGET} after ${turns} turns`);
  p = await page.panel();
  need(/(^| · )At /.test(p.doing), `the doing line says "At" once it stands there (${p.doing})`);

  // Route from… rings the start on the capital; Route to… on TARGET sets [capital, TARGET] and draws the known path.
  await page.button('Route from…');
  need((await page.eval('document.body.dataset.armed')) === 'start', 'Route from… arms the start');
  await page.click(cap);
  await page.eval(frames);
  need((await page.eval('document.body.dataset.routeStart')) === String(cap), `the start is the capital (${cap})`);
  p = await page.panel();
  need(p.hint && p.hint.includes('now Route to'), `the hint says the route's start is set (${p.hint})`);
  await page.shot(join(out, 'start-ring.png'), 1);
  await page.button('Route to…');
  await page.click(TARGET);
  await page.eval(frames);
  s = await scoutOf(page, scout);
  need(JSON.stringify(s.route) === JSON.stringify([cap, TARGET]), `Route to… sets the route from the start to ${TARGET} (${JSON.stringify(s.route)})`);
  need((await page.eval('document.body.dataset.routeStart')) === '', 'the start mark clears once the route is sent');
  need(await page.eval('!!game.map.plan && !!game.map.plan.known'), 'the known path is drawn');

  // Clear route clears it.
  await page.button('Clear route');
  await page.eval(frames);
  need((await scoutOf(page, scout)).route == null, 'Clear route clears the route');

  // Escape clears a start that is set and keeps the scout selected; a right-click sends the selected scout to a tile.
  await page.button('Route from…');
  await page.click(cap);
  await page.eval(frames);
  need((await page.eval('document.body.dataset.routeStart')) === String(cap), 'Route from… rings the capital again');
  await page.key('Escape');
  need((await page.eval('document.body.dataset.routeStart')) === '' && (await page.data()).selected === `unit:${scout}`, 'Escape clears the start mark and keeps the scout selected');
  await page.command(cap);
  await page.eval(frames);
  need((await scoutOf(page, scout)).dest === cap, 'a right-click sends the selected scout to the tile');

  // The reach warning: with Move to… armed, a tile ten steps out whose line can't carry what the scout eats.
  await page.button('Move to…');
  const far = await ringAt(page, cap, 10);
  let warned = null;
  for (const i of far.slice(0, 12)) {
    const tip = await tipAt(page, i);
    if (tip.includes("Past the line's reach")) {
      warned = { i, tip };
      break;
    }
  }
  need(warned !== null, `a tile ten steps out says it is past the line's reach${warned ? ` (${warned.tip.split('\n').join(' / ')})` : ''}`);
  await page.key('Escape');

  // Move to… the far tile, its switches off: the scout goes hungry, and its panel says why.
  if (warned) {
    await page.button('Move to…');
    await page.click(warned.i);
    await page.eval(frames);
  }
  for (const label of ['Auto-forage', 'Auto-supply']) {
    const on = await page.eval(`[...document.querySelectorAll('.actions button')].some((b) => b.textContent.startsWith('${label}: on'))`);
    if (on) await page.button(label);
  }
  need((await scoutOf(page, scout)).auto_forage === false && (await scoutOf(page, scout)).auto_supply === false, 'the scout’s switches are off');
  let hungry = 0;
  for (; hungry < 80 && !((await scoutOf(page, scout)).hungry > 0); hungry++) await page.button('End turn');
  need((await scoutOf(page, scout)).hungry > 0, `the scout goes hungry after ${hungry} turns`);
  p = await page.panel();
  need(p.doing.startsWith('Hungry'), `the doing line leads with the hunger (${p.doing})`);
  need(p.numbers.some((n) => n.label === 'Hunger' && n.value !== 'fed'), `the Hunger row gives its reason (${p.numbers.find((n) => n.label === 'Hunger')?.value})`);
  await page.shot(join(out, 'hungry.png'), 1);
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}

// ---- small seed 12: the route to the rival's capital leaves one mountain unrevealed ----------------------------------
// (Seed 9's scout to the rival is lost by turn 8 under the feeding at the starting numbers, build-5 tunes that; seed 12's
// lives to turn 60 with one mountain left, `relay/specs/153/page.md`.)

const nine = await open('?size=small&seed=12&run=1');
try {
  const { me, cap, scout } = await leadAndSelectScout(nine);
  const rival = await nine.eval(`(() => { const v = game.view(); return v.civs[Object.keys(v.civs).map(Number).find((k) => k !== ${me})].capital; })()`);
  await nine.button('Route to…');
  await nine.click(rival);
  await nine.eval(frames);
  const route = await scoutOf(nine, scout);
  need(JSON.stringify(route.route) === JSON.stringify([cap, rival]), `the scout's route runs from its capital to the rival's (${rival})`);
  // Search turn by turn until a mountain is left: the panel lists it.
  let left = 0;
  let p = null;
  let alive = true;
  for (let n = 0; n < 60 && alive; n++) {
    const u = await scoutOf(nine, scout);
    alive = u !== undefined && u !== null;
    left = alive ? (u.search?.left?.length ?? 0) : 0;
    if (left > 0) break;
    await nine.button('End turn');
  }
  need(alive, 'the scout is alive on its errand');
  need(left > 0, 'a mountain is left unrevealed on the errand');
  p = await nine.panel();
  need(p.lists.some((l) => l.label === 'Left unrevealed' && l.rows.length > 0), 'the panel lists the left mountains under Left unrevealed');
  need(p.numbers.some((n) => n.label === 'Unrevealed' && /to check; \d+ left/.test(n.value)), `the Unrevealed row says how many are left (${p.numbers.find((n) => n.label === 'Unrevealed')?.value})`);
  // A row of the list centres the map on its mountain and leaves the scout selected.
  const before = (await nine.data()).selected;
  await nine.eval("document.querySelector('.list .listrow button').click()");
  await nine.eval(frames);
  need((await nine.data()).selected === before, 'a row of Left unrevealed leaves the scout selected');
  await nine.shot(join(out, 'unrevealed.png'), 1);
  for (const e of nine.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await nine.close();
}

process.exitCode = failed ? 1 : 0;
