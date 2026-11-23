// Cuts an atlas cell from a source region: pure (no I/O), deterministic, every result rounded the same
// way on any machine. See `sources.js` for the regions and cells, and `draw-atlas.mjs` for the build.
import { HEX_R } from './placeholders.js';

const SS = 4; // samples per pixel, each way: a pixel's alpha is its hex's coverage in 16ths

/** The cell's anchor in cell px, after its ops: the point of the cell that sits on the tile's centre. */
export function anchorOf(cell, { w, h }) {
  let [ax, ay] = Array.isArray(cell?.anchor) ? cell.anchor : [w / 2, h / 2];
  if (cell?.ops?.includes('mirror')) ax = w - ax;
  return [ax, ay];
}

/**
 * A copy of the source's RGB over the region's box (its hex's bounding box, widened to hold every
 * patch's rectangles), with the patches applied: `{ x0, y0, w, h, rgb }`.
 */
function patched(source, region) {
  const { cx, cy, rx, ry } = region.hex;
  const rects = [[cx - rx, cy - ry, cx + rx, cy + ry]];
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

/**
 * Cuts `cell` (an atlas cell spec: `{ ops, anchor }`) of size `{ w, h }` from `region` in `source`
 * (`{ width, height, data }`, RGBA8). Returns `{ data, ax, ay, flat }`: `data` RGBA8 for the cell, the
 * anchor in cell px, and `flat`, the mean colour of its opaque pixels as `#rrggbb`.
 */
export function cut(source, region, cell, { w, h }) {
  const box = patched(source, region);
  const R = HEX_R, A = (R * Math.sqrt(3)) / 2; // the cell hex's half-width and circumradius, in cell px
  const { cx, cy } = region.hex;
  const rx = region.hex.rx - region.inset, ry = region.hex.ry - region.inset;
  const data = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let r = 0, g = 0, b = 0, n = 0;
      for (let j = 0; j < SS; j++) {
        for (let i = 0; i < SS; i++) {
          const dx = x + (i + 0.5) / SS - w / 2, dy = y + (j + 0.5) / SS - h / 2;
          const u = dx / A, v = dy / R;
          if (Math.abs(u) > 1 || Math.abs(v) + Math.abs(u) / 2 > 1) continue; // outside the pointy-top hex
          // The cell's hex maps linearly onto the region's hex, shrunk by the inset.
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
    if (data[o + 3] !== 255) continue;
    r += data[o];
    g += data[o + 1];
    b += data[o + 2];
    n++;
  }
  if (!n) throw new Error('the cut has no opaque pixel');
  const hex = (v) => Math.round(v / n).toString(16).padStart(2, '0');
  const [ax, ay] = anchorOf(cell, { w, h });
  return { data, ax, ay, flat: `#${hex(r)}${hex(g)}${hex(b)}` };
}
