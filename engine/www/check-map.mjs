// Checks the map's pieces from the wasm build under Node: the mountain pieces (map/ridges.js), the shore and fringe
// pieces (map/edges.js) on a medium world, each counted again here from the neighbours, and the shore pieces on a
// synthetic seven-tile layout with a river mouth, and the wood pieces (map/woods.js) on the same world, checked slot by slot
// against the rule, and on synthetic rings (a river through forest, with and without fields; marsh with a river).
// Usage: node www/check-map.mjs [seed]. Needs www/pkg (./build-web.sh).
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';

const here = fileURLToPath(new URL('.', import.meta.url));
const { initSync, Sim, sizes, terrain } = await import(pathToFileURL(`${here}pkg/web.js`).href);
initSync({ module: readFileSync(`${here}pkg/web_bg.wasm`) });
const { CORNER } = await import(pathToFileURL(`${here}map/geom.js`).href);

const SIZES = JSON.parse(sizes());
const TERRAIN = JSON.parse(terrain());
const medium = SIZES.find((s) => s.name === 'medium');
const seed = Number(process.argv[2] || 3);
const sim = new Sim(seed, medium.radius);

// The static layout, one column per field, and each tile's neighbours (EDGE_DIR order, -1 off the map).
const layout = JSON.parse(sim.layout());
const n = layout.q.length;
const at = new Map(layout.q.map((q, i) => [`${q},${layout.r[i]}`, i]));
const EDGE = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
// The mountain pieces (map/ridges.js): a class from the neighbours, a peak per mountain tile, a ridge per shared
// mountain edge at its midpoint, and the draw order. The classes are counted here, not read back from the module.
const { ridgeClass, ridgePieces } = await import(pathToFileURL(`${here}map/ridges.js`).href);
const MOUNTAIN = TERRAIN.findIndex((t) => t.name === 'mountain');
assert.ok(MOUNTAIN >= 0, 'the terrain has mountains');
const nbOf = new Int32Array(n * 6).fill(-1);
for (let i = 0; i < n; i++) EDGE.forEach(([dq, dr], d) => { nbOf[i * 6 + d] = at.get(`${layout.q[i] + dq},${layout.r[i] + dr}`) ?? -1; });
const pxOf = Float64Array.from(layout.q, (q, i) => Math.sqrt(3) * (q + layout.r[i] / 2));
const pyOf = Float64Array.from(layout.r, (r) => 1.5 * r);
const mountainDirs = (i) => EDGE.map((_, d) => d).filter((d) => nbOf[i * 6 + d] >= 0 && layout.terrain[nbOf[i * 6 + d]] === MOUNTAIN);
const mountains = [...Array(n).keys()].filter((i) => layout.terrain[i] === MOUNTAIN);
assert.ok(mountains.length > 0, 'the medium map has mountains');
const classCounted = (i) => {
  const dirs = mountainDirs(i);
  if (dirs.length === 0) return 'lone';
  if (dirs.length === 1) return 'end';
  if (dirs.length >= 3) return 'fork';
  const apart = Math.min(dirs[1] - dirs[0], 6 - (dirs[1] - dirs[0]));
  return apart === 3 ? 'straight' : apart === 2 ? 'bend' : 'sharp';
};
for (const i of mountains) assert.equal(ridgeClass(i, { terrain: layout.terrain, nb: nbOf, mountain: MOUNTAIN }), classCounted(i), `tile ${i}: its class`);
const pieces = ridgePieces({ terrain: layout.terrain, nb: nbOf, px: pxOf, py: pyOf, mountain: MOUNTAIN });
const peaks = pieces.filter((p) => p.kind !== 'ridge');
assert.equal(peaks.length, mountains.length, 'a peak per mountain tile');
for (const p of peaks) {
  const kind = classCounted(p.tile);
  assert.equal(p.kind, kind, `tile ${p.tile}: a peak of its class`);
  assert.equal(p.key, kind === 'lone' ? `mountain.lone.${p.tile % 2}` : `mountain.peak.${p.tile % 3}`, `tile ${p.tile}: its key`);
  assert.equal(p.x, pxOf[p.tile]);
  assert.equal(p.y, pyOf[p.tile]);
}
// Each shared mountain edge once, from its tile towards E, SE or SW, at the midpoint of the two centres.
const shared = [];
for (const i of mountains) for (const d of [0, 1, 2]) {
  const j = nbOf[i * 6 + d];
  if (j >= 0 && layout.terrain[j] === MOUNTAIN) shared.push({ i, j, d });
}
const ridges = pieces.filter((p) => p.kind === 'ridge');
assert.equal(ridges.length, shared.length, 'a ridge per shared mountain edge');
const want = shared.map(({ i, j, d }) => `${(pxOf[i] + pxOf[j]) / 2},${(pyOf[i] + pyOf[j]) / 2},mountain.ridge.${['e', 'se', 'sw'][d]}.${Math.min(i, j) % 2}`).sort();
const got = ridges.map((p) => `${p.x},${p.y},${p.key}`).sort();
assert.deepEqual(got, want, 'each ridge sits at its edge\'s midpoint, with its variant');
// Draw order: by y, ridges before peaks at a tie, then x, then tile.
const rank = (p) => (p.kind === 'ridge' ? 0 : 1);
for (let k = 1; k < pieces.length; k++) {
  const a = pieces[k - 1], b = pieces[k];
  const ordered = a.y < b.y || (a.y === b.y && (rank(a) < rank(b) || (rank(a) === rank(b) && (a.x < b.x || (a.x === b.x && a.tile <= b.tile)))));
  assert.ok(ordered, `pieces ${k - 1} and ${k} are in draw order`);
}


