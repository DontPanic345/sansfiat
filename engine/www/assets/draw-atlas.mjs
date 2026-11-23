// Builds `atlas.png` and `atlas.json` from `placeholders.js` and `sources.js`:
//
//   node www/assets/draw-atlas.mjs            the placeholder sheet is drawn in headless Chrome (play.mjs),
//                                             then every CELLS entry is cut from its source over its key's rectangle
//   node www/assets/draw-atlas.mjs --check    no Chrome: recuts every CELLS entry and compares it with atlas.png,
//                                             and atlas.json with layout(); `--atlas DIR` checks another folder
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { cut, anchorOf, derive, fold } from './cut.mjs';
import { decode, encode, readPng } from './png.mjs';
import { fringeBox, fringeMask, layout, TERRAIN_CELL } from './placeholders.js';
import { CELLS, FILES, REGIONS } from './sources.js';

const here = fileURLToPath(new URL('.', import.meta.url));
// The repo root: three folders up from this one (www/assets/ sits in engine/www/).
const ROOT = new URL('../../../', import.meta.url);

const sources = new Map();
const sourceOf = (file) => {
  if (!sources.has(file)) sources.set(file, readPng(fileURLToPath(new URL(FILES[file], ROOT))));
  return sources.get(file);
};

/** The region a CELLS entry is cut from; a derived cell's is its `from` cell's. */
const srcOf = (name) => (CELLS[name].from ? CELLS[CELLS[name].from].region : CELLS[name].region);

/** The cut of CELLS entry `name` in a cell of `size` `{ w, h }`: `{ data, ax, ay, flat }`. A derived cell is
 *  its `from` cell's cut at the terrain size, masked to its fringe. */
const recut = (name, size) => {
  const spec = CELLS[name];
  if (spec.from) {
    const fromSize = { w: TERRAIN_CELL.w, h: TERRAIN_CELL.h };
    const from = CELLS[spec.from], region = REGIONS[from.region];
    return derive(cut(sourceOf(region.file), region, from, fromSize), fromSize, spec.fringe, size);
  }
  const region = REGIONS[spec.region];
  return cut(sourceOf(region.file), region, spec, size);
};

const opName = (op) => (typeof op === 'string' ? op : op.op);

const cellsOf = (sheet, cuts) => {
  const cells = {};
  for (const [name, c] of Object.entries(sheet.cells)) {
    const [ax, ay] = anchorOf(CELLS[name], c);
    if (!CELLS[name]) {
      cells[name] = { x: c.x, y: c.y, w: c.w, h: c.h, ax, ay, note: c.note };
      continue;
    }
    const spec = CELLS[name];
    const src = srcOf(name), region = REGIONS[src];
    const ops = spec.ops?.length ? ` (${spec.ops.map(opName).join(', ')})` : '';
    const how = spec.from ? `Derived from ${spec.from}, masked to fringe edge ${spec.fringe.dir}, variant ${spec.fringe.variant}` : `Cut from ${spec.region}${ops}${spec.rotate ? `, turned ${spec.rotate}°` : ''}`;
    cells[name] = {
      x: c.x, y: c.y, w: c.w, h: c.h, ax, ay,
      note: `${c.note}. ${how}: ${region.note}`,
      src,
      flat: cuts.get(name).flat,
    };
  }
  return cells;
};

/** The cut ops' own checks, which need no atlas: a turn of 0 is no turn, a fill's copies mirror across a seam, and
 *  every fringe pixel sits in its cell's rectangle. Returns the failures. */
