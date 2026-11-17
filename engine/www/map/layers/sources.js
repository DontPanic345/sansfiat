// Sources: a small mark on each tile salt or obsidian comes from. Salt is a white crystal, obsidian a
// black shard, so the two read apart at a glance. Plain shapes, drawn where the map is big enough to show them.

export const name = 'sources';
const isStatic = true;
export { isStatic as static };

export function draw(ctx, model, view) {
  const { l } = view;
  if (!l.source || view.k < 3) return;
  const px = 1 / view.k;
  const u = Math.max(0.12, px * 7);
  ctx.lineJoin = 'round';
  ctx.lineWidth = Math.max(px * 1.2, 0.03);
  ctx.strokeStyle = '#222';
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    const good = l.source[i];
    if (!good) continue;
    const x = view.px[i] - 0.3, y = view.py[i] + 0.28;
    ctx.beginPath();
    if (good === 'salt') {
      // a crystal: a rhombus
      ctx.moveTo(x, y - u);
      ctx.lineTo(x + u * 0.8, y);
      ctx.lineTo(x, y + u * 0.9);
      ctx.lineTo(x - u * 0.8, y);
      ctx.closePath();
      ctx.fillStyle = '#f4f7fa';
    } else {
      // a shard: a tall triangle leaning right
      ctx.moveTo(x - u * 0.7, y + u * 0.9);
      ctx.lineTo(x + u * 0.3, y - u * 1.1);
      ctx.lineTo(x + u * 0.8, y + u * 0.9);
      ctx.closePath();
      ctx.fillStyle = '#0a0a0c';
      ctx.strokeStyle = '#9aa';
    }
    ctx.fill();
    ctx.stroke();
    ctx.strokeStyle = '#222';
  }
}
