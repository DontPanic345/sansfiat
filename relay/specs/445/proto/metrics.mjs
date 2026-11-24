// The place cells' numbers for atlas.md: each cell's region in atlas.png, its opaque pixels (alpha >= 128), the cell's edge
// rows and columns that hold opaque pixels (clipping), each owned cell's share of opaque pixels its mask covers
// (mask alpha >= 128), and the rim: opaque pixels with a clear pixel (alpha < 128) among their 8 neighbours, and how many
// of them are red-dominant (r > 2g and r > 2b).
// From engine/:  node ../relay/specs/445/proto/metrics.mjs
import { readFileSync } from 'node:fs';
import { decode } from '../../../../engine/www/assets/png.mjs';
const json = JSON.parse(readFileSync('www/assets/atlas.json', 'utf8'));
const png = decode(readFileSync('www/assets/atlas.png'));
const cell = (k) => {
  const c = json.cells[k];
  const a = new Uint8Array(c.w * c.h * 4);
  for (let y = 0; y < c.h; y++) for (let x = 0; x < c.w; x++) {
    const s = ((c.y + y) * png.width + c.x + x) * 4, d = (y * c.w + x) * 4;
    for (let i = 0; i < 4; i++) a[d + i] = png.data[s + i];
  }
  return { c, a };
};
const alpha = (img, x, y) => img.a[(y * img.c.w + x) * 4 + 3];
const rows = [];
for (const k of ['place.town', 'place.settlement', 'place.settlement.mask', 'place.capital', 'place.capital.mask']) {
  const img = cell(k), { c, a } = img;
  let opaque = 0, rim = 0, redRim = 0;
  const edge = { top: 0, bottom: 0, left: 0, right: 0 };
  for (let y = 0; y < c.h; y++) for (let x = 0; x < c.w; x++) {
    if (alpha(img, x, y) < 128) continue;
    opaque++;
    if (y === 0) edge.top++;
    if (y === c.h - 1) edge.bottom++;
    if (x === 0) edge.left++;
    if (x === c.w - 1) edge.right++;
    let clear = false;
    for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) {
      const nx = x + dx, ny = y + dy;
      if (nx < 0 || ny < 0 || nx >= c.w || ny >= c.h || alpha(img, nx, ny) < 128) clear = true;
    }
    if (clear) {
      rim++;
      const o = (y * c.w + x) * 4, r = a[o], g = a[o + 1], b = a[o + 2];
      if (r > 2 * g && r > 2 * b) redRim++;
    }
  }
  let cover = '';
  if (k.endsWith('.mask')) {
    // the owned cell it masks: its opaque pixels the mask covers
    const base = cell(k.replace('.mask', ''));
    let both = 0;
    for (let y = 0; y < c.h; y++) for (let x = 0; x < c.w; x++) if (alpha(img, x, y) >= 128 && alpha(base, x, y) >= 128) both++;
    cover = `${(100 * both / (base.c.w * base.c.h ? countOpaque(base) : 1)).toFixed(1)}% of the cell's opaque pixels`;
  }
  rows.push(`${k} | ${c.x},${c.y} ${c.w}x${c.h} | ${opaque} | top ${edge.top} bottom ${edge.bottom} left ${edge.left} right ${edge.right} | ${rim} (${redRim} red) | ${cover}`);
}
function countOpaque(img) {
  let n = 0;
  for (let y = 0; y < img.c.h; y++) for (let x = 0; x < img.c.w; x++) if (alpha(img, x, y) >= 128) n++;
  return n;
}
console.log('cell | rect | opaque (alpha >= 128) | edge rows/cols with opaque | rim (red) | mask covers');
console.log(rows.join('\n'));
