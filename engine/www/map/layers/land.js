// Each civ's land: a border in its colour, solid round the land it holds and dashed round its paper
// (claimed, not held), with a dark casing.
export const name = 'land';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const casing = 'rgba(0,0,0,0.8)';
  for (const { civ, solid, dashed } of view.civShapes()) {
    for (const [path, dash] of [
      [dashed, [Math.max(px * 6, 0.4), Math.max(px * 4, 0.25)]],
      [solid, []],
    ]) {
      for (const [col, extra] of [[casing, px * 3], [civ.color, 0]]) {
        ctx.setLineDash(dash);
        ctx.strokeStyle = col;
        ctx.lineWidth = Math.max(px * 2.2, 0.16) + extra;
        ctx.stroke(path);
      }
    }
  }
  ctx.setLineDash([]);
}
