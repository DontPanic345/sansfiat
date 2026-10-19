// Army UI: zones, forts, captains and detachments; the captain panel (with his food and supply line,
// which the supply module computes), raising armies, promoting cut-off detachments.
// Selection lives in ui.local: {captain, mode}; mode is 'move' | 'paint' | 'unpaint' | null.
import { humanCiv, civCities, stepCost, tileAt, passable } from '../../core.js';
import { key, neighbors } from '../../hex.js';
import { actions, captains, cutOff, orphan, hungry } from './rules.js';
import { ARMY } from './data.js';
import { supplyLine, foodOf, eatRate, gatherError, actions as supplyActions } from '../supply/rules.js';
import { SUPPLY } from '../supply/data.js';

export const z = 20;

// Mode buttons are UI-only (no game action): a document listener flips local.mode, then a resize
// event makes main.js redraw the map and panels.
let lastLocal = null;
if (typeof document !== 'undefined') {
  document.addEventListener('click', (e) => {
    const el = e.target.closest?.('[data-army-mode]');
    if (!el || !lastLocal) return;
    const m = el.dataset.armyMode;
    lastLocal.mode = lastLocal.mode === m ? null : m;
    dispatchEvent(new Event('resize'));
  });
}

const esc = (t) => String(t).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
const btn = (a, label, err) => `<button data-action='${JSON.stringify(a)}'${err ? ` disabled title="${esc(err)}"` : ''}>${label}</button>`
  + (err ? ` <small class="why">${esc(err)}</small>` : '');
const check = (s, a, table = actions) => table[a.type].validate(s, a);
const visibleTo = (s, viewer) => (h) => viewer == null || !s.civs[viewer]?.seen || !!s.civs[viewer].seen[key(h)];

function selected(s, local) {
  const me = humanCiv(s);
  const u = local?.captain != null ? s.units[local.captain] : null;
  return me && u && u.type === 'captain' && u.owner === me.id ? u : null;
}

export function drawLayer(ctx, s, view) {
  const col = (o) => s.civs[o]?.color ?? '#888';
  const vis = visibleTo(s, view.viewer);
  const shown = (u) => u.owner === view.viewer || vis(u);
  ctx.save();
  for (const u of captains(s)) {
    ctx.globalAlpha = 0.25; ctx.fillStyle = col(u.owner);
    for (const k of u.army.zone) {
      const [q, r] = k.split(',').map(Number);
      if (u.owner !== view.viewer && !vis({ q, r })) continue;
      view.hexPath(ctx, view.at({ q, r }), view.size * 0.9); ctx.fill();
    }
  }
  ctx.globalAlpha = 1;
  // forts: a square bastion in the owner's colour
  for (const t of Object.values(s.map.tiles)) {
    if (!t.fort || (t.fort.owner !== view.viewer && !vis(t))) continue;
    const p = view.at(t), r = view.size * 0.55;
    ctx.strokeStyle = col(t.fort.owner); ctx.lineWidth = 3;
    ctx.strokeRect(p.x - r, p.y - r, 2 * r, 2 * r);
  }
  ctx.lineWidth = 1;
  ctx.textAlign = 'center'; ctx.textBaseline = 'middle'; ctx.font = `${Math.max(9, view.size * 0.5)}px sans-serif`;
  for (const u of Object.values(s.units)) {
    if ((u.type !== 'captain' && u.type !== 'detachment') || !shown(u)) continue;
    const p = view.at(u), cap = u.type === 'captain';
    ctx.fillStyle = col(u.owner); ctx.strokeStyle = hungry(u, s) ? '#f33' : '#000';
    ctx.lineWidth = hungry(u, s) ? 2 : 1;
    ctx.beginPath();
    if (cap) { // a captain is a shield-like pentagon
      const r = view.size * 0.45;
      ctx.moveTo(p.x - r, p.y - r * 0.8); ctx.lineTo(p.x + r, p.y - r * 0.8); ctx.lineTo(p.x + r, p.y + r * 0.2); ctx.lineTo(p.x, p.y + r); ctx.lineTo(p.x - r, p.y + r * 0.2); ctx.closePath();
    } else ctx.arc(p.x, p.y, view.size * 0.28, 0, 7);
    ctx.fill(); ctx.stroke();
    ctx.fillStyle = '#fff'; ctx.fillText(String(Math.round(u.army.troops)), p.x, p.y);
  }
  const sel = selected(s, view.local);
  if (sel) {
    const p = view.at(sel);
    ctx.strokeStyle = '#fff'; ctx.lineWidth = 3;
    ctx.beginPath(); ctx.arc(p.x, p.y, view.size * 0.7, 0, 7); ctx.stroke();
    const line = supplyLine(s, sel);
    if (line) {
      ctx.strokeStyle = '#e8d44d'; ctx.lineWidth = 2; ctx.setLineDash([4, 3]);
      ctx.beginPath();
      line.path.forEach((h, i) => { const c = view.at(h); if (i) ctx.lineTo(c.x, c.y); else ctx.moveTo(c.x, c.y); });
      ctx.stroke(); ctx.setLineDash([]);
    }
    if (view.local.mode === 'move') {
      ctx.font = `bold ${Math.max(10, view.size * 0.55)}px sans-serif`;
      for (const n of neighbors(sel)) {
        if (!tileAt(s, n) || !passable(s, n)) continue;
        const c = stepCost(s, n), ok = !check(s, { type: 'moveCaptain', civ: sel.owner, unit: sel.id, q: n.q, r: n.r });
        const at = view.at(n);
        ctx.strokeStyle = ok ? '#fff' : '#f55'; ctx.lineWidth = 2;
        view.hexPath(ctx, at, view.size * 0.85); ctx.stroke();
        ctx.fillStyle = ok ? '#fff' : '#f88'; ctx.fillText(String(c), at.x, at.y + view.size * 0.5);
      }
    }
  }
  ctx.restore();
}

