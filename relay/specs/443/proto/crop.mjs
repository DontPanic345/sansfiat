// Crops a rectangle of a study at Z× to a PNG, with a 10 px grid labelled in source px (for reading coordinates).
// From the repo root:  node relay/specs/443/proto/crop.mjs FILE X Y W H Z OUT.png
import { readFileSync, writeFileSync } from 'node:fs';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const [file, x0, y0, w, h, Z] = [process.argv[2], ...process.argv.slice(3, 8).map(Number)];
const out = process.argv[8];
const src = decode(readFileSync(file));
const W = w * Z, H = h * Z;
const d = new Uint8Array(W * H * 4);
for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
  const sx = x0 + Math.floor(x / Z), sy = y0 + Math.floor(y / Z);
  const s = (sy * src.width + sx) * 4, o = (y * W + x) * 4, a = src.data[s + 3] / 255;
  const grid = (sx % 10 === 0 && x % Z === 0) || (sy % 10 === 0 && y % Z === 0);
  const bg = ((Math.floor(x / Z) + Math.floor(y / Z)) % 2) ? 90 : 60; // checker behind the transparent
  for (let c = 0; c < 3; c++) d[o + c] = grid ? 255 : Math.round(src.data[s + c] * a + bg * (1 - a));
  d[o + 3] = 255;
}
writeFileSync(out, encode({ width: W, height: H, data: Buffer.from(d) }));
console.log('wrote', out, W, H);
