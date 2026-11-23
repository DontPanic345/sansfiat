// Crops a rectangle of the source terrain.png at a scale, with a grid every 10 source px, to a PNG.
// From the repo root:  node relay/specs/442/proto/crop.mjs x y w h scale out.png
import { readFileSync, writeFileSync } from 'node:fs';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const [x0, y0, w, h, Z, out] = process.argv.slice(2);
const src = decode(readFileSync('docs/proposals/map-art/terrain.png'));
const X = +x0, Y = +y0, W = +w, H = +h, S = +Z;
const d = new Uint8Array(W * S * H * S * 4);
for (let y = 0; y < H * S; y++) for (let x = 0; x < W * S; x++) {
  const sx = X + Math.floor(x / S), sy = Y + Math.floor(y / S);
  const o = (y * W * S + x) * 4;
  const grid = (sx % 50 === 0 || sy % 50 === 0) && (x % S === 0 || y % S === 0);
  if (grid) { d[o] = 255; d[o + 1] = 0; d[o + 2] = 255; d[o + 3] = 255; continue; }
  const s = (sy * src.width + sx) * 4;
  // transparent source pixels show on a mid grey
  const a = src.data[s + 3] / 255;
  d[o] = Math.round(src.data[s] * a + 128 * (1 - a));
  d[o + 1] = Math.round(src.data[s + 1] * a + 128 * (1 - a));
  d[o + 2] = Math.round(src.data[s + 2] * a + 128 * (1 - a));
  d[o + 3] = 255;
}
writeFileSync(out, encode({ width: W * S, height: H * S, data: Buffer.from(d) }));
console.log('wrote', out, W * S, H * S);
