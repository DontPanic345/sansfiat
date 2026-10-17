// Headless game: node tools/sim.js [--seed N] [--players N] [--turns N] [--main N] [--every N] [--json]
// Runs the pre-game (up to --turns), then founding with every civ on the AI, then --main turns
// of the main game (default 0: stop at founding).
import { parseArgs } from 'node:util';
import { GOODS } from '../src/data.js';
import { newGame } from '../src/game.js';
import { dispatch, hearths, total, totalPop } from '../src/rules.js';

const { values: opt } = parseArgs({
  options: {
    seed: { type: 'string', default: '1' },
    players: { type: 'string', default: '3' },
    turns: { type: 'string', default: '500' },
    main: { type: 'string', default: '0' },
    every: { type: 'string', default: '25' },
    json: { type: 'boolean', default: false },
  },
});

const line = (s) => {
  const clans = Object.values(s.clans);
  const markets = Object.values(s.markets);
  const fed = clans.filter((c) => c.fed).length;
  const starving = clans.filter((c) => c.starving).length;
  const trading = clans.filter((c) => c.market != null).length;
  const volume = markets.reduce((a, m) => a + m.lastVolume, 0);
  return `t${String(s.turn - 1).padStart(4)}  pop ${totalPop(s).toFixed(0).padStart(5)}  clans ${String(clans.length).padStart(4)}` +
    `  growing ${String(fed).padStart(4)}  starving ${String(starving).padStart(4)}  trading ${String(trading).padStart(4)}  markets ${markets.length}  traded ${volume.toFixed(1)}` +
    `  hearths ${hearths(s).length}`;
};

const t0 = performance.now();
let s = newGame({ seed: Number(opt.seed), players: Number(opt.players) });
const every = Number(opt.every);
const endTurn = () => {
  const r = dispatch(s, { type: 'endTurn' });
  if (r.error) throw new Error(r.error);
  s = r.state;
  if (!opt.json && (s.turn - 1) % every === 0) console.log(line(s));
};
while (s.turn <= Number(opt.turns) && s.phase === 'pregame') endTurn();
const pregameMs = performance.now() - t0;
if (s.phase === 'choose') {
  s = dispatch(s, { type: 'chooseCiv', civ: null }).state;
  for (let i = 0; i < Number(opt.main); i++) endTurn();
}

if (opt.json) {
  console.log(JSON.stringify(s));
} else {
  console.log(line(s));
  console.log(s.phase === 'pregame' ? 'still in the pre-game'
    : `pre-game ended on turn ${s.stats.pop.length - Number(opt.main)} (${pregameMs.toFixed(0)} ms); ` +
      `${Object.keys(s.civs).length} civs, all AI; now turn ${s.turn - 1}, phase ${s.phase}`);
  for (const m of Object.values(s.markets)) {
    const owner = m.owner == null ? 'neutral' : s.civs[m.owner].name;
    const ratios = GOODS.filter((f) => m.prices[f]).map((f) => `${f} ${m.prices[f].toFixed(2)}`).join('  ');
    console.log(`  ${owner.padEnd(7)} @${m.q},${m.r}  since t${m.founded}  traders ${m.traders}  traded ${m.volume.toFixed(0)}  last ${total(m.lastTrade).toFixed(1)}  | ${ratios}`);
  }
}
