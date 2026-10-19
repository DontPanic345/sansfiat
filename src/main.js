import { CLAN, FOOD, FOODS, GOODS, MARKET } from './data.js';
import { distance, fromPixel, key } from './hex.js';
import { newGame, replay, runPregameCounted } from './game.js';
import { civCities, civClans, clanAt, holder, humanCiv, marketAt, notesFor, tileAt, total, totalPop } from './core.js';
import { hearths, marketReach, roamCost } from './clans.js';
import { draw, fogOf } from './render.js';
import * as rules from './rules.js';
import { MODULE_UIS } from './modules/ui.js';
import { VIEWS, VIEW_KEYS, rampCss } from './views.js';

const params = new URLSearchParams(location.search);
const config = {
  seed: Number(params.get('seed') ?? Math.floor(Math.random() * 1e9)),
  players: Number(params.get('players') ?? 3),
};

// Fast-forward in the main game with a human civ runs this many turns, then stops.
const FAST_TURNS = 10;
const ALERTS_KEPT = 10;

let state = newGame(config);
const history = [];
const log = [];
const alerts = [];
let mineOnly = false;
let sel = null; // {q, r}: the tile last clicked (contract 1)
let depletion = true;
let mode = 'terrain';
let range = null;
let timer = null;
let winTurn = null;
// UI-only state per module (selected unit, paint brush, ...). Never part of game state.
const locals = Object.fromEntries(MODULE_UIS.map((u) => [u.name, {}]));
const uiFor = (name) => ({ act: (a) => { const ok = act(a); update(); return ok; }, local: locals[name], selection: sel });

const canvas = document.getElementById('map');
const ctx = canvas.getContext('2d');
let view = {};

const isAlert = (line) => /^(Turn \d+: )?!/.test(line);
const gameOver = () => state.winner != null;

function addLog(lines) {
  log.unshift(...lines.slice().reverse());
  log.length = Math.min(log.length, 300);
  for (const l of lines) if (isAlert(l)) alerts.unshift(l);
  alerts.length = Math.min(alerts.length, ALERTS_KEPT);
}

function act(action) {
  // Own civ only (contract 2): with a human civ, the UI never commands another civ.
  const me = humanCiv(state);
  if (me && action.civ != null && action.civ !== me.id && !['chooseCiv', 'endTurn'].includes(action.type)) {
    addLog([`✗ ${action.type}: not your civ`]);
    return false;
  }
  if (action.type === 'endTurn' && gameOver()) return false;
  const { state: next, error } = rules.dispatch(state, action);
  if (error) {
    addLog([`✗ ${action.type}: ${error}`]);
    return false;
  }
  state = next;
  history.push(action);
  addLog(state.events);
  if (gameOver() && winTurn == null) winTurn = state.mods.civ?.winTurn ?? state.turn - 1;
  return true;
}

function step(n = 1) {
  for (let i = 0; i < n; i++) {
    const before = state.phase;
    if (gameOver() || !act({ type: 'endTurn' }) || state.phase !== before || gameOver()) { stop(); break; }
  }
  update();
}

// Runs the rest of the pre-game at once (headless fast path) and records the turns for replay.
function skipPregame() {
  stop();
  if (state.phase !== 'pregame') return;
  const { state: next, turns } = runPregameCounted(state);
  for (let i = 0; i < turns; i++) history.push({ type: 'endTurn' });
  state = next;
  addLog(state.events);
  update();
}

const watching = () => state.phase === 'main' && !humanCiv(state);

// mode: 'play' (one turn per tick) or 'fast' (pre-game: to founding; main game: FAST_TURNS turns,
// or until someone wins when watching all-AI).
function run(kind) {
  stop();
  if (!canStep()) { update(); return; }
  if (kind === 'fast') {
    let left = state.phase === 'main' && !watching() ? FAST_TURNS : Infinity;
    const per = state.phase === 'pregame' ? 5 : 1;
    timer = setInterval(() => {
      const n = Math.min(per, left);
      left -= n;
      step(n);
      if (left <= 0) { stop(); update(); }
    }, 16);
  } else timer = setInterval(() => step(1), 150);
  update();
}

function stop() {
  clearInterval(timer);
  timer = null;
}

const canStep = () => state.phase !== 'choose' && !gameOver();

// Contract 1: the clicked tile becomes the selection first, then modules see the click in z order.
function select(h) {
  if (!h || !tileAt(state, h)) { sel = null; update(); return; }
  sel = { q: h.q, r: h.r };
  for (const ui of MODULE_UIS) if (ui.onClick?.(state, sel, uiFor(ui.name))) break;
  update();
}

