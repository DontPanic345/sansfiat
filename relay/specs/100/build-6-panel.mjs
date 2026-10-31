// Build 6's panel check: presses the selection panel's order buttons that the five plays don't press
// (Raise warrior, Disband 1 and all, Raise 1, the two switches, Resupply, Forage, Pillage, Stop, Paint
// then Undeploy, and the scout's Clear route) in headless Chrome, and checks each one's effect on the
// world. Prints one `ok` line per check and exits 1 on the first failure. Usage (needs engine/www/pkg):
//
//   node relay/specs/100/build-6-panel.mjs [out-dir]     (default: relay/specs/100/panel)
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from '../../../engine/www/play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, 'panel'));
mkdirSync(out, { recursive: true });

const say = (s) => console.log(`ok       ${s}`);
const need = (ok, what) => {
  if (!ok) throw new Error(`FAILED: ${what}`);
  say(what);
};

const page = await open('?size=small&seed=2&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');

  // The led civ, its capital, camp and scout, read fresh from the page's view each time.
  const me = () =>
    page.eval(`(() => {
      const v = game.view(), civ = v.player, c = v.civs[civ];
      const camp = Object.values(v.units).find((u) => u.civ === civ && u.kind === 'camp') ?? null;
      const scout = Object.values(v.units).find((u) => u.civ === civ && u.kind === 'scout') ?? null;
      return { civ, capital: c.capital, camp, scout, territory: camp ? camp.territory : [] };
    })()`);
  const errorText = () => page.eval("document.getElementById('error').textContent");
  const noError = async (what) => {
    const e = await errorText();
    need(e === '', `${what}: the page shows no error${e ? ` (it shows "${e}")` : ''}`);
  };
  const enabled = (label) =>
    page.eval(`[...document.querySelectorAll('button')].some((b) => b.offsetParent && b.textContent.startsWith(${JSON.stringify(label)}) && !b.disabled)`);
  // Clicks the tile until the unit `id` is the selection (a tile can hold a camp and a scout).
  const pick = async (id, tile) => {
    for (let n = 0; n < 4; n++) {
      if (await page.eval(`document.body.dataset.selected === 'unit:${id}'`)) return;
      await page.click(tile);
    }
    throw new Error(`could not select unit ${id} on tile ${tile}`);
  };
  // Steps between two tiles on the hex grid (the page's layout).
  const STEPS = `(a, b) => { const L = game.layout(); const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b]; return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr)); }`;

  let s = await me();
  const cap = s.capital;

  // The civ's panel: Raise warrior, which the store can only pay for once it has grown (at the pick it
  // is greyed), so end turns until it is enabled.
  let waited = 0;
  for (; waited < 80; waited++) {
    await page.select(cap, 'capital');
    if (await enabled('Raise warrior')) break;
    await page.button('End turn');
  }
  need(await enabled('Raise warrior'), `Raise warrior is enabled after ${waited} turns`);
  const before = (await me()).camp.people;
  await page.button('Raise warrior');
  s = await me();
  need(s.camp.people > before, `Raise warrior: the camp takes people (${before} -> ${s.camp.people})`);
  await noError('Raise warrior');

  // The camp's panel, at the capital.
  await pick(s.camp.id, cap);
  const c0 = (await me()).camp;
  await page.button('Disband 1');
  let c1 = (await me()).camp;
  need(c1.people === c0.people - 1, `Disband 1: one warrior goes (${c0.people} -> ${c1.people})`);
  // Raise 1 is greyed until the store can fill the packs: end turns until it is enabled.
  for (let n = 0; n < 80 && !(await enabled('Raise 1')); n++) {
    await page.button('End turn');
    await pick(c0.id, cap);
  }
  await page.button('Raise 1');
  c1 = (await me()).camp;
  need(c1.people === c0.people, `Raise 1: one warrior comes back (${c1.people})`);

  await page.button('Auto-forage: ');
  let c2 = (await me()).camp;
  need(c2.auto_forage !== c0.auto_forage, `Auto-forage switch flips (${c0.auto_forage} -> ${c2.auto_forage})`);
  await page.button('Auto-forage: ');
  await page.button('Auto-supply: ');
  c2 = (await me()).camp;
  need(c2.auto_supply !== c0.auto_supply, `Auto-supply switch flips (${c0.auto_supply} -> ${c2.auto_supply})`);
  await page.button('Auto-supply: ');
  await noError('the switches');

  await page.button('Resupply');
  await noError('Resupply');
  await page.button('Forage');
  await noError('Forage');

  // Undeploy: paint one tile next to the capital into the zone, then undeploy it.
  const around = await page.eval(`(() => {
    const L = game.layout(), cap = game.view().civs[game.view().player].capital;
    const EDGE = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
    return EDGE.map(([dq, dr]) => L.q.findIndex((q, i) => q === L.q[cap] + dq && L.r[i] === L.r[cap] + dr)).filter((i) => i >= 0);
  })()`);
  let painted = null;
  await page.button('Paint');
  for (const t of around) {
    await page.click(t);
    if ((await me()).camp.territory.length > 1) {
      painted = t;
      break;
    }
  }
  await page.button('Paint');
  need(painted != null, `painting a neighbour of the capital grows the zone (${painted})`);
  await pick((await me()).camp.id, cap);
  await page.button('Undeploy');
  need((await me()).camp.territory.length === 1, 'Undeploy leaves the camp its own tile');
  await noError('Undeploy');
  await page.shot(join(out, 'panel-camp.png'), 1);

  // Pillage needs the camp on a rival's land. Send it to the nearest rival tile (Stop it on the way,
  // then send it again), end turns until it is there, and press Pillage.
  const rival = await page.eval(`(() => {
    const v = game.view(), civ = v.player, cap = v.civs[civ].capital;
    const steps = ${STEPS};
    // A rival's capital and the tiles beside it stay the rival's as the turns pass (a border tile may not).
    const tiles = [...game.layout().q.keys()].filter((i) => { const k = game.tileCiv(i); return k >= 0 && k !== civ && steps(i, v.civs[k].capital) <= 1; });
    return tiles.sort((a, b) => steps(a, cap) - steps(b, cap) || a - b).slice(0, 16);
  })()`);
  let goal = null;
  for (const t of rival) {
    await page.command(t);
    if ((await errorText()) === '' && (await me()).camp.dest != null) {
      goal = t;
      break;
    }
  }
  need(goal != null, `the camp can walk to a rival tile (${goal})`);
  await page.button('Stop');
  need((await me()).camp.dest == null, 'Stop clears the camp\'s destination');
  await noError('Stop');
  await page.command(goal);
  for (let n = 0; n < 30 && (await me()).camp.tile !== goal; n++) {
    const camp = (await me()).camp;
    await page.button('End turn');
    await pick(camp.id, camp.tile);
  }
  need((await me()).camp.tile === goal, `the camp arrives on the rival tile ${goal}`);
  const pillageRow = await page.eval(`[...document.querySelectorAll('button')].filter((b) => b.textContent.startsWith('Pillage')).map((b) => b.parentElement.textContent).join(' | ')`);
  need(await enabled('Pillage'), `Pillage is enabled on a rival's land (the row reads: ${pillageRow})`);
  await page.button('Pillage');
  await noError('Pillage');
  await page.shot(join(out, 'panel-pillage.png'), 1);

  // The scout's panel: Start here, Route to here, then Clear route.
  s = await me();
  await pick(s.scout.id, cap);
  // The route's end: a tile three steps out that the engine takes as land (a refused one leaves the
  // arm set, so each try first disarms it).
  const candidates = await page.eval(`(() => {
    const L = game.layout(), cap = game.view().civs[game.view().player].capital;
    const steps = ${STEPS};
    return [...L.q.keys()].filter((i) => steps(i, cap) === 3);
  })()`);
  let target = null;
  for (const t of candidates) {
    const armed = await page.eval("document.body.dataset.armed || ''");
    if (armed) await page.button(armed === 'start' ? 'Start here' : 'Route to here');
    await page.button('Start here');
    await page.click(cap);
    await page.button('Route to here');
    await page.click(t);
    if ((await errorText()) === '' && (await me()).scout.route != null) {
      target = t;
      break;
    }
  }
  need(target != null, `a route can be set three steps out from the capital (${target})`);
  let sc = (await me()).scout;
  need(sc.route[1] === target, `Route to here sets the scout's route to ${target}`);
  await page.button('Clear route');
  sc = (await me()).scout;
  need(sc.route == null, 'Clear route clears the scout\'s route');
  await noError('the route buttons');
  await page.shot(join(out, 'panel-scout.png'), 1);

  const errors = page.errors();
  need(errors.length === 0, `the page throws no errors${errors.length ? `: ${errors.join('; ')}` : ''}`);
  console.log('done');
} finally {
  await page.close();
}
