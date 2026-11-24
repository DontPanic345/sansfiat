// Fixed map views, shot through play.mjs: each opens the page at its map and window, sets the camera on the
// plains tile nearest world (0, 0) and shots the map canvas itself (the pick's panel is not in it).
//
//   node www/shots.mjs --out DIR [--only NAME,…]    DIR/NAME.png per view, and DIR/views.json (each view's
//                                                   url, window, tile, cx, cy, k and game.map.stats)
//   node www/shots.mjs --diff A B                   two such folders: the cameras must match (exit 1 if not);
//                                                   prints each view's share of pixels that differ
//
// It is not in www/play/, so play/all.mjs (which check.sh runs) doesn't run it. Its --out imports only
// play.mjs, so this file copies alone onto a worktree of main and shoots there.
//
//   node www/shots.mjs --cull DIR                   each view with an `on` (mountain, shore and wood) twice in one page, the
//                                                   second with the edge, woods and ridge layers' culling off: DIR/NAME.png
//                                                   and DIR/NAME-nocull.png; exit 1 unless the two are byte-identical
//   node www/shots.mjs --time NAME,…                for each named view, five static redraws: each one's stats.terrainMs and
//                                                   the median; stats.woods where the branch has it (main prints -)
//   node www/shots.mjs --places DIR                 each view with `places` (the place views): DIR/NAME.png, and per place its tile,
//                                                   role, civ, whether it reads as a town, its clans, opaque, tinted, mean and nearest
//                                                   (the places layer's probe), then each view's digest and the count of each role; exit 1
//                                                   unless every capital and settlement is nearest its own civ and every role is its tile's.
//                                                   On main, which has no probe, prints '-'.
//   node www/shots.mjs --units DIR                  each view with `units` (the unit views): DIR/NAME.png, and per unit its kind,
//                                                   civ, terrain, opaque and tinted pixels, mean colour and nearest civ (the
//                                                   units layer's probe); exit 1 unless every unit is nearest its own civ and has at least TINT_BAR[k] tinted pixels.
//                                                   On main, which has no probe, prints '-'.
//   node www/shots.mjs --clans DIR                  each view with `clans` (the clan views): DIR/NAME.png, and per clan its kind,
//                                                   civ, terrain, starving, since, settled, tinted pixels, mean, nearest (the
//                                                   clans layer's probe), then each kind's count and least tinted pixels; exit 1 unless every clan is nearest
//                                                   its own civ (or none), of the kind its state gives and, as a cell, at least CLAN_BAR[k] tinted pixels. On main, prints '-'.
// A view with `turns` leads the first civ and ends that many turns before its camera is set; `lead: 'Gold'` leads that civ
// instead and `raise: N` raises an army of N warriors after leading (play.mjs's raiseArmy); `stage: 'lineup'` stages the
// lineup (LINEUP) and `stage: 'clans'` the clan lineup (CLANS) instead of a map; `states: true` then sets CLAN_STATES.
// `on: 'at'` puts the camera on tile `at`, or on the mean of the tiles in `between`.
// Every view's record holds the game's digest, which --diff compares too.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { open, lead, raiseArmy, SIZES } from './play.mjs';
import { stage } from './play-stage.mjs';
import { VIEWS, LINEUP, CLANS, CLAN_STATES, PLACES, PLACE_STATES } from './shots-views.mjs';

// The views table, and the lineups it stages, are in shots-views.mjs, which this file imports: both copy onto a main
// worktree together.

/**
 * In the page: sets the camera for `view` (`{on, k, edge}`) and returns `{ tile, cx, cy, k }` as asked. The chooser
 * is proto/nearest.mjs's rule: the class's tile nearest world (0, 0), the lower index on a tie.
 */
