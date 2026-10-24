// Prints "<turn> <digest>" for turn 0 and after each turn, from the wasm build under Node.
// Usage: node cross-build.mjs <seed> <radius> <turns>. Needs www/pkg (./build-web.sh).
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const { default: _init, initSync, Sim } = await import(pathToFileURL(`${here}www/pkg/web.js`).href);
initSync({ module: readFileSync(`${here}www/pkg/web_bg.wasm`) });

const [seed, radius, turns] = process.argv.slice(2).map(Number);
const sim = new Sim(seed, radius);
const lines = [`0 ${sim.digest()}`];
for (let turn = 1; turn <= turns; turn++) {
  sim.act('{"type":"endTurn"}');
  lines.push(`${turn} ${sim.digest()}`);
}
console.log(lines.join('\n'));
