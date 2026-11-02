// A civ in the page (touch point 20): the bar's chip selects it; its panel has Standing with its parts in the
// tooltip, its store and its claim's counts; Raise… raises warriors and a guard; Gift + and −, and Keep, move
// their numbers; P paints the claim and U unpaints it, and the border and counts show; an unaligned clan
// leans to the civ within 30 turns of a raised gift. On small seed 4: the store fills its packs within a few
// turns there (on seed 2 the camp at the capital draws the store down faster than it fills).
//   node www/play/civ.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-civ-')));
mkdirSync(out, { recursive: true });

let failed = false;
const fail = (msg) => {
  console.error(`civ: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('?size=small&seed=4&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  const civ = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${civ}].capital`);
  const name = await page.eval(`game.view().civs[${civ}].name`);

  // The bar's chip selects the civ: its capital, the panel, and the chip marked.
  await page.click(cap); // a click on the capital first selects the camp (the player's unit comes first)
  await page.eval(`document.querySelector('button.chip[data-civ="${civ}"]').click()`);
  await page.eval(frames);
  need((await page.data()).selected === `civ:${civ}`, 'the bar chip selects the civ');
  need((await page.eval(`document.querySelector('button.chip[data-civ="${civ}"]').getAttribute('aria-pressed')`)) === 'true', 'the selected civ’s chip is pressed');
  let p = await page.panel();
  need(p?.name === name, `the civ's panel is named ${name}`);

  // Standing with its parts in the tooltip, and the claim's counts.
  const standing = p.numbers.find((n) => n.label === 'Standing');
  need(standing && /clans and units/i.test(standing.tip) && /unaligned clans on its land/.test(standing.tip), 'Standing shows its two parts in its tooltip');
  for (const word of ['Claimed', 'Held', 'Paper', 'Thinnest guard']) need(p.numbers.some((n) => n.label === word), `the panel shows ${word}`);

  // Raise…: warriors need the store to fill their packs, so turns end until a warrior is allowed; a guard
  // is raised from the civ's clans.
  const peopleOf = (kind) => page.eval(`Object.values(game.view().units).filter((u) => u.civ === ${civ} && u.kind === '${kind}').reduce((a, u) => a + u.people, 0)`);
  const w0 = await peopleOf('camp');
  let raised = false;
  for (let turn = 0; turn < 12 && !raised; turn++) {
    const r = await page.stepper('Raise…', { n: 1, option: 'warriors', go: true });
    if (r.go) raised = true;
    else await page.button('End turn');
  }
  await page.eval(frames);
  need(raised && (await peopleOf('camp')) > w0, 'Raise… raises warriors for the camp');
  const g0 = await peopleOf('guard');
  const g = await page.stepper('Raise…', { n: 1, option: 'a guard', go: true });
  await page.eval(frames);
  need(g.go && (await peopleOf('guard')) > g0, `Raise… raises a guard (${g.go ? 'done' : g.title})`);

  // Gift and Keep.
  const gift0 = await page.eval(`game.view().civs[${civ}].gift`);
  await page.button('Gift +1');
  need((await page.eval(`game.view().civs[${civ}].gift`)) === gift0 + 1, 'Gift + raises the gift');
  await page.button('Gift −1');
  need((await page.eval(`game.view().civs[${civ}].gift`)) === gift0, 'Gift − lowers it back');
  const keep0 = await page.eval(`game.view().civs[${civ}].keep`);
  await page.button('Keep +1');
  need((await page.eval(`game.view().civs[${civ}].keep`)) > keep0, 'Keep + sets the keep');
  await page.button('Keep default');
  need((await page.eval(`game.view().civs[${civ}].keep_set`)) == null, 'Keep default clears the keep');

  // P paints the claim from the capital's neighbour; the border and the counts show it.
  // A free land tile next to the claim, so the claim stays connected to the capital.
  const nb = await page.eval(`(() => { const m = game.map, c = game.view().civs[${civ}];
    const out = new Set();
    for (const t of c.claim) for (let d = 0; d < 6; d++) { const j = m.nb[t * 6 + d]; if (j >= 0 && m.land[j] && !c.claim.includes(j)) out.add(j); }
    return [...out].sort((a, b) => a - b).slice(0, 1); })()`);
  need(nb.length === 1, 'the claim has a free land neighbour to paint');
  const claimed = () => page.eval(`game.view().civs[${civ}].claim.length`);
  const c0 = await claimed();
  await page.key('p');
  need((await page.eval("document.body.dataset.painting")) === 'claim', 'P arms the claim while the civ is selected');
  await page.click(nb[0]);
  await page.eval(frames);
  need((await claimed()) === c0 + 1, `a click paints the claim (${c0} → ${await claimed()})`);
  const border = await page.eval(`game.border(${civ})`);
  need(!!border, 'the civ’s border is drawn');
  p = await page.panel();
  need(p.numbers.find((n) => n.label === 'Claimed')?.value === String(c0 + 1), 'the panel counts the claim');
  await page.key('Escape');
  await page.key('u');
  need((await page.eval("document.body.dataset.painting")) === 'unclaim', 'U arms the unclaim while the civ is selected');
  await page.click(nb[0]);
  await page.eval(frames);
  need((await claimed()) === c0, 'a click while unclaiming takes the tile out of the claim');
  await page.key('Escape');
  await page.shot(join(out, 'civ.png'), 1);

  // A raised gift draws an unaligned clan to lean to the civ: the leanings to it grow within 30 turns.
  const leaning = () => page.eval(`Object.values(game.view().clans).filter((k) => k.civ == null).reduce((a, k) => a + (k.leaning[${civ}] ?? 0), 0)`);
  const before = await leaning();
  for (let i = 0; i < 3; i++) await page.button('Gift +1');
  let turns = 0;
  while ((await leaning()) <= before && turns < 30) {
    await page.button('End turn');
    turns++;
  }
  need((await leaning()) > before, `a raised gift makes an unaligned clan lean to the civ within 30 turns (${turns} turns)`);
  // The leaning clans are rows in the civ's panel: a row selects its clan.
  await page.eval(`game.map.centreOn(${cap}, { minK: 14, smooth: false })`);
  await page.select(cap, 'civ');
  const row = await page.eval("[...document.querySelectorAll('#detail .listrow button')].length");
  if (row > 0) {
    await page.eval("[...document.querySelectorAll('#detail .listrow button')][0].click()");
    need(/^clan:\d+$/.test(await page.eval('document.body.dataset.selected')), 'a leaning clan’s row selects the clan');
  }
  await page.shot(join(out, 'civ-leaning.png'), 1);
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