const CAMERA = `(view) => {
  const m = game.map, M = m.byName.mountain, n = m.px.length, l = m.l;
  const classOf = (i) => {
    const ds = [];
    for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && m.l.terrain[j] === M) ds.push(d); }
    if (ds.length === 0) return 'iso';
    if (ds.length === 1) return 'end';
    if (ds.length === 2) { const g = Math.min((ds[1] - ds[0] + 6) % 6, (ds[0] - ds[1] + 6) % 6); return g === 3 ? 'straight' : g === 2 ? 'bend' : 'sharp'; }
    return ds.length === 3 ? 'fork' : '4+';
  };
  // Water is terrain 0 and off the map counts as water, as proto/shore.mjs counts a shore.
  const runsOf = (i) => {
    const w = [];
    for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; w.push(j < 0 || l.terrain[j] === 0); }
    let ch = 0;
    for (let d = 0; d < 6; d++) if (w[d] !== w[(d + 1) % 6]) ch++;
    return { water: w.filter((x) => x).length, runs: ch / 2 };
  };
  const lakeSize = (i) => {
    const seen = new Set([i]), st = [i];
    while (st.length && seen.size < 4) {
      const a = st.pop();
      for (let d = 0; d < 6; d++) { const b = m.nb[a * 6 + d]; if (b >= 0 && l.terrain[b] === 0 && l.lake[b] && !seen.has(b)) { seen.add(b); st.push(b); } }
    }
    return seen.size;
  };
  const mouthInto = (i, lake) => l.river[i] > 0 && l.flow[i] >= 0 && l.terrain[l.flow[i]] === 0 && !!l.lake[l.flow[i]] === lake;
  const SHORE = {
    neck: (i) => l.terrain[i] !== 0 && runsOf(i).runs >= 2,
    strait: (i) => l.terrain[i] === 0 && runsOf(i).runs >= 2,
    bay: (i) => l.terrain[i] === 0 && !l.lake[i] && runsOf(i).water <= 2 && runsOf(i).runs === 1,
    pond: (i) => l.terrain[i] === 0 && !!l.lake[i] && lakeSize(i) <= 3,
    'mouth-sea': (i) => mouthInto(i, false),
    'mouth-lake': (i) => mouthInto(i, true),
    marsh: (i) => l.terrain[i] === m.byName.marsh,
  };
  // The wood classes (map/woods.js's rule): a forest tile by its forest neighbours, a river through it, or its field plots.
  const FOREST = m.byName.forest, MARSH = m.byName.marsh;
  const plots = new Set(); for (const c of Object.values(game.view().clans)) for (const [i] of c.plots) plots.add(i);
  const forestNbs = (i) => { let f = 0; for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && l.terrain[j] === FOREST) f++; } return f; };
  const riverAt = (i) => l.river[i] > 0 && l.flow[i] >= 0;
  const intoRiver = (i) => { for (let d = 0; d < 6; d++) { const j = m.nb[i * 6 + d]; if (j >= 0 && riverAt(j) && l.flow[j] === i) return true; } return false; };
  const WOOD = {
    interior: (i) => l.terrain[i] === FOREST && forestNbs(i) === 6,
    lone: (i) => l.terrain[i] === FOREST && forestNbs(i) === 0,
    'forest-river': (i) => l.terrain[i] === FOREST && (riverAt(i) || intoRiver(i)),
    'forest-fields': (i) => l.terrain[i] === FOREST && plots.has(i),
    'marsh-fields': (i) => l.terrain[i] === MARSH && plots.has(i),
  };
  let tile = -1, cx, cy;
  if (view.on === 'at') {
    // The named tile, or the mean of the tiles named (no tile then: -1).
    const ts = view.between ?? [view.at];
    cx = ts.reduce((a, i) => a + m.px[i], 0) / ts.length;
    cy = ts.reduce((a, i) => a + m.py[i], 0) / ts.length;
    tile = view.between ? -1 : view.at;
  } else {
    const pick = view.on === 'plains' ? (i) => l.terrain[i] === m.byName.plains : SHORE[view.on] ?? WOOD[view.on] ?? ((i) => l.terrain[i] === M && classOf(i) === view.on);
    let bd = Infinity;
    for (let i = 0; i < n; i++) {
      if (!pick(i)) continue;
      const d = m.px[i] * m.px[i] + m.py[i] * m.py[i];
      if (d < bd) { bd = d; tile = i; }
    }
    if (tile < 0) throw new Error(\`the map has no \${view.on} tile\`);
    cx = m.px[tile]; cy = m.py[tile];
  }
  if (view.edge?.side === 'bottom') cy = m.py[tile] - m.h / 2 / view.k - view.edge.by;
  if (view.edge?.side === 'right') cx = m.px[tile] - m.w / 2 / view.k - view.edge.by;
  m.cx = cx; m.cy = cy; m.k = view.k; m.viewChanged();
  return { tile, cx, cy, k: view.k };
}`;

