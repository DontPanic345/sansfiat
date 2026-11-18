// A left click centres the camera: it eases to the clicked tile's middle with the zoom unchanged, and a second click
// on the same tile (to cycle what is there) leaves the view still; a tile's tooltip names it (q, r). The
// staged capital stands off the centre and not on the rim, with its scout on it.   node www/play/camera.mjs [--out DIR]
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

// Radius 8: the led civ's capital at (-4, 0), the rival's at (4, 0).
const R = 8;
const SPEC = {
  map: plains(R),
  marks: marks(R, { A: [-4, 0], B: [4, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
};

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`camera: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const view = (page) => page.eval('(() => { const m = game.map; return { cx: m.cx, cy: m.cy, k: m.k }; })()');
const settled = (page) => page.until('!game.map.anim');
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('');
try {
  await stage(page, SPEC);
  // The map is zoomed in off its fit: a panel that changes the map's box refits a map at its fit and drops any
  // centring under way. Zoomed, the box only clamps the view.
  await page.eval('game.map.zoomBy(2)');
  await settled(page);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  // The land tile of the player's nearest the capital, off it (the centre clamps at the map's edge, so not a far one).
  const target = await page.eval(`(() => { const v = game.view(), m = game.map;
    const near = v.civs[${me}].land.filter((t) => t !== ${cap} && Math.hypot(m.px[t] - m.px[${cap}], m.py[t] - m.py[${cap}]) > 0);
    near.sort((a, b) => Math.hypot(m.px[a] - m.px[${cap}], m.py[a] - m.py[${cap}]) - Math.hypot(m.px[b] - m.px[${cap}], m.py[b] - m.py[${cap}]));
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
  need(Math.abs(v.cx - p.x) < 1 && Math.abs(v.cy - p.y) < 1, `a click on the capital centres it (${v.cx.toFixed(1)}, ${v.cy.toFixed(1)} vs ${p.x.toFixed(1)}, ${p.y.toFixed(1)}; k ${v.k.toFixed(1)})`);
  const first = (await page.data()).selected;
  await page.click(cap);
  await settled(page);
  const again = await view(page);
  need((await page.data()).selected !== first, `the second click changes the selection (${first} to ${(await page.data()).selected})`);
  need(again.cx === v.cx && again.cy === v.cy, 'and the view does not move');
  // A tile's tooltip names it (q, r), as refusals, the feed and the reach line name tiles.
  const R = await page.eval('game.map.radius');
  const qr = await page.eval(`(() => { for (let q = -${R}; q <= ${R}; q++) for (let r = -${R}; r <= ${R}; r++) if (game.map.at(q, r) === ${target}) return [q, r]; })()`);
  const tip = (await page.tip(target)) ?? '';
  need(tip.includes(`(${qr[0]}, ${qr[1]})`), `the tooltip names the tile (${qr[0]}, ${qr[1]}): ${tip.slice(0, 60)}`);
  for (const e of page.errors()) need(false, `page error: ${e}`);
} catch (e) {
  need(false, e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
