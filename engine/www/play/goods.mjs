// Goods on the page (#355): the map draws a mark on each source tile, a white crystal for salt and a black shard for
// obsidian, and the tile's tooltip and panel name the source; the civ's panel lists salt and obsidian beside its food.
// A staged game on a plains map: a salt source and an obsidian source beside the led civ's capital, and its store set
// to 10 grain.
//   node www/play/goods.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-goods-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`goods: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// The staged game: radius 4, the led civ's capital at A, the rival's at B, salt at S and obsidian at O.
const R = 4;
const SPEC = {
  map: plains(R),
  marks: marks(R, { A: [-2, 0], B: [2, 0], S: [0, -1], O: [0, 1] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  sources: [{ at: { mark: 'S' }, good: 'salt' }, { at: { mark: 'O' }, good: 'obsidian' }],
  stores: [{ civ: 0, food: 10 }],
};

const page = await open('');
try {
  await stage(page, SPEC);
  const civ = await page.eval('game.view().player');

  // The sources: salt and obsidian both on the map.
  const sources = await page.eval('game.map.l.source.map((g, i) => [g, i]).filter(([g]) => g)');
  const of = (good) => sources.filter(([g]) => g === good).map(([, i]) => i);
  need(of('salt').length > 0 && of('obsidian').length > 0, `the map has ${of('salt').length} salt and ${of('obsidian').length} obsidian sources`);

  // The pixels round a tile, the layers over it off, so only the terrain and the sources show.
  for (const l of ['units', 'capitals', 'centres', 'ring', 'routes', 'land', 'held', 'clans', 'towns', 'settle', 'hearths', 'places']) await page.eval(`game.map.setLayer('${l}', false)`);
  const around = (i) =>
    page.eval(`(() => {
      const c = document.getElementById('map');
      const r = c.getBoundingClientRect();
      const p = game.map.screenOf(${i});
      const s = c.width / r.width;
      const h = Math.round(30 * s);
      const x = Math.round((p.x - r.left) * s), y = Math.round((p.y - r.top) * s);
      return Array.from(c.getContext('2d').getImageData(x - h, y - h, 2 * h, 2 * h).data);
    })()`);
  const near = (px, [r, g, b], tol) => {
    let n = 0;
    for (let k = 0; k < px.length; k += 4) if (Math.abs(px[k] - r) < tol && Math.abs(px[k + 1] - g) < tol && Math.abs(px[k + 2] - b) < tol) n++;
    return n;
  };
  for (const [good, colour] of [['salt', [244, 247, 250]], ['obsidian', [10, 10, 12]]]) {
    const i = of(good)[0];
    await page.eval(`game.map.centreOn(${i}, { minK: 25, smooth: false })`);
    await page.eval(frames);
    const on = await around(i);
    await page.eval("game.map.setLayer('sources', false)");
    await page.eval(frames);
    const off = await around(i);
    await page.eval("game.map.setLayer('sources', true)");
    await page.eval(frames);
    const mine = near(on, colour, 12) - near(off, colour, 12);
    need(mine >= 6, `the ${good} source is drawn: ${mine} more ${good === 'salt' ? 'white' : 'black'} pixels with the layer on`);

    const tip = await page.tip(i);
    need(tip && tip.includes(`Source of ${good}`), `the tile's tooltip names the ${good} source (${tip ? tip.split('\n').pop() : 'no tip'})`);
    for (let n = 0; n < 12; n++) {
      if ((await page.eval("document.body.dataset.selected || ''")) === `tile:${i}`) break;
      await page.click(i);
      await page.eval(frames);
    }
    const panel = await page.panel();
    const row = panel?.numbers?.find((r) => r.label === 'Source');
    need(row && row.value === good, `the tile's panel says Source: ${good} (${row ? row.value : 'no row'})`);
  }

  // The civ's panel lists salt and obsidian.
  await page.eval(`document.querySelector('button.chip[data-civ="${civ}"]').click()`);
  await page.eval(frames);
  const p = await page.panel();
  const labels = p?.numbers?.map((n) => n.label) ?? [];
  need(labels.includes('Salt') && labels.includes('Obsidian'), `the civ's panel lists salt and obsidian (${labels.join(', ')})`);
  const store = p?.numbers?.find((n) => n.label === 'Store');
  need(store && /of \d+/.test(store.value), `its Store is the food against the cap (${store ? store.value : 'none'})`);

  await page.shot(join(out, 'goods.png'), 1);
  console.log(`      the screenshot is ${join(out, 'goods.png')}`);
} finally {
  await page.close();
}
if (failed) process.exit(1);
