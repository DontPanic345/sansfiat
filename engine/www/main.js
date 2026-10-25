import init, { Sim, terrain, goods, rules, sizes } from './pkg/web.js';
import { MapView, hue } from './map-view.js';
import { Feed } from './feed.js';

await init();
const TERRAIN = JSON.parse(terrain());
const GOODS = JSON.parse(goods());
const RULES = JSON.parse(rules());
const SIZES = JSON.parse(sizes());
const ZONES = ['', 'valley', 'sealed valley', 'neck'];
const RATES = { slow: 1, normal: 4, fast: 20 };

const $ = (id) => document.getElementById(id);
const f2 = (x) => (Math.round(x * 100) / 100).toString();
const params = new URLSearchParams(location.search);

let sim = null;
let layout = null;
let v = null; // the last Sim.view()
let cursor = 0; // into Sim.events
let selected = null; // {tile, clan, market}
let playing = false;
let running = false;
let loopTimer = null;
let last = 0;
let acc = 0;

// ---- map, feed -------------------------------------------------------------------------------

const view = new MapView($('map'), {
  terrain: TERRAIN,
  rules: RULES,
  onPick: (i) => select(i < 0 ? null : { tile: i, clan: null, market: null }),
  onHover: showTip,
  onHoverEnd: hideTip,
});
const feed = new Feed({ list: $('feed'), filters: $('filters'), onClick: onFeed });
new ResizeObserver(() => view.resize()).observe($('mapbox'));

// ---- starting a map --------------------------------------------------------------------------

const sizeSelect = $('size');
for (const s of SIZES) sizeSelect.add(new Option(`${s.name} (radius ${s.radius})`, s.name));

function start(next, seedShown) {
  stop();
  sim = next;
  layout = JSON.parse(sim.layout());
  selected = null;
  view.setLayout(layout);
  view.setHighlight(null);
  view.setSelection(-1);
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
  if (v.end) {
    $('phasetext').textContent = 'Pre-game over';
    const up = v.end.topped_up.length;
    $('phasenote').textContent = `turn ${v.end.turn}, by ${v.end.by_limit ? 'the limit' : 'centres'}${up ? `, +${up} topped up` : ''}`;
  } else {
    $('phasetext').textContent = 'Pre-game';
    $('phasenote').textContent = 'Phase';
  }
  document.body.dataset.turn = v.turn;
  document.body.dataset.phase = v.end ? 'ended' : 'pregame';
  document.body.dataset.centres = centres;
  pullEvents();
  panel();
  view.drawNow();
}

function pullEvents() {
  const ev = JSON.parse(sim.events(cursor));
  cursor += ev.length;
  if (ev.length) feed.add(ev);
}