/** Ends `n` turns: each waits for the turn number to move on (proto/fields.mjs's loop). */
async function endTurns(page, n) {
  for (let t = 0; t < n; t++) {
    const before = (await page.data()).turn;
    await page.key('Space');
    await page.until(`document.body.dataset.turn !== '${before}'`, 60000);
  }
}

/** Leads the first civ and ends `n` turns. */
async function playTurns(page, n) {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  await endTurns(page, n);
}

/** Opens `view`'s page, plays its turns, sets its camera and waits for it to settle: the page, the camera's tile, and
 *  the game's digest. */
async function setUp(view) {
  const url = `?size=${view.size}&seed=${view.seed}&run=1`;
  let page;
  if (view.stage === 'lineup') {
    page = await open('', { size: view.window });
    await stage(page, LINEUP);
  } else if (view.stage === 'places') {
    page = await open('', { size: view.window });
    await stage(page, PLACES);
    if (view.turns) await endTurns(page, view.turns);
    if (view.states) await page.eval(PLACE_STATES);
  } else if (view.stage === 'clans') {
    page = await open('', { size: view.window });
    await stage(page, CLANS);
    if (view.states) await page.eval(CLAN_STATES);
  } else if (view.lead || view.raise) {
    page = await open('', { size: view.window });
    await lead(page, { size: view.size, seed: view.seed, civ: view.lead });
    if (view.raise) await raiseArmy(page, { warriors: view.raise, turns: 10 });
  } else {
    page = await open(url, { size: view.window });
    await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
    if (view.turns) await playTurns(page, view.turns);
  }
  // Raising pans to the civ: the camera is set once the map has settled.
  await page.until('!game.map.anim && !game.map.frame', 10000);
  const asked = await page.eval(`(${CAMERA})(${JSON.stringify({ on: view.on ?? 'plains', k: view.k, edge: view.edge, at: view.at, between: view.between })})`);
  await page.until('!game.map.anim && !game.map.frame', 5000);
  const cam = await page.eval('(() => { const m = game.map; return { cx: m.cx, cy: m.cy, k: m.k }; })()');
  // The pan clamp moves a camera it doesn't like: the camera must be the one asked for, or the view is not the view named.
  if (cam.k !== asked.k || cam.cx !== asked.cx || cam.cy !== asked.cy) {
    throw new Error(`${view.name}: asked for camera ${JSON.stringify(asked)}, the map gave ${JSON.stringify(cam)} (minK ${await page.eval('game.map.minK')})`);
  }
  if (view.tile !== undefined && asked.tile !== view.tile) throw new Error(`${view.name}: the tile is ${asked.tile}, the view names ${view.tile}`);
  // `select` presses a unit on the map: the first unit of that `kind` and civ (the civ's mark colour name, or its index), or on `tile`.
  if (view.select) {
    const picked = await page.eval(`((sel) => {
      const v = game.view(), us = Object.values(v.units);
      const u = us.find((x) => (sel.tile === undefined || x.tile === sel.tile) && (!sel.kind || x.kind === sel.kind)
        && (sel.civ === undefined || x.civ === (typeof sel.civ === 'string' ? Object.values(v.civs).find((c) => c.capital === game.map.at(...({ red: [-5, 0], blue: [0, -5], gold: [0, 5] })[sel.civ])).id : sel.civ)));
      if (!u) return null;
      game.map.setSelection(u.tile, u.id);
      game.map.drawNow();
      return u.id;
    })(${JSON.stringify(view.select)})`);
    if (picked == null) throw new Error(`${view.name}: no unit to select`);
  }
  const digest = await page.eval('game.digest()');
  return { page, url, asked, cam, digest };
}

