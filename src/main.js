import { CLAN, FOOD, GOODS, MARKET } from './data.js';
import { fromPixel, key } from './hex.js';
import { newGame, replay, runPregameCounted } from './game.js';
import { clanAt, holder, humanCiv, marketAt, notesFor, tileAt, totalPop } from './core.js';
import { hearths, roamCost } from './clans.js';
import { draw } from './render.js';
import * as rules from './rules.js';
import { MODULE_UIS } from './modules/ui.js';
import { VIEWS, VIEW_KEYS, rampCss } from './views.js';

const params = new URLSearchParams(location.search);
const config = {
  seed: Number(params.get('seed') ?? Math.floor(Math.random() * 1e9)),
  players: Number(params.get('players') ?? 3),
};

let state = newGame(config);
const history = [];
const log = [];
let sel = null; // {q, r}
let depletion = true;
let mode = 'terrain';
let range = null;
let timer = null;
// UI-only state per module (selected unit, paint brush, ...). Never part of game state.
const locals = Object.fromEntries(MODULE_UIS.map((u) => [u.name, {}]));
const uiFor = (name) => ({ act: (a) => { const ok = act(a); update(); return ok; }, local: locals[name], selection: sel });

const canvas = document.getElementById('map');
const ctx = canvas.getContext('2d');
let view = {};

function act(action) {
  const { state: next, error } = rules.dispatch(state, action);
  if (error) {
    log.unshift(`✗ ${action.type}: ${error}`);
    return false;
  }
  state = next;
  history.push(action);
  log.unshift(...state.events.slice().reverse());
  return true;
}

