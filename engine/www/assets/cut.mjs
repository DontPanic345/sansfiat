// Cuts an atlas cell from a source region: pure (no I/O), deterministic, every result rounded the same
// way on any machine. See `sources.js` for the regions and cells, and `draw-atlas.mjs` for the build.
//
// Three kinds of region. A hex region is a hex in the source, cut at the cell's hex size. A fill region
// (`fill`) is a hex whose samples read a source rectangle repeated, every other copy mirrored. A sprite region
// (`rect`) is a rectangle of the source with a keep-polygon, cut at a scale into the cell with its ground
// point on the cell's anchor (turned by `rotate`), and a key that clears what is not the sprite. Ops then
// recolour (`ramp`) or flip (`mirror`) the cut; a derived cell (`derive`) masks another cell's cut to a fringe.
import { HEX_R, TERRAIN_CELL, fringeBox, fringeMask } from './placeholders.js';

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
    tuftBlue: 0.72, // yellow tufts and the hill under them have little blue: b < 0.72 g
    note: 'grass, and the yellow tufts and hill under the peaks',
    clears(r, g, b) {
      return (g > r + this.grassRed && g > b + this.grassBlue) || b < this.tuftBlue * g;
    },
  },
  bank: {
    minAlpha: 200, // a sample this transparent in the source is not bank
    note: 'the shore bank: its tan blocks and their pale rim, not the grass above or the water below',
    clears(r, g, b, a) {
      return a <= this.minAlpha || !(r > 0.9 * g && r > b + 30);
    },
  },
  tree: {
    note: "a whole tree, crown and trunk: no colour clears, as the ground key's tuft test would cut the leaves' yellow, so the keep-polygon leaves the grass at the foot out",
    clears() {
      return false;
    },
  },
  tuft: {
    redShare: 0.98, // a blade is yellow: red at least this share of green (the grass round it is 0.9 to 0.98)
    blueShare: 0.4, // and has little blue: b under this share of g (blades 0.29 to 0.37, grass 0.34 to 0.56)
    note: "the plains hex's yellow tuft blades, not the green grass round them",
    clears(r, g, b) {
      return !(r >= this.redShare * g && b < this.blueShare * g);
    },
  },
};