// The shore and fringe pieces (map/edges.js): counted here, straight from the neighbours, as a list of the pieces
// each rule gives. The names of the kinds are the terrain's, with terrain 0 split into sea and lake.
const { edgePieces, FRINGE, kindOf } = await import(pathToFileURL(`${here}map/edges.js`).href);
const TERRAIN_NAMES = TERRAIN.map((t) => t.name);

/** A coordinate to millionths, as an integer: a vertex reached two ways may differ in its last bit, and -0 is 0. */
const r6 = (v) => String(Math.round(v * 1e6));

/** The pieces a map must have, as strings, from its layout and neighbours; the independent count of the rules. */
const expectedPieces = ({ kinds, nb, px, py, river, flow }) => {
  const wet = (i) => kinds[i] === 'sea' || kinds[i] === 'lake';
  const mouth = (i, j) => river[i] > 0 && flow[i] === j;
  const out = [];
  const piece = (kind, key, x, y, tile, other) => out.push(`${kind}|${key}|${r6(x)}|${r6(y)}|${tile}|${other}`);
  const n = kinds.length;
  // Edges: every land tile's edge to a water tile, except a mouth; the variant from the lower-numbered tile.
  for (let i = 0; i < n; i++) {
    if (wet(i)) continue;
    for (let d = 0; d < 6; d++) {
      const j = nb[i * 6 + d];
      if (j < 0 || !wet(j) || mouth(i, j) || kinds[i] === 'marsh') continue;
      piece('edge', `shore.edge.${d}.${Math.min(i, j) % 2}`, (px[i] + px[j]) / 2, (py[i] + py[j]) / 2, i, j);
    }
  }
  // Fringes: each edge between two land kinds, the higher kind's, in the lower tile (a plains tile is the lowest).
  for (let i = 0; i < n; i++) {
    if (wet(i)) continue;
    for (let d = 0; d < 6; d++) {
      const j = nb[i * 6 + d];
      if (j < 0 || j < i || wet(j) || kinds[j] === kinds[i]) continue;
      const ia = FRINGE.indexOf(kinds[i]), ib = FRINGE.indexOf(kinds[j]);
      if (ia < 0 && ib < 0) continue;
      const iHigher = ib < 0 || (ia >= 0 && ia < ib);
      const low = iHigher ? j : i, high = iHigher ? i : j;
      const toHigh = EDGE.findIndex((_, e) => nb[low * 6 + e] === high);
      piece('fringe', `fringe.${kinds[high]}.${toHigh}.${Math.min(i, j) % 2}`, px[low], py[low], low, high);
    }
  }
  // Corners: each hex vertex, once, with the tiles that touch it: corner c of i is shared with the neighbours in
  // directions c - 1 and c. A vertex of three tiles on the map with one or two of them water.
  const seen = new Set();
  for (let i = 0; i < n; i++) {
    for (let c = 0; c < 6; c++) {
      const a = nb[i * 6 + ((c + 5) % 6)], b = nb[i * 6 + c];
      if (a < 0 || b < 0) continue;
      const x = px[i] + CORNER[c][0], y = py[i] + CORNER[c][1];
      const id = `${r6(x)},${r6(y)}`;
      if (seen.has(id)) continue;
      seen.add(id);
      const tiles = [i, a, b];
      if (tiles.every((u) => wet(u) || kinds[u] === 'marsh')) continue; // all marsh and water: the reeds run in
      const waterTiles = tiles.filter(wet);
      if (waterTiles.length === 1 || waterTiles.length === 2) {
        const t = waterTiles.length === 1 ? waterTiles[0] : tiles.find((u) => !wet(u));
        const k = CORNER.findIndex(([cx, cy]) => Math.abs(px[t] + cx - x) < 1e-9 && Math.abs(py[t] + cy - y) < 1e-9);
        assert.ok(k >= 0, `a vertex is a corner of its tile ${t}`);
        piece('corner', `shore.${waterTiles.length === 1 ? 'in' : 'out'}.${k}`, px[t] + CORNER[k][0], py[t] + CORNER[k][1], t, -1);
      }
    }
  }
  return out.sort();
};

