import init, { Sim, terrain, goods, rules, sizes } from './pkg/web.js';
import { MapView, hue } from './map-view.js';
import { Feed } from './feed.js';
import { f2, keyOf, tileOf, alive, candidates, describe, tileKind, marketOn } from './select.js';

await init();
const TERRAIN = JSON.parse(terrain());
const GOODS = JSON.parse(goods());
const RULES = JSON.parse(rules());
const SIZES = JSON.parse(sizes());
const RATES = { slow: 1, normal: 4, fast: 20 };

const $ = (id) => document.getElementById(id);
const params = new URLSearchParams(location.search);

let sim = null;
let layout = null;
let v = null; // the last Sim.view()
let cursor = 0; // into Sim.events
let selected = null; // {kind: 'unit', id} | {kind: 'capital', civ} | {kind: 'tile', tile, clan, market}
let pickFor = null; // the Sim the pick card was built for
let playing = false;
let running = false;
let loopTimer = null;
let last = 0;
let acc = 0;

// ---- map, feed -------------------------------------------------------------------------------

const view = new MapView($('map'), {
  terrain: TERRAIN,
  rules: RULES,
  onPick,
  onCommand,
  onPaint,
  onHover: showTip,
  onHoverEnd: hideTip,
});
const feed = new Feed({ list: $('feed'), filters: $('filters'), onClick: onFeed, mine: (e) => v?.player != null && e.civ === v.player });
new ResizeObserver(() => view.resize()).observe($('mapbox'));

/** What the selection panel and the pick read the world through. */
const ctx = () => ({ v, layout, terrain: TERRAIN, goods: GOODS, rules: RULES, sim, view, painting });

// ---- starting a map --------------------------------------------------------------------------

const sizeSelect = $('size');
for (const s of SIZES) sizeSelect.add(new Option(`${s.name} (radius ${s.radius})`, s.name));

function start(next, seedShown) {
  stop();
  sim = next;
  layout = JSON.parse(sim.layout());
  selected = null;
  pickFor = null;
  view.setLayout(layout);
  view.setHighlight(null);
  view.setSelection(-1);
  document.body.dataset.selected = '';
  cursor = 0;
  feed.reset();
  $('seed').value = seedShown;
  const size = SIZES.find((s) => s.radius === layout.radius);
  if (size) sizeSelect.value = size.name;
  $('error').textContent = '';
  refresh();
  view.fit();
  view.drawNow();
}

function newMap() {
  const size = SIZES.find((s) => s.name === sizeSelect.value) || SIZES[0];
  const seed = Math.max(0, Math.floor(Number($('seed').value) || 0));
  const url = new URL(location.href);
  url.searchParams.set('size', size.name);
  url.searchParams.set('seed', seed);
  for (const k of ['play', 'speed', 'run']) url.searchParams.delete(k);
  history.replaceState(null, '', url);
  start(new Sim(seed, size.radius), seed);
}

// ---- the turn --------------------------------------------------------------------------------

/** Re-reads the world after turns: the top bar, the feed, the panel, the map. */
function refresh() {
  v = JSON.parse(sim.view());
  const sum = v.summary;
  view.setDynamic(v);
  const centres = v.end ? v.end.centres.length : sum.centres.length;
  $('turn').textContent = v.turn;
  $('people').textContent = Math.round(sum.people);
  $('clans').textContent = sum.clans;
  $('markets').textContent = sum.markets;
  $('hearths').textContent = sum.hearths;
  $('centres').textContent = `${centres} / ${RULES.players}`;
  const over = `turn ${v.end?.turn}, by ${v.end?.by_limit ? 'the limit' : 'centres'}${v.end?.topped_up.length ? `, +${v.end.topped_up.length} topped up` : ''}`;
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
  const mine = v.player != null ? Object.values(v.units).filter((u) => u.civ === v.player) : [];
  const warriors = mine.reduce((a, u) => a + u.people, 0);
  document.body.dataset.turn = v.turn;
  document.body.dataset.phase = v.phase;
  document.body.dataset.centres = centres;
  document.body.dataset.civ = v.player ?? '';
  document.body.dataset.units = mine.length;
  document.body.dataset.warriors = Math.round(warriors);
  const choose = v.phase === 'choose';
  for (const id of ['end', 'play', 'run']) {
    $(id).disabled = choose;
    $(id).title = choose ? 'Choose a civ first' : '';
  }
  $('run').hidden = v.phase === 'main';
  civBar(warriors);
  routes();
  pickCard();
  if (selected && !alive(ctx(), selected)) {
    selected = null;
    syncSelection();
  }
  pullEvents();
  panel();
  view.drawNow();
}

