// UI hooks for the civ module: cities and territory on the map; the human civ's panel (store,
// posting, labour, projects, discoveries, years, prosperity, leanings), and actions on the
// selected tile (gifts to an unaligned clan, next city / daughter site, a city of yours).
import { CITY, CLAN, FOODS, GOODS } from '../../data.js';
import { cityAt, civCities, civClans, clanAt, humanCiv, territoryOf } from '../../core.js';
import { key, neighbors, within } from '../../hex.js';
import { CIV } from './data.js';
import {
  capOf, citySiteError, cityNeeds, clanSpoil, giftError, giftGain, keepOf, projectError, prosperity, regionOf,
  siteError, storeSpoil,
} from './rules.js';

export const z = 10;

const fmt = (x) => (Math.abs(x) >= 10 ? x.toFixed(0) : x.toFixed(1));
const pct = (x) => `${Math.round(x * 100)}%`;
const btn = (action, label, err) =>
  `<button data-action='${JSON.stringify(action)}'${err ? ` disabled title="${err.replace(/"/g, '&quot;')}"` : ''}>${label}</button>`;
const why = (err) => (err ? `<span class="sub"> (${err})</span>` : '');

// What the viewer may see of a tile: everything when there is no fog (watch mode, no scout yet).
const seenBy = (s, viewer) => {
  const seen = s.civs[viewer]?.seen;
  return (h) => !seen || !!seen[key(h)];
};

export function drawLayer(ctx, s, view) {
  if (s.phase !== 'main') return;
  const visible = seenBy(s, view.viewer);
  const { size } = view;
  // territory: edges between a civ's ground and anything else
  const owner = new Map();
  for (const c of Object.values(s.cities))
    for (const h of within(c, CITY.territoryRadius)) {
      const k = key(h);
      if (!owner.has(k) && s.map.tiles[k]) owner.set(k, territoryOf(s, h));
    }
  ctx.lineWidth = Math.max(1.5, size * 0.14);
  ctx.setLineDash([size * 0.3, size * 0.2]);
  for (const civ of Object.values(s.civs)) {
    ctx.beginPath();
    for (const [k, o] of owner) {
      if (o !== civ.id) continue;
      const t = s.map.tiles[k];
      if (civ.id !== view.viewer && !visible(t)) continue;
      const a = view.at(t);
      for (const n of neighbors(t)) {
        if (owner.get(key(n)) === civ.id) continue;
        const b = view.at(n);
        const mx = (a.x + b.x) / 2, my = (a.y + b.y) / 2;
        const dx = b.x - a.x, dy = b.y - a.y, len = Math.hypot(dx, dy);
        const px = (-dy / len) * size * 0.55, py = (dx / len) * size * 0.55;
        ctx.moveTo(mx - px, my - py);
        ctx.lineTo(mx + px, my + py);
      }
    }
    ctx.strokeStyle = civ.color;
    ctx.stroke();
  }
  ctx.setLineDash([]);
  // cities: a house in the civ colour (a crowned one for a founding city) with its name
  for (const city of Object.values(s.cities)) {
    if (city.owner !== view.viewer && !visible(city)) continue;
    const { x, y } = view.at(city);
    const w = size * 0.8;
    ctx.beginPath();
    ctx.moveTo(x - w / 2, y + w / 2);
    ctx.lineTo(x - w / 2, y - w / 8);
    ctx.lineTo(x, y - w / 1.6);
    ctx.lineTo(x + w / 2, y - w / 8);
    ctx.lineTo(x + w / 2, y + w / 2);
    ctx.closePath();
    ctx.fillStyle = s.civs[city.owner]?.color ?? '#888';
    ctx.fill();
    ctx.lineWidth = city.founding ? 3 : 1.5;
    ctx.strokeStyle = city.founding ? '#fff' : '#111';
    ctx.stroke();
    if (city.buildings?.wall) { ctx.strokeStyle = '#111'; ctx.lineWidth = 1; ctx.strokeRect(x - w * 0.65, y - w * 0.75, w * 1.3, w * 1.35); }
    ctx.font = `bold ${Math.round(Math.max(10, size * 0.5))}px system-ui, sans-serif`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.lineWidth = 3;
    ctx.strokeStyle = 'rgba(0,0,0,0.8)';
    ctx.strokeText(city.name, x, y + size * 1.05);
    ctx.fillStyle = '#fff';
    ctx.fillText(city.name, x, y + size * 1.05);
  }
  // the human's chosen sites
  const m = s.mods.civ;
  for (const [pick, col] of [[m.citySite?.[view.viewer], '#fff'], [m.siteChoice?.[view.viewer], '#ffe14d']]) {
    if (!pick) continue;
    const [q, r] = pick.k.split(',').map(Number);
    const { x, y } = view.at({ q, r });
    ctx.beginPath();
    ctx.setLineDash([3, 3]);
    ctx.arc(x, y, size * 0.6, 0, Math.PI * 2);
    ctx.strokeStyle = col;
    ctx.lineWidth = 2;
    ctx.stroke();
    ctx.setLineDash([]);
  }
}

