// Fields: three short furrows in a tile's lower half, on every tile a clan works as fields this turn. They
// grow darker with the tile's field days, from 0.35 at one day to 0.9 at its whole ground. Drawn over the
// terrain and under the territories. The atlas's `overlay.fields` cell is used when it is drawn; the
// strokes are the fallback, as floodplains.js falls back.
export const name = 'fields';

const INK = '#7a5a1e';
/** The furrows in a pointy-top hex of circumradius 1: their height below the centre and their half-length. */
const FURROWS = [0.18, 0.4, 0.62].map((y) => ({ y, half: 0.5 }));

/** The opacity of a tile with `days` field days of `ground` a turn. */
export function opacity(days, ground) {
  const t = ground > 1 ? Math.min(1, Math.max(0, (days - 1) / (ground - 1))) : 1;
  return 0.35 + 0.55 * t;
}

export function draw(ctx, model, view) {
  const cell = view.atlas && view.stats.atlas && view.atlas.json.cells['overlay.fields'] ? view.atlas.cell('overlay.fields') : null;
  const s = 2 * 1.02;
  ctx.strokeStyle = INK;
  ctx.lineWidth = Math.max(0.12, (1 / view.k) * 2.4);
  ctx.lineCap = 'round';
  for (const k of Object.values(model.v.clans)) {
    for (const [i, days] of k.plots) {
      if (!view.onScreen(i)) continue;
      const x = view.px[i], y = view.py[i];
      ctx.globalAlpha = opacity(days, view.l.ground[i]);
      if (cell) {
        ctx.drawImage(view.atlas.image, cell.x, cell.y, cell.w, cell.h, x - (s * cell.w) / cell.h / 2, y - s / 2, (s * cell.w) / cell.h, s);
        continue;
      }
      ctx.beginPath();
      for (const f of FURROWS) {
        ctx.moveTo(x - f.half, y + f.y);
        ctx.lineTo(x + f.half, y + f.y);
      }
      ctx.stroke();
    }
  }
  ctx.globalAlpha = 1;
}
