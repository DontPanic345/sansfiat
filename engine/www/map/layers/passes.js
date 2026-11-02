// Passes: a pale notch on the ridge, from the atlas's pass icon (a plain dot where the map is too small).
import { ICON_WORLD } from '../../assets/placeholders.js';

export const name = 'passes';
const isStatic = true;
export { isStatic as static };

export function draw(ctx, model, view) {
  const { l } = view;
  const k = view.k;
  if (k < 5) return;
  const s = ICON_WORLD['icon.pass'];
  const c = view.atlas.cell('icon.pass');
  for (let a = 0; a < view.nvis; a++) {
    const i = view.vis[a];
    if (l.pass[i]) ctx.drawImage(view.atlas.image, c.x, c.y, c.w, c.h, view.px[i] + 0.3 - s / 2, view.py[i] - s / 2, s, s);
  }
}
