// The map's probes (#446): each one opens the views shots-setup.mjs sets up, in headless Chrome, and measures what the spec's
// bars are judged on. Run from engine/ on a tree with www/pkg built; each prints its table, and the modes that judge
// exit 1 on a failure.
//
//   node www/shots-probe.mjs --hits [NAME,…]       per view (default: the verify set and each view with units, clans or places):
//                                                  each drawn unit, clan and place's box centre by tileAt against its own tile;
//                                                  exit 1 on any miss
//   node www/shots-probe.mjs --pop [DIR]           each POPS case, its tile put 90 and 130 css px past each edge, drawn with
//                                                  culling on and off: the pixels that differ; exit 1 on any; DIR gets each
//                                                  differing placement's two PNGs
//   node www/shots-probe.mjs --overlays DIR [NAME,…]  each View layer that starts off (view.js's OFF set, read from game.map.on),
//                                                  turned on alone over each named view (default medium9-play, medium9-places-close,
//                                                  small2-edge): DIR/NAME-LAYER.png and the share of pixels it moves by more than 48
//   node www/shots-probe.mjs --layers [NAME,…]     per static layer, its median draw time over five redraws (default: the verify set)
//   node www/shots-probe.mjs --pan [DIR] [NAME,…]  per view (default: the verify set), the frame a player makes most: twelve panBy steps
//                                                  of (24, 12) css px ending on the view's camera, each drawNow timed; the median,
//                                                  the slowest, the last frame's stats. DIR gets DIR/NAME.png (the last frame) and
//                                                  DIR/pan.json (each view's last camera and drag)
//   node www/shots-probe.mjs --shift [NAME,…]      main's own shift noise: the static layer at the start of --pan's drag, shifted by
//                                                  the drag's device px, against the static layer at its end, over the overlap
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { setUp } from './shots-setup.mjs';
import { VIEWS, VERIFY, POPS } from './shots-views.mjs';

/** The default views of --hits: the verify set, and each view with units, clans or places. */
const HITS = [...new Set([...VERIFY, ...VIEWS.filter((v) => v.units || v.clans || v.places).map((v) => v.name)])];

/** The views named on the command line (a comma list), or `dflt`. */
function named(list, dflt) {
  const names = list ? list.split(',') : dflt;
  return names.map((name) => VIEWS.find((v) => v.name === name) ?? (() => { throw new Error(`no view ${name}`); })());
}

/** The arguments after `mode`: an optional DIR, then an optional NAME list. A first argument that names views is the list. */
function args(mode) {
  const at = process.argv.indexOf(mode);
  const rest = process.argv.slice(at + 1).filter((a) => !a.startsWith('--'));
  const isNames = (a) => a !== undefined && a.split(',').every((n) => VIEWS.some((v) => v.name === n));
  if (rest.length && isNames(rest[0])) return { dir: null, list: rest[0] };
  return { dir: rest[0] ? resolve(rest[0]) : null, list: rest[1] };
}

/** The drag: twelve steps, as a player's pan, of (24, 12) css px unless a smaller step is needed (see PLAN). */
const STEPS = 12;
const STEP = [24, 12];

/** The in-page drag: a twelve-step pan that ends on the view's camera exactly. Its steps are whole css px, (x, x/2) for even x (24 first),
 *  forwards or reversed, whichever first keeps the drag on the map (the clamp would move a step) and whose
 *  start, searched among the floats about it (64 ulps either side), lands the twelve steps on the end camera exactly. */
const PLAN = `(() => {
  const m = game.map, R = m.radius, N = ${STEPS};
  const mx = (Math.sqrt(3) * (2 * R + 1) + 0.6) / 2, my = (3 * R + 2 + 0.6) / 2;
  const ex = m.cx, ey = m.cy, k = m.k;
  const ulp = (v) => 2 ** (Math.floor(Math.log2(Math.max(Math.abs(v), 1e-300))) - 52);
  const find = (e, s0, step) => {
    const u = ulp(Math.max(Math.abs(s0), Math.abs(e)));
    for (let i = 0; i <= 64; i++) for (const sg of [1, -1]) {
      const v = s0 + sg * i * u;
      let x = v; for (let j = 0; j < N; j++) x -= step / k;
      if (x === e) return v;
    }
    return null;
  };
  let first = null;
  for (const a of [${STEP[0]}, ...[22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2]]) for (const s of [1, -1]) {
    const dx = s * a, dy = s * a / 2;
    const sx = ex + N * dx / k, sy = ey + N * dy / k;
    if (Math.abs(sx) > mx || Math.abs(sy) > my) continue;
    first ??= { dx, dy, sx, sy };
    const fx = find(ex, sx, dx), fy = find(ey, sy, dy);
    if (fx !== null && fy !== null) return { dx, dy, sx: fx, sy: fy, ex, ey, N, exact: true };
  }
  if (!first) throw new Error('no drag fits the map');
  return { ...first, ex, ey, N, exact: false };
})()`;

