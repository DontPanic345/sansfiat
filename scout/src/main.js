import { playTurn } from './ai.js';
import { UNITS } from './data.js';
import { distance, fromPixel, key } from './hex.js';
import { newGame, replay } from './game.js';
import { draw } from './render.js';
import * as rules from './rules.js';

const params = new URLSearchParams(location.search);
const config = {
  seed: Number(params.get('seed') ?? Math.floor(Math.random() * 1e9)),
  players: Number(params.get('players') ?? 2),
};
// ?ai=1,2 sets which players the AI controls; ?ai= for all-human hotseat.
const aiPlayers = new Set((params.get('ai') ?? '1').split(',').filter(Boolean).map(Number));

let state = newGame(config);
const history = [];
let sel = null; // {kind: 'unit' | 'city', id}
let picking = null; // {unitId, from?} while choosing a scout route
const log = [];

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

function runAI() {
  while (aiPlayers.has(state.current) && state.winner == null) {
    const before = state.current;
    state = playTurn(state, (s, a) => {
      const r = rules.dispatch(s, a);
      if (!r.error) {
        history.push(a);
        log.unshift(...r.state.events.slice().reverse());
      }
      return r;
    });
    if (state.current === before) break;
  }
}

const selectedUnit = () => (sel?.kind === 'unit' ? state.units[sel.id] : null);
const selectedCity = () => (sel?.kind === 'city' ? state.cities[sel.id] : null);

function attackTargets(u) {
  if (!u || u.owner !== state.current) return [];
  return [...Object.values(state.units), ...Object.values(state.cities)]
    .filter((t) => t.owner !== u.owner && distance(t, u) === 1)
    .filter((t) => !rules.validate(state, { type: 'move', unitId: u.id, to: { q: t.q, r: t.r } }));
}

function onClick(ev) {
  const rect = canvas.getBoundingClientRect();
  const h = fromPixel(ev.clientX - rect.left - view.ox, ev.clientY - rect.top - view.oy, view.size);
  if (picking) {
    if (!rules.tileAt(state, h)) return;
    if (!picking.from) picking.from = h;
    else {
      act({ type: 'assignRoute', unitId: picking.unitId, from: picking.from, to: h });
      picking = null;
    }
    return update();
  }
  const u = selectedUnit();
  if (u && u.owner === state.current) {
    if (attackTargets(u).some((t) => t.q === h.q && t.r === h.r)) {
      act({ type: 'move', unitId: u.id, to: h });
      return update();
    }
    const reach = rules.reachable(state, u);
    if (reach.has(key(h))) {
      for (const step of rules.pathTo(reach, h)) if (!act({ type: 'move', unitId: u.id, to: step })) break;
      return update();
    }
  }
  // Repeated clicks on a tile cycle through its units, then its city.
  const city = rules.cityAt(state, h);
  const here = [...rules.unitsAt(state, h).map((x) => ({ kind: 'unit', id: x.id })), ...(city ? [{ kind: 'city', id: city.id }] : [])];
  const i = here.findIndex((x) => x.kind === sel?.kind && x.id === sel?.id);
  sel = here[(i + 1) % here.length] ?? null;
  update();
}

function nextUnit() {
  const mine = rules.unitsOf(state, state.current).filter((u) => u.movesLeft > 0);
  if (!mine.length) return;
  const i = mine.findIndex((u) => u.id === sel?.id);
  sel = { kind: 'unit', id: mine[(i + 1) % mine.length].id };
}

function startRoute() {
  const u = selectedUnit();
  if (u && u.owner === state.current && UNITS[u.type].canScout) picking = { unitId: u.id };
  update();
}

function endTurn() {
  picking = null;
  act({ type: 'endTurn' });
  runAI();
  sel = null;
  nextUnit();
  update();
}