function captainPanel(s, u) {
  const civ = u.owner, unit = u.id, f = foodOf(u), rate = eatRate(u);
  const line = supplyLine(s, u);
  const deployed = u.army.zone.length > 0;
  let h = `<div><b>Captain ${u.id}</b> (${esc(s.civs[civ]?.name ?? civ)}) at ${u.q},${u.r}: <b>${u.army.troops}</b> troops, `
    + `${deployed ? `deployed over ${u.army.zone.length} tiles` : 'marching as one body'}; labour ${u.labour}/${u.labourMax}</div>`;
  h += `<div>Food ${f.food.toFixed(1)}/${f.foodMax} — eats ${rate.toFixed(2)}/turn (${u.army.troops} troops × ${SUPPLY.eatPerTroop}), `
    + `+${SUPPLY.fightEat}/troop that fights; ${rate > 0 ? Math.floor(f.food / rate) : '∞'} turns left`
    + `${hungry(u, s) ? ' — <b style="color:#f55">HUNGRY: cannot attack, defends at half, loses troops</b>' : ''}</div>`;
  h += `<div>${line ? `Supply line: ${line.src.kind} at ${line.src.at.q},${line.src.at.r}, cost ${line.src.cost}, resupply takes ${line.src.labour} labour` : 'No supply line (no food in reach within line cost ' + SUPPLY.range + ')'}</div>`;
  const mode = (m, label) => `<button data-army-mode="${m}"${'' /* toggles */}>${lastLocal?.mode === m ? '<b>' + label + '</b>' : label}</button>`;
  h += `<div>Mode: ${mode('move', 'Move')} ${mode('paint', 'Paint zone')} ${mode('unpaint', 'Unpaint')}</div>`;
  const hint = { move: 'Click an adjacent tile to move there (labour cost shown on the map).', paint: 'Click tiles next to the zone (or next to the captain) to paint them.', unpaint: 'Click painted tiles to unpaint them.' }[lastLocal?.mode];
  h += `<div><i>${hint ? hint + ' Esc to cancel.' : 'Pick a mode, then click the map.'}</i></div>`;
  const a = (type, extra = {}) => ({ type, civ, unit, ...extra });
  h += '<div>'
    + btn(a('undeploy'), 'Undeploy', check(s, a('undeploy'))) + ' '
    + btn(a('buildFort'), `Build fort (${ARMY.fort.labour} labour, ${ARMY.fort.wood} wood, ${ARMY.fort.stone} stone)`, check(s, a('buildFort'))) + '<br>'
    + btn(a('forage'), `Forage (${SUPPLY.forageLabour} labour)`, gatherError(s, u, false)) + ' '
    + btn(a('pillage'), `Pillage (${SUPPLY.pillageLabour} labour)`, gatherError(s, u, true)) + '<br>'
    + btn(a('resupply'), `Resupply${line ? ` (${line.src.labour} labour)` : ''}`, check(s, a('resupply'), supplyActions)) + ' '
    + btn(a('autoSupply', { on: !f.auto }), `Auto-supply: ${f.auto ? 'on' : 'off'}`) + ' '
    + btn(a('disband'), 'Disband (troops go home)', check(s, a('disband')))
    + '</div>';
  return h;
}

export function panel(s, selection, ui) {
  const me = humanCiv(s);
  if (!me || s.phase !== 'main') return '';
  lastLocal = ui.local;
  const civ = me.id;
  const mine = captains(s, civ);
  let h = `<h3>Army</h3><div>Captains ${mine.length}/${ARMY.maxCaptains}</div>`;
  for (const c of civCities(s, civ)) {
    const a = { type: 'raiseArmy', civ, city: c.id };
    h += `<div>${btn(a, `Raise army at ${esc(c.name ?? c.id)} (${ARMY.size} people from clans, up to ${ARMY.size * ARMY.provision} food from the store)`, check(s, a))}</div>`;
  }
  const sel = selected(s, ui.local);
  if (sel) h += captainPanel(s, sel);
  else if (mine.length) h += `<div>${mine.map((u) => `Captain ${u.id} at ${u.q},${u.r}: ${u.army.troops} troops${hungry(u, s) ? ' (hungry)' : ''}`).join('<br>')}</div><div><i>Click one of your captains on the map to select him.</i></div>`;
  const cut = Object.values(s.units).filter((d) => d.type === 'detachment' && d.owner === civ && (orphan(s, d) || (!sel || d.army.captainId === sel.id) && cutOff(s, d)));
  for (const d of cut) {
    const a = { type: 'promote', civ, unit: d.id };
    h += `<div>Cut-off detachment at ${d.q},${d.r} (${d.army.troops} troops): ${btn(a, 'Promote to captain', check(s, a))}</div>`;
  }
  return h;
}

export function onClick(s, hex, ui) {
  const me = humanCiv(s);
  if (!me) return false;
  const L = ui.local;
  const cap = selected(s, L);
  const k = key(hex);
  if (cap && L.mode) {
    const a = { civ: me.id, unit: cap.id, q: hex.q, r: hex.r };
    if (L.mode === 'move') {
      if (k === key(cap)) { L.captain = null; L.mode = null; return true; }
      ui.act({ type: 'moveCaptain', ...a });
    } else ui.act({ type: L.mode, ...a });
    return true;
  }
  const here = captains(s, me.id).find((u) => key(u) === k);
  if (here) {
    if (L.captain === here.id) { L.captain = null; L.mode = null; } else { L.captain = here.id; L.mode = null; }
    return true;
  }
  return false;
}
