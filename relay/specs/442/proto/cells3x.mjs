// The nine new wood and reed cells at 3×, on mid grey, labelled by key and anchor, from atlas.png in headless Chrome.
// From engine/:  node ../relay/specs/442/proto/cells3x.mjs OUT.png
import { readFileSync, writeFileSync } from 'node:fs';
import { open } from '../../../../engine/www/play.mjs';
const out = process.argv[2];
const atlas = readFileSync('www/assets/atlas.png').toString('base64');
const json = JSON.parse(readFileSync('www/assets/atlas.json', 'utf8'));
const keys = ['tree.big.0', 'tree.big.1', 'tree.big.2', 'tree.small.0', 'tree.small.1', 'tree.small.2', 'reed.0', 'reed.1', 'reed.2'];
const page = await open('', { path: 'assets/blank.html', ready: 'true' });
try {
  const url = await page.eval(`(async () => {
    const src = new Image();
    src.src = 'data:image/png;base64,${atlas}';
    await src.decode();
    const Z = 3;
    const cells = ${JSON.stringify(keys.map((k) => [k, json.cells[k]]))};
    // three columns (the big tree, the small tree, the reed), three rows (variants 0 to 2)
    const cellW = 64 * Z + 20, cellH = 80 * Z + 40;
    const canvas = document.createElement('canvas');
    canvas.width = 3 * cellW + 40;
    canvas.height = 3 * cellH + 40;
    const g = canvas.getContext('2d');
    g.fillStyle = '#808080';
    g.fillRect(0, 0, canvas.width, canvas.height);
    g.font = '18px sans-serif';
    g.fillStyle = '#fff';
    cells.forEach(([k, c], i) => {
      const row = i % 3, col = Math.floor(i / 3);
      const ox = 20 + col * cellW, oy = 20 + row * cellH;
      // ground line through the anchor and the cell's frame
      g.strokeStyle = 'rgba(255,255,255,0.35)';
      g.strokeRect(ox - 1, oy - 1, c.w * Z + 2, c.h * Z + 2);
      g.drawImage(src, c.x, c.y, c.w, c.h, ox, oy, c.w * Z, c.h * Z);
      g.fillStyle = '#ff00ff';
      g.beginPath(); g.arc(ox + c.ax * Z, oy + c.ay * Z, 4, 0, 2 * Math.PI); g.fill();
      g.fillStyle = '#fff';
      g.fillText(k + '  ' + c.w + 'x' + c.h + ' anchor ' + c.ax + ',' + c.ay, ox, oy + c.h * Z + 18);
    });
    return canvas.toDataURL('image/png');
  })()`);
  writeFileSync(out, Buffer.from(url.split(',')[1], 'base64'));
  console.log('wrote', out);
  for (const e of page.errors()) console.error('page error', e);
} finally {
  await page.close();
}
