// Where the places and units lie on a retro map after N turns: each place's and unit's world point, the distance in
// world units from each place to the nearest unit, and the terrain names of each place's six neighbours.
// From engine/:  node ../relay/specs/445/proto/near.mjs medium 9 40
import { open, lead } from '../../../../engine/www/play.mjs';
const [size, seed, turns] = [process.argv[2], Number(process.argv[3]), Number(process.argv[4])];
const page = await open('');
await lead(page, { size, seed });
for (let t = 0; t < turns; t++) { const before = (await page.data()).turn; await page.key('Space'); await page.until(`document.body.dataset.turn !== '${before}'`, 60000); }
await page.until('!game.map.anim && !game.map.frame', 10000);
console.log(JSON.stringify(await page.eval(`(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
  const pt = (i) => [+m.px[i].toFixed(2), +m.py[i].toFixed(2)];
  const homes = {}; for (const c of Object.values(v.clans)) (homes[c.home] ??= []).push(c.id);
  const units = Object.values(v.units || {}).map((u) => ({ id: u.id, kind: u.kind, civ: u.civ, tile: u.tile, at: pt(u.tile) }));
  const places = [...Object.values(v.civs).map((c) => ({ role: 'capital', tile: c.capital, civ: c.id })), ...(v.settlements || []).map((s) => ({ role: 'settlement', tile: s.tile, civ: s.civ })), ...(v.towns || []).map((t) => ({ role: 'town', tile: t.tile }))]
    .map((p) => { const nb = []; for (let d = 0; d < 6; d++) { const j = m.nb[p.tile * 6 + d]; nb.push(j >= 0 ? names[l.terrain[j]] : '-'); }
      const near = units.map((u) => ({ u: u.kind + u.id + '@' + u.tile, d: +Math.hypot(u.at[0] - m.px[p.tile], u.at[1] - m.py[p.tile]).toFixed(1) })).sort((a, b) => a.d - b.d).slice(0, 2);
      return { ...p, at: pt(p.tile), ter: names[l.terrain[p.tile]], clans: (homes[p.tile] || []).length, nb: nb.join(','), near }; });
  return { turn: v.turn, digest: game.digest(), places }; })()`)));
await page.close();
