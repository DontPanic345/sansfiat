// The play screen: the map with its bar, the bottom panel, the feed and the pick over the map, and the
// turn loop. It holds the game in progress: the Sim, its view model, the selection, and the keys. The
// screens around it (start, end) hand it a Sim and read back what it holds through `state()`.

import { MapView } from '../map/view.js';
import { build } from '../model.js';
import { Feed } from '../feed.js';
import { f2, keyOf, tileOf, alive, candidates, describe, tileKind, marketOn, clanRole, unitOrder } from '../select.js';
import { app } from '../app.js';
import { autosave } from '../save.js';
import { legendHtml } from '../legend.js';
import * as bar from '../bar.js';
import { Pick } from './pick.js';

const $ = (id) => document.getElementById(id);
const RATES = { slow: 1, normal: 4, fast: 20 };

let view = null;
let feed = null;
let pick = null;
let sim = null;
let layout = null;
let model = null; // the view model of the last change (model.js)
let v = null; // its Sim.view()
let cursor = 0; // into Sim.events
let selected = null; // {kind: 'unit', id} | {kind: 'capital', civ} | {kind: 'tile', tile, clan, market}
let routing = { arm: null, start: null, unit: null }; // the selected scout's route: a map click arms 'start' or 'to'
let outcome = null; // how the player's last scout route ended: 'found' | 'nopath' | null
let playing = false;
let running = false;
let loopTimer = null;
let last = 0;
let acc = 0;
let country = false; // View → Hidden country: the closed-country outlines, their names and legend rows
/** 'paint', 'unpaint' (a camp's zone), 'claim', 'unclaim' (the player's civ's claim) or null: what a
 *  left press and drag on the map do. */
let painting = null;

/** Builds the map, the feed, the pick and the bar, once, before the first screen shows. */
export function init() {
  const { TERRAIN, RULES } = app.data;
  view = new MapView($('map'), {
    terrain: TERRAIN,
    rules: RULES,
    atlas: app.atlas,
    onPick,
    onCommand,
    onPaint,
    onHover: showTip,
    onHoverEnd: hideTip,
  });
  feed = new Feed({
    list: $('feed'),
    filters: $('filters'),
    alerts: $('alerts'),
    onClick: onFeed,
    mine: (e) => v?.player != null && e.civ === v.player,
    onUnread: bar.feedCount,
  });
  pick = new Pick({ lead: (civ) => chooseCiv(civ), show: showCiv, watch: () => chooseCiv(null) });
  bar.init(
    {
      end: endTurn,
      play: togglePlay,
      skip: runToEnd,
      speed: () => {
        last = performance.now();
        acc = 0;
      },
      selectCiv: (civ) => {
        const c = v?.civs[civ];
        if (!c) return;
        select({ kind: 'capital', civ });
        view.centreOn(c.capital, { minK: 20, smooth: true });
      },
      newGame: () => app.show('start'),
      saveFile,
      standing,
      layer: (name, on) => view.setLayer(name, on),
      country: setCountry,
    },
    view,
  );
  new ResizeObserver(() => view.resize()).observe($('mapbox'));
  $('legenditems').innerHTML = legendHtml(false);
  $('load').onchange = loadFile;
  $('zin').onclick = () => view.zoomBy(1.4);
  $('zout').onclick = () => view.zoomBy(1 / 1.4);
  $('fit').onclick = () => view.fit();
  $('feedtoggle').onclick = () => showFeed($('side').classList.contains('collapsed'));
  $('hidefeed').onclick = () => showFeed(false);
  $('showfeed').onclick = () => showFeed(true);
  // At phone width the feed is a drawer, closed by default.
  showFeed(!matchMedia('(max-width: 760px)').matches);
  $('detail').addEventListener('click', (e) => {
    const p = e.target.closest('[data-paint]');
    if (p) {
      setPainting(painting === p.dataset.paint ? null : p.dataset.paint);
      return;
    }
    const b = e.target.closest('[data-select]');
    if (!b) return;
    const [kind, id] = b.dataset.select.split(':');
    if (kind === 'unit' && Number(id) in v.units) select({ kind: 'unit', id: Number(id) });
    else if (kind === 'clan' && Number(id) in v.clans) select({ kind: 'tile', tile: v.clans[id].home, clan: Number(id) });
  });
  window.addEventListener('keydown', keys);
  // A button blurs after a click, so a space never presses it again.
  document.addEventListener('click', (e) => e.target.closest?.('button, summary')?.blur());
}

