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
// lead_orders). `pick` is the turn chooseCiv was acted, and n the turns since. Each order is the
// led civ's `order` action: it names the civ and, for a unit's order, the unit (its camp or scout).
//   1. At the pick: lead the lowest civ id.
//   2. In the main game, when n > 0 and n % 20 == 0: raise one warrior (a refusal is ignored).
//   3. When n == 30: switches both off; when n == 40: both on. When n > 0 and n % 25 == 0: resupply.
//   4. If the player's camp has no dest: move to the first accepted of the other civs' capitals
//      in civ id order, then its own capital.
//   5. When n == 60: disband one warrior (ignored without a camp).
//   6. When n == 10: setGift twice the default (2 * LEANING.gift_default); when n == 50: setKeep 5;
//      when n == 70: setKeep null.
//   7. When n == 2: the scout searches from the capital for the first forest or hills tile by index
//      seven steps out (ignored when there is none).
//   8. When n == 45, if there is a camp: move to its own tile (it halts), then zone over the first
//      people - 1 tiles of its line (the capital left out; a refusal is ignored). Rule 4 rests while
//      45 <= n < 55, so the zone spreads and is held.
// The state is read only on the pick turn and the turns after it (the camp's dest needs it).
const GIFT_DEFAULT = 2.0; // LEANING.gift_default
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
  const unitOf = (kind) => {
    const state = JSON.parse(sim.state());
    return byId(state.units).find((u) => u.civ === state.player && u.kind === kind);
  };
  const me = JSON.parse(sim.state()).player;
  const order = (o) => ({ type: 'order', civ: me, order: o });
  if (n > 0 && n % 20 === 0) act(order({ type: 'raise', n: 1 }));
  if (n === 30 && unitOf('camp')) act(order({ type: 'switches', unit: unitOf('camp').id, forage: false, supply: false }));
  if (n === 40 && unitOf('camp')) act(order({ type: 'switches', unit: unitOf('camp').id, forage: true, supply: true }));
  if (n > 0 && n % 25 === 0 && unitOf('camp')) act(order({ type: 'resupply', unit: unitOf('camp').id }));
  const state = JSON.parse(sim.state());
  const targets = byId(state.civs).filter((c) => c.id !== me).map((c) => c.capital).concat(state.civs[me].capital);
  if (n === 2) {
    const layout = JSON.parse(sim.layout());
    const cap = state.civs[me].capital;
    const dist = (a, b) => {
      const dq = layout.q[a] - layout.q[b];
      const dr = layout.r[a] - layout.r[b];
      return (Math.abs(dq) + Math.abs(dr) + Math.abs(dq + dr)) / 2;
    };
    const to = layout.terrain.findIndex((t, i) => (t === 3 || t === 4) && dist(i, cap) === 7);
    if (to >= 0 && unitOf('scout')) act(order({ type: 'route', unit: unitOf('scout').id, from: cap, to }));
  }
  const camp = byId(state.units).find((u) => u.civ === me && u.kind === 'camp');
  if (!(n >= 45 && n < 55) && camp && camp.dest === null) {
    for (const to of targets) if (act(order({ type: 'move', unit: camp.id, to }))) break;
  }
  if (n === 45 && camp) {
    act(order({ type: 'move', unit: camp.id, to: camp.home }));
    const line = JSON.parse(sim.view()).units[camp.id].line;
    if (line) {
      const capital = state.civs[me].capital;
      const k = Math.max(0, Math.floor(camp.people) - 1);
      act(order({ type: 'zone', unit: camp.id, tiles: line.path.filter((t) => t !== capital).slice(0, k) }));
    }
  }
  if (n === 60 && unitOf('camp')) act(order({ type: 'disband', unit: unitOf('camp').id, n: 1 }));
  if (n === 10) act({ type: 'setGift', food: 2 * GIFT_DEFAULT });
  if (n === 50) act({ type: 'setKeep', food: 5 });
  if (n === 70) act({ type: 'setKeep', food: null });
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
