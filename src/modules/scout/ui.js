// UI hooks for the scout module: fog dimming, selected scout's route, status panel.
import { key } from '../../hex.js';
import { civUnits, humanCiv } from '../../core.js';
import { routePlan } from './rules.js';

export const z = 90;

export function drawLayer(ctx, s, view) {
  const civ = s.civs[view.viewer];
  if (!civ) return;
  ctx.save();
  ctx.fillStyle = 'rgba(0,0,0,0.55)';
  for (const t of Object.values(s.map.tiles)) {
    if (civ.seen?.[key(t)]) continue;
    view.hexPath(ctx, view.at(t), view.size);
    ctx.fill();
  }
  const u = s.units[view.local?.unit];
  if (u?.scout?.route) {
    const plan = routePlan(s, civ.id, u.scout.route.from, u.scout.route.to);
    ctx.lineWidth = 2;
    for (const [p, col] of [[plan.possible, '#6cf'], [plan.known, '#fc4']]) {
      if (!p) continue;
      ctx.strokeStyle = col;
      ctx.beginPath();
      p.path.forEach((h, i) => { const c = view.at(h); i ? ctx.lineTo(c.x, c.y) : ctx.moveTo(c.x, c.y); });
      ctx.stroke();
    }
  }
  for (const k of Object.keys(civ.revealed ?? {})) {
    const [q, r] = k.split(',').map(Number);
    const c = view.at({ q, r });
    ctx.fillStyle = '#fff';
    ctx.fillRect(c.x - 1, c.y - 1, 3, 3);
  }
  ctx.restore();
}

export function panel(s, selection, ui) {
  const me = humanCiv(s);
  if (!me) return '';
  const scouts = civUnits(s, me.id).filter((u) => u.type === 'scout');
  if (!scouts.length) return '';
  const sel = s.units[ui.local.unit] ?? scouts[0];
  const rt = sel.scout?.route;
  const rows = scouts.map((u) => `<div>${u.id === sel.id ? '&gt; ' : ''}scout ${u.id} at ${u.q},${u.r} labour ${u.labour}/${u.labourMax}` +
    ` ${u.scout.route ? 'on route' : 'idle'}</div>`).join('');
  let status = '';
  if (rt) {
    const p = routePlan(s, me.id, rt.from, rt.to);
    status = `<div>Route to ${rt.to.q},${rt.to.r}: known cost ${p.known?.cost ?? '-'}, best ${p.possible?.cost ?? '-'}, ${p.unknown.length} unrevealed</div>`;
  } else if (selection) status = `<div>Click a tile to send scout ${sel.id} to ${selection.q},${selection.r} (select it first).</div>`;
  return `<h3>Scouts</h3>${rows}${status}<div>Revealed costs: ${Object.keys(me.revealed ?? {}).length}</div>`;
}

// Clicking a tile with a scout on it selects it; clicking elsewhere with one selected sends it there.
export function onClick(s, hex, ui) {
  const me = humanCiv(s);
  if (!me) return false;
  const here = civUnits(s, me.id).find((u) => u.type === 'scout' && u.q === hex.q && u.r === hex.r);
  if (here) { ui.local.unit = here.id; return true; }
  const u = s.units[ui.local.unit];
  if (!u || u.owner !== me.id) return false;
  ui.act({ type: 'assignRoute', civ: me.id, unit: u.id, from: { q: u.q, r: u.r }, to: { q: hex.q, r: hex.r } });
  return false;
}
