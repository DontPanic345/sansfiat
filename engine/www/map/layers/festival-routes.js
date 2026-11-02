// Festival routes: a dashed line from home along the path to the festival, in the clan's colour (dashed,
// so a route does not read as a river). A selected clan's is bright, a selected festival's walkers' too.
import { hue, walkOf } from '../geom.js';

export const name = 'festival-routes';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const clans = Object.values(model.v.clans).filter((c) => walkOf(c).length >= 2);
  // A selected festival's walkers: the clans the model says go to it.
  const market = view.focusMarket;
  const walkers = market != null ? new Set(model.walkers.get(market) ?? []) : null;
  ctx.setLineDash([Math.max(px * 6, 0.5), Math.max(px * 4, 0.35)]);
  for (const casing of [true, false]) {
    for (const c of clans) {
      const walks = walkers != null && walkers.has(c.id);
      const bright = view.focus === c.id || walks;
      ctx.globalAlpha = view.focus != null ? (view.focus === c.id ? 1 : 0.25) : market == null || walks ? 1 : 0.12;
      const w = bright ? 1.6 : 1;
      ctx.lineWidth = (casing ? Math.max(px * 5, 0.25) : Math.max(px * 2.6, 0.14)) * w;
      ctx.strokeStyle = casing ? 'rgba(0,0,0,0.5)' : hue(c.id);
      ctx.beginPath();
      walkOf(c).forEach((i, n) => {
        if (n === 0) ctx.moveTo(view.px[i], view.py[i]);
        else ctx.lineTo(view.px[i], view.py[i]);
      });
      ctx.stroke();
    }
  }
  ctx.setLineDash([]);
  ctx.globalAlpha = 1;
}
