// The place lineup (PLACES), staged on main: radius 8, sea on the rim, plains with #443's forest block on the east and a
// ridge of three mountains north of it. Red's capital at A with three more red clans on it (a capital that reads as a
// town); a blue signpost at S, beside the wood and under the ridge, with three blue clans living on it, which the first
// turn founds as a settlement (a settlement that reads as a town); four clans of no civ on T, which the drawn model
// names a town (the stage makes no market of no civ); red's (the led civ's) army and scout on c, which no AI moves, beside the settlement. Ends one turn,
// then prints the places, the clans on each and the digest, and writes the canvas at k 35 and 18.
// From engine/:  node ../relay/specs/445/proto/lineup.mjs OUTDIR
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { open } from '../../../../engine/www/play.mjs';
import { grid, marks, stage } from '../../../../engine/www/play-stage.mjs';
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const forest = (q, r) => q >= 3 && q <= 5 && r >= -2 && r <= 1;
const ridge = (q, r) => r === -2 && q >= 0 && q <= 2;
const M = { A: [-4, 1], B: [0, -6], C: [-1, 6], S: [2, -1], c: [1, -1], T: [-2, 4], a: [-4, 2], s: [2, 0], t: [-1, 4] };
export const PLACES = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : ridge(q, r) ? '^' : forest(q, r) ? 'f' : '.')),
  marks: marks(8, M),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [{ civ: 0, at: { mark: 'c' }, people: 4 }],
  scouts: [{ civ: 0, at: { mark: 'c' }, pack: 0 }],
  clans: [
    { home: { mark: 'A' }, tiles: [{ mark: 'a' }], people: 6, civ: 0, market: 0 },
    { home: { mark: 'A' }, tiles: [], people: 5, civ: 0, market: 0 },
    { home: { mark: 'A' }, tiles: [], people: 4, civ: 0, market: 0 },
    { home: { mark: 'S' }, tiles: [{ mark: 'S' }, { mark: 's' }], people: 6, civ: 1 },
    { home: { mark: 'S' }, tiles: [], people: 5, civ: 1 },
    { home: { mark: 'S' }, tiles: [], people: 4, civ: 1 },
    { home: { mark: 'T' }, tiles: [{ mark: 'T' }, { mark: 't' }], people: 6 },
    { home: { mark: 'T' }, tiles: [], people: 5 },
    { home: { mark: 'T' }, tiles: [], people: 4 },
    { home: { mark: 'T' }, tiles: [], people: 4 },
  ],
  signposts: [{ civ: 1, at: { mark: 'S' } }],
  food: ['A', 'a', 'S', 's', 'T', 't'].map((m) => ({ at: { mark: m }, good: 1, amount: 40 })),
};
const page = await open('', { size: 'laptop' });
try {
  await stage(page, PLACES);
  const before = (await page.data()).turn; await page.key('Space'); await page.until(`document.body.dataset.turn !== '${before}'`, 30000);
  await page.until('!game.map.anim && !game.map.frame', 10000);
  const info = await page.eval(`(() => { const v = game.view(), m = game.map; const homes = {}; for (const c of Object.values(v.clans)) (homes[c.home] ??= []).push(c.id + (c.civ != null ? '/' + c.civ : ''));
    return { digest: game.digest(), turn: v.turn, marks: Object.fromEntries(Object.entries(${JSON.stringify(M)}).map(([k, [q, r]]) => [k, game.map.at(q, r)])),
      capitals: Object.values(v.civs).map((c) => c.capital), settlements: v.settlements.map((s) => ({ tile: s.tile, civ: s.civ, market: s.market })), signposts: v.signposts.length,
      towns: v.towns, units: Object.values(v.units).map((u) => u.kind + '@' + u.tile), homes }; })()`);
  console.log(JSON.stringify(info));
  for (const k of [35, 18]) {
    const png = await page.eval(`(() => { const m = game.map, ts = [${info.capitals[0]}, ${info.settlements[0]?.tile ?? info.capitals[0]}]; m.cx = (m.px[ts[0]] + m.px[ts[1]]) / 2; m.cy = (m.py[ts[0]] + m.py[ts[1]]) / 2; m.k = ${k}; m.viewChanged(); m.drawNow(); return { cam: [m.cx, m.cy, m.k], png: m.canvas.toDataURL('image/png') }; })()`);
    writeFileSync(join(out, `lineup-k${k}.png`), Buffer.from(png.png.split(',')[1], 'base64')); console.log('k', k, JSON.stringify(png.cam));
  }
} finally { await page.close(); }
