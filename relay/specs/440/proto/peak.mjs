// A peak cut from terrain.png's range: a source rectangle, a keep-polygon that drops the neighbour peak, a
// ground key that drops the grass and the yellow tufts, area-resampled at the plains' scale (74/109 cell px per source px) into a
// 192×128 cell with the ground point at (96, 112). Run from the repo root: node relay/specs/440/proto/peak.mjs
import { readPng, encode } from '../../../../engine/www/assets/png.mjs';
import { writeFileSync } from 'node:fs';
const here = new URL('.', import.meta.url).pathname;
const s = readPng(new URL('../../../../docs/proposals/map-art/terrain.png', import.meta.url).pathname);
const PEAK = { rect: [110, 140, 280, 180], keep: [[110, 140], [300, 140], [300, 150], [385, 258], [372, 300], [110, 312]], ground: [262, 292] };
const W = 192, H = 128, AX = 96, AY = 112, SCALE = 74 / 109, SS = 4;
const inside = (x, y, p) => { let c = false; for (let i = 0, j = p.length - 1; i < p.length; j = i++) { const [xi, yi] = p[i], [xj, yj] = p[j]; if ((yi > y) !== (yj > y) && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) c = !c; } return c; };
const grass = (r, g, b) => (g > r + 6 && g > b + 25) || b < g * 0.6; // green grass, or yellow tufts and hill
const cell = Buffer.alloc(W * H * 4);
for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
  let r = 0, g = 0, b = 0, a = 0;
  for (let j = 0; j < SS; j++) for (let i = 0; i < SS; i++) {
    const sx = Math.floor(PEAK.ground[0] + (x + (i + 0.5) / SS - AX) / SCALE), sy = Math.floor(PEAK.ground[1] + (y + (j + 0.5) / SS - AY) / SCALE);
    if (!inside(sx + 0.5, sy + 0.5, PEAK.keep)) continue;
    const o = (sy * s.width + sx) * 4, [R, G, B, A] = s.data.subarray(o, o + 4);
    if (grass(R, G, B)) continue;
    r += R * A; g += G * A; b += B * A; a += A;
  }
  const o = (y * W + x) * 4;
  if (a) { cell[o] = Math.round(r / a); cell[o + 1] = Math.round(g / a); cell[o + 2] = Math.round(b / a); cell[o + 3] = Math.round(a / (SS * SS)); }
}
writeFileSync(`${here}peak.png`, encode({ width: W, height: H, data: cell }));
// At 3× over the cut plains olive, with the cell's hex outline below the ground point, to read by eye.
const Z = 3, bg = [123, 136, 45], big = Buffer.alloc(W * Z * H * Z * 4);
for (let y = 0; y < H * Z; y++) for (let x = 0; x < W * Z; x++) {
  const o = (Math.floor(y / Z) * W + Math.floor(x / Z)) * 4, al = cell[o + 3] / 255, d = (y * W * Z + x) * 4;
  for (let c = 0; c < 3; c++) big[d + c] = Math.round(cell[o + c] * al + bg[c] * (1 - al));
  big[d + 3] = 255;
  if (Math.abs(x / Z - AX) < 1 && Math.abs(y / Z - AY) < 1) big.set([255, 0, 0, 255], d);
}
writeFileSync(`${here}peak3x.png`, encode({ width: W * Z, height: H * Z, data: big }));
