// Units: each army (camp) and scout is its figure, the cell's drawing from the atlas, with its cloth (the mask) in its
// civ's colour, multiplied in so the figure's shading shows through. Units on a tile stand side by side, and a click
// cycles them (as now). A hungry one is edged red, the selected one white. Each unit's people sit beside its figure:
// one decimal under ten, whole above, and an army's archers split off as "6+2" with a small bow. Every number draws after
// every figure, so no figure covers one. A fallen civ draws nothing. Every number of the figures is in FIGURE.

export const name = 'units';

/** The figures' numbers. Lengths are in `s`, the figure's scale in world units, unless a note says css px. */
export const FIGURE = {
  s: 0.6, // the least scale: a figure at a far zoom is this many world units per cell unit
  minPx: 6, // the figure's scale is never under this many css px, so it shows zoomed out
  h: 2.4, // the figure's height, in `s`
  feet: 0.9, // the feet, below the slot's point, in `s`
  slotX: 0.9, // the slot's point (the first unit on a tile), in `s` from the tile's centre
  slotY: 1.1, // the slot's point, in `s` below the tile's centre
  gap: 1.7, // the distance from one unit's slot to the next on a tile, in `s`
  selected: 2.4, // the white edge of the selected unit, in css px
  hungry: 3.5, // the red edge of a hungry unit, in css px
  labelX: 0.95, // the number's left edge from the slot's point, in `s` (as main's)
  labelY: -0.3, // the number's baseline-middle from the slot's point, in `s` (as main's)
  font: 0.9, // the number's font size, in `s` (main's 1.1 runs two stacked numbers together at gap 1.7)
  bowX: -0.9, // an archers' bow: its centre from the slot's point, in `s` (the figure's upper left)
  bowY: -0.7,
  bowR: 0.45, // the bow's radius, in `s`
};

const HUNGRY = '#ff2a2a';
const SELECTED = '#ffffff';

/** An army's people as the figure's number: its warriors, and its archers split off ("6+2") when it has any. */
export function countLabel(u) {
  const [w = u.people, a = 0] = u.stack ?? [];
  const num = (x) => (x < 9.95 ? (Math.round(x * 10) / 10).toFixed(1) : String(Math.round(x)));
  return a >= 0.05 ? `${num(w)}+${num(a)}` : num(u.people);
}

let measurer = null;
/** A text's width and its ascent and descent, in world units, for a font `size` in world units. Measured at 100 times the
 *  size: the browser rounds a glyph's bounds to whole px, which at world scale is a whole unit. */
function measure(text, size) {
  measurer ??= document.createElement('canvas').getContext('2d');
  const M = 100;
  measurer.font = `bold ${size * M}px system-ui, sans-serif`;
  measurer.textBaseline = 'middle';
  const m = measurer.measureText(text);
  return { width: m.width / M, asc: m.actualBoundingBoxAscent / M, desc: m.actualBoundingBoxDescent / M };
}

/** World to canvas (device) px, as the map's transform maps it. */
const toCanvas = (view, x, y) => [((x - view.cx) * view.k + view.w / 2) * view.dpr, ((y - view.cy) * view.k + view.h / 2) * view.dpr];

/** The units of the map: their figures on the context, then their numbers. Returns the figures' rectangles in canvas px
 *  (`stats.units`) and the numbers' boxes (`stats.labels`). `only` draws one unit, `colour` replaces every civ's colour,
 *  `numbers: false` leaves the numbers out (the probe draws figures alone). */
function paint(ctx, model, view, { only = null, colour = null, numbers = true } = {}) {
  const { v } = model;
  const { k, dpr } = view;
  const s = Math.max(FIGURE.s, FIGURE.minPx / k);
  const H = FIGURE.h * s; // the figure's height in world units
  const hd = Math.max(1, Math.round(H * k * dpr)); // and in device px: the canvas it is drawn from
  const sc = H / hd; // world units per px of that canvas
  const hungryPx = Math.max(1, Math.round(FIGURE.hungry * dpr));
  const selectedPx = Math.max(1, Math.round(FIGURE.selected * dpr));
  const units = [], places = [];
  for (const [tile, list] of model.stacks) {
    if (!view.onScreen(tile)) continue;
    let n = 0;
    for (const u of list) {
      if (v.civs[u.civ]?.fallen) continue;
      const slot = n++;
      if (only != null && u.id !== only) continue;
      const x = view.px[tile] + slot * FIGURE.gap * s + FIGURE.slotX * s;
      const y = view.py[tile] + FIGURE.slotY * s;
      const cell = `unit.${u.kind}`;
      const c = view.atlas.cell(cell);
      const img = view.atlas.scaled(cell, hd);
      // The cell's anchor, its feet, lands on the figure's feet.
      const left = x - ((c.ax * img.width) / c.w) * sc;
      const top = y + FIGURE.feet * s - ((c.ay * img.height) / c.h) * sc;
      const w = img.width * sc, h = img.height * sc;
      const hungry = u.hungry > 0, selected = u.id === view.selectedUnit;
      if (hungry) {
        const o = view.atlas.outline(cell, HUNGRY, hd, hungryPx);
        ctx.drawImage(o, left - hungryPx * sc, top - hungryPx * sc, o.width * sc, o.height * sc);
      }
      if (selected) {
        const o = view.atlas.outline(cell, SELECTED, hd, selectedPx);
        ctx.drawImage(o, left - selectedPx * sc, top - selectedPx * sc, o.width * sc, o.height * sc);
      }
      ctx.drawImage(img, left, top, w, h);
      const mask = view.atlas.shaded(`${cell}.mask`, colour ?? v.civs[u.civ]?.color ?? '#ccc', hd);
      ctx.drawImage(mask, left, top, w, h);
      const [dx, dy] = toCanvas(view, left, top);
      units.push({ unit: u.id, civ: u.civ, kind: u.kind, x: dx, y: dy, w: w * k * dpr, h: h * k * dpr, selected, hungry });
      places.push({ u, x, y });
    }
  }
  const labels = [];
  if (numbers) {
    for (const { u, x, y } of places) {
      const label = countLabel(u);
      const tx = x + FIGURE.labelX * s, ty = y + FIGURE.labelY * s;
      ctx.font = `bold ${s * FIGURE.font}px system-ui, sans-serif`;
      ctx.textAlign = 'left';
      ctx.textBaseline = 'middle';
      ctx.lineWidth = Math.max(1.5 / k, 0.05);
      ctx.strokeStyle = '#fff';
      ctx.fillStyle = '#000';
      const { width, asc, desc } = measure(label, s * FIGURE.font);
      const [bx, by] = toCanvas(view, tx, ty - asc);
      const box = { x: bx, y: by, w: width * k * dpr, h: (asc + desc) * k * dpr };
      labels.push({ unit: u.id, label, box });
      ctx.strokeText(label, tx, ty);
      ctx.fillText(label, tx, ty);
      // Archers: a small bow over the figure's upper left.
      if (u.stack && u.stack[1] >= 0.05) {
        ctx.beginPath();
        ctx.arc(x + FIGURE.bowX * s, y + FIGURE.bowY * s, FIGURE.bowR * s, -Math.PI / 2, Math.PI / 2);
        ctx.strokeStyle = '#000';
        ctx.lineWidth = Math.max(1.2 / k, 0.05);
        ctx.stroke();
      }
    }
  }
  return { units, labels };
}

