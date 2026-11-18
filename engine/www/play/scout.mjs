// A scout in the page (touch point 19). On a staged game of forest and hills on a plains map, the led civ's scout on its
// capital: Move to… sends it to a forest tile (its march is drawn, and it stands there after a few turns); Route from…
// rings the start on the capital; Route to… sets the route from it; Clear route clears it; its panel gives the pack and
// the store, with no switch or feeding button. A second staged game, with a mountain of hidden trail cost on the way to
// the rival's capital, has the scout's route leave the mountain unrevealed, and the panel lists it. Four screenshots go
// into --out: the march, the start ring, the unrevealed list and the pack panel.
//   node www/play/scout.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { grid, marks, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-scout-')));
mkdirSync(out, { recursive: true });

let TARGET; // the forest tile the scout marches to (mark T)
// Radius 8: the led civ's capital at A (-6, 0), the rival's at B (6, 0); a forest on the way at T (-3, 0) and round A's
// ring, hills at (-1, 0) and (1, 1); sea on the rim.
const R = 8;
const FOREST = [[-3, 0], [-3, 1], [-2, -1], [-5, 0]];
const HILLS = [[-1, 0], [1, 1]];
const plainsMap = (mountain) =>
  grid(R, (q, r) => {
    if (Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r)) === R) return '~';
    if (mountain && q === 0 && r === 0) return '^';
    if (FOREST.some(([a, b]) => a === q && b === r)) return 'f';
    if (HILLS.some(([a, b]) => a === q && b === r)) return 'h';
    return '.';
  });
const MARKS = { A: [-6, 0], B: [6, 0], T: [-3, 0], M: [0, 0] };
const SPEC = (mountain) => ({
  map: plainsMap(mountain),
  marks: marks(R, MARKS),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  // The led civ's camp at the capital and its scout, with a pack of 30 grain; the rival's scout is set as a rival's is.
  scouts: [{ civ: 0, at: { mark: 'A' }, pack: 30 }, { civ: 1, at: { mark: 'B' }, pack: 30 }],
  stores: [{ civ: 0, food: 50 }],
  // Hidden trail cost on the mountain: the scout's search leaves it unrevealed.
  hidden: mountain ? [{ at: { mark: 'M' }, trailCost: 9 }] : [],
});
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

/** Stages `spec` and selects the led civ's scout (on its capital, where the camp stands too). Returns the ids the
 *  script needs. */
async function stageAndSelectScout(page, spec) {
  await stage(page, spec);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  const scout = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'scout').id`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${scout}`; n++) {
    await page.click(cap);
    await page.eval(frames);
  }
  return { me, cap, scout };
}

const scoutOf = (page, id) => page.eval(`game.view().units[${id}]`);

// ---- the errands, the start ring and the pack: the staged forest game ----------------------------------------------

const page = await open('');
try {
  const { cap, scout } = await stageAndSelectScout(page, SPEC(false));
  need((await page.data()).selected === `unit:${scout}`, `clicking the capital selects the scout ${scout}`);
  TARGET = await page.eval(`game.layout().q.findIndex((q, i) => q === ${MARKS.T[0]} && game.layout().r[i] === ${MARKS.T[1]})`);
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
  need(/^(Idle|Walking to its destination)/.test(p.doing), `the doing line is the engine's words (${p.doing}); a move's first turn is yet to run`);
  need(await page.eval(`game.map.routes.some((r) => r.unit === ${scout} && r.tiles.length > 1)`), 'the march path is drawn');
  await page.shot(join(out, 'march.png'), 1);

  // The scout stands at TARGET after a few turns, and its line says "At".
  let turns = 0;
  for (; turns < 40 && (await scoutOf(page, scout)).tile !== TARGET; turns++) await page.button('End turn');
  s = await scoutOf(page, scout);
  need(s.tile === TARGET, `the scout stands at ${TARGET} after ${turns} turns`);
  p = await page.panel();
  need(/^(Idle|Walking|Waiting)/.test(p.doing), `the doing line is the engine's words once it stands there (${p.doing})`);

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

  // The scout eats from its pack: its panel has the pack and the store, and no switch or feeding button.
  p = await page.panel();
  const names = p.buttons.map((b) => b.label);
  need(!names.some((l) => /^(Auto-|Forage|Resupply|Pillage)/.test(l)), `a scout has no switches or feeding orders (${names.join(', ')})`);
  need(p.numbers.some((n) => n.label === 'Pack') && p.numbers.some((n) => n.label === 'Store'), 'the panel gives its pack and its store');
  need(!p.numbers.some((n) => n.label === 'Hunger'), 'the panel has no Hunger row');
  need(!p.doing.includes('oraging'), `the doing line never says foraging (${p.doing})`);
  await page.shot(join(out, 'pack.png'), 1);
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}

// ---- the route to the rival's capital: a mountain of hidden trail cost is left unrevealed (its own staged game) ---------

const nine = await open('');
try {
  const { me, cap, scout } = await stageAndSelectScout(nine, SPEC(true));
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
