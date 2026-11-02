// Lines home: the led civ's camp's line, always, thin and faint; the selected camp's line solid in its
// colour, edged dark; and where held ground cuts a line, a cross.
export const name = 'lines';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const casing = 'rgba(0,0,0,0.8)';
  const path = (tiles) => {
    ctx.beginPath();
    tiles.forEach((i, n) => (n === 0 ? ctx.moveTo(view.px[i], view.py[i]) : ctx.lineTo(view.px[i], view.py[i])));
    ctx.stroke();
  };
  const { line, homeLine } = view;
  if (line && line.cut != null) {
    const x = view.px[line.cut], y = view.py[line.cut];
    const s = 0.55;
    for (const [c, w] of [[casing, 7], ['#ff4d4d', 4]]) {
      ctx.strokeStyle = c;
      ctx.lineWidth = Math.max(px * w, 0.12);
      ctx.beginPath();
      ctx.moveTo(x - s, y - s);
      ctx.lineTo(x + s, y + s);
      ctx.moveTo(x + s, y - s);
      ctx.lineTo(x - s, y + s);
      ctx.stroke();
    }
  }
  if (homeLine && homeLine.length > 1) {
    ctx.globalAlpha = 0.45;
    ctx.setLineDash([]);
    ctx.strokeStyle = '#e8f0ff';
    ctx.lineWidth = Math.max(px * 1.5, 0.06);
    path(homeLine);
    ctx.globalAlpha = 1;
  }
  if (line && line.tiles.length > 1) {
    ctx.globalAlpha = 1;
    ctx.setLineDash([]);
    for (const [c, w] of [[casing, 7], [line.colour, 4]]) {
      ctx.strokeStyle = c;
      ctx.lineWidth = Math.max(px * w, 0.12);
      path(line.tiles);
    }
  }
}
