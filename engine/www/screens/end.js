// The end screen: Standing over the game as a line per civ in its colour (the player's thicker), from the
// page's Standing history (model.js). Until R adds a finished phase, the header says the game goes on;
// R fills `over` with {winner, why}. Play on returns to the game; New game goes to the start screen.

import { app } from '../app.js';

const $ = (id) => document.getElementById(id);
const pct = (x) => `${x > 0 && x < 0.1 ? (x * 100).toFixed(1) : Math.round(x * 100)}%`;

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

/** The Standing chart: turns across, Standing up (to the next of 10, 25, 50 or 100% over the highest), a line
 *  per civ, each one's last value labelled at the right edge, the labels kept apart. One turn is a dot. */
function chart(history, civs, player) {
  if (!history.length) return '<p class="none">No turns since the pick yet.</p>';
  const W = 900, H = 380, L = 56, R = 170, T = 16, B = 40;
  const t0 = history[0][0];
  const t1 = history[history.length - 1][0];
  let high = 0;
  for (const [, at] of history) for (const s of Object.values(at)) high = Math.max(high, s);
  const top = [0.1, 0.25, 0.5, 1].find((t) => t >= high * 1.1) ?? 1;
  const x = (t) => L + (t1 === t0 ? 1 : (t - t0) / (t1 - t0)) * (W - L - R);
  const y = (s) => T + (1 - s / top) * (H - T - B);
  const grid = [0, 0.25, 0.5, 0.75, 1]
    .map((f) => `<line class="grid" x1="${L}" x2="${W - R}" y1="${y(f * top)}" y2="${y(f * top)}"/><text x="${L - 8}" y="${y(f * top) + 4}" text-anchor="end">${pct(f * top)}</text>`)
    .join('');
  const lastAt = history[history.length - 1][1];
  const ids = Object.keys(civs);
  // The labels, top to bottom by their value, each at least 16 px below the one above.
  const at = new Map();
  let prev = -Infinity;
  for (const id of [...ids].sort((a, b) => (lastAt[b] ?? 0) - (lastAt[a] ?? 0))) {
    prev = Math.max(y(lastAt[id] ?? 0) + 4, prev + 16);
    at.set(id, prev);
  }
  const lines = ids
    .map((id) => {
      const c = civs[id];
      const pts = history.filter(([, h]) => h[id] !== undefined).map(([t, h]) => `${x(t).toFixed(1)},${y(h[id]).toFixed(1)}`);
      const mine = Number(id) === player;
      const end = lastAt[id] ?? 0;
      const mark = pts.length > 1
        ? `<polyline points="${pts.join(' ')}" fill="none" stroke="${c.color}" stroke-width="${mine ? 4 : 2}" stroke-linejoin="round"><title>${c.name}</title></polyline>`
        : `<circle cx="${x(t1)}" cy="${y(end)}" r="${mine ? 5 : 4}" fill="${c.color}"><title>${c.name}</title></circle>`;
      return mark + `<text x="${W - R + 10}" y="${at.get(id)}" style="fill:${c.color}">${c.name}${mine ? ' (you)' : ''} ${pct(end)}</text>`;
    })
    .join('');
  const axis = `<text x="${L}" y="${H - 12}">turn ${t0}</text><text x="${W - R}" y="${H - 12}" text-anchor="end">turn ${t1}</text>`;
  return `<svg viewBox="0 0 ${W} ${H}" role="img" aria-label="Standing over the game, a line per civ">${grid}${lines}${axis}</svg>`;
}
