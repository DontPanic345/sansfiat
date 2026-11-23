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
//
//   node www/shots.mjs --cull DIR                   each view with an `on` (mountain, shore and wood) twice in one page, the
//                                                   second with the edge, woods and ridge layers' culling off: DIR/NAME.png
//                                                   and DIR/NAME-nocull.png; exit 1 unless the two are byte-identical
//   node www/shots.mjs --time NAME,…                for each named view, five static redraws: each one's stats.terrainMs and
//                                                   the median; stats.woods where the branch has it (main prints -)
// A view with `turns` leads the first civ and ends that many turns before its camera is set; every view's record holds
// the game's digest, which --diff compares too.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { open, SIZES } from './play.mjs';

/**
 * The views: `size` is a map (`small`, `medium`, `large`), `seed` its seed, `window` `laptop` or `phone`, `k` the
 * zoom. `on` is the camera's tile: `plains` (the plains tile nearest world (0, 0), the default) or a mountain
 * class (`fork`, `bend`, `4+`), the tile of that class nearest world (0, 0). `tile` is the tile it must be.
 * `edge` moves the camera so the tile's centre sits `by` world units past that canvas edge.
 * The shore classes (`neck`, `strait`, `bay`, `pond`, `mouth-sea`, `mouth-lake`, `marsh`) are shores.md's, as the
 * chooser counts them: water is terrain 0, and off the map counts as water.
 */
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
  { name: 'small9-fork-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'fork', tile: 469 },
  { name: 'small9-fork-play', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'fork', tile: 469 },
  { name: 'small2-bend-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'bend', tile: 377 },
  { name: 'small2-bend-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'bend', tile: 377 },
  { name: 'medium9-blob-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: '4+', tile: 777 },
  { name: 'large8-bend-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'bend', tile: 1937 },
  { name: 'large8-fork-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'fork', tile: 1953 },
  { name: 'small9-edge-bottom', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'bend', tile: 474, edge: { side: 'bottom', by: 1.0 } },
  { name: 'small9-edge-right', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'bend', tile: 474, edge: { side: 'right', by: 0.8 } },
  { name: 'small9-fork-edge', size: 'small', seed: 9, window: 'phone', k: 6, on: 'fork', tile: 469 },
  { name: 'small9-fork-flat', size: 'small', seed: 9, window: 'phone', k: 5, on: 'fork', tile: 469 },
  { name: 'small2-neck-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'neck', tile: 402 },
  { name: 'small2-delta-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'mouth-sea', tile: 506 },
  { name: 'small2-delta-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'mouth-sea', tile: 506 },
  { name: 'small6-bay-close', size: 'small', seed: 6, window: 'laptop', k: 35, on: 'bay', tile: 376 },
  { name: 'small9-pond-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'pond', tile: 341 },
  { name: 'small9-lake-play', size: 'small', seed: 9, window: 'laptop', k: 18, on: 'mouth-lake', tile: 377 },
  { name: 'medium8-strait-close', size: 'medium', seed: 8, window: 'laptop', k: 35, on: 'strait', tile: 875 },
  { name: 'medium9-neck-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: 'neck', tile: 1004 },
  { name: 'large8-delta-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'marsh', tile: 2639 },
  { name: 'large8-pond-play', size: 'large', seed: 8, window: 'laptop', k: 18, on: 'pond', tile: 1289 },
  { name: 'small2-neck-edge', size: 'small', seed: 2, window: 'phone', k: 6, on: 'neck', tile: 402 },
  { name: 'small2-neck-flat', size: 'small', seed: 2, window: 'phone', k: 5, on: 'neck', tile: 402 },
  { name: 'medium9-woods-close', size: 'medium', seed: 9, window: 'laptop', k: 35, on: 'interior', tile: 827 },
  { name: 'medium9-woods-play', size: 'medium', seed: 9, window: 'laptop', k: 18, on: 'interior', tile: 827 },
  { name: 'medium9-woods-near', size: 'medium', seed: 9, window: 'laptop', k: 12, on: 'interior', tile: 827 },
  { name: 'medium9-woods-far', size: 'medium', seed: 9, window: 'laptop', k: 11, on: 'interior', tile: 827 },
  { name: 'medium9-woods-edge', size: 'medium', seed: 9, window: 'phone', k: 6, on: 'interior', tile: 827 },
  { name: 'medium9-woods-flat', size: 'medium', seed: 9, window: 'phone', k: 5, on: 'interior', tile: 827 },
  { name: 'small2-lone-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'lone', tile: 439 },
  { name: 'small2-lone-play', size: 'small', seed: 2, window: 'laptop', k: 18, on: 'lone', tile: 439 },
  { name: 'small9-river-close', size: 'small', seed: 9, window: 'laptop', k: 35, on: 'forest-river', tile: 258 },
  { name: 'small2-delta-near', size: 'small', seed: 2, window: 'laptop', k: 12, on: 'marsh', tile: 506 },
  { name: 'small2-delta-far', size: 'small', seed: 2, window: 'laptop', k: 11, on: 'marsh', tile: 506 },
  { name: 'small2-delta-edge', size: 'small', seed: 2, window: 'phone', k: 6, on: 'marsh', tile: 506 },
  { name: 'small2-delta-flat', size: 'small', seed: 2, window: 'phone', k: 5, on: 'marsh', tile: 506 },
  { name: 'medium9-fields-close', size: 'medium', seed: 9, window: 'laptop', k: 35, on: 'forest-fields', tile: 828, turns: 8 },
  { name: 'small2-marsh-fields-close', size: 'small', seed: 2, window: 'laptop', k: 35, on: 'marsh-fields', tile: 535, turns: 8 },
];

/**
 * In the page: sets the camera for `view` (`{on, k, edge}`) and returns `{ tile, cx, cy, k }` as asked. The chooser
 * is proto/nearest.mjs's rule: the class's tile nearest world (0, 0), the lower index on a tie.
 */
const CAMERA = `(view) => {
  const m = game.map, M = m.byName.mountain, n = m.px.length, l = m.l;
  const classOf = (i) => {
    const ds = [];
    for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && m.l.terrain[j] === M) ds.push(d); }
    if (ds.length === 0) return 'iso';
    if (ds.length === 1) return 'end';
    if (ds.length === 2) { const g = Math.min((ds[1] - ds[0] + 6) % 6, (ds[0] - ds[1] + 6) % 6); return g === 3 ? 'straight' : g === 2 ? 'bend' : 'sharp'; }
    return ds.length === 3 ? 'fork' : '4+';
  };
  // Water is terrain 0 and off the map counts as water, as proto/shore.mjs counts a shore.
  const runsOf = (i) => {
    const w = [];
    for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; w.push(j < 0 || l.terrain[j] === 0); }
    let ch = 0;
    for (let d = 0; d < 6; d++) if (w[d] !== w[(d + 1) % 6]) ch++;
    return { water: w.filter((x) => x).length, runs: ch / 2 };
  };
  const lakeSize = (i) => {
    const seen = new Set([i]), st = [i];
    while (st.length && seen.size < 4) {
      const a = st.pop();
      for (let d = 0; d < 6; d++) { const b = m.nb[a * 6 + d]; if (b >= 0 && l.terrain[b] === 0 && l.lake[b] && !seen.has(b)) { seen.add(b); st.push(b); } }
    }
    return seen.size;
  };
  const mouthInto = (i, lake) => l.river[i] > 0 && l.flow[i] >= 0 && l.terrain[l.flow[i]] === 0 && !!l.lake[l.flow[i]] === lake;
  const SHORE = {
    neck: (i) => l.terrain[i] !== 0 && runsOf(i).runs >= 2,
    strait: (i) => l.terrain[i] === 0 && runsOf(i).runs >= 2,
    bay: (i) => l.terrain[i] === 0 && !l.lake[i] && runsOf(i).water <= 2 && runsOf(i).runs === 1,
    pond: (i) => l.terrain[i] === 0 && !!l.lake[i] && lakeSize(i) <= 3,
    'mouth-sea': (i) => mouthInto(i, false),
    'mouth-lake': (i) => mouthInto(i, true),
    marsh: (i) => l.terrain[i] === m.byName.marsh,
  };
  // The wood classes (map/woods.js's rule): a forest tile by its forest neighbours, a river through it, or its field plots.
  const FOREST = m.byName.forest, MARSH = m.byName.marsh;
  const plots = new Set(); for (const c of Object.values(game.view().clans)) for (const [i] of c.plots) plots.add(i);
  const forestNbs = (i) => { let f = 0; for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && l.terrain[j] === FOREST) f++; } return f; };
  const riverAt = (i) => l.river[i] > 0 && l.flow[i] >= 0;
  const intoRiver = (i) => { for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && riverAt(j) && l.flow[j] === i) return true; } return false; };
  const WOOD = {
    interior: (i) => l.terrain[i] === FOREST && forestNbs(i) === 6,
    lone: (i) => l.terrain[i] === FOREST && forestNbs(i) === 0,
    'forest-river': (i) => l.terrain[i] === FOREST && (riverAt(i) || intoRiver(i)),
    'forest-fields': (i) => l.terrain[i] === FOREST && plots.has(i),
    'marsh-fields': (i) => l.terrain[i] === MARSH && plots.has(i),
  };
  const pick = view.on === 'plains' ? (i) => l.terrain[i] === m.byName.plains : SHORE[view.on] ?? WOOD[view.on] ?? ((i) => l.terrain[i] === M && classOf(i) === view.on);
  let tile = -1, bd = Infinity;
  for (let i = 0; i < n; i++) {
    if (!pick(i)) continue;
    const d = m.px[i] * m.px[i] + m.py[i] * m.py[i];
    if (d < bd) { bd = d; tile = i; }
  }
  if (tile < 0) throw new Error(\`the map has no \${view.on} tile\`);
  let cx = m.px[tile], cy = m.py[tile];
  if (view.edge?.side === 'bottom') cy = m.py[tile] - m.h / 2 / view.k - view.edge.by;
  if (view.edge?.side === 'right') cx = m.px[tile] - m.w / 2 / view.k - view.edge.by;
  m.cx = cx; m.cy = cy; m.k = view.k; m.viewChanged();
  return { tile, cx, cy, k: view.k };
}`;

/** Leads the first civ and ends `n` turns: each waits for the turn number to move on (proto/fields.mjs's loop). */
async function playTurns(page, n) {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  for (let t = 0; t < n; t++) {
    const before = (await page.data()).turn;
    await page.key('Space');
    await page.until(`document.body.dataset.turn !== '${before}'`, 60000);
  }
}

/** Opens `view`'s page, plays its turns, sets its camera and waits for it to settle: the page, the camera's tile, and
 *  the game's digest. */
async function setUp(view) {
  const url = `?size=${view.size}&seed=${view.seed}&run=1`;
  const page = await open(url, { size: view.window });
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  if (view.turns) await playTurns(page, view.turns);
  const asked = await page.eval(`(${CAMERA})(${JSON.stringify({ on: view.on ?? 'plains', k: view.k, edge: view.edge })})`);
  await page.until('!game.map.anim && !game.map.frame', 5000);
  const cam = await page.eval('(() => { const m = game.map; return { cx: m.cx, cy: m.cy, k: m.k }; })()');
  // The pan clamp moves a camera it doesn't like: the camera must be the one asked for, or the view is not the view named.
  if (cam.k !== asked.k || cam.cx !== asked.cx || cam.cy !== asked.cy) {
    throw new Error(`${view.name}: asked for camera ${JSON.stringify(asked)}, the map gave ${JSON.stringify(cam)} (minK ${await page.eval('game.map.minK')})`);
  }
  if (view.tile !== undefined && asked.tile !== view.tile) throw new Error(`${view.name}: the tile is ${asked.tile}, the view names ${view.tile}`);
  const digest = await page.eval('game.digest()');
  return { page, url, asked, cam, digest };
}

/** Shoots one view into `out`: its PNG, and its record for views.json. */
async function shoot(view, out) {
  const { page, url, asked, cam, digest } = await setUp(view);
  try {
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const stats = await page.eval('({ ...game.map.stats })');
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    return { url, window: view.window, tile: asked.tile, cx: cam.cx, cy: cam.cy, k: cam.k, digest, stats };
  } finally {
    await page.close();
  }
}

/** Shoots a view with the edge, woods and ridge layers culling and again without, in one page; true if the PNGs match. */
async function cull(view, out) {
  const { page, asked } = await setUp(view);
  try {
    const grab = "(() => { game.map.drawNow(); const s = game.map.stats; return { png: game.map.canvas.toDataURL('image/png'), ridges: s.ridges, edges: s.edges, woods: s.woods ?? '-' }; })()";
    const culled = await page.eval(grab);
    await page.eval('(() => { game.map.noCull = true; game.map.dirty = true; })()');
    const uncut = await page.eval(grab);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(culled.png.split(',')[1], 'base64'));
    writeFileSync(join(out, `${view.name}-nocull.png`), Buffer.from(uncut.png.split(',')[1], 'base64'));
    const same = culled.png === uncut.png;
    console.log(`${view.name}  tile ${asked.tile}  stats.ridges ${culled.ridges} culled, ${uncut.ridges} unculled  stats.edges ${culled.edges} culled, ${uncut.edges} unculled  stats.woods ${culled.woods} culled, ${uncut.woods} unculled  ${same ? 'identical' : 'DIFFER'}`);
    return same;
  } finally {
    await page.close();
  }
}

/** Five static redraws of a view, each one's terrain time, and the median; prints them with stats.woods. */
async function time(view) {
  const { page, asked } = await setUp(view);
  try {
    const r = await page.eval('(() => { const m = game.map, ms = []; for (let i = 0; i < 5; i++) { m.dirty = true; m.drawNow(); ms.push(m.stats.terrainMs); } return { ms, woods: m.stats.woods ?? null }; })()');
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    const median = [...r.ms].sort((a, b) => a - b)[2];
    console.log(`${view.name}  tile ${asked.tile}  terrainMs ${r.ms.map((x) => x.toFixed(1)).join(' ')}  median ${median.toFixed(1)}  stats.woods ${r.woods ?? '-'}`);
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
    if (va[name].digest !== undefined && vb[name].digest !== undefined && va[name].digest !== vb[name].digest) {
      console.error(`${name}  digests differ: ${va[name].digest} vs ${vb[name].digest}`);
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
if (args.includes('--cull')) {
  const at = args.indexOf('--cull');
  const out = resolve(args[at + 1]);
  mkdirSync(out, { recursive: true });
  let failed = false;
  for (const view of VIEWS) {
    if (!view.on) continue;
    if (!(await cull(view, out))) failed = true;
  }
  if (failed) {
    console.error('shots --cull: a mountain view draws differently with culling off');
    process.exit(1);
  }
} else if (args.includes('--diff')) {
  const at = args.indexOf('--diff');
  await diff(resolve(args[at + 1]), resolve(args[at + 2]));
} else if (args.includes('--time')) {
  const names = args[args.indexOf('--time') + 1]?.split(',') ?? [];
  for (const name of names) {
    const view = VIEWS.find((v) => v.name === name);
    if (!view) throw new Error(`no view ${name}`);
    await time(view);
  }
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
