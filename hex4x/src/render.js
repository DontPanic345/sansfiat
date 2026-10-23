import { TERRAIN, UNITS } from './data.js';
import { toPixel } from './hex.js';

// view: {size, ox, oy, selected?: {q,r}, reach?: Map, targets?: hex[], supplyLine?: hex[]}
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
    ctx.strokeStyle = u.movesLeft > 0 && u.owner === s.current ? '#fff' : '#111';
    ctx.stroke();
    label(ctx, UNITS[u.type].glyph, x, y + 1, size * 0.5);
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

  if (view.selected) {
    hexPath(ctx, at(view.selected), size * 0.95);
    ctx.strokeStyle = '#ffe14d';
    ctx.lineWidth = 3;
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

