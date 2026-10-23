import { TERRAIN, UNITS } from './data.js';
import { toPixel } from './hex.js';

// view: {size, ox, oy, selected?: {q,r}, reach?: Map, targets?: hex[], supply?: {line: hex[], ok}}
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
    if ((t.depletedUntil ?? 0) > s.turn) {
      hexPath(ctx, at(t), size * 0.6);
      ctx.fillStyle = 'rgba(70,40,10,0.45)';
      ctx.fill();
    }
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

  for (const u of Object.values(s.units)) {
    const { x, y } = at(u);
    ctx.beginPath();
    ctx.arc(x, y, size * 0.42, 0, Math.PI * 2);
    ctx.fillStyle = s.players[u.owner].color;
    ctx.fill();
    ctx.lineWidth = 2;
    ctx.strokeStyle = u.labor > 0 && u.owner === s.current ? '#fff' : '#111';
    ctx.stroke();
    label(ctx, UNITS[u.type].glyph, x, y + 1, size * 0.5);
    const def = UNITS[u.type];
    if (u.hp < def.hp) bar(ctx, x, y - size * 0.62, size, u.hp / def.hp, '#ff5050');
    if (def.upkeep) bar(ctx, x, y + size * 0.62, size, u.food / def.food, '#ffc94d');
  }

  for (const h of view.targets ?? []) {
    hexPath(ctx, at(h), size * 0.85);
    ctx.strokeStyle = '#ff3b3b';
    ctx.lineWidth = 3;
    ctx.stroke();
  }

  if (view.supply?.line.length > 1) {
    ctx.beginPath();
    view.supply.line.forEach((h, i) => (i ? ctx.lineTo : ctx.moveTo).call(ctx, at(h).x, at(h).y));
    ctx.strokeStyle = view.supply.ok ? '#7dffb0' : '#ff9a4d';
    ctx.lineWidth = 4;
    ctx.setLineDash([8, 6]);
    ctx.lineJoin = 'round';
    ctx.stroke();
    ctx.setLineDash([]);
  }

  if (view.selected) {
    hexPath(ctx, at(view.selected), size * 0.95);
    ctx.strokeStyle = '#ffe14d';
    ctx.lineWidth = 3;
    ctx.stroke();
  }
}

function bar(ctx, x, y, size, frac, color) {
  const w = size * 0.8, h = Math.max(3, size * 0.12);
  ctx.fillStyle = 'rgba(0,0,0,0.7)';
  ctx.fillRect(x - w / 2 - 1, y - h / 2 - 1, w + 2, h + 2);
  ctx.fillStyle = color;
  ctx.fillRect(x - w / 2, y - h / 2, w * Math.max(0, frac), h);
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

