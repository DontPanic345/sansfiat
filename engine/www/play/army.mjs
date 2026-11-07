// The armies (build-4): on small seed 3, Blue (the civ that holds forest, and whose corridor meets Gold's guard
// from the first turn), the page raises warriors as a new army through the civ's panel, raises archers into it
// through the army's panel, sees its number on the map, marches it with Move to… onto a rival's guarded tile, and
// reads the fight in the feed and the army's reasons. Gift 0 and the turns that fill the store come first.
// Exits non-zero when any of these is missing; writes army-*.png to --out.   node www/play/army.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open, lead } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-army-')));
mkdirSync(out, { recursive: true });

const SEED = 3;
const SIZE = 'small';
const CIV = 'Blue';
const TURNS = 80;
let failed = false;
const fail = (msg) => {
  console.error(`army: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

/** Clicks tile `i` until the selection is `key` (a click on a stack moves to its next thing). */
async function selectKey(page, key, i) {
  for (let n = 0; n < 8; n++) {
    if ((await page.data()).selected === key) return;
    await page.click(i);
    await page.eval(frames);
  }
  throw new Error(`clicking tile ${i} never selected ${key}`);
}

/** The button in the panel, by its label (`panel()` lists them, greyed or not). */
const buttonOf = async (page, label) => (await page.panel())?.buttons.find((b) => b.label === label) ?? null;

const page = await open('');
try {
  await lead(page, { seed: SEED, size: SIZE, civ: CIV });
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  const civKey = `civ:${me}`;

  // The guard holds the land (a civ with none holds its capital alone): three guards, raised through the civ's Raise….
  await selectKey(page, civKey, cap);
  for (let g = 0; g < 3; g++) await page.stepper('Raise…', { n: 1, option: 'a guard', go: true });
  // The led civ holds forest (archers are made on it), and its corridor meets a rival's guard from the first turn.
  const forest = await page.eval(`(() => { const v = game.view(), L = game.layout(), T = game.terrain();
    return v.civs[${me}].land.filter((i) => T[L.terrain[i]].name === 'forest').length; })()`);
  need(forest > 0, `the led civ holds ${forest} forest tiles`);

  // Gift 0: the store fills from the levy and nothing is given away.
  await selectKey(page, civKey, cap);
  for (let k = 0; k < 12 && (await page.eval(`game.view().civs[${me}].gift`)) > 0; k++) await page.button('Gift −1');
  need((await page.eval(`game.view().civs[${me}].gift`)) === 0, 'the gift is set to 0');

  // End turns until the civ's Raise… warriors is allowed (its count's Go, not greyed): the store fills a pack.
  let turns = 0;
  let offer = null;
  for (; turns < TURNS; turns++) {
    await selectKey(page, civKey, cap);
    offer = await page.stepper('Raise…', { n: 1, option: 'warriors' });
    if (offer.go) break;
    await page.button('End turn');
  }
  const store = await page.eval(`game.view().civs[${me}].store.reduce((a, b) => a + b, 0)`);
  need(offer?.go === true, `Raise… warriors is no longer greyed after ${turns} turns (the store holds ${store.toFixed(1)} food)`);
  need(['warriors', 'archers', 'a guard', 'a scout'].every((o) => (offer?.options ?? []).includes(o)), `the civ's Raise… offers ${(offer?.options ?? []).join(', ')}`);

  // Raise one warrior as a new army through the civ's panel.
  await selectKey(page, civKey, cap);
  const before = await page.eval(`Object.values(game.view().units).filter((u) => u.civ === ${me} && u.kind === 'camp').map((u) => u.id)`);
  const civRaise = await page.stepper('Raise…', { n: 1, option: 'warriors', go: true });
  need(civRaise.go === true, `the civ's Raise… raises warriors${civRaise.go ? '' : ` (greyed: ${civRaise.title})`}`);
  await page.eval(frames);
  const army = await page.eval(`(() => { const ids = ${JSON.stringify(before)};
    const u = Object.values(game.view().units).filter((u) => u.civ === ${me} && u.kind === 'camp' && !ids.includes(u.id));
    return u[0] ? { id: u[0].id, tile: u[0].tile } : null; })()`);
  need(army && army.tile === cap, 'the warriors make a new army at the capital');
  if (!army) throw new Error('no new army was raised');
  const armyKey = `unit:${army.id}`;

  // Raise archers into the army through the army's own panel, once its store fills a pack.
  await selectKey(page, armyKey, army.tile);
  let p = await page.panel();
  need(/'s army \d+$/.test(p.name ?? ''), `the army's panel is named "${p.name}"`);
  let archers = null;
  for (let k = 0; k < TURNS; k++) {
    await selectKey(page, armyKey, army.tile);
    archers = await page.stepper('Raise…', { n: 1, option: 'archers' });
    if (archers.go) break;
    if (k === 0) console.log(`        Raise archers is greyed: ${archers.title ?? 'no count'}`);
    await page.button('End turn');
  }
  need(archers?.go === true, 'the army’s Raise… archers is no longer greyed (its store fills a pack)');
  await selectKey(page, armyKey, army.tile);
  await page.stepper('Raise…', { n: 1, option: 'archers', go: true });
  await page.eval(frames);
  need((await page.panel()).doing?.includes('Making bows'), 'the army says it is making bows the turn it is raised');
  const stack = await page.eval(`game.view().units[${army.id}].stack`);
  need(stack[1] >= 1 - 1e-9, `the army holds ${stack[1].toFixed(1)} archers and ${stack[0].toFixed(1)} warriors`);
  p = await page.panel();
  need(p.numbers.some((n) => n.label === 'Archers') && p.numbers.some((n) => n.label === 'Attack') && p.numbers.some((n) => n.label === 'Defence'), 'the army’s panel gives its archers, attack and defence');
  need((await buttonOf(page, 'Disband…')) != null, 'the army’s panel has Disband…');
  const disbandOpts = (await page.stepper('Disband…', { n: 1 })).options;
  need(disbandOpts.includes('archers') && disbandOpts.includes('warriors') && !disbandOpts.includes('all'), `Disband… offers ${disbandOpts.join(', ')}`);
  need((await buttonOf(page, 'Disband all')) != null, 'the army’s panel has Disband all');

  // The stack's number on the map: the army's label as the units layer drew it.
  await page.eval(frames);
  const drawn = await page.eval(`(async () => { const m = await import('./map/layers/units.js');
    const u = game.view().units[${army.id}];
    const label = m.countLabel(u);
    const seen = game.map.stats.labels.find((l) => l.unit === ${army.id});
    return { label, seen: seen ? seen.label : null }; })()`);
  need(drawn.seen === drawn.label && drawn.label.includes('+'), `the map draws the army's number as "${drawn.seen}"`);
  await page.shot(join(out, 'army-raised.png'), 1);

  // March: Move to… onto a rival's guarded tile: a contested one if the corridors meet one, else the nearest one held.
  const target = await page.eval(`(() => { const v = game.view(), L = game.layout(), me = ${me};
    const u = v.units[${army.id}];
    const d = (a, b) => { const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b]; return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr)); };
    const rival = (t) => Object.values(v.civs).some((c) => c.id !== me && c.land.includes(t));
    const contested = v.contested.filter(rival);
    const pool = contested.length ? contested : Object.values(v.civs).filter((c) => c.id !== me).flatMap((c) => c.land);
    pool.sort((a, b) => d(u.tile, a) - d(u.tile, b) || a - b);
    return pool[0] ?? null; })()`);
  need(target != null, 'a rival holds a tile the army can march onto');
  if (target == null) throw new Error('no rival tile to march to');
  await selectKey(page, armyKey, army.tile);
  await page.button('Move to…');
  await page.click(target);
  await page.eval(frames);
  const dest = await page.eval(`game.view().units[${army.id}].dest`);
  need(dest === target, `the army marches to (${target}) by Move to…`);

  // End turns until the army fights on that tile: a skirmish in the feed, with the army in it.
  const marched = await page.eval('game.view().turn');
  let skirmish = null;
  for (let k = 0; k < 60 && !skirmish; k++) {
    await page.button('End turn');
    skirmish = await page.eval(`(() => game.feed.items.find((it) => it.kind === 'skirmish' && it.unit === ${army.id}) ?? null)()`);
  }
  need(skirmish != null && skirmish.turn > marched, `a skirmish with the army is in the feed (${skirmish ? `turn ${skirmish.turn}: ${skirmish.text}` : 'none'})`);
  if (skirmish) {
    // Click it in the feed: the army is selected, and its panel says it is fighting and why.
    await page.eval(`(() => { const li = [...document.querySelectorAll('#feed li')].find((l) => Number(l.dataset.id) === ${skirmish.id});
      if (li) li.click(); })()`);
    await page.eval(frames);
    need((await page.data()).selected === armyKey, 'a click on the skirmish selects the army');
    p = await page.panel();
    need(p.doing?.includes('Fighting at'), `the army's panel says it is fighting ("${p.doing}")`);
    need(p.reasons.some((r) => r.includes('fought')), `the army's reasons give the fight ("${p.reasons[0] ?? 'none'}")`);
    await page.shot(join(out, 'army-fight.png'), 1);
  }

  // The civ's reasons carry the fight too (its guard fought), and its panel shows its posture.
  await selectKey(page, civKey, cap);
  p = await page.panel();
  need(p.reasons.some((r) => /fought|took/.test(r)), `the civ's reasons give the fight or the tile it took ("${p.reasons[0] ?? 'none'}")`);
  await page.shot(join(out, 'army-civ.png'), 1);

  // The feed with the skirmish in it (the feed is shut at laptop width unless opened).
  if (!(await page.eval("!!document.querySelector('#feed')?.offsetParent"))) await page.button('Feed');
  await page.eval(frames);
  await page.shot(join(out, 'army-feed.png'), 1);

  for (const e of page.errors()) fail(`page error: ${e}`);
  console.log(JSON.stringify({ seed: SEED, size: SIZE, civ: CIV, turns, raisedAt: army.tile, army: army.id, target, skirmish: skirmish?.turn ?? null }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
