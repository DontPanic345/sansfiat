// The shore and fringe pieces on a staged map (map/edges.js): a pond of one tile in plains, a neck (land with sea on
// one side and a lake on the other), a bay (a sea inlet of two tiles in plains) and desert beside plains. The pieces at
// named tiles are the spec's (their keys, kinds and counts), every land–water edge has its bank and every plains–desert
// edge its fringe. With the camera put so the pond's shore tile sits 1.0 world unit beyond the canvas's bottom edge, the
// culled draw and the unculled draw give identical PNGs, and the culled one counts the pieces whose rectangle meets the canvas.
//   node www/play/edges.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { grid, marks, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-edges-')));
mkdirSync(out, { recursive: true });

// Radius 8: sea on the rim, plains inside, and the features by axial (q, r).
const R = 8;
const EXPECT_TERRAIN = {
  '0,0': '=', // the pond: one lake tile in plains
  '0,1': '.', // the pond's shore tile, south of it
  '3,2': '.', // the neck: land between the sea at 4,2 and the lake at 2,2
  '4,2': '~',
  '2,2': '=', '1,2': '=', '1,3': '=', '0,3': '=', '0,4': '=', // the lake the neck meets
  '-3,-1': '~', '-3,-2': '~', // the bay: two sea tiles inland, one water neighbour each
  '3,-4': ':', '4,-4': ':', // desert beside plains
};
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const SPEC = {
  map: grid(R, (q, r) => (dist(q, r) === R ? '~' : EXPECT_TERRAIN[`${q},${r}`] ?? '.')),
  marks: marks(R, { A: [-6, 0], B: [6, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
};
const SHORE_TILE = '0,1';
const K = 35;

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`edges: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};

// Runs in the page: the named tiles' indices, the pieces, the camera on the shore tile 1.0 unit below the canvas, and the
// two draws (culled and unculled), with the rectangles the layer computes, so the culled count can be checked.
const inPage = async ({ shoreKey, k }) => {
  const m = game.map;
  const { edgePieces, FRINGE, kindOf } = await import('/map/edges.js');
  const names = m.names;
  // Every tile's index by its axial key, so a test can find any neighbour.
  const idx = {};
  for (let i = 0; i < m.n; i++) idx[`${m.l.q[i]},${m.l.r[i]}`] = i;
  const { terrain, lake, river, flow } = m.l;
  const pieces = edgePieces({ terrain, lake, river, flow, nb: m.nb, px: m.px, py: m.py, names });
  const kinds = Array.from(terrain, (_, i) => kindOf(i, { terrain, lake, names }));
  const wet = (i) => kinds[i] === 'sea' || kinds[i] === 'lake';
  // Counted here, not in the module: land–water adjacencies, and land–land pairs of different kinds one of which bleeds.
  let landWater = 0, fringes = 0;
  for (let i = 0; i < m.n; i++) for (let d = 0; d < 6; d++) {
    const j = m.nb[i * 6 + d];
    if (j < 0 || j < i) continue;
    if (wet(i) !== wet(j)) landWater++;
    else if (!wet(i) && kinds[i] !== kinds[j] && (FRINGE.includes(kinds[i]) || FRINGE.includes(kinds[j]))) fringes++;
  }
  const s = idx[shoreKey];
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
  const shoreEdge = pieces.find((p) => p.kind === 'edge' && p.tile === s);
  const shoreRect = shoreEdge ? rectOf(shoreEdge) : null;
  m.drawNow();
  const culled = { png: m.canvas.toDataURL('image/png'), edges: m.stats.edges };
  m.noCull = true;
  m.dirty = true;
  m.drawNow();
  const uncut = { png: m.canvas.toDataURL('image/png'), edges: m.stats.edges };
  m.noCull = false;
  m.dirty = true;
  m.drawNow();
  return {
    idx, pieces, camera, asked, visible, culled, uncut, landWater, fringes,
    shoreCentreY: Math.round(((m.py[s] - m.cy) * m.k + m.h / 2) * 1000) / 1000,
    canvasBottom: m.h,
    shoreEdge: shoreRect ? { visible: meets(shoreRect) } : null,
    k: m.k,
  };
};

const page = await open('');
try {
  await stage(page, SPEC);
  const r = await page.eval(`(${inPage})(${JSON.stringify({ shoreKey: SHORE_TILE, k: K })})`);
  const { idx, pieces, camera, asked, visible, culled, uncut } = r;
  const T = (key) => { const [q, r2] = key.split(',').map(Number); return { q, r: r2, i: idx[key] ?? -1 }; };
  const EDGE = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
  const dirOf = (a, b) => EDGE.findIndex(([dq, dr]) => a.q + dq === b.q && a.r + dr === b.r);
  const neighbour = (a, d) => T(`${a.q + EDGE[d][0]},${a.r + EDGE[d][1]}`);
  const pair = (a, b) => `${Math.min(a.i, b.i) % 2}`;
  const withOther = (other, kind) => pieces.filter((p) => p.kind === kind && p.other === other.i);
  const keysOf = (list) => list.map((p) => p.key).sort();
  const sameKeys = (list, want) => JSON.stringify(keysOf(list)) === JSON.stringify([...want].sort());

  // Every named tile is on the map.
  need(Object.keys(EXPECT_TERRAIN).every((k) => T(k).i >= 0), 'every named tile is on the map');

  // The pond: its six plains neighbours each have the bank toward it, and its six vertices each draw an in corner on it.
  const P = T('0,0');
  const pondEdges = [0, 1, 2, 3, 4, 5].map((d) => { const N = neighbour(P, d); return { key: `shore.edge.${(d + 3) % 6}.${pair(N, P)}`, tile: N.i, other: P.i }; });
  need(sameKeys(withOther(P, 'edge'), pondEdges.map((e) => e.key)) && withOther(P, 'edge').every((p) => pondEdges.some((e) => e.key === p.key && e.tile === p.tile)), 'the pond has a bank toward each of its six neighbours, keyed by direction and variant');
  need(sameKeys(pieces.filter((p) => p.kind === 'corner' && p.tile === P.i), ['shore.in.0', 'shore.in.1', 'shore.in.2', 'shore.in.3', 'shore.in.4', 'shore.in.5']), 'the pond draws its six in corners, one at each vertex');

  // The shore tile of the pond: one bank, to the pond (north of it), and no fringe.
  const S = T(SHORE_TILE);
  need(sameKeys(pieces.filter((p) => p.tile === S.i && p.kind === 'edge'), [`shore.edge.${dirOf(S, P)}.${pair(S, P)}`]), 'the shore tile has one bank, to the pond, and no other edge');

  // The neck: land between sea and a lake, a bank toward each water tile, and no corner on the land tile itself.
  const N = T('3,2'), A = T('4,2'), L = T('2,2');
  need(sameKeys(pieces.filter((p) => p.kind === 'edge' && p.tile === N.i), [`shore.edge.${dirOf(N, A)}.${pair(N, A)}`, `shore.edge.${dirOf(N, L)}.${pair(N, L)}`]), 'the neck has a bank to the sea and a bank to the lake');
  need(pieces.filter((p) => p.kind === 'corner' && p.tile === N.i).length === 0, 'the neck\'s own tile draws no corner (its water neighbours are not adjacent)');
  need(pieces.filter((p) => p.kind === 'corner' && p.tile === A.i).length === 6, 'the sea tile beside the neck draws six in corners');

  // The bay: a sea tile with one water neighbour, five banks, four in corners; the two vertices shared with its water
  // neighbour are out corners on the land tiles either side.
  const B = T('-3,-1'), C = T('-3,-2');
  const bayDirs = [0, 1, 2, 3, 5];
  need(sameKeys(withOther(B, 'edge'), bayDirs.map((d) => `shore.edge.${(d + 3) % 6}.${pair(neighbour(B, d), B)}`)), 'the bay has a bank toward each of its five land neighbours');
  need(sameKeys(pieces.filter((p) => p.kind === 'corner' && p.tile === B.i), ['shore.in.0', 'shore.in.1', 'shore.in.2', 'shore.in.3']), 'the bay draws in corners on its four land-facing vertices');
  const W = neighbour(B, 3), NE = neighbour(B, 5);
  need(sameKeys(pieces.filter((p) => p.kind === 'corner' && p.tile === W.i), ['shore.out.0']), 'the land west of the bay draws the out corner at its shared vertex with the bay\'s water neighbour');
  need(sameKeys(pieces.filter((p) => p.kind === 'corner' && p.tile === NE.i), ['shore.out.3']), 'the land north-east of the bay draws its out corner too');
  need(withOther(C, 'edge').length === 5, 'the other sea tile of the bay has five banks');

  // The desert: each plains tile beside desert has a fringe of desert in it, toward the desert; no fringe between desert tiles.
  const D1 = T('3,-4'), D2 = T('4,-4');
  const fr = pieces.filter((p) => p.kind === 'fringe');
  need(fr.every((p) => p.key.startsWith('fringe.desert.')), 'every fringe here is desert');
  const wantFringes = [D1, D2].flatMap((D) => [0, 1, 2, 3, 4, 5].map((d) => ({ D, d, P2: neighbour(D, d) }))).filter(({ P2 }) => P2.i !== D1.i && P2.i !== D2.i).map(({ D, P2 }) => `fringe.desert.${dirOf(P2, D)}.${pair(P2, D)}@${P2.i}`).sort();
  need(JSON.stringify(fr.map((p) => `${p.key}@${p.tile}`).sort()) === JSON.stringify(wantFringes), `the desert has a fringe in each of its ${wantFringes.length} plains neighbours, toward it`);

  // The counts: a bank per land–water edge, a fringe per land–land edge that bleeds.
  need(pieces.filter((p) => p.kind === 'edge').length === r.landWater, `a bank per land–water edge (${r.landWater})`);
  need(fr.length === r.fringes, `a fringe per bleeding land–land edge (${r.fringes})`);

  // The order: fringes from the lowest kind to the highest, then banks, then corners; each by y, x, tile, other.
  let ordered = true;
  for (let k = 1; k < pieces.length; k++) {
    const a = pieces[k - 1], b = pieces[k];
    const g = (p) => ({ fringe: 0, edge: 1, corner: 2 })[p.kind];
    if (g(a) !== g(b)) { if (g(a) > g(b)) ordered = false; continue; }
    if (a.y !== b.y) { if (a.y > b.y) ordered = false; continue; }
    if (a.x !== b.x) { if (a.x > b.x) ordered = false; continue; }
    if (a.tile !== b.tile && a.tile > b.tile) ordered = false;
  }
  need(ordered, `the ${pieces.length} pieces are in draw order`);

  // The camera: as asked, so the clamp did not move it, and the pond's shore tile sits 1.0 unit beyond the bottom edge.
  need(camera.cx === asked.cx && camera.cy === asked.cy && camera.k === asked.k, `the camera is as asked (${JSON.stringify(camera)}), not clamped`);
  need(Math.abs(r.shoreCentreY - (r.canvasBottom + K)) < 1e-6, 'the shore tile\'s centre is 1.0 world unit below the canvas');
  need(r.shoreEdge && r.shoreEdge.visible, 'the shore tile\'s bank reaches onto the canvas');

  // The culled draw counts the pieces whose rectangle meets the canvas; the unculled draw counts them all.
  need(culled.edges === visible, `stats.edges counts the ${visible} pieces whose rectangle meets the canvas (${culled.edges})`);
  need(uncut.edges === pieces.length, `the unculled draw counts every piece (${uncut.edges} of ${pieces.length})`);
  need(visible < pieces.length, 'some pieces are culled');
  need(culled.png === uncut.png, 'the culled and unculled draws are identical PNGs');
  writeFileSync(join(out, 'edges.png'), Buffer.from(culled.png.split(',')[1], 'base64'));
  writeFileSync(join(out, 'edges-nocull.png'), Buffer.from(uncut.png.split(',')[1], 'base64'));

  for (const e of page.errors()) need(false, `page error: ${e}`);
} catch (e) {
  need(false, e.stack ?? String(e));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
