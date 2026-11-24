// Crops main's and the branch's shots of the named views round their units, to relay/specs/443/shots/<view>-{main,branch}.png:
// the same rectangle in both (the branch's units and numbers, padded), zoomed Z× when it is small.
// node relay/specs/443/proto/crops.mjs MAINDIR BRANCHDIR OUTDIR view[:id+id],view,…   (a `:id+id` keeps only those units in the rectangle)
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const [mainDir, branchDir, outDir, names] = process.argv.slice(2);
mkdirSync(outDir, { recursive: true });
const views = JSON.parse(readFileSync(join(branchDir, 'views.json'), 'utf8'));
const PAD = 50;
for (const spec of names.split(',')) {
  const [name, ids] = spec.split(':');
  const keep = ids ? new Set(ids.split('+').map(Number)) : null;
  const st = { units: views[name].stats.units.filter((u) => !keep || keep.has(u.unit)), labels: views[name].stats.labels.filter((l) => !keep || keep.has(l.unit)) };
  const dpr = 1;
  const rects = [...st.units.map((u) => [u.x, u.y, u.x + u.w, u.y + u.h]), ...st.labels.map((l) => [l.box.x, l.box.y, l.box.x + l.box.w, l.box.y + l.box.h])];
  const a = decode(readFileSync(join(branchDir, `${name}.png`)));
  let x0 = Math.floor(Math.min(...rects.map((r) => r[0]))) - PAD, y0 = Math.floor(Math.min(...rects.map((r) => r[1]))) - PAD;
  let x1 = Math.ceil(Math.max(...rects.map((r) => r[2]))) + PAD, y1 = Math.ceil(Math.max(...rects.map((r) => r[3]))) + PAD;
  x0 = Math.max(0, x0); y0 = Math.max(0, y0); x1 = Math.min(a.width, x1); y1 = Math.min(a.height, y1);
  const w = x1 - x0, h = y1 - y0, Z = w <= 330 ? 3 : w <= 650 ? 2 : 1;
  for (const [tag, dir] of [['main', mainDir], ['branch', branchDir]]) {
    const src = decode(readFileSync(join(dir, `${name}.png`)));
    const W = w * Z, H = h * Z, d = new Uint8Array(W * H * 4);
    for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
      const s = ((y0 + Math.floor(y / Z)) * src.width + x0 + Math.floor(x / Z)) * 4, o = (y * W + x) * 4;
      for (let c = 0; c < 4; c++) d[o + c] = src.data[s + c];
    }
    writeFileSync(join(outDir, `${name}-${tag}.png`), encode({ width: W, height: H, data: Buffer.from(d) }));
  }
  console.log(name, `${x0},${y0} ${w}x${h} at ${Z}x`);
}