export function draw(ctx, model, view) {
  const { units, labels } = paint(ctx, model, view);
  view.stats.units = units;
  view.stats.labels = labels;
}

const hexRgb = (hex) => {
  const h = hex.replace('#', '');
  const full = h.length === 3 ? [...h].map((c) => c + c).join('') : h;
  const n = parseInt(full, 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
};
/** A colour by chromaticity: each channel over the sum, so a shading that scales a colour keeps its hue. */
const chroma = ([r, g, b]) => {
  const t = r + g + b;
  return t ? [r / t, g / t, b / t] : [1 / 3, 1 / 3, 1 / 3];
};

/** Each unit in `stats.units`, measured drawn alone: its rectangle, `opaque` (pixels with alpha), `tinted` (pixels that
 *  differ between the drawing in the civ's colours and the drawing with every civ's colour black), `mean` (the mean RGB
 *  of those tinted pixels, in the colours) and `nearest` (the living civ whose colour is nearest `mean` by chromaticity),
 *  and `clipped` (the rectangle reaches past the canvas). Each unit is drawn whole, on a canvas of its own rectangle, so a
 *  figure the canvas cuts is measured as its figure, not as the sliver on screen. */
export function probe(model, view) {
  const s = view.k * view.dpr;
  const pad = 8; // the outlines reach past the rectangle
  const living = Object.values(model.v.civs).filter((c) => !c.fallen);
  const W = view.canvas.width, H = view.canvas.height;
  return view.stats.units.map((rec) => {
    const x0 = Math.floor(rec.x) - pad, y0 = Math.floor(rec.y) - pad;
    const w = Math.ceil(rec.x + rec.w) + pad - x0, h = Math.ceil(rec.y + rec.h) + pad - y0;
    const [ga, gb] = [null, '#000000'].map((colour) => {
      const c = document.createElement('canvas');
      c.width = w;
      c.height = h;
      const g = c.getContext('2d');
      view.setWorld(g);
      g.translate(-x0 / s, -y0 / s); // the unit's rectangle at the canvas's origin
      paint(g, model, view, { only: rec.unit, colour, numbers: false });
      return g;
    });
    const A = ga.getImageData(0, 0, w, h).data;
    const B = gb.getImageData(0, 0, w, h).data;
    const row = {
      ...rec, clipped: rec.x < 0 || rec.y < 0 || rec.x + rec.w > W || rec.y + rec.h > H,
      opaque: 0, tinted: 0, mean: null, nearest: null,
    };
    const sum = [0, 0, 0];
    for (let i = 0; i < A.length; i += 4) {
      if (A[i + 3] > 0) row.opaque++;
      if (A[i] !== B[i] || A[i + 1] !== B[i + 1] || A[i + 2] !== B[i + 2] || A[i + 3] !== B[i + 3]) {
        row.tinted++;
        sum[0] += A[i];
        sum[1] += A[i + 1];
        sum[2] += A[i + 2];
      }
    }
    if (row.tinted) {
      const mean = sum.map((x) => x / row.tinted);
      row.mean = mean.map((x) => Math.round(x * 100) / 100);
      const m = chroma(mean);
      let best = -1, bd = Infinity;
      for (const c of living) {
        const q = chroma(hexRgb(c.color || '#ccc'));
        const d = (q[0] - m[0]) ** 2 + (q[1] - m[1]) ** 2 + (q[2] - m[2]) ** 2;
        if (d < bd) { bd = d; best = c.id; }
      }
      row.nearest = best;
    }
    return row;
  });
}
