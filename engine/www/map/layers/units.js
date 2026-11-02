// Units: each army (camp), scout and guard is its figure over its mask tinted in its civ's colour, with its
// people beside it: one decimal under ten, whole above, and an army's archers split off as "6+2" with a small
// bow. Several units on one tile sit side by side and a click cycles them (as now). A hungry one is edged red,
// the selected one white. A fallen civ draws nothing.
import { tinted, icon } from './icon.js';

export const name = 'units';

/** An army's people as the figure's number: its warriors, and its archers split off ("6+2") when it has any. */
export function countLabel(u) {
  const [w = u.people, a = 0] = u.stack ?? [];
  const num = (x) => (x < 9.95 ? (Math.round(x * 10) / 10).toFixed(1) : String(Math.round(x)));
  return a >= 0.05 ? `${num(w)}+${num(a)}` : num(u.people);
}

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const slot = new Map(); // units on one tile sit side by side
  const labels = []; // the numbers drawn this frame, for the page's scripts (`view.stats.labels`)
  for (const [tile, list] of model.stacks) {
    if (!view.onScreen(tile)) continue;
    for (const u of list) {
      if (v.civs[u.civ]?.fallen) continue;
      const k = slot.get(tile) || 0;
      slot.set(tile, k + 1);
      const s = Math.max(0.6, px * 6);
      const x = view.px[tile] + k * s * 1.7 + s * 0.9, y = view.py[tile] + s * 1.1;
      const size = s * (64 / 22); // the cell's shield is 44 of its 64 px tall
      const colour = v.civs[u.civ]?.color || '#ccc';
      const hungry = u.hungry > 0;
      const selected = u.id === view.selectedUnit;
      const shield = () => {
        ctx.beginPath();
        ctx.moveTo(x - s * 0.8, y - s);
        ctx.lineTo(x + s * 0.8, y - s);
        ctx.lineTo(x + s * 0.8, y + s * 0.1);
        ctx.lineTo(x, y + s);
        ctx.lineTo(x - s * 0.8, y + s * 0.1);
        ctx.closePath();
        ctx.lineJoin = 'round';
      };
      if (hungry) {
        shield();
        ctx.strokeStyle = '#ff2a2a';
        ctx.lineWidth = Math.max(px * 7, 0.3);
        ctx.stroke();
      }
      tinted(ctx, view, `unit.${u.kind}.mask`, colour, x, y, size);
      icon(ctx, view, `unit.${u.kind}`, x, y, size);
      if (selected) {
        shield();
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = Math.max(px * 2.4, 0.06);
        ctx.stroke();
      }
      // The people beside the figure, on its right.
      ctx.font = `bold ${s * 1.1}px system-ui, sans-serif`;
      ctx.textAlign = 'left';
      ctx.textBaseline = 'middle';
      ctx.lineWidth = Math.max(px * 1.5, 0.05);
      ctx.strokeStyle = '#fff';
      ctx.fillStyle = '#000';
      const label = countLabel(u);
      labels.push({ unit: u.id, label });
      const tx = x + s * 0.95, ty = y - s * 0.3;
      ctx.strokeText(label, tx, ty);
      ctx.fillText(label, tx, ty);
      // Archers: a small bow over the figure's left shoulder.
      if (u.stack && u.stack[1] >= 0.05) {
        ctx.beginPath();
        ctx.arc(x - s * 0.9, y - s * 0.7, s * 0.45, -Math.PI / 2, Math.PI / 2);
        ctx.strokeStyle = '#000';
        ctx.lineWidth = Math.max(px * 1.2, 0.05);
        ctx.stroke();
      }
    }
  }
  view.stats.labels = labels;
}
