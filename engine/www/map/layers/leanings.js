// Leanings: an unaligned clan's circle tinted in the colour of the civ it leans to, the more opaque the
// nearer it is to joining. Drawn over the clans' circles.
export const name = 'leanings';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  for (const c of Object.values(model.v.clans)) {
    if (!view.onScreen(c.home)) continue;
    const tint = model.tintOf(c);
    if (!tint) continue;
    const r = Math.min(1.4, Math.max(px * 2.2, 0.22 * Math.sqrt(c.people)));
    ctx.beginPath();
    ctx.arc(view.px[c.home], view.py[c.home], r, 0, 2 * Math.PI);
    ctx.globalAlpha = tint.alpha;
    ctx.fillStyle = tint.color;
    ctx.fill();
    ctx.globalAlpha = 1;
    ctx.lineWidth = Math.max(px, 0.04);
    ctx.strokeStyle = 'rgba(0,0,0,0.75)';
    ctx.stroke();
  }
}
