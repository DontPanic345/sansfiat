// Signposts and settlements on the page (#353): a staged game on a radius-8 plains map, the led civ's capital at
// (-4, 0), a rival's at (4, 0), a signpost of the led civ at (0, 0) with a clan of the civ living on it. The page
// draws the signpost (the settle layer, on and off); a turn's `settle` step founds the settlement, the feed has
// its line, the settlement is drawn, a click on its tile selects it, and its panel gives the name line, the store
// against its cap, the clans, the land and the reasons. Writes settle.png (the settlement, selected, with its land
// lit) to --out.
//   node www/play/settle.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-settle-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`settle: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const R = 8;
const SPEC = {
  map: plains(R),
  marks: marks(R, { A: [-4, 0], B: [4, 0], P: [0, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  clans: [{ home: { mark: 'P' }, tiles: [], people: 4, civ: 0 }],
  signposts: [{ civ: 0, at: { mark: 'P' } }],
};

const page = await open('');
try {
  await stage(page, SPEC);

  const v0 = await page.eval('game.view()');
  need(v0.signposts.length === 1 && v0.settlements.length === 0, 'the staged view has one signpost and no settlement');
  const tile = v0.signposts[0].tile;
  await page.eval(`game.map.centreOn(${tile}, { minK: 40, smooth: false })`);
  await page.eval(frames);

  // The signpost is drawn: the settle layer is on by default, and the tile differs with it off.
  const around = (i, half) =>
    page.eval(`(() => {
      const c = document.getElementById('map');
      const r = c.getBoundingClientRect();
      const p = game.map.screenOf(${i});
      const s = c.width / r.width;
      const h = Math.round(${half} * s);
      const x = Math.round((p.x - r.left) * s), y = Math.round((p.y - r.top) * s);
      return Array.from(c.getContext('2d').getImageData(x - h, y - h, 2 * h, 2 * h).data);
    })()`);
  const differ = (a, b) => {
    let n = 0;
    for (let k = 0; k < a.length; k += 4) if (Math.max(...[0, 1, 2].map((c) => Math.abs(a[k + c] - b[k + c]))) > 8) n++;
    return n;
  };
  need(await page.eval('game.map.on.settle === true'), 'the settle layer is on by default');
  const withPost = await around(tile, 30);
  await page.eval("game.map.setLayer('settle', false)");
  await page.eval(frames);
  const without = await around(tile, 30);
  await page.eval("game.map.setLayer('settle', true)");
  await page.eval(frames);
  need(differ(withPost, without) > 20, `the signpost is drawn on its tile (${differ(withPost, without)} pixels differ with the layer off)`);

  // Its tile's panel: posted, worth, Remove signpost.
  for (let n = 0; n < 12; n++) {
    if (String(await page.eval("document.body.dataset.selected || ''")) === `tile:${tile}`) break;
    await page.click(tile);
    await page.eval(frames);
  }
  const tp = await page.panel();
  need(tp?.numbers.some((x) => x.label === 'Signpost') && tp.buttons.some((b) => b.label === 'Remove signpost' && !b.disabled), 'the signposted tile\'s panel shows the signpost and Remove signpost');

  // A turn founds the settlement.
  await page.button('End turn');
  await page.eval(frames);
  const v1 = await page.eval('game.view()');
  need(v1.settlements.length === 1 && v1.signposts.length === 0, 'a turn founds the settlement and takes the signpost down');
  const s = v1.settlements[0];
  need(s.tile === tile && s.civ === v1.player, 'it is the led civ\'s, on the signposted tile');
  const feed = await page.eval("game.feed.items.filter((i) => i.kind === 'settlementFounded').map((i) => i.text)");
  need(feed.length === 1 && feed[0].includes('founds a settlement'), `the feed has the founding line (${feed.join('; ') || 'none'})`);
  const afterMark = await around(tile, 30);
  need(differ(withPost, afterMark) > 20, 'the settlement is drawn in the signpost\'s place');

  // A click cycles to the settlement; its panel.
  for (let n = 0; n < 12; n++) {
    if (String(await page.eval("document.body.dataset.selected || ''")) === `settlement:${s.id}`) break;
    await page.click(tile);
    await page.eval(frames);
  }
  need((await page.eval("document.body.dataset.selected")) === `settlement:${s.id}`, 'a click on the tile selects the settlement');
  const p = await page.panel();
  need(p && /'s settlement at \(-?\d+, -?\d+\)$/.test(p.name), `the name line reads as a settlement (${p?.name})`);
  for (const label of ['Store', 'Levy', 'Land']) {
    const n = p?.numbers.find((x) => x.label === label);
    need(n && n.tip, `${label} is there with a tooltip (${n?.value})`);
  }
  need(p?.numbers.find((x) => x.label === 'Store').value.includes('of'), 'the store is read against its cap');
  const living = p?.lists.find((l) => l.label === 'Clans living here');
  need(living && living.rows.length === s.clans.length && s.clans.length >= 1, `the clans living there are listed (${living?.rows.join(', ')})`);
  need(p?.lists.some((l) => l.label === 'Clans coming to its market'), 'its market\'s clans are listed');
  need(p?.reasons.length > 0, `it has reasons (${p?.reasons[0]})`);
  need((await page.eval('game.map.highlight?.tiles?.length ?? 0')) === s.land.length, `the map lights its land (${s.land.length} tiles)`);

  // A second signpost, posted as a player posts one: the civ's Signpost… button, then a click on a tile four steps on.
  const post2 = await page.eval(`(() => { const l = game.layout(); for (let i = 0; i < l.q.length; i++) if (l.q[i] === 0 && l.r[i] === 4) return i; return -1; })()`);
  need(post2 >= 0, 'the second signpost\'s tile is on the map');
  for (let n = 0; n < 12 && (await page.eval("document.body.dataset.selected || ''")) !== `civ:${v1.player}`; n++) {
    await page.click(v1.civs[v1.player].capital);
    await page.eval(frames);
  }
  await page.button('Signpost…');
  await page.click(post2);
  await page.eval(frames);
  need((await page.eval('game.view().signposts.length')) === 1, 'a click on the map posts the signpost');
  for (let n = 0; n < 12 && (await page.eval("document.body.dataset.selected || ''")) !== `settlement:${s.id}`; n++) {
    await page.click(tile);
    await page.eval(frames);
  }
  await page.eval("document.getElementById('legend').open = false");
  await page.eval("game.map.centreOn(" + tile + ", { minK: 70, smooth: false })");
  await page.eval(frames);
  await page.shot(join(out, 'settle.png'), 1);
  console.log(`      the screenshot is ${join(out, 'settle.png')}`);
  for (const e of page.errors()) need(false, `page error: ${e}`);
} finally {
  await page.close();
}
if (failed) process.exit(1);
