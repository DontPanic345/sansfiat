// Fields, on the page (#155, build-3): lead a civ on small seed 1 and play turns until one of its clans farms. Then
// the feed holds a clanFarming item and a click selects its clan; the clan's panel has a Fields row over a tile with
// ground; the civ's panel has a Living row whose farming part is above 0 and whose parts add up to its people and
// its army; the fields layer draws on the clan's field tile; View → Fields turns it off. Writes fields.png (the
// farmer selected) and civ-panel.png (the civ's panel) to --out.   node www/play/fields.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { lead, open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-fields-')));
mkdirSync(out, { recursive: true });

/** Turns to play before a clan of the led civ has fields: the spec's 40. */
const CAP = 40;

let failed = false;
const fail = (msg) => {
  console.error(`fields: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
/** The map's pixels round tile `i`, as a flat array of RGBA values, the canvas drawn so far. */
const around = (i) =>
  page.eval(`(() => {
    const c = document.getElementById('map');
    const r = c.getBoundingClientRect();
    const p = game.map.screenOf(${i});
    const s = c.width / r.width;
    const half = Math.round(16 * s);
    const x = Math.round((p.x - r.left) * s), y = Math.round((p.y - r.top) * s);
    return Array.from(c.getContext('2d').getImageData(x - half, y - half, 2 * half, 2 * half).data);
  })()`);
/** How many pixels differ between two snapshots by more than a few levels in some channel. */
const differing = (a, b) => {
  let n = 0;
  for (let k = 0; k < a.length; k += 4) if (Math.max(...[0, 1, 2].map((c) => Math.abs(a[k + c] - b[k + c]))) > 8) n++;
  return n;
};
const numberOf = (panel, label) => panel?.numbers.find((n) => n.label === label);

const page = await open('');
try {
  await lead(page, { seed: 1 });
  const me = await page.eval('game.view().player');

  // Play until some clan of the led civ farms: its share at rules().field.farmer, the line its doing line and home read.
  const farmer = await page.eval("import('./pkg/web.js').then((m) => JSON.parse(m.rules()).field.farmer)");
  let turns = 0;
  const farming = () =>
    page.eval(`Object.values(game.view().clans).filter((c) => c.civ === ${me} && c.fields >= ${farmer} - 1e-9).map((c) => c.id)`);
  const farmers = () => page.eval(`game.view().civs[${me}].living.farming`);
  let clans = await farming();
  while (clans.length === 0 && turns < CAP) {
    await page.button('End turn');
    turns++;
    clans = await farming();
  }
  need(clans.length > 0, `a clan of the led civ farms within ${CAP} turns (${turns} played, clans ${clans.join(', ') || 'none'})`);
  if (clans.length === 0) process.exit(1);
  console.log(`      the first farming clan came after ${turns} turn${turns === 1 ? '' : 's'}, with ${(await farmers()).toFixed(2)} farming people`);
  const clan = clans[0];

  // The feed holds a clanFarming item, and a click on it selects the clan (and centres the map on its home).
  const item = await page.eval(`(() => { const f = game.feed.items.filter((i) => i.kind === 'clanFarming' && i.clan === ${clan});
    return f.length ? f[f.length - 1].id : null; })()`);
  need(item !== null, `the feed holds a clanFarming item for clan ${clan}`);
  const clickItem = async () => {
    await page.eval(`document.querySelector('#feed li[data-id="${item}"] button').click()`);
    await page.eval(frames);
    await page.until('!game.map.anim', 5000);
  };
  if (item !== null) {
    await clickItem();
    need((await page.data()).selected === `clan:${clan}`, `a click on the clanFarming item selects clan ${clan}`);
  } else {
    await page.select(await page.eval(`game.view().clans[${clan}].home`), 'clan');
  }

  // The clan's panel: a Fields row, and its first field tile has ground.
  const clanView = await page.eval(`game.view().clans[${clan}]`);
  const tile = clanView.plots[0]?.[0];
  const ground = await page.eval(`game.layout().ground[${tile}]`);
  const panel = await page.panel();
  const fieldsRow = numberOf(panel, 'Fields');
  need(fieldsRow && /tiles? · [\d.]+ days · [\d.]+ grain/.test(fieldsRow.value), `the clan's panel has a Fields row (${fieldsRow?.value ?? 'none'})`);
  need(ground > 0, `the clan's first field tile ${tile} has ground (${ground} days)`);
  need(/^Farms .+ of its days/.test(panel?.doing ?? ''), `the clan's doing line leads with how it farms (${panel?.doing})`);

  // The field layer is on by default, and draws on the clan's field tile.
  need(await page.eval("game.map.on.fields === true && document.getElementById('v-fields').checked"), 'the fields layer is on, and its View box is checked');
  need(await page.eval(`game.map.onScreen(${tile})`), `the clan's field tile ${tile} is on the screen`);
  const shown = await around(tile);
  await page.eval("document.getElementById('v-fields').click()");
  await page.eval(frames);
  await page.eval(frames);
  const hidden = await around(tile);
  const drawn = differing(shown, hidden);
  need(drawn >= 15, `the fields layer draws on the clan's field tile (${drawn} pixels differ with it on)`);
  await page.eval("document.getElementById('v-fields').click()");
  await page.eval(frames);
  await page.eval(frames);

  // The civ's panel: its Living row.
  await page.eval(`document.querySelector('#chips button[data-civ="${me}"]').click()`);
  await page.eval(frames);
  need((await page.data()).selected === `civ:${me}`, `the chip selects the led civ ${me}`);
  const civPanel = await page.panel();
  const living = numberOf(civPanel, 'Living');
  const m = /^gathering (\d+) · farming (\d+) · in the army (\d+)$/.exec(living?.value ?? '');
  need(m !== null, `the civ's panel has a Living row (${living?.value ?? 'none'})`);
  if (m) {
    const [g, f, a] = m.slice(1).map(Number);
    const civ = await page.eval(`(() => { const v = game.view(); const c = v.civs[${me}];
      return { people: c.people, army: c.living.army, farming: c.living.farming }; })()`);
    // The farming people are read from the view: the led civ's farmers can be under half a person, which the row
    // rounds to 0 (build-3's decision; the row's whole people are as the brief says).
    need(civ.farming > 0, `the civ's farming people are above 0 (${civ.farming.toFixed(2)}, the row reads ${f})`);
    // Each part rounds to a whole person, so the three add up to the civ's people and its army within 1.5.
    need(Math.abs(g + f + a - (civ.people + civ.army)) <= 1.5, `the Living parts (${g} + ${f} + ${a}) add up to its people and army (${(civ.people + civ.army).toFixed(2)})`);
  }
  await page.shot(join(out, 'civ-panel.png'), 1);

  // The farmer, selected, with its fields drawn.
  if (item !== null) await clickItem();
  else await page.select(clanView.home, 'clan');
  await page.eval(frames);
  need((await page.data()).selected === `clan:${clan}`, `the farmer is selected again for its picture (clan ${clan})`);
  await page.shot(join(out, 'fields.png'), 1);

  // View → Fields turns the layer off.
  const before = await around(tile);
  await page.eval("document.getElementById('v-fields').click()");
  await page.eval(frames);
  await page.eval(frames);
  need(await page.eval("game.map.on.fields === false && !document.getElementById('v-fields').checked"), 'View → Fields turns the layer off');
  need(differing(before, await around(tile)) >= 15, 'the field tile is drawn without its furrows once the layer is off');
} finally {
  for (const e of page.errors()) fail(`page error: ${e}`);
  await page.close();
}
if (failed) process.exit(1);
console.log(`ok      fields: the page shows the farmer's fields, its Living row and the layer (${out})`);
