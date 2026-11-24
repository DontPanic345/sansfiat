// The clans on the map (#444, build-2): the clan lineup (CLANS) staged on a built map with its states set (CLAN_STATES),
// the camera between tiles 93 and 170 at k 35 on a laptop. Checks each clan drawn once, as the kind its state gives (the
// rule, not a list), its cloth nearest its own civ's colour (or none's, for a clan of no civ) with tinted pixels, the
// starving clan's red edge round its cell, the shared home's four clans in four places, each anchored inside the hex, a
// click on the shared home reaching each of its four clans, and at the flat zoom (the lineup on a phone at k 5, where the
// terrain is flat) every clan a dot, with the cells back at k 6. Writes clans.png to --out.
//   node www/play/clans.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { CLANS, CLAN_STATES, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-clans-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`clans: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';

// The camera between tiles 93 and 170 (the lineup's shared home and a wood's tile), at the given zoom.
const camera = (k) => `(() => { const m = game.map; m.cx = (m.px[93] + m.px[170]) / 2; m.cy = (m.py[93] + m.py[170]) / 2; m.k = ${k}; m.viewChanged(); })()`;

// What the map has drawn, read after a fresh draw: each clan's cell (kind, rectangle, the tile its anchor lands on), its
// starving edge, its probe, and the lineup's clans with their state and the kind the rule gives it.
const SNAP = `(() => {
  const m = game.map;
  m.drawNow();
  const v = m.model.v, dpr = m.dpr;
  const kindOf = (c) => (c.since <= 1 ? 'walk' : c.settled ? 'hearth' : 'tent');
  const cells = m.stats.clans.map((s) => {
    const c = v.clans[s.clan];
    return {
      clan: s.clan, home: c.home, civ: s.civ, kind: s.kind, expect: m.stats.atlas ? kindOf(c) : 'dot', starving: s.starving,
      since: c.since, settled: !!c.settled, x: s.x, y: s.y, w: s.w, h: s.h,
      // A cell's ground is at 56 of 112 across and 90 of 96 down: the anchor, read back to the tile it lands on.
      tile: m.tileAt((s.x + s.w / 2) / dpr, (s.y + (s.h * 90) / 96) / dpr),
    };
  });
  const probe = m.clanProbe().map((p) => ({ clan: p.clan, civ: p.civ, nearest: p.nearest, tinted: p.tinted }));
  return { cells, probe, total: Object.keys(v.clans).length, atlas: m.stats.atlas };
})()`;

// The red pixels (#ff2a2a within 24) in the ring round a clan's cell, with its starving edge on and off.
const redRing = (clan) => `(() => {
  const m = game.map, c = m.model.v.clans[${clan}];
  const count = () => {
    m.drawNow();
    const s = m.stats.clans.find((r) => r.clan === ${clan}), cv = m.canvas, g = cv.getContext('2d'), pad = 6;
    const x0 = Math.max(0, Math.floor(s.x - pad)), y0 = Math.max(0, Math.floor(s.y - pad));
    const x1 = Math.min(cv.width, Math.ceil(s.x + s.w + pad)), y1 = Math.min(cv.height, Math.ceil(s.y + s.h + pad));
    const d = g.getImageData(x0, y0, x1 - x0, y1 - y0).data;
    let n = 0;
    for (let y = y0; y < y1; y++) {
      for (let x = x0; x < x1; x++) {
        if (x >= s.x && x < s.x + s.w && y >= s.y && y < s.y + s.h) continue;
        const i = ((y - y0) * (x1 - x0) + (x - x0)) * 4;
        if (Math.abs(d[i] - 255) < 24 && Math.abs(d[i + 1] - 42) < 24 && Math.abs(d[i + 2] - 42) < 24) n++;
      }
    }
    return n;
  };
  c.starving = true;
  const on = count();
  c.starving = false;
  const off = count();
  c.starving = true;
  m.drawNow();
  return { on, off };
})()`;

