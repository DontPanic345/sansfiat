// The settlement roles on a retro map, leading the first civ and ending turns: at each checkpoint, every capital,
// settlement and town with its civ, its clans on the tile, its terrain, its forest and mountain neighbours, and the
// units within two steps; and the signposts. One JSON line per checkpoint.
// From engine/:  node ../relay/specs/445/proto/places.mjs small 2 16,40
import { open, lead } from '../../../../engine/www/play.mjs';
const [size, seed, at] = [process.argv[2], Number(process.argv[3]), process.argv[4].split(',').map(Number)];
const PROBE = `(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
  const ter = (i) => names[l.terrain[i]];
  const nbs = (i) => { const o = []; for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0) o.push(j); } return o; };
  const near2 = (i) => { const s = new Set([i]); for (const j of nbs(i)) { s.add(j); for (const k of nbs(j)) s.add(k); } return s; };
  const homes = {}; for (const c of Object.values(v.clans)) (homes[c.home] ??= []).push(c.id);
  const units = Object.values(v.units || {});
  const place = (tile, role, civ) => { const n = nbs(tile); const z = near2(tile);
    return { tile, role, civ, ter: ter(tile), clans: (homes[tile] || []).length, town: (v.towns || []).some((t) => t.tile === tile),
      forest: n.filter((j) => ter(j) === 'forest').length, mountain: n.filter((j) => /mountain|hill/.test(ter(j))).length,
      units: units.filter((u) => z.has(u.tile)).map((u) => u.kind + '/' + u.civ + '@' + u.tile) }; };
  const out = [];
  for (const c of Object.values(v.civs)) if (c.alive !== false) out.push(place(c.capital, 'capital', c.id));
  for (const s of v.settlements || []) out.push(place(s.tile, 'settlement', s.civ));
  const taken = new Set(out.map((p) => p.tile));
  for (const t of v.towns || []) if (!taken.has(t.tile)) out.push(place(t.tile, 'town', null));
  return { turn: v.turn, digest: game.digest(), places: out, signposts: (v.signposts || []).length, maxHome: Math.max(...Object.values(homes).map((h) => h.length)) }; })()`;
const page = await open('');
await lead(page, { size, seed });
let done = 0;
for (const n of at) {
  for (; done < n; done++) { const before = (await page.data()).turn; await page.key('Space'); await page.until(`document.body.dataset.turn !== '${before}'`, 60000); }
  await page.until('!game.map.anim && !game.map.frame', 10000);
  console.log(JSON.stringify({ size, seed, after: n, ...(await page.eval(PROBE)) }));
}
await page.close();