/** The order the pieces must be in: fringes from the lowest kind to the highest, then edges, then corners; each
 *  group by y, x, then tile, then other. */
const groupOf = (p) => ({ fringe: 0, edge: 1, corner: 2 })[p.kind];
const fringeRank = (p, kinds) => FRINGE.length - FRINGE.indexOf(kinds[p.other]);
const before = (a, b, kinds) => {
  if (groupOf(a) !== groupOf(b)) return groupOf(a) < groupOf(b);
  if (a.kind === 'fringe' && fringeRank(a, kinds) !== fringeRank(b, kinds)) return fringeRank(a, kinds) < fringeRank(b, kinds);
  if (a.y !== b.y) return a.y < b.y;
  if (a.x !== b.x) return a.x < b.x;
  if (a.tile !== b.tile) return a.tile < b.tile;
  return a.other <= b.other;
};
const asKeys = (list) => list.map((p) => `${p.kind}|${p.key}|${r6(p.x)}|${r6(p.y)}|${p.tile}|${p.other}`).sort();

const kindsOf = (l, names) => Array.from({ length: l.terrain.length }, (_, i) => kindOf(i, { terrain: l.terrain, lake: l.lake, names }));
const kinds = kindsOf(layout, TERRAIN_NAMES);
const shore = edgePieces({ terrain: layout.terrain, lake: layout.lake, river: layout.river, flow: layout.flow, nb: nbOf, px: pxOf, py: pyOf, names: TERRAIN_NAMES });
assert.deepEqual(asKeys(shore), expectedPieces({ kinds, nb: nbOf, px: pxOf, py: pyOf, river: layout.river, flow: layout.flow }), 'the shore and fringe pieces are the rules\' pieces');
for (let k = 1; k < shore.length; k++) assert.ok(before(shore[k - 1], shore[k], kinds), `pieces ${k - 1} and ${k} are in order`);
const isWet = (i) => kinds[i] === 'sea' || kinds[i] === 'lake';
const mouths = [...Array(n).keys()].filter((i) => !isWet(i) && layout.river[i] > 0 && isWet(layout.flow[i]));
assert.ok(mouths.length > 0, 'the medium map has river mouths');
for (const i of mouths) assert.ok(!shore.some((p) => p.kind === 'edge' && p.tile === i && p.other === layout.flow[i]), `mouth ${i} has no edge piece`);
const counts = Object.fromEntries(['fringe', 'edge', 'corner'].map((k) => [k, shore.filter((p) => p.kind === k).length]));