/** Shoots one view into `out`: its PNG, and its record for views.json. */
async function shoot(view, out) {
  const { page, url, asked, cam, digest } = await setUp(view);
  try {
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const stats = await page.eval('({ ...game.map.stats })');
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    return { url, window: view.window, tile: asked.tile, cx: cam.cx, cy: cam.cy, k: cam.k, digest, stats };
  } finally {
    await page.close();
  }
}

/** Shoots a view with the edge, woods and ridge layers culling and again without, in one page; true if the PNGs match. */
async function cull(view, out) {
  const { page, asked } = await setUp(view);
  try {
    const grab = "(() => { game.map.drawNow(); const s = game.map.stats; return { png: game.map.canvas.toDataURL('image/png'), ridges: s.ridges, edges: s.edges, woods: s.woods ?? '-' }; })()";
    const culled = await page.eval(grab);
    await page.eval('(() => { game.map.noCull = true; game.map.dirty = true; })()');
    const uncut = await page.eval(grab);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(culled.png.split(',')[1], 'base64'));
    writeFileSync(join(out, `${view.name}-nocull.png`), Buffer.from(uncut.png.split(',')[1], 'base64'));
    const same = culled.png === uncut.png;
    console.log(`${view.name}  tile ${asked.tile}  stats.ridges ${culled.ridges} culled, ${uncut.ridges} unculled  stats.edges ${culled.edges} culled, ${uncut.edges} unculled  stats.woods ${culled.woods} culled, ${uncut.woods} unculled  ${same ? 'identical' : 'DIFFER'}`);
    return same;
  } finally {
    await page.close();
  }
}

/** Five static redraws of a view, each one's terrain time, and the median; prints them with stats.woods. */
async function time(view) {
  const { page, asked } = await setUp(view);
  try {
    const r = await page.eval('(() => { const m = game.map, ms = []; for (let i = 0; i < 5; i++) { m.dirty = true; m.drawNow(); ms.push(m.stats.terrainMs); } return { ms, woods: m.stats.woods ?? null }; })()');
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    const median = [...r.ms].sort((a, b) => a - b)[2];
    console.log(`${view.name}  tile ${asked.tile}  terrainMs ${r.ms.map((x) => x.toFixed(1)).join(' ')}  median ${median.toFixed(1)}  stats.woods ${r.woods ?? '-'}`);
  } finally {
    await page.close();
  }
}

/** The least tinted pixels a figure has, by the view's zoom: enough that the views show the civ's colour reading (relay/specs/443/views.md). */
const TINT_BAR = { 35: 350, 18: 100, 6: 40, 5: 40 };

/** The least tinted pixels a clan's cell has, by the view's zoom (relay/specs/444/views.md). A dot at the flat zoom is main's circle, whole. */
const CLAN_BAR = { 35: 100, 18: 40 };

/** Shoots a unit view into `out` and prints each unit's probe; false if a unit is nearest another civ's colour. On main,
 *  which has no probe, prints '-' and passes. */
async function units(view, out) {
  const { page, asked } = await setUp(view);
  try {
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const rows = await page.eval(`(() => {
      const m = game.map;
      if (typeof m.unitProbe !== 'function') return null;
      const v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
      return m.unitProbe().map((p) => ({ ...p, terrain: names[m.l.terrain[v.units[p.unit].tile]] }));
    })()`);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    if (rows === null) {
      console.log(`${view.name}  tile ${asked.tile}  -`);
      return true;
    }
    let ok = true;
    const bar = TINT_BAR[view.k] ?? 0;
    for (const r of rows) {
      const own = r.nearest === r.civ;
      const enough = r.tinted >= bar;
      if (!own || !enough) ok = false;
      const share = r.opaque ? ((100 * r.tinted) / r.opaque).toFixed(1) : '-';
      const mean = r.mean ? r.mean.map((x) => Math.round(x)).join(',') : '-';
      console.log(`${view.name}  unit ${r.unit}  ${r.kind}  civ ${r.civ}  on ${r.terrain}  opaque ${r.opaque}  tinted ${r.tinted}  share ${share}%  mean ${mean}  nearest ${r.nearest}  ${!own ? 'NOT ITS OWN CIV' : enough ? 'ok' : `BELOW THE BAR (${bar})`}`);
    }
    return ok;
  } finally {
    await page.close();
  }
}

