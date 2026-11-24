// Clans: each clan is drawn where it lives. Above the flat zoom it is its cell from the atlas (a tent that gathers, a hearth
// when settled, walkers when it came to its home this turn), its cloth in its civ's colour (pale for a clan of no civ),
// on the leaning disc of an unaligned clan, every cell edged dark and a starving clan's red. Where the terrain is flat or
// its cell is under `CLAN.from` css px (k 13.6) each is a circle, main's, its
// area by its people, in its civ's colour, a dark square when settled, a red ring when starving. Clans sharing a home
// stand round its centre in a ring, drawn back to front. Every number of the clans is in `CLAN`.

import { cellHeight, isFlat } from './terrain.js';
import { disc } from './leanings.js';
import { PLACE, yardTiles } from './places.js';
import { chroma, hexRgb } from './units.js';

export const name = 'clans';

/** The fill of a clan of no civ. */
export const UNALIGNED = '#d8d2c4';
const STARVING = '#ff2a2a';
const EDGE = 'rgba(24,14,4,0.9)';

/** The clans' numbers, in world units unless a note says css px (`view.dpr` turns it into device px). Build-3 tunes them. */
export const CLAN = {
  per: 0.42, // a lone clan's cell is this many world units tall per √people
  max: 1.5, // and at most this tall
  minPx: 10, // and never shorter than this many css px, so it shows zoomed out
  feet: 0.4, // the cell's anchor (its ground) sits this far below the tile's centre, so the group's middle is near it
  ring: 0.42, // a clan on a shared home stands this far from the home's centre
  shared: 0.8, // and is at most this tall
  starving: 3.5, // the red edge of a starving clan, in css px
  walkers: 1.75, // a lone clan of walkers is drawn this much taller than a tent of the same people: three figures read as people only from about 25 px
  walkersShared: 1.3, // and one on a shared home this much, so it doesn't cover the others
  edge: 1.3, // the dark edge every cell has, in css px, as main's circle has its stroke: a pale tent shows on grass by it
  reach: 3.5, // a home's clans cull while their drawing is on the canvas: a ring or a yard and the tallest walkers reach this far from the tile
  from: 28, // the terrain cell's height in css px below which the clans are circles: k 13.6 on any screen (a 12 px tent is a brown speck)
};

/** The hex's half-width over its vertical sides (a tile's circumradius is 1). */
const HEX_HALF = Math.sqrt(3) / 2;

/** The circles of main's flat zoom: a lone clan's radius at most 0.75, a shared home's clan at most 0.3, on a ring. */
const SHARED_MAX = 0.3;
const RING = 0.34;

/** Whether the clans are circles: the terrain is flat, or its cell is under `CLAN.from` css px. */
export const clansFlat = (view) => {
  const H = cellHeight(view);
  return isFlat(H, view) || H / view.dpr < CLAN.from;
};

/** World to canvas (device) px, as the map's transform maps it. */
const toCanvas = (view, x, y) => [((x - view.cx) * view.k + view.w / 2) * view.dpr, ((y - view.cy) * view.k + view.h / 2) * view.dpr];

/** Clans by home, each home's on screen, in id order. */
function byHome(model, view) {
  const homes = new Map();
  for (const c of Object.values(model.v.clans)) {
    if (!view.onScreen(c.home, CLAN.reach)) continue;
    if (!homes.has(c.home)) homes.set(c.home, []);
    homes.get(c.home).push(c);
  }
  for (const list of homes.values()) list.sort((a, b) => a.id - b.id);
  return homes;
}

/** The flat zoom's circles: each clan's centre and radius in world units, main's own rule. */
function flatDots(model, view) {
  const px = 1 / view.k;
  const dots = [];
  for (const [home, clans] of byHome(model, view)) {
    const n = clans.length;
    clans.forEach((c, k) => {
      if (n === 1) {
        dots.push({ c, x: view.px[home], y: view.py[home], r: Math.min(0.75, Math.max(px * 2.2, 0.16 * Math.sqrt(c.people))) });
        return;
      }
      const a = -Math.PI / 2 + (2 * Math.PI * k) / n;
      dots.push({
        c,
        x: view.px[home] + RING * Math.cos(a),
        y: view.py[home] + RING * Math.sin(a),
        r: Math.min(SHARED_MAX, Math.max(px * 2.2, 0.1 * Math.sqrt(c.people))),
      });
    });
  }
  return dots;
}

/** The kind of a clan's cell: walkers if it came to its home this turn (`since` ≤ 1), a hearth if settled, else a tent. */
export const kindOf = (c) => (c.since <= 1 ? 'walk' : c.settled ? 'hearth' : 'tent');

