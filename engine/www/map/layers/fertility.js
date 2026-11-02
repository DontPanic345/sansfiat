// Debug: each land tile filled by its fertility, from brown to green.
export const name = 'fertility';
const isStatic = true;
export { isStatic as static };

const fertilityColor = (f) => {
  const y = Math.max(0, Math.min(1, (f - 0.4) / 1.6));
  return `hsl(${30 + 90 * y} 60% ${25 + 30 * y}%)`;
};
const BUCKETS = 16;
const bucket = (f) => Math.max(0, Math.min(BUCKETS - 1, Math.floor(((f - 0.4) / 1.6) * BUCKETS)));

export function draw(ctx, model, view) {
  const { l } = view;
  const lists = new Map();
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (!view.land[i]) continue;
    const b = bucket(l.fertility[i]);
    let list = lists.get(b);
    if (!list) lists.set(b, (list = []));
    list.push(i);
  }
  for (const [b, list] of lists) {
    ctx.fillStyle = fertilityColor(0.4 + ((b + 0.5) / BUCKETS) * 1.6);
    ctx.beginPath();
    for (const i of list) view.hexAt(ctx, i, 1.03);
    ctx.fill();
  }
}
