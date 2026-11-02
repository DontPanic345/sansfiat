// The layout at three sizes (touch point 26): opens each of SIZES and shoots the start screen, the pick, play
// with the camp selected, play with the civ selected, and the end screen, as `<size>-<screen>.png` (laptop and
// wide at half scale), and a large map (seed 2) at fit at laptop size for the placeholders. It fails on a
// horizontal scroll, a bar that wraps past one line, a button in the panel's row that is clipped and can't be
// scrolled to, and (at laptop and wide, with the camp selected and the feed as it opens) a map under `--share`
// of the window, counting only the map nothing covers. Its output says each size's share.
//   node www/play/layout.mjs [--out DIR] [--share 0.6]     (--share 0 skips the share check)
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open, SIZES } from '../play.mjs';

const args = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : dflt;
};
const out = resolve(flag('--out', mkdtempSync(join(tmpdir(), 'play-layout-'))));
const SHARE = Number(flag('--share', '0.6'));
mkdirSync(out, { recursive: true });

let failed = false;
const fail = (msg) => {
  console.error(`layout: ${msg}`);
  failed = true;
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
const scale = (size) => (size === 'phone' ? 1 : 0.5);

/** The page is not wider than the window: no horizontal scroll anywhere. */
async function noScroll(page, size, screen) {
  const wide = await page.eval('document.documentElement.scrollWidth > window.innerWidth');
  if (wide) fail(`${size} ${screen}: the page scrolls sideways`);
}

/** The bar is one line: its height is no more than a line of its controls and its padding. */
async function barOneLine(page, size, screen) {
  const h = await page.eval("document.getElementById('bar').getBoundingClientRect().height");
  if (h > 50) fail(`${size} ${screen}: the bar is ${Math.round(h)} px tall, past one line`);
  return Math.round(h);
}

/** Each button in the panel's row is on the screen, or can be scrolled to: each is scrolled into view in turn,
 *  and one that is still clipped then is a fail. Returns how many were clipped before scrolling. */
async function panelButtons(page, size, screen) {
  const inside = (label) => page.eval(`(() => { const d = document.getElementById('detail').getBoundingClientRect();
    const b = [...document.querySelectorAll('#detail .actions button')].find((b) => b.textContent.trim() === ${JSON.stringify(label)});
    if (!b) return 'gone'; const r = b.getBoundingClientRect();
    return r.width > 0 && r.left >= d.left - 0.5 && r.right <= d.right + 0.5 && r.top >= d.top - 0.5 && r.bottom <= d.bottom + 0.5; })()`);
  const labels = await page.eval("[...document.querySelectorAll('#detail .actions button')].map((b) => b.textContent.trim())");
  let clippedFirst = 0;
  const stuck = [];
  for (const label of labels) {
    if ((await inside(label)) === true) continue;
    clippedFirst++;
    await page.eval(`[...document.querySelectorAll('#detail .actions button')].find((b) => b.textContent.trim() === ${JSON.stringify(label)}).scrollIntoView({ block: 'nearest', inline: 'nearest' })`);
    await page.eval(frames);
    if ((await inside(label)) !== true) stuck.push(label);
  }
  if (stuck.length) fail(`${size} ${screen}: panel buttons clipped and out of reach: ${stuck.join(', ')}`);
  return clippedFirst;
}

/** The share of the window the map shows, uncovered: its box less what the legend, the zoom buttons, the
 *  Show feed button and the error line cover of it. */
const share = (page) =>
  page.eval(`(() => { const W = window.innerWidth * window.innerHeight; const m = document.getElementById('mapbox').getBoundingClientRect();
    const cover = ['#legend[open]', '#zoom', '#showfeed:not([hidden])', '#error:not(:empty)'].map((s) => document.querySelector(s)?.getBoundingClientRect())
      .filter(Boolean).map((r) => Math.max(0, Math.min(r.right, m.right) - Math.max(r.left, m.left)) * Math.max(0, Math.min(r.bottom, m.bottom) - Math.max(r.top, m.top)));
    const covered = Math.min(cover.reduce((a, b) => a + b, 0), m.width * m.height);
    return { share: (m.width * m.height - covered) / W, map: [Math.round(m.width), Math.round(m.height)] }; })()`);

const report = [];
for (const size of Object.keys(SIZES)) {
  const s = scale(size);
  // The start screen.
  {
    const page = await open('', { size });
    try {
      await page.shot(join(out, `${size}-start.png`), s);
      await noScroll(page, size, 'start');
    } finally {
      await page.close();
    }
  }
  // The pick, over the map.
  {
    const page = await open('?size=small&seed=2&run=1', { size });
    try {
      await page.until("document.body.dataset.phase === 'choose'");
      await page.shot(join(out, `${size}-pick.png`), s);
      await noScroll(page, size, 'pick');
    } finally {
      await page.close();
    }
  }
  // Play: the camp selected, then the civ.
  const page = await open('?size=small&seed=2&run=1', { size });
  try {
    await page.until("document.body.dataset.phase === 'choose'");
    await page.button('Lead ');
    await page.until("document.body.dataset.phase === 'main'");
    const me = await page.eval('game.view().player');
    const cap = await page.eval(`game.view().civs[${me}].capital`);
    const camp = await page.eval(`Object.values(game.view().units).find((u) => u.civ === ${me} && u.kind === 'camp').id`);
    for (let n = 0; n < 8 && (await page.data()).selected !== `unit:${camp}`; n++) {
      await page.click(cap);
      await page.eval(frames);
    }
    if ((await page.data()).selected !== `unit:${camp}`) fail(`${size}: the camp could not be selected`);
    await page.eval(frames);
    const bar = await barOneLine(page, size, 'camp');
    await noScroll(page, size, 'camp');
    const row = await panelButtons(page, size, 'camp');
    const shares = size === 'phone' ? null : await share(page);
    await page.shot(join(out, `${size}-camp.png`), s);
    for (let n = 0; n < 8 && (await page.data()).selected !== `civ:${me}`; n++) {
      await page.click(cap);
      await page.eval(frames);
    }
    await page.eval(frames);
    await barOneLine(page, size, 'civ');
    await noScroll(page, size, 'civ');
    const civRow = await panelButtons(page, size, 'civ');
    await page.shot(join(out, `${size}-civ.png`), s);
    // The end screen, from the Game menu.
    await page.eval("document.getElementById('menu-standing').click()");
    await page.until("document.body.dataset.screen === 'end'");
    await page.eval(frames);
    await noScroll(page, size, 'end');
    await page.shot(join(out, `${size}-end.png`), s);
    if (size === 'laptop') {
      // A large map at fit, for the placeholders at low zoom.
      await page.goto('?size=large&seed=2&run=1');
      await page.until("document.body.dataset.phase === 'choose'", 300_000);
      await page.button('Lead ');
      await page.until("document.body.dataset.phase === 'main'");
      await page.eval('game.map.fit()');
      await page.eval(frames);
      await page.shot(join(out, 'laptop-large-fit.png'), s);
    }
    report.push({ size, bar_px: bar, panel_clipped_before_scroll: row + civRow, share: shares?.share ?? null, map_px: shares?.map ?? null });
    for (const e of page.errors()) fail(`${size}: page error: ${e}`);
  } finally {
    await page.close();
  }
}

for (const r of report) {
  const share = r.share == null ? 'n/a' : `${Math.round(r.share * 1000) / 10}%`;
  console.log(`${r.size.padEnd(7)} bar ${r.bar_px} px · map ${share} of the window${r.map_px ? ` (${r.map_px[0]}×${r.map_px[1]} px)` : ''}`);
  if (SHARE > 0 && r.share != null && r.share < SHARE) fail(`${r.size}: the map shows ${share} of the window with the camp selected, under ${Math.round(SHARE * 100)}%`);
}
console.log(`shots in ${out}`);
process.exitCode = failed ? 1 : 0;
