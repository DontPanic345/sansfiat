import init, { Sim, goods, terrain } from './pkg/web.js';

await init();
const GOODS = goods();
const TERRAIN = JSON.parse(terrain());
const GOOD_COLORS = ['#e8c547', '#5ab0e8', '#d0743c', '#b36ad6'];
const MARKET_COLORS = ['#ff6b6b', '#4ecdc4', '#ffd93d', '#a29bfe', '#fd79a8', '#55efc4'];

const $ = (id) => document.getElementById(id);
const canvas = $('map');
const ctx = canvas.getContext('2d');
let sim, s, selected = null, timer = null;

function reset() {
  const num = (id) => Number($(id).value);
  sim = new Sim(num('seed'), 8, num('clans'), num('markets'), num('reach'));
  selected = null;
  refresh();
}

function refresh() {
  s = JSON.parse(sim.state());
  draw();
  panel();
}

const size = () => Math.min(canvas.width / (Math.sqrt(3) * (2 * s.config.radius + 1)), canvas.height / (3 * s.config.radius + 2));
const at = (t) => ({
  x: canvas.width / 2 + size() * Math.sqrt(3) * (t.q + t.r / 2),
  y: canvas.height / 2 + size() * 1.5 * t.r,
});
const dist = (a, b) => (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;

function hexPath(c, r) {
  ctx.beginPath();
  for (let i = 0; i < 6; i++) {
    const a = (Math.PI / 3) * i - Math.PI / 6;
    ctx.lineTo(c.x + r * Math.cos(a), c.y + r * Math.sin(a));
  }
  ctx.closePath();
}

function draw() {
  const r = size();
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (const t of s.tiles) {
    hexPath(at(t), r);
    ctx.fillStyle = TERRAIN[t.terrain].color;
    ctx.fill();
    ctx.strokeStyle = 'rgba(0,0,0,0.25)';
    ctx.stroke();
  }
  for (const m of s.markets) {
    const home = s.tiles[m.tile];
    for (const t of s.tiles) {
      if (dist(t, home) !== s.config.reach) continue;
      hexPath(at(t), r * 0.5);
      ctx.fillStyle = MARKET_COLORS[m.id % MARKET_COLORS.length] + '55';
      ctx.fill();
    }
  }
  for (const c of s.clans) {
    const p = at(s.tiles[c.tile]);
    ctx.beginPath();
    ctx.arc(p.x, p.y, r * (0.2 + c.people / 40), 0, Math.PI * 2);
    ctx.fillStyle = c.market == null ? '#666' : MARKET_COLORS[c.market % MARKET_COLORS.length];
    ctx.fill();
    ctx.lineWidth = selected?.clan === c.id ? 3 : 1;
    ctx.strokeStyle = selected?.clan === c.id ? '#fff' : '#111';
    ctx.stroke();
  }
  ctx.lineWidth = 2;
  for (const m of s.markets) {
    const p = at(s.tiles[m.tile]);
    const w = r * 0.9;
    ctx.fillStyle = MARKET_COLORS[m.id % MARKET_COLORS.length];
    ctx.strokeStyle = selected?.market === m.id ? '#fff' : '#111';
    ctx.fillRect(p.x - w / 2, p.y - w / 2, w, w);
    ctx.strokeRect(p.x - w / 2, p.y - w / 2, w, w);
  }
  ctx.lineWidth = 1;
}

const fmt = (v) => v.toFixed(2);
const goodHeads = () => GOODS.map((g, i) => `<th style="color:${GOOD_COLORS[i]}">${g}</th>`).join('');
const cells = (b) => b.map((v) => `<td>${fmt(v)}</td>`).join('');
const utility = (b) => b.reduce((sum, v) => sum + Math.sqrt(Math.max(0, v)), 0);

function panel() {
  const last = s.history.at(-1);
  const gain = last ? `${(100 * (last.traded - last.autarky) / last.autarky).toFixed(1)}%` : '—';
  let html = `<p>Turn ${s.turn} · welfare gain from trade ${gain}</p>
    <table><tr><th>market</th>${goodHeads()}<th>traders</th></tr>
    ${s.markets.map((m) => `<tr><td style="color:${MARKET_COLORS[m.id % MARKET_COLORS.length]}">${m.id}</td>${cells(m.prices)}<td>${m.cleared ? m.traders : '–'}</td></tr>`).join('')}
    </table><p class="dim">Prices are exchange ratios (geometric mean 1). Click a market or clan.</p>`;
  if (selected?.market != null) {
    const m = s.markets[selected.market];
    html += `<h1>Market ${m.id}</h1><canvas id="chart"></canvas>
      <table><tr><th>volume</th>${goodHeads()}</tr><tr><td></td>${cells(m.volume)}</tr></table>`;
  }
  if (selected?.clan != null) {
    const c = s.clans[selected.clan];
    html += `<h1>Clan ${c.id}</h1><p class="dim">${c.people} people on ${TERRAIN[s.tiles[c.tile].terrain].name},
      ${c.market == null ? 'no market in reach' : `trades at market ${c.market}`}</p>
      <table><tr><th></th>${goodHeads()}<th>utility</th></tr>
      <tr><td>harvest</td>${cells(c.harvest)}<td>${fmt(utility(c.harvest))}</td></tr>
      <tr><td>after trade</td>${cells(c.basket)}<td>${fmt(utility(c.basket))}</td></tr></table>`;
  }
  $('info').innerHTML = html;
  if (selected?.market != null) chart(selected.market);
}

function chart(id) {
  const el = $('chart');
  el.width = el.clientWidth * devicePixelRatio;
  el.height = el.clientHeight * devicePixelRatio;
  const c = el.getContext('2d');
  const series = s.history.map((t) => t.prices[id].map(Math.log));
  if (series.length < 2) return;
  const all = series.flat();
  const lo = Math.min(...all), hi = Math.max(...all);
  const x = (i) => (i / (series.length - 1)) * el.width;
  const y = (v) => el.height - ((v - lo) / (hi - lo || 1)) * (el.height - 8) - 4;
  c.lineWidth = devicePixelRatio;
  GOODS.forEach((_, g) => {
    c.beginPath();
    series.forEach((p, i) => c.lineTo(x(i), y(p[g])));
    c.strokeStyle = GOOD_COLORS[g];
    c.stroke();
  });
}

canvas.addEventListener('click', (e) => {
  const rect = canvas.getBoundingClientRect();
  const px = ((e.clientX - rect.left) / rect.width) * canvas.width;
  const py = ((e.clientY - rect.top) / rect.height) * canvas.height;
  const near = (t) => Math.hypot(at(t).x - px, at(t).y - py) < size() * 0.9;
  const m = s.markets.find((m) => near(s.tiles[m.tile]));
  const c = s.clans.find((c) => near(s.tiles[c.tile]));
  selected = m ? { market: m.id } : c ? { clan: c.id } : null;
  draw();
  panel();
});

$('reset').onclick = reset;
$('step').onclick = () => { sim.step(1); refresh(); };
$('ten').onclick = () => { sim.step(10); refresh(); };
$('play').onclick = () => {
  if (timer) { clearInterval(timer); timer = null; $('play').textContent = 'Play'; return; }
  timer = setInterval(() => { sim.step(1); refresh(); }, 200);
  $('play').textContent = 'Pause';
};

reset();
window.sim = () => s;
