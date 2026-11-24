// Towns, on the page (#156, build-3): a staged game on a plains map, its capital at the centre with the led civ's
// clan there, and two more clans living on the capital's tile, so that tile is a festival with three homes. Then
// the page shows the cluster of three hearths round the tile, the town mark above it (View → Towns turns it off),
// the tile's tooltip says "Town: 3 clans at the festival", the tile panel lists the three clans, and the first
// turn's feed has the town line. Writes towns.png (the cluster and the mark) to --out.
//   node www/play/towns.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-towns-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`towns: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// ---- the staged map: radius 3, sea on the rim, plains inside ------------------------------------------------------
const R = 3;
const MARK = { A: [0, 0], X: [1, -1], S: [-1, 1] };
const SPEC = {
  map: plains(R),
  marks: marks(R, MARK),
  civs: [{ mark: 'A' }],
  people: 12,
  lead: 0,
  clans: [
    { home: { mark: 'A' }, tiles: [{ mark: 'X' }], people: 4 },
    { home: { mark: 'A' }, tiles: [{ mark: 'S' }], people: 3 },
  ],
  // Food on each of the three tiles, so the clans are fed through the first turn and none merges.
  food: [
    { at: { mark: 'A' }, good: 1, amount: 30 },
    { at: { mark: 'X' }, good: 1, amount: 30 },
    { at: { mark: 'S' }, good: 1, amount: 30 },
  ],
};

const page = await open('');
try {
  await stage(page, SPEC);

  const view = await page.eval('game.view()');
  const me = view.player;
  const tile = view.civs[me].capital;
  const town = view.towns.find((t) => t.tile === tile);
  need(view.towns.length === 1 && town, 'the staged view has one town, on the capital\'s tile');
  need(town && town.clans.length === 3, `the town has three clans (${town ? town.clans.length : 0})`);
  need(town && typeof town.market === 'number', 'the town names its festival');
  need(view.civs[me].capital === tile && view.clans[town.clans[0]].home === tile, 'every town clan lives on the tile');
  need(await page.eval("game.map.on.towns === true"), 'the towns layer is on by default');

  // The tooltip reads as a town.
  const tip = await page.tip(tile);
  need(tip && tip.includes('Town: 3 clans at the festival'), `the tile's tooltip says it is a town (${tip ? tip.split('\n').pop() : 'no tip'})`);

  // The tile panel lists the three clans that live on it.
  // A click on the tile cycles what it holds (units, the civ, its clans, the festival) and then the bare tile.
  for (let n = 0; n < 12; n++) {
    if (String(await page.eval('document.body.dataset.selected || \'\'')).startsWith('tile:')) break;
    await page.click(tile);
    await page.eval(frames);
  }
  const panel = await page.panel();
  const living = panel?.lists.find((l) => l.label === 'Clans living here');
  need(living && living.rows.length === 3, `the tile panel lists the three clans living here (${living ? living.rows.join(', ') : 'none'})`);

  // The page draws the cluster: each clan's colour is on the map beside the tile. The capital, its units, the
  // centre's ring and the civ's ring cover the tile, so they are turned off for the cluster and the mark.
  for (const l of ['units', 'capitals', 'centres', 'ring', 'routes', 'land', 'held']) await page.eval(`game.map.setLayer('${l}', false)`);
  await page.eval(frames);
  const around = (i, half) =>
    page.eval(`(() => {
      const c = document.getElementById('map');
      const r = c.getBoundingClientRect();
      const p = game.map.screenOf(${i});
      const s = c.width / r.width;
      const h = Math.round(${half} * s);
      const x = Math.round((p.x - r.left) * s), y = Math.round((p.y - r.top) * s);
      return { w: 2 * h, data: Array.from(c.getContext('2d').getImageData(x - h, y - h, 2 * h, 2 * h).data) };
    })()`);
  // Each town clan is drawn on the tile in its civ's colour: the clans layer's probe measures each clan alone, its cloth
  // nearest its own colour (the pale for a clan of no civ). A multiplied cloth is shaded, so a count of pixels near the
  // flat colour falls short; the probe measures the same claim.
  const probes = await page.eval('game.map.clanProbe()');
  for (const id of town.clans) {
    const p = probes.find((x) => x.clan === id);
    need(p && p.tinted >= 8 && String(p.nearest) === String(p.civ ?? 'none'),
      `clan ${id} is drawn on the tile in its civ's colour (${p ? p.tinted : 0} tinted pixels, nearest ${p ? p.nearest : 'none'})`);
  }

  // The town mark: on with the layer, gone with it off. The capital's house is on, as on any capital: the mark must
  // show above it, not under it.
  await page.eval("game.map.setLayer('capitals', true)");
  await page.eval(frames);
  const mark = await around(tile, 24);
  await page.eval("game.map.setLayer('towns', false)");
  await page.eval(frames);
  const bare = await around(tile, 24);
  let differing = 0;
  for (let k = 0; k < mark.data.length; k += 4) {
    if (Math.max(...[0, 1, 2].map((c) => Math.abs(mark.data[k + c] - bare.data[k + c]))) > 8) differing++;
  }
  await page.eval("game.map.setLayer('towns', true)");
  await page.eval(frames);
  need(differing >= 100, `the town mark shows above the capital's house (${differing} pixels differ with the layer off)`);

  // A turn: the feed has the town line.
  await page.button('End turn');
  await page.eval(frames);
  const feed = await page.eval("game.feed.items.filter((i) => i.kind === 'townFormed').map((i) => i.text)");
  need(feed.length === 1 && feed[0].startsWith('A town forms at'), `the feed has the town line (${feed.join('; ') || 'none'})`);

  await page.shot(join(out, 'towns.png'), 1);
  console.log(`      the screenshot is ${join(out, 'towns.png')}`);
} finally {
  await page.close();
}
if (failed) process.exit(1);