// Contract 3: Escape clears the tile and every module's scratch (same objects, emptied in place).
function escape() {
  sel = null;
  for (const l of Object.values(locals)) for (const k of Object.keys(l)) delete l[k];
  update();
}

document.getElementById('step').onclick = () => { stop(); step(); };
document.getElementById('play').onclick = () => (timer ? (stop(), update()) : run('play'));
document.getElementById('fast').onclick = () => run('fast');
document.getElementById('skip').onclick = () => skipPregame();
// Any element with data-action='{json}' (module panels, the founding choice) dispatches it.
document.querySelector('aside').addEventListener('click', (e) => {
  const el = e.target.closest('[data-action]');
  if (!el || el.disabled) return;
  e.preventDefault();
  act(JSON.parse(el.dataset.action));
  update();
});
const loadGame = (seed) => {
  params.set('seed', seed);
  params.set('players', document.getElementById('players').value);
  location.search = params;
};
document.getElementById('newMap').onclick = (e) => { e.preventDefault(); loadGame(Math.floor(Math.random() * 1e9)); };
document.getElementById('seedForm').onsubmit = (e) => {
  e.preventDefault();
  const v = document.getElementById('seed').value;
  loadGame(v !== '' ? Math.floor(Number(v)) : config.seed);
};
document.getElementById('seed').value = config.seed;
document.getElementById('players').value = String(config.players);
document.getElementById('mineOnly').onchange = (e) => { mineOnly = e.target.checked; renderPanel(); };
const modeSelect = document.getElementById('mode');
modeSelect.innerHTML = Object.entries(VIEWS).map(([k, v], i) => `<option value="${k}">${VIEW_KEYS[i]}. ${v.label}</option>`).join('');
const setMode = (m) => { mode = m; modeSelect.value = m; update(); };
modeSelect.onchange = () => setMode(modeSelect.value);
document.getElementById('depletion').onchange = (e) => { depletion = e.target.checked; update(); };
canvas.addEventListener('click', (ev) => {
  const rect = canvas.getBoundingClientRect();
  select(fromPixel(ev.clientX - rect.left - view.ox, ev.clientY - rect.top - view.oy, view.size));
});
addEventListener('keydown', (e) => {
  if (['INPUT', 'SELECT', 'TEXTAREA'].includes(e.target.tagName)) return;
  if (e.key === ' ') { e.preventDefault(); stop(); if (canStep()) step(); }
  else if (e.key === 'p') document.getElementById('play').click();
  else if (e.key === 'g') run('fast');
  else if (e.key === 'k') skipPregame();
  else if (e.key === 'd') { depletion = !depletion; document.getElementById('depletion').checked = depletion; update(); }
  else if (e.key === 'Escape') escape();
  else if (VIEW_KEYS.includes(e.key) && Object.keys(VIEWS)[VIEW_KEYS.indexOf(e.key)]) setMode(Object.keys(VIEWS)[VIEW_KEYS.indexOf(e.key)]);
});
addEventListener('resize', update);

function layout() {
  const dpr = devicePixelRatio || 1;
  const w = canvas.clientWidth, h = canvas.clientHeight;
  canvas.width = w * dpr;
  canvas.height = h * dpr;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  const { width, height } = state.map;
  const size = Math.min(w / (Math.sqrt(3) * (width + 0.5)), h / (1.5 * (height - 1) + 2)) * 0.97;
  const mapW = Math.sqrt(3) * size * (width + 0.5), mapH = size * (1.5 * (height - 1) + 2);
  view = { size, ox: (w - mapW) / 2 + (Math.sqrt(3) / 2) * size, oy: (h - mapH) / 2 + size };
}

function update() {
  layout();
  range = draw(ctx, state, { ...view, selected: sel, depletion, mode, viewer: humanCiv(state)?.id ?? null, locals });
  renderPanel();
}

const fmt = (v) => (v >= 10 ? v.toFixed(0) : v.toFixed(1));
const swatch = (color) => `<span class="swatch" style="background:${color}"></span>`;
const plural = (n, one, many) => `${n} ${n === 1 ? one : many}`;
const esc = (t) => String(t).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

const goodColor = (g) => FOOD[g]?.color ?? '#9aa4ae';
function basketRow(b) {
  const parts = GOODS.filter((f) => (b[f] ?? 0) > 0.05).map((f) => `${swatch(goodColor(f))}${fmt(b[f])}`);
  return parts.length ? parts.join(' ') : '—';
}

