// Units: a shield per civ per tile, side by side, with a count when several; hungry ones edged red, the
// selected one white. Each is the atlas's figure over its mask tinted in the civ's colour.
import { tinted, icon } from './icon.js';

export const name = 'units';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const groups = new Map();
  for (const [tile, list] of model.stacks) {
    if (!view.onScreen(tile)) continue;
    for (const u of list) {
      const key = `${tile}:${u.civ}`;
      let g = groups.get(key);
      if (!g) groups.set(key, (g = { tile, civ: u.civ, n: 0, hungry: false, selected: false, kind: u.kind }));
      g.n++;
      if (u.hungry > 0) g.hungry = true;
      if (u.id === view.selectedUnit) g.selected = true;
    }
  }
  const slot = new Map(); // shields on one tile sit side by side
  for (const g of [...groups.values()].sort((a, b) => a.tile - b.tile || a.civ - b.civ)) {
    const k = slot.get(g.tile) || 0;
    slot.set(g.tile, k + 1);
    const s = Math.max(0.6, px * 6);
    const x = view.px[g.tile] + k * s * 1.7 + s * 0.9, y = view.py[g.tile] + s * 1.1;
    const size = s * (64 / 22) ; // the cell's shield is 44 of its 64 px tall
    const colour = v.civs[g.civ]?.color || '#ccc';
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
    if (g.hungry) {
      shield();
      ctx.strokeStyle = '#ff2a2a';
      ctx.lineWidth = Math.max(px * 7, 0.3);
      ctx.stroke();
    }
    tinted(ctx, view, `unit.${g.kind}.mask`, colour, x, y, size);
    icon(ctx, view, `unit.${g.kind}`, x, y, size);
    if (g.selected) {
      shield();
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = Math.max(px * 2.4, 0.06);
      ctx.stroke();
    }
    if (g.n > 1 && view.k >= 8) {
      ctx.font = `bold ${s * 1.3}px system-ui, sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillStyle = '#000';
      ctx.fillText(String(g.n), x, y - s * 0.1);
    }
  }
}
