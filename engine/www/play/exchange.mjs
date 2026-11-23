// Exchange on the page (#356): a store's picked good draws its chains on the map. A staged game: the led civ's
// capital A, a rival's capital B, a salt source S and an obsidian source O, and the capital's store holding 40 salt
// and 3 obsidian, each dug at its source. Picks the store's obsidian and checks the flow layer drew a chain whose
// first point is a source, then salt, then the second click and Escape clear it.
//   node www/play/exchange.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-exchange-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`exchange: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// goods()'s order: the foods, then salt and obsidian (core/src/data/goods.rs).
const SALT = 5;
const OBSIDIAN = 6;

const R = 4;
const SPEC = {
  map: plains(R),
  marks: marks(R, { A: [-2, 0], B: [2, 0], S: [0, -1], O: [0, 1] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  sources: [
    { at: { mark: 'S' }, good: 'salt' },
    { at: { mark: 'O' }, good: 'obsidian' },
  ],
  stores: [{ civ: 0, food: 10, salt: 40, obsidian: 3 }],
};

const page = await open('');
try {
  await stage(page, SPEC);
  const v0 = await page.eval('game.view()');
  const civ = v0.player;
  const capital = v0.civs[civ].capital;
  const sources = await page.eval('game.map.l.source');
  const S = sources.indexOf('salt');
  const O = sources.indexOf('obsidian');
  need(S >= 0 && O >= 0, `the map has a salt source (${S}) and an obsidian source (${O})`);
  need(v0.civs[civ].store[SALT] > 0 && v0.civs[civ].store[OBSIDIAN] > 0, 'the capital’s store holds salt and obsidian');

  // The store's obsidian: pick it, and the flow layer draws its chain from the obsidian source.
  for (let n = 0; n < 12 && (await page.data()).selected !== `civ:${civ}`; n++) {
    await page.click(capital);
    await page.eval(frames);
  }
  need((await page.data()).selected === `civ:${civ}`, 'the led civ’s panel is selected through its capital');
  const pickRow = (name) =>
    page.eval(`(() => { const b = [...document.querySelectorAll('#detail button.num.pick')].find((b) => b.textContent.includes(${JSON.stringify(name)})); if (!b) return false; b.click(); return true; })()`);
  need(await pickRow('Obsidian'), 'the store’s Obsidian row is a pick');
  await page.eval(frames);
  const flow = await page.eval('(() => { const f = game.map.flow; return f ? JSON.parse(JSON.stringify(f)) : null; })()');
  need(flow && flow.good === 'obsidian' && flow.chains.length > 0, `the picked obsidian makes ${flow ? flow.chains.length : 0} chain(s) (body.good ${await page.eval("document.body.dataset.good")})`);
  need(flow && flow.chains.every((c) => sources[c.path[0]] === 'obsidian'), 'each obsidian chain starts on an obsidian source');
  need(flow && flow.chains.every((c) => c.path[c.path.length - 1] === capital), 'each chain ends at the store');
  need(flow && flow.chains.every((c) => c.path.length >= 2 && c.share > 0), 'each chain has its share and its path');

  // The drawn pixels: the chain is near-black, so the dark pixels round its midpoint are its; the layer off, they are not.
  for (const l of ['units', 'capitals', 'centres', 'ring', 'routes', 'land', 'held', 'clans', 'towns', 'settle', 'hearths', 'sources']) {
    await page.eval(`game.map.setLayer('${l}', false)`);
  }
  await page.eval(frames);
  const around = () =>
    page.eval(`(() => {
      const c = document.getElementById('map');
      const r = c.getBoundingClientRect();
      const a = game.map.screenOf(${O}), b = game.map.screenOf(${capital});
      const s = c.width / r.width;
      const h = Math.round(40 * s);
      const x = Math.round(((a.x + b.x) / 2 - r.left) * s), y = Math.round(((a.y + b.y) / 2 - r.top) * s);
      return Array.from(c.getContext('2d').getImageData(x - h, y - h, 2 * h, 2 * h).data);
    })()`);
  const dark = (px) => {
    let n = 0;
    for (let k = 0; k < px.length; k += 4) if (px[k] < 40 && px[k + 1] < 40 && px[k + 2] < 40) n++;
    return n;
  };
  const on = dark(await around());
  await page.eval("game.map.setLayer('flow', false)");
  await page.eval(frames);
  const off = dark(await around());
  await page.eval("game.map.setLayer('flow', true)");
  await page.eval(frames);
  need(on - off >= 20, `the flow layer draws the chain: ${on - off} more dark pixels round its midpoint with it on`);

  // A second click clears the pick; the salt row picks salt's chains, and Escape clears them.
  await pickRow('Obsidian');
  await page.eval(frames);
  need((await page.data()).good === '' && (await page.eval('game.map.flow')) === null, 'a second click on the Obsidian row clears the chains');
  need(await pickRow('Salt'), 'the store’s Salt row picks');
  await page.eval(frames);
  const saltFlow = await page.eval('game.map.flow');
  need(saltFlow && saltFlow.good === 'salt' && saltFlow.chains.every((c) => c.path[0] === S), 'the picked salt’s chains start on the salt source');
  await page.key('Escape');
  await page.eval(frames);
  need((await page.data()).good === '' && (await page.eval('game.map.flow')) === null, 'Escape clears the picked good');

  await page.shot(join(out, 'exchange.png'), 1);
  console.log(`      the screenshot is ${join(out, 'exchange.png')}`);
} finally {
  await page.close();
}
if (failed) process.exit(1);