document.getElementById('seed').textContent = `Seed ${config.seed}`;
document.getElementById('endTurn').onclick = endTurn;
canvas.addEventListener('click', onClick);
addEventListener('keydown', (e) => {
  if (e.target.tagName === 'SELECT') return;
  if (e.key === ' ' || e.key === 'Enter') { e.preventDefault(); endTurn(); }
  else if (e.key === 'n') { nextUnit(); update(); }
  else if (e.key === 'f' && selectedUnit()) { act({ type: 'foundCity', unitId: sel.id }); nextUnit(); update(); }
  else if (e.key === 't') startRoute();
  else if (e.key === 'Escape') { if (picking) picking = null; else sel = null; update(); }
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
  const u = selectedUnit();
  const mine = u && u.owner === state.current;
  draw(ctx, state, {
    ...view,
    selected: sel && (u ?? selectedCity()),
    selectedUnit: u?.id,
    reach: mine ? rules.reachable(state, u) : null,
    targets: mine ? attackTargets(u) : [],
    supplyLine: u ? rules.supplyLine(state, u) : [],
    viewer: state.current,
    route: u?.route ? { ...u.route, ...rules.routePlan(state, u.owner, u.route.from, u.route.to) } : null,
  });
  renderPanel();
}

function routeInfo(u, mine) {
  if (picking?.unitId === u.id) return `<br><i>Click the route ${picking.from ? 'target' : 'start'} (Esc cancels)</i>`;
  const cost = (c) => (c === Infinity ? 'none' : c);
  let html = '';
  if (u.route) {
    const plan = rules.routePlan(state, u.owner, u.route.from, u.route.to);
    html += `<br>Route: known ${cost(plan.known.cost)} · possible ${cost(plan.possible.cost)}` +
      (plan.unknown.length ? `<br>${plan.unknown.length} unrevealed tile(s) could improve it` : '');
  }
  return html + (mine ? `<br><button id="route">${u.route ? 'Reassign' : 'Assign'} route (T)</button>` : '');
}

function renderPanel() {
  const p = state.players[state.current];
  const swatch = (pl) => `<span class="swatch" style="background:${pl.color}"></span>`;
  document.getElementById('status').innerHTML = state.winner != null
    ? `${swatch(state.players[state.winner])}${state.players[state.winner].name} wins`
    : `Turn ${state.turn} — ${swatch(p)}${p.name}`;

  const el = document.getElementById('selection');
  const u = selectedUnit(), c = selectedCity();
  if (u) {
    const mine = u.owner === state.current;
    const others = rules.unitsAt(state, u).filter((x) => x.id !== u.id).map((x) => x.name);
    el.innerHTML = `<b>${rules.unitName(state, u)}</b>` +
      (others.length ? `<br><small>Also here: ${others.join(', ')} (click to cycle)</small>` : '') + `<br>Moves ${u.movesLeft}/${UNITS[u.type].moves}` +
      (UNITS[u.type].upkeep ? `<br>Supplied by ${u.supplier ? state.cities[u.supplier]?.name : 'nobody'}` : '') +
      (mine && UNITS[u.type].canFound ? `<br><button id="found">Found city (F)</button>` : '') +
      (UNITS[u.type].canScout ? routeInfo(u, mine) : '');
    document.getElementById('route')?.addEventListener('click', startRoute);
    document.getElementById('found')?.addEventListener('click', () => {
      act({ type: 'foundCity', unitId: u.id });
      nextUnit();
      update();
    });
  } else if (c) {
    const y = rules.cityYield(state, c);
    const opts = Object.keys(UNITS).map((t) => `<option ${t === c.build ? 'selected' : ''}>${t}</option>`).join('');
    el.innerHTML = `<b>${c.name}</b><br>Food ${y.food} · Prod ${y.prod}<br>` +
      `Building ${c.stock}/${UNITS[c.build].cost} <select id="build" ${c.owner === state.current ? '' : 'disabled'}>${opts}</select>`;
    document.getElementById('build').addEventListener('change', (e) => {
      act({ type: 'setBuild', cityId: c.id, build: e.target.value });
      update();
    });
  } else {
    el.innerHTML = '<i>Nothing selected</i>';
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

runAI();
nextUnit();
update();
