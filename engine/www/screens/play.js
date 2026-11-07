// The play screen: the map with its bar, the bottom panel, the feed and the pick over the map, and the
// turn loop. It holds the game in progress: the Sim, its view model, the selection, and the keys. The
// screens around it (start, end) hand it a Sim and read back what it holds through `state()`.

import { MapView } from '../map/view.js';
import { build } from '../model.js';
import { Feed } from '../feed.js';
import { f2, keyOf, tileOf, alive, candidates, describe, subjectOf, tileKind, marketOn, clanRole, unitOrder } from '../panels.js';
import * as panel from '../panel.js';
import { app } from '../app.js';
import { autosave } from '../save.js';
import { legendHtml } from '../legend.js';
import * as bar from '../bar.js';
import { Pick } from './pick.js';
import { routing, resetRouting, routeDone, arm, disarm, clearStart, armedPick, onCommand, orderTip, markStart } from './orders.js';

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
let selected = null; // {kind: 'unit', id} | {kind: 'civ', civ} | {kind: 'clan', id} | {kind: 'festival', id} | {kind: 'tile', tile}
let ringShares = null; // the ring of the selected army's or civ's civ, tile → share, or null
let outcome = null; // how the player's last scout route ended: 'found' | 'nopath' | null
let playing = false;
let running = false;
let endSeen = false; // whether this game's end has been shown, or was there when it loaded (`checkEnd`)
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
    onCommand: (i) => onCommand(i, host()),
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
        select({ kind: 'civ', civ });
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
  showFeed(matchMedia('(min-width: 1400px)').matches);
  window.addEventListener('keydown', keys);
  // A button blurs after a click, so a space never presses it again.
  document.addEventListener('click', (e) => e.target.closest?.('button, summary')?.blur());
}

/** Shows the game: `args.sim` starts it (`speed`, `run` to skip the pre-game, `play` to run it); no args
 *  returns to the game in progress. */
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
/** The shape the bottom panel draws for the selection now (panels.js's `describe`), or null. */
export const shape = () => (selected ? describe(ctx(), selected) : null);

/** Starts a game: `sim` replaces the one in progress. */
function begin(next, { speed = null, run = false, play: go = false } = {}) {
  stop();
  sim = next;
  endSeen = sim.phase() === 'over';
  $('save').disabled = sim.staged();
  layout = JSON.parse(sim.layout());
  model = null;
  selected = null;
  resetRouting();
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
  model = build(sim, layout, RULES);
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
  pick.update(sim, v.phase, v.turn);
  if (selected && !alive(ctx(), selected)) {
    selected = null;
    syncSelection();
  }
  pullEvents();
  const scout = v.player != null ? Object.values(v.units).find((u) => u.civ === v.player && u.kind === 'scout') : null;
  document.body.dataset.scout = scout ? scout.id : '';
  document.body.dataset.routeState = scout?.route && outcome !== 'found' ? 'set' : (outcome ?? 'none');
  document.body.dataset.armed = routing.arm ?? '';
  markChips();
  panelDraw();
  view.drawNow();
}

/** The bar, with the selected civ's chip marked. */
function markChips() {
  if (!v) return;
  bar.update({ v, model, selectedCiv: selected?.kind === 'civ' ? selected.civ : null });
}

/** Hands the map the player's camps' routes (the selected one drawn firmly, the rest faint), and the
 *  selected camp's line home. */
