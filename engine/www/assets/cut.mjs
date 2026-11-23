// Cuts an atlas cell from a source region: pure (no I/O), deterministic, every result rounded the same
// way on any machine. See `sources.js` for the regions and cells, and `draw-atlas.mjs` for the build.
//
// Two kinds of region. A hex region is a hex in the source, cut at the cell's hex size. A sprite region
// (`rect`) is a rectangle of the source with a keep-polygon, cut at a scale into the cell with its ground
// point on the cell's anchor, and a key that clears what is not the sprite (grass, the tufts).
import { HEX_R } from './placeholders.js';

const SS = 4; // samples per pixel, each way: a pixel's alpha is its hex's coverage in 16ths
// A cell's opaque pixels for `flat`: a hex's full cover is 255, a sprite source's interior 250 and up.
const OPAQUE = 250;

/**
 * The keys a sprite region may name: each clears the source's pixels that are not the sprite. Thresholds
 * sit with their key, and a key's note says why.
 */
export const KEYS = {
  ground: {
    grassRed: 6, // grass is green over red and blue: g > r + 6 and g > b + 25
    grassBlue: 25,
    tuftBlue: 0.6, // yellow tufts and the hill under them have little blue: b < 0.6 g
    note: 'grass, and the yellow tufts and hill under the peaks',
    clears(r, g, b) {
      return (g > r + this.grassRed && g > b + this.grassBlue) || b < this.tuftBlue * g;
    },
  },
};

/** The cell's anchor before its ops, in cell px: the cell spec's, else the layout cell's, else the centre. */
const baseAnchor = (cell, size) => {
  if (Array.isArray(cell?.anchor)) return cell.anchor;
  if (size.ax !== undefined) return [size.ax, size.ay];
  return [size.w / 2, size.h / 2];
};

/** The cell's anchor in cell px, after its ops: the point of the cell that sits on the tile's centre. */
export function anchorOf(cell, size) {
  let [ax, ay] = baseAnchor(cell, size);
  if (cell?.ops?.includes('mirror')) ax = size.w - ax;
  return [ax, ay];
}

/** A region's source rectangle: its sprite rect, or its hex's bounding box. */
const rectOf = (region) => {
  if (region.rect) {
    const [x, y, w, h] = region.rect;
    return [x, y, x + w, y + h];
  }
  const { cx, cy, rx, ry } = region.hex;
  return [cx - rx, cy - ry, cx + rx, cy + ry];
};

/**
 * A copy of the source's RGB over the region's box (its rectangle, widened to hold every patch's
 * rectangles), with the patches applied: `{ x0, y0, w, h, rgb }`.
 */
function patched(source, region) {
  const rects = [rectOf(region)];
  for (const p of region.patches ?? []) {
    for (const [px, py] of [p.from, p.to]) rects.push([px, py, px + p.w, py + p.h]);
  }
  const x0 = Math.floor(Math.min(...rects.map((r) => r[0])));
  const y0 = Math.floor(Math.min(...rects.map((r) => r[1])));
  const w = Math.ceil(Math.max(...rects.map((r) => r[2]))) - x0;
  const h = Math.ceil(Math.max(...rects.map((r) => r[3]))) - y0;
  if (x0 < 0 || y0 < 0 || x0 + w > source.width || y0 + h > source.height) throw new Error(`region lies outside its source`);
  const rgb = new Uint8Array(w * h * 3);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const s = ((y0 + y) * source.width + x0 + x) * 4, d = (y * w + x) * 3;
      rgb[d] = source.data[s];
      rgb[d + 1] = source.data[s + 1];
      rgb[d + 2] = source.data[s + 2];
    }
  }
  const box = { x0, y0, w, h, rgb };
  // Each patch reads its source from the copy as the patches before it left it.
  for (const p of region.patches ?? []) {
    const before = Uint8Array.from(rgb);
    const [fx, fy] = p.from, [tx, ty] = p.to;
    for (let v = 0; v < p.h; v++) {
      for (let u = 0; u < p.w; u++) {
        // Distance in px to the nearest edge of the destination rectangle, feathered over `feather` px.
        const d = Math.min(u + 0.5, v + 0.5, p.w - u - 0.5, p.h - v - 0.5);
        const t = Math.min(1, d / p.feather);
        const dst = ((ty - y0 + v) * w + (tx - x0 + u)) * 3;
        const src = ((fy - y0 + v) * w + (fx - x0 + u)) * 3;
        for (let c = 0; c < 3; c++) rgb[dst + c] = Math.round(before[dst + c] * (1 - t) + before[src + c] * t);
      }
    }
  }
  return box;
}