/** Shoots a place view into `out` and prints each place's probe beside the rule for its tile: a living capital, else a
 *  settlement, else a town. False if a capital or settlement is nearest another civ's colour, or a place's role is not its
 *  tile's. On main, which has no probe, prints '-' and passes. */
async function places(view, out) {
  const { page, asked } = await setUp(view);
  try {
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const res = await page.eval(`(() => {
      const m = game.map;
      if (typeof m.placeProbe !== 'function') return null;
      const v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
      const capOf = new Map(Object.values(v.civs).filter((c) => !c.fallen).map((c) => [c.capital, c.id]));
      const setOf = new Map(v.settlements.map((s) => [s.tile, s.civ]));
      const townSet = new Set(v.towns.map((t) => t.tile));
      const rule = (tile) => (capOf.has(tile) ? ['capital', capOf.get(tile)] : setOf.has(tile) ? ['settlement', setOf.get(tile)] : townSet.has(tile) ? ['town', null] : null);
      const probes = m.placeProbe();
      const rows = m.stats.places.map((rec) => {
        const p = probes.find((x) => x.tile === rec.tile) ?? { opaque: 0, tinted: 0, mean: null, nearest: null };
        const want = rule(rec.tile);
        const clans = Object.values(v.clans).filter((c) => c.home === rec.tile).map((c) => c.id);
        return { ...rec, ...p, terrain: names[m.l.terrain[rec.tile]], reads: townSet.has(rec.tile), clans, wantRole: want?.[0] ?? null, wantCiv: want?.[1] ?? null };
      });
      const onScreen = [...capOf.keys(), ...setOf.keys(), ...townSet].filter((t, i, a) => a.indexOf(t) === i && m.onScreen(t)).length;
      return { digest: game.digest(), onScreen, rows };
    })()`);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    if (res === null) {
      console.log(`${view.name}  tile ${asked.tile}  digest ${res ?? '-'}  -`);
      return true;
    }
    let ok = true;
    const roles = {};
    for (const r of res.rows) {
      // A mark (the flat zoom) is the capital's or settlement's house: its role is the tile's, with no cloth to measure.
      const mark = r.role === 'mark';
      const roleOk = mark ? r.wantRole === 'capital' || r.wantRole === 'settlement' : r.role === r.wantRole;
      const own = mark || (r.role === 'town' ? r.nearest == null : String(r.nearest) === String(r.civ));
      const cloth = mark || (r.role === 'town' ? r.tinted === 0 : r.tinted > 0);
      const fine = roleOk && own && cloth;
      if (!fine) ok = false;
      roles[mark ? 'mark' : r.role] = (roles[mark ? 'mark' : r.role] ?? 0) + 1;
      const mean = r.mean ? r.mean.map((x) => Math.round(x)).join(',') : '-';
      const verdict = !roleOk ? `ROLE NOT ITS TILE'S (${r.wantRole ?? 'none'})` : !own ? 'NOT ITS OWN CIV' : !cloth ? 'NO CLOTH AS ITS ROLE GIVES' : 'ok';
      console.log(`${view.name}  tile ${r.tile}  ${r.role}  civ ${r.civ ?? 'none'}  on ${r.terrain}  reads as a town ${r.reads}  clans ${r.clans.join(',') || '-'}  opaque ${r.opaque}  tinted ${r.tinted}  mean ${mean}  nearest ${r.nearest ?? '-'}  ${verdict}`);
    }
    console.log(`${view.name}  tile ${asked.tile}  digest ${res.digest}  places on screen ${res.onScreen}  roles: ${Object.entries(roles).map(([k, n]) => `${k} ${n}`).join(', ') || 'none'}`);
    return ok;
  } finally {
    await page.close();
  }
}

/** Shoots a clan view into `out` and prints each clan's probe beside its state; false if a clan is nearest another civ's
 *  colour (none's, for a clan of no civ), or drawn as another kind than its state gives. On main, which has no probe,
 *  prints '-' and passes. */
