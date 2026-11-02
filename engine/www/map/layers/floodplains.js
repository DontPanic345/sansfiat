// Floodplains: a bright green wash with a thin darker border (the delta's marsh is its own colour).
export const name = 'floodplains';
const isStatic = true;
export { isStatic as static };

export function draw(ctx, model, view) {
  const { l } = view;
  const px = 1 / view.k;
  const marsh = view.byName.marsh;
  const wet = (i) => l.flood[i] && l.terrain[i] !== marsh;
  const cells = view.atlas && view.stats.atlas;
  if (cells) {
    const s = 2 * 1.02 * (74 / 74); // the cell is 148 px for a hex of circumradius 74
    const c = view.atlas.cell('overlay.floodplain');
    for (let a = 0; a < view.nvis; a++) {
      const i = view.vis[a];
      if (wet(i)) ctx.drawImage(view.atlas.image, c.x, c.y, c.w, c.h, view.px[i] - (s * c.w) / c.h / 2, view.py[i] - s / 2, (s * c.w) / c.h, s);
    }
  } else {
    ctx.fillStyle = 'rgba(96,235,70,0.42)';
    ctx.beginPath();
    for (let a = 0; a < view.nvis; a++) if (wet(view.vis[a])) view.hexAt(ctx, view.vis[a], 1.02);
    ctx.fill();
  }
  ctx.strokeStyle = 'rgba(28,110,24,0.9)';
  ctx.lineWidth = Math.max(px * 1.2, 0.05);
  view.edges(ctx, (i) => l.flood[i] === 1, (i, j) => l.flood[j] === 1);
}
