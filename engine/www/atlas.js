// The atlas loader: `assets/atlas.json` names every cell, `assets/atlas.png` holds them. `cell(name)`
// gives a cell's source rectangle, `tinted(name, colour)` a `.mask` cell filled in a colour (once per
// colour), and `scaled(name, h)` a cell pre-scaled to `h` px tall, for a map drawing thousands of tiles.
// `shaded` multiplies a cell by a colour, `outline` edges its silhouette (the units' figures use both).

export async function loadAtlas(base = './assets/') {
  const json = await (await fetch(`${base}atlas.json`)).json();
  const image = new Image();
  image.src = `${base}${json.image}`;
  await image.decode();
  const tintCache = new Map();
  const scaleCache = new Map();
  const shadeCache = new Map();
  const outlineCache = new Map();
  const make = (w, h) => {
    const c = document.createElement('canvas');
    c.width = Math.max(1, w);
    c.height = Math.max(1, h);
    return c;
  };
  return {
    json,
    image,
    names: Object.keys(json.cells),
    /** `{x, y, w, h, note}` of cell `name`, or throws. */
    cell(name) {
      const c = json.cells[name];
      if (!c) throw new Error(`no atlas cell ${name}`);
      return c;
    },
    /** A canvas of mask cell `name` filled with `color`; draw it whole (its size is the cell's). */
    tinted(name, color) {
      const key = `${name}|${color}`;
      let c = tintCache.get(key);
      if (!c) {
        const cell = this.cell(name);
        c = make(cell.w, cell.h);
        const g = c.getContext('2d');
        g.drawImage(image, cell.x, cell.y, cell.w, cell.h, 0, 0, cell.w, cell.h);
        g.globalCompositeOperation = 'source-in';
        g.fillStyle = color;
        g.fillRect(0, 0, cell.w, cell.h);
        tintCache.set(key, c);
      }
      return c;
    },
    /** A canvas of cell `name` scaled to `h` px tall (width in proportion), cached by `h`. */
    scaled(name, h) {
      const key = `${name}@${h}`;
      let c = scaleCache.get(key);
      if (!c) {
        if (scaleCache.size > 400) scaleCache.clear();
        const cell = this.cell(name);
        c = make(Math.round((cell.w * h) / cell.h), h);
        const g = c.getContext('2d');
        g.imageSmoothingQuality = 'high';
        g.drawImage(image, cell.x, cell.y, cell.w, cell.h, 0, 0, c.width, c.height);
        scaleCache.set(key, c);
      }
      return c;
    },
    /** Cell `name` scaled to `h` px tall, multiplied by `colour` with its alpha kept (a grey cloth shows the colour
     *  through its shading), cached by name, colour and height. */
    shaded(name, colour, h) {
      const key = `${name}|${colour}|${h}`;
      let c = shadeCache.get(key);
      if (!c) {
        if (shadeCache.size > 400) shadeCache.clear();
        const s = this.scaled(name, h);
        c = make(s.width, s.height);
        const g = c.getContext('2d');
        g.drawImage(s, 0, 0);
        g.globalCompositeOperation = 'multiply';
        g.fillStyle = colour;
        g.fillRect(0, 0, c.width, c.height);
        g.globalCompositeOperation = 'destination-in';
        g.drawImage(s, 0, 0);
        // The cloth's soft edge is made firmer: drawn over itself, a pixel of alpha a has 2a - a², so the figure's own colour
        // shows less at the cloth's rim, which matters when a cell is shrunk to a dozen px.
        const d = make(c.width, c.height);
        const dg = d.getContext('2d');
        dg.drawImage(c, 0, 0);
        dg.drawImage(c, 0, 0);
        c = d;
        shadeCache.set(key, c);
      }
      return c;
    },
    /** The silhouette of cell `name` at `h` px tall, filled in `colour` and dilated by `width` device px, on a canvas
     *  `width` larger each way: its anchor is the cell's plus `width`. Cached by name, colour, height and width. */
    outline(name, colour, h, width) {
      const key = `${name}|${colour}|${h}|${width}`;
      let c = outlineCache.get(key);
      if (!c) {
        if (outlineCache.size > 400) outlineCache.clear();
        const s = this.scaled(name, h);
        const sil = make(s.width, s.height);
        const g = sil.getContext('2d');
        g.drawImage(s, 0, 0);
        g.globalCompositeOperation = 'source-in';
        g.fillStyle = colour;
        g.fillRect(0, 0, sil.width, sil.height);
        c = make(s.width + 2 * width, s.height + 2 * width);
        const o = c.getContext('2d');
        for (let dy = -width; dy <= width; dy++) {
          for (let dx = -width; dx <= width; dx++) {
            if (dx * dx + dy * dy <= width * width) o.drawImage(sil, width + dx, width + dy);
          }
        }
        outlineCache.set(key, c);
      }
      return c;
    },
  };
}
