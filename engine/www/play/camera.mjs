// A left click centres the camera: it eases to the clicked tile's middle with the zoom unchanged, and a second click
// on the same tile (to cycle what is there) leaves the view still.   node www/play/camera.mjs [--out DIR]     (small seed 1)
import { open } from '../play.mjs';

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`camera: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const view = (page) => page.eval('(() => { const m = game.map; return { cx: m.cx, cy: m.cy, k: m.k }; })()');
const settled = (page) => page.until('!game.map.anim');

const page = await open('?size=small&seed=1&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  await settled(page);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  // A land tile of the player's, a few tiles off the capital's, well inside the map.
  const target = await page.eval(`(() => { const v = game.view(), m = game.map;
    const near = v.civs[${me}].land.filter((t) => t !== ${cap} && Math.hypot(m.px[t] - m.px[${cap}], m.py[t] - m.py[${cap}]) > 0);
    near.sort((a, b) => Math.hypot(m.px[b] - m.px[${cap}], m.py[b] - m.py[${cap}]) - Math.hypot(m.px[a] - m.px[${cap}], m.py[a] - m.py[${cap}]));
    return near[0]; })()`);
  const at = (i) => page.eval(`({ x: game.map.px[${i}], y: game.map.py[${i}] })`);
  const k0 = (await view(page)).k;

  await page.click(target);
  await settled(page);
  let v = await view(page);
  let p = await at(target);
  need(Math.abs(v.cx - p.x) < 1 && Math.abs(v.cy - p.y) < 1, `a click on tile ${target} centres it (${v.cx.toFixed(1)}, ${v.cy.toFixed(1)} vs ${p.x.toFixed(1)}, ${p.y.toFixed(1)})`);
  need(v.k === k0, 'the zoom is unchanged');

  await page.click(cap);
  await settled(page);
  v = await view(page);
  p = await at(cap);
  need(Math.abs(v.cx - p.x) < 1 && Math.abs(v.cy - p.y) < 1, 'a click on the capital centres it');
  const first = (await page.data()).selected;
  await page.click(cap);
  await settled(page);
  const again = await view(page);
  need((await page.data()).selected !== first, `the second click changes the selection (${first} to ${(await page.data()).selected})`);
  need(again.cx === v.cx && again.cy === v.cy, 'and the view does not move');
  for (const e of page.errors()) need(false, `page error: ${e}`);
} catch (e) {
  need(false, e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