/** Ends `n` turns; shows why if refused. Returns whether it all went through. */
function turns(n) {
  try {
    for (let i = 0; i < n; i++) sim.act('{"type":"endTurn"}');
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

/** The play loop: as many turns as the speed asks for, one refresh per pass; stops when the pre-game ends. */
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
    do ok = turns(1); while (ok && performance.now() < until && !sim.ended());
  } else {
    acc = Math.min(acc + ((now - last) / 1000) * RATES[speed], 4);
    last = now;
    const n = Math.floor(acc);
    acc -= n;
    if (n > 0) ok = turns(n);
  }
  if (sim !== mine) return;
  refresh();
  if (!ok || (v.end && (running || playing))) {
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
  if (v.end) return;
  stop();
  running = true;
  $('run').textContent = 'Stop';
  loopTimer = setTimeout(loop, 0);
}

// ---- selecting and the detail panel ----------------------------------------------------------

function select(sel) {
  selected = sel;
  view.setSelection(sel ? sel.tile : -1);
  view.setHighlight(sel && (sel.clan != null || sel.market != null) ? { clan: sel.clan, market: sel.market } : null);
  feed.select(null);
  panel();
}

function onFeed(item) {
  selected = { tile: item.tile, clan: item.clan, market: item.market };
  view.setSelection(item.tile);
  view.setHighlight(item.clan != null || item.market != null ? { clan: item.clan, market: item.market } : null);
  view.centreOn(item.tile, { minK: 24, smooth: true });
  panel();
}

function terrainLabel(i) {
  const l = layout;
  const t = TERRAIN[l.terrain[i]];
  if (l.terrain[i] === 0) return l.lake[i] ? 'lake' : 'ocean';
  if (t.name === 'marsh') return 'marsh (delta)';
  return t.name;
}

/** The terrain name and what's on it: river, floodplain, pass, zone. */
function tileKind(i) {
  const l = layout;
  const bits = [terrainLabel(i)];
  if (l.river[i] > 0) bits.push(`river (${l.river[i]} upstream)`);
  if (l.flood[i]) bits.push('floodplain');
  if (l.pass[i]) bits.push('pass');
  if (l.zone[i]) bits.push(ZONES[l.zone[i]]);
  return bits.join(', ');
}

const marketOn = (i) => Object.values(v.markets).find((m) => m.tile === i);
const isHearth = (i) => {
  const tame = v.tame.find(([t]) => t === i);
  return layout.wild_grain[i] > 0 && tame && tame[1] >= RULES.tame_turns;
};

function clanHtml(c, how) {
  const m = c.market != null ? `market ${c.market}, trip ${f2(c.trip)}` : 'no market';
  return `<p><b style="color:${hue(c.id)}">Clan ${c.id}</b> <span class="dim">(${how})</span><br>${f2(c.pop)} people · fed ${f2(c.fed_ratio)}${c.starving ? ' · <b style="color:#ff7b72">starving</b>' : ''}<br>store ${f2(c.store.reduce((a, b) => a + b, 0))} · round ${c.round.length} tiles, walk ${f2(c.walk)} · ${m}</p>`;
}

function marketHtml(m, how) {
  const prices = GOODS.map((g, f) => `${g} ${f2(m.prices[f])}`).join(', ');
  return `<p><b>Market ${m.id}</b> <span class="dim">(${how})</span>${v.end && v.end.centres.includes(m.id) ? ' · <b style="color:#ffc800">centre</b>' : ''}<br>age ${v.turn - m.founded} · ${m.traders} traders · activity ${f2(m.activity)}<br><span class="dim">prices ${prices}</span></p>`;
}

function panel() {
  const el = $('detail');
  if (!selected) {
    el.innerHTML = '';
    return;
  }
  const i = selected.tile;
  const t = JSON.parse(sim.tile(i));
  const land = TERRAIN[t.terrain].land;
  let html = `<h2><span>Tile ${t.q}, ${t.r}</span><button type="button" id="closedetail" aria-label="Close details">×</button></h2>`;
  html += `<p class="dim">${tileKind(i)}${land ? ` · fertility ${f2(t.fertility)} · trail ${t.trail ? 'yes' : 'no'}` : ''}</p>`;
  if (land) {
    html += '<table>';
    GOODS.forEach((g, f) => {
      if (t.cap[f] > 0) html += `<tr><td>${g}</td><td>${f2(t.stock[f])} / ${f2(t.cap[f])}</td></tr>`;
    });
    if (t.wild_grain > 0) html += `<tr><td>tame</td><td>${Math.round((100 * t.tame) / RULES.tame_turns)}%</td></tr>`;
    html += '</table>';
  }
  const shown = new Set();
  const addClan = (id, how) => {
    if (id == null || shown.has(id) || !v.clans[id]) return;
    shown.add(id);
    html += clanHtml(v.clans[id], how);
  };
  addClan(selected.clan, 'chosen');
  const h = view.holder(i);
  if (h) addClan(h[0], h[1] === 'round' ? 'on its round' : 'in its territory');
  const m = selected.market != null ? v.markets[selected.market] : marketOn(i);
  if (m) html += marketHtml(m, 'here');
  else if (selected.market != null) html += `<p class="dim">Market ${selected.market} is gone.</p>`;
  el.innerHTML = html;
  $('closedetail').onclick = () => select(null);
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

function fillTip(i) {
  tipTile = i;
  const h = view.holder(i);
  const lines = [`<b>${tileKind(i)}</b>`];
  if (layout.terrain[i] !== 0 && TERRAIN[layout.terrain[i]].land) {
    lines.push(h ? `Held by clan ${h[0]} <span class="dim">(${h[1]})</span>` : 'Held by nobody');
    lines.push(`Fertility ${f2(layout.fertility[i])}`);
  }
  const m = marketOn(i);
  if (m) lines.push(`Market ${m.id}${v.end && v.end.centres.includes(m.id) ? ' (centre)' : ''}`);
  if (isHearth(i)) lines.push('Hearth');
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
    [sw('background:transparent;border:0;border-top:2px solid hsl(40,70%,55%);height:0;margin:6px 0'), 'round'],
    [sw('background:#f2dca4;border:0;height:3px;margin:5px 0'), 'trail'],
    [sw('background:#ffd400;width:9px;height:9px;margin:2px 4px;border-color:#3a2c00'), 'hearth'],
    [sw('background:transparent;border:2px solid #fff;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'market'],
    [sw('background:transparent;border:3px solid #ffc800;border-radius:50%;width:14px;height:14px;margin:0 2px'), 'centre'],
    [sw('background:hsl(120,70%,55%);border:2px solid #ff2a2a;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'clan (red ring: starving)'],
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
for (const key of ['fertility', 'territory', 'rounds', 'markets', 'hidden']) {
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
    case 'Escape': select(null); break;
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
  act: (a) => { sim.act(JSON.stringify(a)); refresh(); },
  save: () => JSON.parse(sim.save()),
  map: view,
  feed,
};
