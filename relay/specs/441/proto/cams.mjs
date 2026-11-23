// The candidate shore views on main: is each camera unclamped, and a canvas of each. From engine/:
//   node ../relay/specs/441/proto/cams.mjs /tmp/cams
import { open } from '../../../../engine/www/play.mjs';
import { mkdirSync, writeFileSync } from 'node:fs';
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const V = [
  ['small', 2, 'laptop', [['neck-close', 402, 35], ['delta-close', 506, 35], ['delta-play', 506, 18]]],
  ['small', 2, 'phone', [['neck-edge', 402, 6], ['neck-flat', 402, 5]]],
  ['small', 6, 'laptop', [['bay-close', 376, 35]]],
  ['small', 9, 'laptop', [['lake-play', 377, 18], ['pond-close', 341, 35]]],
  ['medium', 8, 'laptop', [['strait-close', 875, 35]]],
  ['medium', 9, 'laptop', [['neck-play', 1004, 18]]],
  ['large', 8, 'laptop', [['delta-play', 2639, 18], ['pond-play', 1289, 18]]],
];
for (const [size, seed, win, views] of V) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`, { size: win });
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  for (const [name, tile, k] of views) {
    const asked = await page.eval(`(() => { const m = game.map; m.cx = m.px[${tile}]; m.cy = m.py[${tile}]; m.k = ${k}; m.viewChanged(); return { cx: m.cx, cy: m.cy }; })()`);
    await page.eval(`(() => { const m = game.map; m.cx = m.px[${tile}]; m.cy = m.py[${tile}]; m.k = ${k}; m.viewChanged(); })()`);
    await page.until('!game.map.anim && !game.map.frame', 5000);
    const r = await page.eval(`(() => { const m = game.map; m.drawNow(); return { cx: m.cx, cy: m.cy, k: m.k, want: [m.px[${tile}], m.py[${tile}]], png: m.canvas.toDataURL('image/png') }; })()`);
    const clamped = r.cx !== r.want[0] || r.cy !== r.want[1] || r.k !== k;
    console.log(`${size}${seed}-${name} tile ${tile} k ${k} ${clamped ? 'CLAMPED ' + JSON.stringify([r.cx, r.cy, r.k]) : 'ok'}`);
    writeFileSync(`${out}/${size}${seed}-${name}.png`, Buffer.from(r.png.split(',')[1], 'base64'));
  }
  await page.close();
}
