// The shores and fringes: the bank along each land–water edge and round each shore vertex, and the fringe where two
// land kinds meet (`map/edges.js` says which, in what order), drawn over the terrain at the terrain's scale. A static
// layer, redrawn with the terrain when the view moves. A piece whose drawn rectangle misses the canvas is skipped,
// unless `view.noCull` (the culling check draws both ways and compares).
import { HEX_R } from '../../assets/placeholders.js';
import { edgePieces } from '../edges.js';
import { cellHeight, isFlat } from './terrain.js';

export const name = 'edges';
const isStatic = true;
export { isStatic as static };

/** A terrain cell's layout height in cell px: the pieces scale from it to the terrain's drawn height. */
const TERRAIN_CELL_H = 2 * HEX_R;

/** Each map's pieces, built once. The key is the view's neighbour table, which `setLayout` makes new per map. */
const pieces = new WeakMap();
const piecesOf = (view) => {
  let p = pieces.get(view.nb);
  if (!p) {
    const { terrain, lake, river, flow } = view.l;
    p = edgePieces({ terrain, lake, river, flow, nb: view.nb, px: view.px, py: view.py, names: view.names });
    pieces.set(view.nb, p);
  }
  return p;
};

export function draw(ctx, model, view) {
  view.stats.edges = 0;
  const H = cellHeight(view); // the terrain cell's drawn height, in device px
  if (isFlat(H, view)) return;
  const { atlas, dpr, k } = view;
  const W = view.canvas.width, Hc = view.canvas.height;
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  // Each key's image at the terrain's scale, and its anchor scaled to match, read once per draw.
  const placed = new Map();
  const place = (key) => {
    let p = placed.get(key);
    if (!p) {
      const cell = atlas.cell(key);
      const h = Math.round((cell.h * H) / TERRAIN_CELL_H);
      const img = atlas.scaled(key, h);
      p = { img, h, ax: ((cell.ax ?? cell.w / 2) * img.width) / cell.w, ay: ((cell.ay ?? cell.h / 2) * h) / cell.h };
      placed.set(key, p);
    }
    return p;
  };
  for (const piece of piecesOf(view)) {
    const { img, h, ax, ay } = place(piece.key);
    const x = Math.round(((piece.x - view.cx) * k + view.w / 2) * dpr - ax);
    const y = Math.round(((piece.y - view.cy) * k + view.h / 2) * dpr - ay);
    if (!view.noCull && (x + img.width <= 0 || y + h <= 0 || x >= W || y >= Hc)) continue;
    ctx.drawImage(img, x, y);
    view.stats.edges++;
  }
}
