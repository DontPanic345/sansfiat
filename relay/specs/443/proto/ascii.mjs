// Prints a region of the actor study as text, one character per `step` source px: a class per sample, so the
// cloth, the metal, the wood and the outline can be read off the study's pixels.
// From the repo root:  node relay/specs/443/proto/ascii.mjs X Y W H [STEP]
import { readFileSync } from 'node:fs';
import { decode } from '../../../../engine/www/assets/png.mjs';
const [x0, y0, w, h, step = 2] = process.argv.slice(2).map(Number);
const src = decode(readFileSync('docs/proposals/map-art/actors.png'));
const cls = (r, g, b, a) => {
  if (a < 128) return ' ';
  const lum = 0.299 * r + 0.587 * g + 0.114 * b;
  if (r > 200 && g > 190 && b > 150 && r - b < 60) return 'w'; // pale cloth
  if (r > g + 40 && r > 120) return 'r'; // red
  if (Math.abs(r - g) < 14 && Math.abs(g - b) < 14 && lum > 110) return 'g'; // grey metal
  if (lum < 60) return '#'; // dark outline or shadow
  if (r > g && g > b && lum > 120) return 'o'; // light ochre
  if (r > g && g > b) return 'b'; // brown
  return '?';
};
const header = '    ' + Array.from({ length: Math.ceil(w / step) }, (_, i) => (x0 + i * step) % 10).join('');
console.log(header);
for (let y = y0; y < y0 + h; y += step) {
  let line = String(y).padStart(4);
  for (let x = x0; x < x0 + w; x += step) {
    const s = (y * src.width + x) * 4;
    line += cls(src.data[s], src.data[s + 1], src.data[s + 2], src.data[s + 3]);
  }
  console.log(line);
}
