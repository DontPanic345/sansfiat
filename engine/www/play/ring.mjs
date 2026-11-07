// The ring in the page (build-3): on small seed 2 the led civ raises an army, and selecting the army draws the
// ring and covers the capital's tile; selecting the civ still draws it; selecting a clan takes it away. The tile
// tooltip, with an army selected, says the ring's share on the capital ("Ring: its line takes 0% of an army's
// days here") and that a tile past the ring can't be fed ("Past the ring: its line can't feed an army here").
// Writes ring.png (the army selected, its ring drawn) to --out.   node www/play/ring.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open, raiseArmy } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-ring-')));
mkdirSync(out, { recursive: true });

let failed = false;
const fail = (msg) => {
  console.error(`ring: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

/** Clicks tile `i` until the selection is `key` (a click on a stack moves to its next thing). */
async function selectKey(page, key, i) {
  for (let n = 0; n < 8; n++) {
    if ((await page.data()).selected === key) return;
    await page.click(i);
    await page.eval(frames);
  }
  throw new Error(`clicking tile ${i} never selected ${key}`);
}

/** Whether the ring is drawn (the layer on, a ring set) and covers tile `tile`. */
const drawn = (page, tile) => page.eval(`(() => { const m = game.map; return !!(m.on.ring && m.ring && m.ring.shares.has(${tile})); })()`);

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  const civ = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${civ}].capital`);
  await raiseArmy(page);
  const army = await page.eval(`(() => { const u = Object.values(game.view().units).find((u) => u.civ === ${civ} && u.kind === 'camp'); return u && { id: u.id, tile: u.tile }; })()`);
  need(!!army, 'the civ has an army to select');
  if (!army) throw new Error('no army raised');

  // The army selected: its ring is drawn and covers the capital, which is no share at all (0).
  await selectKey(page, `unit:${army.id}`, army.tile);
  need(await drawn(page, cap), 'selecting the army draws its civ’s ring, and the ring covers the capital’s tile');
  const capShare = await page.eval(`game.map.ring.shares.get(${cap})`);
  need(capShare === 0, `the capital is 0 of an army's days (${capShare})`);

  // The civ selected: still drawn.
  await page.eval(`document.querySelector('button.chip[data-civ="${civ}"]').click()`);
  await page.eval(frames);
  need((await page.data()).selected === `civ:${civ}`, 'the bar chip selects the civ');
  need(await drawn(page, cap), 'selecting the civ still draws its ring');

  // A clan selected: the ring is gone.
  const clan = await page.eval(`(() => { const v = game.view(); const c = Object.entries(v.clans).find(([id, k]) => k.home != null && k.home !== ${cap} && !Object.values(v.units).some((u) => u.tile === k.home)); return c && { id: +c[0], home: c[1].home }; })()`);
  need(!!clan, 'a clan with a home to select');
  if (clan) {
    await selectKey(page, `clan:${clan.id}`, clan.home);
    need((await page.eval('!!game.map.ring')) === false, 'selecting a clan takes the ring away');
  }

  // The tooltip, with the army selected: the capital's share, and a tile past the ring.
  await selectKey(page, `unit:${army.id}`, army.tile);
  const capTip = await page.tip(cap);
  need(capTip && capTip.includes("Ring: its line takes 0% of an army's days here"), `the capital's tooltip has its ring line (${JSON.stringify(capTip)})`);
  const past = await page.eval(`[...game.map.ring.shares].filter(([i, s]) => s >= 1).sort((a, b) => a[1] - b[1]).slice(0, 20).map(([i]) => i)`);
  need(past.length > 0, 'the ring has tiles past it to hover');
  let pastTip = null;
  for (const t of past) {
    const tip = await page.tip(t);
    if (tip && tip.includes('Past the ring: its line can\'t feed an army here')) {
      pastTip = { t, tip };
      break;
    }
  }
  need(!!pastTip, `a tile past the ring says so in its tooltip${pastTip ? ` (tile ${pastTip.t})` : ''}`);

  // The army selected again, for the screenshot.
  await selectKey(page, `unit:${army.id}`, army.tile);
  // The shot: the capital centred and near, the tooltip down.
  await page.eval(`(() => { document.getElementById('tip').hidden = true; game.map.centreOn(${cap}, { minK: 14 }); })()`);
  await page.eval(frames);
  await page.shot(join(out, 'ring.png'), 1);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
