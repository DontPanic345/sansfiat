// The six clan cells at 3x: on mid grey and on a forest-green patch, each mask beside its cell; then each cell with its
// mask multiplied in red, blue, gold and the pale (#d64545, #3b6fd6, #d6a63b, #d8d2c4) drawn over it.
// From engine/:  node ../relay/specs/444/proto/cells.mjs OUT.png   (headless Chrome, as 443's cells.mjs)
import { readFileSync, writeFileSync } from 'node:fs';
import { open } from '../../../../engine/www/play.mjs';
const out = process.argv[2];
const atlas = readFileSync('www/assets/atlas.png').toString('base64');
const json = JSON.parse(readFileSync('www/assets/atlas.json', 'utf8'));
const keys = ['clan.tent', 'clan.tent.mask', 'clan.hearth', 'clan.hearth.mask', 'clan.walk', 'clan.walk.mask'];
const COLOURS = ['#d64545', '#3b6fd6', '#d6a63b', '#d8d2c4'];
const page = await open('', { path: 'assets/blank.html', ready: 'true' });
try {
  const url = await page.eval(`(async () => {
    const src = new Image();
    src.src = 'data:image/png;base64,${atlas}';
    await src.decode();
    const Z = 3, W = 112 * Z, H = 96 * Z, PAD = 24, LEFT = 150;
    const cells = ${JSON.stringify(keys.map((k) => [k, json.cells[k]]))};
    const COLOURS = ${JSON.stringify(COLOURS)};
    const crop = (c) => { const k = document.createElement('canvas'); k.width = c.w; k.height = c.h; k.getContext('2d').drawImage(src, c.x, c.y, c.w, c.h, 0, 0, c.w, c.h); return k; };
    const tinted = (mask, hex) => {
      const k = document.createElement('canvas'); k.width = mask.width; k.height = mask.height;
      const g = k.getContext('2d'), d = mask.getContext('2d').getImageData(0, 0, mask.width, mask.height);
      const rgb = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
      for (let o = 0; o < d.data.length; o += 4) for (let c = 0; c < 3; c++) d.data[o + c] = Math.round((d.data[o + c] * rgb[c]) / 255);
      g.putImageData(d, 0, 0);
      return k;
    };
    const canvas = document.createElement('canvas');
    const cols = 6, rowsN = 2 + 3;
    canvas.width = LEFT + cols * (W + PAD) + PAD; canvas.height = 40 + rowsN * (H + PAD) + PAD;
    const g = canvas.getContext('2d');
    g.imageSmoothingEnabled = false;
    g.fillStyle = '#202020'; g.fillRect(0, 0, canvas.width, canvas.height);
    g.font = '15px sans-serif'; g.fillStyle = '#fff';
    const label = (t, y) => g.fillText(t, PAD, y);
    const grounds = [['mid grey', '#808080'], ['forest green', '#3f7a3a']];
    grounds.forEach(([name, bg], r) => {
      const y0 = 40 + PAD + r * (H + PAD);
      label(name, y0 + H / 2);
      cells.forEach(([k, c], i) => {
        const x0 = LEFT + i * (W + PAD);
        g.fillStyle = bg; g.fillRect(x0, y0, W, H);
        g.drawImage(crop(c), 0, 0, c.w, c.h, x0, y0, W, H);
        g.fillStyle = '#ff00ff'; g.beginPath(); g.arc(x0 + c.ax * Z, y0 + c.ay * Z, 3, 0, 2 * Math.PI); g.fill();
      });
    });
    [['tent', 0], ['hearth', 2], ['walk', 4]].forEach(([kind, i], r) => {
      const y0 = 40 + PAD + (2 + r) * (H + PAD);
      const fig = cells[i][1], mask = cells[i + 1][1];
      label(kind + ' + mask, multiplied', y0 + H / 2);
      COLOURS.forEach((hex, j) => {
        const x0 = LEFT + j * (W + PAD) * 1.5;
        g.fillStyle = '#808080'; g.fillRect(x0, y0, W, H);
        g.drawImage(crop(fig), 0, 0, fig.w, fig.h, x0, y0, W, H);
        g.drawImage(tinted(crop(mask), hex), 0, 0, mask.w, mask.h, x0, y0, W, H);
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
