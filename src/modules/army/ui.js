// Army UI: zones, captains, detachments with troop counts. Click paints (or selects a captain).
export const z = 20;

export function drawLayer(ctx, s, view) {
  const col = (o) => s.civs[o]?.color ?? '#888';
  ctx.save();
  for (const u of Object.values(s.units)) {
    if (u.type !== 'captain') continue;
    ctx.globalAlpha = 0.3; ctx.fillStyle = col(u.owner);
    for (const k of u.army.zone) {
      const [q, r] = k.split(',').map(Number);
      view.hexPath(ctx, view.at({ q, r }), view.size * 0.9); ctx.fill();
    }
  }
  ctx.globalAlpha = 1; ctx.textAlign = 'center'; ctx.textBaseline = 'middle'; ctx.font = `${Math.max(9, view.size * 0.5)}px sans-serif`;
  for (const u of Object.values(s.units)) {
    if (u.type !== 'captain' && u.type !== 'detachment') continue;
    const p = view.at(u), cap = u.type === 'captain';
    ctx.fillStyle = col(u.owner); ctx.strokeStyle = '#000';
    ctx.beginPath(); ctx.arc(p.x, p.y, view.size * (cap ? 0.4 : 0.28), 0, 7); ctx.fill(); ctx.stroke();
    ctx.fillStyle = '#fff'; ctx.fillText(String(Math.round(u.army.troops)), p.x, p.y);
  }
  ctx.restore();
}

export function panel(s, selection, ui) {
  const sel = ui?.local?.captain != null ? s.units[ui.local.captain] : null;
  if (!sel) return '';
  return `<div>Captain: ${sel.army.troops} troops, zone ${sel.army.zone.length}
    <button data-action='{"type":"undeploy","civ":${sel.owner},"unit":${sel.id}}'>Undeploy</button>
    <button data-action='{"type":"buildFort","civ":${sel.owner},"unit":${sel.id}}'>Build fort</button></div>`;
}

export function onClick(s, hex, ui) {
  const here = Object.values(s.units).find((u) => u.type === 'captain' && u.q === hex.q && u.r === hex.r);
  if (here) { ui.local.captain = here.id; return true; }
  const cap = ui.local.captain != null ? s.units[ui.local.captain] : null;
  if (!cap) return false;
  const k = `${hex.q},${hex.r}`;
  if (cap.army.zone.includes(k)) return ui.act({ type: 'unpaint', civ: cap.owner, unit: cap.id, ...hex });
  return ui.act({ type: 'paint', civ: cap.owner, unit: cap.id, ...hex });
}
