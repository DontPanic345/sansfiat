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
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const SPAN = (r) => ({ lo: Math.max(-R, -r - R), hi: Math.min(R, -r + R) });
const gridText = (cell) => {
  let text = '';
  for (let r = -R; r <= R; r++) {
    const { lo, hi } = SPAN(r);
    text += ' '.repeat(2 * lo + r + 2 * R);
    const row = [];
    for (let q = lo; q <= hi; q++) row.push(cell(q, r));
    text += `${row.join(' ')}\n`;
  }
  return text;
};
const MAP = gridText((q, r) => (dist(q, r) === R ? '~' : '.'));
const MARK = { A: [0, 0], X: [1, -1], S: [-1, 1] };
const MARKS = gridText((q, r) => Object.keys(MARK).find((k) => MARK[k][0] === q && MARK[k][1] === r) ?? '.');
const SPEC = {
  map: MAP,
  marks: MARKS,
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

// ---- colours: a clan's hue (`map/geom.js`'s hsl) as RGB, to find its pixels --------------------------------------
const hslToRgb = (h, s, l) => {
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  const [r, g, b] = h < 60 ? [c, x, 0] : h < 120 ? [x, c, 0] : h < 180 ? [0, c, x] : h < 240 ? [0, x, c] : h < 300 ? [x, 0, c] : [c, 0, x];
  return [r + m, g + m, b + m].map((v) => Math.round(v * 255));
};
const rgbOf = (id) => hslToRgb((id * 137.508) % 360, 0.7, 0.55);

const page = await open('');
try {
  await page.eval(`game.stage(${JSON.stringify(SPEC)})`);
  await page.until("document.body.dataset.phase === 'main'");
  await page.eval(frames);

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
  const count = (px, [r, g, b]) => {
    let n = 0;
    for (let k = 0; k < px.data.length; k += 4) {
      if (Math.abs(px.data[k] - r) < 24 && Math.abs(px.data[k + 1] - g) < 24 && Math.abs(px.data[k + 2] - b) < 24) n++;
    }
    return n;
  };
  const px = await around(tile, 24);
  for (const id of town.clans) {
    const n = count(px, rgbOf(id));
    need(n >= 8, `clan ${id} is drawn on the tile (${n} pixels in its colour)`);
  }

  // The town mark: on with the layer, gone with it off.
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
  need(differing > 0, `the town mark is drawn above the tile (${differing} pixels differ with the layer off)`);

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
