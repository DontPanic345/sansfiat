// What a script asks the page (inspect.js): on a staged game of two civs on plains, the led civ's scout reads the same
// through `game.inspect` by id, as `{id}` and from its tile; `game.list` lists the view's units; `game.where` gives a
// screen point on the map's canvas over the scout's tile; `game.ui` gives each pressable thing a selector that picks it
// alone, and the led civ's panel buttons a `data-action`; `page.press` presses Gift +1 by its selector, which the civ's
// reasons then show, and refuses a hidden control. A screenshot of the civ's panel goes into --out.
//   node www/play/inspect.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { marks, plains, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-inspect-')));
mkdirSync(out, { recursive: true });

const R = 6;
const SPEC = {
  map: plains(R),
  marks: marks(R, { A: [-3, 0], B: [3, 0] }),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  scouts: [{ civ: 0, at: { mark: 'A' }, pack: 20 }, { civ: 1, at: { mark: 'B' }, pack: 20 }],
  stores: [{ civ: 0, food: 50 }],
};

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`inspect: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

const page = await open('');
try {
  await stage(page, SPEC);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  const scout = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'scout')`);

  // The query: by id, as {id}, and the tile it stands on lists it.
  const v = await page.eval(`game.inspect('#${scout.id}')`);
  need(v.kind === 'scout' && v.id === scout.id && v.at.tile === scout.tile, `#${scout.id} is the scout on tile ${scout.tile} (${v.kind} on ${v.at.tile})`);
  need(v.civ === me && v.state.people === scout.people, `its civ and warriors are the view's (${v.civ}, ${v.state.people})`);
  need(JSON.stringify(await page.eval(`game.inspect({ id: ${scout.id} })`)) === JSON.stringify(v), '{id} reads the same as #ID');
  const t = await page.eval(`game.inspect('${v.at.q},${v.at.r}')`);
  need(t.kind === 'tile' && t.here.units.some((u) => u.id === scout.id), `its tile (${v.at.q},${v.at.r}) lists it`);
  need(JSON.stringify(await page.eval(`game.inspect(${scout.tile})`)) === JSON.stringify(t), 'a tile index reads the same as its hex');
  const bad = await page.eval(`(() => { try { game.inspect('#999999'); return ''; } catch (e) { return String(e); } })()`);
  need(/nothing has id/.test(bad), `an id nothing has throws the engine's reason (${bad})`);
  const ids = await page.eval(`game.list('units').map((u) => u.id).sort((a, b) => a - b)`);
  const view = await page.eval(`Object.keys(game.view().units).map(Number).sort((a, b) => a - b)`);
  need(JSON.stringify(ids) === JSON.stringify(view), `game.list('units') is the view's units (${ids})`);

  // Where: a point on the canvas, over the scout's tile.
  await page.eval(`game.map.centreOn(${scout.tile}, { smooth: false })`);
  await page.eval(frames);
  const w = await page.eval(`game.where('#${scout.id}')`);
  const back = await page.eval(`(() => { const r = game.map.canvas.getBoundingClientRect(); return game.map.tileAt(${w.x} - r.left, ${w.y} - r.top); })()`);
  need(w.onScreen && back === scout.tile, `game.where's point (${w.x}, ${w.y}) is on the map, over tile ${scout.tile} (${back})`);
  need(await page.eval(`document.elementFromPoint(${w.x}, ${w.y})?.id === 'map'`), 'nothing covers it');

  // The led civ's panel: every pressable thing has a selector that picks it alone; each action button has data-action.
  for (let n = 0; n < 8 && (await page.data()).selected !== `civ:${me}`; n++) {
    await page.click(cap);
    await page.eval(frames);
  }
  need((await page.data()).selected === `civ:${me}`, 'clicking the capital selects the led civ');
  const rows = await page.eval('game.ui()');
  const alone = await page.eval(`game.ui().every((r) => { const all = document.querySelectorAll(r.selector); return all.length === 1; })`);
  need(rows.length > 5 && alone, `each of the ${rows.length} pressable things has a selector that picks it alone`);
  const actions = await page.eval(`game.ui('action')`);
  const named = (a) => actions.find((r) => r.action === a);
  for (const a of ['gift-minus-1', 'gift-plus-1', 'keep-minus-1', 'keep-plus-1']) need(!!named(a), `the civ's row has ${a}`);
  need(actions.every((r) => r.selector.includes('[data-action=')), 'each action button is picked by its data-action');
  need((await page.eval(`document.getElementById('detail').dataset.key`)) === `civ:${me}`, 'the panel carries its key');
  await page.shot(join(out, 'civ-panel.png'), 1);

  // Press: Gift +1 by its selector sets the gift, as the civ's reasons say; a hidden control is refused.
  await page.press(named('gift-plus-1').selector);
  await page.eval(frames);
  const said = (await page.eval(`game.inspect('#${me}')`)).notes[0]?.text ?? '';
  need(/set its gift to/.test(said), `pressing Gift +1 sets the gift ("${said}")`);
  const refused = await page.press('#load').then(() => '', (e) => e.message);
  need(/hidden/.test(refused), `a hidden control is refused (${refused})`);
} catch (e) {
  console.error(`inspect: ${e.stack || e.message}`);
  failed = true;
} finally {
  for (const e of page.errors()) {
    console.error(`inspect: page error: ${e}`);
    failed = true;
  }
  await page.close();
}
process.exit(failed ? 1 : 0);
