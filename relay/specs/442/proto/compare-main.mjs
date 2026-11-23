// Every key main's atlas.json has keeps main's x, y, w, h and main's pixels in the branch's atlas.png.
// From the repo root:  node relay/specs/442/proto/compare-main.mjs MAIN_DIR  (MAIN_DIR holds main's atlas.json and atlas.png)
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { decode } from '../../../../engine/www/assets/png.mjs';
const main = process.argv[2];
const mj = JSON.parse(readFileSync(join(main, 'atlas.json'), 'utf8'));
const bj = JSON.parse(readFileSync('engine/www/assets/atlas.json', 'utf8'));
const mp = decode(readFileSync(join(main, 'atlas.png')));
const bp = decode(readFileSync('engine/www/assets/atlas.png'));
let bad = 0, keys = 0, px = 0;
for (const [k, m] of Object.entries(mj.cells)) {
  keys++;
  const b = bj.cells[k];
  if (!b) { console.log('missing', k); bad++; continue; }
  for (const f of ['x', 'y', 'w', 'h']) if (b[f] !== m[f]) { console.log(k, f, m[f], '->', b[f]); bad++; }
  for (let y = 0; y < m.h; y++) for (let x = 0; x < m.w; x++) {
    const a = ((m.y + y) * mp.width + m.x + x) * 4, c = ((b.y + y) * bp.width + b.x + x) * 4;
    for (let i = 0; i < 4; i++) if (mp.data[a + i] !== bp.data[c + i]) { px++; break; }
  }
}
console.log(`main keys ${keys}, mismatched rectangles ${bad}, mismatched pixels ${px}, sheet ${bp.width}x${bp.height} (main ${mp.width}x${mp.height})`);
const added = Object.keys(bj.cells).filter((k) => !(k in mj.cells));
console.log(`new keys ${added.length}: ${added.join(' ')}`);
process.exit(bad || px ? 1 : 0);
