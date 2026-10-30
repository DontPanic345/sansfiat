// Plays the page's reasons and alerts, headless: leads the first civ (seed 2, small), selects a clan's home
// tile and screenshots the panel with its reasons, then the camp's; turns Hidden country on and screenshots
// the map, then off; moves the camp out until an alert shows and screenshots the alerts box; and loads a
// save with its `version` removed through the file input, checking #error names the version. Usage:
//
//   node www/play-read.mjs [out-dir]     (default: relay/specs/57/screens; needs www/pkg)
//
// Uses play.mjs's driver. Exits non-zero if a reason, an alert or the load error is missing, or the page errored.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const out = resolve(process.argv[2] ?? join(here, '../../relay/specs/57/screens'));
mkdirSync(out, { recursive: true });

const missing = [];
const need = (ok, what) => {
  if (!ok) missing.push(what);
  console.log(`${ok ? 'ok     ' : 'MISSING'} ${what}`);
};

const page = await open('?size=small&seed=2&run=1', { height: 1000 });
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  const civ = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${civ}].capital`);
  // A few turns, so the clans have acted on themselves.
  for (let turn = 0; turn < 10; turn++) await page.button('End turn');

  // A clan's home tile: the panel's Why list (the clan with the most reasons, so the shot reads).
  const clan = await page.eval(`(() => {
    const v = game.view();
    const rows = Object.values(v.clans).map((c) => ({ id: c.id, home: c.home, n: JSON.parse(sim_notes(c.id)).length }));
    return rows.sort((a, b) => b.n - a.n || a.id - b.id)[0];
  })()`.replace('sim_notes', 'game.notes'));
  await page.select(clan.home, 'tile');
  const why = await page.eval("[...document.querySelectorAll('#detail .why li')].map((l) => l.textContent)");
  console.log(JSON.stringify({ clan: clan.id, why }));
  need(why.length > 0, `the clan ${clan.id} panel lists reasons`);
  await page.shot(join(out, 'clan-why.png'), 1);

  await page.select(cap, 'unit');
  const campWhy = await page.eval("[...document.querySelectorAll('#detail .why li')].map((l) => l.textContent)");
  console.log(JSON.stringify({ camp: campWhy }));
  need(campWhy.length > 0, 'the camp panel lists reasons');
  await page.shot(join(out, 'camp-why.png'), 1);

  // Hidden country: off by default, on, shot, off.
  need(!(await page.eval("document.getElementById('v-country').checked")), 'Hidden country is off by default');
  await page.eval("document.getElementById('v-country').click()");
  await page.eval('game.map.fit()');
  await page.shot(join(out, 'country-on.png'), 1);
  await page.eval("document.getElementById('v-country').click()");
  need(!(await page.eval("document.getElementById('v-country').checked")), 'Hidden country toggles off');

  // The camp out past its line's reach until an alert shows.
  const far = await page.eval(`(() => {
    const v = game.view(), L = game.layout(), cap = v.civs[${civ}].capital;
    const steps = (a, b) => {
      const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b];
      return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr));
    };
    const rival = new Set(Object.entries(v.civs).filter(([id]) => +id !== ${civ}).flatMap(([, c]) => c.territory));
    return [...Array(L.q.length).keys()].filter((i) => steps(i, cap) >= 7 && !rival.has(i)).sort((a, b) => steps(a, cap) - steps(b, cap) || a - b).slice(0, 40);
  })()`);
  await page.select(cap, 'unit');
  let gone;
  for (const tile of far) {
    await page.command(tile);
    if (await page.eval(`Object.values(game.view().units).some((u) => u.civ === ${civ} && u.kind === 'camp' && u.dest === ${tile})`)) {
      gone = tile;
      break;
    }
  }
  need(gone !== undefined, 'a tile to march the camp out to');
  let alerts = [];
  for (let turn = 0; turn < 30 && !alerts.length; turn++) {
    await page.button('End turn');
    alerts = await page.eval("[...document.querySelectorAll('#alerts li')].map((l) => l.textContent)");
  }
  console.log(JSON.stringify({ turn: await page.eval('game.view().turn'), alerts }));
  need(alerts.length > 0, 'an alert shows in the alerts box');
  await page.shot(join(out, 'alerts.png'), 1);

  // A save without its version, loaded through the file input.
  const error = await page.eval(`(async () => {
    const save = game.save();
    delete save.version;
    const dt = new DataTransfer();
    dt.items.add(new File([JSON.stringify(save)], 'old.json', { type: 'application/json' }));
    const input = document.getElementById('load');
    input.files = dt.files;
    input.dispatchEvent(new Event('change'));
    for (let n = 0; n < 50 && !document.getElementById('error').textContent; n++) await new Promise((r) => setTimeout(r, 100));
    return document.getElementById('error').textContent;
  })()`);
  console.log(JSON.stringify({ error }));
  need(/version/.test(error), '#error names the version of an old save');

  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length || missing.length) process.exitCode = 1;
} finally {
  await page.close();
}