export function onClick() { return false; }

// ---- panel ----------------------------------------------------------------------------------

function storeHtml(s, civ) {
  const cap = capOf(s, civ.id);
  const held = Object.values(civ.store).reduce((a, b) => a + b, 0);
  const over = s.mods.civ.overflow?.[civ.id] ?? 0;
  const custom = s.mods.civ.posting[civ.id] ?? {};
  const rows = GOODS.filter((g) => (civ.store[g] ?? 0) > 0.05 || custom[g] != null || FOODS.includes(g)).map((g) => {
    const keep = keepOf(s, civ.id, g);
    const set = (k) => ({ type: 'setPosting', civ: civ.id, good: g, keep: k });
    return `<tr><td>${g}</td><td>${fmt(civ.store[g] ?? 0)}</td><td>${custom[g] != null ? `<b>${fmt(keep)}</b>` : fmt(keep)}</td><td>` +
      btn(set(Math.max(0, Math.round(keep) - 5)), '−5', keep <= 0 ? 'already posting all of it' : null) +
      btn(set(Math.round(keep) + 5), '+5') +
      (custom[g] != null ? btn(set(null), 'auto') : '') + '</td></tr>';
  }).join('');
  const t = s.mods.civ.trade[civ.id];
  const basket = (b) => Object.entries(b ?? {}).filter(([, v]) => v > 0.05).map(([g, v]) => `${g} ${fmt(v)}`).join(', ') || 'nothing';
  return `<div class="sub">Store ${fmt(held)} / ${fmt(cap)} (cap: ${CIV.baseCap} a city + ${CIV.storehouseCap} a storehouse)` +
    `${over > 0.05 ? ` · <span class="alert">${fmt(over)} lost to the cap</span>` : ''}</div>` +
    `<div class="sub">Food spoils ${pct(CLAN.spoilage * storeSpoil(s, civ.id))} a turn` +
    ` (granary ×${CIV.granarySpoil}, pottery ×${CIV.potterySpoil})</div>` +
    `<table style="font-size:12px;width:100%"><tr class="sub"><td>good</td><td>have</td><td>keep</td><td>post above keep</td></tr>${rows}</table>` +
    `<div class="sub">Last market: posted ${basket(t?.posted)}; got ${basket(t?.received)}</div>`;
}

function labourHtml(s, civ) {
  const lab = s.mods.civ.labour[civ.id];
  if (!lab) return '';
  const clans = civClans(s, civ.id);
  const pop = clans.reduce((a, c) => a + c.pop, 0);
  const spared = clans.filter((c) => c.starving).length;
  return `<div class="sub">Labour: ${fmt(lab.drafted)} of ${fmt(pop)} people drafted (wanted ${fmt(lab.wanted)}; ` +
    `at most ${pct(CIV.draftMax)} of a clan${spared ? `; ${spared} starving clan${spared > 1 ? 's' : ''} spared` : ''}). ` +
    `Drafted people do not forage.</div>`;
}