function asserts() {
  const bad = [];
  const sheet = layout();
  // A sprite region cut with rotate 0 is the same cut as without.
  for (const [name, cell] of [['shore.edge.0.0', CELLS['shore.edge.0.0']], ['mountain.peak.0', CELLS['mountain.peak.0']]]) {
    const region = REGIONS[cell.region];
    const size = sheet.cells[name];
    const plain = cut(sourceOf(region.file), region, { ...cell, rotate: undefined }, size);
    const turned = cut(sourceOf(region.file), { ...region, rotate: 0 }, { ...cell, rotate: undefined }, size);
    if (Buffer.compare(Buffer.from(plain.data), Buffer.from(turned.data)) !== 0) bad.push(`assert: ${name} cut with rotate 0 differs from the cut without`);
  }
  // A fill's copies seam: the source column read by a sample just left of a copy boundary is the one read just right of it, mirrored.
  const fillSource = { width: 200, height: 100, data: new Uint8Array(200 * 100 * 4) };
  for (let i = 0; i < fillSource.width * fillSource.height; i++) {
    fillSource.data[i * 4] = (i % fillSource.width) % 256;
    fillSource.data[i * 4 + 3] = 255;
  }
  const seam = { file: 'terrain', fill: { rect: [0, 0, 40, 60], scale: 1, offset: [0, 0] }, note: 'the seam test' };
  const row = cut(fillSource, seam, { ops: [] }, { w: 128, h: 148 });
  const red = (x) => row.data[(74 * 128 + x) * 4];
  for (let j = 0; j < 4; j++) {
    if (red(40 - 1 - j) !== red(40 + j)) bad.push(`assert: a fill's copies do not mirror across the seam at x 40 (${red(39 - j)} and ${red(40 + j)})`);
    if (fold(40 - 1 - j, 40) !== fold(40 + j, 40)) bad.push(`assert: fold does not mirror across a copy boundary`);
  }
  // Every derived cell's rectangle holds every pixel of its mask above 0.
  for (const [name, spec] of Object.entries(CELLS)) {
    if (!spec.from) continue;
    const box = fringeBox(spec.fringe.dir), size = sheet.cells[name];
    if (box.w !== size.w || box.h !== size.h) bad.push(`assert: ${name} is ${size.w}×${size.h}, its fringe is ${box.w}×${box.h}`);
    let out = 0;
    for (let y = 0; y < TERRAIN_CELL.h; y++) {
      for (let x = 0; x < TERRAIN_CELL.w; x++) {
        if (fringeMask(spec.fringe.dir, spec.fringe.variant, x + 0.5, y + 0.5) <= 0) continue;
        if (x < box.x || y < box.y || x >= box.x + box.w || y >= box.y + box.h) out++;
      }
    }
    if (out) bad.push(`assert: ${name}: ${out} mask pixels above 0 lie outside its rectangle`);
  }
  return bad;
}

/** Checks `dir`'s atlas.png and atlas.json against layout() and the sources; prints each failure. */
function check(dir) {
  const bad = [];
  const sheet = layout();
  const image = readPng(join(dir, 'atlas.png'));
  const json = JSON.parse(readFileSync(join(dir, 'atlas.json'), 'utf8'));
  if (image.width !== sheet.width || image.height !== sheet.height) bad.push(`atlas.png is ${image.width}×${image.height}, layout() says ${sheet.width}×${sheet.height}`);
  if (json.width !== sheet.width || json.height !== sheet.height) bad.push(`atlas.json is ${json.width}×${json.height}, layout() says ${sheet.width}×${sheet.height}`);
  for (const key of Object.keys(CELLS)) if (!(key in sheet.cells)) bad.push(`CELLS has ${key}, which layout() does not make`);
  for (const name of Object.keys(json.cells)) if (!(name in sheet.cells)) bad.push(`atlas.json has ${name}, which layout() does not make`);
  let nCut = 0;
  for (const [name, c] of Object.entries(sheet.cells)) {
    const j = json.cells[name];
    if (!j) {
      bad.push(`atlas.json has no ${name}`);
      continue;
    }
    for (const f of ['x', 'y', 'w', 'h']) if (j[f] !== c[f]) bad.push(`${name}: ${f} is ${j[f]}, layout() says ${c[f]}`);
    const [ax, ay] = anchorOf(CELLS[name], c);
    if (j.ax !== ax || j.ay !== ay) bad.push(`${name}: anchor is ${j.ax},${j.ay}, expected ${ax},${ay}`);
    if (!CELLS[name]) {
      if (j.src !== undefined || j.flat !== undefined) bad.push(`${name}: has src or flat but is not cut`);
      continue;
    }
    nCut++;
    const cut = recut(name, c);
    if (j.src !== srcOf(name)) bad.push(`${name}: src is ${j.src}, expected ${srcOf(name)}`);
    if (j.flat !== cut.flat) bad.push(`${name}: flat is ${j.flat}, the cut gives ${cut.flat}`);
    if (c.x + c.w > image.width || c.y + c.h > image.height) {
      bad.push(`${name}: its rectangle lies outside atlas.png`);
      continue;
    }
    let differ = 0, first = null;
    for (let y = 0; y < c.h; y++) {
      for (let x = 0; x < c.w; x++) {
        const a = ((c.y + y) * image.width + c.x + x) * 4, b = (y * c.w + x) * 4;
        const got = [...image.data.subarray(a, a + 4)], want = [...cut.data.subarray(b, b + 4)];
        if (got.some((v, k) => v !== want[k])) {
          differ++;
          first ??= { x, y, got, want };
        }
      }
    }
    if (differ) bad.push(`${name}: ${differ} pixels differ from the cut; first at (${first.x}, ${first.y}) in the cell: atlas.png ${first.got}, cut ${first.want}`);
  }
  bad.push(...asserts());
  if (bad.length) {
    for (const line of bad) console.error(`atlas: ${line}`);
    process.exit(1);
  }
  console.log(`atlas: ${nCut} cut cells match their sources, ${Object.keys(sheet.cells).length} cells match layout()`);
}