/** The clans drawn above the flat zoom, back to front by the ground point's y: each with its kind, its ground (world units)
 *  and its height (world units). A lone clan's ground is its tile's centre plus `CLAN.feet`; a shared home's clans stand on
 *  a ring round the centre, in id order from the top, each at its ring point plus `CLAN.feet`. */
function layout(model, view) {
  const items = [];
  const yard = yardTiles(model, view);
  for (const [home, clans] of byHome(model, view)) {
    const n = clans.length;
    const Y = PLACE.yard;
    clans.forEach((c, j) => {
      const people = Math.sqrt(c.people);
      let ax = view.px[home], ay = view.py[home] + CLAN.feet, h, spread = 0; // `spread`: a yard row's x offset, as a share of the row's width
      if (yard.has(home)) {
        // On a place's tile: rows in front of the place, the back row first, each spread across the yard.
        const row = Math.floor(j / Y.perRow), m = Math.min(Y.perRow, n - row * Y.perRow), at = j - row * Y.perRow;
        ay = view.py[home] + (n === 1 ? Y.lone : Y.y + row * Y.dy);
        spread = m === 1 ? 0 : at / (m - 1) - 0.5;
        h = Math.min(n === 1 ? Y.hLone : Y.h, Math.max(CLAN.minPx / view.k, CLAN.per * people));
      } else if (n === 1) {
        h = Math.max(CLAN.minPx / view.k, Math.min(CLAN.max, CLAN.per * people));
      } else {
        const a = -Math.PI / 2 + (2 * Math.PI * j) / n;
        ax += CLAN.ring * Math.cos(a);
        ay += CLAN.ring * Math.sin(a);
        h = Math.min(CLAN.shared, Math.max(CLAN.minPx / view.k, CLAN.per * people));
      }
      const kind = kindOf(c);
      if (kind === 'walk') h *= n === 1 ? CLAN.walkers : CLAN.walkersShared;
      if (spread) {
        // The row spreads as wide as keeps each clan's middle inside the hex at its height: lower rows are narrower.
        const cell = view.atlas.cell(`clan.${kind}`);
        const mid = (ay - view.py[home] - h * (cell.ay / cell.h - 0.5)); // the cell's middle below the tile's centre
        const half = Math.min(Y.span / 2, (mid <= 0.5 ? HEX_HALF : HEX_HALF * 2 * (1 - mid)) - Y.margin);
        ax += 2 * spread * Math.max(half, 0);
      }
      items.push({ c, civ: model.clanCiv.get(c.id) ?? null, kind, ax, ay, h });
    });
  }
  return items.sort((a, b) => a.ay - b.ay || a.c.id - b.c.id);
}

/** A clan's cloth colour: its civ's, or UNALIGNED for a clan of no civ. */
const clothOf = (model, civ) => model.v.civs[civ]?.color ?? UNALIGNED;

export function draw(ctx, model, view) {
  view.stats.clans = [];
  view.stats.clansFlat = clansFlat(view);
  if (view.stats.clansFlat) {
    const px = 1 / view.k;
    for (const d of flatDots(model, view)) {
      drawCircle(ctx, model, view, d.c, d.x, d.y, d.r, px);
      const [x, y] = toCanvas(view, d.x - d.r, d.y - d.r);
      const size = 2 * d.r * view.k * view.dpr;
      view.stats.clans.push({ clan: d.c.id, civ: model.clanCiv.get(d.c.id) ?? null, kind: 'dot', x, y, w: size, h: size, starving: !!d.c.starving });
    }
    return;
  }
  // Each leaning disc first, so each clan's cell stands on it.
  for (const c of Object.values(model.v.clans)) {
    if (!view.onScreen(c.home, CLAN.reach)) continue;
    const tint = model.tintOf(c);
    if (tint) disc(ctx, view, c, tint);
  }
  view.stats.clans = paint(ctx, model, view, {}).records;
}

/** Main's circle for one clan at the flat zoom, with its settled square and its starving ring. */
function drawCircle(ctx, model, view, c, x, y, r, px) {
  const civId = model.clanCiv.get(c.id);
  ctx.beginPath();
  ctx.arc(x, y, r, 0, 2 * Math.PI);
  ctx.fillStyle = civId != null ? model.v.civs[civId].color : UNALIGNED;
  ctx.fill();
  ctx.lineWidth = Math.max(px, 0.04);
  ctx.strokeStyle = 'rgba(0,0,0,0.75)';
  ctx.stroke();
  if (c.settled) {
    const h = Math.max(r * 0.42, px * 1.5);
    ctx.fillStyle = '#2a1a0a';
    ctx.fillRect(x - h, y - h, 2 * h, 2 * h);
    ctx.lineWidth = Math.max(px * 0.8, 0.025);
    ctx.strokeStyle = '#f2dca4';
    ctx.strokeRect(x - h, y - h, 2 * h, 2 * h);
  }
  if (c.starving) {
    ctx.beginPath();
    ctx.arc(x, y, r + px * 1.6, 0, 2 * Math.PI);
    ctx.lineWidth = Math.max(px * 2.2, 0.1);
    ctx.strokeStyle = STARVING;
    ctx.stroke();
  }
}