/** Shows the game: `args.sim` starts it (`history` for a loaded autosave, `speed`, `run` to skip the
 *  pre-game, `play` to run it); no args returns to the game in progress. */
export function show(args = null) {
  if (args?.sim) {
    begin(args.sim, args);
    return;
  }
  if (view) {
    view.resize();
    view.drawNow();
  }
}

/** Leaving the screen stops the run, which writes the autosave once. */
export function hide() {
  stop();
}

/** The game in progress, for `window.game` and the screens: null before the first game. */
export function state() {
  return { sim, layout, model, v };
}

export const map = () => view;
export const feedList = () => feed;

/** Starts a game: `sim` replaces the one in progress. */
function begin(next, { history = null, speed = null, run = false, play: go = false } = {}) {
  stop();
  sim = next;
  layout = JSON.parse(sim.layout());
  model = history ? { history } : null;
  selected = null;
  routing = { arm: null, start: null, unit: null };
  outcome = null;
  painting = null;
  view.setLayout(layout);
  view.setHighlight(null);
  view.setSelection(-1);
  view.setPainting(false);
  document.body.dataset.selected = '';
  document.body.dataset.painting = '';
  cursor = 0;
  feed.reset();
  if (speed && (speed === 'max' || Object.hasOwn(RATES, speed))) $('speed').value = speed;
  $('error').textContent = '';
  refresh();
  view.fit();
  view.drawNow();
  if (run) runToEnd();
  else if (go) setPlaying(true);
}

// ---- the turn --------------------------------------------------------------------------------

/** Re-reads the world after turns: the bar, the pick, the feed, the panel, the map. */
function refresh() {
  const { RULES } = app.data;
  model = build(sim, layout, RULES, model);
  v = model.v;
  view.setModel(model);
  const centres = v.end ? v.end.centres.length : v.summary.centres.length;
  const mine = model.mine;
  const warriors = mine.reduce((a, u) => a + u.people, 0);
  document.body.dataset.turn = v.turn;
  document.body.dataset.phase = v.phase;
  document.body.dataset.centres = centres;
  document.body.dataset.civ = v.player ?? '';
  document.body.dataset.units = mine.length;
  document.body.dataset.warriors = Math.round(warriors);
  routes();
  pick.update(sim, v.phase);
  if (selected && !alive(ctx(), selected)) {
    selected = null;
    syncSelection();
  }
  pullEvents();
  const scout = v.player != null ? Object.values(v.units).find((u) => u.civ === v.player && u.kind === 'scout') : null;
  document.body.dataset.scout = scout ? scout.id : '';
  document.body.dataset.routeState = scout?.route ? 'set' : (outcome ?? 'none');
  document.body.dataset.armed = routing.arm ?? '';
  markChips();
  panel();
  view.drawNow();
}

/** The bar, with the selected civ's chip marked. */
function markChips() {
  if (!v) return;
  bar.update({ v, model, selectedCiv: selected?.kind === 'capital' ? selected.civ : null });
}

/** Hands the map the player's camps' routes (the selected one drawn firmly, the rest faint), and the
 *  selected camp's line home. */
