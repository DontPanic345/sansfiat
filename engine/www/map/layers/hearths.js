// Hearths: a small yellow square on each tile tamed long enough.
import { icon } from './icon.js';
import { ICON_WORLD } from '../../assets/placeholders.js';

export const name = 'hearths';

export function draw(ctx, model, view) {
  for (const i of view.hearths) if (view.onScreen(i)) icon(ctx, view, 'icon.hearth', view.px[i], view.py[i], ICON_WORLD['icon.hearth']);
}
