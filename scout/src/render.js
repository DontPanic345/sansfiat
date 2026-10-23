import { TERRAIN, UNITS } from './data.js';
import { DIRS, key, toPixel } from './hex.js';

// view: {size, ox, oy, viewer, selected?: {q,r}, selectedUnit?: id, reach?: Map, targets?: hex[], supplyLine?: hex[],
//        route?: {from, to, known, possible, unknown} (see rules.routePlan)}
export function draw(ctx, s, view) {
  const { size } = view;
  const at = (h) => {
    const p = toPixel(h, size);
    return { x: p.x + view.ox, y: p.y + view.oy };
  };

  ctx.fillStyle = '#10161d';
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

  for (const t of Object.values(s.map.tiles)) {
    hexPath(ctx, at(t), size);
    ctx.fillStyle = TERRAIN[t.terrain].color;
    ctx.fill();
    ctx.strokeStyle = 'rgba(0,0,0,0.25)';
    ctx.lineWidth = 1;
    ctx.stroke();
  }

  drawTrails(ctx, s, at, size);
  const revealed = s.players[view.viewer]?.revealed ?? {};
  for (const t of Object.values(s.map.tiles)) {
    if (!revealed[key(t)] || t.trail) continue;
    const { x, y } = at(t);
    const text = t.trailCost == null ? 'x' : t.span ? `${t.trailCost}/${t.span}` : `${t.trailCost}`;
    label(ctx, text, x, y - size * 0.5, size * 0.36);
  }

  for (const r of view.reach?.values() ?? []) {
    hexPath(ctx, at(r.hex), size * 0.92);
    ctx.fillStyle = 'rgba(255,255,255,0.28)';
    ctx.fill();
  }

  for (const c of Object.values(s.cities)) {
    const { x, y } = at(c);
    const w = size * 0.9;
    ctx.fillStyle = s.players[c.owner].color;
    ctx.strokeStyle = '#111';
    ctx.lineWidth = 2;
    ctx.fillRect(x - w / 2, y - w / 2, w, w);
    ctx.strokeRect(x - w / 2, y - w / 2, w, w);
    label(ctx, c.name, x, y + size * 0.95, size * 0.42);
  }

  const stacks = new Map();
  for (const u of Object.values(s.units)) stacks.set(key(u), [...(stacks.get(key(u)) ?? []), u]);
  for (const stack of stacks.values()) {
    const u = stack.find((x) => x.id === view.selectedUnit)
      ?? stack.find((x) => x.movesLeft > 0 && x.owner === s.current) ?? stack[0];
    const { x, y } = at(u);
    ctx.beginPath();
    ctx.arc(x, y, size * 0.42, 0, Math.PI * 2);
    ctx.fillStyle = s.players[u.owner].color;
    ctx.fill();
    ctx.lineWidth = 2;
    ctx.strokeStyle = u.movesLeft > 0 && u.owner === s.current ? '#fff' : '#111';
    ctx.stroke();
    label(ctx, `${UNITS[u.type].glyph}${u.n}`, x, y + 1, size * 0.42);
    if (stack.length > 1) label(ctx, `${stack.length}`, x + size * 0.45, y - size * 0.4, size * 0.34);
  }

  for (const h of view.targets ?? []) {
    hexPath(ctx, at(h), size * 0.85);
    ctx.strokeStyle = '#ff3b3b';
    ctx.lineWidth = 3;
    ctx.stroke();
  }

  if (view.supplyLine?.length > 1) {
    ctx.beginPath();
    view.supplyLine.forEach((h, i) => (i ? ctx.lineTo : ctx.moveTo).call(ctx, at(h).x, at(h).y));
    ctx.strokeStyle = '#7dffb0';
    ctx.lineWidth = 4;
    ctx.setLineDash([8, 6]);
    ctx.lineJoin = 'round';
    ctx.stroke();
    ctx.setLineDash([]);
  }

  if (view.route) drawRoute(ctx, view.route, at, size);

  if (view.selected) {
    hexPath(ctx, at(view.selected), size * 0.95);
    ctx.strokeStyle = '#ffe14d';
    ctx.lineWidth = 3;
    ctx.stroke();
  }
}

function drawTrails(ctx, s, at, size) {
  ctx.strokeStyle = ctx.fillStyle = '#f0dca0';
  ctx.lineWidth = 3;
  ctx.setLineDash([5, 4]);
  for (const t of Object.values(s.map.tiles)) {
    if (!t.trail) continue;
    const a = at(t);
    for (const [dq, dr] of DIRS.slice(0, 3)) {
      const n = { q: t.q + dq, r: t.r + dr };
      if (!s.map.tiles[key(n)]?.trail) continue;
      const b = at(n);
      ctx.beginPath();
      ctx.moveTo(a.x, a.y);
      ctx.lineTo(b.x, b.y);
      ctx.stroke();
    }
    ctx.beginPath();
    ctx.arc(a.x, a.y, size * 0.12, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.setLineDash([]);
}

// Best known route solid, a cheaper possible route dashed, its unrevealed tiles marked '?'.
function drawRoute(ctx, route, at, size) {
  const line = (path, color, dash) => {
    if (path.length < 2) return;
    ctx.beginPath();
    path.forEach((h, i) => (i ? ctx.lineTo : ctx.moveTo).call(ctx, at(h).x, at(h).y));
    ctx.strokeStyle = color;
    ctx.lineWidth = 3;
    ctx.lineJoin = 'round';
    ctx.setLineDash(dash);
    ctx.stroke();
    ctx.setLineDash([]);
  };
  line(route.known.path, '#ff9f40', []);
  if (route.possible.cost < route.known.cost) line(route.possible.path, '#ffe14d', [6, 6]);
  for (const h of route.unknown) {
    const { x, y } = at(h);
    label(ctx, '?', x, y, size * 0.6);
  }
  for (const h of [route.from, route.to]) {
    hexPath(ctx, at(h), size * 0.7);
    ctx.strokeStyle = '#ffe14d';
    ctx.lineWidth = 2;
    ctx.stroke();
  }
}

function hexPath(ctx, { x, y }, size) {
  ctx.beginPath();
  for (let i = 0; i < 6; i++) {
    const a = (Math.PI / 180) * (60 * i - 30);
    ctx.lineTo(x + size * Math.cos(a), y + size * Math.sin(a));
  }
  ctx.closePath();
}

function label(ctx, text, x, y, px) {
  ctx.font = `bold ${Math.round(px)}px system-ui, sans-serif`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.lineWidth = 3;
  ctx.strokeStyle = 'rgba(0,0,0,0.7)';
  ctx.strokeText(text, x, y);
  ctx.fillStyle = '#fff';
  ctx.fillText(text, x, y);
}
