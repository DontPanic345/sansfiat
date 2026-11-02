// What an event or the panel points at (a set of tiles, a clan with its route, a festival, a tile), and
// the selected tile's outline.
import { walkOf } from '../geom.js';

export const name = 'overlay';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const hl = view.highlight;
  if (hl) {
    let tile = -1;
    if (hl.tiles) {
      // A set of tiles (the pick's "Show": a civ's land).
      ctx.fillStyle = 'rgba(255,255,255,0.22)';
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = Math.max(px * 1.5, 0.08);
      for (const i of hl.tiles) {
        if (!view.onScreen(i)) continue;
        ctx.beginPath();
        view.hexAt(ctx, i, 0.96);
        ctx.fill();
        ctx.stroke();
      }
    }
    if (hl.tile != null) tile = hl.tile;
    if (hl.clan != null && v.clans[hl.clan]) {
      const c = v.clans[hl.clan];
      tile = c.home;
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = Math.max(px * 2.6, 0.2);
      ctx.beginPath();
      walkOf(c).forEach((i, n) => (n === 0 ? ctx.moveTo(view.px[i], view.py[i]) : ctx.lineTo(view.px[i], view.py[i])));
      ctx.stroke();
    } else if (hl.market != null && v.markets[hl.market]) {
      tile = v.markets[hl.market].tile;
    }
    if (tile >= 0) {
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = Math.max(px * 3, 0.25);
      ctx.beginPath();
      ctx.arc(view.px[tile], view.py[tile], 2.1, 0, 2 * Math.PI);
      ctx.stroke();
      ctx.strokeStyle = '#000';
      ctx.lineWidth = Math.max(px, 0.06);
      ctx.beginPath();
      ctx.arc(view.px[tile], view.py[tile], 2.1 + Math.max(px * 1.6, 0.12), 0, 2 * Math.PI);
      ctx.stroke();
    }
  }
  if (view.selected >= 0) {
    ctx.strokeStyle = '#fff';
    ctx.lineWidth = Math.max(px * 2.5, 0.12);
    ctx.beginPath();
    view.hexAt(ctx, view.selected, 1);
    ctx.stroke();
    ctx.strokeStyle = '#000';
    ctx.lineWidth = Math.max(px, 0.04);
    ctx.beginPath();
    view.hexAt(ctx, view.selected, 1 + Math.max(px * 1.6, 0.06));
    ctx.stroke();
  }
}
