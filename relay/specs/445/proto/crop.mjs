// A crop of the actor study at a scale, on a mid-grey ground so alpha and halo show: x y w h scale out.png
// From the repo root:  node relay/specs/444/proto/crop.mjs 75 350 380 200 2 /tmp/tent.png
import { readPng, writePng } from '../../../../engine/www/assets/png.mjs';
const [x0, y0, w, h, s] = process.argv.slice(2, 7).map(Number); const out = process.argv[7];
const img = readPng('docs/proposals/map-art/actors.png'); const W = img.width;
const data = Buffer.alloc(w * s * h * s * 4);
for (let y = 0; y < h * s; y++) for (let x = 0; x < w * s; x++) {
  const i = ((y0 + Math.floor(y / s)) * W + x0 + Math.floor(x / s)) * 4, o = (y * w * s + x) * 4, a = img.data[i + 3] / 255;
  for (let c = 0; c < 3; c++) data[o + c] = Math.round(img.data[i + c] * a + 128 * (1 - a)); data[o + 3] = 255; }
writePng(out, { width: w * s, height: h * s, data });
