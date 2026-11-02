// Worn land: a dark wash, darker the more its most worn food is worn (dark, so it shows on hills too).
export const name = 'worn';

export function draw(ctx, model, view) {
  ctx.fillStyle = '#1a0f07';
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (view.worn[i] <= 0) continue;
    ctx.globalAlpha = Math.min(0.6, view.worn[i]);
    ctx.beginPath();
    view.hexAt(ctx, i, 0.98);
    ctx.fill();
  }
  ctx.globalAlpha = 1;
}
