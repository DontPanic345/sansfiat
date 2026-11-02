// Hidden country: an outline round each closed-country zone's outer edge (valley, sealed valley, neck).
export const name = 'country';
const isStatic = true;
export { isStatic as static };

const ZONE_COLOR = [null, '#f0c030', '#d9663f', '#b070e0'];

export function draw(ctx, model, view) {
  const { l } = view;
  const px = 1 / view.k;
  for (let z = 1; z <= 3; z++) {
    ctx.strokeStyle = ZONE_COLOR[z];
    ctx.lineWidth = Math.max(px * 2.2, 0.13);
    ctx.setLineDash(z === 2 ? [0.5, 0.3] : []);
    ctx.lineCap = z === 2 ? 'butt' : 'round';
    view.edges(ctx, (i) => l.zone[i] === z, (i, j) => l.zone[j] === z);
  }
  ctx.setLineDash([]);
  ctx.lineCap = 'round';
}