/** One view's hits: each drawn unit, clan and place against its tile, through tileAt. */
async function hits(view) {
  const { page } = await setUp(view);
  try {
    const r = await page.eval(`(() => {
      const m = game.map, v = game.view(), d = m.dpr, s = m.stats, out = [];
      const at = (e) => m.tileAt((e.x + e.w / 2) / d, (e.y + e.h / 2) / d);
      for (const e of s.units ?? []) { const u = v.units[e.unit]; if (u) out.push({ what: 'unit ' + e.unit + ' ' + e.kind, own: u.tile, at: at(e) }); }
      for (const e of s.clans ?? []) if (e.w) { const c = v.clans[e.clan] ?? v.clans[e.id]; out.push({ what: 'clan ' + (e.clan ?? e.id), own: c ? c.home : e.tile, at: at(e) }); }
      for (const e of s.places ?? []) if (e.w && e.role !== 'mark') out.push({ what: 'place ' + e.role, own: e.tile, at: at(e) });
      return out;
    })()`);
    const miss = r.filter((e) => e.own !== e.at);
    console.log(`${view.name}  k ${view.k}  drawn ${r.length}  off their tile ${miss.length}  ${miss.map((e) => `${e.what} ${e.own}->${e.at}`).join(', ')}`);
    return miss.length;
  } finally {
    await page.close();
  }
}

/** Each POPS case: its tile at each edge, 90 and 130 css px out, drawn culled and uncut; the pixels that differ. */
async function pops(dir) {
  let failed = 0;
  for (const { view: name, tile } of POPS) {
    const view = VIEWS.find((v) => v.name === name);
    const { page } = await setUp(view);
    try {
      // Each placement: the tile 90 or 130 css px past one edge (a negative offset is past the left or top edge).
      const placements = [];
      for (const d of [90, 130]) placements.push(['left', -d], ['right', d], ['top', -d], ['bottom', d]);
      for (const [side, off] of placements) {
        const r = await page.eval(`(async () => {
          const m = game.map, t = ${tile}, side = ${JSON.stringify(side)}, off = ${off}, want = ${dir ? 'true' : 'false'};
          const x = m.px[t], y = m.py[t];
          const sx = side === 'left' ? off : side === 'right' ? m.w + off : m.w / 2;
          const sy = side === 'top' ? off : side === 'bottom' ? m.h + off : m.h / 2;
          m.cx = x - (sx - m.w / 2) / m.k; m.cy = y - (sy - m.h / 2) / m.k;
          const grab = () => { m.dirty = true; m.drawNow(); const c = m.canvas; return { data: c.getContext('2d').getImageData(0, 0, c.width, c.height).data, png: want ? c.toDataURL('image/png') : null }; };
          m.noCull = false; const a = grab(); m.noCull = true; const b = grab(); m.noCull = false;
          let d = 0; for (let i = 0; i < a.data.length; i += 4) if (a.data[i] !== b.data[i] || a.data[i + 1] !== b.data[i + 1] || a.data[i + 2] !== b.data[i + 2]) d++;
          return { d, culled: a.png, uncut: b.png };
        })()`);
        console.log(`${name} tile ${tile} ${side} ${off}px: ${r.d} px differ`);
        if (r.d) {
          failed++;
          if (dir) {
            const base = `${name}-${tile}-${side}-${Math.abs(off)}`;
            writeFileSync(join(dir, `${base}-culled.png`), Buffer.from(r.culled.split(',')[1], 'base64'));
            writeFileSync(join(dir, `${base}-uncut.png`), Buffer.from(r.uncut.split(',')[1], 'base64'));
          }
        }
      }
    } finally {
      await page.close();
    }
  }
  return failed;
}