function routes() {
  const out = [];
  for (const u of Object.values(v.units)) {
    if (u.civ !== v.player || u.kind !== 'camp' || u.dest == null) continue;
    const r = JSON.parse(sim.route(u.id));
    if (r) out.push({ unit: u.id, civ: u.civ, tiles: [u.tile, ...r.path], selected: selected?.kind === 'unit' && selected.id === u.id });
  }
  view.setRoutes(out);
  // The player's camp's line home, always faint; the selected clan's territory and route bright.
  const mineCamp = v.player != null ? Object.values(v.units).find((u) => u.civ === v.player && u.kind === 'camp') : null;
  view.setHomeLine(mineCamp?.line?.path.length ? [mineCamp.tile, ...mineCamp.line.path] : null);
  view.setFocus(selected?.kind === 'tile' ? (selected.clan ?? view.clanAt(selected.tile)) : null);
  // A selected festival (a market's tile, or its row): its walkers' routes are drawn bright.
  const festival = selected?.kind === 'tile' && selected.clan == null ? (selected.market ?? marketOn(ctx(), selected.tile)?.id) : null;
  view.setFocusMarket(festival ?? null);
  const camp = selected?.kind === 'unit' ? v.units[selected.id] : null;
  if (camp && camp.cut_at) {
    // Cut by held ground: the line it would have, drawn to the tile where it stops, marked.
    const stop = camp.cut_at.path.indexOf(camp.cut_at.at);
    view.setLine({ tiles: [camp.tile, ...camp.cut_at.path.slice(0, stop + 1)], colour: lineColour(camp), cut: camp.cut_at.at });
  } else {
    view.setLine(camp && camp.line && camp.line.path.length ? { tiles: [camp.tile, ...camp.line.path], colour: lineColour(camp) } : null);
  }
  // The selected scout's route, from where its search starts: known and possible.
  const scout = selected?.kind === 'unit' ? v.units[selected.id] : null;
  if (scout?.plan) {
    const [from] = scout.route;
    const tilesFrom = (p) => (p ? [from, ...p.path] : null);
    view.setPlan({ known: tilesFrom(scout.plan.known), possible: tilesFrom(scout.plan.possible) });
  } else {
    view.setPlan(null);
  }
}

/** Gives the engine an order; shows why not in #error. Writes the autosave after one that goes through. */
function order(action) {
  let ok = true;
  try {
    sim.act(JSON.stringify(action));
    $('error').textContent = '';
    if (action.type === 'order' && (action.order.type === 'route' || action.order.type === 'clearRoute')) {
      outcome = null;
      routing.arm = null;
      routing.start = null;
    }
  } catch (e) {
    $('error').textContent = String(e);
    ok = false;
  }
  refresh();
  if (ok) autosave.write(sim, model);
  return ok;
}

/** The pick itself: lead a civ (selecting its capital) or watch all three (`null`). */
function chooseCiv(civ) {
  if (!order({ type: 'chooseCiv', civ })) return;
  view.setHighlight(null);
  const c = civ != null && v.civs[civ];
  if (c) {
    select({ kind: 'capital', civ });
    view.centreOn(c.capital, { minK: 20, smooth: true });
  }
}

/** "Show": centres the map on a civ's capital and highlights its land. */
function showCiv(civ) {
  const c = v.civs[civ];
  if (!c) return;
  const tiles = [];
  for (let i = 0; i < layout.q.length; i++) if (sim.tile_civ(i) === civ) tiles.push(i);
  view.setHighlight({ tiles, tile: c.capital });
  view.centreOn(c.capital, { minK: 12, smooth: true });
}

function pullEvents() {
  const ev = JSON.parse(sim.events(cursor));
  cursor += ev.length;
  for (const e of ev) {
    if (e.civ !== v.player) continue;
    if (e.kind === 'scoutRouteFound') outcome = 'found';
    if (e.kind === 'scoutNoRoute') outcome = 'nopath';
  }
  if (ev.length) {
    feed.add(ev);
    app.sound.hear(ev);
  }
}

/** Ends `n` turns; shows why if refused. Returns whether it all went through. */
function turns(n) {
  try {
    for (let i = 0; i < n && sim.phase() !== 'choose'; i++) sim.act('{"type":"endTurn"}');
    $('error').textContent = '';
    return true;
  } catch (e) {
    $('error').textContent = String(e);
    return false;
  }
}

function endTurn() {
  if (!sim || v.phase === 'choose') return;
  turns(1);
  refresh();
  autosave.write(sim, model);
}

function togglePlay() {
  if (playing) stop();
  else {
    running = false;
    setPlaying(true);
  }
}

function setPlaying(on) {
  playing = on;
  bar.transport({ playing, running });
  if (on) {
    last = performance.now();
    acc = 0;
    if (!loopTimer) loopTimer = setTimeout(loop, 0);
  }
}

/** Ends a run (Play or Skip): the autosave is written once, as it stops. */
function stop() {
  const was = playing || running;
  playing = false;
  running = false;
  bar.transport({ playing, running });
  if (was && sim) autosave.write(sim, model);
}

