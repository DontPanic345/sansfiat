// A view on main as shots.mjs would set it: lead the first civ, end `turns` turns, put the camera on the mean of the
// tiles, check the map kept the camera, and write the canvas. Prints the camera, the digest, and the places and units on screen.
// From engine/:  node ../relay/specs/445/proto/scene.mjs medium 9 40 laptop 18 735,566 /tmp/x.png
import { open, lead } from '../../../../engine/www/play.mjs';
import { writeFileSync } from 'node:fs';
const [size, seed, turns, win, k, tiles, out] = process.argv.slice(2);
const page = await open('', { size: win });
await lead(page, { size, seed: Number(seed) });
for (let t = 0; t < Number(turns); t++) { const before = (await page.data()).turn; await page.key('Space'); await page.until(`document.body.dataset.turn !== '${before}'`, 60000); }
await page.until('!game.map.anim && !game.map.frame', 10000);
const ts = tiles.split(',').map(Number);
const r = await page.eval(`(() => { const m = game.map, ts = ${JSON.stringify(ts)}; const cx = ts.reduce((a, i) => a + m.px[i], 0) / ts.length, cy = ts.reduce((a, i) => a + m.py[i], 0) / ts.length;
  m.cx = cx; m.cy = cy; m.k = ${Number(k)}; m.viewChanged(); return { cx, cy }; })()`);
await page.until('!game.map.anim && !game.map.frame', 5000);
const info = await page.eval(`(() => { const m = game.map, v = game.view(); m.drawNow(); const on = Object.values(v.clans).filter((c) => m.onScreen(c.home));
  return { cam: { cx: m.cx, cy: m.cy, k: m.k }, dpr: m.dpr, turn: v.turn, digest: game.digest(), onScreen: on.length, capitals: Object.values(v.civs).map((c) => c.capital).filter((t) => m.onScreen(t)), settlements: v.settlements.filter((s) => m.onScreen(s.tile)).map((s) => s.tile), towns: v.towns.filter((t) => m.onScreen(t.tile)).map((t) => t.tile + '/' + t.clans.length), units: Object.values(v.units).filter((u) => m.onScreen(u.tile)).map((u) => u.kind + '@' + u.tile), posts: v.signposts.filter((s) => m.onScreen(s.tile)).length,
    png: m.canvas.toDataURL('image/png') }; })()`);
writeFileSync(out, Buffer.from(info.png.split(',')[1], 'base64')); delete info.png;
console.log(JSON.stringify({ asked: r, ...info, clamped: info.cam.cx !== r.cx || info.cam.cy !== r.cy }));
await page.close();