function routes() {
  const out = [];
  for (const u of Object.values(v.units)) {
    if (u.civ !== v.player || (u.kind !== 'camp' && u.kind !== 'scout') || u.dest == null) continue;
    const r = JSON.parse(sim.route(u.id));
    if (r) out.push({ unit: u.id, civ: u.civ, tiles: [u.tile, ...r.path], selected: selected?.kind === 'unit' && selected.id === u.id });
  }
  view.setRoutes(out);
  markStart(host());
  // The player's camp's line home, always faint; the selected clan's territory and route bright.
  const mineCamp = v.player != null ? Object.values(v.units).find((u) => u.civ === v.player && u.kind === 'camp') : null;
  view.setHomeLine(mineCamp?.line?.path.length ? [mineCamp.tile, ...mineCamp.line.path] : null);
  // The ring of the civ an army or a civ is selected for: drawn, and read by the tile tooltip.
  const ringCiv = selected?.kind === 'civ' ? selected.civ : selected?.kind === 'unit' && v.units[selected.id]?.kind === 'camp' ? v.units[selected.id].civ : null;
  ringShares = ringCiv == null ? null : new Map(JSON.parse(sim.ring(ringCiv)));
  view.setRing(ringShares?.size ? { colour: v.civs[ringCiv]?.color ?? '#ffffff', shares: ringShares } : null);
  view.setFocus(selected?.kind === 'clan' ? selected.id : selected?.kind === 'tile' ? view.clanAt(selected.tile) : null);
  // A selected festival: its walkers' routes are drawn bright.
  view.setFocusMarket(selected?.kind === 'festival' ? selected.id : null);
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
      routeDone();
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
    select({ kind: 'civ', civ });
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

/** Whether a run stops here: at the pick, or at the game's first end (the end screen comes up, `checkEnd`). */
function halt() {
  return sim.phase() === 'choose' || (sim.phase() === 'over' && !endSeen);
}

/** The first view in `over` stops the run and shows the end screen, once a game. Play on goes back to the
 *  map, where the world runs on. True when the end screen came up. */
function checkEnd() {
  if (!v.over || endSeen) return false;
  endSeen = true;
  stop();
  app.show('end', { view: v });
  return true;
}

/** Ends `n` turns, stopping at the pick or the game's end; shows why if refused. Returns whether it all went through. */
function turns(n) {
  try {
    for (let i = 0; i < n && !halt(); i++) sim.act('{"type":"endTurn"}');
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
  checkEnd();
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
    do ok = turns(1); while (ok && performance.now() < until && !halt());
  } else {
    acc = Math.min(acc + ((now - last) / 1000) * RATES[speed], 4);
    last = now;
    const n = Math.floor(acc);
    acc -= n;
    if (n > 0) ok = turns(n);
  }
  if (sim !== mine) return;
  refresh();
  if (checkEnd()) return;
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
  app.show('end', { view: v });
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

/** Selects `sel` (or nothing). Selecting a subject marks the alerts about it seen. `fromFeed` keeps the
 *  feed's own highlight of the item clicked. */
function select(sel, { fromFeed = false } = {}) {
  const unit = sel?.kind === 'unit' ? sel.id : null;
  // Picking another thing forgets a route being set; the same unit keeps its start.
  if (unit !== routing.unit) resetRouting(unit);
  else routing.arm = null;
  if (painting && keyOf(sel) !== keyOf(selected)) {
    painting = null;
    view.setPainting(false);
    document.body.dataset.painting = '';
  }
  selected = sel;
  if (!fromFeed) feed.select(null);
  if (sel) feed.see((item) => keyOf(subjectOf(ctx(), item)) === keyOf(sel));
  syncSelection();
  markChips();
  panelDraw();
}

/** Puts the selection on the map and in `body.dataset.selected`. */
function syncSelection() {
  view.setSelection(selected ? tileOf(ctx(), selected) : -1, selected?.kind === 'unit' ? selected.id : -1);
  if (v) routes();
  const hl = selected?.kind === 'clan' ? { clan: selected.id, market: null } : selected?.kind === 'festival' ? { clan: null, market: selected.id } : null;
  view.setHighlight(hl);
  document.body.dataset.selected = keyOf(selected);
}

/** A left-click: the first thing on the tile, or the next one if something there is already selected. */
function onPick(i) {
  if (routing.arm) {
    armedPick(i, host());
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

/** A feed item's subject is selected and the map centred on it. */
function onFeed(item) {
  const sel = subjectOf(ctx(), item);
  select(sel, { fromFeed: true });
  view.centreOn(tileOf(ctx(), sel), { minK: 24, smooth: true });
}

/** What the armed clicks and the right-click need of this screen (orders.js). */
const host = () => ({ view, v, order, panelDraw });

/** The bottom panel: what is selected, drawn by panel.js from the shape panels.js gives. */
function panelDraw() {
  panel.render($('detail'), selected ? describe(ctx(), selected) : null, {
    check: (action) => sim.check(action),
    run: order,
    arm: (kind) => arm(kind, host()),
    armed: routing.arm,
    paint: (mode) => setPainting(painting === mode ? null : mode),
    painting,
    select,
    centre: (i) => view.centreOn(i, { minK: 12, smooth: true }),
    redraw: panelDraw,
  });
}

// ---- painting a zone and a claim ---------------------------------------------------------------

/** Sets what a press and drag paint: 'paint' or 'unpaint' (a camp's zone, or a civ's claim when a civ is
 *  selected), 'claim' or 'unclaim' (a civ's claim), or null. */
function setPainting(mode) {
  const u = selected?.kind === 'unit' ? v?.units[selected.id] : null;
  const camp = u && u.kind === 'camp' && u.civ === v.player;
  const civ = selected?.kind === 'civ' && selected.civ === v.player;
  if (mode === 'paint' || mode === 'unpaint') painting = camp ? mode : civ ? (mode === 'paint' ? 'claim' : 'unclaim') : null;
  else if (mode === 'claim' || mode === 'unclaim') painting = civ ? mode : null;
  else painting = null;
  view.setPainting(painting != null);
  document.body.dataset.painting = painting ?? '';
  panelDraw();
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
    if (routing.arm) disarm(host());
    else if (painting) setPainting(null);
    else if (routing.start != null) clearStart(host());
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
    case 'p': case 'P': setPainting(painting === 'paint' || painting === 'claim' ? null : 'paint'); break;
    case 'u': case 'U': setPainting(painting === 'unpaint' || painting === 'unclaim' ? null : 'unpaint'); break;
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
    for (const u of model.holders(i)) lines.push(`Held by ${v.civs[u.civ]?.name ?? '?'}'s army`);
    if (ringShares?.size) {
      const share = ringShares.get(i);
      lines.push(share !== undefined && share < 1 ? `Ring: its line takes ${Math.round(100 * share)}% of an army's days here` : "Past the ring: its line can't feed an army here");
    }
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
  lines.push(...orderTip(sim, u, i, v));
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