/** The play loop: as many turns as the speed asks for, one refresh per pass; stops at the pick. */
function loop() {
  loopTimer = null;
  if (!playing && !running) return;
  const mine = sim;
  const now = performance.now();
  const speed = $('speed').value;
  let ok = true;
  if (running || speed === 'max') {
    // As many turns as fit in a frame's worth of time.
    const until = now + 18;
    do ok = turns(1); while (ok && performance.now() < until && sim.phase() !== 'choose');
  } else {
    acc = Math.min(acc + ((now - last) / 1000) * RATES[speed], 4);
    last = now;
    const n = Math.floor(acc);
    acc -= n;
    if (n > 0) ok = turns(n);
  }
  if (sim !== mine) return;
  refresh();
  autosave.tick(sim, model);
  if (!ok || v.phase === 'choose') {
    stop();
    return;
  }
  loopTimer = setTimeout(loop, running || speed === 'max' ? 0 : 16);
}

/** Skip to the pick: the pre-game at full speed; pressed again, it stops. */
function runToEnd() {
  if (running) {
    stop();
    return;
  }
  if (v.phase !== 'pregame') return;
  stop();
  running = true;
  bar.transport({ playing, running });
  if (!loopTimer) loopTimer = setTimeout(loop, 0);
}

/** Standing so far: the end screen over the game as it has run. */
function standing() {
  app.show('end', { over: null, history: model.history, civs: v.civs, player: v.player });
}

function saveFile() {
  const a = document.createElement('a');
  const text = sim.save();
  a.href = URL.createObjectURL(new Blob([text], { type: 'application/json' }));
  a.download = `sansfiat-seed${JSON.parse(text).config.seed}-turn${v.turn}.json`;
  a.click();
  URL.revokeObjectURL(a.href);
}

async function loadFile(e) {
  const file = e.target.files[0];
  e.target.value = '';
  if (!file) return;
  let loaded;
  try {
    loaded = app.data.Sim.load(await file.text());
  } catch (err) {
    $('error').textContent = String(err);
    return;
  }
  begin(loaded);
}

/** The Hidden country toggle: the closed-country outlines and their legend rows. */
function setCountry(on) {
  country = on;
  view.setLayer('country', on);
  $('legenditems').innerHTML = legendHtml(on);
}

function showFeed(on) {
  $('side').classList.toggle('collapsed', !on);
  bar.feedOpen(on);
  $('showfeed').hidden = on;
  feed.setOpen(on);
  // The map box changes size; the observer resizes the canvas.
}

// ---- selecting and the selection panel -------------------------------------------------------

function select(sel) {
  const unit = sel?.kind === 'unit' ? sel.id : null;
  // Picking another thing forgets a route being set; the same unit keeps its start.
  if (unit !== routing.unit) routing = { arm: null, start: null, unit };
  else routing.arm = null;
  if (painting && keyOf(sel) !== keyOf(selected)) {
    painting = null;
    view.setPainting(false);
    document.body.dataset.painting = '';
  }
  selected = sel;
  feed.select(null);
  syncSelection();
  markChips();
  panel();
}

/** Puts the selection on the map and in `body.dataset.selected`. */
function syncSelection() {
  const hl = selected && selected.kind === 'tile' && (selected.clan != null || selected.market != null);
  view.setSelection(selected ? tileOf(ctx(), selected) : -1, selected?.kind === 'unit' ? selected.id : -1);
  if (v) routes();
  view.setHighlight(hl ? { clan: selected.clan, market: selected.market } : null);
  document.body.dataset.selected = keyOf(selected);
}

/** A left-click: the first thing on the tile, or the next one if something there is already selected. */
function onPick(i) {
  if (routing.arm) {
    armedPick(i);
    return;
  }
  if (i < 0) {
    select(null);
    return;
  }
  const things = candidates(ctx(), i);
  let next = things[0];
  if (selected && tileOf(ctx(), selected) === i) {
    const at = things.findIndex((t) => keyOf(t) === keyOf(selected));
    next = things[(at + 1) % things.length];
  }
  select(next);
}

/** The map click after a route button: the start, or the end, which sends `SetRoute` from the start (or the scout's tile). */
function armedPick(i) {
  const kind = routing.arm;
  routing.arm = null;
  if (i < 0) {
    panel();
    syncArm();
    return;
  }
  if (kind === 'start') {
    routing.start = i;
    panel();
    syncArm();
    return;
  }
  const u = v.units[routing.unit];
  if (!u) return;
  order(unitOrder(u, { type: 'route', from: routing.start ?? u.tile, to: i }));
}

