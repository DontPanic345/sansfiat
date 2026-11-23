// The ridge layer on a staged map: a lone peak, a straight run of three ending in caps, a bend and a fork, on plains in
// sea, and a lone peak far to the south. The pieces are the spec's (map/ridges.js: the keys, the kinds, one ridge per shared
// edge at its midpoint, the draw order). With the camera put so the straight run's middle peak sits 1.0 world unit below
// the canvas, that peak still reaches onto the canvas: the culled draw counts the pieces whose drawn rectangle meets the
// canvas, the unculled draw counts them all, and the two PNGs are identical.   node www/play/ridges.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { grid, marks, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-ridges-')));
mkdirSync(out, { recursive: true });

// Radius 8. Each mountain's class, by its axial (q, r): a lone peak, a fork (three neighbours, spread), a straight run
// of three (two caps), a bend (two that are 120° apart, each a cap), and a lone peak far south, out of the view.
const R = 8;
const EXPECT = {
  '3,-5': 'lone',
  '-2,-3': 'fork', '-1,-3': 'end', '-3,-2': 'end', '-2,-4': 'end',
  '-4,2': 'end', '-3,2': 'straight', '-2,2': 'end',
  '2,0': 'bend', '3,-1': 'end', '2,1': 'end',
  '0,4': 'lone',
};
const CAMERA_TILE = '-3,2'; // the straight run's middle: a peak
const K = 35;
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const SPEC = {
  map: grid(R, (q, r) => (dist(q, r) === R ? '~' : `${q},${r}` in EXPECT ? '^' : '.')),
  marks: marks(R, { A: [-6, 0], B: [6, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
};

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`ridges: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};

// Runs in the page: the pieces for the staged map, the camera on CAMERA_TILE 1.0 unit below the canvas's bottom edge,
// and the two draws. The drawn rectangle of each piece is the layer's arithmetic, so the culled count can be checked.
const inPage = async ({ expect, cameraTile, k }) => {
  const m = game.map;
  const { ridgePieces, ridgeClass } = await import('/map/ridges.js');
  const M = m.byName.mountain;
  const tileOf = (key) => { const [q, r] = key.split(',').map(Number); return m.at(q, r); };
  const classes = {}, tiles = {};
  for (const key of Object.keys(expect)) {
    const i = tileOf(key);
    tiles[key] = i;
    classes[key] = i >= 0 && m.l.terrain[i] === M ? ridgeClass(i, { terrain: m.l.terrain, nb: m.nb, mountain: M }) : 'not a mountain';
  }
  const pieces = ridgePieces({ terrain: m.l.terrain, nb: m.nb, px: m.px, py: m.py, mountain: M });
  const t = tileOf(cameraTile);
  m.k = k;
  m.cx = m.px[t];
  m.cy = m.py[t] - m.h / 2 / k - 1.0;
  m.viewChanged();
  await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
  const camera = { cx: m.cx, cy: m.cy, k: m.k };
  const W = m.canvas.width, H = m.canvas.height;
  const cellH = Math.round(2 * 1.03 * m.k * m.dpr);
  const rectOf = (p) => {
    const cell = game.atlas.cell(p.key);
    const h = Math.round((cell.h * cellH) / 148);
    const img = game.atlas.scaled(p.key, h);
    const ax = ((cell.ax ?? cell.w / 2) * img.width) / cell.w;
    const ay = ((cell.ay ?? cell.h / 2) * h) / cell.h;
    return {
      x: Math.round(((p.x - m.cx) * m.k + m.w / 2) * m.dpr - ax),
      y: Math.round(((p.y - m.cy) * m.k + m.h / 2) * m.dpr - ay),
      w: img.width,
      h,
    };
  };
  const meets = (r) => r.x + r.w > 0 && r.y + r.h > 0 && r.x < W && r.y < H;
  const visible = pieces.filter((p) => meets(rectOf(p))).length;
  const peak = pieces.find((p) => p.tile === t && p.kind !== 'ridge');
  const peakRect = peak ? rectOf(peak) : null;
  m.drawNow();
  const culled = { png: m.canvas.toDataURL('image/png'), ridges: m.stats.ridges };
  m.noCull = true;
  m.dirty = true;
  m.drawNow();
  const uncut = { png: m.canvas.toDataURL('image/png'), ridges: m.stats.ridges };
  m.noCull = false;
  m.dirty = true;
  m.drawNow();
  // The bottom edge in world units, and the peak's drawn top in device px: it reaches onto the canvas when its top is above H.
  return {
    classes, tiles, pieces, camera, asked: { cx: m.px[t], cy: m.py[t] - m.h / 2 / k - 1.0, k }, visible, culled, uncut,
    peak: peak && { key: peak.key, ground: peak.y, top: peakRect.y },
    canvasBottom: H, groundBottomPx: Math.round(((m.py[t] - m.cy) * m.k + m.h / 2) * m.dpr),
  };
};

const page = await open('');
try {
  await stage(page, SPEC);
  const r = await page.eval(`(${inPage})(${JSON.stringify({ expect: EXPECT, cameraTile: CAMERA_TILE, k: K })})`);
  const { classes, tiles, pieces, camera, asked, visible, culled, uncut } = r;

  // The classes the staged map has, from the spec's rule.
  for (const [key, want] of Object.entries(EXPECT)) need(classes[key] === want, `tile ${key} is ${want} (${classes[key]})`);

  // The peaks: one per mountain tile, of its class, with its class's key.
  const pos = (key) => key.split(',').map(Number);
  const peaks = pieces.filter((p) => p.kind !== 'ridge');
  need(peaks.length === Object.keys(EXPECT).length, `${peaks.length} peaks, one per mountain tile`);
  for (const key of Object.keys(EXPECT)) {
    const p = peaks.find((x) => x.tile === tiles[key]);
    need(p !== undefined && p.kind === EXPECT[key], `tile ${key} has its ${EXPECT[key]} peak`);
  }

  // The ridges: one per shared mountain edge, from its tile towards E, SE or SW, at the midpoint, with the spec's variant.
  const DIRS = [[1, 0], [0, 1], [-1, 1]];
  const NAMES = ['e', 'se', 'sw'];
  const px = (q, r) => Math.sqrt(3) * (q + r / 2);
  const py = (r) => 1.5 * r;
  const wantRidges = [];
  for (const key of Object.keys(EXPECT)) {
    const [q, r] = pos(key);
    DIRS.forEach(([dq, dr], d) => {
      const nkey = `${q + dq},${r + dr}`;
      if (!(nkey in EXPECT)) return;
      const [nq, nr] = pos(nkey);
      const i = tiles[key], j = tiles[nkey];
      wantRidges.push(`${(px(q, r) + px(nq, nr)) / 2},${(py(r) + py(nr)) / 2},mountain.ridge.${NAMES[d]}.${Math.min(i, j) % 2}`);
    });
  }
  const gotRidges = pieces.filter((p) => p.kind === 'ridge').map((p) => `${p.x},${p.y},${p.key}`);
  need(gotRidges.length === wantRidges.length, `${gotRidges.length} ridges, one per shared mountain edge (${wantRidges.length})`);
  need(JSON.stringify([...gotRidges].sort()) === JSON.stringify([...wantRidges].sort()), 'each ridge sits at its edge\'s midpoint, with its key');

  // The keys of the peaks, by class: the lone peak's variant is i % 2, the others' i % 3.
  const keyOk = peaks.every((p) => (p.kind === 'lone' ? /^mountain\.lone\.[01]$/ : /^mountain\.peak\.[012]$/).test(p.key));
  need(keyOk, 'every peak is a lone or a peak key');
  need(peaks.filter((p) => p.kind === 'lone').length === 2 && peaks.filter((p) => p.kind === 'straight').length === 1, 'two lone peaks and a straight run\'s middle');

  // The draw order: by y, ridges before peaks at a tie, then x, then tile.
  const rank = (p) => (p.kind === 'ridge' ? 0 : 1);
  let ordered = true;
  for (let k = 1; k < pieces.length; k++) {
    const a = pieces[k - 1], b = pieces[k];
    const ok = a.y < b.y || (a.y === b.y && (rank(a) < rank(b) || (rank(a) === rank(b) && (a.x < b.x || (a.x === b.x && a.tile <= b.tile)))));
    if (!ok) ordered = false;
  }
  need(ordered, `the ${pieces.length} pieces are in draw order`);

  // The camera: as asked, so the clamp did not move it.
  need(camera.cx === asked.cx && camera.cy === asked.cy && camera.k === asked.k, `the camera is as asked (${JSON.stringify(camera)}), not clamped`);

  // The culled draw: the peak reaches onto the canvas, and what misses the canvas is culled.
  need(culled.ridges === visible, `stats.ridges counts the ${visible} pieces whose rectangle meets the canvas (${culled.ridges})`);
  need(uncut.ridges === pieces.length, `the unculled draw counts every piece (${uncut.ridges} of ${pieces.length})`);
  need(visible < pieces.length, 'the lone peak far south is culled');
  need(r.peak && r.peak.top < r.canvasBottom && r.groundBottomPx > r.canvasBottom, 'the straight run\'s peak reaches onto the canvas from below its tile');
  need(culled.png === uncut.png, 'the culled and unculled draws are identical PNGs');
  writeFileSync(join(out, 'ridges.png'), Buffer.from(culled.png.split(',')[1], 'base64'));
  writeFileSync(join(out, 'ridges-nocull.png'), Buffer.from(uncut.png.split(',')[1], 'base64'));

  for (const e of page.errors()) need(false, `page error: ${e}`);
} catch (e) {
  need(false, e.stack ?? String(e));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