async function clansView(view, out) {
  const { page, asked } = await setUp(view);
  try {
    const png = await page.eval("(() => { game.map.drawNow(); return game.map.canvas.toDataURL('image/png'); })()");
    writeFileSync(join(out, `${view.name}.png`), Buffer.from(png.split(',')[1], 'base64'));
    const rows = await page.eval(`(() => {
      const m = game.map;
      if (typeof m.clanProbe !== 'function') return null;
      const names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
      const probes = m.clanProbe(), flat = !!m.stats.clansFlat;
      return probes.map((p) => {
        const c = m.model.v.clans[p.clan];
        const expect = flat ? 'dot' : c.since <= 1 ? 'walk' : c.settled ? 'hearth' : 'tent';
        return { ...p, id: p.clan, terrain: names[m.l.terrain[c.home]], starving: !!c.starving, since: c.since, settled: !!c.settled, expect };
      });
    })()`);
    for (const e of page.errors()) throw new Error(`${view.name}: page error: ${e}`);
    if (rows === null) {
      console.log(`${view.name}  tile ${asked.tile}  -`);
      return true;
    }
    let ok = true;
    const kinds = {}, tints = {};
    for (const r of rows) {
      const own = String(r.nearest) === String(r.civ ?? 'none');
      const kindOk = r.kind === r.expect;
      const bar = r.kind === 'dot' ? 0 : CLAN_BAR[view.k] ?? 0;
      const enough = r.tinted >= bar;
      if (!own || !kindOk || !enough) ok = false;
      const t = (tints[r.kind] ??= { min: Infinity, shares: [] });
      t.min = Math.min(t.min, r.tinted);
      if (r.opaque) t.shares.push((100 * r.tinted) / r.opaque);
      kinds[r.kind] = (kinds[r.kind] ?? 0) + 1;
      const share = r.opaque ? ((100 * r.tinted) / r.opaque).toFixed(1) : '-';
      const mean = r.mean ? r.mean.map((x) => Math.round(x)).join(',') : '-';
      const verdict = !own ? 'NOT ITS OWN CIV' : !kindOk ? `KIND NOT ITS STATE'S (${r.expect})` : enough ? 'ok' : `BELOW THE BAR (${bar})`;
      console.log(`${view.name}  clan ${r.id}  ${r.kind}  civ ${r.civ ?? 'none'}  on ${r.terrain}  starving ${r.starving}  since ${r.since}  settled ${r.settled}  opaque ${r.opaque}  tinted ${r.tinted}  share ${share}%  mean ${mean}  nearest ${r.nearest}  ${verdict}`);
    }
    console.log(`${view.name}  kinds: ${Object.entries(kinds).map(([k, n]) => `${k} ${n}`).join(', ')}`);
    for (const [k, t] of Object.entries(tints)) console.log(`${view.name}  ${k}: least tinted ${t.min}, share ${Math.min(...t.shares).toFixed(0)}–${Math.max(...t.shares).toFixed(0)}%${CLAN_BAR[view.k] && k !== 'dot' ? `, bar ${CLAN_BAR[view.k]}` : ''}`);
    return ok;
  } finally {
    await page.close();
  }
}

/** Compares two output folders: cameras must match; prints each view's share of differing pixels. */
async function diff(a, b) {
  const { decode } = await import('./assets/png.mjs');
  const va = JSON.parse(readFileSync(join(a, 'views.json'), 'utf8'));
  const vb = JSON.parse(readFileSync(join(b, 'views.json'), 'utf8'));
  let failed = false;
  for (const name of Object.keys(va)) {
    if (!(name in vb)) {
      console.log(`${name}  only in ${a}`);
      continue;
    }
    const cam = (v) => ({ tile: v.tile, cx: v.cx, cy: v.cy, k: v.k });
    if (JSON.stringify(cam(va[name])) !== JSON.stringify(cam(vb[name]))) {
      console.error(`${name}  cameras differ: ${JSON.stringify(cam(va[name]))} vs ${JSON.stringify(cam(vb[name]))}`);
      failed = true;
      continue;
    }
    if (va[name].digest !== undefined && vb[name].digest !== undefined && va[name].digest !== vb[name].digest) {
      console.error(`${name}  digests differ: ${va[name].digest} vs ${vb[name].digest}`);
      failed = true;
      continue;
    }
    const x = decode(readFileSync(join(a, `${name}.png`))), y = decode(readFileSync(join(b, `${name}.png`)));
    const sizes = `${x.width}×${x.height} vs ${y.width}×${y.height}`;
    if (x.width !== y.width || x.height !== y.height) {
      console.log(`${name}  sizes differ (${sizes})`);
      continue;
    }
    let differ = 0;
    for (let i = 0; i < x.width * x.height; i++) {
      const o = i * 4;
      if (x.data[o] !== y.data[o] || x.data[o + 1] !== y.data[o + 1] || x.data[o + 2] !== y.data[o + 2] || x.data[o + 3] !== y.data[o + 3]) differ++;
    }
    const share = +((100 * differ) / (x.width * x.height)).toFixed(3);
    console.log(`${name}  ${share}% differ  (${differ} of ${x.width * x.height} pixels; ${sizes})`);
  }
  for (const name of Object.keys(vb)) if (!(name in va)) console.log(`${name}  only in ${b}`);
  if (failed) process.exit(1);
}

