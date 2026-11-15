// Each civ's land: a border in its colour round its claim, with a dark casing.
export const name = 'land';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const casing = 'rgba(0,0,0,0.8)';
  ctx.setLineDash([]);
  for (const { civ, solid } of view.civShapes()) {
    for (const [col, extra] of [[casing, px * 3], [civ.color, 0]]) {
      ctx.strokeStyle = col;
      ctx.lineWidth = Math.max(px * 2.2, 0.16) + extra;
      ctx.stroke(solid);
    }
  }
}
