// The places on the page (#445, build-2): the place lineup (PLACES) staged on a built map, one turn ended, its states set
// (PLACE_STATES), the camera between tiles 47 and 140 at k 35 on a laptop. Checks each place on screen is the role its tile
// holds by the rule (a living capital, else a settlement, else a town: not a list), a capital's and a settlement's cloth
// nearest its own civ's colour with tinted pixels and a town's none, every clan on a place's tile drawn after the place (the
// clans layer follows places in LAYERS) and standing in its yard (PLACE.yard: its foot below the tile's centre), a click on the settlement's tile reaching the settlement and each of its clans, and
// at the flat zoom (a phone at k 5) every place a mark, with each capital's and settlement's house drawn in its civ's colour
// at its tile. Writes places.png to --out.
//   node www/play/places.mjs [--out DIR]
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';
import { PLACES, PLACE_STATES, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-places-')));
mkdirSync(out, { recursive: true });

let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`places: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
const frames = 'new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))';
const CAMERA = (k) => `(() => { const m = game.map; m.cx = (m.px[47] + m.px[140]) / 2; m.cy = (m.py[47] + m.py[140]) / 2; m.k = ${k}; m.viewChanged(); })()`;

// What the map drew after a fresh draw: each place (its tile, role, civ), the probe of each (its nearest colour and its
// tinted pixels), the rule for every place on screen, the clans drawn and the layers' order.
const SNAP = `(() => {
  const m = game.map, v = m.model.v;
  m.drawNow();
  const capOf = new Map(Object.values(v.civs).filter((c) => !c.fallen).map((c) => [c.capital, c.id]));
  const setOf = new Map(v.settlements.map((s) => [s.tile, s.civ]));
  const townSet = new Set(v.towns.map((t) => t.tile));
  const rule = (tile) => (capOf.has(tile) ? 'capital' : setOf.has(tile) ? 'settlement' : townSet.has(tile) ? 'town' : null);
  const expected = [...new Set([...capOf.keys(), ...setOf.keys(), ...townSet])].filter((t) => m.onScreen(t)).map((t) => ({ tile: t, role: rule(t) }));
  const probes = m.placeProbe().map((p) => ({ tile: p.tile, role: p.role, civ: p.civ, nearest: p.nearest, tinted: p.tinted }));
  const centre = (t) => ((m.py[t] - m.cy) * m.k + m.h / 2) * m.dpr; // a tile's centre row, in canvas px
  const clans = m.stats.clans.map((s) => ({ clan: s.clan, home: v.clans[s.clan].home, bottom: s.y + s.h, row: centre(v.clans[s.clan].home) }));
  const layers = Object.keys(m.on);
  return {
    places: m.stats.places.map((p) => ({ tile: p.tile, role: p.role, civ: p.civ })),
    expected, probes, clans, flat: !!m.stats.clansFlat, towns: [...townSet],
    order: layers.indexOf('places') < layers.indexOf('clans'),
    civColour: Object.fromEntries(Object.values(v.civs).map((c) => [c.id, c.color])),
  };
})()`;

const page = await open('', { size: 'laptop' });
try {
  await stage(page, PLACES);
  const before = (await page.data()).turn;
  await page.key('Space');
  await page.until(`document.body.dataset.turn !== '${before}'`, 60000);
  await page.eval(PLACE_STATES);
  await page.eval(CAMERA(35));
  await page.until('!game.map.anim && !game.map.frame', 5000);
  const snap = await page.eval(SNAP);

  // Each place on screen once, of the role its tile holds by the rule, and no other.
  const placed = new Map(snap.places.map((p) => [p.tile, p]));
  const wrong = snap.expected.filter((e) => placed.get(e.tile)?.role !== e.role);
  need(snap.places.length === snap.expected.length && wrong.length === 0,
    `each place on screen is the role its tile holds (${snap.expected.map((e) => `${e.tile} ${e.role}`).join(', ')}${wrong.length ? `; wrong: ${wrong.map((e) => e.tile).join(', ')}` : ''})`);
  const towns = snap.expected.filter((e) => e.role === 'town').map((e) => e.tile);
  const both = snap.expected.filter((e) => e.role === 'capital' && snap.towns.includes(e.tile)).map((e) => e.tile);
  need(towns.length >= 1 && both.length >= 1, `a town on its own tile (${towns.join(', ')}) and a capital that reads as a town (${both.join(', ')})`);

  // The colours: each capital and settlement nearest its own civ, with tinted pixels; a town none.
  const colour = snap.probes.filter((p) => p.role !== 'town' && (String(p.nearest) !== String(p.civ) || !(p.tinted > 0)));
  need(snap.probes.length === snap.places.length && colour.length === 0,
    `each capital's and settlement's cloth is nearest its own civ's colour (${snap.probes.filter((p) => p.role !== 'town').map((p) => `${p.tile} ${p.civ}→${p.nearest}`).join(' ')})`);
  const tints = snap.probes.filter((p) => p.role === 'town');
  need(tints.length >= 1 && tints.every((p) => p.tinted === 0), `a town has no tint (${tints.map((p) => `${p.tile} ${p.tinted}`).join(', ')})`);

  // Each clan on a place's tile is drawn, and after the place: the clans layer follows places in LAYERS.
  const placeTiles = new Set(snap.places.map((p) => p.tile));
  const onPlaces = snap.clans.filter((c) => placeTiles.has(c.home));
  need(snap.order && onPlaces.length > 0 && snap.clans.length > 0 && !snap.flat,
    `the clans layer follows the places layer, and every clan on a place's tile is drawn (${onPlaces.length} clans on ${[...placeTiles].join(', ')})`);
  const behind = onPlaces.filter((c) => !(c.bottom > c.row));
  need(onPlaces.length > 0 && behind.length === 0, `every clan on a place's tile stands in its yard, its foot below the tile's centre (${onPlaces.length} clans${behind.length ? `; behind: ${behind.map((c) => c.clan).join(', ')}` : ''})`);
  const drawn = new Set(snap.clans.map((c) => c.clan));
  const missing = (await page.eval('Object.values(game.map.model.v.clans).map((c) => ({ id: c.id, home: c.home }))')).filter((c) => placeTiles.has(c.home) && !drawn.has(c.id));
  need(missing.length === 0, `every clan on a place's tile is in stats.clans (${missing.map((c) => c.id).join(', ') || 'none missing'})`);

  // A click on the settlement's tile reaches the settlement and each of its clans.
  const SETTLE = await page.eval('game.map.at(2, -1)');
  const sid = await page.eval(`game.map.model.v.settlements.find((s) => s.tile === ${SETTLE})?.id ?? null`);
  const clanIds = await page.eval(`Object.values(game.map.model.v.clans).filter((c) => c.home === ${SETTLE}).map((c) => c.id)`);
  const seen = new Set();
  for (let n = 0; n < 12 && !(seen.has(`settlement:${sid}`) && clanIds.every((id) => seen.has(`clan:${id}`))); n++) {
    await page.click(SETTLE);
    await page.eval(frames);
    const sel = (await page.data()).selected ?? '';
    if (sel) seen.add(sel);
  }
  need(sid != null && seen.has(`settlement:${sid}`) && clanIds.every((id) => seen.has(`clan:${id}`)),
    `a click on the settlement's tile reaches it and its clans (${[...seen].join(', ')})`);

  await page.eval(CAMERA(35));
  await page.eval(frames);
  const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
  writeFileSync(join(out, 'places.png'), Buffer.from(png.split(',')[1], 'base64'));
  console.log(`      the screenshot is ${join(out, 'places.png')}`);
  for (const e of page.errors()) {
    console.error(`places: page error: ${e}`);
    failed = true;
  }
} finally {
  await page.close();
}

