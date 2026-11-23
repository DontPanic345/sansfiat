// Terrain: each tile's atlas cell (variant `i % 3`, so a map always looks the same), the desert's stipple
// over it, and a hex outline when zoomed in. Where a cell is only a few pixels across the tile is a flat
// fill, one path per colour.
import { TERRAIN_COLOR } from '../../assets/placeholders.js';

export const name = 'terrain';
const isStatic = true;
export { isStatic as static };

/** Below this cell height in device px the tile is a flat fill. */
const FLAT_BELOW = 12;
const OVER = 1.03; // a tile is drawn this much wider than its hex, so neighbours overlap

/** A terrain cell's height on the canvas, in device px: the scale the other atlas pieces are drawn at too. */
export const cellHeight = (view) => Math.round(2 * OVER * view.k * view.dpr);
/** Whether the tiles are flat fills at cell height `H`: too small to draw cells, or no atlas. */
export const isFlat = (H, view) => H < FLAT_BELOW || !view.atlas;

/** A flat fill's colour for terrain `key`: its cell's mean, else the placeholder colour. A mountain's is its
 *  peak's rock, so a range reads as grey at the flat zoom. */
const flatColour = (view, key) => {
  const cells = view.atlas?.json.cells;
  if (key === 'mountain') return cells?.['mountain.peak.0']?.flat || cells?.['terrain.mountain.0']?.flat || TERRAIN_COLOR.mountain;
  return cells?.[`terrain.${key}.0`]?.flat || TERRAIN_COLOR[key] || '#f0f';
};

export function draw(ctx, model, view) {
  const { l } = view;
  const k = view.k;
  const px = 1 / k;
  const kind = (i) => {
    const t = l.terrain[i];
    return t === 0 && l.lake[i] ? 'lake' : view.names[t];
  };
  const H = cellHeight(view);
  const flat = isFlat(H, view);
  view.stats.atlas = !flat;
  if (flat) {
    const buckets = new Map();
    for (let a = 0; a < view.nvis; a++) {
      const i = view.vis[a];
      const key = kind(i);
      let b = buckets.get(key);
      if (!b) buckets.set(key, (b = []));
      b.push(i);
    }
    for (const [key, list] of buckets) {
      ctx.fillStyle = flatColour(view, key);
      ctx.beginPath();
      for (const i of list) view.hexAt(ctx, i, OVER);
      ctx.fill();
    }
    view.stats.cells = 0;
  } else {
    // Whole device pixels, from pre-scaled cells: one drawImage per tile, with no resampling.
    const { atlas, dpr } = view;
    const desert = view.byName.desert;
    const dots = k >= 9 ? atlas.scaled('overlay.desert-dots', H) : null;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    // Each cell's anchor, scaled to the drawn size, lands on the tile's centre; read once per draw.
    const placed = new Map();
    const place = (name) => {
      let p = placed.get(name);
      if (!p) {
        const cell = atlas.cell(name);
        const img = atlas.scaled(name, H);
        p = { img, ax: ((cell.ax ?? cell.w / 2) * img.width) / cell.w, ay: ((cell.ay ?? cell.h / 2) * H) / cell.h };
        placed.set(name, p);
      }
      return p;
    };
    let n = 0;
    for (let a = 0; a < view.nvis; a++) {
      const i = view.vis[a];
      const { img, ax, ay } = place(`terrain.${kind(i)}.${i % 3}`);
      const x = Math.round(((view.px[i] - view.cx) * k + view.w / 2) * dpr - ax);
      const y = Math.round(((view.py[i] - view.cy) * k + view.h / 2) * dpr - ay);
      ctx.drawImage(img, x, y);
      if (dots && l.terrain[i] === desert) ctx.drawImage(dots, x, y);
      n++;
    }
    view.stats.cells = n;
    view.setWorld(ctx);
  }
  if (k >= 14) {
    ctx.strokeStyle = 'rgba(0,0,0,0.22)';
    ctx.lineWidth = px;
    ctx.beginPath();
    for (let a = 0; a < view.nvis; a++) view.hexAt(ctx, view.vis[a], 1);
    ctx.stroke();
  }
}
