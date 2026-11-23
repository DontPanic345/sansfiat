// Fixed map views, shot through play.mjs: each opens the page at its map and window, sets the camera on the
// plains tile nearest world (0, 0) and shots the map canvas itself (the pick's panel is not in it).
//
//   node www/shots.mjs --out DIR [--only NAME,…]    DIR/NAME.png per view, and DIR/views.json (each view's
//                                                   url, window, tile, cx, cy, k and game.map.stats)
//   node www/shots.mjs --diff A B                   two such folders: the cameras must match (exit 1 if not);
//                                                   prints each view's share of pixels that differ
//
// It is not in www/play/, so play/all.mjs (which check.sh runs) doesn't run it. Its --out imports only
// play.mjs, so this file copies alone onto a worktree of main and shoots there.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { open, SIZES } from './play.mjs';

/** The views: `size` is a map (`small`, `medium`, `large`), `seed` its seed, `window` `laptop` or `phone`, `k` the zoom. */
export const VIEWS = [
  { name: 'small2-close', size: 'small', seed: 2, window: 'laptop', k: 35 },
  { name: 'small2-play', size: 'small', seed: 2, window: 'laptop', k: 18 },
  { name: 'small2-edge', size: 'small', seed: 2, window: 'phone', k: 6 },
  { name: 'small2-flat', size: 'small', seed: 2, window: 'phone', k: 5 },
  { name: 'small3-play', size: 'small', seed: 3, window: 'laptop', k: 18 },
  { name: 'small6-play', size: 'small', seed: 6, window: 'laptop', k: 18 },
  { name: 'small9-play', size: 'small', seed: 9, window: 'laptop', k: 18 },
  { name: 'medium8-play', size: 'medium', seed: 8, window: 'laptop', k: 18 },
  { name: 'medium9-play', size: 'medium', seed: 9, window: 'laptop', k: 18 },
  { name: 'large8-play', size: 'large', seed: 8, window: 'laptop', k: 18 },
];

/** Shoots one view into `out`: its PNG, and its record for views.json. */
async function shoot(view, out) {
  const url = `?size=${view.size}&seed=${view.seed}&run=1`;
  const page = await open(url, { size: view.window });
  try {
    await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
    // The plains tile nearest world (0, 0); the lowest index on a tie.
    const tile = await page.eval(`(() => {
      const m = game.map;
      let best = -1, bd = Infinity;
      for (let i = 0; i < m.px.length; i++) {
        if (m.l.terrain[i] !== m.byName.plains) continue;
        const d = m.px[i] * m.px[i] + m.py[i] * m.py[i];
        if (d < bd) { bd = d; best = i; }
      }
      return best;
    })()`);
    if (tile < 0) throw new Error(`${view.name}: the map has no plains`);
    await page.eval(`(() => { const m = game.map; m.cx = m.px[${tile}]; m.cy = m.py[${tile}]; m.k = ${view.k}; m.viewChanged(); })()`);
    await page.until('!game.map.anim && !game.map.frame', 5000);
    const cam = await page.eval('(() => { const m = game.map; return { cx: m.cx, cy: m.cy, k: m.k }; })()');
    if (cam.k !== view.k) throw new Error(`${view.name}: asked for k ${view.k}, the map gave ${cam.k} (minK ${await page.eval('game.map.minK')})`);
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const stats = await page.eval('({ ...game.map.stats })');
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    return { url, window: view.window, tile, cx: cam.cx, cy: cam.cy, k: cam.k, stats };
  } finally {
    await page.close();
  }
}

/** Compares two output folders: cameras must match; prints each view's share of differing pixels. */
async function diff(a, b) {
  const { decode } = await import('./assets/png.mjs');
  const va = JSON.parse(readFileSync(join(a, 'views.json'), 'utf8'));
  const vb = JSON.parse(readFileSync(join(b, 'views.json'), 'utf8'));
  let failed = false;
  for (const name of Object.keys(va)) {
    if (!(name in vb)) {
      console.log(`${name}  only in ${a}`);
      continue;
    }
    const cam = (v) => ({ tile: v.tile, cx: v.cx, cy: v.cy, k: v.k });
    if (JSON.stringify(cam(va[name])) !== JSON.stringify(cam(vb[name]))) {
      console.error(`${name}  cameras differ: ${JSON.stringify(cam(va[name]))} vs ${JSON.stringify(cam(vb[name]))}`);
      failed = true;
      continue;
    }
    const x = decode(readFileSync(join(a, `${name}.png`))), y = decode(readFileSync(join(b, `${name}.png`)));
    const sizes = `${x.width}×${x.height} vs ${y.width}×${y.height}`;
    if (x.width !== y.width || x.height !== y.height) {
      console.log(`${name}  sizes differ (${sizes})`);
      continue;
    }
    let differ = 0;
    for (let i = 0; i < x.width * x.height; i++) {
      const o = i * 4;
      if (x.data[o] !== y.data[o] || x.data[o + 1] !== y.data[o + 1] || x.data[o + 2] !== y.data[o + 2] || x.data[o + 3] !== y.data[o + 3]) differ++;
    }
    const share = +((100 * differ) / (x.width * x.height)).toFixed(3);
    console.log(`${name}  ${share}% differ  (${differ} of ${x.width * x.height} pixels; ${sizes})`);
  }
  for (const name of Object.keys(vb)) if (!(name in va)) console.log(`${name}  only in ${b}`);
  if (failed) process.exit(1);
}

const args = process.argv.slice(2);
if (args.includes('--diff')) {
  const at = args.indexOf('--diff');
  await diff(resolve(args[at + 1]), resolve(args[at + 2]));
} else {
  const at = args.indexOf('--out');
  if (at < 0 || !args[at + 1]) {
    console.error('usage: node www/shots.mjs --out DIR [--only NAME,…]  |  node www/shots.mjs --diff A B');
    process.exit(2);
  }
  const out = resolve(args[at + 1]);
  const only = args.indexOf('--only') >= 0 ? args[args.indexOf('--only') + 1].split(',') : null;
  mkdirSync(out, { recursive: true });
  const views = {};
  for (const view of VIEWS) {
    if (only && !only.includes(view.name)) continue;
    views[view.name] = await shoot(view, out);
    console.log(`${view.name}  k ${views[view.name].k}  tile ${views[view.name].tile}  ${SIZES[view.window].width}×${SIZES[view.window].height}`);
  }
  writeFileSync(join(out, 'views.json'), `${JSON.stringify(views, null, 1)}\n`);
}
