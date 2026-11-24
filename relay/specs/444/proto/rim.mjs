// Per clan cell: its rim (opaque pixels, alpha >= 128, within 1 px of a clear one, alpha < 128), with and without the floor,
// and each cell's mask share (opaque pixels its mask covers). From engine/:  node ../relay/specs/444/proto/rim.mjs
import { readPng } from '../../../../engine/www/assets/png.mjs';
import { cut } from '../../../../engine/www/assets/cut.mjs';
import { layout } from '../../../../engine/www/assets/placeholders.js';
import { CELLS, FILES, REGIONS } from '../../../../engine/www/assets/sources.js';
const source = readPng(`../${FILES.actors}`);
const sheet = layout();
const rim = (d, w, h) => {
  let opaque = 0, near = 0, red = 0;
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
    const o = (y * w + x) * 4;
    if (d[o + 3] < 128) continue;
    opaque++;
    let clear = false;
    for (let dy = -1; dy <= 1 && !clear; dy++) for (let dx = -1; dx <= 1; dx++) {
      const X = x + dx, Y = y + dy;
      if (X < 0 || Y < 0 || X >= w || Y >= h || (dx === 0 && dy === 0)) continue;
      if (d[(Y * w + X) * 4 + 3] < 128) { clear = true; break; }
    }
    if (clear) { near++; const r = d[o], g = d[o + 1], b = d[o + 2]; if (r > 2 * g && r > 2 * b) red++; }
  }
  return { opaque, near, red };
};
const rows = [];
for (const k of ['clan.tent', 'clan.hearth', 'clan.walk']) {
  const c = sheet.cells[k], spec = CELLS[k], region = REGIONS[spec.region];
  const size = { w: c.w, h: c.h, ax: c.ax, ay: c.ay };
  const withFloor = cut(source, region, spec, size);
  const noFloor = cut(source, { ...region, floor: undefined }, spec, size);
  const a = rim(withFloor.data, c.w, c.h), b = rim(noFloor.data, c.w, c.h);
  const ms = CELLS[`${k}.mask`]; const m = cut(source, region, { ...ms, cloth: ms.cloth[ms.region] }, size);
  let cover = 0;
  for (let o = 0; o < withFloor.data.length; o += 4) if (withFloor.data[o + 3] >= 128 && m.data[o + 3] > 0) cover++;
  rows.push(`| ${k} | ${a.opaque} | ${a.near} (${a.red} red) | ${b.opaque} | ${b.near} (${b.red} red) | ${(100 * cover / a.opaque).toFixed(1)}% |`);
}
console.log('| cell | opaque, floor 128 | rim, floor 128 | opaque, no floor | rim, no floor | mask covers, floor 128 |');
console.log('| --- | --- | --- | --- | --- | --- |');
console.log(rows.join('\n'));
