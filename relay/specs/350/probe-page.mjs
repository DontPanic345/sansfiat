import { open } from '/home/runner/work/sansfiat/sansfiat/engine/www/play.mjs';
import { marks, plains, stage, LINEUP } from '/home/runner/work/sansfiat/sansfiat/engine/www/play-stage.mjs';
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
const R = 8;
const SPEC = { map: plains(R), marks: marks(R, { A: [-4, 0], B: [4, 0], K: [0, 3] }), civs: [{ mark: 'A' }, { mark: 'B' }], people: 12, lead: 0,
  stores: [{ civ: 0, food: 50 }], clans: [{ home: { mark: 'K' }, tiles: [], people: 4, market: 0 }] };
let bad = 0;
const need = (ok, m) => { console.log(ok ? 'ok  ' : 'FAIL', m); if (!ok) bad++; };
const page = await open('');
try {
  await stage(page, SPEC);
  const civ = await page.eval('game.view().player');
  const free = () => page.eval(`(() => { const m = game.map, c = game.view().civs[${civ}]; const out = new Set();
    for (const t of c.claim) for (let d = 0; d < 6; d++) { const j = m.nb[t * 6 + d]; if (j >= 0 && m.land[j] && !c.claim.includes(j)) out.add(j); }
    return [...out].sort((a, b) => a - b)[0]; })()`);
  const claimed = () => page.eval(`game.view().civs[${civ}].claim.length`);
  await page.eval(`document.querySelector('button.chip[data-civ="${civ}"]').click()`);
  await page.eval(frames);
  await page.key('p');
  let c0 = await claimed();
  await page.click(await free());
  need((await claimed()) === c0 + 1, 'first P + click claims');
  await page.button('End turn');
  await page.eval(frames);
  // The next turn's claim: P then a press on a free tile, as a player does each turn.
  await page.key('p');
  c0 = await claimed();
  await page.click(await free());
  await page.eval(frames);
  need((await claimed()) === c0 + 1, `P after a claim and a turn still claims (painting=${await page.eval('document.body.dataset.painting')}, selected=${await page.eval('document.body.dataset.selected')})`);

  // Move to… pressed twice, then a click.
  await stage(page, LINEUP);
  const army = await page.eval(`Object.values(game.view().units).find((u) => u.civ === game.view().player && u.kind === 'camp').id`);
  const at = await page.eval(`game.view().units[${army}].tile`);
  await page.select(at, 'unit');
  const sel = await page.eval('document.body.dataset.selected');
  if (sel !== `unit:${army}`) await page.select(at, 'unit');
  const to = await page.eval('game.map.at(-2, 2)');
  await page.button('Move to…');
  await page.button('Move to…');
  await page.click(to);
  await page.eval(frames);
  need((await page.eval(`game.view().units[${army}].dest`)) === to, `Move to… pressed again, then a click, moves the army (armed=${await page.eval('document.body.dataset.armed')}, selected=${await page.eval('document.body.dataset.selected')})`);
  for (const e of page.errors()) need(false, e);
} finally { await page.close(); }
process.exit(bad ? 1 : 0);
