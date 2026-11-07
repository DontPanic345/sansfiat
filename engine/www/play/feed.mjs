// The feed and alerts (touch point 22-24): the feed lists newest first; it collapses and opens; a click on an
// item selects its subject and centres the map; an alert stays in the alerts box after more turns until it
// is clicked; the panel's reasons show for a clan and for the camp.   node www/play/feed.mjs [--out DIR]
// (small seed 1)
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open, raiseArmy } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-feed-')));
mkdirSync(out, { recursive: true });

let failed = false;
const fail = (msg) => {
  console.error(`feed: ${msg}`);
  failed = true;
};
const need = (ok, msg) => {
  if (!ok) fail(msg);
  else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
const alertTexts = () => page.eval("[...document.querySelectorAll('#alerts li')].map((l) => l.textContent.trim())");

const page = await open('?size=small&seed=1&run=1');
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  await raiseArmy(page);
  const me = await page.eval('game.view().player');
  const cap = await page.eval(`game.view().civs[${me}].capital`);
  for (let turn = 0; turn < 6; turn++) await page.button('End turn');

  // Newest first: the turns down the list never rise.
  const turns = await page.eval("[...document.querySelectorAll('#feed li')].map((l) => Number(l.dataset.turn))");
  need(turns.length > 0 && turns.every((t, k) => k === 0 || turns[k - 1] >= t), `the feed lists newest first (${turns.length} lines, from turn ${turns[0]})`);

  // Collapsing and opening.
  await page.eval("document.getElementById('hidefeed').click()");
  need(await page.eval("document.getElementById('side').classList.contains('collapsed')"), 'the feed collapses');
  await page.eval("document.getElementById('feedtoggle').click()");
  need(!(await page.eval("document.getElementById('side').classList.contains('collapsed')")), 'the Feed toggle opens it');

  // A click on an item with a clan selects the clan and centres the map on its home.
  const item = await page.eval(`(() => { const f = game.feed.items.filter((i) => i.clan != null && i.clan in game.view().clans && !i.alert);
    const it = f[f.length - 1]; return it ? { id: it.id, clan: it.clan, home: game.view().clans[it.clan].home } : null; })()`);
  need(item !== null, 'the feed has an item about a clan');
  if (item) {
    await page.eval(`document.querySelector('#feed li[data-id="${item.id}"] button').click()`);
    await page.eval(frames);
    await page.until('!game.map.anim', 5000);
    need((await page.data()).selected === `clan:${item.clan}`, `a click on the item selects clan ${item.clan}`);
    const c = await page.eval(`(() => { const r = document.getElementById('map').getBoundingClientRect(); const p = game.map.screenOf(${item.home});
      return { dx: Math.abs(p.x - (r.left + r.width / 2)), w: r.width }; })()`);
    need(c.dx < c.w * 0.3, `the click centres the map on the clan's home (${Math.round(c.dx)} px from the middle)`);
  }

  // An alert stays in the box until clicked: march the camp out past its line's reach until one shows.
  const far = await page.eval(`(() => { const v = game.view(), L = game.layout(), cap = v.civs[${me}].capital;
    const steps = (a, b) => { const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b]; return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr)); };
    const rival = new Set(Object.entries(v.civs).filter(([id]) => +id !== ${me}).flatMap(([, c]) => c.land));
    return [...Array(L.q.length).keys()].filter((i) => steps(i, cap) >= 7 && !rival.has(i)).sort((a, b) => steps(a, cap) - steps(b, cap) || a - b).slice(0, 40); })()`);
  const camp = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'camp').id`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) {
    await page.click(cap);
    await page.eval(frames);
  }
  let marched = false;
  for (const tile of far) {
    await page.command(tile);
    if (await page.eval(`game.view().units[${camp}].dest === ${tile}`)) {
      marched = true;
      break;
    }
  }
  need(marched, 'the camp marches out past its reach');
  let alerts = [];
  for (let turn = 0; turn < 30 && alerts.length === 0; turn++) {
    await page.button('End turn');
    alerts = await alertTexts();
  }
  need(alerts.length > 0, `an alert shows in the alerts box (${alerts[0] ?? 'none'})`);
  const badge = await page.eval("document.getElementById('feedtoggle').textContent");
  need(/Feed \(\d+\)/.test(badge) || alerts.length === 0, `the Feed toggle counts the unseen alerts (${badge})`);
  for (let turn = 0; turn < 3; turn++) await page.button('End turn');
  const later = await alertTexts();
  need(alerts.every((a) => later.includes(a)), `the alert is still in the box three turns on (${later.length} unseen)`);
  await page.eval("document.querySelector('#alerts li button').click()");
  await page.eval(frames);
  const after = await alertTexts();
  // A click sees the alert and, by selecting its subject, every alert about that subject (feed.js `see`): so the
  // alert clicked is gone, and the others about its subject with it.
  need(!after.includes(later[0]) && after.length < later.length, `a click on an alert takes it out of the box (${later.length} → ${after.length})`);
  await page.shot(join(out, 'alerts.png'), 1);

  // The panel's reasons: a clan with the most, and the camp.
  const clan = await page.eval(`(() => { const v = game.view();
    const rows = Object.values(v.clans).map((c) => ({ id: c.id, n: JSON.parse(game.notes(c.id)).length }));
    return rows.sort((a, b) => b.n - a.n || a.id - b.id)[0]; })()`);
  const home = await page.eval(`game.view().clans[${clan.id}].home`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `clan:${clan.id}`; n++) {
    await page.click(home);
    await page.eval(frames);
  }
  let p = await page.panel();
  need(p.name === `Clan ${clan.id}` && p.reasons.length > 0, `the clan's panel shows its reasons (${p.reasons.length})`);
  const campTile = await page.eval(`game.view().units[${camp}].tile`);
  for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) {
    await page.click(campTile);
    await page.eval(frames);
  }
  p = await page.panel();
  need(p.reasons.length > 0, `the camp's panel shows its reasons (${p.reasons.length})`);
  for (const e of page.errors()) fail(`page error: ${e}`);
} catch (e) {
  fail(e.stack ?? String(e));
} finally {
  await page.close();
}
process.exitCode = failed ? 1 : 0;
