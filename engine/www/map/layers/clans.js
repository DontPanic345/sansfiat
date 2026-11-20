// Clans: a circle at home, its area by people, in its civ's colour, or pale for a clan of no civ; a red
// ring when starving. A settled clan's hearth carries a dark square in its middle. Clans sharing a home are drawn as small hearths in a ring round the home's centre,
// capped below a lone clan's size, so each stays visible.

export const name = 'clans';

/** The fill of a clan of no civ. */
export const UNALIGNED = '#d8d2c4';

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
  const civId = model.clanCiv.get(c.id);
  ctx.beginPath();
  ctx.arc(x, y, r, 0, 2 * Math.PI);
  ctx.fillStyle = civId != null ? model.v.civs[civId].color : UNALIGNED;
  ctx.fill();
  ctx.lineWidth = Math.max(px, 0.04);
  ctx.strokeStyle = 'rgba(0,0,0,0.75)';
  ctx.stroke();
  if (c.settled) {
    const h = Math.max(r * 0.42, px * 1.5);
    ctx.fillStyle = '#2a1a0a';
    ctx.fillRect(x - h, y - h, 2 * h, 2 * h);
    ctx.lineWidth = Math.max(px * 0.8, 0.025);
    ctx.strokeStyle = '#f2dca4';
    ctx.strokeRect(x - h, y - h, 2 * h, 2 * h);
  }
  if (c.starving) {
    ctx.beginPath();
    ctx.arc(x, y, r + px * 1.6, 0, 2 * Math.PI);
    ctx.lineWidth = Math.max(px * 2.2, 0.1);
    ctx.strokeStyle = '#ff2a2a';
    ctx.stroke();
  }
}

