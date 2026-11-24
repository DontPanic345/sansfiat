// The clans on a retro map: at the pick (leading the first civ), and 8 and 16 turns on (the player only ends turns).
// Per moment: how many clans, how many settled, gathering, moved this turn (since 0) or lately (since < 3), starving,
// of a civ or of none; how many clans share a home (the homes by count); the settled share by civ; people by state;
// and per state the clan nearest world (0, 0) with its tile and terrain, for the views.
// From engine/:  node ../relay/specs/444/proto/clans.mjs small 2
import { open, lead } from '../../../../engine/www/play.mjs';
const [size, seed] = [process.argv[2], Number(process.argv[3])];
const PROBE = `(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
  const cs = Object.values(v.clans);
  const homes = {}; for (const c of cs) (homes[c.home] ??= []).push(c.id);
  const hist = {}; for (const ids of Object.values(homes)) hist[ids.length] = (hist[ids.length] || 0) + 1;
  const civ = (c) => c.civ ?? null;
  const n = (f) => cs.filter(f).length;
  const ppl = (f) => { const a = cs.filter(f).map((c) => c.people).sort((x, y) => x - y); return a.length ? [+a[0].toFixed(1), +a[a.length >> 1].toFixed(1), +a[a.length - 1].toFixed(1)] : []; };
  const near = {}; const put = (k, c) => { const d = m.px[c.home] ** 2 + m.py[c.home] ** 2; if (!near[k] || d < near[k].d) near[k] = { clan: c.id, tile: c.home, t: names[l.terrain[c.home]], civ: civ(c), share: homes[c.home].length, d: +d.toFixed(1) }; };
  for (const c of cs) { put(c.settled ? 'settled' : 'gathering', c); if (c.since === 0) put('moved', c); if (c.starving) put('starving', c); if (civ(c) == null) put('none', c); if (homes[c.home].length >= 4) put('crowd', c); if (c.settled && c.since === 0) put('settled-moved', c); }
  const byCiv = {}; for (const c of cs) { const k = civ(c) ?? 'none'; byCiv[k] ??= [0, 0]; byCiv[k][0]++; if (c.settled) byCiv[k][1]++; }
  const mixed = Object.values(homes).filter((ids) => { const s = new Set(ids.map((i) => v.clans[i].settled)); return s.size > 1; }).length;
  const mixedCiv = Object.values(homes).filter((ids) => new Set(ids.map((i) => v.clans[i].civ ?? -1)).size > 1).length;
  return { turn: document.body.dataset.turn, clans: cs.length, settled: n((c) => c.settled), gathering: n((c) => !c.settled),
    moved0: n((c) => c.since === 0), movedLt3: n((c) => c.since < 3), starving: n((c) => c.starving), none: n((c) => civ(c) == null),
    homes: Object.keys(homes).length, shareHist: hist, mixedSettledHomes: mixed, mixedCivHomes: mixedCiv, byCiv,
    people: { settled: ppl((c) => c.settled), gathering: ppl((c) => !c.settled) }, hearths: game.map.hearths.length,
    markets: Object.keys(v.markets).length, towns: (v.towns || []).length, keys: Object.keys(cs[0]).join(','), near, digest: game.digest() }; })()`;
const page = await open('');
await lead(page, { size, seed });
const turns = async (k) => { for (let t = 0; t < k; t++) { const before = (await page.data()).turn; await page.key('Space'); try { await page.until(`document.body.dataset.turn !== '${before}'`, 30000); } catch { await page.button('End turn').catch(() => {}); } } };
const out = { size, seed, at0: await page.eval(PROBE) };
await turns(8); out.at8 = await page.eval(PROBE);
await turns(8); out.at16 = await page.eval(PROBE);
console.log(JSON.stringify(out));
await page.close();
