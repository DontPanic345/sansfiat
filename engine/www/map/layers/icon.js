// Drawing an atlas icon on the map, centred on a point, `size` world units across.
export function icon(ctx, view, name, x, y, size) {
  const c = view.atlas.cell(name);
  ctx.drawImage(view.atlas.image, c.x, c.y, c.w, c.h, x - size / 2, y - size / 2, size, size);
}

/** As `icon`, for a `.mask` cell tinted `color`. */
export function tinted(ctx, view, name, color, x, y, size) {
  const c = view.atlas.tinted(name, color);
  ctx.drawImage(c, 0, 0, c.width, c.height, x - size / 2, y - size / 2, size, size);
}