const needText = (n) => Object.entries(n).filter(([, v]) => v > 0).map(([k, v]) => `${k} ${v}`).join(', ') || 'none';

function cityHtml(s, civ, city, full = false) {
  const built = Object.keys(city.buildings ?? {}).join(', ') || 'none';
  let proj;
  if (city.project) {
    const p = CIV.projects[city.project.kind];
    proj = `building ${city.project.kind} ${fmt(city.project.labour)}/${p.labour} labour`;
  } else {
    proj = Object.entries(CIV.projects).filter(([k]) => !city.buildings?.[k]).map(([k, p]) => {
      const cost = `${p.labour} labour, ${Object.entries(p.goods).map(([g, v]) => `${v} ${g}`).join(' ')}`;
      return btn({ type: 'setProject', civ: civ.id, city: city.id, project: k }, `${k} (${cost})`, projectError(s, civ.id, city, k));
    }).join(' ') || 'all built';
  }
  const workers = s.mods.civ.labour[civ.id]?.needs?.[city.id];
  return `<div><b>${city.name}</b>${city.founding ? ' (capital)' : ''} · built: ${built}<br>${proj}` +
    (full ? `<div class="sub">Workers wanted: ${needText(workers ?? cityNeeds(s, city))}</div>` : '') + '</div>';
}

function discHtml(s, civ) {
  const d = s.mods.civ.disc[civ.id];
  if (!d) return '';
  return `<div class="sub">Discoveries: ${Object.keys(CIV.practice).map((g) => d.known[g] ? `<b>${g}</b>`
    : `${g} ${fmt(Math.min(d.practice[g], CIV.practice[g]))}/${CIV.practice[g]}`).join(' · ')}` +
    ` (pottery: dig clay by a river; herding: hunt game near a city; trade partners who know it help)</div>`;
}

function yearsHtml(s, civ) {
  const mine = new Set([...civCities(s, civ.id), ...civClans(s, civ.id)].map(regionOf));
  const ys = Object.entries(s.mods.civ.years).filter(([r]) => mine.has(r));
  if (!ys.length) return '';
  return `<div class="sub">Years near you: ${ys.map(([r, y]) => `${y.factor < 1 ? '<span class="alert">drought</span>' : 'bumper year'} ` +
    `in region ${r} (×${y.factor}, ${y.left} turns)`).join('; ')}</div>`;
}

function prosperityHtml(s, civ) {
  const m = s.mods.civ;
  if (m.startTurn == null) return '';
  const left = Math.max(0, m.startTurn + CIV.turnLimit - s.turn);
  const visible = seenBy(s, civ.id);
  const row = (c) => { const p = prosperity(s, c.id); return `${c.name} ${pct(p.score)} (${fmt(p.pop)} people)`; };
  const rivals = Object.values(s.civs).filter((c) => c.id !== civ.id);
  const known = rivals.filter((c) => civCities(s, c.id).some(visible));
  return `<div class="sub">Prosperity in ${left} turns: people now over people at founding. You ${row(civ)}` +
    (known.length ? `; ${known.map(row).join('; ')}` : '') +
    (rivals.length > known.length ? `; ${rivals.length - known.length} rival${rivals.length - known.length > 1 ? 's' : ''} unseen` : '') + '</div>';
}

function leaningHtml(s, civ) {
  const visible = seenBy(s, civ.id);
  const list = Object.values(s.clans).filter((c) => c.owner == null && (c.leaning[civ.id] ?? 0) > 0.01 && visible(c))
    .sort((a, b) => b.leaning[civ.id] - a.leaning[civ.id]).slice(0, 8);
  return `<div class="sub">Clans leaning toward you (join at ${CIV.joinAt}): ${list.map((c) => {
    const rival = Object.entries(c.leaning).filter(([id]) => Number(id) !== civ.id).sort((a, b) => b[1] - a[1])[0];
    return `${c.name} ${fmt(c.leaning[civ.id])}${rival ? ` (${s.civs[rival[0]]?.name} ${fmt(rival[1])})` : ''}`;
  }).join(', ') || 'none'}</div>`;
}

