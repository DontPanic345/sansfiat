// Places: a town, a settlement and a capital drawn as their cells from the atlas, above the flat zoom. A town is the
// thatched hamlet of three huts, no colour; a settlement the palisade with its flag, and a capital the civic hall with its
// banners, their cloth in the civ's colour, multiplied over the art as the units' and clans' are. Each place stands where
// its tile is, its ground a little below the tile's centre, and the clans on its tile draw over it (this layer is before
// `clans` in LAYERS). At the flat zoom (`placesFlat`) it draws nothing: capitals.js and settle.js draw main's houses there,
// and `stats.places` lists each capital and settlement as a mark. Every number of the places is in `PLACE`.

import { cellHeight, isFlat } from './terrain.js';
import { chroma, hexRgb } from './units.js';

export const name = 'places';

/** The places' numbers, in world units unless a note says css px (`view.dpr` turns it into device px). Build-3 tunes them. */
export const PLACE = {
  w: {
    town: 1.6, // a town's cell is this many world units wide: the hamlet of thatched huts
    settlement: 1.8, // a settlement's, the palisade with its flag
    capital: 2.0, // a capital's, the civic hall on its mound
  },
  feet: 0.3, // the cell's anchor (its ground) sits this far below the tile's centre: the place rises into the hex's upper half
  from: 28, // the terrain cell's height in css px below which the places are main's marks: the clans' own zoom (CLAN.from)
};

/** Whether the places are main's marks: the terrain is flat, or its cell is under `PLACE.from` css px. */
export const placesFlat = (view) => {
  const H = cellHeight(view);
  return isFlat(H, view) || H / view.dpr < PLACE.from;
};

/** World to canvas (device) px, as the map's transform maps it. */
const toCanvas = (view, x, y) => [((x - view.cx) * view.k + view.w / 2) * view.dpr, ((y - view.cy) * view.k + view.h / 2) * view.dpr];

/** The cloth colour of a place's civ; a town has none. */
const clothOf = (model, civ) => model.v.civs[civ]?.color ?? '#ccc';

/** The places the map draws, on screen: each living civ's capital, each settlement not on a capital, each town not on
 *  either, in that order. A fallen civ's capital tile is a town if the tile reads as one, else nothing. */
export function placesOn(model, view) {
  const v = model.v;
  const out = [];
  const seen = new Set();
  const add = (tile, role, civ) => {
    if (tile == null || seen.has(tile)) return;
    seen.add(tile);
    if (view.onScreen(tile)) out.push({ tile, role, civ });
  };
  for (const c of Object.values(v.civs ?? {})) if (!c.fallen) add(c.capital, 'capital', c.id);
  for (const s of v.settlements ?? []) add(s.tile, 'settlement', s.civ);
  for (const t of v.towns ?? []) add(t.tile, 'town', null);
  return out;
}

/** The places drawn above the flat zoom, each `{tile, role, civ, x, y, w, h}` in canvas px. `only` draws one tile's place,
 *  `colour` replaces every cloth's colour. Returns the cells' rectangles (`records`). */
function paint(ctx, model, view, { only = null, colour = null } = {}) {
  const { dpr, k } = view;
  const records = [];
  for (const p of placesOn(model, view)) {
    if (only != null && p.tile !== only) continue;
    const cell = `place.${p.role}`;
    const c = view.atlas.cell(cell);
    const H = (PLACE.w[p.role] * c.h) / c.w; // the cell's height in world units, at its width
    const hd = Math.max(1, Math.round(H * k * dpr)); // and in device px: the canvas it is drawn from
    const sc = H / hd; // world units per px of that canvas
    const img = view.atlas.scaled(cell, hd);
    // The cell's anchor, its ground, lands on the place's ground point.
    const ax = view.px[p.tile], ay = view.py[p.tile] + PLACE.feet;
    const left = ax - ((c.ax * img.width) / c.w) * sc;
    const top = ay - ((c.ay * img.height) / c.h) * sc;
    const w = img.width * sc, h = hd * sc;
    ctx.drawImage(img, left, top, w, h);
    if (p.role !== 'town') {
      const mask = view.atlas.shaded(`${cell}.mask`, colour ?? clothOf(model, p.civ), hd);
      ctx.drawImage(mask, left, top, w, h);
    }
    const [x, y] = toCanvas(view, left, top);
    records.push({ tile: p.tile, role: p.role, civ: p.civ, x, y, w: img.width, h: hd });
  }
  return records;
}

