import { playTurn } from './ai.js';
import { SUPPLY, UNITS } from './data.js';
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
  const unit = rules.unitAt(state, h);
  const city = rules.cityAt(state, h);
  sel = unit && !(city && sel?.kind === 'unit' && sel.id === unit.id) ? { kind: 'unit', id: unit.id }
    : city ? { kind: 'city', id: city.id }
    : null;
  update();
}

const gatherType = (u) => ([null, u.owner].includes(rules.territoryOwner(state, u)) ? 'forage' : 'pillage');

function nextUnit() {
  const mine = rules.unitsOf(state, state.current).filter((u) => u.labor > 0);
  if (!mine.length) return;
  const i = mine.findIndex((u) => u.id === sel?.id);
  sel = { kind: 'unit', id: mine[(i + 1) % mine.length].id };
}

function endTurn() {
  act({ type: 'endTurn' });
  runAI();
  sel = null;
  nextUnit();
  update();
}

document.getElementById('endTurn').onclick = endTurn;
canvas.addEventListener('click', onClick);
addEventListener('keydown', (e) => {
  if (e.target.tagName === 'SELECT') return;
  if (e.key === ' ' || e.key === 'Enter') { e.preventDefault(); endTurn(); }
  else if (e.key === 'n') { nextUnit(); update(); }
  else if (e.key === 'f' && selectedUnit()) { act({ type: 'foundCity', unitId: sel.id }); nextUnit(); update(); }
  else if (e.key === 'r' && selectedUnit()) { act({ type: 'resupply', unitId: sel.id }); update(); }
  else if (e.key === 'g' && selectedUnit()) { act({ type: gatherType(selectedUnit()), unitId: sel.id }); update(); }
  else if (e.key === 'Escape') { sel = null; update(); }
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
    reach: mine ? rules.reachable(state, u) : null,
    targets: mine ? attackTargets(u) : [],
    supply: u && UNITS[u.type].upkeep ? supplyView(u) : null,
  });
  renderPanel();
}

// ok: the unit could draw on this line now (spare supply, enough labor).
function supplyView(u) {
  const src = rules.supplySource(state, u);
  return src && { line: src.line, ok: src.city.spare > 0 && u.labor >= src.labor };
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
    const def = UNITS[u.type];
    const src = def.upkeep && rules.supplySource(state, u);
    const tile = rules.tileAt(state, u);
    const button = (type, text) => {
      const err = rules.validate(state, { type, unitId: u.id });
      return `<button data-act="${type}" ${err ? `disabled title="${err}"` : ''}>${text}</button>`;
    };
    el.innerHTML = `<b>${state.players[u.owner].name} ${u.type}</b> <span class="dim">on ${tile.terrain}</span>` +
      `<br>Labor ${u.labor}/${def.labor} · HP ${u.hp}/${def.hp}` +
      (def.upkeep ? `<br>Food ${u.food}/${def.food} <span class="dim">(eats ${def.upkeep}/turn, ${SUPPLY.fightFood} per fight)</span>` +
        `<br>Supply: ${src ? `${src.city.name} — line ${src.cost}, resupply costs ${src.labor} labor, city spare ${src.city.spare}` : '<b>cut off</b>'}` +
        (u.food === 0 ? '<br><b class="warn">Starving: resupply or lose HP</b>' : '') : '') +
      (mine && def.upkeep ? `<div class="actions">${button('resupply', 'Resupply (R)')}` +
        `${button(gatherType(u), gatherType(u) === 'forage' ? 'Forage (G)' : 'Pillage (G)')}</div>` : '') +
      (mine && def.canFound ? `<br><button data-act="foundCity">Found city (F)</button>` : '');
    for (const b of el.querySelectorAll('button[data-act]')) {
      b.addEventListener('click', () => {
        act({ type: b.dataset.act, unitId: u.id });
        if (b.dataset.act === 'foundCity') nextUnit();
        update();
      });
    }
  } else if (c) {
    const y = rules.cityYield(state, c);
    const opts = Object.keys(UNITS).map((t) => `<option ${t === c.build ? 'selected' : ''}>${t}</option>`).join('');
    el.innerHTML = `<b>${c.name}</b><br>Food ${y.food} · Prod ${y.prod}<br>` +
      `Spare supply ${c.spare}<br>Building ${c.stock}/${UNITS[c.build].cost} <select id="build" ${c.owner === state.current ? '' : 'disabled'}>${opts}</select>`;
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
