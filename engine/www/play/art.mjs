// The atlas: every cell of atlas.json lies inside atlas.png and is not blank, the map drew its terrain
// from the atlas, and the map is shot at the pick.   node www/play/art.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { open } from '../play.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-art-')));
mkdirSync(out, { recursive: true });

const page = await open('?size=small&seed=2&run=1');
let failed = false;
const fail = (msg) => {
  console.error(`art: ${msg}`);
  failed = true;
};
try {
  await page.until("document.body.dataset.phase === 'choose'");
  const sheet = await page.eval(`(() => {
    const a = game.atlas, c = document.createElement('canvas');
    c.width = a.image.naturalWidth; c.height = a.image.naturalHeight;
    const g = c.getContext('2d', { willReadFrequently: true });
    g.drawImage(a.image, 0, 0);
    const bad = [];
    for (const name of a.names) {
      const { x, y, w, h } = a.cell(name);
      if (x < 0 || y < 0 || x + w > c.width || y + h > c.height) { bad.push([name, 'outside the image']); continue; }
      const d = g.getImageData(x, y, w, h).data;
      let seen = 0;
      for (let i = 3; i < d.length; i += 4) if (d[i] > 0) seen++;
      if (!seen) bad.push([name, 'blank']);
    }
    return { cells: a.names.length, width: c.width, height: c.height, bad };
  })()`);
  for (const [name, why] of sheet.bad) fail(`cell ${name} is ${why}`);
  if (sheet.cells < 41) fail(`only ${sheet.cells} cells in atlas.json`);
  const stats = await page.eval('({ ...game.map.stats })');
  if (!stats.atlas || !stats.cells) fail(`the map did not draw its terrain from the atlas: ${JSON.stringify(stats)}`);
  await page.shot(join(out, 'art-pick.png'));
  for (const e of page.errors()) fail(`page error: ${e}`);
  console.log(JSON.stringify({ cells: sheet.cells, sheet: `${sheet.width}x${sheet.height}`, terrainCells: stats.cells }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
