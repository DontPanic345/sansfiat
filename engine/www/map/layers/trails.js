// Trails: a short line across the tile; the tiles the led civ has revealed get a faint ring, and the
// trails under way a line as long as the labour put in so far.
export const name = 'trails';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const trailW = Math.max(px * 1.6, 0.14);
  for (const [col, extra] of [['rgba(60,40,10,0.6)', px * 1.6], ['#f2dca4', 0]]) {
    ctx.strokeStyle = col;
    ctx.lineWidth = trailW + extra;
    ctx.beginPath();
    for (const i of model.v.trails) {
      if (!view.onScreen(i)) continue;
      ctx.moveTo(view.px[i] - 0.45, view.py[i] + 0.22);
      ctx.lineTo(view.px[i] + 0.45, view.py[i] - 0.22);
    }
    ctx.stroke();
  }
  ctx.strokeStyle = 'rgba(180,220,255,0.45)';
  ctx.lineWidth = Math.max(px * 1.2, 0.05);
  ctx.beginPath();
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (view.revealed[i]) view.hexAt(ctx, i, 0.9);
  }
  ctx.stroke();
  ctx.strokeStyle = '#f2dca4';
  ctx.lineWidth = trailW;
  ctx.globalAlpha = 0.45;
  ctx.beginPath();
  for (const [i, done, need] of view.work) {
    if (!view.onScreen(i) || need <= 0) continue;
    const len = 0.9 * Math.min(1, done / need);
    ctx.moveTo(view.px[i] - 0.45, view.py[i] + 0.22);
    ctx.lineTo(view.px[i] - 0.45 + len, view.py[i] + 0.22 - len * (0.44 / 0.9));
  }
  ctx.stroke();
  ctx.globalAlpha = 1;
}
