// Territories: a faint fill in each clan's colour, a firmer line where it meets other ground.
import { hue, CORNER } from '../geom.js';

export const name = 'territories';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const byClan = new Map();
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    const o = view.claim[i];
    if (!o) continue;
    let list = byClan.get(o);
    if (!list) byClan.set(o, (list = []));
    list.push(i);
  }
  const f = view.focus;
  const dim = (id, on, off) => (f == null || f === id ? on : off);
  for (const [id, list] of byClan) {
    ctx.globalAlpha = dim(id, f == null ? 0.3 : 0.45, 0.12);
    ctx.fillStyle = hue(id);
    ctx.beginPath();
    for (const i of list) view.hexAt(ctx, i, 1.02);
    ctx.fill();
  }
  for (const [id, list] of byClan) {
    ctx.globalAlpha = dim(id, 1, 0.3);
    ctx.lineWidth = Math.max(px * (f === id ? 2.4 : 1.2), 0.04);
    ctx.strokeStyle = hue(id);
    ctx.beginPath();
    for (const i of list) {
      for (let d = 0; d < 6; d++) {
        const j = view.nb[i * 6 + d];
        if (j >= 0 && view.claim[j] === id) continue;
        const e = (d + 1) % 6;
        ctx.moveTo(view.px[i] + CORNER[d][0], view.py[i] + CORNER[d][1]);
        ctx.lineTo(view.px[i] + CORNER[e][0], view.py[i] + CORNER[e][1]);
      }
    }
    ctx.stroke();
  }
  ctx.globalAlpha = 1;
}
