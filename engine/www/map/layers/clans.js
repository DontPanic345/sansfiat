// Clans: a circle at home, its area by people, in the clan's colour; a red ring when starving. A clan with
// a civ gets a ring in the civ's colour (with the civ's land).
import { hue } from '../geom.js';

export const name = 'clans';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const casing = 'rgba(0,0,0,0.8)';
  for (const c of Object.values(v.clans)) {
    if (!view.onScreen(c.home)) continue;
    const x = view.px[c.home], y = view.py[c.home];
    const r = Math.min(0.75, Math.max(px * 2.2, 0.16 * Math.sqrt(c.people)));
    ctx.beginPath();
    ctx.arc(x, y, r, 0, 2 * Math.PI);
    ctx.fillStyle = hue(c.id);
    ctx.fill();
    ctx.lineWidth = Math.max(px, 0.04);
    ctx.strokeStyle = 'rgba(0,0,0,0.75)';
    ctx.stroke();
    if (c.starving) {
      ctx.beginPath();
      ctx.arc(x, y, r + px * 1.6, 0, 2 * Math.PI);
      ctx.lineWidth = Math.max(px * 2.2, 0.1);
      ctx.strokeStyle = '#ff2a2a';
      ctx.stroke();
    }
    const civId = model.clanCiv.get(c.id);
    if (civId != null && view.on.land) {
      const rr = r + px * 3.4;
      for (const [col, w] of [[casing, 4.2], [v.civs[civId].color, 2.2]]) {
        ctx.strokeStyle = col;
        ctx.lineWidth = Math.max(px * w, 0.08);
        ctx.beginPath();
        ctx.arc(x, y, rr, 0, 2 * Math.PI);
        ctx.stroke();
      }
    }
  }
}
