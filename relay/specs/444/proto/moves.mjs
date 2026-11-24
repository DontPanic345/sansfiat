// Turn by turn from the pick (the player only ends turns), which clans moved home (and their `since` the turn they
// arrive), which were born (new ids, and their `since`), which settled or went back to gathering; and at turns 0, 8
// and 16 the examples of each state for the views: settled off a capital, gathering of a civ and of none, leaning,
// starving, just moved, and every shared home with its clans.
// From engine/:  node ../relay/specs/444/proto/moves.mjs small 2 [turns]
import { open, lead } from '../../../../engine/www/play.mjs';
const [size, seed, N] = [process.argv[2], Number(process.argv[3]), Number(process.argv[4] ?? 16)];
const SNAP = `(() => { const v = game.view(); const o = {}; for (const c of Object.values(v.clans)) o[c.id] = [c.home, c.since, c.settled ? 1 : 0, c.starving ? 1 : 0, c.civ ?? -1, +c.people.toFixed(1)]; return { turn: +document.body.dataset.turn, clans: o, digest: game.digest() }; })()`;
const EX = `(() => { const m = game.map, l = m.l, v = game.view(), model = game.map.model, names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
  const caps = new Set(Object.values(v.civs).map((c) => c.capital)); const homes = {}; for (const c of Object.values(v.clans)) (homes[c.home] ??= []).push(c);
  const d = (t) => +(m.px[t] ** 2 + m.py[t] ** 2).toFixed(0); const row = (c) => ({ id: c.id, tile: c.home, t: names[l.terrain[c.home]], civ: c.civ ?? null, people: +c.people.toFixed(1), settled: !!c.settled, since: c.since, starving: !!c.starving, tint: model && model.tintOf ? model.tintOf(c) : undefined, cap: caps.has(c.home), share: homes[c.home].length, d: d(c.home) });
  const all = Object.values(v.clans).map(row).sort((a, b) => a.d - b.d);
  const pick = (f, k = 4) => all.filter(f).slice(0, k);
  return { turn: +document.body.dataset.turn, settledOff: pick((r) => r.settled && !r.cap), gatherCiv: pick((r) => !r.settled && r.civ != null), gatherNone: pick((r) => !r.settled && r.civ == null && !r.tint), leaning: pick((r) => r.tint), starving: pick((r) => r.starving), recent: pick((r) => r.since <= 1, 8),
    shared: Object.entries(homes).filter(([, cs]) => cs.length > 1).map(([t, cs]) => ({ tile: +t, t: names[l.terrain[t]], d: d(t), cap: caps.has(+t), town: (v.towns || []).some((w) => w.tile === +t), clans: cs.map((c) => [c.id, c.settled ? 'S' : 'g', c.civ ?? '-', +c.people.toFixed(1), c.starving ? 'starv' : ''].join(':')) })).sort((a, b) => a.d - b.d),
    caps: [...caps], digest: game.digest() }; })()`;
const page = await open('');
await lead(page, { size, seed });
const turn = async () => { const before = (await page.data()).turn; await page.key('Space'); try { await page.until(`document.body.dataset.turn !== '${before}'`, 30000); } catch { await page.button('End turn').catch(() => {}); } };
let prev = await page.eval(SNAP); const log = [];
console.log(JSON.stringify({ size, seed, ex0: await page.eval(EX) }));
for (let t = 1; t <= N; t++) {
  await turn(); const now = await page.eval(SNAP); const ev = { turn: now.turn, moved: [], born: [], settled: [], gathers: [] };
  for (const [id, s] of Object.entries(now.clans)) { const p = prev.clans[id]; if (!p) ev.born.push([+id, s[0], 'since', s[1]]); else { if (p[0] !== s[0]) ev.moved.push([+id, p[0], s[0], 'since', s[1]]); if (!p[2] && s[2]) ev.settled.push([+id, s[0]]); if (p[2] && !s[2]) ev.gathers.push([+id, s[0]]); } }
  log.push(ev); prev = now;
  if (t === 8 || t === N) console.log(JSON.stringify({ size, seed, t, ex: await page.eval(EX) }));
}
console.log(JSON.stringify({ size, seed, log }));
await page.close();