/** The marks of the flat zoom: each capital's and settlement's house as main's capitals.js and settle.js draw it, its
 *  rectangle in canvas px. */
function marks(model, view) {
  const px = 1 / view.k;
  const out = [];
  for (const p of placesOn(model, view)) {
    if (p.role === 'town') continue;
    const x = view.px[p.tile];
    let x0, x1, y0, y1;
    if (p.role === 'capital') {
      const s = Math.max(0.95, px * 8), W = s * (64 / 22), y = view.py[p.tile] - 0.075 * s;
      x0 = x - W / 2; x1 = x + W / 2; y0 = y - W / 2; y1 = y + W / 2;
    } else {
      const u = Math.max(0.5, px * 14), w = u * 0.9, y = view.py[p.tile];
      x0 = x - w; x1 = x + w; y0 = y - 0.95 * w; y1 = y + 0.8 * w;
    }
    const [cx, cy] = toCanvas(view, x0, y0);
    out.push({ tile: p.tile, role: 'mark', civ: p.civ, x: cx, y: cy, w: (x1 - x0) * view.k * view.dpr, h: (y1 - y0) * view.k * view.dpr });
  }
  return out;
}

export function draw(ctx, model, view) {
  if (placesFlat(view)) {
    view.stats.places = marks(model, view);
    return;
  }
  view.stats.places = paint(ctx, model, view);
}

/** Each place drawn on the map, measured drawn alone on a canvas of its own rectangle, once in its colour and once with
 *  every colour black: its `opaque` pixels, `tinted` (pixels that differ between the two), `mean` (the mean RGB of the
 *  cloth in them: each one's colour drawing less its black drawing) and `nearest` (the living civ whose colour is nearest
 *  `mean` by chromaticity, or null where nothing is tinted: a town). Above the flat zoom only. */
export function probe(model, view) {
  if (placesFlat(view)) return [];
  const s = view.k * view.dpr;
  const pad = 8; // the cell's edge reaches past its rectangle
  const candidates = Object.values(model.v.civs)
    .filter((c) => !c.fallen)
    .map((c) => ({ id: c.id, q: chroma(hexRgb(c.color || '#ccc')) }));
  return view.stats.places.map((rec) => {
    const x0 = Math.floor(rec.x) - pad, y0 = Math.floor(rec.y) - pad;
    const w = Math.ceil(rec.x + rec.w) + pad - x0, h = Math.ceil(rec.y + rec.h) + pad - y0;
    const [ga, gb] = [null, '#000000'].map((colour) => {
      const c = document.createElement('canvas');
      c.width = w;
      c.height = h;
      const g = c.getContext('2d');
      view.setWorld(g);
      g.translate(-x0 / s, -y0 / s); // the place's rectangle at the canvas's origin
      paint(g, model, view, { only: rec.tile, colour });
      return g;
    });
    const A = ga.getImageData(0, 0, w, h).data;
    const B = gb.getImageData(0, 0, w, h).data;
    const row = {
      ...rec, clipped: rec.x < 0 || rec.y < 0 || rec.x + rec.w > view.canvas.width || rec.y + rec.h > view.canvas.height,
      opaque: 0, tinted: 0, mean: null, nearest: null,
    };
    const sum = [0, 0, 0];
    for (let i = 0; i < A.length; i += 4) {
      if (A[i + 3] > 0) row.opaque++;
      if (A[i] !== B[i] || A[i + 1] !== B[i + 1] || A[i + 2] !== B[i + 2] || A[i + 3] !== B[i + 3]) {
        row.tinted++;
        sum[0] += A[i] - B[i];
        sum[1] += A[i + 1] - B[i + 1];
        sum[2] += A[i + 2] - B[i + 2];
      }
    }
    if (row.tinted) {
      const mean = sum.map((x) => x / row.tinted);
      row.mean = mean.map((x) => Math.round(x * 100) / 100);
      const m = chroma(mean);
      let best = null, bd = Infinity;
      for (const c of candidates) {
        const d = (c.q[0] - m[0]) ** 2 + (c.q[1] - m[1]) ** 2 + (c.q[2] - m[2]) ** 2;
        if (d < bd) { bd = d; best = c.id; }
      }
      row.nearest = best;
    }
    return row;
  });
}
