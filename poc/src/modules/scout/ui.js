// UI hooks for the scout module: trails, scouts, the selected scout's route, fog (drawn last),
// and the scouts panel. local: {unit, from} (selected scout id; a city the next route starts at).
import { key, neighbors } from '../../hex.js';
import { civCities, civUnits, humanCiv } from '../../core.js';
import { TERRAIN } from '../../data.js';
import { SCOUT } from './data.js';
import { actions, routePlan, visibleNow } from './rules.js';

export const z = 90;

const FOG_UNSEEN = '#141a22'; // opaque: nothing under it shows
const FOG_HINT = 0.16; // terrain colour laid faintly over unseen land
const FOG_MEMORY = 'rgba(12,16,22,0.42)';

export function drawLayer(ctx, s, view) {
  const civ = s.civs[view.viewer] ?? null; // null: watch mode or pre-game, everything shows
  const seen = (k) => !civ || civ.seen?.[k];
  const vis = civ ? visibleNow(s, civ.id) : null;
  const visible = (k) => !vis || vis.has(k);
  const { size } = view;
  ctx.save();
  ctx.lineCap = 'round';

  // Trails: a track between neighbouring trailed tiles, bridges as a plank over the mountain.
  const trailed = (h) => s.map.tiles[key(h)]?.trail && seen(key(h));
  for (const t of Object.values(s.map.tiles)) {
    if (!t.trail || !seen(key(t))) continue;
    const c = view.at(t);
    ctx.strokeStyle = '#d9b277';
    ctx.lineWidth = Math.max(1.5, size * 0.16);
    let linked = false;
    for (const n of neighbors(t)) {
      if (!trailed(n)) continue;
      linked = true;
      const d = view.at(n);
      ctx.beginPath(); ctx.moveTo(c.x, c.y); ctx.lineTo((c.x + d.x) / 2, (c.y + d.y) / 2); ctx.stroke();
    }
    if (t.terrain === 'mountain') {
      ctx.fillStyle = '#d8b26a'; ctx.strokeStyle = '#3b2a17'; ctx.lineWidth = 1;
      ctx.fillRect(c.x - size * 0.45, c.y - size * 0.14, size * 0.9, size * 0.28);
      ctx.strokeRect(c.x - size * 0.45, c.y - size * 0.14, size * 0.9, size * 0.28);
    } else if (!linked) {
      ctx.fillStyle = '#d9b277'; ctx.strokeStyle = '#3b2a17'; ctx.lineWidth = 1;
      ctx.beginPath(); ctx.arc(c.x, c.y, size * 0.2, 0, 7); ctx.fill(); ctx.stroke();
    }
  }

  // Fog: unseen land nearly black; remembered land not in view dimmed.
  if (civ) {
    for (const t of Object.values(s.map.tiles)) {
      const k = key(t);
      if (visible(k)) continue;
      view.hexPath(ctx, view.at(t), size + 0.6);
      if (civ.seen?.[k]) { ctx.fillStyle = FOG_MEMORY; ctx.fill(); continue; }
      ctx.fillStyle = FOG_UNSEEN; ctx.fill();
      ctx.globalAlpha = FOG_HINT; ctx.fillStyle = TERRAIN[t.terrain]?.color ?? '#000'; ctx.fill();
      ctx.globalAlpha = 1;
    }
  }

  // The selected scout's route: known (sure) path gold, best possible path blue dashed.
  const sel = s.units[view.local?.unit];
  if (sel?.scout?.route && (!civ || sel.owner === civ.id)) {
    const rt = sel.scout.route;
    const plan = routePlan(s, sel.owner, rt.from, rt.to);
    ctx.lineWidth = 2;
    for (const [p, col, dash] of [[plan.possible, '#6cf', [4, 3]], [plan.known, '#fc4', []]]) {
      if (!p) continue;
      ctx.strokeStyle = col; ctx.setLineDash(dash);
      ctx.beginPath();
      p.path.forEach((h, i) => { const c = view.at(h); i ? ctx.lineTo(c.x, c.y) : ctx.moveTo(c.x, c.y); });
      ctx.stroke();
    }
    ctx.setLineDash([]);
    const end = view.at(rt.to);
    ctx.strokeStyle = '#fc4'; ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(end.x, end.y, size * 0.5, 0, 7); ctx.stroke();
  }
  if (civ && view.local?.from) {
    const c = view.at(view.local.from);
    ctx.strokeStyle = '#6cf'; ctx.lineWidth = 2; ctx.setLineDash([3, 2]);
    ctx.beginPath(); ctx.arc(c.x, c.y, size * 0.7, 0, 7); ctx.stroke();
    ctx.setLineDash([]);
  }

  // Revealed hidden costs, on tiles in sight or memory.
  ctx.fillStyle = '#fff';
  for (const k of Object.keys(civ?.revealed ?? {})) {
    const [q, r] = k.split(',').map(Number);
    const c = view.at({ q, r });
    ctx.fillRect(c.x - 1, c.y - 1, 3, 3);
  }

  // Scouts: a small flag in the owner's colour; others' only where the viewer sees now.
  for (const u of Object.values(s.units)) {
    if (u.type !== 'scout') continue;
    if (civ && u.owner !== civ.id && !visible(key(u))) continue;
    const c = view.at(u);
    const x = c.x + size * 0.3, y = c.y - size * 0.1;
    if (u.id === sel?.id) {
      ctx.strokeStyle = '#ff0'; ctx.lineWidth = 2.5;
      ctx.beginPath(); ctx.arc(c.x, c.y, size * 0.75, 0, 7); ctx.stroke();
    }
    ctx.strokeStyle = '#111'; ctx.lineWidth = 1.5;
    ctx.beginPath(); ctx.moveTo(x, y + size * 0.45); ctx.lineTo(x, y - size * 0.45); ctx.stroke();
    ctx.fillStyle = s.civs[u.owner]?.color ?? '#ccc';
    ctx.beginPath(); ctx.moveTo(x, y - size * 0.45); ctx.lineTo(x + size * 0.45, y - size * 0.28); ctx.lineTo(x, y - size * 0.1);
    ctx.closePath(); ctx.fill(); ctx.lineWidth = 1; ctx.stroke();
  }
  ctx.restore();
}

