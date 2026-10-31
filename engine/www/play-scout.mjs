// Plays a scout in the page, headless: leads the first civ, selects its scout (it shares the capital with
// its camp, so clicks there until the scout is selected), picks the route's start and end only by clicking
// the map (seed 2, small map: the capital to tile 58, hills seven steps out, as `core/src/script.rs` gives), ends
// turns until the route is found or no route can exist, and screenshots into the out dir: while both
// routes show, and after a trail on the route is built. Usage:
//
//   node www/play-scout.mjs [out-dir]     (default: relay/specs/28/screens; needs www/pkg)
//
// Uses play.mjs's driver. Prints what the scout did, and exits non-zero if the route was not found or the
// page reported an error.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, '../../relay/specs/28/screens'));
mkdirSync(out, { recursive: true });

const TARGET = 58; // seed 2, small: the first forest or hills tile seven steps from the capital (tile 166)
const page = await open('?size=small&seed=2&run=1');
let exitCode = 0;
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");

  // The scout, selected from the capital where the camp stands too.
  const scout = await page.eval('Object.values(game.view().units).find((u) => u.civ === game.view().player && u.kind === "scout").id');
  const capital = await page.eval('game.view().civs[game.view().player].capital');
  for (let n = 0; n < 6; n++) {
    if ((await page.data()).selected === `unit:${scout}`) break;
    await page.click(capital);
  }
  if ((await page.data()).selected !== `unit:${scout}`) throw new Error(`clicking the capital never selected the scout ${scout}`);

  // The start, then the end: each by a map click after its button.
  await page.button('Start here');
  await page.click(capital);
  await page.button('Route to here');
  await page.click(TARGET);
  const set = await page.data();
  if (set.routeState !== 'set') throw new Error(`the route was not set (routeState ${set.routeState}); the error: ${await page.eval("document.getElementById('error').textContent")}`);
  const plan = await page.eval(`(() => { const u = game.view().units[${scout}]; return { route: u.route, known: u.plan.known && u.plan.known.cost,
    possible: u.plan.possible && u.plan.possible.cost, steps: u.plan.known && u.plan.known.path.length }; })()`);
  console.log(JSON.stringify({ scout, capital, target: TARGET, ...plan }));
  // The route's path, and the trails already standing on it (the clans' walks, not the scout's).
  const path = await page.eval(`game.view().units[${scout}].plan.known.path`);
  const standing = await page.eval(`(() => { const t = new Set(game.view().trails); return ${JSON.stringify(path)}.filter((i) => t.has(i)); })()`);
  // The shots centre on the route, close enough to read its dashes and trails.
  const focus = (tile) => page.eval(`game.map.centreOn(${tile}, { minK: 14, smooth: false })`);
  await focus(path[Math.floor(path.length / 2)]);
  await page.shot(join(out, 'route-both.png'), 1);

  // End turns until the route is found or no route can exist; shoot once the scout builds a trail on it.
  let shotTrail = false;
  let turns = 0;
  let state = '';
  for (; turns < 80; ) {
    await page.button('End turn');
    turns++;
    const d = await page.data();
    state = d.routeState;
    if (!shotTrail) {
      const built = await page.eval(`(() => { const t = new Set(game.view().trails); return ${JSON.stringify(path)}.filter((i) => t.has(i) && !${JSON.stringify(standing)}.includes(i)); })()`);
      if (built.length > 0) {
        await focus(built[0]);
        await page.shot(join(out, 'trail-built.png'), 1);
        shotTrail = true;
      }
    }
    if (state === 'found' || state === 'nopath') break;
  }
  const end = await page.eval(`(() => { const v = game.view(); const u = v.units[${scout}];
    return { turn: v.turn, trails: v.trails.length, scout: u ? { people: u.people, hungry: u.hungry, tile: u.tile } : null }; })()`);
  console.log(JSON.stringify({ turns, routeState: state, shotTrail, ...end }));
  if (state !== 'found') {
    console.error(`the route did not come: ${state}`);
    exitCode = 1;
  }
} finally {
  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length) exitCode = 1;
  await page.close();
}
process.exitCode = exitCode;
