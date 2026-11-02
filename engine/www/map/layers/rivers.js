// Rivers: a line from each river tile's centre to its flow tile's centre; wider downstream.
export const name = 'rivers';
const isStatic = true;
export { isStatic as static };

export function draw(ctx, model, view) {
  const { l } = view;
  const k = view.k;
  const px = 1 / k;
  const classes = [[], [], [], [], [], [], []];
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (l.river[i] > 0 && l.flow[i] >= 0) classes[Math.min(6, Math.floor(Math.sqrt(l.river[i])))].push(i);
  }
  const width = (c) => Math.max(px * 1.3, 0.14 + 0.06 * c);
  for (const pass of [0, 1]) {
    if (pass === 0 && k < 9) continue;
    ctx.strokeStyle = pass === 0 ? '#1d5f96' : '#58b8f2';
    classes.forEach((list, c) => {
      if (!list.length) return;
      ctx.lineWidth = width(c) + (pass === 0 ? px * 1.6 : 0);
      ctx.beginPath();
      for (const i of list) {
        const j = l.flow[i];
        const x = view.px[i], y = view.py[i];
        // Into the sea a little past the shore, so a mouth opens into the water.
        const f = l.terrain[j] === 0 ? 0.62 : 1;
        ctx.moveTo(x, y);
        ctx.lineTo(x + (view.px[j] - x) * f, y + (view.py[j] - y) * f);
      }
      ctx.stroke();
    });
  }
}
