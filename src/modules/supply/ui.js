// Supply UI: caches and porters on the map; the porter panel (build, load, unload, route).
// The selected captain's food and supply line live in the army panel. ui.local: {porter, mode}.
import { humanCiv, civCities, civUnits, unitsAt } from '../../core.js';
import { key } from '../../hex.js';
import { actions, foodIn } from './rules.js';
import { SUPPLY } from './data.js';

export const z = 15; // before army, so a porter in route mode gets the click on a captain's tile

let lastLocal = null;
if (typeof document !== 'undefined') {
  document.addEventListener('click', (e) => {
    const el = e.target.closest?.('[data-supply-ui]');
    if (!el || !lastLocal) return;
    const [what, v] = el.dataset.supplyUi.split(':');
    if (what === 'porter') { lastLocal.porter = lastLocal.porter === Number(v) ? null : Number(v); lastLocal.mode = null; }
    if (what === 'route') lastLocal.mode = lastLocal.mode === 'route' ? null : 'route';
    dispatchEvent(new Event('resize'));
  });
}

const esc = (t) => String(t).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
const btn = (a, label, err) => `<button data-action='${JSON.stringify(a)}'${err ? ` disabled title="${esc(err)}"` : ''}>${label}</button>`
  + (err ? ` <small class="why">${esc(err)}</small>` : '');
const check = (s, a) => actions[a.type].validate(s, a);
const visibleTo = (s, viewer) => (h) => viewer == null || !s.civs[viewer]?.seen || !!s.civs[viewer].seen[key(h)];

function selected(s, local) {
  const me = humanCiv(s);
  const u = local?.porter != null ? s.units[local.porter] : null;
  return me && u && u.type === 'porter' && u.owner === me.id ? u : null;
}

export function drawLayer(ctx, s, view) {
  const vis = visibleTo(s, view.viewer);
  const col = (o) => s.civs[o]?.color ?? '#888';
  ctx.save();
  for (const t of Object.values(s.map.tiles)) { // caches: a sack of food in the owner's colour
    if (!t.cache || (t.cache.owner !== view.viewer && !vis(t))) continue;
    const p = view.at(t), r = view.size * 0.22, x = p.x - view.size * 0.45, y = p.y + view.size * 0.35;
    ctx.fillStyle = '#d8a22b'; ctx.strokeStyle = col(t.cache.owner); ctx.lineWidth = 2;
    ctx.fillRect(x - r, y - r, 2 * r, 2 * r); ctx.strokeRect(x - r, y - r, 2 * r, 2 * r);
  }
  for (const u of Object.values(s.units)) { // porters: a small diamond, lower right
    if (u.type !== 'porter' || (u.owner !== view.viewer && !vis(u))) continue;
    const p = view.at(u), r = view.size * 0.25, x = p.x + view.size * 0.45, y = p.y + view.size * 0.35;
    ctx.fillStyle = col(u.owner); ctx.strokeStyle = u.supply?.food > 0 ? '#d8a22b' : '#000'; ctx.lineWidth = 2;
    ctx.beginPath(); ctx.moveTo(x, y - r); ctx.lineTo(x + r, y); ctx.lineTo(x, y + r); ctx.lineTo(x - r, y); ctx.closePath();
    ctx.fill(); ctx.stroke();
  }
  const p = selected(s, view.local);
  if (p) {
    const c = view.at(p);
    ctx.strokeStyle = '#fff'; ctx.lineWidth = 2;
    ctx.beginPath(); ctx.arc(c.x + view.size * 0.45, c.y + view.size * 0.35, view.size * 0.4, 0, 7); ctx.stroke();
    const rt = p.supply.route, city = rt && s.cities[rt.cityId];
    if (rt && city) {
      ctx.strokeStyle = '#d8a22b'; ctx.setLineDash([2, 4]);
      const a = view.at(city), b = view.at(rt.to);
      ctx.beginPath(); ctx.moveTo(a.x, a.y); ctx.lineTo(b.x, b.y); ctx.stroke(); ctx.setLineDash([]);
    }
  }
  ctx.restore();
}

export function panel(s, selection, ui) {
  const me = humanCiv(s);
  if (!me || s.phase !== 'main') return '';
  lastLocal = ui.local;
  const civ = me.id;
  let h = `<h3>Supply</h3><div>Food in the store: ${foodIn(s, civ).toFixed(1)}</div>`;
  for (const c of civCities(s, civ)) {
    const a = { type: 'buildPorter', civ, city: c.id };
    h += `<div>${btn(a, `Build porter at ${esc(c.name ?? c.id)} (${SUPPLY.porterCost} food; carries ${SUPPLY.porterCarry}, eats ${SUPPLY.porterEat}/turn while loaded)`, check(s, a))}</div>`;
  }
  const ps = civUnits(s, civ).filter((u) => u.type === 'porter');
  const sel = selected(s, ui.local);
  for (const p of ps) {
    const rt = p.supply.route;
    const where = rt ? `route ${s.cities[rt.cityId]?.name ?? rt.cityId} → ${rt.to.q},${rt.to.r} (keep ${rt.target})` : p.supply.returning != null ? 'returning home' : 'idle';
    h += `<div><button data-supply-ui="porter:${p.id}">${p.id === sel?.id ? '<b>Porter ' + p.id + '</b>' : 'Porter ' + p.id}</button> at ${p.q},${p.r}, food ${p.supply.food.toFixed(1)}/${p.supply.foodMax}, ${where}</div>`;
  }
  if (sel) {
    const a = (type, extra = {}) => ({ type, civ, unit: sel.id, ...extra });
    h += '<div>'
      + btn(a('loadPorter'), 'Load (in your city)', check(s, a('loadPorter'))) + ' '
      + btn(a('unloadPorter'), 'Unload here', check(s, a('unloadPorter'))) + ' '
      + `<button data-supply-ui="route">${ui.local.mode === 'route' ? '<b>Set route…</b>' : 'Set route'}</button> `
      + btn(a('clearPorterRoute'), 'Clear route', sel.supply.route ? null : 'no route')
      + '</div>';
    h += `<div><i>${ui.local.mode === 'route' ? 'Click a tile (an army or a cache site) to send porter ' + sel.id + ' there from the nearest city. Esc to cancel.' : 'Set route, then click a target tile.'}</i></div>`;
  } else if (ps.length) h += '<div><i>Select a porter above (or click it on the map) to give it orders.</i></div>';
  const caches = Object.values(s.map.tiles).filter((t) => t.cache?.owner === civ);
  if (caches.length) h += `<div>Caches: ${caches.map((t) => `${t.q},${t.r} (${t.cache.food.toFixed(1)})`).join(', ')}; spoil ${SUPPLY.cacheSpoil * 100}%/turn</div>`;
  return h;
}

export function onClick(s, hex, ui) {
  const me = humanCiv(s);
  if (!me) return false;
  const L = ui.local;
  const sel = selected(s, L);
  if (sel && L.mode === 'route') {
    ui.act({ type: 'setPorterRoute', civ: me.id, unit: sel.id, to: { q: hex.q, r: hex.r } });
    L.mode = null;
    return true;
  }
  // a porter's tile selects it, unless one of the human's captains stands there (army handles that)
  const here = unitsAt(s, hex);
  const p = here.find((u) => u.type === 'porter' && u.owner === me.id);
  if (!p || here.some((u) => u.type === 'captain' && u.owner === me.id)) return false;
  if (L.porter === p.id) { L.porter = null; L.mode = null; } else { L.porter = p.id; L.mode = null; }
  return true;
}