/** Hands the map the player's camps' routes (the selected one drawn firmly, the rest faint), and the
 *  selected camp's line home. */
function routes() {
  const out = [];
  for (const u of Object.values(v.units)) {
    if (u.civ !== v.player || u.dest == null) continue;
    const r = JSON.parse(sim.route(u.id));
    if (r) out.push({ unit: u.id, civ: u.civ, tiles: [u.tile, ...r.path], selected: selected?.kind === 'unit' && selected.id === u.id });
  }
  view.setRoutes(out);
  const camp = selected?.kind === 'unit' ? v.units[selected.id] : null;
  if (camp && camp.cut_at) {
    // Cut by held ground: the line it would have, drawn to the tile where it stops, marked.
    const stop = camp.cut_at.path.indexOf(camp.cut_at.at);
    view.setLine({ tiles: [camp.tile, ...camp.cut_at.path.slice(0, stop + 1)], colour: lineColour(camp), cut: camp.cut_at.at });
  } else {
    view.setLine(camp && camp.line && camp.line.path.length ? { tiles: [camp.tile, ...camp.line.path], colour: lineColour(camp) } : null);
  }
}

// ---- painting a zone ---------------------------------------------------------------------------

/** 'paint', 'unpaint' or null: what a left press and drag on the map do to the selected camp's zone. */
let painting = null;

function setPainting(mode) {
  const u = selected?.kind === 'unit' ? v?.units[selected.id] : null;
  painting = u && u.civ === v.player ? mode : null;
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
  order({ type: 'zone', tiles: [...zone].filter((t) => t !== u.tile).sort((a, b) => a - b) });
}

/** A camp's line home, coloured by how much of its labour the line takes to feed it: green where a
 *  turn's food is cheap to carry, amber halfway, red when it is cut off. The thresholds are the
 *  page's to tune. */
function lineColour(u) {
  const t = u.cut_off ? 1 : u.line.carry == null ? 0 : Math.min(1, (u.people * RULES.unit_food) / u.line.carry);
  // Green (120) at 0, amber (40) at a half, red (0) at 1.
  const hue = t < 0.5 ? 120 - (80 * t) / 0.5 : 40 - (40 * (t - 0.5)) / 0.5;
  return `hsl(${hue} 85% 55%)`;
}

/** The player's civ in the top bar: swatch, name, people, store, warriors; "Watching" when it leads none. */
function civBar(warriors) {
  const bar = $('civbar');
  bar.hidden = v.phase !== 'main';
  if (bar.hidden) return;
  const c = v.civs[v.player];
  $('civstats').hidden = !c;
  $('civsw').hidden = !c;
  $('civname').textContent = c ? c.name : 'Watching';
  if (!c) return;
  $('civsw').style.background = c.color;
  const total = c.store.reduce((a, b) => a + b, 0);
  $('civpeople').textContent = Math.round(c.people);
  $('civinunits').textContent = '';
  $('civstore').textContent = `${Math.round(total)} / ${RULES.store_cap}`;
  const lines = GOODS.map((g, f) => [g, c.store[f]]).filter(([, x]) => x > 0.005).map(([g, x]) => `${g} ${f2(x)}`);
  $('st-store').title = `${lines.join(', ') || 'empty'}${c.lost > 0.005 ? `; ${f2(c.lost)} lost to the cap last turn` : ''}`;
  $('civunits').textContent = Math.round(warriors);
}

/** The pick: a card over the map while the phase is `choose`, built once per game. */
function pickCard() {
  const el = $('pick');
  el.hidden = v.phase !== 'choose';
  if (el.hidden) {
    pickFor = null;
    return;
  }
  if (pickFor === sim) return;
  pickFor = sim;
  $('legend').open = false;
  const cards = JSON.parse(sim.pick());
  const rows = cards.map((c) => {
    const terrain = c.terrain.map(([name, share]) => `${name} ${Math.round(share * 100)}%`).join(', ');
    const rival = c.rival_steps == null ? 'no rival' : `${c.rival_steps} steps to the nearest rival`;
    return `<div class="card" data-civ="${c.civ}"><div class="head"><i class="sw" style="background:${c.color}"></i><b>${c.name}</b> <span class="dim">capital at ${c.q}, ${c.r}</span></div>
<div>${c.clans} clan${c.clans === 1 ? '' : 's'} · ${Math.round(c.people * 10) / 10} people · ${c.hearths} hearth${c.hearths === 1 ? '' : 's'}</div>
<div class="dim">${terrain || 'no land'}</div>
<div class="dim">${rival}${c.rival_by_land ? '' : ' · no land route to a rival'}</div>
<div class="buttons"><button type="button" data-lead="${c.civ}">Lead ${c.name}</button><button type="button" data-show="${c.civ}">Show</button></div></div>`;
  });
  el.innerHTML = `<h2>Choose your civ</h2>${rows.join('')}<div class="buttons"><button type="button" data-watch="1">Watch all three</button></div>`;
}

