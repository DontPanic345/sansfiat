// Renders `placeholders.js` into `atlas.png` and writes `atlas.json` (every cell's place, size and note),
// through play.mjs's headless Chrome:  node www/assets/draw-atlas.mjs
import { writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { open } from '../play.mjs';
import { layout } from './placeholders.js';

const here = fileURLToPath(new URL('.', import.meta.url));
const sheet = layout();
const page = await open('', { path: 'assets/blank.html', ready: 'true' });
try {
  const url = await page.eval(`(async () => {
    const { draw } = await import('/assets/placeholders.js');
    const sheet = ${JSON.stringify(sheet)};
    const canvas = document.createElement('canvas');
    canvas.width = sheet.width;
    canvas.height = sheet.height;
    const ctx = canvas.getContext('2d');
    for (const [name, c] of Object.entries(sheet.cells)) {
      ctx.save();
      ctx.translate(c.x, c.y);
      ctx.beginPath();
      ctx.rect(0, 0, c.w, c.h);
      ctx.clip();
      draw(name, ctx, c.w, c.h);
      ctx.restore();
    }
    return canvas.toDataURL('image/png');
  })()`);
  writeFileSync(`${here}atlas.png`, Buffer.from(url.split(',')[1], 'base64'));
  writeFileSync(`${here}atlas.json`, `${JSON.stringify({ image: 'atlas.png', width: sheet.width, height: sheet.height, cells: sheet.cells }, null, 1)}\n`);
  console.log(`${Object.keys(sheet.cells).length} cells, ${sheet.width}×${sheet.height}`);
  for (const e of page.errors()) console.error(`page error: ${e}`);
} finally {
  await page.close();
}