// What the human can do with the selected tile.
function selectionHtml(s, civ, sel) {
  if (!sel || !s.map.tiles[key(sel)] || !seenBy(s, civ.id)(sel)) return '';
  const out = [];
  const city = cityAt(s, sel);
  if (city?.owner === civ.id) out.push(cityHtml(s, civ, city, true));
  const clan = clanAt(s, sel);
  if (clan && clan.owner == null) {
    const gifts = [5, 10].map((n) => btn({ type: 'giftFood', civ: civ.id, clan: clan.id, amount: n },
      `Gift ${n} food (+${giftGain(s, clan, civ.id, n).toFixed(2)})`, giftError(s, civ.id, clan.id, n))).join(' ');
    out.push(`<div><b>${clan.name}</b> (unaligned, ${fmt(clan.pop)} people${clan.starving ? ', starving' : ''}): ` +
      `leaning ${fmt(clan.leaning[civ.id] ?? 0)} / ${CIV.joinAt}<br>${gifts}` +
      `<div class="sub">At most ${CIV.giftMax} food a turn; a starving clan values it most.</div></div>`);
  }
  if (!city) {
    const site = `${sel.q},${sel.r}`;
    const ce = citySiteError(s, civ.id, sel), de = siteError(s, civ.id, sel);
    out.push(`<div>${btn({ type: 'setCitySite', civ: civ.id, site }, 'Found next city here', ce)}${why(ce)}<br>` +
      `${btn({ type: 'setDaughterSite', civ: civ.id, site }, 'Send next daughter here', de)}${why(de)}</div>`);
  }
  return out.join('');
}

function choicesHtml(s, civ) {
  const m = s.mods.civ;
  const out = [];
  const c = m.citySite[civ.id], d = m.siteChoice[civ.id];
  if (c) out.push(`Next city at ${c.k} (${Math.max(0, CIV.citySiteTurns - (s.turn - c.turn))} turns left) ` +
    btn({ type: 'setCitySite', civ: civ.id, site: null }, 'clear'));
  if (d) out.push(`Next daughter to ${d.k} (${Math.max(0, CIV.siteTurns - (s.turn - d.turn))} turns left) ` +
    btn({ type: 'setDaughterSite', civ: civ.id, site: null }, 'clear'));
  return out.length ? `<div class="sub">${out.join('<br>')}</div>` : '';
}

export function panel(s, selection) {
  const civ = humanCiv(s);
  if (!civ || s.phase !== 'main' || !s.mods.civ?.disc[civ.id]) return '';
  const clans = civClans(s, civ.id);
  const cities = civCities(s, civ.id);
  const sel = selectionHtml(s, civ, selection);
  return `<h3 style="margin:0">${civ.name}</h3>` +
    `<div>${cities.length} cit${cities.length === 1 ? 'y' : 'ies'} · ${clans.length} member clans · ` +
    `${fmt(clans.reduce((a, c) => a + c.pop, 0))} people</div>` +
    prosperityHtml(s, civ) + yearsHtml(s, civ) +
    (sel ? `<div class="sub">Selected tile</div>${sel}` : '') + choicesHtml(s, civ) +
    `<div class="sub">Cities</div>${cities.map((c) => cityHtml(s, civ, c)).join('')}` +
    labourHtml(s, civ) + storeHtml(s, civ) + discHtml(s, civ) + leaningHtml(s, civ) +
    `<div class="sub">Member clans near a granary spoil ${pct(CLAN.spoilage * CIV.granarySpoil)} instead of ${pct(CLAN.spoilage)}` +
    `${clans.some((c) => clanSpoil(s, c) < 1) ? '' : ' (none yet)'}.</div>`;
}