// A synthetic seven-tile layout: a land tile with a river flowing east into a sea tile. The mouth has no edge piece,
// and the two corners of its edge still draw.
const ring = [[0, 0], ...EDGE];
const at7 = new Map(ring.map(([q, r], i) => [`${q},${r}`, i]));
const L7 = {
  terrain: ring.map((_, i) => (i === 1 ? 0 : TERRAIN.findIndex((t) => t.name === 'plains'))),
  lake: ring.map(() => false),
  river: ring.map((_, i) => (i === 0 ? 1 : 0)),
  flow: ring.map((_, i) => (i === 0 ? 1 : -1)),
  q: ring.map(([q]) => q),
  r: ring.map(([, r]) => r),
};
const px7 = Float64Array.from(L7.q, (q, i) => Math.sqrt(3) * (q + L7.r[i] / 2));
const py7 = Float64Array.from(L7.r, (r) => 1.5 * r);
const nb7 = new Int32Array(ring.length * 6).fill(-1);
ring.forEach(([q, r], i) => EDGE.forEach(([dq, dr], d) => { nb7[i * 6 + d] = at7.get(`${q + dq},${r + dr}`) ?? -1; }));
const names7 = TERRAIN_NAMES;
const kinds7 = kindsOf(L7, names7);
const synthetic = edgePieces({ terrain: L7.terrain, lake: L7.lake, river: L7.river, flow: L7.flow, nb: nb7, px: px7, py: py7, names: names7 });
assert.deepEqual(asKeys(synthetic), expectedPieces({ kinds: kinds7, nb: nb7, px: px7, py: py7, river: L7.river, flow: L7.flow }), 'the synthetic layout\'s pieces are the rules\'');
assert.ok(!synthetic.some((p) => p.kind === 'edge' && p.tile === 0), 'the mouth has no edge piece');
const mouthCorners = synthetic.filter((p) => p.kind === 'corner' && p.tile === 1).map((p) => p.key).sort();
assert.deepEqual(mouthCorners, ['shore.in.3', 'shore.in.4'], 'the mouth\'s two corners draw, on the sea tile');
assert.ok(synthetic.some((p) => p.kind === 'edge' && p.tile === 2), 'the land beside the sea still has its bank');

