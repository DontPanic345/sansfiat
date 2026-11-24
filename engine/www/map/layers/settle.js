// Signposts and settlements: a signpost is a small post with a board in its civ's colour, at every zoom; a settlement is a
// small house with a dark casing in its civ's colour, drawn smaller than the capital's, at the flat zoom only (above it
// places.js draws the settlement's palisade). Plain shapes, no ring.
import { placesFlat } from './places.js';

export const name = 'settle';

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const u = Math.max(0.5, px * 14);
  ctx.lineJoin = 'round';
  for (const s of model.v.signposts ?? []) {
    if (!view.onScreen(s.tile)) continue;
    const x = view.px[s.tile], y = view.py[s.tile];
    const colour = model.v.civs[s.civ]?.color ?? '#fff';
    ctx.lineWidth = Math.max(px * 1.5, 0.05);
    ctx.strokeStyle = '#222';
    ctx.fillStyle = '#eee';
    ctx.beginPath(); // the post
    ctx.rect(x - u * 0.1, y - u * 0.9, u * 0.2, u * 1.9);
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = colour;
    ctx.beginPath(); // the board
    ctx.rect(x - u * 0.2, y - u * 0.9, u * 1.0, u * 0.7);
    ctx.fill();
    ctx.stroke();
  }
  if (!placesFlat(view)) return;
  for (const s of model.v.settlements ?? []) {
    if (!view.onScreen(s.tile)) continue;
    const x = view.px[s.tile], y = view.py[s.tile];
    const w = u * 0.9;
    ctx.lineWidth = Math.max(px * 1.8, 0.06);
    ctx.strokeStyle = '#222';
    ctx.fillStyle = model.v.civs[s.civ]?.color ?? '#fff';
    ctx.beginPath(); // a house: walls and a roof
    ctx.moveTo(x - w, y + w * 0.8);
    ctx.lineTo(x - w, y - w * 0.1);
    ctx.lineTo(x, y - w * 0.95);
    ctx.lineTo(x + w, y - w * 0.1);
    ctx.lineTo(x + w, y + w * 0.8);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = '#fff';
    ctx.fillRect(x - w * 0.18, y + w * 0.1, w * 0.36, w * 0.7);
  }
}
