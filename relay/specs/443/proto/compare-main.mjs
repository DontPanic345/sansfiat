// Every key main's atlas.json has keeps main's pixels in the branch's atlas.png, compared by key: each key's own
// rectangle in each atlas, so a row that moved down still compares. Rectangles are reported apart from pixels.
// From the repo root:  node relay/specs/443/proto/compare-main.mjs MAIN_DIR  (MAIN_DIR holds main's atlas.json and atlas.png)
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { decode } from '../../../../engine/www/assets/png.mjs';
const UNITS = new Set(['unit.camp', 'unit.camp.mask', 'unit.scout', 'unit.scout.mask']);
const main = process.argv[2];
const mj = JSON.parse(readFileSync(join(main, 'atlas.json'), 'utf8'));
const bj = JSON.parse(readFileSync('engine/www/assets/atlas.json', 'utf8'));
const mp = decode(readFileSync(join(main, 'atlas.png')));
const bp = decode(readFileSync('engine/www/assets/atlas.png'));
let keys = 0, sizeChanged = [], moved = 0, pixelKeys = [], px = 0;
for (const [k, m] of Object.entries(mj.cells)) {
  keys++;
  const b = bj.cells[k];
  if (!b) { pixelKeys.push(`${k} missing`); continue; }
  if (b.w !== m.w || b.h !== m.h) sizeChanged.push(`${k} ${m.w}x${m.h} -> ${b.w}x${b.h}`);
  if (b.x !== m.x || b.y !== m.y) moved++;
  if (UNITS.has(k)) continue; // the four unit cells are the piece's: their pixels change
  if (b.w !== m.w || b.h !== m.h) { pixelKeys.push(`${k} size differs`); continue; }
  let differ = 0;
  for (let y = 0; y < m.h; y++) for (let x = 0; x < m.w; x++) {
    const a = ((m.y + y) * mp.width + m.x + x) * 4, c = ((b.y + y) * bp.width + b.x + x) * 4;
    for (let i = 0; i < 4; i++) if (mp.data[a + i] !== bp.data[c + i]) { differ++; break; }
  }
  if (differ) { pixelKeys.push(`${k} ${differ} px`); px += differ; }
}
console.log(`main keys ${keys}, keys whose pixels differ ${pixelKeys.length}${pixelKeys.length ? ': ' + pixelKeys.join(', ') : ''}, sheet ${bp.width}x${bp.height} (main ${mp.width}x${mp.height})`);
console.log(`rectangles moved ${moved} (the rows below the unit row), sizes changed: ${sizeChanged.join('; ')}`);
const added = Object.keys(bj.cells).filter((k) => !(k in mj.cells));
console.log(`new keys ${added.length}: ${added.join(' ')}`);
const unitPx = [...UNITS].map((k) => {
  const b = bj.cells[k];
  let n = 0;
  for (let y = 0; y < b.h; y++) for (let x = 0; x < b.w; x++) if (bp.data[((b.y + y) * bp.width + b.x + x) * 4 + 3] > 0) n++;
  return `${k} ${b.w}x${b.h} anchor ${b.ax},${b.ay} opaque ${n}`;
});
console.log(unitPx.join('\n'));
process.exit(pixelKeys.length ? 1 : 0);