// The "why" log of an entity, newest first.
function whyHtml(id) {
  const notes = notesFor(state, id);
  if (!notes.length) return '';
  return `<div class="sub">Why</div><ul class="why">${notes.slice().reverse()
    .map((n) => `<li><span>t${n.turn}</span> ${n.text}</li>`).join('')}</ul>`;
}

const civName = (id) => (state.civs[id] ? `${swatch(state.civs[id].color)}${state.civs[id].name}` : '');

function sparkline(series, { width = 236, height = 40, color = '#dde3ea', min, max } = {}) {
  if (series.length < 2) return '';
  const lo = min ?? Math.min(...series), hi = max ?? Math.max(...series);
  const pts = series.map((v, i) => `${(i / (series.length - 1)) * width},${height - ((v - lo) / (hi - lo || 1)) * height}`);
  return `<polyline fill="none" stroke="${color}" stroke-width="1.5" points="${pts.join(' ')}"/>`;
}

function marketHtml(m) {
  const owner = m.owner != null ? civName(m.owner) : m.temporary ? 'Gathering' : 'Neutral market';
  const goods = GOODS.filter((f) => m.prices[f]);
  const hi = Math.max(...goods.map((f) => m.prices[f]), 1);
  const bars = goods.map((f) =>
    `<div class="bar"><span>${f}</span><i style="width:${(m.prices[f] / hi) * 100}%;background:${goodColor(f)}"></i><b>${m.prices[f].toFixed(2)}</b></div>`).join('');
  const hist = m.history.filter((h) => goods.every((f) => h.prices[f]));
  const all = hist.flatMap((h) => goods.map((f) => Math.log(h.prices[f])));
  const lines = goods.map((f) => sparkline(hist.map((h) => Math.log(h.prices[f])),
    { color: goodColor(f), min: Math.min(...all), max: Math.max(...all), height: 60 })).join('');
  const vol = m.lastTrade ?? {};
  const food = FOODS.reduce((a, f) => a + (vol[f] ?? 0), 0);
  const other = total(vol) - food;
  return `<b>${owner}</b> · since turn ${m.founded}<br>` +
    (m.temporary ? `Disperses when trade falls below ${MARKET.gathering.minVolume} a turn (now ${fmt(m.activity)})<br>` : '') +
    `${m.traders} clans trading · ${fmt(m.lastVolume ?? 0)} goods changed hands last turn ` +
    `(${fmt(food)} food${other > 0.05 ? `, ${fmt(other)} materials` : ''})<br>` +
    `<div class="sub">Exchange ratios last turn (geometric mean = 1)</div>${bars}` +
    `<div class="sub">Volume by good</div>${basketRow(vol)}` +
    `<div class="sub">Traded</div>${(m.lastTraders ?? []).join(', ') || '—'}` +
    (lines ? `<svg width="236" height="60" class="spark">${lines}</svg>` : '') + whyHtml(m.id);
}

const PHASE_LABEL = { pregame: 'Pre-game', choose: 'Founding', main: 'Game' };

// What leading a founding market would give the player, from state alone.
function candidateHtml(c) {
  const m = state.markets[c.marketId];
  const reach = marketReach(state, m);
  const clans = Object.values(state.clans).filter((x) => reach.has(key(x)));
  const pop = clans.reduce((a, x) => a + x.pop, 0);
  let tame = 0;
  const terr = {};
  for (const k of reach.keys()) {
    const t = state.map.tiles[k];
    if (t.tame === 1) tame++;
    terr[t.terrain] = (terr[t.terrain] ?? 0) + 1;
  }
  const mix = Object.entries(terr).sort((a, b) => b[1] - a[1]).slice(0, 3)
    .map(([t, n]) => `${t} ${Math.round((n / reach.size) * 100)}%`).join(', ');
  const rivals = Object.values(state.civs).filter((o) => o.id !== c.id).map((o) => distance(m, state.markets[o.marketId]));
  const near = rivals.length ? Math.min(...rivals) : null;
  const btn = `<button data-action='${JSON.stringify({ type: 'chooseCiv', civ: c.id })}'>Lead ${c.name}</button>`;
  return `<div class="cand"><div class="row">${btn} ${swatch(c.color)} at ${key(m)}</div>` +
    `<div class="sub">${clans.length} clans, ${fmt(pop)} people in reach · ${tame} domesticated tiles` +
    (near != null ? ` · nearest rival ${near} hexes` : '') + `<br>${mix}</div></div>`;
}

// Founding: the player picks which founding market to lead, or watches with all AI.
function chooseHtml() {
  return `<b>Choose your civ</b><div class="sub">The pre-game is over. Each founding market becomes a civ; ` +
    `you inherit the clans in its reach. Click a market on the map to see its trade.</div>` +
    Object.values(state.civs).map(candidateHtml).join('') +
    `<div class="row"><button data-action='${JSON.stringify({ type: 'chooseCiv', civ: null })}'>Watch (all AI)</button></div>`;
}

