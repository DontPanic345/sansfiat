import { CLAN, FOOD, FOODS } from './data.js';
import { fromPixel } from './hex.js';
import { newGame, replay } from './game.js';
import { draw } from './render.js';
import * as rules from './rules.js';
import { VIEWS, rampCss } from './views.js';

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
    act({ type: 'endTurn' });
    if (state.phase !== before) { stop(); break; }
  }
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
modeSelect.innerHTML = Object.entries(VIEWS).map(([k, v], i) => `<option value="${k}">${i + 1}. ${v.label}</option>`).join('');
const setMode = (m) => { mode = m; modeSelect.value = m; update(); };
modeSelect.onchange = () => setMode(modeSelect.value);
document.getElementById('depletion').onchange = (e) => { depletion = e.target.checked; update(); };
canvas.addEventListener('click', (ev) => {
  const rect = canvas.getBoundingClientRect();
  const h = fromPixel(ev.clientX - rect.left - view.ox, ev.clientY - rect.top - view.oy, view.size);
  sel = rules.tileAt(state, h) ? h : null;
  update();
});
addEventListener('keydown', (e) => {
  if (['INPUT', 'SELECT'].includes(e.target.tagName)) return;
  if (e.key === ' ') { e.preventDefault(); stop(); step(); }
  else if (e.key === 'p') document.getElementById('play').click();
  else if (e.key === 'g') run('fast');
  else if (e.key === 'd') { depletion = !depletion; document.getElementById('depletion').checked = depletion; update(); }
  else if (e.key === 'Escape') { sel = null; update(); }
  else if (/^[1-9]$/.test(e.key) && Object.keys(VIEWS)[e.key - 1]) setMode(Object.keys(VIEWS)[e.key - 1]);
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
  range = draw(ctx, state, { ...view, selected: sel, depletion, mode });
  renderPanel();
}

const fmt = (v) => (v >= 10 ? v.toFixed(0) : v.toFixed(1));
const swatch = (color) => `<span class="swatch" style="background:${color}"></span>`;

function basketRow(b) {
  const parts = FOODS.filter((f) => (b[f] ?? 0) > 0.05).map((f) => `${swatch(FOOD[f].color)}${fmt(b[f])}`);
  return parts.length ? parts.join(' ') : '—';
}

function sparkline(series, { width = 236, height = 40, color = '#dde3ea', min, max } = {}) {
  if (series.length < 2) return '';
  const lo = min ?? Math.min(...series), hi = max ?? Math.max(...series);
  const pts = series.map((v, i) => `${(i / (series.length - 1)) * width},${height - ((v - lo) / (hi - lo || 1)) * height}`);
  return `<polyline fill="none" stroke="${color}" stroke-width="1.5" points="${pts.join(' ')}"/>`;
}

function marketHtml(m) {
  const owner = m.owner == null ? 'Neutral market' : `${swatch(state.players[m.owner].color)}${state.players[m.owner].name}`;
  const goods = FOODS.filter((f) => m.prices[f]);
  const hi = Math.max(...goods.map((f) => m.prices[f]), 1);
  const bars = goods.map((f) =>
    `<div class="bar"><span>${f}</span><i style="width:${(m.prices[f] / hi) * 100}%;background:${FOOD[f].color}"></i><b>${m.prices[f].toFixed(2)}</b></div>`).join('');
  const hist = m.history.filter((h) => goods.every((f) => h.prices[f]));
  const all = hist.flatMap((h) => goods.map((f) => Math.log(h.prices[f])));
  const lines = goods.map((f) => sparkline(hist.map((h) => Math.log(h.prices[f])),
    { color: FOOD[f].color, min: Math.min(...all), max: Math.max(...all), height: 60 })).join('');
  return `<b>${owner}</b> · since turn ${m.founded}<br>` +
    `${m.traders} clans trading · ${fmt(m.lastVolume)} food changed hands last turn<br>` +
    `<div class="sub">Exchange ratios (geometric mean = 1)</div>${bars}` +
    (lines ? `<svg width="236" height="60" class="spark">${lines}</svg>` : '');
}

function renderPanel() {
  const clans = Object.values(state.clans);
  document.getElementById('status').textContent =
    `Turn ${state.turn} · ${state.phase === 'pregame' ? 'Pre-game' : 'Game'}`;
  const v = VIEWS[mode];
  document.getElementById('legend').innerHTML = mode === 'terrain' ? '' :
    `<div class="sub">${v.note}</div>` +
    (range ? `<div class="ramp" style="background:${rampCss}"></div><div class="ends"><span>${fmt(range.lo)}</span><span>${fmt(range.hi)}</span></div>` : '');
  document.getElementById('play').textContent = timer ? 'Pause (P)' : 'Play (P)';
  document.getElementById('summary').innerHTML =
    `Population ${fmt(rules.totalPop(state))} · ${clans.length} clans<br>` +
    `${clans.filter((c) => c.fed).length} fed · ${clans.filter((c) => c.market != null).length} trading · ` +
    `${Object.keys(state.markets).length} markets` +
    `<svg width="236" height="40" class="spark">${sparkline(state.stats.pop)}</svg>` +
    state.players.filter((p) => p.market != null)
      .map((p) => `${swatch(p.color)}${p.name} at ${state.markets[p.market].q},${state.markets[p.market].r}`).join('<br>');

  const el = document.getElementById('selection');
  if (!sel) {
    el.innerHTML = '<i>Click a tile</i>';
  } else {
    const t = rules.tileAt(state, sel);
    const clan = rules.clanAt(state, sel);
    const market = rules.marketAt(state, sel) ?? (clan && state.markets[clan.market]);
    const feats = [t.river && 'river', t.coast && 'coast', t.shallows && 'shallows'].filter(Boolean).join(', ');
    let html = `<b>${t.terrain}</b>${feats ? ` (${feats})` : ''} · ${sel.q},${sel.r}<br>` +
      `Fertility ×${t.fertility}<br>` +
      `Lived on ${t.occTotal} turns (${t.occ} in a row)<br>` +
      FOODS.filter((f) => t.cap[f]).map((f) => `${swatch(FOOD[f].color)}${f} ${fmt(t.stock[f])}/${t.cap[f]}`).join('<br>');
    if (clan)
      html += `<hr><b>${clan.name}</b> · pop ${fmt(clan.pop)} · ${clan.fed ? 'fed' : `${Math.round(clan.fedRatio * 100)}% fed`}<br>` +
        (clan.dest ? `Walking to ${clan.dest}, ${clan.route.length} steps left<br>`
          : !clan.fed ? `Hungry ${clan.hungry} turn${clan.hungry === 1 ? '' : 's'}; moves on after ${CLAN.patience}<br>` : '') +
        `<div class="sub">Harvest</div>${basketRow(clan.harvest)}` +
        `<div class="sub">After trade</div>${basketRow(clan.basket)}` +
        `<div class="sub">Store</div>${basketRow(clan.store)}` +
        (clan.market != null ? `<div class="sub">Trips to market cost ${Math.round(clan.trip * 100)}% of labour</div>` : '');
    if (market) html += `<hr>${marketHtml(market)}`;
    el.innerHTML = html;
  }
  document.getElementById('log').textContent = log.slice(0, 30).join('\n');
}

// Console / agent handle: game.state, game.act({...}), game.history, game.rules
window.game = {
  get state() { return state; },
  act: (a) => { const ok = act(a); update(); return ok; },
  history,
  config,
  rules,
  replay: () => replay(config, history),
};

update();
