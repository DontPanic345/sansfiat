// The pan cache: the static layer kept across a pan. A pan moves the camera by whole device px, so the layer's
// pixels are copied by that shift and only the strips the pan uncovers are drawn (each static layer culls to
// `view.strip`, the strip's rectangle). Anything else that changes the layer sets `view.full`: a full redraw.

const EPS = 1e-6;

/** Records the camera the static layer was drawn at; the next draw may shift from it. */
export function keep(view) {
  view.kept = { cx: view.cx, cy: view.cy, k: view.k, w: view.w, h: view.h, dpr: view.dpr };
  view.full = false;
}

/** Brings the static layer to the view's camera by shifting it and drawing the uncovered strips; false (nothing
 *  changed) when it has to be drawn whole: a full change, a zoom, a resize, a shift off whole device px or past half the canvas. */
export function shifted(view) {
  const p = view.kept;
  if (view.full || !p || p.k !== view.k || p.w !== view.w || p.h !== view.h || p.dpr !== view.dpr) return false;
  const s = view.k * view.dpr;
  const fx = (p.cx - view.cx) * s, fy = (p.cy - view.cy) * s;
  const sx = Math.round(fx), sy = Math.round(fy);
  const W = view.layer.width, H = view.layer.height;
  if (Math.abs(fx - sx) > EPS || Math.abs(fy - sy) > EPS || Math.abs(sx) * 2 >= W || Math.abs(sy) * 2 >= H) return false;
  if (!sx && !sy) return true;
  // Copied into a second canvas, which becomes the layer: a canvas drawn onto itself costs a snapshot.
  if (!view.spare) {
    view.spare = document.createElement('canvas');
    view.sctx = view.spare.getContext('2d');
  }
  if (view.spare.width !== W || view.spare.height !== H) view.spare.width = W, view.spare.height = H;
  view.sctx.setTransform(1, 0, 0, 1, 0, 0);
  view.sctx.globalCompositeOperation = 'copy';
  view.sctx.drawImage(view.layer, sx, sy);
  view.sctx.globalCompositeOperation = 'source-over';
  [view.layer, view.spare, view.lctx, view.sctx] = [view.spare, view.layer, view.sctx, view.lctx];
  const ctx = view.lctx;
  const strips = [];
  if (sx) strips.push([sx > 0 ? 0 : W + sx, 0, sx > 0 ? sx : W, H]);
  if (sy) strips.push([sx > 0 ? sx : 0, sy > 0 ? 0 : H + sy, sx < 0 ? W + sx : W, sy > 0 ? sy : H]);
  for (const [x0, y0, x1, y1] of strips) {
    view.strip = [x0, y0, x1, y1];
    view.collectVisible();
    ctx.save();
    ctx.beginPath();
    ctx.rect(x0, y0, x1 - x0, y1 - y0);
    ctx.clip();
    view.drawStatic();
    ctx.restore();
  }
  view.strip = null;
  return true;
}