function statusHtml() {
  let html = `Turn ${state.turn} · ${PHASE_LABEL[state.phase]}`;
  const me = humanCiv(state);
  if (gameOver()) {
    const w = state.civs[state.winner];
    const kind = state.mods.civ?.winKind ?? 'victory';
    const mine = me && me.id === w?.id;
    html += `<div class="banner ${me ? (mine ? 'win' : 'lose') : 'win'}">${swatch(w?.color ?? '#fff')}` +
      `${w?.name ?? 'Someone'} wins by ${kind} on turn ${winTurn ?? state.turn - 1}${mine ? ' — you win!' : ''}</div>`;
  } else if (me && state.mods.civ?.lost?.[me.id] != null) {
    html += `<div class="banner lose">${me.name} has fallen (turn ${state.mods.civ.lost[me.id]}). ` +
      `You can keep ending turns to watch, or start a new game.</div>`;
  }
  return html;
}

function summaryHtml(fog) {
  const civRows = Object.values(state.civs).map((c) => {
    const lost = state.mods.civ?.lost?.[c.id] != null;
    return `${civName(c.id)}${c.isHuman ? ' (you)' : ''}${lost ? ' · fallen' : ''}` + (fog ? '' : ` at ${key(state.markets[c.marketId] ?? {})}`);
  }).join('<br>');
  if (fog) {
    // Only what the human civ can know: its own people, and what it has seen.
    const me = fog.civ;
    const mine = civClans(state, me.id);
    const seenClans = Object.values(state.clans).filter((c) => fog.seen(key(c)));
    const seenMarkets = Object.values(state.markets).filter((m) => fog.seen(key(m)));
    return `<b>${civName(me.id)}</b>: ${fmt(mine.reduce((a, c) => a + c.pop, 0))} people in ${mine.length} clans · ` +
      `${plural(civCities(state, me.id).length, 'city', 'cities')}<br>` +
      `${mine.filter((c) => c.fed).length} growing · ${mine.filter((c) => c.starving).length} starving<br>` +
      `Seen: ${seenClans.length} clans, ${seenMarkets.length} markets<br>${civRows}`;
  }
  const clans = Object.values(state.clans);
  return `Population ${fmt(totalPop(state))} · ${clans.length} clans<br>` +
    `${clans.filter((c) => c.fed).length} growing · ${clans.filter((c) => !c.fed && !c.starving).length} stable · ` +
    `${clans.filter((c) => c.starving).length} starving<br>${clans.filter((c) => c.market != null).length} trading · ` +
    `${Object.keys(state.markets).length} ${state.phase === 'pregame' ? 'gatherings' : 'markets'}<br>` +
    (state.phase === 'pregame' ? `Domesticated hearths ${hearths(state).length} of ${state.config.players}` : '') +
    `<svg width="236" height="40" class="spark">${sparkline(state.stats.pop)}</svg>` + civRows;
}

function selectionHtml(fog) {
  if (!sel) return '<i>Click a tile</i>';
  const t = tileAt(state, sel);
  const feats = [t.river && 'river', t.coast && 'coast', t.shallows && 'shallows'].filter(Boolean).join(', ');
  const head = `<b>${t.terrain}</b>${feats ? ` (${feats})` : ''} · ${sel.q},${sel.r}<br>`;
  // Contract 7: tiles the human has not seen show terrain only.
  if (fog && !fog.seen(key(sel))) return `${head}<i>Unexplored: send a scout to see who lives here.</i>`;
  const clan = clanAt(state, sel) ?? holder(state, sel);
  let market = marketAt(state, sel) ?? (clan && state.markets[clan.market]);
  if (market && fog && !fog.seen(key(market))) market = null;
  let html = head +
    `Fertility ×${t.fertility}<br>` +
    `Held ${t.occTotal} turns (${t.occ} in a row)<br>` +
    (t.wildGrain != null ? `Grain ${t.tame === 1 ? 'domesticated' : `${Math.round(t.tame * 100)}% tamed`}<br>` : '') +
    Object.keys(FOOD).filter((f) => t.cap[f]).map((f) => `${swatch(FOOD[f].color)}${f} ${fmt(t.stock[f])}/${t.cap[f]}`).join('<br>');
  if (clan)
    html += `<hr><b>${clan.name}</b>${clan.owner != null ? ` · ${civName(clan.owner)}` : ' · unaligned'} · pop ${fmt(clan.pop)} · ${clan.fed ? 'growing' : `${Math.round(clan.fedRatio * 100)}% fed, ${clan.starving ? 'starving' : 'stable'}`}<br>` +
      `Holds ${clan.land.length} tile${clan.land.length === 1 ? '' : 's'} (roam ${roamCost(state, clan.land)}/${CLAN.roam}) · settled ${clan.since} turns<br>` +
      (clan.dest ? `Walking to ${clan.dest}, ${clan.route.length} steps left<br>`
        : clan.starving ? `Starving ${clan.hungry} turn${clan.hungry === 1 ? '' : 's'}; moves on after ${CLAN.patience}<br>` : '') +
      `<div class="sub">Harvest</div>${basketRow(clan.harvest)}` +
      `<div class="sub">After trade</div>${basketRow(clan.basket)}` +
      `<div class="sub">Store</div>${basketRow(clan.store)}` +
      (clan.market != null ? `<div class="sub">Trips to market cost ${Math.round(clan.trip * 100)}% of labour</div>` : '') +
      whyHtml(clan.id);
  if (market) html += `<hr>${marketHtml(market)}`;
  return html;
}

