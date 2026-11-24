// What the floor removes from each clan cell: the alpha it clears and the red-dominant opaque pixels with and without it.
// From the repo root:  node relay/specs/444/proto/floor.mjs
import { readPng } from '../../../../engine/www/assets/png.mjs';
import { cut } from '../../../../engine/www/assets/cut.mjs';
import { layout } from '../../../../engine/www/assets/placeholders.js';
import { CELLS, REGIONS } from '../../../../engine/www/assets/sources.js';
const source = readPng('docs/proposals/map-art/actors.png');
const sheet = layout();
for (const k of ['clan.tent', 'clan.hearth', 'clan.walk']) {
  const c = sheet.cells[k], spec = CELLS[k], region = REGIONS[spec.region];
  const size = { w: c.w, h: c.h, ax: c.ax, ay: c.ay };
  const f = cut(source, region, spec, size), n = cut(source, { ...region, floor: undefined }, spec, size);
  let changed = 0, redF = 0, redN = 0, alphaLoss = 0;
  for (let o = 0; o < f.data.length; o += 4) {
    if (f.data[o + 3] !== n.data[o + 3]) changed++;
    const red = (d) => d[o] > 2 * d[o + 1] && d[o] > 2 * d[o + 2];
    if (f.data[o + 3] >= 128 && red(f.data)) redF++;
    if (n.data[o + 3] >= 128 && red(n.data)) redN++;
    alphaLoss += n.data[o + 3] - f.data[o + 3];
  }
  console.log(k, { alphaChangedPx: changed, redOpaqueFloor: redF, redOpaqueNoFloor: redN, totalAlphaLost: alphaLoss });
}
