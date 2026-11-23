// The units on the map (build-2): the lineup staged on a built map (play-stage.mjs's LINEUP), the camera between tiles
// 125 and 170 at k 35 on a laptop. Checks six figures drawn on their tiles (three camps, three scouts), every unit's probe
// nearest its own civ's colour with tinted pixels, a click on Gold's tile cycling its two units, and both of Gold's numbers
// in stats.labels with boxes that don't overlap. Exits non-zero when one is missing; writes units.png to --out.
//   node www/play/units.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { LINEUP, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-units-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`units: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// The lineup's units: each civ's camp and scout, on the hex [q, r] of its mark (LINEUP's marks). The civs are found by
// their capitals, which stand on the marks A (red, led), B (blue) and C (gold), so their ids are the game's own.
const CIVS = { red: [-5, 0], blue: [0, -5], gold: [0, 5] };
const EXPECT = [
  { kind: 'camp', civ: 'red', qr: [1, 0] },
  { kind: 'camp', civ: 'blue', qr: [4, -1] },
  { kind: 'camp', civ: 'gold', qr: [1, 1] },
  { kind: 'scout', civ: 'red', qr: [4, 0] },
  { kind: 'scout', civ: 'blue', qr: [2, -1] },
  { kind: 'scout', civ: 'gold', qr: [1, 1] },
];
const GOLD = 126;

const overlap = (a, b) => a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
// What the map has drawn, with each unit's tile, read after a fresh draw.
const drawn = `(() => { game.map.drawNow(); const m = game.map, v = game.view(), tileOf = (id) => Object.values(v.units).find((u) => u.id === id).tile;
  return { units: m.stats.units.map((e) => ({ ...e, tile: tileOf(e.unit) })), labels: m.stats.labels.map((l) => ({ ...l, tile: tileOf(l.unit) })) }; })()`;

const page = await open('', { size: 'laptop' });
try {
  await stage(page, LINEUP);
  await page.eval(`(() => { const m = game.map; m.cx = (m.px[125] + m.px[170]) / 2; m.cy = (m.py[125] + m.py[170]) / 2; m.k = 35; m.viewChanged(); })()`);
  await page.until('!game.map.anim && !game.map.frame', 5000);

  const want = await page.eval(`(() => { const m = game.map, v = game.view(), capital = (qr) => m.at(qr[0], qr[1]);
    const id = Object.fromEntries(Object.values(v.civs).map((c) => [c.name, c.id]));
    const ofMark = Object.fromEntries(Object.entries(${JSON.stringify(CIVS)}).map(([n, qr]) => [n, Object.values(v.civs).find((c) => c.capital === capital(qr)).id]));
    return ${JSON.stringify(EXPECT)}.map((e) => ({ ...e, civ: ofMark[e.civ], tile: m.at(e.qr[0], e.qr[1]) })); })()`);
  need(want.every((e) => e.tile >= 0 && e.civ != null) && want.filter((e) => e.tile === GOLD).length === 2, `Gold's stack is on tile ${GOLD}`);

  const view = await page.eval(drawn);
  const found = want.every((e) => view.units.some((d) => d.kind === e.kind && d.civ === e.civ && d.tile === e.tile));
  need(view.units.length === 6 && found, `six figures are drawn on their tiles: ${view.units.map((d) => `${d.kind} civ ${d.civ} on ${d.tile}`).join(', ')}`);

  const probe = await page.eval('game.map.unitProbe()');
  const bad = probe.filter((p) => p.nearest !== p.civ || !(p.tinted > 0));
  need(probe.length === 6 && bad.length === 0, `every unit's nearest civ colour is its own, with tinted pixels (nearest ${probe.map((p) => p.nearest).join(', ')}; tinted ${probe.map((p) => p.tinted).join(', ')})`);

  // Two clicks on Gold's tile: each selects one of its two units, and the other on the next.
  const pick = async () => {
    await page.click(GOLD);
    await page.eval(frames);
    const key = (await page.data()).selected;
    const marked = (await page.eval(drawn)).units.filter((d) => d.selected).map((d) => d.unit);
    return { key, marked };
  };
  const first = await pick();
  const second = await pick();
  const idOf = (key) => (/^unit:\d+$/.test(key ?? '') ? Number(key.slice(5)) : null);
  const a = idOf(first.key), b = idOf(second.key);
  const onGold = view.units.filter((d) => d.tile === GOLD).map((d) => d.unit);
  need(
    a !== null && b !== null && a !== b && onGold.includes(a) && onGold.includes(b) && first.marked.join() === String(a) && second.marked.join() === String(b),
    `a click on Gold's tile selects one of its two units (${first.key}), the next click the other (${second.key}), and the figure marked follows`,
  );

  const after = await page.eval(drawn);
  const gold = after.labels.filter((l) => l.tile === GOLD);
  need(
    gold.length === 2 && !overlap(gold[0].box, gold[1].box),
    `both numbers on Gold's tile are drawn (${gold.map((l) => l.label).join(' and ')}), and their boxes do not overlap`,
  );

  const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
  writeFileSync(join(out, 'units.png'), Buffer.from(png.split(',')[1], 'base64'));
  for (const e of page.errors()) {
    console.error(`units: page error: ${e}`);
    failed = true;
  }
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