$('pick').addEventListener('click', (e) => {
  const b = e.target.closest('button');
  if (!b) return;
  if (b.dataset.lead) chooseCiv(Number(b.dataset.lead));
  else if (b.dataset.show) showCiv(Number(b.dataset.show));
  else if (b.dataset.watch) chooseCiv(null);
});

/** "Show": centres the map on a civ's capital and highlights its land. */
function showCiv(civ) {
  const c = v.civs[civ];
  if (!c) return;
  const tiles = [];
  for (let i = 0; i < layout.q.length; i++) if (sim.tile_civ(i) === civ) tiles.push(i);
  view.setHighlight({ tiles, tile: c.capital });
  view.centreOn(c.capital, { minK: 12, smooth: true });
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

/** Gives the engine an order; shows why not in #error. Returns whether it went through. */
function order(action) {
  let ok = true;
  try {
    sim.act(JSON.stringify(action));
    $('error').textContent = '';
  } catch (e) {
    $('error').textContent = String(e);
    ok = false;
  }
  refresh();
  return ok;
}

function pullEvents() {
  const ev = JSON.parse(sim.events(cursor));
  cursor += ev.length;
  if (ev.length) feed.add(ev);
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
  turns(1);
  refresh();
}

function setPlaying(on) {
  playing = on;
  $('play').textContent = on ? 'Pause' : 'Play';
  $('play').setAttribute('aria-pressed', String(on));
  if (on) {
    last = performance.now();
    acc = 0;
    if (!loopTimer) loopTimer = setTimeout(loop, 0);
  }
}

function stop() {
  setPlaying(false);
  running = false;
  $('run').textContent = 'Run to end';
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
  if (!ok || v.phase === 'choose') {
    stop();
    return;
  }
  loopTimer = setTimeout(loop, running || speed === 'max' ? 0 : 16);
}

function runToEnd() {
  if (running) {
    stop();
    return;
  }
  if (v.phase !== 'pregame') return;
  stop();
  running = true;
  $('run').textContent = 'Stop';
  loopTimer = setTimeout(loop, 0);
}

// ---- selecting and the selection panel -------------------------------------------------------

function select(sel) {
  if (painting && !(sel?.kind === 'unit' && selected?.kind === 'unit' && sel.id === selected.id)) {
    painting = null;
    view.setPainting(false);
    document.body.dataset.painting = '';
  }
  selected = sel;
  feed.select(null);
  syncSelection();
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

/** A right-click: the selected unit of the player's goes to the tile. */
function onCommand(i) {
  const u = selected && selected.kind === 'unit' ? v.units[selected.id] : null;
  if (i < 0 || !u || u.civ !== v.player) return;
  order({ type: 'moveCamp', to: i });
}

function onFeed(item) {
  const alive = item.unit != null && item.unit in v.units;
  if (alive) selected = { kind: 'unit', id: item.unit };
  else if (item.kind === 'civFounded' && item.civ != null && item.civ in v.civs) selected = { kind: 'capital', civ: item.civ };
  else selected = { kind: 'tile', tile: item.tile, clan: item.clan, market: item.market };
  syncSelection();
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
      const json = JSON.stringify(a.action);
      const why = sim.check(json);
      if (!row || a.row === undefined || a.row !== rowKey) {
        row = document.createElement('div');
        rowKey = a.row;
        box.append(row);
      }
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = a.label;
      b.disabled = why !== '';
      b.onclick = () => order(a.action);
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

const isHearth = (i) => {
  const tame = v.tame.find(([t]) => t === i);
  return layout.wild_grain[i] > 0 && tame && tame[1] >= RULES.tame_turns;
};

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
  if (c.home !== i) return `Clan ${id}'s territory`;
  if (c.market == null) return `Clan ${id}'s home, no festival in reach`;
  const here = c.routes.length === 0 || c.routes[0].path.length === 0;
  return `Clan ${id}'s home, ${here ? `its festival ${c.market} is here` : `walks to festival ${c.market}`}`;
}

/** A worn tile's foods as shares, each with the cap it would have unworn. */
function wearTip(i) {
  const w = v.wear.find(([t]) => t === i);
  if (!w) return [];
  const [, worn, unworn] = w;
  return GOODS.flatMap((g, f) => {
    const pct = Math.round(100 * worn[f]);
    return pct > 0 ? [`${g} worn ${pct}%, unworn cap ${f2(unworn[f])}`] : [];
  });
}

function fillTip(i) {
  tipTile = i;
  const h = view.clanAt(i);
  const lines = [`<b>${tileKind(ctx(), i)}</b>`];
  if (layout.terrain[i] !== 0 && TERRAIN[layout.terrain[i]].land) {
    lines.push(h ? clanTip(i, h) : 'Held by nobody');
    lines.push(`Fertility ${f2(layout.fertility[i])}`);
    lines.push(...wearTip(i));
  }
  const civ = sim.tile_civ(i);
  if (civ >= 0 && v.civs[civ]) {
    const c = v.civs[civ];
    const name = `<i class="sw" style="background:${c.color}"></i>${c.name}`;
    if (c.capital === i) lines.push(`${name}'s capital`);
    else if (c.territory.includes(i)) lines.push(`${name}'s land <span class="dim">(capital territory)</span>`);
    else if (h) lines.push(`${name}, clan ${h}'s territory`);
    else lines.push(`${name}'s land`);
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

// ---- the legend ------------------------------------------------------------------------------

function legend() {
  const sw = (css) => `<i style="${css}"></i>`;
  const items = [
    [sw('background:#1f4f7a'), 'ocean'],
    [sw('background:#3f8fc4'), 'lake'],
    [sw('background:#d6c487'), 'coast'],
    [sw('background:#9cbf5a'), 'plains'],
    [sw('background:#3f7a3a'), 'forest'],
    [sw('background:#8a7a5c'), 'hills'],
    [sw('background:#7c7a85;color:#c4c2cc;text-align:center;font-size:10px;line-height:12px').replace('></i>', '>▲</i>'), 'mountain'],
    [sw('background:#efe3b8 radial-gradient(#b39a55 1px,transparent 1.5px) 0 0/5px 5px'), 'desert'],
    [sw('background:#5f8a6a repeating-linear-gradient(90deg,#2f5a46 0 1px,transparent 1px 5px)'), 'marsh (delta)'],
    [sw('background:linear-gradient(#9cbf5a,#9cbf5a);border:0;height:4px;margin:5px 0;background:#58b8f2;border-radius:2px'), 'river'],
    [sw('background:rgba(96,235,70,.6);border:1px solid #1c6e18'), 'floodplain'],
    [sw('background:transparent;border:2px solid #f0c030'), 'valley'],
    [sw('background:transparent;border:2px dashed #b0583a'), 'sealed valley'],
    [sw('background:transparent;border:2px solid #b070e0'), 'neck'],
    [sw('background:#fff2b0;border-radius:50%;width:10px;height:10px;margin:2px 4px;border-color:#3a2c00'), 'pass'],
    [sw('background:hsla(200,70%,55%,.35);border:1px solid hsl(200,70%,55%)'), 'territory'],
    [sw('background:rgba(26,15,7,.6)'), 'worn land'],
    [sw('background:transparent;border:0;border-top:3px dashed hsl(120,70%,55%);height:0;margin:5px 0'), 'festival route'],
    [sw('background:#f2dca4;border:0;height:3px;margin:5px 0'), 'trail'],
    [sw('background:#ffd400;width:9px;height:9px;margin:2px 4px;border-color:#3a2c00'), 'hearth'],
    [sw('background:transparent;border:2px solid #fff;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'market'],
    [sw('background:transparent;border:3px solid #ffc800;border-radius:50%;width:14px;height:14px;margin:0 2px'), 'centre'],
    [sw('background:hsl(120,70%,55%);border:2px solid #ff2a2a;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'clan (red ring: starving)'],
    [sw('background:hsl(120,70%,55%);border:3px solid #d6a63b;border-radius:50%;width:10px;height:10px;margin:2px 3px;box-shadow:0 0 0 1px #000'), 'civ clan (ring in the civ colour)'],
    [sw('background:#d64545;border:2px solid #fff;outline:2px solid #000;width:10px;height:10px;margin:2px 4px;clip-path:polygon(0 25%,50% 0,100% 25%,100% 100%,0 100%)'), 'capital'],
    [sw('background:transparent;border:3px solid #3b6fd6;box-shadow:0 0 0 1px #000;width:12px;height:12px;margin:1px 3px'), 'civ land (capital territory)'],
    [sw('background:#3b6fd6;border:2px solid #000;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'warrior'],
    [sw('background:#3b6fd6;border:2px solid #ff2a2a;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'hungry warrior'],
    [sw('background:transparent;border:0;border-top:3px dashed #fff;height:0;margin:5px 0'), 'route (selected unit)'],
    [sw('background:hsl(120,85%,55%);height:3px;margin:5px 0;border:0'), 'camp line (green: cheap to feed, red: cut off)'],
  ];
  $('legenditems').innerHTML = items.map(([s, name]) => `<span class="item">${s}${name}</span>`).join('');
}

// ---- controls --------------------------------------------------------------------------------

$('end').onclick = endTurn;
$('play').onclick = () => {
  if (playing) stop();
  else {
    running = false;
    $('run').textContent = 'Run to end';
    setPlaying(true);
  }
};
$('speed').onchange = () => {
  last = performance.now();
  acc = 0;
};
$('run').onclick = runToEnd;
$('new').onclick = newMap;
$('dice').onclick = () => {
  $('seed').value = Math.floor(Math.random() * 100000);
  newMap();
};
$('seed').addEventListener('keydown', (e) => {
  if (e.key === 'Enter') newMap();
});
for (const key of ['fertility', 'territory', 'wear', 'routes', 'markets', 'hidden', 'civs']) {
  $(`v-${key}`).onchange = (e) => view.setOptions({ [key]: e.target.checked });
}
$('zin').onclick = () => view.zoomBy(1.4);
$('zout').onclick = () => view.zoomBy(1 / 1.4);
$('fit').onclick = () => view.fit();

function showFeed(on) {
  $('side').classList.toggle('collapsed', !on);
  $('feedtoggle').setAttribute('aria-pressed', String(on));
  $('showfeed').hidden = on;
  // The map box changes size; the observer resizes the canvas.
}
$('feedtoggle').onclick = () => showFeed($('side').classList.contains('collapsed'));
$('hidefeed').onclick = () => showFeed(false);
$('showfeed').onclick = () => showFeed(true);

window.addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  if (e.target.matches('input, select, textarea')) return;
  const step = 60;
  switch (e.key) {
    case 'ArrowLeft': view.panBy(step, 0); break;
    case 'ArrowRight': view.panBy(-step, 0); break;
    case 'ArrowUp': view.panBy(0, step); break;
    case 'ArrowDown': view.panBy(0, -step); break;
    case '+': case '=': view.zoomBy(1.4); break;
    case '-': case '_': view.zoomBy(1 / 1.4); break;
    case '0': view.fit(); break;
    case 'Escape': if (painting) setPainting(null); else select(null); break;
    case 'p': case 'P': setPainting(painting === 'paint' ? null : 'paint'); break;
    case 'u': case 'U': setPainting(painting === 'unpaint' ? null : 'unpaint'); break;
    default: return;
  }
  e.preventDefault();
});
document.addEventListener('pointerdown', (e) => {
  for (const m of document.querySelectorAll('details.menu[open]')) if (!m.contains(e.target)) m.open = false;
});

$('save').onclick = () => {
  const a = document.createElement('a');
  a.href = URL.createObjectURL(new Blob([sim.save()], { type: 'application/json' }));
  a.download = `sansfiat-seed${$('seed').value}-turn${v.turn}.json`;
  a.click();
  URL.revokeObjectURL(a.href);
};
$('loadButton').onclick = () => $('load').click();
$('load').onchange = async (e) => {
  const file = e.target.files[0];
  if (!file) return;
  try {
    const loaded = Sim.load(await file.text());
    // The feed is rebuilt from the replay: start() reads events(0).
    start(loaded, JSON.parse(loaded.save()).config.seed);
  } catch (err) {
    $('error').textContent = String(err);
  }
  e.target.value = '';
};

// ---- go --------------------------------------------------------------------------------------

legend();
{
  const size = SIZES.find((s) => s.name === params.get('size')) || SIZES[0];
  const seed = Math.max(0, Math.floor(Number(params.get('seed')) || 1));
  sizeSelect.value = size.name;
  $('seed').value = seed;
  const sp = params.get('speed');
  if (sp && ['slow', 'normal', 'fast', 'max'].includes(sp)) $('speed').value = sp;
  start(new Sim(seed, size.radius), seed);
  if (params.get('run') === '1') runToEnd();
  else if (params.get('play') === '1') setPlaying(true);
}
window.game = {
  view: () => v,
  layout: () => layout,
  tileCiv: (i) => sim.tile_civ(i),
  save: () => JSON.parse(sim.save()),
  map: view,
  feed,
};
