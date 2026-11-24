// The camera and the page set-up every view shares: setUp opens a view's page, stages or plays it, sets its camera and waits
// for it to settle. shots.mjs (the views' PNGs, --diff, --time and the probes) and shots-probe.mjs import it, so
// this file copies with them onto a control tree (proto/control.sh copies every shots*.mjs).
import { open, lead, raiseArmy } from './play.mjs';
import { stage } from './play-stage.mjs';
import { LINEUP, CLANS, CLAN_STATES, PLACES, PLACE_STATES } from './shots-views.mjs';

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
export async function setUp(view) {
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

