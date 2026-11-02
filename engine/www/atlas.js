// The atlas loader: `assets/atlas.json` names every cell, `assets/atlas.png` holds them. `cell(name)`
// gives a cell's source rectangle, `tinted(name, colour)` a `.mask` cell filled in a colour (once per
// colour), and `scaled(name, h)` a cell pre-scaled to `h` px tall, for a map drawing thousands of tiles.

export async function loadAtlas(base = './assets/') {
  const json = await (await fetch(`${base}atlas.json`)).json();
  const image = new Image();
  image.src = `${base}${json.image}`;
  await image.decode();
  const tintCache = new Map();
  const scaleCache = new Map();
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
  };
}
