// The clan lineup (CLANS), staged on main: radius 8, sea on the rim, plains with a forest block on the east, the three
// civs of the unit lineup. Clans of red, blue, gold and of no civ, each on plains (row r 2) and in the wood (row r -1,
// q 3 to 5); a shared home of four (tile mark h: red settled, red gathering, red moving, blue gathering); a gathering
// clan of no civ on gold's army's tile (mark c) beside the units. The states the stage can't set (settled, since,
// starving, leaning) are set on the drawn model, as play/clans.mjs will. Prints each clan, its tile and the digest,
// and writes the canvas at k 35 and 18 before and after the states are set.
// From engine/:  node ../relay/specs/444/proto/lineup.mjs OUTDIR   (the views put the camera between tiles 93 and 170)
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { open } from '../../../../engine/www/play.mjs';
import { grid, marks } from '../../../../engine/www/play-stage.mjs';
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const forest8 = (q, r) => q >= 3 && q <= 5 && r >= -2 && r <= 1;
export const CLANS = {
  map: grid(8, (q, r) => (dist(q, r) === 8 ? '~' : forest8(q, r) ? 'f' : '.')),
  marks: marks(8, { A: [-5, 0], B: [0, -5], C: [0, 5], c: [1, 1], h: [-1, 2], p: [0, 2], q: [1, 2], r: [2, 2], s: [3, 2], w: [3, -1], x: [4, -1], y: [5, -1], z: [4, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }, { mark: 'C' }],
  people: 12,
  lead: 0,
  armies: [{ civ: 2, at: { mark: 'c' }, people: 4 }],
  scouts: [{ civ: 2, at: { mark: 'c' }, pack: 0 }],
  clans: [
    { home: { mark: 'p' }, tiles: [{ mark: 'p' }], people: 6, civ: 0 }, // red, plains
    { home: { mark: 'q' }, tiles: [{ mark: 'q' }], people: 6, civ: 1 }, // blue, plains
    { home: { mark: 'r' }, tiles: [{ mark: 'r' }], people: 6, civ: 2 }, // gold, plains
    { home: { mark: 's' }, tiles: [{ mark: 's' }], people: 6 }, // none, plains
    { home: { mark: 'w' }, tiles: [{ mark: 'w' }], people: 6, civ: 0 }, // red, wood
    { home: { mark: 'x' }, tiles: [{ mark: 'x' }], people: 6, civ: 1 }, // blue, wood
    { home: { mark: 'y' }, tiles: [{ mark: 'y' }], people: 6, civ: 2 }, // gold, wood
    { home: { mark: 'z' }, tiles: [{ mark: 'z' }], people: 6 }, // none, wood
    { home: { mark: 'h' }, tiles: [{ mark: 'h' }], people: 9, civ: 0 }, // the shared home: red, to be settled
    { home: { mark: 'h' }, tiles: [], people: 5, civ: 0 }, // red gathering
    { home: { mark: 'h' }, tiles: [], people: 3, civ: 0 }, // red moving
    { home: { mark: 'h' }, tiles: [], people: 4, civ: 1 }, // blue gathering
    { home: { mark: 'c' }, tiles: [], people: 4 }, // none, beside gold's units
  ],
};
const page = await open('', { size: 'laptop' });
try {
  await page.eval(`game.stage(${JSON.stringify(CLANS)})`);
  await page.until("document.body.dataset.phase === 'main'");
  const info = await page.eval(`(() => { const m = game.map, v = game.view(); return { digest: game.digest(), clans: Object.values(v.clans).map((c) => [c.id, c.home, m.l.terrain[c.home], c.civ ?? '-', +c.people.toFixed(1), c.settled ? 'S' : 'g', c.since, c.starving ? 'starv' : '']), civs: Object.values(v.civs).map((c) => [c.id, c.name, c.color, c.capital, c.clan]), units: Object.values(v.units).map((u) => [u.id, u.kind, u.tile]) }; })()`);
  console.log(JSON.stringify(info));
  const shoot = async (name, k) => {
    await page.eval(`(() => { const m = game.map, a = m.at(-1, -1), b = m.at(4, 2); m.cx = (m.px[a] + m.px[b]) / 2; m.cy = (m.py[a] + m.py[b]) / 2; m.k = ${k}; m.viewChanged(); })()`);
    await page.until('!game.map.anim && !game.map.frame', 5000);
    const png = await page.eval(`(() => { const m = game.map; m.dirty = true; m.drawNow(); return m.canvas.toDataURL('image/png'); })()`);
    writeFileSync(join(out, `${name}-${k}.png`), Buffer.from(png.split(',')[1], 'base64'));
  };
  await shoot('staged', 35);
  // The states on the drawn model: the shared home's first clan settled, its third moving (since 0); gold's plains clan
  // starving; the plains clan of no civ leaning to blue at 0.6 of the way to joining, the wood's to gold at 0.2.
  const set = await page.eval(`(() => { const m = game.map, v = m.model.v, at = (q, r) => m.at(q, r), on = (t) => Object.values(v.clans).filter((c) => c.home === t).sort((a, b) => a.id - b.id);
    const h = on(at(-1, 2)); h[0].settled = true; h[2].since = 0; for (const c of h) if (c !== h[2] && c.since === 0) c.since = 5;
    for (const c of Object.values(v.clans)) if (c.since === 0 && c !== h[2]) c.since = 5;
    on(at(2, 2))[0].starving = true;
    const civId = (qr) => Object.values(v.civs).find((c) => c.capital === at(...qr)).id;
    on(at(3, 2))[0].leaning = { [civId([0, -5])]: 0.6 * m.model.rules.join }; on(at(4, 0))[0].leaning = { [civId([0, 5])]: 0.2 * m.model.rules.join };
    return 'set'; })()`).catch((e) => String(e));
  console.log(set);
  await shoot('states', 35); await shoot('states', 18);
} finally { await page.close(); }
