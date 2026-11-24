// The review's drag: on medium 9 led, at k 6, 12 and 18, drag the map 40 steps of whole css px (a random walk), then compare
// the panned static layer and the whole frame with a fresh full redraw at the same camera; then end a turn mid-pan and toggle
// a View overlay mid-pan, each compared the same way. Writes the worst frame's diff mask to OUT/drag-*.png.
//   node relay/specs/446/proto/drag.mjs OUT
import { open, lead } from '../../../../engine/www/play.mjs';
import { writeFileSync } from 'node:fs';
const out = process.argv[2] ?? '/tmp';
const inPage = `(async (k) => {
  const m = game.map;
  let seed = 7 + k; const rnd = () => (seed = (seed * 16807) % 2147483647) / 2147483647;
  const draws = (window._draws ??= { whole: 0, strips: 0 });
  if (!m._wrapped) { const d = m.drawStatic.bind(m); m.drawStatic = () => (m.strip ? draws.strips++ : draws.whole++, d()); m._wrapped = true; }
  const grab = (c) => c.getContext('2d').getImageData(0, 0, c.width, c.height).data;
  const cmp = (a, b) => { let n = 0, max = 0, big = 0; for (let i = 0; i < a.length; i += 4) { const d = Math.max(Math.abs(a[i]-b[i]), Math.abs(a[i+1]-b[i+1]), Math.abs(a[i+2]-b[i+2])); if (d) n++; if (d > 40) big++; if (d > max) max = d; } return { pct: +(100 * n / (a.length / 4)).toFixed(3), big: +(100 * big / (a.length / 4)).toFixed(3), max }; };
  const fresh = () => { const L = grab(m.layer), F = grab(m.canvas); m.dirty = true; m.drawNow(); return { layer: cmp(L, grab(m.layer)), frame: cmp(F, grab(m.canvas)) }; };
  m.k = k; m.viewChanged(); m.drawNow();
  const res = {};
  const walk = (n) => { for (let s = 0; s < n; s++) { const dx = Math.round((rnd() - 0.5) * 60), dy = Math.round((rnd() - 0.5) * 60); m.panBy(dx, dy); m.drawNow(); } };
  draws.whole = draws.strips = 0; walk(40); res.drag = { ...draws, ...fresh() };
  draws.whole = draws.strips = 0; walk(10); const t0 = game.view?.().turn; document.body.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true })); res.turns = [t0, game.view?.().turn]; walk(10); res.endTurn = { ...draws, ...fresh() };
  draws.whole = draws.strips = 0; walk(10); m.setLayer('fertility', true); walk(10); m.setLayer('fertility', false); walk(5); res.overlay = { ...draws, ...fresh() };
  return res;
})`;
const page = await open('');
try {
  await lead(page, { seed: 9, size: 'medium' });
  for (const k of [6, 12, 18]) console.log(`k ${k}`, JSON.stringify(await page.eval(`${inPage}(${k})`)));
  for (const e of page.errors()) console.log('page error', e);
} finally { await page.close(); }
