import init, { Sim, terrain } from './pkg/web.js';

await init();
const TERRAIN = JSON.parse(terrain());

const $ = (id) => document.getElementById(id);
const canvas = $('map');
const ctx = canvas.getContext('2d');
let sim, s, selected = null, timer = null;

function start(next) {
  sim = next;
  selected = null;
  refresh();
}

function refresh() {
  s = JSON.parse(sim.state());
  draw();
  panel();
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

function draw() {
  const r = size();
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  s.tiles.forEach((t, i) => {
    hexPath(at(t), r);
    ctx.fillStyle = TERRAIN[t.terrain].color;
    ctx.fill();
    ctx.lineWidth = selected === i ? 3 : 1;
    ctx.strokeStyle = selected === i ? '#fff' : 'rgba(0,0,0,0.25)';
    ctx.stroke();
  });
  ctx.lineWidth = 1;
}

function panel() {
  let html = `<p>Turn ${s.turn} · ${s.log.length} actions</p>`;
  if (selected != null) {
    const t = s.tiles[selected];
    html += `<h1>Tile ${t.q}, ${t.r}</h1><p class="dim">${TERRAIN[t.terrain].name}</p>`;
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

$('reset').onclick = () => start(new Sim(Number($('seed').value), Number($('radius').value)));
$('end').onclick = () => act({ type: 'endTurn' });
$('ten').onclick = () => { for (let i = 0; i < 10; i++) act({ type: 'endTurn' }); };
$('play').onclick = () => {
  if (timer) { clearInterval(timer); timer = null; $('play').textContent = 'Play'; return; }
  timer = setInterval(() => act({ type: 'endTurn' }), 200);
  $('play').textContent = 'Pause';
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
window.game = { state: () => s, act, save: () => JSON.parse(sim.save()) };
