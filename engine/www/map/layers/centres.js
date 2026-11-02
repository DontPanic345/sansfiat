// Centres: the three festivals picked out by a gold ring, over the clans so they can be found on a crowded map.
import { icon } from './icon.js';
import { ICON_WORLD } from '../../assets/placeholders.js';

export const name = 'centres';

export function draw(ctx, model, view) {
  for (const t of view.centres) {
    if (view.onScreen(t)) icon(ctx, view, 'icon.centre', view.px[t], view.py[t], ICON_WORLD['icon.centre']);
  }
}