function step(n = 1) {
  for (let i = 0; i < n; i++) {
    const before = state.phase;
    if (!act({ type: 'endTurn' }) || state.phase !== before) { stop(); break; }
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
  log.unshift(...state.events.slice().reverse());
  update();
}

// mode: 'play' (one turn per tick) or 'fast' (to game start)
function run(mode) {
  stop();
  timer = setInterval(() => step(mode === 'fast' ? 5 : 1), mode === 'fast' ? 16 : 150);
  update();
}

function stop() {
  clearInterval(timer);
  timer = null;
}

document.getElementById('step').onclick = () => { stop(); step(); };
document.getElementById('play').onclick = () => (timer ? (stop(), update()) : run('play'));
document.getElementById('fast').onclick = () => run('fast');
document.getElementById('skip').onclick = () => skipPregame();
// Any element with data-action='{json}' (module panels, the founding choice) dispatches it.
document.querySelector('aside').addEventListener('click', (e) => {
  const el = e.target.closest('[data-action]');
  if (!el) return;
  e.preventDefault();
  act(JSON.parse(el.dataset.action));
  update();
});
const loadSeed = (seed) => {
  params.set('seed', seed);
  location.search = params;
};
document.getElementById('newMap').onclick = (e) => { e.preventDefault(); loadSeed(Math.floor(Math.random() * 1e9)); };
document.getElementById('seedForm').onsubmit = (e) => {
  e.preventDefault();
  const v = document.getElementById('seed').value;
  if (v !== '') loadSeed(Math.floor(Number(v)));
};
document.getElementById('seed').value = config.seed;
const modeSelect = document.getElementById('mode');
modeSelect.innerHTML = Object.entries(VIEWS).map(([k, v], i) => `<option value="${k}">${VIEW_KEYS[i]}. ${v.label}</option>`).join('');
const setMode = (m) => { mode = m; modeSelect.value = m; update(); };
modeSelect.onchange = () => setMode(modeSelect.value);
document.getElementById('depletion').onchange = (e) => { depletion = e.target.checked; update(); };
canvas.addEventListener('click', (ev) => {
  const rect = canvas.getBoundingClientRect();
  const h = fromPixel(ev.clientX - rect.left - view.ox, ev.clientY - rect.top - view.oy, view.size);
  if (!tileAt(state, h)) { sel = null; update(); return; }
  for (const ui of MODULE_UIS) if (ui.onClick?.(state, h, uiFor(ui.name))) { update(); return; }
  sel = h;
  update();
});
addEventListener('keydown', (e) => {
  if (['INPUT', 'SELECT'].includes(e.target.tagName)) return;
  if (e.key === ' ') { e.preventDefault(); stop(); step(); }
  else if (e.key === 'p') document.getElementById('play').click();
  else if (e.key === 'g') run('fast');
  else if (e.key === 'k') skipPregame();
  else if (e.key === 'd') { depletion = !depletion; document.getElementById('depletion').checked = depletion; update(); }
  else if (e.key === 'Escape') { sel = null; update(); }
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
  return `<b>${owner}</b> · since turn ${m.founded}<br>` +
    (m.temporary ? `Disperses when trade falls below ${MARKET.gathering.minVolume} a turn (now ${fmt(m.activity)})<br>` : '') +
    `${m.traders} clans trading · ${fmt(m.lastVolume)} food changed hands last turn<br>` +
    `<div class="sub">Exchange ratios last turn (geometric mean = 1)</div>${bars}` +
    `<div class="sub">Volume by good</div>${basketRow(m.lastTrade ?? {})}` +
    `<div class="sub">Traded</div>${(m.lastTraders ?? []).join(', ') || '—'}` +
    (lines ? `<svg width="236" height="60" class="spark">${lines}</svg>` : '') + whyHtml(m.id);
}

const PHASE_LABEL = { pregame: 'Pre-game', choose: 'Founding', main: 'Game' };

// Founding: the player picks which founding market to lead, or watches with all AI.
function chooseHtml() {
  const btn = (civ, label) => `<button data-action='${JSON.stringify({ type: 'chooseCiv', civ })}'>${label}</button>`;
  return `<b>Choose your civ</b><div class="sub">The pre-game is over. Each founding market becomes a civ.</div>` +
    Object.values(state.civs).map((c) => {
      const m = state.markets[c.marketId];
      return `<div class="row">${btn(c.id, `Lead ${c.name}`)} ${swatch(c.color)} market at ${key(m)}, ${m.traders} traders</div>`;
    }).join('') + `<div class="row">${btn(null, 'Watch (all AI)')}</div>`;
}

function renderPanel() {
  const clans = Object.values(state.clans);
  document.getElementById('status').textContent = `Turn ${state.turn} · ${PHASE_LABEL[state.phase]}`;
  document.getElementById('step').textContent = state.phase === 'main' ? 'End turn' : 'Step';
  document.getElementById('skip').disabled = state.phase !== 'pregame';
  document.getElementById('choose').innerHTML = state.phase === 'choose' ? chooseHtml() : '';
  const v = VIEWS[mode];
  document.getElementById('legend').innerHTML = mode === 'terrain' ? '' :
    `<div class="sub">${v.note}</div>` +
    (range ? `<div class="ramp" style="background:${rampCss}"></div><div class="ends"><span>${fmt(range.lo)}</span><span>${fmt(range.hi)}</span></div>` : '');
  document.getElementById('play').textContent = timer ? 'Pause (P)' : 'Play (P)';
  document.getElementById('summary').innerHTML =
    `Population ${fmt(totalPop(state))} · ${clans.length} clans<br>` +
    `${clans.filter((c) => c.fed).length} growing · ${clans.filter((c) => !c.fed && !c.starving).length} stable · ` +
    `${clans.filter((c) => c.starving).length} starving<br>${clans.filter((c) => c.market != null).length} trading · ` +
    `${Object.keys(state.markets).length} ${state.phase === 'pregame' ? 'gatherings' : 'markets'}<br>` +
    (state.phase === 'pregame' ? `Domesticated hearths ${hearths(state).length} of ${state.config.players}` : '') +
    `<svg width="236" height="40" class="spark">${sparkline(state.stats.pop)}</svg>` +
    Object.values(state.civs).map((c) => `${civName(c.id)}${c.isHuman ? ' (you)' : ''} at ${key(state.markets[c.marketId] ?? {})}`).join('<br>');

  const el = document.getElementById('selection');
  if (!sel) {
    el.innerHTML = '<i>Click a tile</i>';
  } else {
    const t = tileAt(state, sel);
    const clan = clanAt(state, sel) ?? holder(state, sel);
    const market = marketAt(state, sel) ?? (clan && state.markets[clan.market]);
    const feats = [t.river && 'river', t.coast && 'coast', t.shallows && 'shallows'].filter(Boolean).join(', ');
    let html = `<b>${t.terrain}</b>${feats ? ` (${feats})` : ''} · ${sel.q},${sel.r}<br>` +
      `Fertility ×${t.fertility}<br>` +
      `Held ${t.occTotal} turns (${t.occ} in a row)<br>` +
      (t.wildGrain != null ? `Grain ${t.tame === 1 ? 'domesticated' : `${Math.round(t.tame * 100)}% tamed`}<br>` : '') +
      Object.keys(FOOD).filter((f) => t.cap[f]).map((f) => `${swatch(FOOD[f].color)}${f} ${fmt(t.stock[f])}/${t.cap[f]}`).join('<br>');
    if (clan)
      html += `<hr><b>${clan.name}</b>${clan.owner != null ? ` · ${civName(clan.owner)}` : ''} · pop ${fmt(clan.pop)} · ${clan.fed ? 'growing' : `${Math.round(clan.fedRatio * 100)}% fed, ${clan.starving ? 'starving' : 'stable'}`}<br>` +
        `Holds ${clan.land.length} tile${clan.land.length === 1 ? '' : 's'} (roam ${roamCost(state, clan.land)}/${CLAN.roam}) · settled ${clan.since} turns<br>` +
        (clan.dest ? `Walking to ${clan.dest}, ${clan.route.length} steps left<br>`
          : clan.starving ? `Starving ${clan.hungry} turn${clan.hungry === 1 ? '' : 's'}; moves on after ${CLAN.patience}<br>` : '') +
        `<div class="sub">Harvest</div>${basketRow(clan.harvest)}` +
        `<div class="sub">After trade</div>${basketRow(clan.basket)}` +
        `<div class="sub">Store</div>${basketRow(clan.store)}` +
        (clan.market != null ? `<div class="sub">Trips to market cost ${Math.round(clan.trip * 100)}% of labour</div>` : '') +
        whyHtml(clan.id);
    if (market) html += `<hr>${marketHtml(market)}`;
    el.innerHTML = html;
  }
  document.getElementById('modules').innerHTML = MODULE_UIS
    .map((ui) => ui.panel?.(state, sel, uiFor(ui.name)) ?? '').filter(Boolean).join('<hr>');
  document.getElementById('log').textContent = log.slice(0, 30).join('\n');
}

// Console / agent handle: game.state, game.act({...}), game.history, game.rules
window.game = {
  get state() { return state; },
  act: (a) => { const ok = act(a); update(); return ok; },
  history,
  config,
  get view() { return view; },
  rules,
  replay: () => replay(config, history),
};

update();
