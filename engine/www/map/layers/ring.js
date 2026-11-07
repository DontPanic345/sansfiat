// The ring of the selected army's or civ's civ: the tiles its line feeds an army from with days to spare,
// each filled in the civ's colour, the less of an army's days its line takes the stronger; the edge
// between the ring and what is outside it (or past it) drawn solid in the civ's colour, edged dark as the
// line home is. Nothing past the ring is drawn.
import { CORNER } from '../geom.js';

export const name = 'ring';

export function draw(ctx, model, view) {
  const ring = view.ring;
  if (!ring) return;
  const { colour, shares } = ring;
  const inside = (i) => {
    const s = shares.get(i);
    return s !== undefined && s < 1;
  };
  const corner = (path, i, d, move) => {
    const x = view.px[i] + CORNER[d][0], y = view.py[i] + CORNER[d][1];
    if (move) path.moveTo(x, y);
    else path.lineTo(x, y);
  };
  ctx.setLineDash([]);
  ctx.fillStyle = colour;
  for (const [i, s] of shares) {
    if (s >= 1) continue;
    ctx.globalAlpha = 0.08 + 0.32 * (1 - s);
    ctx.beginPath();
    for (let d = 0; d < 6; d++) corner(ctx, i, d, d === 0);
    ctx.closePath();
    ctx.fill();
  }
  const edge = new Path2D();
  for (const [i, s] of shares) {
    if (s >= 1) continue;
    for (let d = 0; d < 6; d++) {
      const j = view.nb[i * 6 + d];
      if (j < 0 || inside(j)) continue;
      const e = (d + 1) % 6;
      corner(edge, i, d, true);
      corner(edge, i, e, false);
    }
  }
  const px = 1 / view.k;
  ctx.globalAlpha = 1;
  for (const [c, w] of [['rgba(0,0,0,0.8)', 7], [colour, 4]]) {
    ctx.strokeStyle = c;
    ctx.lineWidth = Math.max(px * w, 0.12);
    ctx.stroke(edge);
  }
}
