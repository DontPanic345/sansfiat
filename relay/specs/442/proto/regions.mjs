// Each new sprite region at 3× on mid grey: its rectangle, its keep-polygon outlined in magenta, its ground point in cyan.
// From the repo root:  node relay/specs/442/proto/regions.mjs OUT.png
import { readFileSync, writeFileSync } from 'node:fs';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
import { REGIONS } from '../../../../engine/www/assets/sources.js';
const out = process.argv[2];
const src = decode(readFileSync('docs/proposals/map-art/terrain.png'));
const NAMES = ['tree.big.0', 'tree.big.1', 'tree.big.2', 'tree.small.0', 'tree.small.1', 'tree.small.2', 'reed.0', 'reed.1'];
const Z = 3, PAD = 12;
const panels = NAMES.map((n) => {
  const r = REGIONS[n];
  const [x0, y0, w, h] = r.rect;
  return { n, r, x0, y0, w, h, W: w * Z, H: h * Z };
});
const inside = (x, y, p) => { let c = false; for (let i = 0, j = p.length - 1; i < p.length; j = i++) { const [xi, yi] = p[i], [xj, yj] = p[j]; if ((yi > y) !== (yj > y) && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) c = !c; } return c; };
const totalW = panels.reduce((s, p) => s + p.W + PAD, PAD), totalH = Math.max(...panels.map((p) => p.H)) + 2 * PAD + 14;
const d = new Uint8Array(totalW * totalH * 4);
for (let i = 0; i < totalW * totalH; i++) { d[i * 4] = 128; d[i * 4 + 1] = 128; d[i * 4 + 2] = 128; d[i * 4 + 3] = 255; }
const put = (x, y, rgb) => { if (x < 0 || y < 0 || x >= totalW || y >= totalH) return; const o = (y * totalW + x) * 4; d[o] = rgb[0]; d[o + 1] = rgb[1]; d[o + 2] = rgb[2]; };
let ox = PAD;
for (const p of panels) {
  const oy = PAD;
  for (let y = 0; y < p.H; y++) for (let x = 0; x < p.W; x++) {
    const sx = p.x0 + Math.floor(x / Z), sy = p.y0 + Math.floor(y / Z);
    const s = (sy * src.width + sx) * 4, a = src.data[s + 3] / 255;
    put(ox + x, oy + y, [src.data[s] * a + 128 * (1 - a), src.data[s + 1] * a + 128 * (1 - a), src.data[s + 2] * a + 128 * (1 - a)]);
  }
  // the keep polygon: a source pixel inside it whose neighbour to the right or below is outside
  const keep = p.r.keep;
  for (let y = 0; y < p.H; y++) for (let x = 0; x < p.W; x++) {
    const sx = p.x0 + Math.floor(x / Z) + 0.5, sy = p.y0 + Math.floor(y / Z) + 0.5;
    const here = inside(sx, sy, keep);
    if (here !== inside(sx + 1, sy, keep) || here !== inside(sx, sy + 1, keep)) {
      if (here) put(ox + x, oy + y, [255, 0, 255]);
    }
  }
  // the rectangle, and the ground point
  for (let x = 0; x < p.W; x++) { put(ox + x, oy, [255, 255, 255]); put(ox + x, oy + p.H - 1, [255, 255, 255]); }
  for (let y = 0; y < p.H; y++) { put(ox, oy + y, [255, 255, 255]); put(ox + p.W - 1, oy + y, [255, 255, 255]); }
  const gx = Math.round((p.r.ground[0] - p.x0) * Z), gy = Math.round((p.r.ground[1] - p.y0) * Z);
  for (let dy = -4; dy <= 4; dy++) for (let dx = -4; dx <= 4; dx++) if (dx * dx + dy * dy <= 16) put(ox + gx + dx, oy + gy + dy, [0, 255, 255]);
  ox += p.W + PAD;
}
writeFileSync(out, encode({ width: totalW, height: totalH, data: Buffer.from(d) }));
console.log('wrote', out, totalW, totalH, panels.map((p) => `${p.n} ${p.w}x${p.h}`).join(', '));
