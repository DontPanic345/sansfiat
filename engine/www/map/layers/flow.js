// Flow: with a store's good picked, each lot of it in the store drawn as its chain, from the source through the
// tile of each market it changed hands at to the store, its width by share, the source marked. Nothing is drawn
// with no good picked.
export const name = 'flow';

const COLOUR = { salt: '#f4f7fa', obsidian: '#0a0a0c' };

export function draw(ctx, model, view) {
  const flow = view.flow;
  if (!flow || !flow.chains.length) return;
  const px = 1 / view.k;
  const colour = COLOUR[flow.good] ?? '#ffffff';
  // Obsidian is nearly black: its chain is edged light; salt's is edged dark.
  const edge = flow.good === 'obsidian' ? 'rgba(255,255,255,0.75)' : 'rgba(0,0,0,0.8)';
  const path = (tiles) => {
    ctx.beginPath();
    tiles.forEach((i, n) => (n === 0 ? ctx.moveTo(view.px[i], view.py[i]) : ctx.lineTo(view.px[i], view.py[i])));
    ctx.stroke();
  };
  ctx.globalAlpha = 1;
  ctx.setLineDash([]);
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  for (const { share, path: tiles } of flow.chains) {
    const w = 2 + 8 * share;
    for (const [c, k] of [[edge, w + 2.5], [colour, w]]) {
      ctx.strokeStyle = c;
      ctx.lineWidth = Math.max(px * k, 0.1);
      path(tiles);
    }
  }
  // The source of each chain, ringed: the tile the good was dug at.
  for (const source of new Set(flow.chains.map((c) => c.path[0]))) {
    ctx.beginPath();
    ctx.arc(view.px[source], view.py[source], 0.3, 0, 2 * Math.PI);
    ctx.lineWidth = Math.max(px * 4, 0.1);
    ctx.strokeStyle = edge;
    ctx.stroke();
    ctx.lineWidth = Math.max(px * 2, 0.05);
    ctx.strokeStyle = colour;
    ctx.stroke();
  }
}
