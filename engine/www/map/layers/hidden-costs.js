// Debug: trail cost / span on forest, hills and mountain, bridgeable mountains tinted.
export const name = 'hidden-costs';
const isStatic = true;
export { isStatic as static };

export function draw(ctx, model, view) {
  const { l } = view;
  const { mountain } = view.byName;
  ctx.fillStyle = 'rgba(255,140,0,0.5)';
  ctx.beginPath();
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (l.terrain[i] === mountain && l.span[i] != null) view.hexAt(ctx, i, 1);
  }
  ctx.fill();
  if (view.k < 13) return;
  ctx.font = '0.55px system-ui, sans-serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    const c = l.trail_cost[i];
    if (c == null) continue;
    const t = l.terrain[i];
    if (t !== mountain && view.names[t] !== 'forest' && view.names[t] !== 'hills') continue;
    const text = l.span[i] != null ? `${c}/${l.span[i]}` : `${c}`;
    ctx.fillStyle = 'rgba(0,0,0,0.65)';
    ctx.fillText(text, view.px[i] + 0.03, view.py[i] + 0.82 * 0.5 + 0.03);
    ctx.fillStyle = '#fff';
    ctx.fillText(text, view.px[i], view.py[i] + 0.82 * 0.5);
  }
}
