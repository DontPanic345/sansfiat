// The top bar. Before the pick: the pre-game's counts and Skip to the pick. After it: the turn, a chip
// per civ with its Standing (yours marked, the leader ▲), End turn, Play and speed, the Game, View and
// Debug menus, and the Feed toggle. Every number carries a tooltip (its `title`). Its menus toggle map
// layers through the handlers the play screen gives in `init`.

import { app } from './app.js';
import { autosave } from './save.js';

const $ = (id) => document.getElementById(id);
const esc = (t) => String(t).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
const pct = (x) => `${(Math.round(x * 1000) / 10).toFixed(1)}%`;
const ORD = (n) => n + (n % 100 >= 11 && n % 100 <= 13 ? 'th' : ['th', 'st', 'nd', 'rd'][n % 10] ?? 'th');
/** The Game menu's line and the Continue line use the same wording. */
const LAYER_OF = { 'v-territory': 'territories', 'v-routes': 'festival-routes', 'v-fertility': 'fertility', 'v-hidden': 'hidden-costs', 'v-wear': 'worn' };

let H = {};
let titles = {};

/**
 * @param handlers {end, play, skip, speed, selectCiv(civ), newGame, saveFile, standing, layer(name, on), country(on)}
 * @param map the MapView, whose `on` says which layers start on
 */
export function init(handlers, map) {
  H = handlers;
  for (const id of ['end', 'play', 'run']) titles[id] = $(id).title;
  $('end').onclick = () => H.end();
  $('play').onclick = () => H.play();
  $('run').onclick = () => H.skip();
  $('speed').onchange = () => H.speed();
  $('chips').addEventListener('click', (e) => {
    const b = e.target.closest('button[data-civ]');
    if (b) H.selectCiv(Number(b.dataset.civ));
  });
  $('more').onclick = () => {
    const open = !$('bar').classList.contains('more-open');
    $('bar').classList.toggle('more-open', open);
    $('more').setAttribute('aria-expanded', String(open));
  };
  $('menu-new').onclick = () => H.newGame();
  $('save').onclick = () => H.saveFile();
  $('loadButton').onclick = () => $('load').click();
  $('menu-standing').onclick = () => H.standing();
  for (const [id, layer] of Object.entries(LAYER_OF)) {
    const box = $(id);
    box.checked = map.on[layer];
    box.onchange = () => H.layer(layer, box.checked);
  }
  $('v-country').checked = map.on.country;
  $('v-country').onchange = (e) => H.country(e.target.checked);
  $('v-legend').onchange = (e) => ($('legend').open = e.target.checked);
  // The legend opens and closes on its own (the pick closes it): the box follows it.
  $('legend').addEventListener('toggle', () => ($('v-legend').checked = $('legend').open));
  autosave.onChange = line;
  document.addEventListener('pointerdown', (e) => {
    for (const m of document.querySelectorAll('details.menu[open]')) if (!m.contains(e.target)) m.open = false;
    if (!e.target.closest('.more-items, #more') && $('bar').classList.contains('more-open')) {
      $('bar').classList.remove('more-open');
      $('more').setAttribute('aria-expanded', 'false');
    }
  });
  line();
}

/** Escape's first step: closes the open menus and the phone's ☰ menu. True if one was open. */
export function closeMenus() {
  let any = false;
  for (const m of document.querySelectorAll('details.menu[open]')) {
    m.open = false;
    any = true;
  }
  if ($('bar').classList.contains('more-open')) {
    $('bar').classList.remove('more-open');
    $('more').setAttribute('aria-expanded', 'false');
    any = true;
  }
  return any;
}

/** Play and Skip's labels: Pause while playing, Stop while skipping. */
export function transport({ playing, running }) {
  $('play').textContent = playing ? 'Pause' : 'Play';
  $('play').setAttribute('aria-pressed', String(playing));
  $('run').textContent = running ? 'Stop' : 'Skip to the pick';
}

/** The Feed toggle's count of unread alerts, and whether the feed is open. */
export function feedCount(n) {
  $('feedtoggle').textContent = n ? `Feed (${n})` : 'Feed';
  document.body.dataset.unread = n;
}

export function feedOpen(on) {
  $('feedtoggle').setAttribute('aria-pressed', String(on));
}

/** The bar as the world stands: `v` (Sim.view), `model` (model.js), `selectedCiv` the civ whose capital is selected. */
export function update({ v, model, selectedCiv }) {
  const { RULES } = app.data;
  const pre = v.phase !== 'main';
  const sum = v.summary;
  const centres = v.end ? v.end.centres.length : sum.centres.length;
  const over = `turn ${v.end?.turn}, by ${v.end?.by_limit ? 'the limit' : 'centres'}${v.end?.topped_up.length ? `, +${v.end.topped_up.length} topped up` : ''}`;
  $('turn').textContent = v.turn;
  $('people').textContent = Math.round(sum.people);
  $('clans').textContent = sum.clans;
  $('festivals').textContent = sum.markets;
  $('hearths').textContent = sum.hearths;
  $('centres').textContent = `${centres} / ${RULES.players}`;
  $('turnstat').title = `Turn ${v.turn}: ${Math.round(sum.people)} people, ${sum.clans} clans, ${sum.markets} festivals, ${sum.hearths} hearths, ${centres} of ${RULES.players} centres`;
  if (v.phase === 'pregame') {
    $('phasetext').textContent = 'Pre-game';
    $('phasenote').textContent = 'Phase';
  } else if (v.phase === 'choose') {
    $('phasetext').textContent = 'Pre-game over';
    $('phasenote').textContent = over;
  } else {
    $('phasetext').textContent = 'Main game';
    $('phasenote').textContent = `pre-game ${over}`;
  }
  $('phase').title = pre ? 'The pre-game runs to the pick, when each civ is chosen' : 'The main game: the pre-game is over';
  $('counts').hidden = !pre;
  $('chips').hidden = pre;
  if (!pre) chips(v, model, selectedCiv);
  const choose = v.phase === 'choose';
  for (const id of ['end', 'play', 'run']) {
    $(id).disabled = choose;
    $(id).title = choose ? 'Choose a civ first' : titles[id];
  }
  $('run').hidden = v.phase !== 'pregame';
  line();
}

/** A chip per civ: the swatch, the name and its Standing; yours marked "you", the leader ▲. */
function chips(v, model, selectedCiv) {
  const civs = Object.values(v.civs);
  $('chips').innerHTML = civs
    .map((c) => {
      const s = model.standing.get(c.id);
      const mine = c.id === v.player;
      const tip = `Standing ${pct(s.total)}: ${c.name}'s clans and units ${pct(s.clans)}, unaligned clans on its land ${pct(s.land)}; ${ORD(s.rank)} of ${civs.length}`;
      return `<button type="button" class="chip${mine ? ' mine' : ''}" data-civ="${c.id}" aria-pressed="${selectedCiv === c.id}" title="${esc(tip)}"><i class="sw" style="background:${esc(c.color)}"></i><b>${esc(c.name)}</b> <span>${pct(s.total)}</span>${mine ? '<em>you</em>' : ''}${model.leader === c.id ? '<i class="lead" title="Leads the Standing">▲</i>' : ''}</button>`;
    })
    .join('');
}

/** The Game menu's autosave line. */
function line() {
  $('autosave-line').textContent = autosave.line();
}
