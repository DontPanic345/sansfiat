// Each figure's opaque pixels (alpha 128 and up) and the share of them its mask covers (the mask's alpha above 0 at
// the same pixel), and the mask's own pixels off the figure. From the repo root:  node relay/specs/443/proto/shares.mjs
import { readFileSync } from 'node:fs';
import { decode } from '../../../../engine/www/assets/png.mjs';
const bp = decode(readFileSync('engine/www/assets/atlas.png'));
const bj = JSON.parse(readFileSync('engine/www/assets/atlas.json', 'utf8'));
const px = (c, x, y) => bp.data[((c.y + y) * bp.width + c.x + x) * 4 + 3];
for (const kind of ['camp', 'scout']) {
  const fig = bj.cells[`unit.${kind}`], mask = bj.cells[`unit.${kind}.mask`];
  let opaque = 0, covered = 0, offFigure = 0, maskPx = 0, top = Infinity, bottom = -1, left = Infinity, right = -1;
  for (let y = 0; y < fig.h; y++) for (let x = 0; x < fig.w; x++) {
    const f = px(fig, x, y) >= 128, m = px(mask, x, y) > 0;
    if (f) { opaque++; if (m) covered++; top = Math.min(top, y); bottom = Math.max(bottom, y); left = Math.min(left, x); right = Math.max(right, x); }
    if (m) { maskPx++; if (!f) offFigure++; }
  }
  console.log(`${kind}: figure opaque ${opaque} in cell x ${left}-${right}, y ${top}-${bottom} (${bottom - top + 1} px tall), covered by its mask ${covered} (${((100 * covered) / opaque).toFixed(1)}%); mask pixels ${maskPx}, off the figure ${offFigure}`);
}
