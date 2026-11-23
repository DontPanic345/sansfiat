// The wood pieces on a staged map (map/woods.js): a seven-tile wood (a forest tile and its six forest neighbours), a lone
// forest tile, and a strip of marsh on the sea. The interior tile holds only big trees; the lone tile has big trees in its
// middle, none in its band (within `band` of an edge) but small ones; the marsh holds reeds and draws no bank on its sea
// edges. With the camera put so the wood's southern tile sits 1.0 world unit below the canvas, the culled and unculled
// draws give identical PNGs, and the culled one counts the pieces whose rectangle meets the canvas.
//   node www/play/woods.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { grid, marks, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-woods-')));
mkdirSync(out, { recursive: true });

// Radius 8: sea on the rim, plains inside. The wood is centred on (-3, 0), the lone tree on (3, -2), and the marsh runs
// along the north rim on the sea: (0, -7), (1, -7), (2, -7).
const R = 8;
const WOOD = [[-3, 0], [-2, 0], [-3, 1], [-4, 1], [-4, 0], [-3, -1], [-2, -1]];
const LONE = [3, -2];
const MARSH = [[0, -7], [1, -7], [2, -7]];
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const EXPECT = {};
for (const [q, r] of WOOD) EXPECT[`${q},${r}`] = 'f';
EXPECT[`${LONE[0]},${LONE[1]}`] = 'f';
for (const [q, r] of MARSH) EXPECT[`${q},${r}`] = '%';
const SPEC = {
  map: grid(R, (q, r) => (dist(q, r) === R ? '~' : EXPECT[`${q},${r}`] ?? '.')),
  marks: marks(R, { A: [-6, 0], B: [6, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
};
const K = 35;

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`woods: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};

// Runs in the page: the named tiles' indices, the pieces the layer keeps, the rule's depths counted from each piece's
// point, the camera on the wood's southern tile 1.0 unit below the canvas, and the two draws (culled and unculled) with
// the rectangles the layer computes, so the culled count can be checked.
const inPage = async ({ wood, lone, marsh, k }) => {
  const m = game.map;
  const { woodPieces, WOODS } = await import('/map/woods.js');
  const { edgePieces, kindOf } = await import('/map/edges.js');
  const names = m.names;
  const idx = {};
  for (let i = 0; i < m.n; i++) idx[`${m.l.q[i]},${m.l.r[i]}`] = i;
  const { terrain, lake, river, flow } = m.l;
  const forest = m.byName.forest, marshT = m.byName.marsh;
  const pieces = woodPieces({ terrain, river, flow, nb: m.nb, px: m.px, py: m.py, forest, marsh: marshT, fields: new Set(m.woodsFields) });
  const edges = edgePieces({ terrain, lake, river, flow, nb: m.nb, px: m.px, py: m.py, names });
  // The wood's southern tile: the highest y, the lower index on a tie.
  const southern = wood.map((key) => idx[key]).reduce((a, b) => (m.py[b] > m.py[a] || (m.py[b] === m.py[a] && b < a) ? b : a));
  const s = southern;
  m.k = k;
  m.cx = m.px[s];
  m.cy = m.py[s] - m.h / 2 / k - 1.0;
  m.viewChanged();
  await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
  const camera = { cx: m.cx, cy: m.cy, k: m.k };
  const asked = { cx: m.px[s], cy: m.py[s] - m.h / 2 / k - 1.0, k };
  const W = m.canvas.width, H = m.canvas.height;
  const cellH = Math.round(2 * 1.03 * m.k * m.dpr);
  const rectOf = (p) => {
    const cell = game.atlas.cell(p.key);
    const h = Math.round((cell.h * cellH) / 148);
    const img = game.atlas.scaled(p.key, h);
    const ax = ((cell.ax ?? cell.w / 2) * img.width) / cell.w;
    const ay = ((cell.ay ?? cell.h / 2) * h) / cell.h;
    return { x: Math.round(((p.x - m.cx) * m.k + m.w / 2) * m.dpr - ax), y: Math.round(((p.y - m.cy) * m.k + m.h / 2) * m.dpr - ay), w: img.width, h };
  };
  const meets = (r) => r.x + r.w > 0 && r.y + r.h > 0 && r.x < W && r.y < H;
  const visible = pieces.filter((p) => meets(rectOf(p))).length;
  m.drawNow();
  const culled = { png: m.canvas.toDataURL('image/png'), woods: m.stats.woods };
  m.noCull = true;
  m.dirty = true;
  m.drawNow();
  const uncut = { png: m.canvas.toDataURL('image/png'), woods: m.stats.woods };
  m.noCull = false;
  m.dirty = true;
  m.drawNow();
  // The rule's depth from the nearest open edge, counted from the piece's own point: a tile's neighbours that are not
  // forest (or off the map) are its open edges, and each edge's outward normal comes from the axial direction.
  const EDGE = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
  const depthOf = (p) => {
    let depth = Infinity;
    for (let d = 0; d < 6; d++) {
      const j = m.nb[p.tile * 6 + d];
      if (j >= 0 && terrain[j] === forest) continue;
      const [dq, dr] = EDGE[d];
      const wx = Math.sqrt(3) * (dq + dr / 2), wy = 1.5 * dr;
      const len = Math.hypot(wx, wy);
      depth = Math.min(depth, Math.sqrt(3) / 2 - ((p.x - m.px[p.tile]) * wx + (p.y - m.py[p.tile]) * wy) / len);
    }
    return depth;
  };
  const onTile = (t) => pieces.filter((p) => p.tile === t);
  const edgesOn = (t) => edges.filter((p) => p.kind === 'edge' && p.tile === t);
  return {
    idx, camera, asked, visible, culled, uncut, pieces: pieces.length, southern: s,
    interior: onTile(idx[wood[0]]).map((p) => p.key),
    lone: onTile(idx[lone]).map((p) => ({ key: p.key, depth: depthOf(p) })),
    marsh: marsh.map((key) => ({ keys: onTile(idx[key]).map((p) => p.key), banks: edgesOn(idx[key]).length })),
    band: WOODS.band,
    shoreCentreY: Math.round(((m.py[s] - m.cy) * m.k + m.h / 2) * 1000) / 1000,
    canvasBottom: m.h,
  };
};

const page = await open('');
try {
  await stage(page, SPEC);
  const r = await page.eval(`(${inPage})(${JSON.stringify({ wood: WOOD.map(([q, r2]) => `${q},${r2}`), lone: `${LONE[0]},${LONE[1]}`, marsh: MARSH.map(([q, r2]) => `${q},${r2}`), k: K })})`);
  const { idx, camera, asked, visible, culled, uncut } = r;
  const centre = `${WOOD[0][0]},${WOOD[0][1]}`;

  // The interior tile: all six neighbours are forest, so every slot is deep: big trees only.
  need(r.interior.length > 0 && r.interior.every((k) => k.startsWith('tree.big.')), `the interior tile ${idx[centre]} holds big trees only (${r.interior.length})`);

  // The lone tile: big trees in its middle, none in its band, and small ones in the band.
  const lone = r.lone;
  const big = lone.filter((p) => p.key.startsWith('tree.big.'));
  const small = lone.filter((p) => p.key.startsWith('tree.small.'));
  need(big.length > 0 && big.every((p) => p.depth >= r.band), `the lone tile's big trees stand beyond its band (${big.length} big, all at depth ≥ ${r.band})`);
  need(small.length > 0 && small.every((p) => p.depth < r.band), `the lone tile's small trees are in its band (${small.length} small, all at depth < ${r.band})`);
  need(lone.length === big.length + small.length, 'the lone tile holds only trees');

  // The marsh: reeds on every slot, and no bank on its sea edges.
  const marsh = r.marsh;
  need(marsh.every((m) => m.keys.length > 0 && m.keys.every((k) => k.startsWith('reed.'))), `the marsh holds reeds (${marsh.map((m) => m.keys.length).join(', ')} a tile)`);
  need(marsh.every((m) => m.banks === 0), 'the marsh draws no bank on its sea edges');

  // The camera: as asked, and the wood's southern tile sits 1.0 unit below the canvas.
  need(camera.cx === asked.cx && camera.cy === asked.cy && camera.k === asked.k, `the camera is as asked (${JSON.stringify(camera)}), not clamped`);
  need(Math.abs(r.shoreCentreY - (r.canvasBottom + K)) < 1e-6, 'the southern tile sits 1.0 world unit below the canvas');

  // The culled draw counts the pieces whose rectangle meets the canvas; the unculled draw counts them all.
  need(culled.woods === visible, `stats.woods counts the ${visible} pieces whose rectangle meets the canvas (${culled.woods})`);
  need(uncut.woods === r.pieces, `the unculled draw counts every piece (${uncut.woods} of ${r.pieces})`);
  need(visible < r.pieces, 'some pieces are culled');
  need(culled.png === uncut.png, 'the culled and unculled draws are identical PNGs');
  writeFileSync(join(out, 'woods.png'), Buffer.from(culled.png.split(',')[1], 'base64'));
  writeFileSync(join(out, 'woods-nocull.png'), Buffer.from(uncut.png.split(',')[1], 'base64'));

  for (const e of page.errors()) need(false, `page error: ${e}`);
} catch (e) {
  need(false, e.stack ?? String(e));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
