// Plays a camp in the page, headless: leads the first civ, selects its camp, moves it about three steps
// out on its own land, ends turns until it has walked there, and screenshots the line home; then marches
// it past its line's reach (off rival land, seven or more steps out) until it goes hungry, and back to
// its post until it is fed again, screenshotting each. Usage:
//
//   node www/play-camp.mjs [out-dir]     (default: relay/specs/29/screens; needs www/pkg)
//
// Uses play.mjs's driver. Prints what the camp did, and exits non-zero if the page reported an error.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, '../../relay/specs/29/screens'));
mkdirSync(out, { recursive: true });

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  // The camp at the capital, and a tile three steps out on the civ's own land.
  const plan = await page.eval(`(() => {
    const v = game.view(), L = game.layout(), civ = v.player, cap = v.civs[civ].capital;
    const steps = (a, b) => {
      const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b];
      return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr));
    };
    const camp = Object.values(v.units).find((u) => u.civ === civ);
    // The land tile nearest three steps out (a small map's land may not reach that far).
    const target = v.civs[civ].territory
      .filter((i) => i !== cap)
      .sort((a, b) => Math.abs(steps(a, cap) - 3) - Math.abs(steps(b, cap) - 3) || a - b)[0];
    return { civ, camp: camp.id, cap, target };
  })()`);
  if (plan.target === undefined) throw new Error('the civ has no land but its capital');
  await page.select(plan.cap, 'unit');
  await page.command(plan.target);
  await page.shot(join(out, 'camp-ordered.png'), 1);
  for (let turn = 0; turn < 8; turn++) await page.button('End turn');
  const state = await page.eval(`(() => { const u = game.view().units[${plan.camp}]; return { tile: u.tile, cut_off: u.cut_off,
    line: u.line && { steps: u.line.path.length, cost: u.line.cost, carry: u.line.carry }, food_turns: u.food_turns, hungry: u.hungry }; })()`);
  console.log(JSON.stringify({ camp: plan.camp, target: plan.target, ...state }));
  if (state.tile !== plan.target) console.log('the camp has not reached its target yet');
  await page.shot(join(out, 'camp-line.png'), 1);

  // Past its reach: the nearest land tile seven or more steps out that no rival holds, on a path.
  const far = await page.eval(`(() => {
    const v = game.view(), L = game.layout(), civ = v.player, cap = v.civs[civ].capital;
    const steps = (a, b) => {
      const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b];
      return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr));
    };
    const rival = new Set(Object.entries(v.civs).filter(([id]) => +id !== civ).flatMap(([, c]) => c.territory));
    return [...Array(L.q.length).keys()]
      .filter((i) => steps(i, cap) >= 7 && !rival.has(i))
      .sort((a, b) => steps(a, cap) - steps(b, cap) || a - b)
      .slice(0, 40);
  })()`);
  const camp = `game.view().units[${plan.camp}]`;
  const status = () => page.eval(`(() => { const u = ${camp}; return u && { tile: u.tile, warriors: u.people, hungry: u.hungry,
    cut_off: u.cut_off, pack: u.store.reduce((a, b) => a + b, 0), line: u.line && { steps: u.line.path.length, cost: u.line.cost, carry: u.line.carry } }; })()`);
  let gone;
  for (const tile of far) {
    await page.command(tile);
    if (await page.eval(`${camp}.dest === ${tile}`)) {
      gone = tile;
      break;
    }
  }
  if (gone === undefined) throw new Error('no tile past the reach to march to');
  let s;
  for (let turn = 0; turn < 20; turn++) {
    await page.button('End turn');
    s = await status();
    console.log(JSON.stringify({ out: turn + 1, ...s }));
    if (s.hungry > 0) break;
  }
  if (!s || s.hungry === 0) throw new Error('the camp never went hungry past its reach');
  await page.shot(join(out, 'camp-hungry.png'), 1);
  await page.command(plan.target);
  for (let turn = 0; turn < 20; turn++) {
    await page.button('End turn');
    s = await status();
    console.log(JSON.stringify({ back: turn + 1, ...s }));
    if (s.tile === plan.target && s.hungry === 0 && !s.cut_off) break;
  }
  if (s.hungry > 0 || s.cut_off) throw new Error('the camp was not fed again at its post');
  await page.shot(join(out, 'camp-back.png'), 1);
  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length) process.exitCode = 1;
} finally {
  await page.close();
}
