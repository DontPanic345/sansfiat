import { unitsAt } from '../../core.js';
import { supplyLine, ensure, troopsOf } from './rules.js';

export const z = 30;

const selCaptain = (s, h) => h && unitsAt(s, h).find((u) => u.type === 'captain');

export function drawLayer(ctx, s, view) {
  for (const t of Object.values(s.map.tiles)) {
    if (!t.cache) continue;
    const p = view.at(t);
    ctx.fillStyle = '#d8a22b';
    ctx.fillRect(p.x - 4, p.y - 4, 8, 8);
  }
  const u = selCaptain(s, view.selected);
  const line = u && supplyLine(s, u);
  if (!line) return;
  ctx.strokeStyle = '#e8d44d';
  ctx.lineWidth = 2;
  ctx.beginPath();
  line.path.forEach((h, i) => { const p = view.at(h); if (i) ctx.lineTo(p.x, p.y); else ctx.moveTo(p.x, p.y); });
  ctx.stroke();
}

export function panel(s, selection, ui) {
  const u = selCaptain(s, selection);
  if (!u) return '';
  const f = ensure(u), line = supplyLine(s, u);
  return `<div><b>Supply</b> food ${f.food.toFixed(1)}/${f.foodMax} (army eats ${troopsOf(u)}/turn)${u.hungry ? ' HUNGRY' : ''}<br>`
    + (line ? `line: ${line.src.kind}, cost ${line.src.cost}, ${line.src.labour} labour` : 'no supply line')
    + `<br><button data-action='{"type":"resupply","civ":${u.owner},"unitId":${u.id}}'>Resupply</button> `
    + `<button data-action='{"type":"autoSupply","civ":${u.owner},"unitId":${u.id},"on":${!f.auto}}'>Auto: ${f.auto ? 'on' : 'off'}</button></div>`;
}

export function onClick() { return false; }
