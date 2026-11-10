// Clans: a circle at home, its area by people, in the clan's colour; a red ring when starving. A clan with
// a civ gets a ring in the civ's colour (with the civ's land). Clans sharing a home are drawn as small
// hearths in a ring round the home's centre, each in its own colour and capped below a lone clan's size, so
// each stays visible; a lone clan draws as before.
import { hue } from '../geom.js';

export const name = 'clans';

/** The most a clan on a shared home is drawn at, in world units; a lone clan's radius is up to 0.75. */
const SHARED_MAX = 0.3;
/** How far from the home's centre a clan on a shared home sits, in world units. */
const RING = 0.34;

export function draw(ctx, model, view) {
  const px = 1 / view.k;
  const { v } = model;
  const byHome = new Map();
  for (const c of Object.values(v.clans)) {
    if (!byHome.has(c.home)) byHome.set(c.home, []);
    byHome.get(c.home).push(c);
  }
  for (const [home, clans] of byHome) {
    if (!view.onScreen(home)) continue;
    clans.sort((a, b) => a.id - b.id);
    const n = clans.length;
    clans.forEach((c, k) => {
      if (n === 1) {
        drawClan(ctx, model, view, c, view.px[home], view.py[home], Math.min(0.75, Math.max(px * 2.2, 0.16 * Math.sqrt(c.people))), px);
        return;
      }
      const a = -Math.PI / 2 + (2 * Math.PI * k) / n;
      const x = view.px[home] + RING * Math.cos(a), y = view.py[home] + RING * Math.sin(a);
      const r = Math.min(SHARED_MAX, Math.max(px * 2.2, 0.1 * Math.sqrt(c.people)));
      drawClan(ctx, model, view, c, x, y, r, px);
    });
  }
}

function drawClan(ctx, model, view, c, x, y, r, px) {
  const { v } = model;
  const casing = 'rgba(0,0,0,0.8)';
  ctx.beginPath();
  ctx.arc(x, y, r, 0, 2 * Math.PI);
  ctx.fillStyle = hue(c.id);
  ctx.fill();
  ctx.lineWidth = Math.max(px, 0.04);
  ctx.strokeStyle = 'rgba(0,0,0,0.75)';
  ctx.stroke();
  if (c.starving) {
    ctx.beginPath();
    ctx.arc(x, y, r + px * 1.6, 0, 2 * Math.PI);
    ctx.lineWidth = Math.max(px * 2.2, 0.1);
    ctx.strokeStyle = '#ff2a2a';
    ctx.stroke();
  }
  const civId = model.clanCiv.get(c.id);
  if (civId != null && view.on.land) {
    const rr = r + px * 3.4;
    for (const [col, w] of [[casing, 4.2], [v.civs[civId].color, 2.2]]) {
      ctx.strokeStyle = col;
      ctx.lineWidth = Math.max(px * w, 0.08);
      ctx.beginPath();
      ctx.arc(x, y, rr, 0, 2 * Math.PI);
      ctx.stroke();
    }
  }
}
