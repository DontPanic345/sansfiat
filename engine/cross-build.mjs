// Prints "<turn> <digest>" for turn 0 and after each turn, from the wasm build under Node.
// Usage: node cross-build.mjs <seed> <radius> <turns> [lead]. Needs www/pkg (./build-web.sh).
// With `lead` it plays the scripted game of `sansfiat digest --lead` (see lead() below); without it,
// it watches at the pick.
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const { default: _init, initSync, Sim } = await import(pathToFileURL(`${here}www/pkg/web.js`).href);
initSync({ module: readFileSync(`${here}www/pkg/web_bg.wasm`) });

const [seed, radius, turns] = process.argv.slice(2, 5).map(Number);
const lead = process.argv[5] === 'lead';

// The scripted game, between turns and before each endTurn (the same rule as the CLI's
// lead_orders). `pick` is the turn chooseCiv was acted, and n the turns since.
//   1. At the pick: lead the lowest civ id.
//   2. In the main game, when n > 0 and n % 20 == 0: raise one warrior (a refusal is ignored).
//   3. When n == 30: setSwitches both off; when n == 40: both on. When n > 0 and n % 25 == 0: resupply.
//   4. If the player's camp has no dest: moveCamp to the first accepted of the other civs' capitals
//      in civ id order, then its own capital.
//   5. When n == 60: disband one warrior (ignored without a camp).
// The state is read only on the pick turn and the turns after it (the camp's dest needs it).
let pick = null;
const act = (a) => {
  try {
    sim.act(JSON.stringify(a));
    return true;
  } catch {
    return false;
  }
};
const byId = (o) => Object.values(o).sort((a, b) => a.id - b.id);
function leadOrders(turn) {
  if (sim.phase() === 'choose') {
    const first = byId(JSON.parse(sim.state()).civs)[0];
    act({ type: 'chooseCiv', civ: first.id });
    pick = turn;
  }
  if (pick === null || sim.phase() !== 'main') return;
  const n = turn - pick;
  if (n > 0 && n % 20 === 0) act({ type: 'raise', n: 1 });
  if (n === 30) act({ type: 'setSwitches', forage: false, supply: false });
  if (n === 40) act({ type: 'setSwitches', forage: true, supply: true });
  if (n > 0 && n % 25 === 0) act({ type: 'resupply' });
  const state = JSON.parse(sim.state());
  const me = state.player;
  const civs = byId(state.civs);
  const targets = civs.filter((c) => c.id !== me).map((c) => c.capital).concat(state.civs[me].capital);
  if (byId(state.units).some((u) => u.civ === me && u.dest === null)) {
    for (const to of targets) if (act({ type: 'moveCamp', to })) break;
  }
  if (n === 60) act({ type: 'disband', n: 1 });
}
const sim = new Sim(seed, radius);
const lines = [`0 ${sim.digest()}`];
for (let turn = 1; turn <= turns; turn++) {
  if (lead) leadOrders(turn - 1);
  // As native World::run does at the pick: watch.
  else if (sim.phase() === 'choose') sim.act('{"type":"chooseCiv","civ":null}');
  sim.act('{"type":"endTurn"}');
  lines.push(`${turn} ${sim.digest()}`);
}
console.log(lines.join('\n'));
