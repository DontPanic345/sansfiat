// Selecting on the map (touch point 21): a click on a clan's home selects the clan, with a panel and no
// buttons; a festival selects, listing its clans, with their walkers drawn; a bare tile's panel names whose
// it is; clicking again cycles a stack; Escape steps back through an armed Move to…, painting, then the
// selection.   node www/play/select.mjs [--out DIR]     (small seed 1)
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open, raiseArmy } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-select-')));
mkdirSync(out, { recursive: true });

let failed = false;
const fail = (msg) => {
  console.error(`select: ${msg}`);
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
    if ((await page.data()).selected === key) return true;
    await page.click(i);
    await page.eval(frames);
  }
  return false;
}

const page = await open('?size=small&seed=1&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  await raiseArmy(page);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);

  // A clan's home: the clan, its panel, and no buttons.
  const clan = await page.eval(`(() => { const v = game.view(); const k = Object.values(v.clans).find((k) => k.civ == null && !Object.values(v.units).some((u) => u.tile === k.home) && !Object.values(v.civs).some((c) => c.capital === k.home));
    return k ? { id: k.id, home: k.home } : null; })()`);
  need(clan !== null, 'a clan with its home on an empty tile');
  need(await selectKey(page, `clan:${clan.id}`, clan.home), `a click on clan ${clan.id}'s home selects the clan`);
  let p = await page.panel();
  need(p && /^Clan \d+$/.test(p.name) && p.numbers.length >= 2, `the clan's panel is named "${p?.name}" with numbers`);
  need(p.buttons.length === 0, 'a clan has no buttons');
  await page.shot(join(out, 'clan.png'), 1);

  // A festival, with the clans going to it: listed, and their walkers drawn bright.
  const market = await page.eval(`(() => { const ms = Object.values(game.view().markets).sort((a, b) => b.clans.length - a.clans.length || a.id - b.id); return ms.length ? { id: ms[0].id, tile: ms[0].tile, clans: ms[0].clans.length } : null; })()`);
  need(market !== null && market.clans > 0, 'a festival with clans going to it');
  if (market) {
    need(await selectKey(page, `festival:${market.id}`, market.tile), `a festival selects (festival ${market.id})`);
    p = await page.panel();
    need(p.lists.some((l) => l.label === 'Clans going' && l.rows.length === market.clans), `the festival lists its ${market.clans} clans`);
    need((await page.eval('game.map.focusMarket')) === market.id, 'the festival’s walkers are drawn while it is selected');
    need(p.buttons.length === 0, 'a festival has no buttons');
    await page.shot(join(out, 'festival.png'), 1);
    // Its clan's row selects the clan.
    await page.eval(`[...document.querySelectorAll('#detail .listrow button')][0].click()`);
    need(/^clan:\d+$/.test(await page.eval('document.body.dataset.selected')), 'a clan row in the festival selects the clan');
  }

  // A bare tile: a land tile of the player's, with no unit, clan or festival on it, names whose it is.
  const bare = await page.eval(`(() => { const v = game.view(), c = v.civs[${me}];
    const busy = new Set([...Object.values(v.units).map((u) => u.tile), ...Object.values(v.clans).map((k) => k.home), ...Object.values(v.markets).map((m) => m.tile), ...Object.values(v.civs).map((x) => x.capital)]);
    return c.land.find((t) => !busy.has(t)) ?? null; })()`);
  need(bare !== null, 'a bare tile of the player’s land');
  if (bare !== null) {
    need(await selectKey(page, `tile:${bare}`, bare), `a bare tile selects (tile ${bare})`);
    p = await page.panel();
    need(p.doing === `${await page.eval(`game.view().civs[${me}].name`)}'s land`, `the bare tile's panel names whose it is ("${p.doing}")`);
  }

  // Clicking again cycles a stack: the capital has the player's camp and scout, then the civ.
  const seen = [];
  for (let n = 0; n < 4; n++) {
    seen.push(await page.eval("document.body.dataset.selected"));
    await page.click(cap);
    await page.eval(frames);
  }
  need(new Set(seen).size >= 3, `clicking again cycles the stack (${seen.join(', ')})`);

  // Escape: an armed Move to…, then painting, then the selection.
  const camp = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'camp').id`);
  await selectKey(page, `unit:${camp}`, cap);
  await page.button('Move to…');
  need((await page.eval("document.body.dataset.armed")) === 'move', 'Move to… is armed');
  await page.key('Escape');
  need((await page.eval("document.body.dataset.armed")) === '' && (await page.data()).selected === `unit:${camp}`, 'Escape disarms the armed click and keeps the selection');
  await page.key('p');
  need((await page.eval("document.body.dataset.painting")) === 'paint', 'P arms painting');
  await page.key('Escape');
  need((await page.eval("document.body.dataset.painting")) === '' && (await page.data()).selected === `unit:${camp}`, 'Escape stops painting and keeps the selection');
  await page.key('Escape');
  need((await page.data()).selected === '', 'Escape clears the selection');
  need((await page.panel()) === null, 'the panel is gone with the selection');
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
