// Mountains: a peak on each mountain tile and a ridge on each shared mountain edge (`map/ridges.js` says
// which, in what order), drawn over the terrain at the terrain's scale. A static layer, redrawn with the
// terrain when the view moves. A piece whose drawn rectangle misses the canvas is skipped, unless
// `view.noCull` (the culling check draws both ways and compares).
import { HEX_R } from '../../assets/placeholders.js';
import { ridgePieces } from '../ridges.js';
import { cellHeight, isFlat } from './terrain.js';

export const name = 'ridges';
const isStatic = true;
export { isStatic as static };

/** A terrain cell's layout height in cell px: the pieces scale from it to the terrain's drawn height. */
const TERRAIN_CELL_H = 2 * HEX_R;

/** Each map's pieces, built once. The key is the view's neighbour table, which `setLayout` makes new per map. */
const pieces = new WeakMap();
const piecesOf = (view) => {
  let p = pieces.get(view.nb);
  if (!p) {
    p = ridgePieces({ terrain: view.l.terrain, nb: view.nb, px: view.px, py: view.py, mountain: view.byName.mountain });
    pieces.set(view.nb, p);
  }
  return p;
};

export function draw(ctx, model, view) {
  view.stats.ridges = 0;
  const H = cellHeight(view); // the terrain cell's drawn height, in device px
  if (isFlat(H, view)) return;
  const { atlas, dpr, k } = view;
  const W = view.canvas.width, Hc = view.canvas.height;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  for (const p of piecesOf(view)) {
    const cell = atlas.cell(p.key);
    // The piece's cell at the terrain's scale, and its anchor (the ground point) scaled to match.
    const h = Math.round((cell.h * H) / TERRAIN_CELL_H);
    const img = atlas.scaled(p.key, h);
    const ax = ((cell.ax ?? cell.w / 2) * img.width) / cell.w;
    const ay = ((cell.ay ?? cell.h / 2) * h) / cell.h;
    const x = Math.round(((p.x - view.cx) * k + view.w / 2) * dpr - ax);
    const y = Math.round(((p.y - view.cy) * k + view.h / 2) * dpr - ay);
    if (!view.noCull && (x + img.width <= 0 || y + h <= 0 || x >= W || y >= Hc)) continue;
    ctx.drawImage(img, x, y);
    view.stats.ridges++;
  }
}