// The flat zoom: the lineup on a phone at k 5 (its terrain flat), every place a mark, and each capital's and settlement's house
// drawn in its civ's colour at its tile. The clans and units are off, so their dots don't stand for the house.
const flat = await open('', { size: 'phone' });
try {
  await stage(flat, PLACES);
  const turn0 = (await flat.data()).turn;
  await flat.key('Space');
  await flat.until(`document.body.dataset.turn !== '${turn0}'`, 60000);
  await flat.eval(PLACE_STATES);
  await flat.eval(CAMERA(5));
  for (const l of ['clans', 'units', 'towns']) await flat.eval(`game.map.setLayer('${l}', false)`);
  await flat.until('!game.map.anim && !game.map.frame', 5000);
  const at5 = await flat.eval(SNAP);
  need(at5.flat && at5.places.length > 0 && at5.places.every((p) => p.role === 'mark'),
    `at the flat zoom every place is a mark (${at5.places.map((p) => `${p.tile} ${p.civ ?? 'none'}`).join(', ')})`);
  for (const tile of [47, 140]) {
    const house = await flat.eval(`(() => {
      const c = document.getElementById('map');
      const r = c.getBoundingClientRect();
      const p = game.map.screenOf(${tile});
      const s = c.width / r.width;
      const h = Math.round(4 * s);
      const x = Math.round((p.x - r.left) * s), y = Math.round((p.y - r.top) * s);
      const civ = game.map.model.v.civs[game.map.stats.places.find((q) => q.tile === ${tile}).civ];
      const want = [1, 3, 5].map((i) => parseInt(civ.color.slice(i, i + 2), 16));
      const d = c.getContext('2d').getImageData(x - h, y - h, 2 * h, 2 * h).data;
      let n = 0;
      for (let k = 0; k < d.length; k += 4) if (Math.abs(d[k] - want[0]) < 40 && Math.abs(d[k + 1] - want[1]) < 40 && Math.abs(d[k + 2] - want[2]) < 40) n++;
      return n;
    })()`);
    need(house >= 3, `the house on ${tile} is drawn in its civ's colour at the flat zoom (${house} pixels near the colour)`);
  }
  for (const e of flat.errors()) {
    console.error(`places: page error: ${e}`);
    failed = true;
  }
} finally {
  await flat.close();
}
process.exit(failed ? 1 : 0);