/** Maps a source coordinate in a fill: copies of `n` px, every other mirrored, so neighbouring copies seam. */
export const fold = (t, n) => {
  const m = ((t % (2 * n)) + 2 * n) % (2 * n);
  return m < n ? m : 2 * n - 1 - m;
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

/** A region's source rectangle: its fill or sprite rect, or its hex's bounding box. */
const rectOf = (region) => {
  if (region.fill) {
    const [x, y, w, h] = region.fill.rect;
    return [x, y, x + w, y + h];
  }
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

/**
 * Each sample inside the cell's hex, and its source pixel: `at(x, y)` gives the source (sx, sy) for the cell
 * sample at (x, y) in cell px; the colour is the mean of those samples and the alpha their hex coverage.
 */
function hexCover(box, { w, h }, at) {
  const R = HEX_R, A = (R * Math.sqrt(3)) / 2; // the cell hex's half-width and circumradius, in cell px
  const data = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let r = 0, g = 0, b = 0, n = 0;
      for (let j = 0; j < SS; j++) {
        for (let i = 0; i < SS; i++) {
          const px = x + (i + 0.5) / SS, py = y + (j + 0.5) / SS;
          const dx = px - w / 2, dy = py - h / 2;
          const u = dx / A, v = dy / R;
          if (Math.abs(u) > 1 || Math.abs(v) + Math.abs(u) / 2 > 1) continue; // outside the pointy-top hex
          const [sx, sy] = at(px, py, u, v);
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

/** A hex region's cell: each sample inside the cell's hex maps onto the region's hex, shrunk by the inset. */
function hexData(box, region, size) {
  // The inset drops the drawn outline; the right edge may take more (`insetRight`), which moves the centre.
  const left = region.inset, right = region.insetRight ?? region.inset;
  const cx = region.hex.cx + (left - right) / 2, cy = region.hex.cy;
  const rx = region.hex.rx - (left + right) / 2, ry = region.hex.ry - region.inset;
  return hexCover(box, size, (px, py, u, v) => [Math.floor(cx + u * rx), Math.floor(cy + v * ry)]);
}

/**
 * A fill region's cell: a sample at cell px (x, y) reads the fill's rectangle at `(x / scale + offset)`, each
 * axis folded so the copies seam. The hex's coverage is the alpha, as for a hex.
 */
function fillData(box, region, size) {
  const [fx, fy, fw, fh] = region.fill.rect;
  const { scale, offset: [ox, oy] } = region.fill;
  return hexCover(box, size, (px, py) => [
    fx + fold(Math.floor(px / scale + ox), fw),
    fy + fold(Math.floor(py / scale + oy), fh),
  ]);
}

/**
 * A sprite region's cell: a cell sample maps to source `ground + Rot(−deg) (cell − anchor) / scale` (its y over `scaleY`, if the
 * region names one, unturned), and counts when it
 * is inside the rectangle and the keep-polygon and the key doesn't clear it. Colour is the mean of the kept
 * samples weighted by their source alpha; alpha is those alphas summed over all the samples.
 */
function spriteData(source, box, region, cell, size) {
  const { w, h } = size;
  const key = KEYS[region.key];
  if (!key) throw new Error(`no key ${region.key}`);
  const [ax, ay] = baseAnchor(cell, size);
  const [gx, gy] = region.ground, scale = region.scale;
  // `scaleY` stretches the sprite vertically; it is unturned, so it cannot come with `rotate` or `shape`.
  const scaleY = region.scaleY ?? scale;
  if (region.scaleY !== undefined && (region.rotate !== undefined || cell.rotate !== undefined || cell.shape)) {
    throw new Error(`region ${region.key} sets scaleY and also rotate or shape`);
  }
  const [rx0, ry0, rx1, ry1] = rectOf(region);
  // The cell turns about its anchor by `rotate` degrees (the cell's, else the region's): Rot(−deg) on the cell offset.
  // `shape` bends the strip: an edge piece stops `half` px either side of its anchor along the edge; a corner
  // piece (`in`: water pokes into land, `out`: land pokes into water) fans or mitres the strip round the vertex.
  const shape = cell.shape ?? null;
  const rad = (deg) => (deg * Math.PI) / 180;
  const straight = (dx, dy, deg) => {
    const cs = Math.cos(rad(deg)), sn = Math.sin(rad(deg));
    return [Math.floor(gx + (cs * dx + sn * dy) / scale), Math.floor(gy + (-sn * dx + cs * dy) / scaleY)];
  };
  const wrap = (a) => ((((a + 180) % 360) + 360) % 360) - 180;
  const locate = (dx, dy) => {
    const deg = cell.rotate ?? region.rotate ?? 0;
    if (!shape) return straight(dx, dy, deg);
    if (shape.type === 'edge') {
      const along = -dx * Math.sin(rad(deg + 90)) + dy * Math.cos(rad(deg + 90));
      return Math.abs(along) > shape.half ? null : straight(dx, dy, deg);
    }
    const r = Math.hypot(dx, dy), phi = (Math.atan2(dy, dx) * 180) / Math.PI;
    const psi = 60 * shape.c - 30; // the screen angle of corner c from its tile's centre
    if (shape.type === 'in') {
      // Water's corner: the bank fans round the vertex between the two edges' outward normals (psi ± 30).
      const delta = wrap(phi - psi);
      if (Math.abs(delta) > shape.span) return null;
      return [Math.floor(gx + (r * rad(delta)) / scale), Math.floor(gy - r / scale)];
    }
    // Land's corner: the two edges' strips meet on the bisector (psi + 180), each serving the half nearer its edge.
    if (Math.abs(wrap(phi - (psi + 180))) > 60 + shape.reach) return null;
    const a1 = psi + 120, a2 = psi - 120;
    const first = Math.abs(wrap(phi - a1)) <= Math.abs(wrap(phi - a2));
    const a = first ? a1 : a2, d = first ? shape.c : (shape.c + 5) % 6;
    const mx = (HEX_R / 2) * Math.cos(rad(a)), my = (HEX_R / 2) * Math.sin(rad(a));
    return straight(dx - mx, dy - my, 60 * d - 90);
  };
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
          const at = locate(x + (i + 0.5) / SS - ax, y + (j + 0.5) / SS - ay);
          if (!at) continue;
          const [sx, sy] = at;
          if (sx < rx0 || sx >= rx1 || sy < ry0 || sy >= ry1) continue;
          if (!inside(sx + 0.5, sy + 0.5, region.keep)) continue;
          const bx = sx - box.x0, by = sy - box.y0;
          const s = (by * box.w + bx) * 3;
          const R = box.rgb[s], G = box.rgb[s + 1], B = box.rgb[s + 2];
          const A = source.data[(sy * source.width + sx) * 4 + 3];
          if (key.clears(R, G, B, A)) continue;
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

const lum = (r, g, b) => 0.299 * r + 0.587 * g + 0.114 * b; // Rec. 601

/** The mean colour of a source rectangle `[x, y, w, h]`, as `[r, g, b]`. */
function meanOf(source, [x, y, w, h]) {
  let r = 0, g = 0, b = 0;
  for (let yy = 0; yy < h; yy++) {
    for (let xx = 0; xx < w; xx++) {
      const s = ((y + yy) * source.width + x + xx) * 4;
      r += source.data[s];
      g += source.data[s + 1];
      b += source.data[s + 2];
    }
  }
  return [r / (w * h), g / (w * h), b / (w * h)];
}

/**
 * The ramp op: each pixel's luminance, normalised over the cell's opaque pixels, picks a colour on the line from
 * `dark` to `light` (the means of two source rectangles), blended with the pixel's own colour by `mix`. `lum: [lo, hi]`
 * replaces the cell's own luminance range.
 */
function ramp(data, source, { dark, light, mix, lum: fixed }) {
  const [dr, dg, db] = meanOf(source, dark), [lr, lg, lb] = meanOf(source, light);
  let lo = Infinity, hi = -Infinity;
  for (let o = 0; o < data.length; o += 4) {
    if (data[o + 3] < OPAQUE) continue;
    const l = lum(data[o], data[o + 1], data[o + 2]);
    lo = Math.min(lo, l);
    hi = Math.max(hi, l);
  }
  if (fixed) [lo, hi] = fixed; // a fixed luminance range keeps a flat variant from stretching to the whole ramp
  const span = hi > lo ? hi - lo : 1;
  for (let o = 0; o < data.length; o += 4) {
    if (!data[o + 3]) continue;
    const t = Math.min(1, Math.max(0, (lum(data[o], data[o + 1], data[o + 2]) - lo) / span));
    const target = [dr + t * (lr - dr), dg + t * (lg - dg), db + t * (lb - db)];
    for (let c = 0; c < 3; c++) data[o + c] = Math.round(data[o + c] * (1 - mix) + target[c] * mix);
  }
}

/** The hex of a mean colour, `#rrggbb`. */
const hexOf = (rgb) => `#${rgb.map((v) => Math.round(v).toString(16).padStart(2, '0')).join('')}`;

/** A cell's `flat` from its opaque pixels' mean, or from the cell's `flat` rectangle of the source. */
function flatOf(data, source, cell) {
  if (cell.flat) return hexOf(meanOf(source, cell.flat));
  let r = 0, g = 0, b = 0, n = 0;
  for (let o = 0; o < data.length; o += 4) {
    if (data[o + 3] < OPAQUE) continue;
    r += data[o];
    g += data[o + 1];
    b += data[o + 2];
    n++;
  }
  return n ? hexOf([r / n, g / n, b / n]) : null;
}

/**
 * Cuts `cell` (an atlas cell spec: `{ ops, anchor, rotate?, flat? }`) of size `size` `{ w, h, ax?, ay? }` from
 * `region` in `source` (`{ width, height, data }`, RGBA8). Returns `{ data, ax, ay, flat }`: `data` RGBA8 for the
 * cell, the anchor in cell px, and `flat`, the mean colour of its opaque pixels as `#rrggbb` (or of its `flat`
 * rectangle of the source, when it names one).
 */
export function cut(source, region, cell, size) {
  const { w, h } = size;
  const box = patched(source, region);
  const data = region.rect ? spriteData(source, box, region, cell, size) : region.fill ? fillData(box, region, size) : hexData(box, region, size);
  for (const op of cell.ops ?? []) {
    if (op === 'mirror') {
      for (let y = 0; y < h; y++) {
        for (let x = 0; x < w / 2; x++) {
          const a = (y * w + x) * 4, b = (y * w + (w - 1 - x)) * 4;
          for (let c = 0; c < 4; c++) [data[a + c], data[b + c]] = [data[b + c], data[a + c]];
        }
      }
    } else if (op?.op === 'ramp') {
      ramp(data, source, op);
    } else {
      throw new Error(`unknown op ${op?.op ?? op}`);
    }
  }
  const flat = flatOf(data, source, cell);
  if (!flat) throw new Error('the cut has no opaque pixel');
  const [ax, ay] = anchorOf(cell, size);
  return { data, ax, ay, flat };
}

/**
 * A derived cell: the cut of another cell (`base`, cut at its own size `fromSize`) with edge `fr.dir`'s fringe
 * mask multiplied into its alpha, cropped to the fringe's rectangle, which `size` must be. Its `flat` is null
 * when nothing is opaque, as a fringe never is a flat fill.
 */
export function derive(base, fromSize, fr, size) {
  const box = fringeBox(fr.dir);
  if (size.w !== box.w || size.h !== box.h) throw new Error(`fringe ${fr.dir} is ${box.w}×${box.h}, not ${size.w}×${size.h}`);
  const data = new Uint8Array(box.w * box.h * 4);
  for (let y = 0; y < box.h; y++) {
    for (let x = 0; x < box.w; x++) {
      const sx = box.x + x, sy = box.y + y;
      const m = fringeMask(fr.dir, fr.variant, sx + 0.5, sy + 0.5);
      if (!m) continue;
      const s = (sy * fromSize.w + sx) * 4, o = (y * box.w + x) * 4;
      data[o] = base.data[s];
      data[o + 1] = base.data[s + 1];
      data[o + 2] = base.data[s + 2];
      data[o + 3] = Math.round(base.data[s + 3] * m);
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
  return { data, ax: TERRAIN_CELL.w / 2 - box.x, ay: TERRAIN_CELL.h / 2 - box.y, flat: n ? hexOf([r / n, g / n, b / n]) : null };
}
