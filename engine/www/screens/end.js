// The end screen: Standing over the game as a line per civ in its colour (the player's thicker), from the
// page's Standing history (model.js). Until R adds a finished phase, the header says the game goes on;
// R fills `over` with {winner, why}. Play on returns to the game; New game goes to the start screen.

import { app } from '../app.js';

const $ = (id) => document.getElementById(id);
const pct = (x) => `${Math.round(x * 100)}%`;

export function init() {
  $('end-playon').onclick = () => app.show('play');
  $('end-new').onclick = () => app.show('start');
}

/** @param args {over: null | {winner, why}, history: [turn, {civ: standing}][], civs, player} */
export function show({ over = null, history = [], civs = {}, player = null } = {}) {
  const turn = history.length ? history[history.length - 1][0] : 0;
  $('end-head').textContent = over ? `${over.winner}: ${over.why}` : `The game goes on: turn ${turn}`;
  $('end-chart').innerHTML = chart(history, civs, player);
}

export function hide() {}

/** The Standing chart: turns across, Standing (0 to 100%) up, a line per civ, each one's last value labelled. */
function chart(history, civs, player) {
  if (!history.length) return '<p class="none">No turns since the pick yet.</p>';
  const W = 900, H = 380, L = 56, R = 170, T = 16, B = 40;
  const t0 = history[0][0];
  const t1 = history[history.length - 1][0];
  const x = (t) => L + (t1 === t0 ? 0 : (t - t0) / (t1 - t0)) * (W - L - R);
  const y = (s) => T + (1 - s) * (H - T - B);
  const grid = [0, 0.25, 0.5, 0.75, 1]
    .map((s) => `<line class="grid" x1="${L}" x2="${W - R}" y1="${y(s)}" y2="${y(s)}"/><text x="${L - 8}" y="${y(s) + 4}" text-anchor="end">${pct(s)}</text>`)
    .join('');
  const [lastTurn, lastAt] = history[history.length - 1];
  const lines = Object.keys(civs)
    .map((id) => {
      const c = civs[id];
      const pts = history.filter(([, at]) => at[id] !== undefined).map(([t, at]) => `${x(t).toFixed(1)},${y(at[id]).toFixed(1)}`);
      const mine = Number(id) === player;
      const end = lastAt[id] ?? 0;
      return `<polyline points="${pts.join(' ')}" fill="none" stroke="${c.color}" stroke-width="${mine ? 4 : 2}" stroke-linejoin="round"><title>${c.name}</title></polyline>` +
        `<text x="${x(lastTurn) + 8}" y="${y(end) + 4}" style="fill:${c.color}">${c.name}${mine ? ' (you)' : ''} ${pct(end)}</text>`;
    })
    .join('');
  const axis = `<text x="${L}" y="${H - 12}">turn ${t0}</text><text x="${W - R}" y="${H - 12}" text-anchor="end">turn ${t1}</text>`;
  return `<svg viewBox="0 0 ${W} ${H}" role="img" aria-label="Standing over the game, a line per civ">${grid}${lines}${axis}</svg>`;
}
