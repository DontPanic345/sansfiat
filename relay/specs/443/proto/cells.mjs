// The four unit cells at 3×: on mid grey and on a forest-green patch, each mask beside its figure; then each figure
// with its mask multiplied in red, blue and gold (#d64545, #3b6fd6, #d6a63b) drawn over it.
// From engine/:  node ../relay/specs/443/proto/cells.mjs OUT.png   (headless Chrome, as cells3x.mjs)
import { readFileSync, writeFileSync } from 'node:fs';
import { open } from '../../../../engine/www/play.mjs';
const out = process.argv[2];
const atlas = readFileSync('www/assets/atlas.png').toString('base64');
const json = JSON.parse(readFileSync('www/assets/atlas.json', 'utf8'));
const keys = ['unit.camp', 'unit.camp.mask', 'unit.scout', 'unit.scout.mask'];
const COLOURS = ['#d64545', '#3b6fd6', '#d6a63b'];
const page = await open('', { path: 'assets/blank.html', ready: 'true' });
try {
  const url = await page.eval(`(async () => {
    const src = new Image();
    src.src = 'data:image/png;base64,${atlas}';
    await src.decode();
    const Z = 3, W = 96 * Z, PAD = 24;
    const cells = ${JSON.stringify(keys.map((k) => [k, json.cells[k]]))};
    const COLOURS = ${JSON.stringify(COLOURS)};
    const crop = (c) => { const k = document.createElement('canvas'); k.width = c.w; k.height = c.h; k.getContext('2d').drawImage(src, c.x, c.y, c.w, c.h, 0, 0, c.w, c.h); return k; };
    // the mask tinted by multiply: each pixel's grey times the colour, alpha kept
    const tinted = (mask, hex) => {
      const k = document.createElement('canvas'); k.width = mask.width; k.height = mask.height;
      const g = k.getContext('2d'), d = mask.getContext('2d').getImageData(0, 0, mask.width, mask.height);
      const rgb = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
      for (let o = 0; o < d.data.length; o += 4) for (let c = 0; c < 3; c++) d.data[o + c] = Math.round((d.data[o + c] * rgb[c]) / 255);
      g.putImageData(d, 0, 0);
      return k;
    };
    const cellsW = 4 * W + 5 * PAD, rows = 5;
    const canvas = document.createElement('canvas');
    canvas.width = cellsW; canvas.height = rows * (W + PAD) + 2 * PAD + 60;
    const g = canvas.getContext('2d');
    g.imageSmoothingEnabled = false;
    g.font = '15px sans-serif';
    const label = (t, y) => { g.fillStyle = '#fff'; g.fillText(t, PAD, y); };
    const grounds = [['#808080', 0], ['#3f7a3a', 1]];
    grounds.forEach(([bg], r) => {
      const y0 = 50 + PAD + r * (W + PAD);
      g.fillStyle = bg; g.fillRect(PAD / 2, y0 - PAD / 2, cellsW - PAD, W + PAD);
      cells.forEach(([k, c], i) => {
        const x0 = PAD + i * (W + PAD);
        g.drawImage(crop(c), 0, 0, c.w, c.h, x0, y0, W, W);
        g.strokeStyle = 'rgba(255,255,255,0.4)'; g.strokeRect(x0 - 0.5, y0 - 0.5, W + 1, W + 1);
        // the anchor: the feet at (48, 92)
        g.fillStyle = '#ff00ff'; g.beginPath(); g.arc(x0 + c.ax * Z, y0 + c.ay * Z, 3, 0, 2 * Math.PI); g.fill();
      });
      label(grounds[r][0] === '#808080' ? 'mid grey' : 'forest green', y0 - 8);
    });
    // the figure with its mask tinted, drawn over it
    const yRow = 50 + PAD + 2 * (W + PAD) + 30;
    label('figure with its mask multiplied in red, blue and gold, over the figure', yRow - 8);
    [['unit.camp', 'unit.camp.mask'], ['unit.scout', 'unit.scout.mask']].forEach(([fk, mk], r) => {
      const fig = cells.find(([k]) => k === fk)[1], mask = cells.find(([k]) => k === mk)[1];
      COLOURS.forEach((hex, i) => {
        const x0 = PAD + i * (W + PAD), y0 = yRow + PAD + r * (W + PAD);
        g.fillStyle = '#808080'; g.fillRect(x0, y0, W, W);
        g.drawImage(crop(fig), 0, 0, fig.w, fig.h, x0, y0, W, W);
        const t = tinted(crop(mask), hex);
        g.drawImage(t, 0, 0, mask.w, mask.h, x0, y0, W, W);
      });
    });
    return canvas.toDataURL('image/png');
  })()`);
  writeFileSync(out, Buffer.from(url.split(',')[1], 'base64'));
  console.log('wrote', out);
  for (const e of page.errors()) console.error('page error', e);
} finally {
  await page.close();
}
