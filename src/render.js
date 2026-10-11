import { TERRAIN } from './data.js';
import { key, neighbors, toPixel } from './hex.js';
import { marketReach, total } from './rules.js';

// view: {size, ox, oy, selected?: {q,r}, depletion: bool}
export function draw(ctx, s, view) {
  const { size } = view;
  const at = (h) => {
    const p = toPixel(h, size);
    return { x: p.x + view.ox, y: p.y + view.oy };
  };
  const tiles = s.map.tiles;

  ctx.fillStyle = '#10161d';
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

  for (const t of Object.values(tiles)) {
    hexPath(ctx, at(t), size);
    ctx.fillStyle = t.shallows ? '#3a72a0' : TERRAIN[t.terrain].color;
    ctx.fill();
    const cap = total(t.cap);
    if (view.depletion && cap > 0) {
      ctx.fillStyle = `rgba(20,12,4,${0.65 * (1 - total(t.stock) / cap)})`;
      ctx.fill();
    }
    ctx.strokeStyle = 'rgba(0,0,0,0.2)';
    ctx.lineWidth = 1;
    ctx.stroke();
  }

  ctx.strokeStyle = '#6cb8f0';
  ctx.lineCap = 'round';
  ctx.lineWidth = Math.max(2, size * 0.22);
  for (const t of Object.values(tiles)) {
    if (!t.river) continue;
    const down = t.flow ? tiles[t.flow] : neighbors(t).map((n) => tiles[key(n)]).find((n) => n?.terrain === 'ocean');
    if (!down) continue;
    const a = at(t), b = at(down);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(t.flow ? b.x : (a.x + b.x) / 2, t.flow ? b.y : (a.y + b.y) / 2);
    ctx.stroke();
  }

  const selMarket = view.selected && Object.values(s.markets).find((m) => m.q === view.selected.q && m.r === view.selected.r);
  if (selMarket) {
    for (const k of marketReach(s, selMarket).keys()) {
      hexPath(ctx, at(tiles[k]), size * 0.9);
      ctx.fillStyle = 'rgba(255,255,255,0.18)';
      ctx.fill();
    }
  }

  ctx.lineWidth = 1.5;
  for (const c of Object.values(s.clans)) {
    const m = s.markets[c.market];
    if (!m) continue;
    const a = at(c), b = at(m);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.strokeStyle = m.owner == null ? 'rgba(255,255,255,0.35)' : hexAlpha(s.players[m.owner].color, 0.6);
    ctx.stroke();
  }

  for (const m of Object.values(s.markets)) {
    const { x, y } = at(m);
    const w = size * 1.1;
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(Math.PI / 4);
    ctx.fillStyle = m.owner == null ? '#e9e4d8' : s.players[m.owner].color;
    ctx.strokeStyle = '#111';
    ctx.lineWidth = 2;
    ctx.fillRect(-w / 2, -w / 2, w, w);
    ctx.strokeRect(-w / 2, -w / 2, w, w);
    ctx.restore();
  }

  ctx.lineJoin = 'round';
  for (const c of Object.values(s.clans)) {
    if (c.trail.length) {
      ctx.beginPath();
      [...c.trail.map((k) => tiles[k]), c].forEach((h, i) => (i ? ctx.lineTo : ctx.moveTo).call(ctx, at(h).x, at(h).y));
      ctx.strokeStyle = 'rgba(255,240,200,0.55)';
      ctx.lineWidth = 2;
      ctx.stroke();
    }
    if (c.route.length) {
      ctx.beginPath();
      [c, ...c.route.map((k) => tiles[k])].forEach((h, i) => (i ? ctx.lineTo : ctx.moveTo).call(ctx, at(h).x, at(h).y));
      ctx.strokeStyle = '#ffe9a8';
      ctx.lineWidth = 2;
      ctx.setLineDash([4, 4]);
      ctx.stroke();
      ctx.setLineDash([]);
    }
  }

  for (const c of Object.values(s.clans)) {
    const { x, y } = at(c);
    ctx.beginPath();
    ctx.arc(x, y, Math.min(0.6, 0.14 * Math.sqrt(c.pop)) * size, 0, Math.PI * 2);
    ctx.fillStyle = c.fed ? '#f4ead2' : c.fedRatio > 0.85 ? '#e8a33d' : '#d9534f';
    ctx.fill();
    ctx.lineWidth = 1.5;
    ctx.strokeStyle = '#1a1208';
    ctx.stroke();
  }

  if (view.selected) {
    hexPath(ctx, at(view.selected), size * 0.95);
    ctx.strokeStyle = '#ffe14d';
    ctx.lineWidth = 3;
    ctx.stroke();
    const clan = Object.values(s.clans).find((c) => c.q === view.selected.q && c.r === view.selected.r);
    if (clan) label(ctx, clan.name, at(clan).x, at(clan).y - size * 1.1, Math.max(11, size * 0.6));
  }
}

function label(ctx, text, x, y, px) {
  ctx.font = `bold ${Math.round(px)}px system-ui, sans-serif`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.lineWidth = 3;
  ctx.strokeStyle = 'rgba(0,0,0,0.75)';
  ctx.strokeText(text, x, y);
  ctx.fillStyle = '#fff';
  ctx.fillText(text, x, y);
}

function hexAlpha(hex, a) {
  const n = parseInt(hex.slice(1), 16);
  return `rgba(${n >> 16},${(n >> 8) & 255},${n & 255},${a})`;
}

function hexPath(ctx, { x, y }, size) {
  ctx.beginPath();
  for (let i = 0; i < 6; i++) {
    const a = (Math.PI / 180) * (60 * i - 30);
    ctx.lineTo(x + size * Math.cos(a), y + size * Math.sin(a));
  }
  ctx.closePath();
}