const attr = (a) => `data-action='${JSON.stringify(a)}'`;
const button = (label, a, err) => `<button ${attr(a)}${err ? ` disabled title="${err}"` : ''}>${label}</button>`;

export function panel(s, selection, ui) {
  const me = humanCiv(s);
  if (!me || s.phase !== 'main') return '';
  const scouts = civUnits(s, me.id).filter((u) => u.type === 'scout');
  const local = ui?.local ?? {};
  const sel = scouts.find((u) => u.id === local.unit) ?? null;
  const rows = scouts.map((u) => {
    const rt = u.scout?.route;
    const doing = rt ? `${rt.walking ? 'walking' : 'scouting'} to ${rt.to.q},${rt.to.r}` : 'idle';
    return `<div>${u.id === sel?.id ? '<b>&gt; ' : ''}scout ${u.id} at ${u.q},${u.r}, labour ${u.labour}/${u.labourMax}, ${doing}${u.id === sel?.id ? '</b>' : ''}</div>`;
  }).join('');
  const build = { type: 'buildScout', civ: me.id };
  let out = `<h3>Scouts</h3>${rows || '<div>No scouts.</div>'}` +
    `<div>${button(`Build scout (free, ${scouts.length}/${SCOUT.maxPerCiv})`, build, actions.buildScout.validate(s, build))}</div>`;
  if (!sel) {
    if (scouts.length) out += '<div>Click one of your scouts to select it.</div>';
  } else {
    const rt = sel.scout?.route;
    if (rt) {
      const p = routePlan(s, me.id, rt.from, rt.to);
      out += `<div>Route ${rt.from.q},${rt.from.r} to ${rt.to.q},${rt.to.r}: known cost ${p.known?.cost ?? '-'}, ` +
        `best ${p.possible?.cost ?? '-'}, ${p.unknown.length} unrevealed. ` +
        `${button('Stop', { type: 'cancelRoute', civ: me.id, unit: sel.id })}</div>`;
    }
    const from = local.from;
    out += from
      ? `<div><b>Click a tile: scout ${sel.id} will scout and trail the route from ${from.name ?? `${from.q},${from.r}`} to it.</b> ` +
        `(Click the city again to send the scout there; Esc to cancel.)</div>`
      : `<div><b>Click a tile to send scout ${sel.id} there (Esc to cancel).</b> ` +
        `To trail a route between two places, click one of your cities first: the route then starts there.</div>`;
  }
  out += `<div>Revealed costs: ${Object.keys(me.revealed ?? {}).length}. Gold line: sure route; blue dashes: best if hidden costs are kind.</div>`;
  return out;
}

// Clicking your scout selects it (again: deselects, or selects the next scout on that tile).
// With a scout selected, clicking a tile sends it there; clicking one of your cities first makes
// that city the start of the route (click it again to send the scout to it).
export function onClick(s, hex, ui) {
  const me = humanCiv(s);
  const local = ui.local;
  if (!me || s.phase !== 'main') return false;
  const at = (h) => h.q === hex.q && h.r === hex.r;
  const here = civUnits(s, me.id).filter((u) => u.type === 'scout' && at(u));
  const u = s.units[local.unit];
  const mine = u && u.owner === me.id && u.type === 'scout' ? u : null;
  if (here.length && !local.from) {
    const i = mine ? here.findIndex((x) => x.id === mine.id) : -1;
    if (i < 0) local.unit = here[0].id;
    else if (i + 1 < here.length) local.unit = here[i + 1].id;
    else delete local.unit;
    return false; // the tile stays inspected (a city under the scout still shows)
  }
  if (!mine) return false;
  const city = civCities(s, me.id).find(at);
  if (city && !local.from) {
    local.from = { q: city.q, r: city.r, name: city.name };
    return false;
  }
  const from = local.from && !at(local.from) ? { q: local.from.q, r: local.from.r } : { q: mine.q, r: mine.r };
  delete local.from;
  ui.act({ type: 'assignRoute', civ: me.id, unit: mine.id, from, to: { q: hex.q, r: hex.r } });
  return true;
}