/** Draws layout()'s sheet in headless Chrome, as RGBA8 (`{ width, height, data }`). */
async function drawPlaceholders(sheet) {
  const { open } = await import('../play.mjs');
  const page = await open('', { path: 'assets/blank.html', ready: 'true' });
  try {
    const url = await page.eval(`(async () => {
      const { draw } = await import('/assets/placeholders.js');
      const sheet = ${JSON.stringify(sheet)};
      const canvas = document.createElement('canvas');
      canvas.width = sheet.width;
      canvas.height = sheet.height;
      const ctx = canvas.getContext('2d');
      for (const [name, c] of Object.entries(sheet.cells)) {
        ctx.save();
        ctx.translate(c.x, c.y);
        ctx.beginPath();
        ctx.rect(0, 0, c.w, c.h);
        ctx.clip();
        draw(name, ctx, c.w, c.h);
        ctx.restore();
      }
      return canvas.toDataURL('image/png');
    })()`);
    for (const e of page.errors()) console.error(`page error: ${e}`);
    return decode(Buffer.from(url.split(',')[1], 'base64'));
  } finally {
    await page.close();
  }
}

async function build() {
  const sheet = layout();
  for (const key of Object.keys(CELLS)) if (!(key in sheet.cells)) throw new Error(`CELLS has ${key}, which layout() does not make`);
  const image = await drawPlaceholders(sheet);
  if (image.width !== sheet.width || image.height !== sheet.height) throw new Error(`Chrome drew ${image.width}×${image.height}, not ${sheet.width}×${sheet.height}`);
  const cuts = new Map();
  for (const name of Object.keys(CELLS)) {
    const c = sheet.cells[name];
    const cut = recut(name, c);
    cuts.set(name, cut);
    for (let y = 0; y < c.h; y++) {
      image.data.set(cut.data.subarray(y * c.w * 4, (y + 1) * c.w * 4), ((c.y + y) * image.width + c.x) * 4);
    }
  }
  writeFileSync(join(here, 'atlas.png'), encode(image));
  const json = { image: 'atlas.png', width: sheet.width, height: sheet.height, cells: cellsOf(sheet, cuts) };
  writeFileSync(join(here, 'atlas.json'), `${JSON.stringify(json, null, 1)}\n`);
  console.log(`${Object.keys(sheet.cells).length} cells, ${sheet.width}×${sheet.height}, ${cuts.size} cut`);
}

const args = process.argv.slice(2);
if (args.includes('--check')) {
  const at = args.indexOf('--atlas');
  check(at >= 0 ? args[at + 1] : here);
} else {
  await build();
}
