// Headless AI-vs-AI game: node tools/sim.js [--seed N] [--players N] [--turns N] [--json]
import { parseArgs } from 'node:util';
import { playTurn } from '../src/ai.js';
import { newGame } from '../src/game.js';
import { dispatch } from '../src/rules.js';

const { values: opt } = parseArgs({
  options: {
    seed: { type: 'string', default: '1' },
    players: { type: 'string', default: '2' },
    turns: { type: 'string', default: '100' },
    json: { type: 'boolean', default: false },
  },
});

let s = newGame({ seed: Number(opt.seed), players: Number(opt.players) });
while (s.turn <= Number(opt.turns) && s.winner == null) s = playTurn(s, dispatch);

if (opt.json) {
  console.log(JSON.stringify(s));
} else {
  console.log(`seed ${opt.seed}: turn ${s.turn}, winner ${s.winner == null ? '-' : s.players[s.winner].name}`);
  for (const p of s.players) {
    const units = Object.values(s.units).filter((u) => u.owner === p.id).length;
    const cities = Object.values(s.cities).filter((c) => c.owner === p.id).length;
    console.log(`  ${p.name.padEnd(5)} cities ${cities}  units ${units}${p.alive ? '' : '  (eliminated)'}`);
  }
}
