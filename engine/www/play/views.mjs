// The map's views: pan, zoom and fit move it; each layer toggles by name and the map redraws; H toggles
// hidden country; a hover shows a tooltip naming the terrain. The staged map has a river, forest, hills, marsh and
// mountain among the plains, so each layer that draws terrain has something to draw.   node www/play/views.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { grid, marks, stage } from '../play-stage.mjs';

// Radius 12, so the fit is below the zoom's cap. Sea on the rim, the land cycled through plains, forest, hills,
// marsh and mountain, and plains where the marks stand: two capitals, a river (R flows to S), and a clan's home
// (P) going to the led civ's festival.
const R = 12;
const TERRAIN = ['.', 'f', '.', 'h', '%', '.', '^', 'f', '.'];
const MARKS = { A: [-3, 0], B: [3, 0], R: [0, -2], S: [1, -2], P: [-2, 1] };
const SPEC = {
  map: grid(R, (q, r) => {
    const d = Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
    if (d === R) return '~';
    if (Object.values(MARKS).some(([a, b]) => a === q && b === r)) return '.';
    return TERRAIN[(((q * 4 + r * 7) % TERRAIN.length) + TERRAIN.length) % TERRAIN.length];
  }),
  marks: marks(R, MARKS),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  rivers: [{ at: { mark: 'R' }, flow: { mark: 'S' } }],
  clans: [{ home: { mark: 'P' }, tiles: [], people: 4, market: 0 }],
};

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-views-')));
mkdirSync(out, { recursive: true });

const page = await open('');
let failed = false;
const fail = (msg) => {
  console.error(`views: ${msg}`);
  failed = true;
};
const settle = () => page.until('!game.map.anim && !game.map.frame', 5000);
const view = () => page.eval('(() => { const m = game.map; return { k: m.k, cx: m.cx, cy: m.cy }; })()');
try {
  await stage(page, SPEC);
  // A turn walks the clan to its festival: the routes are drawn from the walks.
  await page.button('End turn');
  await page.eval('game.map.fit()');
  const fit = await view();

  // Zoom in on the map's centre, pan by a drag of the wheel's kind, then fit again.
  await page.eval('game.map.zoomBy(2)');
  const zoomed = await view();
  if (!(zoomed.k > fit.k * 1.5)) fail(`zoom did not zoom: ${fit.k} -> ${zoomed.k}`);
  await page.key('ArrowLeft');
  const panned = await view();
  if (panned.cx === zoomed.cx) fail('the arrow key did not pan');
  await page.key('0');
  const fitted = await view();
  if (Math.abs(fitted.k - fit.k) > 1e-6 || fitted.cx !== 0 || fitted.cy !== 0) fail(`fit did not return the view: ${JSON.stringify(fitted)}`);
  await page.eval('game.map.zoomBy(2)');
  await page.key('0');

  // Each layer, by name: toggling it changes what the map draws.
  const names = await page.eval('game.map.constructor.name && Object.keys(game.map.on)');
  for (const must of ['terrain', 'rivers', 'territories', 'festival-routes', 'land', 'units', 'country', 'fertility', 'hidden-costs']) {
    if (!names.includes(must)) fail(`no layer ${must}`);
  }
  await page.eval('game.map.zoomBy(3)');
  await settle();
  const sample = () => page.eval(`(() => { const m = game.map; m.drawNow(); const g = m.canvas.getContext('2d'); const d = g.getImageData(0, 0, m.canvas.width, m.canvas.height).data; let h = 0; for (let i = 0; i < d.length; i += 4) h = (Math.imul(h, 31) + d[i] + d[i + 1] * 3 + d[i + 2] * 7) | 0; return h; })()`);
  for (const name of ['territories', 'fertility', 'country', 'hidden-costs', 'festival-routes', 'land', 'units', 'rivers']) {
    const before = await sample();
    await page.eval(`game.map.setLayer(${JSON.stringify(name)}, !game.map.on[${JSON.stringify(name)}])`);
    const toggled = await sample();
    await page.eval(`game.map.setLayer(${JSON.stringify(name)}, !game.map.on[${JSON.stringify(name)}])`);
    const back = await sample();
    // Hidden costs and country draw only on some tiles: allow the view where they show to be empty.
    if (toggled === before && !['country', 'hidden-costs'].includes(name)) fail(`toggling ${name} did not redraw the map`);
    if (back !== before) fail(`toggling ${name} twice did not restore the map`);
  }

  // H toggles hidden country.
  const country = () => page.eval('game.map.on.country');
  const was = await country();
  await page.key('h');
  if ((await country()) === was) fail('H did not toggle hidden country');
  await page.key('h');
  if ((await country()) !== was) fail('H twice did not restore hidden country');

  // The View and Debug menus: each item starts as its layer does, and toggling it toggles the layer.
  const items = [
    ['v-territory', 'territories', false],
    ['v-routes', 'festival-routes', false],
    ['v-country', 'country', false],
    ['v-fertility', 'fertility', false],
    ['v-hidden', 'hidden-costs', false],
    ['v-wear', 'worn', false],
  ];
  for (const [id, layer, on] of items) {
    const box = await page.eval(`document.getElementById('${id}').checked`);
    if (box !== on || (await page.eval(`game.map.on['${layer}']`)) !== on) fail(`${id} starts ${box}, not ${on}`);
    await page.eval(`document.getElementById('${id}').click()`);
    if ((await page.eval(`game.map.on['${layer}']`)) === on) fail(`${id} did not toggle ${layer}`);
    await page.eval(`document.getElementById('${id}').click()`);
    if ((await page.eval(`game.map.on['${layer}']`)) !== on) fail(`${id} twice did not restore ${layer}`);
  }
  // Legend: the View menu's item opens and closes the legend.
  const legend = () => page.eval("document.getElementById('legend').open");
  const wasOpen = await legend();
  await page.eval("document.getElementById('v-legend').click()");
  await page.until(`document.getElementById('legend').open === ${!wasOpen}`, 2000).catch(() => fail('the Legend item did not open or close the legend'));
  await page.eval("document.getElementById('v-legend').click()");
  await page.until(`document.getElementById('legend').open === ${wasOpen}`, 2000).catch(() => fail('the Legend item twice did not restore the legend'));

  // A hover names the terrain.
  await page.eval('game.map.fit()');
  const tile = await page.eval(`(() => { const l = game.layout(), m = game.map; return l.terrain.findIndex((t, i) => m.land[i] && l.q[i] > -3 && l.q[i] < 3 && l.r[i] > -3 && l.r[i] < 3); })()`);
  const text = await page.tip(tile);
  const name = await page.eval(`game.map.names[game.layout().terrain[${tile}]]`);
  if (!text || !text.toLowerCase().includes(name === 'marsh' ? 'marsh' : name)) fail(`the tooltip over tile ${tile} (${name}) reads ${JSON.stringify(text)}`);
  await page.shot(join(out, 'views-fit.png'));
  for (const e of page.errors()) fail(`page error: ${e}`);
  console.log(JSON.stringify({ fit, zoomed, tip: text }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
