// Plays a civ's claim in the page, headless: leads the first civ on small seed 4, paints its claim out
// from the capital (the capital's panel's Claim button, then a click on each tile), raises three guards,
// ends ten turns, and screenshots the civ's border and the tints, then a festival's walkers. Exits
// non-zero if the civ's land border or the claim's counts are not there. Usage:
//
//   node www/play-claim.mjs [out-dir]     (default: relay/specs/102/screens; needs www/pkg)
//
// Uses play.mjs's driver. Prints what the civ did; exits non-zero on a page error too.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, '../../relay/specs/102/screens'));
mkdirSync(out, { recursive: true });
const FAR = 9; // steps out of the far tile the claim paints to: past what three guards hold, so it is paper
const GUARDS = 3; // guards raised, one person each: one person holds only the capital
const TURNS = 10;
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('?size=small&seed=4&run=1', { height: 1000 });
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  const civ = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${civ}].capital`);
  await page.select(cap, 'capital');

  // The tiles to paint: the walks from the capital to the nearest unaligned clan's home within six
  // steps (the clan leans to the civ once its home is held) and to the nearest unclaimed land three
  // steps out, their unclaimed tiles in order of distance, so each is joined to the claim by the one
  // before it.
  const before = await page.eval(`game.view().civs[${civ}].claim.length`);
  const plan = await page.eval(`(() => {
    const v = game.view(), m = game.map, cap = v.civs[${civ}].capital;
    const claim = new Set(v.civs[${civ}].claim);
    const prev = new Map([[cap, -1]]);
    const queue = [cap];
    for (let k = 0; k < queue.length; k++) {
      const i = queue[k];
      for (let d = 0; d < 6; d++) {
        const j = m.nb[i * 6 + d];
        if (j < 0 || prev.has(j) || !m.land[j]) continue;
        prev.set(j, i);
        queue.push(j);
      }
    }
    const walkTo = (t) => { const w = []; for (let x = t; x !== cap; x = prev.get(x)) w.unshift(x); return w; };
    const unpainted = (w) => w.filter((t) => !claim.has(t));
    const homes = Object.values(v.clans).filter((k) => k.civ == null && prev.has(k.home)).sort((a, b) => walkTo(a.home).length - walkTo(b.home).length);
    const home = homes.find((k) => walkTo(k.home).length <= 6 && unpainted(walkTo(k.home)).length > 0);
    const far = queue.find((t) => t !== home?.home && !claim.has(t) && walkTo(t).length >= ${FAR});
    const targets = [home?.home, far].filter((t) => t !== undefined);
    const tiles = new Set(targets.flatMap((t) => unpainted(walkTo(t))));
    const walk = [...tiles].sort((a, b) => walkTo(a).length - walkTo(b).length || a - b);
    return { walk, home: home?.home ?? null };
  })()`);
  if (!plan || plan.walk.length === 0) throw new Error('no unclaimed land to paint out from the capital');

  // Paint: the Claim button arms the capital's claim paint, then a click on each tile paints it. The map
  // settles after each order (the panel and the canvas resize), so a click waits two frames and is
  // repeated until its tile is in the claim.
  await page.button('Claim (paint)');
  if ((await page.eval('document.body.dataset.painting')) !== 'claim') throw new Error('the Claim button did not arm painting');
  for (const t of plan.walk) {
    for (let tries = 0; ; tries++) {
      await page.eval(frames);
      await page.click(t);
      await page.eval(frames);
      if (await page.eval(`game.view().civs[${civ}].claim.includes(${t})`)) break;
      if (tries === 3) throw new Error(`tile ${t} would not paint: ${await page.eval("document.getElementById('error').textContent")}`);
    }
  }
  await page.eval(`document.querySelector('[data-paint="claim"]').click()`); // off again
  const painted = await page.eval(`game.view().civs[${civ}].claim.length`);
  if (painted !== before + plan.walk.length) throw new Error(`the claim went from ${before} to ${painted} tiles, not ${before + plan.walk.length}`);

  // The guard: raised from the civ's clans onto its claim, at the capital.
  await page.select(cap, 'capital');
  for (let k = 0; k < GUARDS; k++) await page.button('Raise guard');
  const guards = await page.eval(`Object.values(game.view().units).filter((u) => u.civ === ${civ} && u.kind === 'guard').map((u) => u.people)`);
  if (guards.length !== 1 || !(guards[0] > 0)) throw new Error(`the guard is not there: ${JSON.stringify(guards)}`);
  const heldRaised = await page.eval(`game.view().civs[${civ}].land.length`);
  await page.eval(`game.map.centreOn(${cap}, { minK: 14, smooth: false })`);
  await page.shot(join(out, 'raised.png'), 0.6);

  // Ten turns. The guard eats from the civ's store, which is short, so it may starve on the way.
  for (let turn = 0; turn < TURNS; turn++) await page.button('End turn');
  await page.select(cap, 'capital');
  await page.eval(`game.map.centreOn(${cap}, { minK: 22, smooth: false })`);

  // The civ's border and the claim's counts: the border is drawn (held tiles, paper), and the panel reads
  // the counts.
  const border = await page.eval(`game.border(${civ})`);
  const counts = await page.eval(`document.getElementById('detail').textContent`);
  if (!border || border.held < 1) throw new Error(`the civ's land border is not drawn: ${JSON.stringify(border)}`);
  for (const word of ['claim ', 'held ', 'paper ', 'thinnest held guard']) {
    if (!counts.includes(word)) throw new Error(`the civ's panel does not show "${word}"`);
  }
  await page.shot(join(out, 'border.png'), 0.6);

  // The tints: the unaligned clans that lean to a civ, and the civ each leans to most.
  const leaning = await page.eval(`Object.values(game.view().clans).filter((k) => k.civ == null && Object.keys(k.leaning).length > 0)
    .map((k) => ({ clan: k.id, home: k.home, to: Object.entries(k.leaning).sort(([, a], [, b]) => b - a)[0][0] }))`);
  if (leaning.length) {
    await page.eval(`game.map.centreOn(${leaning[0].home}, { minK: 22, smooth: false })`);
  }
  await page.shot(join(out, 'tints.png'), 0.6);

  // A festival selected (the one with the most clans going to it, not the civ's own at its capital): the
  // clans going to it are listed and their walks drawn bright.
  const at = await page.eval(`(() => { const ms = Object.values(game.view().markets).sort((a, b) => b.clans.length - a.clans.length || a.id - b.id);
    return ms.length ? ms[0].tile : null; })()`);
  if (at !== null) {
    await page.select(at, 'tile');
    await page.eval(`game.map.centreOn(${at}, { minK: 14, smooth: false })`);
    await page.shot(join(out, 'festival.png'), 0.6);
  }

  const state = await page.eval(`(() => { const v = game.view(), c = v.civs[${civ}];
    return { turn: v.turn, claim: c.claim.length, held: c.land.length, guard_people: Object.values(v.units).filter((u) => u.civ === ${civ} && u.kind === 'guard').reduce((a, u) => a + u.people, 0) }; })()`);
  console.log(JSON.stringify({ civ, painted: plan.walk.length, home_walk: plan.home, held_raised: heldRaised, ...state, border,
    leaning_unaligned: leaning, festival_tile: at }));
  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length) process.exitCode = 1;
} finally {
  await page.close();
}
