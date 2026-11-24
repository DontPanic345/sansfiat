// The last --pan frame of each view on two trees, compared: node pandiff.mjs DIR_A DIR_B  (run from engine/www)
// Prints per view the share of pixels that differ, and the share that differ by more than 40 in a channel.
import { readdirSync } from 'node:fs';
import { join } from 'node:path';
import { readPng } from '/home/runner/work/sansfiat/sansfiat/engine/www/assets/png.mjs';
const [a, b] = process.argv.slice(2);
for (const f of readdirSync(a).filter((f) => f.endsWith('.png'))) {
  const x = readPng(join(a, f)), y = readPng(join(b, f));
  let d = 0, big = 0;
  for (let o = 0; o < x.data.length; o += 4) {
    const m = Math.max(...[0, 1, 2].map((c) => Math.abs(x.data[o + c] - y.data[o + c])));
    if (m) d++;
    if (m > 40) big++;
  }
  const n = x.width * x.height;
  console.log(`${f.replace('.png', '')}  differ ${((100 * d) / n).toFixed(3)}%  by more than 40 ${((100 * big) / n).toFixed(3)}%`);
}
