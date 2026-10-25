// node check.mjs <deck folder>   (run from a folder with playwright-core installed)
// Screenshots every slide to <deck>/shots, plays the deck in Chrome, and reports
// script errors, audio that failed to load, and slides that flash their builds on entry.
import { chromium } from 'playwright-core';
import { mkdirSync } from 'node:fs';

const deck = process.argv[2].replace(/\\/g, '/');
mkdirSync(`${deck}/shots`, { recursive: true });
const browser = await chromium.launch({
  channel: 'chrome',
  args: ['--autoplay-policy=no-user-gesture-required', '--allow-file-access-from-files'],
});
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
const errors = [];
page.on('pageerror', e => errors.push(e.message));
page.on('requestfailed', r => errors.push(`failed to load ${r.url()}`));
await page.goto(`file:///${deck}/index.html`);
await page.waitForTimeout(1000);

const count = await page.locator('.chips button').count();
for (let n = 0; n < count; n++) {
  await page.locator('.chips button').nth(n).click();
  await page.waitForTimeout(600);
  await page.locator('#stage').screenshot({ path: `${deck}/shots/${String(n + 1).padStart(2, '0')}.png` });
}

await page.locator('.chips button').nth(0).click();
await page.evaluate(() => {
  window.flash = {};
  const stage = document.getElementById('stage');
  stage.querySelector('.slide.on').dataset.seen = 1;
  new MutationObserver(() => {
    const s = stage.querySelector('.slide.on');
    if (!s || s.dataset.seen) return;
    s.dataset.seen = 1;
    const last = [...s.querySelectorAll('[data-step]')].at(-1);
    const t0 = performance.now();
    const tick = () => {
      window.flash[s.dataset.id] = Math.max(window.flash[s.dataset.id] ?? 0, +getComputedStyle(last).opacity);
      if (performance.now() - t0 < 400) requestAnimationFrame(tick);
    };
    tick();
  }).observe(stage, { subtree: true, attributes: true, attributeFilter: ['class'] });
});
await page.click('#play');
for (let n = 1; n < count; n++) {
  await page.waitForTimeout(1500);
  await page.click('#next');
}
await page.waitForTimeout(800);
const flash = await page.evaluate(() => window.flash);
const flashed = Object.entries(flash).filter(([, op]) => op > 0.05).map(([id]) => id);

console.log(JSON.stringify({ slides: count, shots: `${deck}/shots`, flashed, errors }, null, 1));
await browser.close();