/** The clans above the flat zoom on `ctx`: each cell, its starving edge under it and its cloth multiplied over it. `only`
 *  draws one clan (its placement still counts every clan on its home), `colour` replaces every cloth's colour, `outline:
 *  false` leaves the starving edge out. Returns the cells' rectangles in canvas px (`records`). */
function paint(ctx, model, view, { only = null, colour = null, outline = true } = {}) {
  const { dpr, k } = view;
  const records = [];
  for (const it of layout(model, view)) {
    if (only != null && it.c.id !== only) continue;
    const cell = `clan.${it.kind}`;
    const hd = Math.max(1, Math.round(it.h * k * dpr)); // the cell's height in device px: the canvas it is drawn from
    const sc = it.h / hd; // world units per px of that canvas
    const c = view.atlas.cell(cell);
    const img = view.atlas.scaled(cell, hd);
    // The cell's anchor, its ground, lands on the clan's ground point.
    const left = it.ax - ((c.ax * img.width) / c.w) * sc;
    const top = it.ay - ((c.ay * img.height) / c.h) * sc;
    const w = img.width * sc, h = hd * sc;
    if (outline && it.c.starving) {
      const sw = Math.max(1, Math.round(CLAN.starving * dpr));
      const o = view.atlas.outline(cell, STARVING, hd, sw);
      ctx.drawImage(o, left - sw * sc, top - sw * sc, o.width * sc, o.height * sc);
    }
    if (outline) {
      const ew = Math.max(1, Math.round(CLAN.edge * dpr));
      const e = view.atlas.outline(cell, EDGE, hd, ew);
      ctx.drawImage(e, left - ew * sc, top - ew * sc, e.width * sc, e.height * sc);
    }
    ctx.drawImage(img, left, top, w, h);
    const mask = view.atlas.shaded(`${cell}.mask`, colour ?? clothOf(model, it.civ), hd);
    ctx.drawImage(mask, left, top, w, h);
    const [x, y] = toCanvas(view, left, top);
    records.push({ clan: it.c.id, civ: it.civ, kind: it.kind, x, y, w: img.width, h: hd, starving: !!it.c.starving });
  }
  return { records };
}

/** Each clan drawn on the map, measured drawn alone: its rectangle, `opaque` (pixels with alpha), `tinted` (pixels that
 *  differ between the drawing in its colour and the drawing with every colour black), `mean` (the mean RGB of the cloth
 *  in those pixels: each one's colour drawing less its black drawing, so the art under the cloth's soft edge doesn't
 *  count) and `nearest` (the living civ, or 'none' for the pale of a clan of no civ, whose colour is nearest `mean` by
 *  chromaticity). A clan is drawn whole on a canvas of its own rectangle. Its cell and cloth only: no disc, no starving
 *  edge; at the flat zoom, its circle's fill. */
export function probe(model, view) {
  const s = view.k * view.dpr;
  const pad = 8; // the starving edge reaches past the rectangle
  const candidates = [
    ...Object.values(model.v.civs).filter((c) => !c.fallen).map((c) => ({ id: c.id, color: c.color || '#ccc' })),
    { id: 'none', color: UNALIGNED },
  ].map((c) => ({ ...c, q: chroma(hexRgb(c.color)) }));
  const flat = clansFlat(view);
  return view.stats.clans.map((rec) => {
    const x0 = Math.floor(rec.x) - pad, y0 = Math.floor(rec.y) - pad;
    const w = Math.ceil(rec.x + rec.w) + pad - x0, h = Math.ceil(rec.y + rec.h) + pad - y0;
    const [ga, gb] = [null, '#000000'].map((colour) => {
      const c = document.createElement('canvas');
      c.width = w;
      c.height = h;
      const g = c.getContext('2d');
      view.setWorld(g);
      g.translate(-x0 / s, -y0 / s); // the clan's rectangle at the canvas's origin
      if (flat) flatFill(g, model, view, rec.clan, colour);
      else paint(g, model, view, { only: rec.clan, colour, outline: false });
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

/** The fill of one flat circle, for the probe: no stroke, no settled square, no starving ring. */
function flatFill(ctx, model, view, id, colour) {
  const d = flatDots(model, view).find((x) => x.c.id === id);
  ctx.beginPath();
  ctx.arc(d.x, d.y, d.r, 0, 2 * Math.PI);
  ctx.fillStyle = colour ?? clothOf(model, model.clanCiv.get(id) ?? null);
  ctx.fill();
}