/** A hex region's cell: each sample inside the cell's hex maps onto the region's hex, shrunk by the inset. */
function hexData(box, region, { w, h }) {
  const R = HEX_R, A = (R * Math.sqrt(3)) / 2; // the cell hex's half-width and circumradius, in cell px
  // The inset drops the drawn outline; the right edge may take more (`insetRight`), which moves the centre.
  const left = region.inset, right = region.insetRight ?? region.inset;
  const cx = region.hex.cx + (left - right) / 2, cy = region.hex.cy;
  const rx = region.hex.rx - (left + right) / 2, ry = region.hex.ry - region.inset;
  const data = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let r = 0, g = 0, b = 0, n = 0;
      for (let j = 0; j < SS; j++) {
        for (let i = 0; i < SS; i++) {
          const dx = x + (i + 0.5) / SS - w / 2, dy = y + (j + 0.5) / SS - h / 2;
          const u = dx / A, v = dy / R;
          if (Math.abs(u) > 1 || Math.abs(v) + Math.abs(u) / 2 > 1) continue; // outside the pointy-top hex
          const sx = Math.floor(cx + u * rx), sy = Math.floor(cy + v * ry);
          const bx = sx - box.x0, by = sy - box.y0;
          if (bx < 0 || by < 0 || bx >= box.w || by >= box.h) throw new Error(`sample (${sx}, ${sy}) lies outside the region`);
          const s = (by * box.w + bx) * 3;
          r += box.rgb[s];
          g += box.rgb[s + 1];
          b += box.rgb[s + 2];
          n++;
        }
      }
      const o = (y * w + x) * 4;
      if (n) {
        data[o] = Math.round(r / n);
        data[o + 1] = Math.round(g / n);
        data[o + 2] = Math.round(b / n);
        data[o + 3] = Math.round((255 * n) / (SS * SS));
      }
    }
  }
  return data;
}

/**
 * A sprite region's cell: a cell sample maps to source `ground + (cell − anchor) / scale`, and counts when it
 * is inside the rectangle and the keep-polygon and the key doesn't clear it. Colour is the mean of the kept
 * samples weighted by their source alpha; alpha is those alphas summed over all the samples.
 */
function spriteData(source, box, region, cell, size) {
  const { w, h } = size;
  const key = KEYS[region.key];
  if (!key) throw new Error(`no key ${region.key}`);
  const [ax, ay] = baseAnchor(cell, size);
  const [gx, gy] = region.ground, scale = region.scale;
  const [rx0, ry0, rx1, ry1] = rectOf(region);
  const inside = (x, y, p) => {
    let c = false;
    for (let i = 0, j = p.length - 1; i < p.length; j = i++) {
      const [xi, yi] = p[i], [xj, yj] = p[j];
      if ((yi > y) !== (yj > y) && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) c = !c;
    }
    return c;
  };
  const data = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let r = 0, g = 0, b = 0, a = 0;
      for (let j = 0; j < SS; j++) {
        for (let i = 0; i < SS; i++) {
          const sx = Math.floor(gx + (x + (i + 0.5) / SS - ax) / scale);
          const sy = Math.floor(gy + (y + (j + 0.5) / SS - ay) / scale);
          if (sx < rx0 || sx >= rx1 || sy < ry0 || sy >= ry1) continue;
          if (!inside(sx + 0.5, sy + 0.5, region.keep)) continue;
          const bx = sx - box.x0, by = sy - box.y0;
          const s = (by * box.w + bx) * 3;
          const R = box.rgb[s], G = box.rgb[s + 1], B = box.rgb[s + 2];
          if (key.clears(R, G, B)) continue;
          const A = source.data[(sy * source.width + sx) * 4 + 3];
          r += R * A;
          g += G * A;
          b += B * A;
          a += A;
        }
      }
      if (a) {
        const o = (y * w + x) * 4;
        data[o] = Math.round(r / a);
        data[o + 1] = Math.round(g / a);
        data[o + 2] = Math.round(b / a);
        data[o + 3] = Math.round(a / (SS * SS));
      }
    }
  }
  return data;
}

/**
 * Cuts `cell` (an atlas cell spec: `{ ops, anchor }`) of size `size` `{ w, h, ax?, ay? }` from `region` in
 * `source` (`{ width, height, data }`, RGBA8). Returns `{ data, ax, ay, flat }`: `data` RGBA8 for the cell, the
 * anchor in cell px, and `flat`, the mean colour of its opaque pixels as `#rrggbb`.
 */
export function cut(source, region, cell, size) {
  const { w, h } = size;
  const box = patched(source, region);
  const data = region.rect ? spriteData(source, box, region, cell, size) : hexData(box, region, size);
  for (const op of cell.ops ?? []) {
    if (op !== 'mirror') throw new Error(`unknown op ${op}`);
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w / 2; x++) {
        const a = (y * w + x) * 4, b = (y * w + (w - 1 - x)) * 4;
        for (let c = 0; c < 4; c++) [data[a + c], data[b + c]] = [data[b + c], data[a + c]];
      }
    }
  }
  let r = 0, g = 0, b = 0, n = 0;
  for (let o = 0; o < data.length; o += 4) {
    if (data[o + 3] < OPAQUE) continue;
    r += data[o];
    g += data[o + 1];
    b += data[o + 2];
    n++;
  }
  if (!n) throw new Error('the cut has no opaque pixel');
  const hex = (v) => Math.round(v / n).toString(16).padStart(2, '0');
  const [ax, ay] = anchorOf(cell, size);
  return { data, ax, ay, flat: `#${hex(r)}${hex(g)}${hex(b)}` };
}
