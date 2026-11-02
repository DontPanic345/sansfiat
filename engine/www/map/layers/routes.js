// A unit's route: a line along the path with a flag at the end (the selected one firm, the rest faint),
// and the selected scout's plan: known solid, possible dashed, both from where its search starts.
export const name = 'routes';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const casing = 'rgba(0,0,0,0.8)';
  const flag = (x, y, col) => {
    ctx.beginPath();
    ctx.moveTo(x, y);
    ctx.lineTo(x, y - 1.3);
    ctx.lineTo(x + 0.8, y - 1);
    ctx.lineTo(x, y - 0.7);
    ctx.fillStyle = col;
    ctx.fill();
    ctx.stroke();
  };
  const trace = (tiles) => {
    ctx.beginPath();
    tiles.forEach((i, n) => (n === 0 ? ctx.moveTo(view.px[i], view.py[i]) : ctx.lineTo(view.px[i], view.py[i])));
    ctx.stroke();
  };
  for (const sel of [false, true]) {
    for (const r of view.routes) {
      if (r.selected !== sel || r.tiles.length < 2) continue;
      const col = v.civs[r.civ]?.color || '#fff';
      ctx.setLineDash([Math.max(px * 6, 0.5), Math.max(px * 4, 0.35)]);
      ctx.globalAlpha = sel ? 1 : 0.55;
      for (const [c, w] of [[casing, sel ? 5 : 3.5], [sel ? '#fff' : col, sel ? 2.6 : 1.8]]) {
        ctx.strokeStyle = c;
        ctx.lineWidth = Math.max(px * w, 0.1);
        trace(r.tiles);
      }
      ctx.setLineDash([]);
      const end = r.tiles[r.tiles.length - 1];
      ctx.lineWidth = Math.max(px * 1.4, 0.06);
      ctx.strokeStyle = casing;
      flag(view.px[end], view.py[end], sel ? '#fff' : col);
      ctx.globalAlpha = 1;
    }
  }
  if (view.plan) {
    const { known, possible } = view.plan;
    const draw = (tiles, dash, colour, w) => {
      ctx.setLineDash(dash);
      for (const [c, lw] of [[casing, w + 2], [colour, w]]) {
        ctx.strokeStyle = c;
        ctx.lineWidth = Math.max(px * lw, 0.1);
        trace(tiles);
      }
      ctx.setLineDash([]);
    };
    // Known first, the possible dashed over it: where the two are one path, both still show.
    if (known && known.length > 1) draw(known, [], '#7fd3ff', 4);
    if (possible && possible.length > 1) draw(possible, [Math.max(px * 6, 0.5), Math.max(px * 4, 0.35)], '#ffb347', 3);
  }
}
