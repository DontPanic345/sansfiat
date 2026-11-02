// Held ground: each camp's zone outlined in its civ's colour, the tiles its warriors hold filled faintly.
export const name = 'held';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const casing = 'rgba(0,0,0,0.8)';
  for (const u of Object.values(v.units || {})) {
    if (u.kind !== 'camp') continue;
    const col = v.civs[u.civ]?.color || '#fff';
    const zone = new Set(u.territory);
    if (zone.size <= 1) continue;
    ctx.globalAlpha = 0.22;
    ctx.fillStyle = col;
    ctx.beginPath();
    for (const i of u.held) if (view.onScreen(i)) view.hexAt(ctx, i, 0.92);
    ctx.fill();
    ctx.globalAlpha = 1;
    for (const [c, w] of [[casing, 4], [col, 2]]) {
      ctx.strokeStyle = c;
      ctx.lineWidth = Math.max(px * w, 0.1);
      ctx.setLineDash(c === casing ? [] : [Math.max(px * 4, 0.3), Math.max(px * 2, 0.15)]);
      view.edges(ctx, (i) => zone.has(i), (_, j) => zone.has(j));
    }
    ctx.setLineDash([]);
  }
}
