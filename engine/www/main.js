import init, { Sim, terrain, goods, rules } from './pkg/web.js';

await init();
const TERRAIN = JSON.parse(terrain());
const GOODS = JSON.parse(goods());
const RULES = JSON.parse(rules());

const $ = (id) => document.getElementById(id);
const canvas = $('map');
const ctx = canvas.getContext('2d');
let sim, s, sum, selected = null, timer = null, running = false;
let index = new Map(), territory = new Map();

function start(next) {
  stop();
  sim = next;
  selected = null;
  refresh();
}

function stop() {
  if (timer) { clearInterval(timer); timer = null; $('play').textContent = 'Play'; }
  running = false;
}

function refresh() {
  s = JSON.parse(sim.state());
  sum = JSON.parse(sim.summary());
  if (index.size !== s.tiles.length) index = new Map(s.tiles.map((t, i) => [`${t.q},${t.r}`, i]));
  territory = new Map();
  for (const c of Object.values(s.clans)) {
    for (const i of terr(c)) {
      if (!territory.has(i)) territory.set(i, c.id);
    }
  }
  draw();
  panel();
}

const DIRS = [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]];
const land = (i) => TERRAIN[s.tiles[i].terrain].land;

/** The tiles of a clan's territory: its round and their land neighbours. */
function terr(c) {
  const out = new Set();
  for (const i of c.round) {
    out.add(i);
    const t = s.tiles[i];
    for (const [dq, dr] of DIRS) {
      const j = index.get(`${t.q + dq},${t.r + dr}`);
      if (j !== undefined && land(j)) out.add(j);
    }
  }
  return out;
}

/** Applies an action; shows why if it's refused. */
function act(action) {
  try {
    sim.act(JSON.stringify(action));
    $('error').textContent = '';
  } catch (e) {
    $('error').textContent = String(e);
  }
  refresh();
}

const size = () => Math.min(canvas.width / (Math.sqrt(3) * (2 * s.config.radius + 1)), canvas.height / (3 * s.config.radius + 2));
const at = (t) => ({
  x: canvas.width / 2 + size() * Math.sqrt(3) * (t.q + t.r / 2),
  y: canvas.height / 2 + size() * 1.5 * t.r,
});

function hexPath(c, r) {
  ctx.beginPath();
  for (let i = 0; i < 6; i++) {
    const a = (Math.PI / 3) * i - Math.PI / 6;
    ctx.lineTo(c.x + r * Math.cos(a), c.y + r * Math.sin(a));
  }
  ctx.closePath();
}

/** A colour from a clan's id. */
const hue = (id) => `hsl(${(id * 137.508) % 360} 70% 55%)`;

