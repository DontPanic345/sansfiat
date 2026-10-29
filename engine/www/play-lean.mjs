// Plays a civ's gift in the page, headless: leads the first civ, raises its gift three steps above the
// default, and ends 30 turns. On the first turn an unaligned clan leans towards the civ, it screenshots
// the civ's panel (its "Leaning clans") and then the panel of the strongest such clan, reached by clicking
// its row. Usage:
//
//   node www/play-lean.mjs [out-dir]     (default: relay/specs/55/screens; needs www/pkg)
//
// Uses play.mjs's driver. Prints what the civ did, and exits non-zero if the page reported an error.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, '../../relay/specs/55/screens'));
mkdirSync(out, { recursive: true });

const page = await open('?size=small&seed=2&run=1', { height: 1000 });
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  const civ = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${civ}].capital`);
  await page.select(cap, 'capital');
  const before = await page.eval(`game.view().civs[${civ}].gift`);
  for (let i = 0; i < 3; i++) await page.button('Gift +1');
  // The unaligned clans leaning to the civ, strongest first (a clan joins once it reaches LEANING.join).
  const leaners = () => page.eval(`Object.values(game.view().clans)
    .filter((k) => k.civ == null && (k.leaning[${civ}] ?? 0) > 0)
    .sort((a, b) => b.leaning[${civ}] - a.leaning[${civ}] || a.id - b.id)
    .map((k) => ({ id: k.id, home: k.home, leaning: k.leaning[${civ}], people: k.people }))`);
  let first = null;
  for (let turn = 0; turn < 30; turn++) {
    await page.button('End turn');
    if (first) continue;
    const seen = await leaners();
    if (!seen.length) continue;
    // The civ's panel with its leaning clans, then the strongest clan's own panel.
    first = { turn: await page.eval('game.view().turn'), clan: seen[0], count: seen.length };
    await page.select(cap, 'capital');
    await page.shot(join(out, 'civ-panel.png'), 1);
    await page.eval(`document.querySelector('[data-select="clan:${seen[0].id}"]').click()`);
    const selected = await page.eval("document.body.dataset.selected");
    if (selected !== `tile:${seen[0].home}`) throw new Error(`the clan's row selected ${selected}, not its home tile`);
    await page.shot(join(out, 'clan-leaning.png'), 1);
  }
  if (!first) throw new Error('no unaligned clan leaned towards the civ in 30 turns');
  console.log(JSON.stringify({ first_leaning: first }));

  const state = await page.eval(`(() => { const c = game.view().civs[${civ}]; return { gift: c.gift, given: c.given,
    posted: c.posted, joined: c.joined, gifted: c.gifted, keep: c.keep, keep_set: c.keep_set }; })()`);
  console.log(JSON.stringify({ civ, gift_before: before, turn: await page.eval('game.view().turn'), ...state }));

  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length) process.exitCode = 1;
} finally {
  await page.close();
}
