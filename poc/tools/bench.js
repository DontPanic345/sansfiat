// Times the headless pre-game to founding: node tools/bench.js [--players N] [seeds...]
import { parseArgs } from 'node:util';
import { newGame, runPregameCounted } from '../src/game.js';

const { values: opt, positionals } = parseArgs({ allowPositionals: true, options: { players: { type: 'string', default: '3' } } });
for (const seed of (positionals.length ? positionals : ['1', '2', '3']).map(Number)) {
  const t0 = performance.now();
  const { state, turns } = runPregameCounted(newGame({ seed, players: Number(opt.players) }));
  console.log(`seed ${seed}: ${(performance.now() - t0).toFixed(0)} ms for ${turns} turns -> ${state.phase}, ` +
    `${Object.keys(state.clans).length} clans, ${Object.keys(state.markets).length} markets`);
}
