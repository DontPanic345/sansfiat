// Crops a rectangle of a shot PNG, nearest-neighbour at Z×. node relay/specs/443/proto/shotcrop.mjs IN.png X Y W H Z OUT.png
import { readFileSync, writeFileSync } from 'node:fs';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const [file, x0, y0, w, h, Z] = [process.argv[2], ...process.argv.slice(3, 8).map(Number)];
const src = decode(readFileSync(file));
const W = w * Z, H = h * Z;
const d = new Uint8Array(W * H * 4);
for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
  const s = ((y0 + Math.floor(y / Z)) * src.width + x0 + Math.floor(x / Z)) * 4, o = (y * W + x) * 4;
  for (let c = 0; c < 4; c++) d[o + c] = src.data[s + c];
}
writeFileSync(process.argv[8], encode({ width: W, height: H, data: Buffer.from(d) }));