/** Arms (or disarms, if it is armed already) a map click for the route: 'start' or 'to'. */
function arm(kind) {
  routing.arm = routing.arm === kind ? null : kind;
  syncArm();
  panel();
}

function syncArm() {
  document.body.dataset.armed = routing.arm ?? '';
}

/** A right-click: the selected camp of the player's goes to the tile (a scout's sends nothing). */
function onCommand(i) {
  const u = selected && selected.kind === 'unit' ? v.units[selected.id] : null;
  if (i < 0 || !u || u.civ !== v.player || u.kind !== 'camp') return;
  order(unitOrder(u, { type: 'move', to: i }));
}

function onFeed(item) {
  const alive = item.unit != null && item.unit in v.units;
  if (alive) selected = { kind: 'unit', id: item.unit };
  else if (item.kind === 'civFounded' && item.civ != null && item.civ in v.civs) selected = { kind: 'capital', civ: item.civ };
  else selected = { kind: 'tile', tile: item.tile, clan: item.clan, market: item.market };
  syncSelection();
  markChips();
  view.centreOn(alive ? v.units[item.unit].tile : item.tile, { minK: 24, smooth: true });
  panel();
}

/** The selection panel: what is selected and its actions as buttons, greyed with the reason when refused. */
function panel() {
  const el = $('detail');
  const d = selected ? describe(ctx(), selected) : null;
  if (!d) {
    el.innerHTML = '';
    return;
  }
  el.innerHTML = `<h2><span>${d.title}</span><button type="button" id="closedetail" aria-label="Close details">×</button></h2>${d.html}`;
  if (d.actions.length) {
    const box = document.createElement('div');
    box.className = 'actions';
    // Actions with the same `row` share a line; others each have their own.
    let row = null;
    let rowKey;
    for (const a of d.actions) {
      const why = a.arm ? '' : sim.check(JSON.stringify(a.action));
      if (!row || a.row === undefined || a.row !== rowKey) {
        row = document.createElement('div');
        rowKey = a.row;
        box.append(row);
      }
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = a.label;
      b.disabled = why !== '';
      if (a.arm) {
        b.setAttribute('aria-pressed', String(routing.arm === a.arm));
        b.onclick = () => arm(a.arm);
      } else {
        b.onclick = () => order(a.action);
      }
      row.append(b);
      if (why) {
        const note = document.createElement('span');
        note.className = 'dim';
        note.textContent = ` ${why}`;
        row.append(note);
      }
    }
    const slot = el.querySelector('[data-actions]');
    if (slot) slot.replaceWith(box);
    else el.append(box);
  }
  $('closedetail').onclick = () => select(null);
}

// ---- painting a zone and a claim ---------------------------------------------------------------

function setPainting(mode) {
  const u = selected?.kind === 'unit' ? v?.units[selected.id] : null;
  const capital = selected?.kind === 'capital' && selected.civ === v.player;
  if (mode === 'paint' || mode === 'unpaint') painting = u && u.kind === 'camp' && u.civ === v.player ? mode : null;
  else if (mode === 'claim' || mode === 'unclaim') painting = capital ? mode : null;
  else painting = null;
  view.setPainting(painting != null);
  document.body.dataset.painting = painting ?? '';
  panel();
}

/** The tiles of `zone` (a Set holding `home`) still connected to `home` through each other. */
function pruneZone(zone, home) {
  const keep = new Set([home]);
  const todo = [home];
  while (todo.length) {
    const i = todo.pop();
    for (let d = 0; d < 6; d++) {
      const j = view.nb[i * 6 + d];
      if (j >= 0 && zone.has(j) && !keep.has(j)) {
        keep.add(j);
        todo.push(j);
      }
    }
  }
  return keep;
}

/** A painted tile: added to the selected camp's zone, or in unpaint mode taken out with whatever is
 *  then no longer connected to the camp (paint-army's prune). The zone goes to the engine at once;
 *  a refusal says why under the panel. */