function logHtml() {
  const me = humanCiv(state);
  const re = me && mineOnly ? new RegExp(`\\b${me.name}\\b`) : null;
  const line = (l) => `<div class="${isAlert(l) ? 'alert' : ''}">${esc(l)}</div>`;
  return log.filter((l) => !re || re.test(l) || isAlert(l)).slice(0, 30).map(line).join('');
}

// The new-game form is open in the pre-game and once the game is over, folded away otherwise.
let formFor = null;
function syncNewGame() {
  const want = state.phase === 'pregame' || gameOver();
  if (formFor !== want) document.getElementById('newGame').open = want;
  formFor = want;
}

function renderPanel() {
  syncNewGame();
  const fog = fogOf(state);
  const main = state.phase === 'main';
  const over = gameOver();
  document.getElementById('status').innerHTML = statusHtml();
  const stepBtn = document.getElementById('step');
  stepBtn.textContent = main ? 'End turn (Space)' : 'Step (Space)';
  stepBtn.disabled = !canStep();
  document.getElementById('play').disabled = !canStep();
  const fast = document.getElementById('fast');
  fast.disabled = !canStep();
  fast.textContent = main && !watching() ? `Fast-forward ${FAST_TURNS} (G)` : state.phase === 'pregame' ? 'Fast-forward to founding (G)' : 'Fast-forward (G)';
  document.getElementById('skip').hidden = state.phase !== 'pregame';
  document.getElementById('choose').innerHTML = state.phase === 'choose' ? chooseHtml() : '';
  const v = VIEWS[mode];
  document.getElementById('legend').innerHTML = mode === 'terrain' ? '' :
    `<div class="sub">${fog ? (v.fogNote ?? v.note) : v.note}${fog && mode !== 'passable' ? ' (only tiles you have seen)' : ''}</div>` +
    (range ? `<div class="ramp" style="background:${rampCss}"></div><div class="ends"><span>${fmt(range.lo)}</span><span>${fmt(range.hi)}</span></div>` : '');
  document.getElementById('play').textContent = timer ? 'Pause (P)' : 'Play (P)';
  document.getElementById('summary').innerHTML = summaryHtml(fog);
  document.getElementById('selection').innerHTML = selectionHtml(fog);
  document.getElementById('modules').innerHTML = MODULE_UIS
    .map((ui) => ui.panel?.(state, sel, uiFor(ui.name)) ?? '').filter(Boolean).join('<hr>');
  document.getElementById('alerts').innerHTML = alerts.length
    ? `<div class="sub">Alerts</div>${alerts.map((l) => `<div class="alert">${esc(l)}</div>`).join('')}` : '';
  document.getElementById('mineRow').hidden = !humanCiv(state);
  document.getElementById('log').innerHTML = logHtml();
  if (over) stop();
}

// Console / agent handle: game.state, game.act({...}), game.history, game.replay(), and
// game.ui for automated play-tests (click a hex without pixel maths, Escape, module scratch).
window.game = {
  get state() { return state; },
  act: (a) => { const ok = act(a); update(); return ok; },
  history,
  config,
  get view() { return view; },
  rules,
  replay: () => replay(config, history),
  ui: {
    select,
    click: select,
    escape,
    get selection() { return sel; },
    locals,
    step: (n = 1) => step(n),
    fast: () => run('fast'),
    stop: () => { stop(); update(); },
    get running() { return timer != null; },
    get alerts() { return alerts.slice(); },
    get log() { return log.slice(); },
    setMode,
  },
};

update();