const page = await open('', { size: 'laptop' });
try {
  await stage(page, CLANS);
  await page.eval(CLAN_STATES);
  await page.eval(camera(35));
  await page.until('!game.map.anim && !game.map.frame', 5000);
  const snap = await page.eval(SNAP);
  const HOME = await page.eval('game.map.at(-1, 2)');

  // One cell per clan on screen, each of the kind its state gives.
  const kinds = {};
  for (const c of snap.cells) kinds[c.kind] = (kinds[c.kind] ?? 0) + 1;
  need(snap.cells.length === snap.total, `every clan is drawn once on screen (${snap.cells.length} of ${snap.total}): ${Object.entries(kinds).map(([k, n]) => `${n} ${k}`).join(', ')}`);
  const wrong = snap.cells.filter((c) => c.kind !== c.expect);
  need(wrong.length === 0 && snap.cells.some((c) => c.kind === 'walk') && snap.cells.some((c) => c.kind === 'hearth') && snap.cells.some((c) => c.kind === 'tent'),
    `each clan is drawn as the kind its state gives (${wrong.map((c) => `clan ${c.clan} ${c.kind} for ${c.expect}`).join(', ') || 'all'}), and all three kinds are on screen`);
  const shared = snap.cells.filter((c) => c.home === HOME);
  const third = shared.find((c) => c.since <= 1);
  need(shared.length === 4 && third && shared.filter((c) => c.kind === 'walk').length === 1 && third.kind === 'walk', 'the shared home has four clans, and its third (since 0) is walkers');

  // Every clan's cloth is nearest its own civ's colour, or the pale for a clan of no civ, with tinted pixels.
  const colour = snap.probe.filter((p) => String(p.nearest) !== String(p.civ ?? 'none') || !(p.tinted > 0));
  need(snap.probe.length === snap.cells.length && colour.length === 0,
    `every clan's cloth is nearest its own civ's colour, or the pale of a clan of no civ, with tinted pixels (${snap.probe.map((p) => `${p.civ ?? 'none'}→${p.nearest}`).join(' ')})`);

  // The starving clan: marked in stats, and red round its cell only when it starves.
  const starving = snap.cells.filter((c) => c.starving);
  const onPlains = await page.eval('game.map.at(2, 2)');
  need(starving.length === 1 && starving[0].home === onPlains, `the starving clan is marked, on gold's plains tile (${starving.map((c) => `clan ${c.clan}`).join(', ') || 'none'})`);
  if (starving.length === 1) {
    const ring = await page.eval(redRing(starving[0].clan));
    need(ring.on - ring.off >= 10, `the starving clan has a red edge round its cell (${ring.on} red pixels with the edge, ${ring.off} without)`);
  }

  // The shared home: four places inside its hex, each anchor on the home's tile.
  const places = shared.map((c) => [c.x + c.w / 2, c.y + (c.h * 90) / 96]);
  const apart = places.every((a, i) => places.every((b, j) => i >= j || Math.hypot(a[0] - b[0], a[1] - b[1]) >= 4));
  need(shared.length === 4 && apart, 'the shared home\'s four clans stand at four different places');
  const outside = snap.cells.filter((c) => c.tile !== c.home);
  need(outside.length === 0, `every clan's anchor lies inside its home's hex (${outside.map((c) => `clan ${c.clan} on ${c.tile}`).join(', ') || 'all'})`);

  // A click on the shared home cycles its four clans.
  const seen = new Set();
  for (let n = 0; n < 12 && seen.size < 4; n++) {
    await page.click(HOME);
    await page.eval(frames);
    const sel = (await page.data()).selected ?? '';
    if (sel.startsWith('clan:')) seen.add(sel);
  }
  const ids = shared.map((c) => `clan:${c.clan}`).sort();
  need(seen.size === 4 && [...seen].sort().join() === ids.join(), `a click on the shared home reaches each of its four clans (${[...seen].join(', ')})`);

  await page.eval(camera(35));
  await page.eval(frames);
  const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
  writeFileSync(join(out, 'clans.png'), Buffer.from(png.split(',')[1], 'base64'));
  for (const e of page.errors()) {
    console.error(`clans: page error: ${e}`);
    failed = true;
  }
} finally {
  await page.close();
}

// The flat zoom: the lineup on a phone at k 5 (its laptop minK is above it), every clan a dot; at k 6 the cells are back.
const flat = await open('', { size: 'phone' });
try {
  await stage(flat, CLANS);
  await flat.eval(CLAN_STATES);
  await flat.eval(camera(5));
  const at5 = await flat.eval(SNAP);
  need(!at5.atlas && at5.cells.length === at5.total && at5.cells.every((c) => c.kind === 'dot'),
    `at the flat zoom (phone k 5) every clan is a dot (${at5.cells.length} of ${at5.total})`);
  await flat.eval(camera(6));
  const at6 = await flat.eval(SNAP);
  need(at6.atlas && at6.cells.length === at6.total && at6.cells.every((c) => c.kind !== 'dot'), 'at phone k 6 the clans are cells, not dots');
  for (const e of flat.errors()) {
    console.error(`clans: page error: ${e}`);
    failed = true;
  }
} finally {
  await flat.close();
}
process.exit(failed ? 1 : 0);
