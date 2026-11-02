// Capitals: a house in the civ's colour, at least 14 px wide so it shows zoomed out.
import { tinted, icon } from './icon.js';

export const name = 'capitals';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  for (const civ of Object.values(model.v.civs || {})) {
    if (!view.onScreen(civ.capital)) continue;
    const s = Math.max(0.95, px * 8); // half the house's width
    const w = s * (64 / 22); // the cell's house is 22 of its 64 px per half-width
    const x = view.px[civ.capital], y = view.py[civ.capital] - 0.075 * s;
    tinted(ctx, view, 'icon.capital.mask', civ.color, x, y, w);
    icon(ctx, view, 'icon.capital', x, y, w);
  }
}