/** Each layer that starts off, turned on alone over the view: its PNG, and the share of pixels it moves by more than 48. */
async function overlays(view, dir) {
  const { page } = await setUp(view);
  try {
    const layers = await page.eval('Object.keys(game.map.on).filter((n) => !game.map.on[n])');
    for (const n of layers) {
      const r = await page.eval(`(() => {
        const m = game.map, c = m.canvas;
        const grab = () => { m.dirty = true; m.drawNow(); return c.getContext('2d').getImageData(0, 0, c.width, c.height).data; };
        const a = grab(); m.setLayer(${JSON.stringify(n)}, true); const b = grab(); const png = c.toDataURL('image/png'); m.setLayer(${JSON.stringify(n)}, false);
        let d = 0; for (let i = 0; i < a.length; i += 4) if (Math.abs(a[i] - b[i]) > 48 || Math.abs(a[i + 1] - b[i + 1]) > 48 || Math.abs(a[i + 2] - b[i + 2]) > 48) d++;
        return { png, share: d / (a.length / 4) };
      })()`);
      writeFileSync(join(dir, `${view.name}-${n}.png`), Buffer.from(r.png.split(',')[1], 'base64'));
      console.log(`${view.name} ${n}: ${(100 * r.share).toFixed(1)}% of pixels move by > 48`);
    }
  } finally {
    await page.close();
  }
}

/** Each static layer's median draw time over five redraws, as drawStatic draws them (layers under 0.3 ms are left out). */
async function layers(view) {
  const { page } = await setUp(view);
  try {
    const r = await page.eval(`(async () => {
      const { LAYERS } = await import('./map/view.js');
      const m = game.map, ctx = m.lctx, t = {};
      for (let i = 0; i < 5; i++) {
        m.collectVisible();
        ctx.setTransform(1, 0, 0, 1, 0, 0); ctx.fillRect(0, 0, m.layer.width, m.layer.height); m.setWorld(ctx);
        for (const l of LAYERS) {
          if (!l.static || !(m.on[l.name] || l.always?.(m))) continue;
          const t0 = performance.now(); l.draw(ctx, m.model, m); (t[l.name] ??= []).push(performance.now() - t0);
        }
      }
      return Object.entries(t).map(([n, xs]) => [n, xs.sort((a, b) => a - b)[2]]).filter(([, x]) => x >= 0.3);
    })()`);
    console.log(`${view.name}  k ${view.k}  ${r.map(([n, x]) => `${n} ${x.toFixed(1)}`).join('  ')}`);
  } finally {
    await page.close();
  }
}

/** The pan (the frame a player makes most): twelve panBy steps, each drawNow timed. */
async function pan(view, dir, cameras) {
  const { page, asked } = await setUp(view);
  try {
    const r = await page.eval(`(() => {
      const P = ${PLAN};
      const m = game.map;
      m.cx = P.sx; m.cy = P.sy; m.dirty = true; m.drawNow();
      const ms = [];
      for (let j = 0; j < P.N; j++) {
        m.panBy(P.dx, P.dy);
        const t0 = performance.now(); m.drawNow(); ms.push(performance.now() - t0);
      }
      return { ms, terrainMs: m.stats.terrainMs, overlayMs: m.stats.overlayMs, cx: m.cx, cy: m.cy, k: m.k, dx: P.dx, dy: P.dy, exact: P.exact,
        landed: m.cx === P.ex && m.cy === P.ey, png: ${dir ? "m.canvas.toDataURL('image/png')" : 'null'} };
    })()`);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    const s = [...r.ms].sort((a, b) => a - b);
    const median = (s[r.ms.length / 2 - 1] + s[r.ms.length / 2]) / 2;
    const slowest = s[s.length - 1];
    const camera = r.landed && r.cx === asked.cx && r.cy === asked.cy && r.k === asked.k ? 'on the view camera' : `OFF the view camera (${r.cx} ${r.cy} k ${r.k} vs ${asked.cx} ${asked.cy} k ${asked.k})`;
    console.log(`${view.name}  k ${view.k}  pan ${r.ms.length} frames: median ${median.toFixed(1)}  slowest ${slowest.toFixed(1)}  last terrainMs ${r.terrainMs.toFixed(1)} overlayMs ${r.overlayMs.toFixed(1)}  drag ${r.dx},${r.dy}  ${camera}`);
    cameras[view.name] = { cx: r.cx, cy: r.cy, k: r.k, drag: [r.dx, r.dy], steps: STEPS, landed: r.landed };
    if (dir) writeFileSync(join(dir, `${view.name}.png`), Buffer.from(r.png.split(',')[1], 'base64'));
    return { median, slowest };
  } finally {
    await page.close();
  }
}

