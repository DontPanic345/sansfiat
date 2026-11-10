// Towns: a town mark at the top of each festival tile that reads as a town (several clans live on it).
import { icon } from './icon.js';
import { ICON_WORLD } from '../../assets/placeholders.js';

export const name = 'towns';

export function draw(ctx, model, view) {
  for (const t of model.v.towns ?? []) {
    if (view.onScreen(t.tile)) icon(ctx, view, 'icon.town', view.px[t.tile], view.py[t.tile] - 0.7, ICON_WORLD['icon.town']);
  }
}
