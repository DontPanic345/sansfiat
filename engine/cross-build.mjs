// Prints "<turn> <digest>" for turn 0 and after each turn, from the wasm build under Node.
// Usage: node cross-build.mjs <seed> <radius> <turns> [lead]. Needs www/pkg (./build-web.sh).
// With `lead` it plays the scripted game of `sansfiat digest --lead`, which lives in core/src/script.rs
// and runs through `sim.lead_orders()`; without it, it watches at the pick.
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const { default: _init, initSync, Sim } = await import(pathToFileURL(`${here}www/pkg/web.js`).href);
initSync({ module: readFileSync(`${here}www/pkg/web_bg.wasm`) });

const [seed, radius, turns] = process.argv.slice(2, 5).map(Number);
const lead = process.argv[5] === 'lead';

const sim = new Sim(seed, radius);
const lines = [`0 ${sim.digest()}`];
for (let turn = 1; turn <= turns; turn++) {
  if (lead) sim.lead_orders();
  // As native World::run does at the pick: watch.
  else if (sim.phase() === 'choose') sim.act('{"type":"chooseCiv","civ":null}');
  sim.act('{"type":"endTurn"}');
  lines.push(`${turn} ${sim.digest()}`);
}
console.log(lines.join('\n'));