/** Hex steps between two tiles. */
function steps(a, b) {
  return (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;
}

/** Tile indices within a market's reach, as the engine sees it, approximated by hex steps
 * over passable land out to the reach (move cost 1 on plains and coast). */
function reachOf(m) {
  const out = [];
  const from = s.tiles[m.tile];
  s.tiles.forEach((t, i) => {
    if (land(i) && steps(from, t) <= 3) out.push(i);
  });
  return out;
}

const fertilityColor = (f) => {
  const y = Math.max(0, Math.min(1, (f - 0.4) / 1.6));
  return `hsl(${30 + 90 * y} 60% ${25 + 30 * y}%)`;
};

function draw() {
  const r = size();
  const showFertility = $('fertility').checked;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  s.tiles.forEach((t, i) => {
    hexPath(at(t), r);
    ctx.fillStyle = showFertility && TERRAIN[t.terrain].land ? fertilityColor(t.fertility) : TERRAIN[t.terrain].color;
    ctx.fill();
    ctx.lineWidth = selected === i ? 3 : 1;
    ctx.strokeStyle = selected === i ? '#fff' : 'rgba(0,0,0,0.25)';
    ctx.stroke();
  });
  ctx.lineWidth = 1;

  // Territory: a faint fill in the clan's colour.
  ctx.globalAlpha = 0.22;
  for (const [i, id] of territory) {
    hexPath(at(s.tiles[i]), r);
    ctx.fillStyle = hue(id);
    ctx.fill();
  }
  ctx.globalAlpha = 1;

  // Trails: a short tan line across the tile; hearths: a small yellow mark.
  s.tiles.forEach((t) => {
    const c = at(t);
    if (t.trail) {
      ctx.strokeStyle = '#d2b48c';
      ctx.lineWidth = Math.max(1, r * 0.15);
      ctx.beginPath();
      ctx.moveTo(c.x - r * 0.4, c.y + r * 0.2);
      ctx.lineTo(c.x + r * 0.4, c.y - r * 0.2);
      ctx.stroke();
    }
    if (t.wild_grain > 0 && t.tame >= RULES.tame_turns) {
      ctx.fillStyle = '#ffd400';
      ctx.fillRect(c.x - r * 0.18, c.y - r * 0.18, r * 0.36, r * 0.36);
    }
  });

  // Rounds: a closed line through the tiles' centres in order, back to home.
  ctx.lineWidth = Math.max(1, r * 0.12);
  for (const c of Object.values(s.clans)) {
    if (c.round.length < 2) continue;
    ctx.strokeStyle = hue(c.id);
    ctx.beginPath();
    c.round.forEach((i, k) => {
      const p = at(s.tiles[i]);
      if (k === 0) ctx.moveTo(p.x, p.y); else ctx.lineTo(p.x, p.y);
    });
    ctx.closePath();
    ctx.stroke();
  }

  // Markets: a ring with the reach outlined; centres picked out.
  const centres = new Set(s.end ? s.end.centres : sum.centres);
  for (const m of Object.values(s.markets)) {
    const p = at(s.tiles[m.tile]);
    ctx.strokeStyle = 'rgba(255,255,255,0.35)';
    ctx.lineWidth = 1;
    for (const i of reachOf(m)) {
      hexPath(at(s.tiles[i]), r);
      ctx.stroke();
    }
    ctx.strokeStyle = '#fff';
    ctx.lineWidth = Math.max(1.5, r * 0.15);
    ctx.beginPath();
    ctx.arc(p.x, p.y, r * 0.9, 0, 2 * Math.PI);
    ctx.stroke();
  }
  for (const id of centres) {
    const m = s.markets[id];
    if (!m) continue;
    const p = at(s.tiles[m.tile]);
    ctx.strokeStyle = '#ffc800';
    ctx.lineWidth = Math.max(3, r * 0.35);
    ctx.beginPath();
    ctx.arc(p.x, p.y, r * 1.5, 0, 2 * Math.PI);
    ctx.stroke();
  }

  // Clans: a circle at home, its area by people; red outline when starving.
  for (const c of Object.values(s.clans)) {
    const p = at(s.tiles[c.home]);
    ctx.beginPath();
    ctx.arc(p.x, p.y, Math.max(2, r * 0.22 * Math.sqrt(c.pop)), 0, 2 * Math.PI);
    ctx.fillStyle = hue(c.id);
    ctx.fill();
    ctx.lineWidth = c.starving ? 2.5 : 1;
    ctx.strokeStyle = c.starving ? '#ff2a2a' : 'rgba(0,0,0,0.6)';
    ctx.stroke();
  }
  ctx.lineWidth = 1;
}

const f2 = (x) => (Math.round(x * 100) / 100).toString();

function clanHtml(c, how) {
  const m = c.market != null ? `market ${c.market}, trip ${f2(c.trip)}` : 'no market';
  return `<p><b>Clan ${c.id}</b> <span class="dim">(${how})</span><br>${f2(c.pop)} people · fed ${f2(c.fed_ratio)}${c.starving ? ' · starving' : ''}<br>store ${f2(c.store.reduce((a, b) => a + b, 0))} · round ${c.round.length} tiles, walk ${f2(c.walk)} · ${m}</p>`;
}

function panel() {
  let html = `<p>Turn ${sum.turn} · ${s.log.length} actions</p>`;
  html += `<p class="dim">${sum.clans} clans, ${f2(sum.people)} people, fed ${f2(sum.mean_fed)}, ${sum.starving} starving · held ${sum.held}, territory ${sum.territory} · round ${f2(sum.round_len)} tiles costing ${f2(sum.round_cost)} · ${sum.trails} trails, ${sum.hearths} hearths · ${sum.markets} markets, ${sum.trading} clans trading · ${sum.centres.length} centres, cover ${f2(sum.cover)}</p>`;
  if (s.end) {
    html += `<p><b>Ended turn ${s.end.turn}</b> ${s.end.by_limit ? 'by the turn limit' : 'by its centres'}; ${s.end.centres.length} centres, cover ${f2(s.end.cover)}</p>`;
  }
  if (selected != null) {
    const t = s.tiles[selected];
    html += `<h1>Tile ${t.q}, ${t.r}</h1><p class="dim">${TERRAIN[t.terrain].name}, fertility ${f2(t.fertility)}, trail ${t.trail ? 'yes' : 'no'}</p>`;
    if (TERRAIN[t.terrain].land) {
      html += '<table>';
      GOODS.forEach((g, f) => {
        if (t.cap[f] > 0) html += `<tr><td>${g}</td><td>${f2(t.stock[f])} / ${f2(t.cap[f])}</td></tr>`;
      });
      if (t.wild_grain > 0) html += `<tr><td>tame</td><td>${Math.round((100 * t.tame) / RULES.tame_turns)}%</td></tr>`;
      html += '</table>';
    }
    const claim = t.claim != null ? s.clans[t.claim] : null;
    if (claim) html += clanHtml(claim, 'on its round');
    const terrId = territory.get(selected);
    if (terrId != null && terrId !== t.claim) html += clanHtml(s.clans[terrId], 'in its territory');
    const m = Object.values(s.markets).find((m) => m.tile === selected);
    if (m) {
      const prices = GOODS.map((g, f) => `${g} ${f2(m.prices[f])}`).join(', ');
      html += `<p><b>Market ${m.id}</b><br>age ${s.turn - m.founded} · ${m.traders} traders · activity ${f2(m.activity)}<br><span class="dim">prices ${prices}</span></p>`;
    }
  } else {
    html += '<p class="dim">Click a tile.</p>';
  }
  $('info').innerHTML = html;
}

canvas.addEventListener('click', (e) => {
  const rect = canvas.getBoundingClientRect();
  const px = ((e.clientX - rect.left) / rect.width) * canvas.width;
  const py = ((e.clientY - rect.top) / rect.height) * canvas.height;
  const i = s.tiles.findIndex((t) => Math.hypot(at(t).x - px, at(t).y - py) < size() * 0.9);
  selected = i < 0 ? null : i;
  draw();
  panel();
});

$('fertility').onchange = draw;
$('reset').onclick = () => start(new Sim(Number($('seed').value), Number($('radius').value)));
$('end').onclick = () => act({ type: 'endTurn' });
$('ten').onclick = () => { for (let i = 0; i < 10; i++) act({ type: 'endTurn' }); };
$('play').onclick = () => {
  if (timer) { stop(); return; }
  timer = setInterval(() => act({ type: 'endTurn' }), 200);
  $('play').textContent = 'Pause';
};

/** Runs turns in batches until the pre-game ends, redrawing between batches. */
$('run').onclick = async () => {
  if (running) { running = false; return; }
  stop();
  running = true;
  $('run').textContent = 'Stop';
  const mine = sim;
  while (running && sim === mine && !s.end) {
    try {
      for (let i = 0; i < 5; i++) sim.act('{"type":"endTurn"}');
    } catch (e) {
      $('error').textContent = String(e);
      break;
    }
    refresh();
    await new Promise((r) => setTimeout(r, 0));
  }
  running = false;
  $('run').textContent = 'Run to end';
};
$('save').onclick = () => {
  const a = document.createElement('a');
  a.href = URL.createObjectURL(new Blob([sim.save()], { type: 'application/json' }));
  a.download = `sansfiat-seed${s.config.seed}-turn${s.turn}.json`;
  a.click();
  URL.revokeObjectURL(a.href);
};
$('loadButton').onclick = () => $('load').click();
$('load').onchange = async (e) => {
  const file = e.target.files[0];
  if (!file) return;
  try {
    start(Sim.load(await file.text()));
    $('error').textContent = '';
  } catch (err) {
    $('error').textContent = String(err);
  }
  e.target.value = '';
};

$('reset').click();
window.game = { state: () => s, summary: () => sum, act, save: () => JSON.parse(sim.save()) };
