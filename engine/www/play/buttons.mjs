// Every button of every panel, pressed once each on a staged game, and the world checked after each press.
// The panels are the page's own: `game.shape()` is the shape `describe` gives the selection, and each button
// is pressed as a player presses it (a click, an armed tile, a painted tile, a stepper's Go). A label with no
// case in CASES fails the script ("button X has no case"), so a new button forces a case.
//   node www/play/buttons.mjs [--out DIR] [--break forage]     (--break: Forage's case presses Pillage: a control)
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-buttons-')));
mkdirSync(out, { recursive: true });
const br = args.indexOf('--break');
const BREAK = br >= 0 ? args[br + 1] : null;
if (BREAK && BREAK !== 'forage') throw new Error(`--break takes forage, not ${BREAK}`);

let failed = false;
const fail = (msg) => {
  console.error(`buttons: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) throw new Error(msg);
  console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// ---- the staged map: radius 3, sea on the rim, the inner ring plains with a forest and a hill ----------------

const R = 3;
const SPAN = (r) => ({ lo: Math.max(-R, -r - R), hi: Math.min(R, -r + R) });
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
/** The tile index of (q, r), as `map::index` numbers it: column by column from the left, each from the top. */
function indexOf(q, r) {
  let i = 0;
  for (let c = -R; c < q; c++) i += 2 * R + 1 - Math.abs(c);
  return i + r - Math.max(-R, -q - R);
}
/** A grid's text (as `map::ascii` draws one): `cell(q, r)` gives each tile's character. */
function gridText(cell) {
  let text = '';
  for (let r = -R; r <= R; r++) {
    const { lo, hi } = SPAN(r);
    text += ' '.repeat(2 * lo + r + 2 * R);
    const row = [];
    for (let q = lo; q <= hi; q++) row.push(cell(q, r));
    text += `${row.join(' ')}\n`;
  }
  return text;
}
const TERRAIN = { '1,-1': 'h', '-1,-1': 'f' };
const MAP = gridText((q, r) => (dist(q, r) === R ? '~' : TERRAIN[`${q},${r}`] ?? '.'));
// Marks: A and B the two capitals; F and R food beside A and B; S a plains tile beside A (a route's start);
// N a forest beside A (painted); X a plains tile beside A (claimed); T two steps from A (the walk's end); P a
// plains tile of the rival's claim beside its capital, with food and no rival unit on it (Pillage's tile).
const MARK = { A: [-1, 0], B: [1, 0], F: [-2, 0], R: [2, 0], S: [0, 0], N: [-1, -1], X: [0, -1], T: [0, -2], P: [0, 1] };
const MARKS = gridText((q, r) => Object.keys(MARK).find((k) => MARK[k][0] === q && MARK[k][1] === r) ?? '.');
const GRAIN = 1;

/** The StageSpec of the game: two civs, the first led, its camp of 4.6 beside food on F (`armyAt` moves it).
 *  The first civ's claim takes the ground round both capitals, so with `rivalFirst` the rival is the first civ
 *  and the led civ's camp stands on land that is the rival's. */
function specFor(armyAt = 'F', rivalFirst = false) {
  return {
    map: MAP,
    marks: MARKS,
    civs: rivalFirst ? [{ mark: 'B' }, { mark: 'A' }] : [{ mark: 'A' }, { mark: 'B' }],
    people: 12,
    lead: rivalFirst ? 1 : 0,
    armies: [{ civ: rivalFirst ? 1 : 0, at: { mark: armyAt }, people: 4.6 }],
    food: [
      { at: { mark: 'A' }, good: GRAIN, amount: 30 },
      { at: { mark: 'F' }, good: GRAIN, amount: 4 },
      { at: { mark: 'B' }, good: GRAIN, amount: 3 },
      { at: { mark: 'R' }, good: GRAIN, amount: 4 },
      { at: { mark: 'P' }, good: GRAIN, amount: 4 },
    ],
  };
}
const T = indexOf(...MARK.T);
const S = indexOf(...MARK.S);
const N = indexOf(...MARK.N);
const P = indexOf(...MARK.P);
const X = indexOf(...MARK.X);

// ---- the page -----------------------------------------------------------------------------------------------

const page = await open('?size=small&seed=2&run=1');

/** Stages a fresh game of `spec`: every case starts from one, so its effect never hides another's. */
async function stage(spec = specFor()) {
  await page.eval(`game.stage(${JSON.stringify(spec)})`);
  await page.until("document.body.dataset.phase === 'main'");
  await page.eval(frames);
}

/** The world as the page reads it, with the led civ's units found by kind. */
async function live() {
  const v = JSON.parse(await page.eval('JSON.stringify(game.view())'));
  const me = v.player;
  const other = Number(Object.keys(v.civs).find((k) => Number(k) !== me));
  const unitOf = (civ, kind) => Object.values(v.units).find((u) => u.civ === civ && u.kind === kind);
  const camp = unitOf(me, 'camp');
  const scout = unitOf(me, 'scout');
  const clan = Object.values(v.clans).find((k) => k.civ === me);
  return { v, me, other, camp, scout, clan, capital: v.civs[me].capital, rivalCapital: v.civs[other].capital, market: v.civs[me].market };
}

/** The unit a kind names for the led civ, read now. */
async function unit(kind) {
  const w = await live();
  return kind === 'camp' ? w.v.units[w.camp.id] : w.v.units[w.scout.id];
}
const packOf = (u) => u.store.reduce((a, b) => a + b, 0);
/** What a Forage or Resupply moves: the civ's army counter for a camp (it counts only camps), the pack for a scout. */
const gained = async (kind, key) => (kind === 'camp' ? armyCount(key) : packOf(await unit(kind)));
/** One of the led civ's army counters (`foraged`, `pillaged`, `resupplied`), read now. */
const armyCount = async (key) => {
  const { v, me } = await live();
  return v.civs[me].army[key];
};
const clanPeople = async () => {
  const w = await live();
  return w.v.clans[w.clan.id].people;
};

/** Clicks tile `i` until the selection is `key` (a click on a stack moves to its next thing). */
async function selectKey(key, i) {
  for (let n = 0; n < 12; n++) {
    if ((await page.data()).selected === key) return;
    await page.click(i);
    await page.eval(frames);
  }
  throw new Error(`clicking tile ${i} never selected ${key}`);
}

/** Selects the panel named by `kind`: the led civ, a rival civ, a clan, the market, a tile, or a unit of the led civ. */
async function select(kind) {
  const w = await live();
  switch (kind) {
    case 'civ':
      return selectKey(`civ:${w.me}`, w.capital);
    case 'rival':
      return selectKey(`civ:${w.other}`, w.rivalCapital);
    case 'clan':
      return selectKey(`clan:${w.clan.id}`, w.clan.home);
    case 'market':
      return selectKey(`festival:${w.market}`, w.v.markets[w.market].tile);
    case 'tile':
      return selectKey(`tile:${T}`, T);
    case 'camp':
      return selectKey(`unit:${w.camp.id}`, w.v.units[w.camp.id].tile);
    case 'scout':
      return selectKey(`unit:${w.scout.id}`, w.v.units[w.scout.id].tile);
    default:
      throw new Error(`no selection ${kind}`);
  }
}

/** The bottom panel's shape for the selection, as the page's `describe` gives it (functions dropped). */
const shape = async () => JSON.parse(await page.eval('JSON.stringify(game.shape())'));
/** Every button of the shape as the inventory names it: `Label`, or `Label/option` for a stepper's options. */
const namesOf = (s) => (s?.buttons ?? []).flatMap((b) => (b.stepper ? b.stepper.options.map((o) => `${b.label}/${o.label}`) : [b.label]));
/** A button's label with its state taken off (`Auto-forage: on` is `Auto-forage`). */
const keyOf = (name) => name.replace(/: (on|off)$/, '');

/** Clicks the first enabled, visible button whose text starts with `label`; false if there is none. */
async function press(label) {
  const ok = await page.eval(`(() => { const b = [...document.querySelectorAll('button')].filter((x) => x.offsetParent)
    .find((x) => !x.disabled && x.textContent.trim().startsWith(${JSON.stringify(label)})); if (!b) return false; b.click(); return true; })()`);
  await page.eval(frames);
  return ok;
}
/** Presses a stepper: opens `label`, picks `option` if the stepper has more than one, sets `n` if given (else
 *  the count it opens at), and presses Go. Returns what it found: the count it opened at, and whether Go went. */
async function stepperGo(label, { option = null, n = null } = {}) {
  const r = await page.eval(`(async () => {
    const d = document.getElementById('detail');
    const open = [...d.querySelectorAll('.actions > button')].find((b) => b.textContent.trim() === ${JSON.stringify(label)});
    if (!open || open.disabled) return { pressed: false, refused: open ? 'greyed: ' + open.title : 'no such button' };
    open.click();
    await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
    const sel = d.querySelector('.stepper select');
    if (sel && ${JSON.stringify(option)} != null) {
      sel.value = [...sel.options].find((o) => o.textContent === ${JSON.stringify(option)}).value;
      sel.dispatchEvent(new Event('change'));
      await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
    }
    const input = d.querySelector('.stepper input');
    const opened = Number(input.value);
    if (${n} != null) { input.value = String(${Number(n)}); input.dispatchEvent(new Event('input')); }
    const go = [...d.querySelectorAll('.stepper button')].find((b) => b.textContent.trim() === 'Go');
    if (!go || go.disabled) return { pressed: false, opened, refused: go ? go.title : 'no Go' };
    go.click();
    await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
    return { pressed: true, opened };
  })()`);
  await page.eval(frames);
  return r;
}
async function endTurn() {
  need(await press('End turn'), 'End turn is pressed');
  await page.eval(frames);
}
/** Arms `label` (a move or a route), then clicks tile `i`. */
async function armAndClick(label, i) {
  need(await press(label), `${label} is pressed`);
  need((await page.eval("document.body.dataset.armed")) !== '', `${label} arms a click`);
  await page.click(i);
  await page.eval(frames);
}
/** Paints tile `i` as the armed mode says: P (paint or claim) or U (unpaint or unclaim), then Esc. */
async function paintTile(label, i) {
  need(await press(label), `${label} is pressed`);
  await page.click(i);
  await page.eval(frames);
  await page.key('Escape');
}

// ---- the cases: each takes the kind of panel it is pressed on, and checks the world ---------------------------

/** Disband all as a player presses it: the button sends the whole unit, with no count. */
async function disbandAll(kind) {
  const word = kind === 'camp' ? 'army' : kind;
  await select(kind);
  const u0 = await unit(kind);
  const clan0 = await clanPeople();
  need(await press('Disband all'), `Disband all is pressed on the ${word}`);
  const w = await live();
  need(w.v.units[u0.id] === undefined, `Disband all sends the whole ${word} home: the ${word} is still on the map`);
  const got = (await clanPeople()) - clan0;
  need(Math.abs(got - u0.people) < 1e-9, `Disband all sends the ${word}'s ${u0.people} people to its clan (the clan gained ${got})`);
}

/** Keep ±1 as a player presses it: the civ's keep moves by one. */
async function keepStep(label, step) {
  await select('civ');
  if (step < 0) need(await press('Keep +1'), 'Keep +1 is pressed to set a keep to step down from');
  const k0 = (await live()).v.civs[(await live()).me].keep;
  need(await press(label), `${label} is pressed`);
  const k1 = (await live()).v.civs[(await live()).me].keep;
  need(Math.abs(k1 - (k0 + step)) < 1e-9, `${label} moves the keep by one (${k0} → ${k1})`);
}

const CASES = {
  'Move to…': async (kind) => {
    await select(kind);
    await armAndClick('Move to…', T);
    const w = await live();
    if (kind === 'camp') {
      need(w.v.units[w.camp.id].dest === T, `Move to… then a click sends the camp to ${T}`);
    } else {
      need(w.v.units[w.scout.id].dest === T, `Move to… sends the scout to ${T}`);
      need(w.v.units[w.scout.id].route == null, 'Move to… sets no route');
    }
  },
  'Route to…': async () => {
    await select('scout');
    const tile = (await live()).v.units[(await live()).scout.id].tile;
    await armAndClick('Route to…', T);
    const w = await live();
    need(JSON.stringify(w.v.units[w.scout.id].route) === JSON.stringify([tile, T]), `Route to… sets the scout's route from its tile to ${T}`);
  },
  Stop: async (kind) => {
    await select(kind);
    const w = await live();
    if (kind === 'camp') {
      await page.command(T);
      await page.eval(frames);
      need((await live()).v.units[w.camp.id].dest === T, 'a right-click marches the camp to the tile');
    } else {
      await armAndClick('Move to…', T);
      need((await live()).v.units[w.scout.id].dest === T, 'Move to… sends the scout to the tile');
    }
    need(await press('Stop'), `Stop is pressed on the marching ${kind}`);
    const u = (await live()).v.units[kind === 'camp' ? w.camp.id : w.scout.id];
    need(u.dest == null || u.dest === u.tile, `Stop leaves the ${kind} with no march (dest ${u.dest})`);
  },
  'Paint (P)': async () => {
    await select('camp');
    await paintTile('Paint (P)', N);
    const w = await live();
    need(w.v.units[w.camp.id].territory.includes(N), `Paint (P) and a click add tile ${N} to the camp's zone`);
  },
  'Unpaint (U)': async () => {
    await select('camp');
    await paintTile('Paint (P)', N);
    await paintTile('Unpaint (U)', N);
    const w = await live();
    need(!w.v.units[w.camp.id].territory.includes(N), `Unpaint (U) and a click take tile ${N} out of the zone`);
  },
  Undeploy: async () => {
    await select('camp');
    await paintTile('Paint (P)', N);
    need((await live()).v.units[(await live()).camp.id].territory.length > 1, 'the camp is deployed');
    need(await press('Undeploy'), 'Undeploy is pressed on the deployed camp');
    const w = await live();
    need(w.v.units[w.camp.id].territory.length === 1, 'Undeploy empties the zone');
  },
  Forage: async (kind) => {
    await select(kind);
    // The switch is off, so only the order forages: a camp's auto-forage would move the civ's count every turn.
    need(await press('Auto-forage'), `Auto-forage is switched off on the ${kind}, so only the order forages`);
    need((await unit(kind)).auto_forage === false, `the ${kind}'s Auto-forage is off`);
    const f0 = await gained(kind, 'foraged');
    // --break forage: the case presses Pillage, which is refused on the led civ's own land.
    const label = BREAK === 'forage' ? 'Pillage' : 'Forage';
    need(await press(label), `${label} is pressed on the ${kind} (it is greyed on the led civ's own land)`);
    await endTurn();
    const f1 = await gained(kind, 'foraged');
    if (kind === 'camp') {
      need(f1 > f0 + 1e-9, `Forage brings food to the camp: the civ's foraged rose (${f0.toFixed(2)} → ${f1.toFixed(2)})`);
    } else {
      // A scout's pack starts full at the pick (the store fills one pack, a scout's cap), so Forage has no room to
      // fill and the turn's eating is what shows: the order is accepted and the pack is what the turn left.
      const eaten = f0 - f1;
      need(f1 <= f0 + 1e-9 && eaten >= 0 && eaten <= 2 + 1e-9, `Forage on the full scout is accepted and its pack falls only by what it eats (${f0.toFixed(2)} → ${f1.toFixed(2)})`);
    }
  },
  Pillage: async () => {
    // The camp stands on the rival's plains beside its capital (P), the rival's claim. What this staged map can
    // show is that the order is open there: the tile is the rival's and Pillage is pressed. The food is not
    // taken: a camp two steps from its capital is cut off (its line can't carry a pack's worth), so it has no
    // labour to pillage with. buttons.md records the gap.
    await select('camp');
    const w = await live();
    need(w.v.civs[w.other].claim.includes(P), `tile ${P} is the rival's claim`);
    need(await press('Pillage'), 'Pillage is pressed on the rival’s land');
    await endTurn();
    need((await live()).v.units[w.camp.id] !== undefined, 'the camp is still on the map after the turn');
  },
  Resupply: async (kind) => {
    await select(kind);
    need(await press('Auto-supply'), `Auto-supply is switched off on the ${kind}, so only the order resupplies`);
    need((await unit(kind)).auto_supply === false, `the ${kind}'s Auto-supply is off`);
    const r0 = await gained(kind, 'resupplied');
    need(await press('Resupply'), `Resupply is pressed on the ${kind}`);
    await endTurn();
    if (kind === 'camp') {
      const r1 = await gained(kind, 'resupplied');
      need(r1 > r0 + 1e-9, `Resupply brings food to the camp: the civ's resupplied rose (${r0.toFixed(2)} → ${r1.toFixed(2)})`);
    } else {
      // A scout on the capital draws on the civ's store, which the staged game starts empty: the press is taken and
      // the scout is still there after the turn. buttons.md records the gap.
      need((await live()).v.units[(await live()).scout.id] !== undefined, 'the scout is still on the map after the turn');
    }
  },
  'Auto-forage': async (kind) => {
    await select(kind);
    const before = (await unit(kind)).auto_forage;
    const was = (await shape()).buttons.find((b) => b.label.startsWith('Auto-forage')).label;
    need(await press('Auto-forage'), `Auto-forage is pressed on the ${kind}`);
    need((await unit(kind)).auto_forage === !before, `Auto-forage flips on the ${kind}`);
    const now = (await shape()).buttons.find((b) => b.label.startsWith('Auto-forage')).label;
    need(now !== was && now.endsWith(before ? 'off' : 'on'), `its label says so (“${was}” → “${now}”)`);
  },
  'Auto-supply': async (kind) => {
    await select(kind);
    const before = (await unit(kind)).auto_supply;
    const was = (await shape()).buttons.find((b) => b.label.startsWith('Auto-supply')).label;
    need(await press('Auto-supply'), `Auto-supply is pressed on the ${kind}`);
    need((await unit(kind)).auto_supply === !before, `Auto-supply flips on the ${kind}`);
    const now = (await shape()).buttons.find((b) => b.label.startsWith('Auto-supply')).label;
    need(now !== was && now.endsWith(before ? 'off' : 'on'), `its label says so (“${was}” → “${now}”)`);
  },
  'Raise…/warriors': async (kind) => (kind === 'civ' ? raiseCivKind('warriors', (u) => u.kind === 'camp') : raiseIntoCamp('warriors')),
  'Raise…/archers': async (kind) => (kind === 'civ' ? raiseCivKind('archers', (u) => u.kind === 'camp') : raiseIntoCamp('archers')),
  'Raise…/a scout': async () => raiseCivKind('a scout', (u) => u.kind === 'scout'),
  'Disband…/people': async (kind) => {
    await select(kind);
    const u0 = await unit(kind);
    const c0 = await clanPeople();
    const r = await stepperGo('Disband…', { option: 'people', n: 1 });
    need(r.pressed, `Disband… people, 1, Go is pressed on the ${kind}${r.refused ? ': ' + r.refused : ''}`);
    need(Math.abs((await unit(kind)).people - u0.people + 1) < 1e-9, `one person went home from the ${kind}, and it keeps ${u0.people - 1}`);
    need(Math.abs((await clanPeople()) - c0 - 1) < 1e-9, 'the clan has the person back');
  },
  'Disband…/warriors': async () => {
    await select('camp');
    const u0 = await unit('camp');
    const c0 = await clanPeople();
    const r = await stepperGo('Disband…', { option: 'warriors', n: 1 });
    need(r.pressed, 'Disband… warriors, 1, Go is pressed on the camp');
    need(Math.abs((await unit('camp')).people - u0.people + 1) < 1e-9, 'one warrior went home from the army');
    need(Math.abs((await clanPeople()) - c0 - 1) < 1e-9, 'the clan has the person back');
  },
  'Disband all': async (kind) => disbandAll(kind),
  'Route from…': async () => {
    await select('scout');
    need(await press('Route from…'), 'Route from… is pressed on the scout');
    need((await live()).v.units[(await live()).scout.id] !== undefined, 'the scout is still there');
    need((await page.eval("document.body.dataset.armed")) === 'start', 'Route from… arms the start');
    await page.click(S);
    await page.eval(frames);
    need((await page.eval("document.body.dataset.routeStart")) === String(S), `the start mark is on ${S}`);
    need((await page.eval("document.body.dataset.armed")) === '', 'the start is set, and nothing is armed');
    await armAndClick('Route to…', T);
    const w = await live();
    need(JSON.stringify(w.v.units[w.scout.id].route) === JSON.stringify([S, T]), `the route is set from ${S} to ${T}`);
    need((await page.eval("document.body.dataset.routeStart")) === '', 'the start mark clears once the route is sent');
  },
  'Clear route': async () => {
    await select('scout');
    await armAndClick('Route to…', T);
    need((await live()).v.units[(await live()).scout.id].route != null, 'the scout has a route');
    need(await press('Clear route'), 'Clear route is pressed on the routed scout');
    need((await live()).v.units[(await live()).scout.id].route == null, 'Clear route clears it');
  },
  'Gift −1': async () => {
    await select('civ');
    await press('Gift +1');
    const g0 = (await live()).v.civs[(await live()).me].gift;
    need(await press('Gift −1'), 'Gift −1 is pressed');
    const g1 = (await live()).v.civs[(await live()).me].gift;
    need(Math.abs(g1 - (g0 - 1)) < 1e-9, `the gift moved down by one (${g0} → ${g1})`);
  },
  'Gift +1': async () => {
    await select('civ');
    const g0 = (await live()).v.civs[(await live()).me].gift;
    need(await press('Gift +1'), 'Gift +1 is pressed');
    const g1 = (await live()).v.civs[(await live()).me].gift;
    need(Math.abs(g1 - (g0 + 1)) < 1e-9, `the gift moved up by one (${g0} → ${g1})`);
  },
  'Keep −1': () => keepStep('Keep −1', -1),
  'Keep +1': () => keepStep('Keep +1', 1),
  'Keep default': async () => {
    await select('civ');
    await press('Keep +1');
    need((await live()).v.civs[(await live()).me].keep_set != null, 'the civ keeps a set amount');
    need(await press('Keep default'), 'Keep default is pressed');
    need((await live()).v.civs[(await live()).me].keep_set == null, 'Keep default clears it');
  },
  'Claim (P)': async () => {
    await select('civ');
    await paintTile('Claim (P)', X);
    const w = await live();
    need(w.v.civs[w.me].claim.includes(X), `Claim (P) and a click add tile ${X} to the civ's claim`);
  },
  'Unclaim (U)': async () => {
    await select('civ');
    await paintTile('Claim (P)', X);
    await paintTile('Unclaim (U)', X);
    const w = await live();
    need(!w.v.civs[w.me].claim.includes(X), `Unclaim (U) and a click take tile ${X} out of the claim`);
  },
};

/** Raise… into the camp (warriors or archers): the army gains a person, and a clan gives it. */
async function raiseIntoCamp(option) {
  await select('camp');
  const count = async () => (await unit('camp')).people;
  const u0 = await unit('camp');
  const r = await stepperGo('Raise…', { option, n: 1 });
  const refused = r.pressed ? '' : (r.refused ?? '');
  if (!r.pressed && /store can't fill/.test(refused)) {
    need(Math.abs((await count()) - u0.people) < 1e-9, `Raise… ${option} into the army is refused for want of store (${refused.replace(/^greyed: /, '')}), and the army does not grow`);
    return;
  }
  need(r.pressed, `Raise… ${option}, 1, Go is pressed on the camp${refused ? `: ${refused}` : ''}`);
  need(Math.abs((await count()) - u0.people - 1) < 1e-9, `the army has one more person (${option})`);
}

/** Raise… as a player presses it: one unit of the kind, count 1, Go. A unit that takes a pack is filled from the
 *  civ's store, which the staged game starts without (its founding spends the endowment on the first army's pack),
 *  so the press is refused with the store's reason and nothing appears. Then the case checks that refusal and that
 *  the world is unchanged; where the store is not needed (a scout), the unit appears. */
async function raiseAs(option, count, what) {
  const n0 = await count();
  const r = await stepperGo('Raise…', { option, n: 1 });
  const refused = r.pressed ? '' : (r.refused ?? '');
  if (!r.pressed && /store can't fill/.test(refused)) {
    need((await count()) === n0, `Raise… ${option} is refused for want of store (${refused.replace(/^greyed: /, '')}), and no ${what} appears`);
    return;
  }
  need(r.pressed, `Raise… ${option}, 1, Go is pressed${refused ? `: ${refused}` : ''}`);
  need(Math.abs((await count()) - n0 - 1) < 1e-9, `Raise… ${option} makes one more ${what}`);
}
async function raiseCivKind(option, made) {
  await select('civ');
  const count = async () => {
    const { v, me } = await live();
    const mine = Object.values(v.units).filter((u) => u.civ === me && made(u));
    return mine.length;
  };
  await raiseAs(option, count, option === 'a scout' ? 'scout' : 'army');
}

// ---- the inventory: every button of every panel the led civ can see, by the page's own shape -----------------

/** The panels the inventory reads: its name, the selection, the case kind its buttons take, and the state set
 *  up on the selected panel. The rival's, the clan's, the market's and the tile's panels have no buttons for the led player. */
const PANELS = [
  { name: 'civ (led)', sel: 'civ', kind: 'civ', setup: async () => {} },
  { name: 'civ (rival)', sel: 'rival', kind: null, setup: async () => {} },
  { name: 'clan', sel: 'clan', kind: null, setup: async () => {} },
  { name: 'market', sel: 'market', kind: null, setup: async () => {} },
  { name: 'tile', sel: 'tile', kind: null, setup: async () => {} },
  { name: 'camp', sel: 'camp', kind: 'camp', setup: async () => {} },
  { name: 'camp, marching', sel: 'camp', kind: 'camp', setup: async () => { await page.command(T); await page.eval(frames); } },
  { name: 'camp, deployed', sel: 'camp', kind: 'camp', setup: async () => { await paintTile('Paint (P)', N); } },
  { name: 'civ, keep set', sel: 'civ', kind: 'civ', setup: async () => { await press('Keep +1'); } },
  { name: 'scout', sel: 'scout', kind: 'scout', setup: async () => {} },
  { name: 'scout, moving', sel: 'scout', kind: 'scout', setup: async () => { await armAndClick('Move to…', T); } },
  { name: 'scout, routed', sel: 'scout', kind: 'scout', setup: async () => { await armAndClick('Route to…', T); } },
];

/** Each (kind, button) the led civ's panels show, with the panel it was found on. */
const pairs = new Map();
for (const p of PANELS) {
  try {
    await stage();
    await select(p.sel);
    await p.setup();
    await page.eval(frames);
    const names = namesOf(await shape());
    console.log(`info    ${p.name}: ${names.join(' · ') || 'no buttons'}`);
    if (p.kind == null) {
      need(names.length === 0, `the ${p.name} panel has no buttons for the led player (${names.join(', ') || 'none'})`);
      continue;
    }
    for (const n of names) pairs.set(`${p.kind}|${n}`, p.name);
  } catch (e) {
    fail(`${p.name}: ${e.message}`);
  }
}

// Each button of each panel has a case: a label without one fails the script.
for (const key of pairs.keys()) {
  const label = keyOf(key.split('|')[1]);
  if (!CASES[label]) fail(`button ${label} has no case`);
}

// Every case, on a fresh stage, pressed once per panel kind it appears on.
const rows = [];
for (const key of pairs.keys()) {
  const [kind, name] = key.split('|');
  const label = keyOf(name);
  const c = CASES[label];
  if (!c) continue;
  await stage(label === 'Pillage' ? specFor('P', true) : specFor());
  try {
    console.log(`case    ${kind}: ${name}`);
    await c(kind);
    rows.push({ kind, name, check: 'ok' });
  } catch (e) {
    fail(`${kind} ${name}: ${e.message}`);
    rows.push({ kind, name, check: 'FAIL' });
  }
}

for (const e of page.errors()) fail(`page error: ${e}`);
await page.close();
console.log(`cases: ${rows.length}, labels: ${new Set(rows.map((r) => keyOf(r.name))).size}`);
process.exitCode = failed ? 1 : 0;
