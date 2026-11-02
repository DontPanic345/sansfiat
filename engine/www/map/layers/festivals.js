// Festivals: a ring on each festival's tile.
import { icon } from './icon.js';
import { ICON_WORLD } from '../../assets/placeholders.js';

export const name = 'festivals';

export function draw(ctx, model, view) {
  for (const m of Object.values(model.v.markets)) {
    if (view.onScreen(m.tile)) icon(ctx, view, 'icon.festival', view.px[m.tile], view.py[m.tile], ICON_WORLD['icon.festival']);
  }
}