function onPaint(i) {
  if (painting === 'claim' || painting === 'unclaim') {
    onPaintClaim(i);
    return;
  }
  const u = selected?.kind === 'unit' ? v.units[selected.id] : null;
  if (!painting || !u || u.civ !== v.player || i === u.tile) return;
  let zone = new Set(u.territory);
  if (painting === 'paint') {
    if (zone.has(i)) return;
    zone.add(i);
  } else {
    if (!zone.has(i)) return;
    zone.delete(i);
    zone = pruneZone(zone, u.tile);
  }
  order(unitOrder(u, { type: 'zone', tiles: [...zone].filter((t) => t !== u.tile).sort((a, b) => a - b) }));
}

/** A painted tile of the claim: added (land only, the engine says why not), or in unclaim mode taken
 *  out with whatever is then no longer connected to the capital. The claim goes to the engine at once,
 *  as the zone does. */
function onPaintClaim(i) {
  const c = v.civs[v.player];
  if (!c || i === c.capital) return;
  let claim = new Set(c.claim);
  if (painting === 'claim') {
    if (claim.has(i) || !app.data.TERRAIN[layout.terrain[i]].land) return;
    claim.add(i);
  } else {
    if (!claim.has(i)) return;
    claim.delete(i);
    claim = pruneZone(claim, c.capital);
  }
  claim.add(c.capital);
  order({ type: 'order', civ: v.player, order: { type: 'claim', tiles: [...claim].filter((t) => t !== c.capital).sort((a, b) => a - b) } });
}

/** A camp's line home, coloured by how much of its labour the line takes to feed it: green where a
 *  turn's food is cheap to carry, amber halfway, red when it is cut off. The thresholds are the
 *  page's to tune. */
function lineColour(u) {
  const t = u.cut_off ? 1 : u.line.carry == null ? 0 : Math.min(1, (u.people * app.data.RULES.unit_food) / u.line.carry);
  // Green (120) at 0, amber (40) at a half, red (0) at 1.
  const hue = t < 0.5 ? 120 - (80 * t) / 0.5 : 40 - (40 * (t - 0.5)) / 0.5;
  return `hsl(${hue} 85% 55%)`;
}

// ---- keys ------------------------------------------------------------------------------------

/** The keys of the play screen. Escape steps back one thing: a menu, an armed click, painting, the selection. */
function keys(e) {
  if (document.body.dataset.screen !== 'play' || !sim) return;
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.key === 'Escape') {
    e.preventDefault();
    if (bar.closeMenus()) return;
    if (routing.arm) {
      routing.arm = null;
      syncArm();
      panel();
    } else if (painting) setPainting(null);
    else select(null);
    return;
  }
  if (e.target.matches('input, select, textarea')) return;
  if (e.target.matches('button, summary')) e.target.blur();
  const step = 60;
  switch (e.key) {
    case 'ArrowLeft': view.panBy(step, 0); break;
    case 'ArrowRight': view.panBy(-step, 0); break;
    case 'ArrowUp': view.panBy(0, step); break;
    case 'ArrowDown': view.panBy(0, -step); break;
    case '+': case '=': view.zoomBy(1.4); break;
    case '-': case '_': view.zoomBy(1 / 1.4); break;
    case '0': view.fit(); break;
    case ' ':
      if (v.phase === 'choose') return;
      if (playing || running) stop();
      else endTurn();
      break;
    case 'h': case 'H': $('v-country').click(); break;
    case 'p': case 'P': setPainting(painting === 'paint' ? null : 'paint'); break;
    case 'u': case 'U': setPainting(painting === 'unpaint' ? null : 'unpaint'); break;
    default: return;
  }
  e.preventDefault();
}

// ---- the tooltip -----------------------------------------------------------------------------

let tipTimer = null;
let tipTile = -2;
let tipAt = [0, 0];

function hideTip() {
  clearTimeout(tipTimer);
  tipTimer = null;
  tipTile = -2;
  $('tip').hidden = true;
}

function showTip(i, x, y) {
  tipAt = [x, y];
  if (i < 0 || !v) {
    hideTip();
    return;
  }
  const tip = $('tip');
  if (!tip.hidden) {
    // Already up: follow the cursor and keep the text current.
    if (i !== tipTile) fillTip(i);
    placeTip();
    return;
  }
  if (i !== tipTile) {
    clearTimeout(tipTimer);
    tipTile = i;
    tipTimer = setTimeout(() => {
      fillTip(i);
      tip.hidden = false;
      placeTip();
    }, 350);
  }
}