const args = process.argv.slice(2);
if (args.includes('--cull')) {
  const at = args.indexOf('--cull');
  const out = resolve(args[at + 1]);
  mkdirSync(out, { recursive: true });
  let failed = false;
  for (const view of VIEWS) {
    if (!view.on || view.on === 'at') continue; // the units views have no culled layer
    if (!(await cull(view, out))) failed = true;
  }
  if (failed) {
    console.error('shots --cull: a mountain view draws differently with culling off');
    process.exit(1);
  }
} else if (args.includes('--units')) {
  const out = resolve(args[args.indexOf('--units') + 1]);
  mkdirSync(out, { recursive: true });
  let failed = false;
  for (const view of VIEWS) {
    if (!view.units) continue;
    if (!(await units(view, out))) failed = true;
  }
  if (failed) {
    console.error('shots --units: a unit is drawn nearest another civ\'s colour');
    process.exit(1);
  }
} else if (args.includes('--places')) {
  const out = resolve(args[args.indexOf('--places') + 1]);
  mkdirSync(out, { recursive: true });
  let failed = false;
  for (const view of VIEWS) {
    if (!view.places) continue;
    if (!(await places(view, out))) failed = true;
  }
  if (failed) {
    console.error('shots --places: a capital or settlement is nearest another civ\'s colour, or a place\'s role is not its tile\'s');
    process.exit(1);
  }
} else if (args.includes('--clans')) {
  const out = resolve(args[args.indexOf('--clans') + 1]);
  mkdirSync(out, { recursive: true });
  let failed = false;
  for (const view of VIEWS) {
    if (!view.clans) continue;
    if (!(await clansView(view, out))) failed = true;
  }
  if (failed) {
    console.error('shots --clans: a clan is drawn nearest another civ\'s colour, or as another kind than its state gives');
    process.exit(1);
  }
} else if (args.includes('--diff')) {
  const at = args.indexOf('--diff');
  await diff(resolve(args[at + 1]), resolve(args[at + 2]));
} else if (args.includes('--time')) {
  const names = args[args.indexOf('--time') + 1]?.split(',') ?? [];
  for (const name of names) {
    const view = VIEWS.find((v) => v.name === name);
    if (!view) throw new Error(`no view ${name}`);
    await time(view);
  }
} else {
  const at = args.indexOf('--out');
  if (at < 0 || !args[at + 1]) {
    console.error('usage: node www/shots.mjs --out DIR [--only NAME,…]  |  node www/shots.mjs --diff A B');
    process.exit(2);
  }
  const out = resolve(args[at + 1]);
  const only = args.indexOf('--only') >= 0 ? args[args.indexOf('--only') + 1].split(',') : null;
  mkdirSync(out, { recursive: true });
  const views = {};
  for (const view of VIEWS) {
    if (only && !only.includes(view.name)) continue;
    views[view.name] = await shoot(view, out);
    console.log(`${view.name}  k ${views[view.name].k}  tile ${views[view.name].tile}  ${SIZES[view.window].width}×${SIZES[view.window].height}`);
  }
  writeFileSync(join(out, 'views.json'), `${JSON.stringify(views, null, 1)}\n`);
}
