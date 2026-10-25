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
//   2. In the main game, when n > 0 and n % 20 == 0: makeUnit (a refusal is ignored).
//   3. Each of the player's units with no dest, in id order: moveUnit to the first accepted of the
//      other civs' capitals in civ id order, then its own capital.
//   4. When n == 60: disbandUnit the player's lowest unit id, if any.
// The state is read only on the pick turn and the turns after it (the order of the units needs it).
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
  if (n > 0 && n % 20 === 0) act({ type: 'makeUnit' });
  const state = JSON.parse(sim.state());
  const me = state.player;
  const civs = byId(state.civs);
  const targets = civs.filter((c) => c.id !== me).map((c) => c.capital).concat(state.civs[me].capital);
  for (const u of byId(state.units).filter((u) => u.civ === me)) {
    if (u.dest !== null) continue;
    for (const to of targets) if (act({ type: 'moveUnit', unit: u.id, to })) break;
  }
  if (n === 60) {
    const mine = byId(JSON.parse(sim.state()).units).filter((u) => u.civ === me);
    if (mine.length > 0) act({ type: 'disbandUnit', unit: mine[0].id });
  }
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