/** What a held tile says of its clan: a home names the festival its clan walks to. */
function clanTip(i, id) {
  const c = v.clans[id];
  if (c.home !== i) return `Clan ${id}'s territory${clanRole(c, i).includes('route') ? ', on its festival route' : ''}`;
  if (c.market == null) return `Clan ${id}'s home, no festival in reach`;
  const here = c.routes.length === 0 || c.routes[0].path.length === 0;
  return `Clan ${id}'s home, ${here ? `its festival ${c.market} is here` : `walks to festival ${c.market}`}`;
}

/** A worn tile's foods as shares, each with the cap it would have unworn. */
function wearTip(i) {
  const w = v.wear.find(([t]) => t === i);
  if (!w) return [];
  const [, worn, unworn] = w;
  return app.data.GOODS.flatMap((g, f) => {
    const pct = Math.round(100 * worn[f]);
    return pct > 0 ? [`${g} worn ${pct}%, unworn cap ${f2(unworn[f])}`] : [];
  });
}

const isHearth = (i) => {
  const tame = v.tame.find(([t]) => t === i);
  return layout.wild_grain[i] > 0 && tame && tame[1] >= app.data.RULES.tame_turns;
};

function fillTip(i) {
  const { TERRAIN } = app.data;
  tipTile = i;
  const h = view.clanAt(i);
  const lines = [`<b>${tileKind(ctx(), i)}</b>`];
  if (layout.terrain[i] !== 0 && TERRAIN[layout.terrain[i]].land) {
    lines.push(h ? clanTip(i, h) : 'Held by nobody');
    for (const u of model.holders(i)) lines.push(`Held by ${v.civs[u.civ]?.name ?? '?'}'s camp`);
    lines.push(`Fertility ${f2(layout.fertility[i])}`);
    lines.push(...wearTip(i));
  }
  const civ = sim.tile_civ(i);
  if (civ >= 0 && v.civs[civ]) {
    const c = v.civs[civ];
    const name = `<i class="sw" style="background:${c.color}"></i>${c.name}`;
    lines.push(c.capital === i ? `${name}'s capital` : `${name}'s land`);
  }
  for (const c of Object.values(v.civs)) {
    if (c.claim.includes(i) && !c.land.includes(i)) lines.push(`paper of <i class="sw" style="background:${c.color}"></i>${c.name}`);
  }
  const here = Object.values(v.units).filter((u) => u.tile === i);
  if (here.length) {
    const names = here.map((u) => `${v.civs[u.civ]?.name ?? '?'} ${u.kind} ${u.id}${u.hungry ? ' (hungry)' : ''}`);
    lines.push(names.slice(0, 4).join('<br>') + (names.length > 4 ? `<br>and ${names.length - 4} more` : ''));
  }
  const m = marketOn(ctx(), i);
  if (m) lines.push(`Market ${m.id}${v.end && v.end.centres.includes(m.id) ? ' (centre)' : ''}`);
  if (isHearth(i)) lines.push('Hearth');
  const u = selected && selected.kind === 'unit' ? v.units[selected.id] : null;
  if (u && u.civ === v.player) {
    const r = JSON.parse(sim.path(u.tile, i));
    lines.push(r ? `Move here: cost ${f2(r.cost)}, ${r.turns} turn${r.turns === 1 ? '' : 's'}` : 'No route');
  }
  $('tip').innerHTML = lines.join('<br>');
}

function placeTip() {
  const tip = $('tip');
  const [x, y] = tipAt;
  const w = tip.offsetWidth, h = tip.offsetHeight;
  let left = x + 16, top = y + 16;
  if (left + w > window.innerWidth - 4) left = x - w - 12;
  if (top + h > window.innerHeight - 4) top = y - h - 12;
  tip.style.left = `${Math.max(4, left)}px`;
  tip.style.top = `${Math.max(4, top)}px`;
}

/** The ctx the selection panel and the tooltips read the world through. */
function ctx() {
  const { TERRAIN, GOODS, RULES } = app.data;
  return { v, model, layout, terrain: TERRAIN, goods: GOODS, rules: RULES, sim, view, routing, outcome, painting, country };
}
