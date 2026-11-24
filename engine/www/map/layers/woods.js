// The woods: a tree or a reed on each slot of a forest or marsh tile (`map/woods.js` says which, by what rule), drawn
// over the terrain at the terrain's scale, after the shores and before the ridges. A static layer, redrawn with the
// terrain when the view moves, and when the field tiles on forest and marsh change (`stale`). It draws nothing while
// the terrain is flat or its cell is under `WOODS.from` high. A piece whose drawn rectangle misses the canvas is
// skipped, unless `view.noCull` (the culling check draws both ways and compares).
import { HEX_R } from '../../assets/placeholders.js';
import { WOODS, woodPieces } from '../woods.js';
import { cellHeight, isFlat } from './terrain.js';

export const name = 'woods';
const isStatic = true;
export { isStatic as static };

/** A terrain cell's layout height in cell px: the pieces scale from it to the terrain's drawn height. */
const TERRAIN_CELL_H = 2 * HEX_R;

/** The field tiles that lie on a forest or marsh tile, sorted: the clans' plots this turn, or none without a model. */
const fieldTiles = (model, view) => {
  if (!model) return [];
  const { terrain } = view.l;
  const tiles = new Set();
  for (const c of Object.values(model.v.clans)) {
    for (const [i] of c.plots) if (terrain[i] === view.byName.forest || terrain[i] === view.byName.marsh) tiles.add(i);
  }
  return [...tiles].sort((a, b) => a - b);
};

/** Keeps the fields key for `model` on the view and says whether it differs from the one kept before. The layer draws
 *  from the kept key, so it is kept on every turn, drawn or not. */
export function stale(model, view) {
  const tiles = fieldTiles(model, view);
  const key = tiles.join(',');
  const changed = key !== view.woodsKey;
  view.woodsKey = key;
  view.woodsFields = tiles;
  return changed;
}

/** Each map's pieces, for the latest fields key: the cache is keyed by the view's neighbour table, which `setLayout`
 *  makes new per map, and holds the one key that map has drawn last. */
const cache = new WeakMap();
const piecesOf = (view) => {
  const hit = cache.get(view.nb);
  if (hit && hit.key === view.woodsKey) return hit.list;
  const { terrain, river, flow } = view.l;
  const list = woodPieces({
    terrain, river, flow, nb: view.nb, px: view.px, py: view.py,
    forest: view.byName.forest, marsh: view.byName.marsh, fields: new Set(view.woodsFields),
  });
  cache.set(view.nb, { key: view.woodsKey, list });
  return list;
};

export function draw(ctx, model, view) {
  view.stats.woods = 0;
  const H = cellHeight(view); // the terrain cell's drawn height, in device px
  if (isFlat(H, view) || H < WOODS.from) return;
  const { atlas, dpr, k } = view;
  const [X0, Y0, X1, Y1] = view.box();
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
    if (!view.noCull && (x + img.width <= X0 || y + h <= Y0 || x >= X1 || y >= Y1)) continue;
    ctx.drawImage(img, x, y);
    view.stats.woods++;
  }
}