// The wood pieces (map/woods.js). The lattice, the draws and the river's course come from the module; the rule is counted
// here, slot by slot, from the neighbours and WOODS: each forest slot holds the tree the rule says (or nothing), each
// marsh slot a reed, no piece lies within WOODS.river of its tile's course, and the order is the spec's.
const { WOODS, SALT, slotDraw, slotsOf, riverSegments, woodPieces } = await import(pathToFileURL(`${here}map/woods.js`).href);
const FOREST = TERRAIN.findIndex((t) => t.name === 'forest');
const MARSH = TERRAIN.findIndex((t) => t.name === 'marsh');
assert.ok(FOREST >= 0 && MARSH >= 0, 'the terrain has forest and marsh');
const TREE_SLOTS = slotsOf(WOODS.gap.tree, WOODS.inset.tree), REED_SLOTS = slotsOf(WOODS.gap.reed, WOODS.inset.reed);
const segDist = (x, y, [x0, y0, x1, y1]) => {
  const dx = x1 - x0, dy = y1 - y0;
  const t = Math.max(0, Math.min(1, ((x - x0) * dx + (y - y0) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(x0 + t * dx - x, y0 + t * dy - y);
};
/** Each slot's expected piece, by the rule: `{key, kind}` or null for a slot that holds nothing, for one tile. */
const expectSlot = (m, i, slot, tree) => {
  const lattice = tree ? TREE_SLOTS : REED_SLOTS;
  const s = lattice[slot];
  const x = m.px[i] + s.x + (slotDraw(i, slot, SALT.jx) * 2 - 1) * WOODS.jitter;
  const y = m.py[i] + s.y + (slotDraw(i, slot, SALT.jy) * 2 - 1) * WOODS.jitter;
  if (riverSegments(i, m).some((seg) => segDist(x, y, seg) < WOODS.river)) return null;
  const lx = x - m.px[i], ly = y - m.py[i];
  if (m.fields.has(i) && Math.abs(lx) < WOODS.clearing.x && ly > WOODS.clearing.top && ly < WOODS.clearing.bottom) return null;
  const v =Math.floor(slotDraw(i, slot, SALT.variant) * 3);
  if (!tree) return { key: `reed.${v}`, kind: 'reed' };
  // Open edges, from the axial directions: a neighbour that is not forest, or off the map.
  let depth = Infinity;
  for (let d = 0; d < 6; d++) {
    const j = m.nb[i * 6 + d];
    if (j >= 0 && m.terrain[j] === FOREST) continue;
    const [dq, dr] = EDGE[d];
    const len = Math.hypot(Math.sqrt(3) * (dq + dr / 2), 1.5 * dr);
    const nx = (Math.sqrt(3) * (dq + dr / 2)) / len, ny = (1.5 * dr) / len;
    depth = Math.min(depth, Math.sqrt(3) / 2 - ((x - m.px[i]) * nx + (y - m.py[i]) * ny));
  }
  if (depth >= WOODS.band) return { key: `tree.big.${v}`, kind: 'tree' };
  return slotDraw(i, slot, SALT.sparse) < WOODS.sparse ? { key: `tree.small.${v}`, kind: 'tree' } : null;
};
/** Checks a layout's wood pieces against the rule, and returns them with counts: `big`, `small`, `reed`, `cleared` slots. */
const checkWoods = (m, label) => {
  const pieces = woodPieces({ terrain: m.terrain, river: m.river, flow: m.flow, nb: m.nb, px: m.px, py: m.py, forest: FOREST, marsh: MARSH, fields: m.fields });
  const got = new Map(pieces.map((p) => [`${p.tile}|${p.slot}`, p]));
  assert.equal(got.size, pieces.length, `${label}: one piece a slot`);
  const counts = { big: 0, small: 0, reed: 0, cleared: 0 };
  for (let i = 0; i < m.terrain.length; i++) {
    const t = m.terrain[i];
    if (t !== FOREST && t !== MARSH) continue;
    const slots = t === FOREST ? TREE_SLOTS : REED_SLOTS;
    for (let slot = 0; slot < slots.length; slot++) {
      const want = expectSlot(m, i, slot, t === FOREST);
      const have = got.get(`${i}|${slot}`);
      assert.equal(have ? have.key : null, want ? want.key : null, `${label}: tile ${i} slot ${slot} holds the rule's piece`);
      if (!want) counts.cleared++;
      else if (want.kind === 'reed') counts.reed++;
      else counts[want.key.startsWith('tree.big') ? 'big' : 'small']++;
      if (!have) continue;
      assert.equal(have.x, m.px[i] + slots[slot].x + (slotDraw(i, slot, SALT.jx) * 2 - 1) * WOODS.jitter, `${label}: tile ${i} slot ${slot}: the moved x`);
      assert.equal(have.y, m.py[i] + slots[slot].y + (slotDraw(i, slot, SALT.jy) * 2 - 1) * WOODS.jitter, `${label}: tile ${i} slot ${slot}: the moved y`);
      for (const seg of riverSegments(i, m)) assert.ok(segDist(have.x, have.y, seg) >= WOODS.river, `${label}: tile ${i} slot ${slot} is clear of the river`);
    }
  }
  for (let k = 1; k < pieces.length; k++) {
    const a = pieces[k - 1], b = pieces[k];
    const ordered = a.y < b.y || (a.y === b.y && (a.x < b.x || (a.x === b.x && (a.tile < b.tile || (a.tile === b.tile && a.slot < b.slot)))));
    assert.ok(ordered, `${label}: pieces ${k - 1} and ${k} are in draw order`);
  }
  return { pieces, counts };
};
const mapOf = (layout) => ({
  terrain: layout.terrain, river: layout.river, flow: layout.flow, nb: nbOf, px: pxOf, py: pyOf,
  fields: new Set(), // the medium world is read before any turn: no fields
});
const wood = checkWoods(mapOf(layout), 'medium');
const woodCount = (list) => list.filter((p) => p.kind === 'tree').length;
// The forest's classes on the medium world: a forest tile by its forest neighbours (six: interior; none: lone; else edge).
const forestTiles = [...Array(n).keys()].filter((i) => layout.terrain[i] === FOREST);
const forestNbs = (i) => EDGE.map((_, d) => d).filter((d) => nbOf[i * 6 + d] >= 0 && layout.terrain[nbOf[i * 6 + d]] === FOREST).length;
const classes = { interior: 0, edge: 0, lone: 0 };
for (const i of forestTiles) classes[forestNbs(i) === 6 ? 'interior' : forestNbs(i) === 0 ? 'lone' : 'edge']++;
const lacks = Object.keys(classes).filter((c) => classes[c] === 0);
if (lacks.length) console.log(`note: the medium world's forest lacks ${lacks.join(', ')} tiles`);
// Synthetic layouts on the seven-tile ring: forest with a river through its centre, with and without fields (the
// clearing), and marsh with the same river.
const ringOf = (terrainName, { river = false, fields = [] } = {}) => {
  const t = TERRAIN.findIndex((x) => x.name === terrainName);
  const L = {
    terrain: ring.map(() => t),
    river: ring.map((_, i) => (river && i === 0 ? 1 : 0)),
    flow: ring.map((_, i) => (river && i === 0 ? 1 : -1)),
    nb: nb7, px: px7, py: py7, fields: new Set(fields),
  };
  return L;
};
const withFields = checkWoods(ringOf('forest', { river: true, fields: [0] }), 'ring with fields');
const withoutFields = checkWoods(ringOf('forest', { river: true }), 'ring without fields');
const course = riverSegments(0, ringOf('forest', { river: true }));
const inClearing = (list) => list.filter((p) => p.tile === 0 && Math.abs(p.x - px7[0]) < WOODS.clearing.x && p.y - py7[0] > WOODS.clearing.top && p.y - py7[0] < WOODS.clearing.bottom);
const onCourse = (list) => list.filter((p) => p.tile === 0 && course.some((seg) => segDist(p.x, p.y, seg) < WOODS.river));
assert.equal(inClearing(withFields.pieces).length, 0, 'a tile with fields has no piece in its clearing');
assert.ok(inClearing(withoutFields.pieces).length > 0, 'the same tile without fields has pieces in its clearing');
assert.equal(onCourse(withFields.pieces).length, 0, 'no piece stands on the river course');
assert.equal(onCourse(withoutFields.pieces).length, 0, 'no piece stands on the river course without fields');
const marshRing = checkWoods(ringOf('marsh', { river: true }), 'marsh ring');
assert.ok(marshRing.pieces.every((p) => p.kind === 'reed'), 'a marsh tile holds reeds only');
assert.ok(marshRing.pieces.some((p) => p.tile === 0), 'a marsh tile holds reeds');

console.log(`ok: edges, medium seed ${seed}: ${counts.fringe} fringes, ${counts.edge} shore edges, ${counts.corner} corners, ${mouths.length} river mouths; the synthetic mouth draws its corners`);
console.log(`ok: woods, medium seed ${seed}: ${wood.counts.big} big trees, ${wood.counts.small} small, ${wood.counts.reed} reeds, ${wood.counts.cleared} clear slots; forest tiles ${classes.interior} interior, ${classes.edge} edge, ${classes.lone} lone (${woodCount(wood.pieces)} trees); the ring's clearing and course hold`);
