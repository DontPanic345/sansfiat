// The Tests screen: the stage tests core lists (`Sim.tests()`). A chosen test is built as a Sim with its run
// (`Sim.stage_test`), drawn on its own map, and played a turn at a time (`test_step`). The verdict is core's:
// this shows `test_state()` and never decides pass or fail itself.

import { MapView } from '../map/view.js';
import { build } from '../model.js';
import { app } from '../app.js';
import { holdItems, listRows, resultLine } from '../tests-view.js';

const $ = (id) => document.getElementById(id);
const PACE = 500; // ms a turn while played

let mapView = null;
let all = null; // Sim.tests(), read once
let sim = null; // the chosen test's Sim, with its run
let json = null; // the chosen test's file text: Again builds it afresh
let layout = null;
let model = null; // the view model of the test's Sim
let current = null; // test_state() as last read
let timer = null;
let pace = PACE;

const tests = () => (all ??= JSON.parse(app.data.Sim.tests()));

export function init() {
  mapView = new MapView($('tests-map'), {
    terrain: app.data.TERRAIN,
    rules: app.data.RULES,
    atlas: app.atlas,
    onPick: () => {},
    onHover: () => {},
    onHoverEnd: () => {},
  });
  $('tests-back').onclick = () => app.show('start');
  $('tests-up').onclick = showList;
  $('tests-play').onclick = togglePlay;
  $('tests-step').onclick = () => {
    stop();
    turn();
  };
  $('tests-again').onclick = () => start(json);
  // The map's box is hidden on the list: it resizes when the test opens, and only then.
  new ResizeObserver(() => {
    if (app.screen === 'tests' && sim) mapView.resize();
  }).observe($('tests-mapbox'));
  window.addEventListener('keydown', keys);
}

/** Opens the list, or the test `args.test` names (or gives, as an object), at `args.pace` ms a turn. */
export function show(args = {}) {
  pace = Number(args.pace) || PACE;
  if (args.test == null) showList();
  else choose(args.test);
}

export function hide() {
  stop();
}

/** The chosen test's state as `test_state()` gave it (`{turn, turns, done, passed, holds}`), or null. */
export const state = () => current;

/** The chosen test's view (`Sim.view()`, as the play screen reads it), or null on the list. */
export const v = () => model?.v ?? null;

function showList() {
  stop();
  sim = null;
  json = null;
  current = null;
  $('tests-open').hidden = true;
  $('tests-list').hidden = false;
  listRows($('tests-rows'), tests(), (t) => choose(t.name));
  paintButtons();
}

/** Opens a test by name (one of the list) or as an object (a copy of one, for scripts). */
function choose(which) {
  if (typeof which === 'string') {
    const t = tests().find((x) => x.name === which);
    if (!t) {
      showList();
      $('tests-error').textContent = `no test named ${which}`;
      return;
    }
    start(t.json);
  } else start(JSON.stringify(which));
}

/** Builds the test from its file text and shows it at the start. */
function start(text) {
  stop();
  $('tests-error').textContent = '';
  let next;
  try {
    next = app.data.Sim.stage_test(text);
  } catch (err) {
    $('tests-error').textContent = String(err);
    return;
  }
  sim = next;
  json = text;
  const t = JSON.parse(text);
  const listed = tests().find((x) => x.name === t.name);
  $('tests-list').hidden = true;
  $('tests-open').hidden = false;
  $('tests-name').textContent = t.name;
  $('tests-claim').textContent = t.claim;
  const kept = listed?.kept ?? [];
  $('tests-kept').textContent = kept.join('; ');
  $('tests-kept').hidden = kept.length === 0;
  layout = JSON.parse(sim.layout());
  mapView.setLayout(layout);
  paint(JSON.parse(sim.test_state()));
  mapView.fit();
  mapView.drawNow();
}

/** Plays one turn through core. A refused order stops the play and says why. */
function turn() {
  let s;
  try {
    s = JSON.parse(sim.test_step());
  } catch (err) {
    stop();
    $('tests-error').textContent = String(err);
    return;
  }
  if (s.done) stop();
  paint(s);
}

/** Draws the state `s` (core's): the map, the turn, the result and the holds. */
function paint(s) {
  current = s;
  model = build(sim, layout, app.data.RULES);
  mapView.setModel(model);
  mapView.drawNow();
  $('tests-turn').textContent = `${s.turn} of ${s.turns}`;
  $('tests-result').textContent = resultLine(s);
  holdItems($('tests-holds'), s);
  document.body.dataset.testTurn = s.turn;
  document.body.dataset.testDone = s.done;
  paintButtons();
}

function paintButtons() {
  const running = timer != null;
  const done = current?.done ?? false;
  const open = sim != null;
  $('tests-play').textContent = running ? 'Pause' : 'Play';
  $('tests-play').setAttribute('aria-pressed', String(running));
  $('tests-play').disabled = !open || done;
  $('tests-step').disabled = !open || done || running;
  $('tests-again').disabled = !open;
}

function togglePlay() {
  if (timer != null) stop();
  else if (sim && !current.done) {
    timer = setInterval(turn, pace);
    paintButtons();
  }
}

function stop() {
  if (timer != null) clearInterval(timer);
  timer = null;
  paintButtons();
}

function keys(e) {
  if (app.screen !== 'tests') return;
  if (e.key === ' ' && sim) {
    e.preventDefault();
    togglePlay();
  } else if (e.key === 'Escape' && sim) {
    showList();
  }
}
