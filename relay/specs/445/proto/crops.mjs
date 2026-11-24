// Crops main's and the branch's shots of the named views round their clans, to <OUT>/<view>-{main,branch}.png:
// the same rectangle in both (the branch's clans, padded), zoomed Z× when it is small.
// node relay/specs/445/proto/crops.mjs MAINDIR BRANCHDIR OUTDIR view[:x0,y0,x1,y1][:Z],…   (a rectangle in canvas px replaces the clans' own)
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const [mainDir, branchDir, outDir, names] = process.argv.slice(2);
mkdirSync(outDir, { recursive: true });
const views = (() => { try { return JSON.parse(readFileSync(join(branchDir, 'views.json'), 'utf8')); } catch { return null; } })();
const PAD = 40;
for (const spec of names.split(',')) {
  const [name, box, zoom] = spec.split(':');
  const a = decode(readFileSync(join(branchDir, `${name}.png`)));
  const rs = box && box !== '-' ? [] : views[name].stats.places;
  let [x0, y0, x1, y1] = box && box !== '-' ? box.split('.').map(Number) : [
    Math.floor(Math.min(...rs.map((r) => r.x))) - PAD, Math.floor(Math.min(...rs.map((r) => r.y))) - PAD,
    Math.ceil(Math.max(...rs.map((r) => r.x + r.w))) + PAD, Math.ceil(Math.max(...rs.map((r) => r.y + r.h))) + PAD];
  x0 = Math.max(0, x0); y0 = Math.max(0, y0); x1 = Math.min(a.width, x1); y1 = Math.min(a.height, y1);
  const w = x1 - x0, h = y1 - y0, Z = zoom ? +zoom : w <= 330 ? 3 : w <= 650 ? 2 : 1;
  for (const [tag, dir] of [['main', mainDir], ['branch', branchDir]]) {
    const src = decode(readFileSync(join(dir, `${name}.png`)));
    const W = w * Z, H = h * Z, d = new Uint8Array(W * H * 4);
    for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
      const s = ((y0 + Math.floor(y / Z)) * src.width + x0 + Math.floor(x / Z)) * 4, o = (y * W + x) * 4;
      for (let c = 0; c < 4; c++) d[o + c] = src.data[s + c];
    }
    writeFileSync(join(outDir, `${name}-${tag}.png`), encode({ width: W, height: H, data: Buffer.from(d) }));
  }
  console.log(name, `${x0}.${y0}.${x1}.${y1} ${w}x${h} at ${Z}x`);
}
