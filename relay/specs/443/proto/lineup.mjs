// The unit lineup on a built map: radius 8, sea on the rim, plains inside with a forest block on the east. Three civs
// (red led, blue, gold) and each one's army and scout: red's army on plains and its scout in the wood, blue's army in
// the wood and its scout on plains, gold's army and scout stacked on one plains tile. So the led civ's and a rival's
// units each stand on plains and in forest, and one tile holds a stack. Shoots the map at k 35 and k 18 (laptop) and
// k 6 (phone) with the camera on (2, 0), and prints the units the view carries and the digest.
// From engine/:  node ../relay/specs/443/proto/lineup.mjs OUTDIR
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { open } from '../../../../engine/www/play.mjs';
import { grid, marks } from '../../../../engine/www/play-stage.mjs';
const R = 8;
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const forest = (q, r) => q >= 3 && q <= 5 && r >= -2 && r <= 1;
export const LINEUP = {
  map: grid(R, (q, r) => (dist(q, r) === R ? '~' : forest(q, r) ? 'f' : '.')),
  marks: marks(R, { A: [-5, 0], B: [0, -5], C: [0, 5], a: [1, 0], b: [4, -1], c: [1, 1], s: [4, 0], t: [2, -1] }),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [
    { civ: 0, at: { mark: 'a' }, people: 6 }, // red on plains
    { civ: 1, at: { mark: 'b' }, people: 6 }, // blue in the wood
    { civ: 2, at: { mark: 'c' }, people: 4 }, // gold on plains, stacked with its scout
  ],
  scouts: [
    { civ: 0, at: { mark: 's' }, pack: 0 }, // red in the wood
    { civ: 1, at: { mark: 't' }, pack: 0 }, // blue on plains
    { civ: 2, at: { mark: 'c' }, pack: 0 }, // gold on its army's tile
  ],
};
if (process.argv[1]?.endsWith('lineup.mjs')) {
  const out = process.argv[2] ?? '/tmp/lineup';
  mkdirSync(out, { recursive: true });
  for (const [win, k] of [['laptop', 35], ['laptop', 18], ['phone', 6]]) {
    const page = await open('', { size: win });
    await page.eval(`game.stage(${JSON.stringify(LINEUP)})`);
    await page.until("document.body.dataset.phase === 'main'");
    await page.until('!game.map.anim && !game.map.frame', 10000);
    const info = await page.eval(`(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
      return Object.values(v.units).map((u) => [u.id, v.civs[u.civ].color, u.civ === v.player ? 'mine' : 'rival', u.kind, u.tile, names[l.terrain[u.tile]], u.people].join(' ')); })()`);
    const centre = await page.eval(`(() => { const v = game.view(); const u = Object.values(v.units).find((u) => u.kind === 'camp' && u.civ === v.player); return u.tile; })()`);
    await page.eval(`(() => { const m = game.map; m.cx = m.px[${centre}] + 1.7; m.cy = m.py[${centre}]; m.k = ${k}; m.viewChanged(); })()`);
    await page.until('!game.map.anim && !game.map.frame', 5000);
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `lineup-${win}-k${k}.png`), Buffer.from(png.split(',')[1], 'base64'));
    console.log(win, k, JSON.stringify(info), 'digest', await page.eval('game.digest()'), page.errors().join('; '));
    await page.close();
  }
}