/** main's shift noise: the static layer at the drag's start, shifted by the drag's device px, against the static layer at its end. */
async function shift(view) {
  const { page } = await setUp(view);
  try {
    const r = await page.eval(`(() => {
      const P = ${PLAN};
      const m = game.map, c = m.layer, ctx = c.getContext('2d');
      const layer = () => ctx.getImageData(0, 0, c.width, c.height).data;
      m.cx = P.sx; m.cy = P.sy; m.dirty = true; m.drawNow(); const A = layer();
      m.cx = P.ex; m.cy = P.ey; m.dirty = true; m.drawNow(); const B = layer();
      const dpr = m.dpr, Dx = Math.round(P.N * P.dx * dpr), Dy = Math.round(P.N * P.dy * dpr), w = c.width, h = c.height;
      let n = 0, d = 0, d40 = 0;
      for (let y = 0; y < h; y++) {
        const sy = y - Dy; if (sy < 0 || sy >= h) continue;
        for (let x = 0; x < w; x++) {
          const sx = x - Dx; if (sx < 0 || sx >= w) continue;
          const o = (y * w + x) * 4, p = (sy * w + sx) * 4; n++;
          const dd = Math.max(Math.abs(A[p] - B[o]), Math.abs(A[p + 1] - B[o + 1]), Math.abs(A[p + 2] - B[o + 2]));
          if (dd > 0) d++;
          if (dd > 40) d40++;
        }
      }
      return { n, d, d40, Dx, Dy, dpr };
    })()`);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    console.log(`${view.name}  k ${view.k}  shift ${r.Dx},${r.Dy} device px  overlap ${r.n}  differ ${r.d} (${(100 * r.d / r.n).toFixed(3)}%)  by more than 40 ${r.d40} (${(100 * r.d40 / r.n).toFixed(3)}%)`);
  } finally {
    await page.close();
  }
}

const mode = process.argv[2];
if (mode === '--hits') {
  const views = named(args('--hits').list, HITS);
  let misses = 0;
  for (const view of views) misses += await hits(view);
  if (misses) { console.error(`shots-probe --hits: ${misses} drawn figures are not on their own tile`); process.exit(1); }
} else if (mode === '--pop') {
  const { dir } = args('--pop');
  if (dir) mkdirSync(dir, { recursive: true });
  const failed = await pops(dir);
  if (failed) { console.error(`shots-probe --pop: ${failed} placements draw differently with culling off`); process.exit(1); }
} else if (mode === '--overlays') {
  const { dir, list } = args('--overlays');
  if (!dir) throw new Error('usage: --overlays DIR [NAME,…]');
  mkdirSync(dir, { recursive: true });
  for (const view of named(list, ['medium9-play', 'medium9-places-close', 'small2-edge'])) await overlays(view, dir);
} else if (mode === '--layers') {
  for (const view of named(args('--layers').list, VERIFY)) await layers(view);
} else if (mode === '--pan') {
  const { dir, list } = args('--pan');
  if (dir) mkdirSync(dir, { recursive: true });
  const cameras = {};
  for (const view of named(list, VERIFY)) await pan(view, dir, cameras);
  if (dir) writeFileSync(join(dir, 'pan.json'), `${JSON.stringify(cameras, null, 1)}\n`);
} else if (mode === '--shift') {
  for (const view of named(args('--shift').list, VERIFY)) await shift(view);
} else {
  console.error('usage: node www/shots-probe.mjs --hits [NAME,…] | --pop [DIR] | --overlays DIR [NAME,…] | --layers [NAME,…] | --pan [DIR] [NAME,…] | --shift [NAME,…]');
  process.exit(2);
}
