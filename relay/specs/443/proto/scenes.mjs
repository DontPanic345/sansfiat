// The unit scenes on the retro maps, as the spec's views set them up: lead a civ (the first card, or the one named),
// optionally raise an army of 3 at the capital (play.mjs's raiseArmy) and end turns; then put the camera on a tile or
// between two, shoot the map canvas, and print the units, their terrain and the digest.
// From engine/:  node ../relay/specs/443/proto/scenes.mjs OUTDIR
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { open, lead, raiseArmy } from '../../../../engine/www/play.mjs';
const out = process.argv[2] ?? '/tmp/scenes';
mkdirSync(out, { recursive: true });
const SCENES = [
  { name: 'medium9-units-play', size: 'medium', seed: 9, raise: 3, window: 'laptop', k: 18, between: [547, 780] },
  { name: 'medium9-stack-close', size: 'medium', seed: 9, raise: 3, window: 'laptop', k: 35, tile: 547 },
  { name: 'medium9-wood-close', size: 'medium', seed: 9, window: 'laptop', k: 35, tile: 780 },
  { name: 'medium9-gold-close', size: 'medium', seed: 9, civ: 'Gold', raise: 3, window: 'laptop', k: 35, tile: 780 },
  { name: 'medium8-units-play', size: 'medium', seed: 8, window: 'laptop', k: 18, between: [1145, 639] },
  { name: 'small2-camps-play', size: 'small', seed: 2, turns: 8, window: 'laptop', k: 18, tile: 527 },
  { name: 'medium9-units-edge', size: 'medium', seed: 9, raise: 3, window: 'phone', k: 6, between: [547, 780] },
];
const only = process.argv[3]?.split(',');
for (const s of SCENES.filter((s) => !only || only.includes(s.name))) {
  const page = await open('', { size: s.window });
  await lead(page, { size: s.size, seed: s.seed, civ: s.civ });
  if (s.raise) await raiseArmy(page, { warriors: s.raise, turns: 10 });
  for (let t = 0; t < (s.turns ?? 0); t++) {
    const before = (await page.data()).turn;
    await page.key('Space');
    await page.until(`document.body.dataset.turn !== '${before}'`, 60000);
  }
  await page.until('!game.map.anim && !game.map.frame', 10000);
  await page.eval(`(() => { const m = game.map, t = ${JSON.stringify(s.between ?? [s.tile])};
    m.cx = t.reduce((a, i) => a + m.px[i], 0) / t.length; m.cy = t.reduce((a, i) => a + m.py[i], 0) / t.length; m.k = ${s.k}; m.viewChanged(); })()`);
  await page.until('!game.map.anim && !game.map.frame', 5000);
  const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
  writeFileSync(join(out, `${s.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
  const info = await page.eval(`(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
    return { turn: v.turn, player: v.player, cam: [+m.cx.toFixed(2), +m.cy.toFixed(2), m.k], units: Object.values(v.units).map((u) => [u.id, v.civs[u.civ].name ?? u.civ, u.civ === v.player ? 'mine' : 'rival', u.kind, u.tile, names[l.terrain[u.tile]], +u.people.toFixed(1), u.hungry > 0 ? 'hungry' : ''].join(' ')), digest: game.digest() }; })()`);
  console.log(s.name, JSON.stringify(info), page.errors().join('; '));
  await page.close();
}
