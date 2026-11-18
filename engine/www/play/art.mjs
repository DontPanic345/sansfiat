// The atlas: every cell of atlas.json lies inside atlas.png and is not blank, the map drew its terrain
// from the atlas, and the map is shot. The staged map has every terrain glyph the map text reads, so each
// tile kind is drawn from the atlas.   node www/play/art.mjs [--out DIR]
import { mkdirSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { open } from '../play.mjs';
import { KINDS } from '../feed.js';
import { grid, marks, stage } from '../play-stage.mjs';

const args = process.argv.slice(2);
const at = args.indexOf('--out');
const out = resolve(at >= 0 ? args[at + 1] : mkdtempSync(join(tmpdir(), 'play-art-')));
mkdirSync(out, { recursive: true });

// Radius 6: sea on the rim, coast inside it, and every other glyph (plains with a river, a floodplain, a pass,
// the three closed country glyphs, a lake) cycled over the rest. The two capitals stand on plains.
const R = 6;
const GLYPHS = ['.', 'f', 'h', '^', ':', '%', 'r', 'z', 'P', 'V', 'S', 'N', '=', ','];
const dist = (q, r) => Math.max(Math.abs(q), Math.abs(r), Math.abs(q + r));
const CAPITALS = { A: [-2, 0], B: [2, 0] };
const SPEC = {
  map: grid(R, (q, r) => {
    if (dist(q, r) === R) return '~';
    if (dist(q, r) === R - 1) return ',';
    if (Object.values(CAPITALS).some(([a, b]) => a === q && b === r)) return '.';
    return GLYPHS[(((q * 5 + r * 3) % GLYPHS.length) + GLYPHS.length) % GLYPHS.length];
  }),
  marks: marks(R, CAPITALS),
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
};

const page = await open('');
let failed = false;
const fail = (msg) => {
  console.error(`art: ${msg}`);
  failed = true;
};
try {
  await stage(page, SPEC);
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

  // The sound bus: every feed kind has a file in sounds.json, and after ten turns the bus has heard kinds.
  const manifest = JSON.parse(readFileSync(new URL('../assets/sounds.json', import.meta.url), 'utf8'));
  for (const kind of Object.keys(KINDS)) if (!(kind in manifest)) fail(`sounds.json has no file for ${kind}`);
  const pageFiles = await page.eval('Object.keys(game.sound.files)');
  for (const kind of Object.keys(KINDS)) if (!pageFiles.includes(kind)) fail(`the page's sound bus has no file for ${kind}`);
  for (let i = 0; i < 10; i++) await page.button('End turn');
  const heard = await page.eval('({ ...game.sound.heard })');
  const kinds = Object.keys(heard);
  if (kinds.length === 0) fail('the sound bus heard no kinds in ten turns');
  for (const kind of kinds) if (!(kind in KINDS)) fail(`the sound bus heard ${kind}, which is not a feed kind`);
  if (Object.values(heard).some((n) => !(n > 0))) fail('the sound bus counts a kind with no events');
  if ((await page.eval('game.sound.muted')) !== true) fail('the sound bus is not muted before the first click');

  for (const e of page.errors()) fail(`page error: ${e}`);
  console.log(JSON.stringify({ cells: sheet.cells, sheet: `${sheet.width}x${sheet.height}`, terrainCells: stats.cells, kinds: kinds.length }));
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
