// A played game on the page (spec, Left unsettled): leads a civ on a small map and plays 30 turns with only
// the page's buttons, keys and clicks. On the way it changes the gift, raises a guard, paints a claim, moves
// the camp with Move to… and paints its zone, follows an alert, reloads the page at turn 15 and Continues,
// and at the end saves to a file. The page's `game.digest()` must match `sansfiat digest --game FILE`, and
// `sansfiat judge FILE` reads the save. The script reads the page (game.view()) to choose its targets; every
// change to the game goes through the page's controls. It prints what the panel said at each step.
//   node www/play/game.mjs [--seed 2] [--out DIR] [--no-cli]
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open, lead, raiseArmy } from '../play.mjs';

const args = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : dflt;
};
const SEED = Number(flag('--seed', '2'));
const out = resolve(flag('--out', mkdtempSync(join(tmpdir(), 'play-game-'))));
const cli = !args.includes('--no-cli');
mkdirSync(out, { recursive: true });
const engine = fileURLToPath(new URL('../..', import.meta.url));

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`game: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const note = (msg) => console.log(`note    ${msg}`);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('');
try {
  await lead(page, { seed: SEED });
  await raiseArmy(page);
  const turn = async () => Number(await page.eval('game.view().turn'));
  const me = Number(await page.eval('game.view().player'));
  const t0 = await turn();
  const say = async (step) => {
    const p = await page.panel();
    note(`turn ${await turn()} ${step}: ${p ? `${p.name} · ${p.doing ?? ''} · hint: ${p.hint ?? '-'} · refused: ${p.refused ?? '-'}` : 'no panel'}`);
  };
  const endTurnsTo = async (n) => {
    while ((await turn()) < t0 + n) {
      await page.button('End turn');
      await followAlert();
    }
  };
  let followed = null;
  const followAlert = async () => {
    if (followed) return;
    const text = await page.eval("document.querySelector('#alerts li button')?.textContent.trim() ?? null");
    if (!text) return;
    // The alerts box is in the feed: open it if the feed is closed (laptop's default), as a player would.
    if (await page.eval("document.getElementById('side').classList.contains('collapsed')")) await page.button('Show feed');
    await page.eval("document.querySelector('#alerts li button').click()");
    await page.eval(frames);
    followed = { text, selected: (await page.data()).selected };
    await say(`followed the alert "${text}"`);
    await page.key('Escape');
  };
  const civ = `civ:${me}`;
  need((await page.data()).selected === civ, `Lead selects the civ (turn ${t0})`);
  await say('led');

  // The gift: Gift +1 on the civ's panel.
  const gift0 = await page.eval(`game.view().civs[${me}].gift`);
  await page.button('Gift +1');
  need((await page.eval(`game.view().civs[${me}].gift`)) === gift0 + 1, `Gift +1 raises the gift (${gift0} → ${gift0 + 1})`);

  // A guard, through Raise…'s stepper.
  const guards = () => page.eval(`Object.values(game.view().units).filter((u) => u.civ === ${me} && u.kind === 'guard').reduce((a, u) => a + u.people, 0)`);
  const g0 = await guards();
  const g = await page.stepper('Raise…', { n: 1, option: 'a guard', go: true });
  await page.eval(frames);
  need(g.go && (await guards()) > g0, `Raise… raises a guard (${g.go ? 'done' : g.title})`);

  // A claim: P, a click on a free land tile beside the claim, Escape.
  const nb = await page.eval(`(() => { const m = game.map, c = game.view().civs[${me}]; const out = new Set();
    for (const t of c.claim) for (let d = 0; d < 6; d++) { const j = m.nb[t * 6 + d]; if (j >= 0 && m.land[j] && !c.claim.includes(j)) out.add(j); }
    return [...out].sort((a, b) => a - b)[0]; })()`);
  const claimed = () => page.eval(`game.view().civs[${me}].claim.length`);
  const c0 = await claimed();
  await page.key('p');
  await page.click(nb);
  await page.eval(frames);
  await say('painting the claim');
  await page.key('Escape');
  need((await claimed()) === c0 + 1, `P and a click paint the claim (${c0} → ${await claimed()})`);
  await page.key('Escape');

  await endTurnsTo(5);

  // The camp: select it on its tile, Move to… a tile past its line's reach, then paint its zone there.
  const camp = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'camp').id`);
  const campTile = () => page.eval(`game.view().units[${camp}].tile`);
  await page.select(await campTile(), 'unit');
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) await page.click(await campTile());
  need((await page.data()).selected === `unit:${camp}`, 'a click on its tile selects the camp');
  await say('camp selected');
  const far = await page.eval(`(() => { const v = game.view(), L = game.layout(), cap = v.civs[${me}].capital;
    const steps = (a, b) => { const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b]; return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr)); };
    const rival = new Set(Object.entries(v.civs).filter(([id]) => +id !== ${me}).flatMap(([, c]) => c.land));
    return [...Array(L.q.length).keys()].filter((i) => game.map.land[i] && steps(i, cap) >= 6 && !rival.has(i)).sort((a, b) => steps(a, cap) - steps(b, cap) || a - b).slice(0, 30); })()`);
  let dest = null;
  for (const t of far) {
    await page.button('Move to…');
    await page.click(t);
    await page.eval(frames);
    if (await page.eval(`game.view().units[${camp}].dest === ${t}`)) {
      dest = t;
      break;
    }
    if ((await page.data()).armed) await page.key('Escape');
    for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) await page.click(await campTile());
  }
  need(dest != null, `Move to… and a click send the camp to tile ${dest}`);
  await say('camp ordered');
  for (let n = 0; n < 12 && (await campTile()) !== dest; n++) {
    await page.button('End turn');
    await followAlert();
  }
  need((await campTile()) === dest, `the camp walks to tile ${dest} (turn ${await turn()})`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) await page.click(dest);
  const ring = await page.eval(`(() => { const m = game.map, out = []; for (let d = 0; d < 6; d++) { const j = m.nb[${dest} * 6 + d]; if (j >= 0 && m.land[j]) out.push(j); } return out.slice(0, 2); })()`);
  await page.key('p');
  for (const t of ring) await page.click(t);
  await say('painting the zone');
  await page.key('Escape');
  const zone = await page.eval(`game.view().units[${camp}].territory.length`);
  need(zone > 1, `P and clicks paint the camp's zone (${zone} tiles)`);
  await page.key('Escape');

  // Turn 15: reload the page and Continue.
  await endTurnsTo(15);
  const mid = { turn: await turn(), digest: await page.eval('game.digest()') };
  await page.goto('');
  await page.reload();
  await page.until("document.body.dataset.screen === 'start'");
  const line = await page.eval("document.getElementById('screen-start').textContent.replace(/\\s+/g, ' ').trim()").catch(() => null);
  note(`start screen after the reload: ${line ?? '(no #start text)'}`);
  await page.button('Continue');
  await page.until("document.body.dataset.screen === 'play'");
  need((await turn()) === mid.turn && (await page.eval('game.digest()')) === mid.digest, `Continue gives turn ${mid.turn} with its digest ${mid.digest}`);

  await endTurnsTo(30);
  need(followed != null, `an alert was followed (${followed ? `"${followed.text}" → ${followed.selected}` : 'none came'})`);

  // Save to file.
  await page.download(out);
  await page.eval("document.querySelector('#gamemenu > summary').click()");
  await page.button('Save to file');
  let saved;
  for (let i = 0; i < 50 && !saved; i++) {
    saved = readdirSync(out).find((f) => /^sansfiat-seed.*\.json$/.test(f));
    if (!saved) await sleep(100);
  }
  need(!!saved, `Save to file writes ${saved}`);
  const end = { turn: await turn(), digest: await page.eval('game.digest()') };
  note(`page: turn ${end.turn} digest ${end.digest}`);
  for (const e of page.errors()) need(false, `page error: ${e}`);
  await page.shot(join(out, 'game.png'));

  if (saved && cli) {
    const run = (sub) => spawnSync('cargo', ['run', '--release', '-q', '-p', 'sansfiat', '--', ...sub, ...(sub[0] === 'digest' ? ['--game'] : []), join(out, saved)], { cwd: engine, encoding: 'utf8' });
    const d = run(['digest']);
    note(`cli digest: ${d.stdout.trim()}`);
    need(d.stdout.trim() === `${end.turn} ${end.digest}`, 'sansfiat digest --game matches the page');
    const j = run(['judge']);
    console.log(j.stdout.trim());
    need(j.status === 0, `sansfiat judge reads the save${j.status ? `: ${j.stderr.trim().slice(-300)}` : ''}`);
  }
  console.log(JSON.stringify({ seed: SEED, from: t0, ...end, saved: saved && join(out, saved) }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
