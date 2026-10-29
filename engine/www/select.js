// What the selection panel shows: one function from a selection to {title, html, actions}. Later
// pieces add actions by adding entries to the lists below; main.js greys a button when
// `Sim.check(action)` refuses it and runs it when clicked.
//
// A selection is {kind: 'unit', id}, {kind: 'capital', civ} or {kind: 'tile', tile, clan?, market?}.
// `ctx` is what the page knows: {v (Sim.view), layout, terrain, goods, rules, sim, view (MapView)}.

export const f2 = (x) => (Math.round(x * 100) / 100).toString();
const ZONES = ['', 'valley', 'sealed valley', 'neck'];
/** A clan's colour from its id (as the map draws it). */
const clanHue = (id) => `hsl(${(id * 137.508) % 360} 70% 55%)`;

const swatch = (color) => `<i class="sw" style="background:${color}"></i>`;
const civName = (ctx, id) => {
  const c = ctx.v.civs[id];
  return c ? `${swatch(c.color)}${c.name}` : `civ ${id}`;
};
const hex = (ctx, i) => `${ctx.layout.q[i]}, ${ctx.layout.r[i]}`;

/** The key `body.dataset.selected` and the cycle use: `unit:40`, `capital:12` (the civ), `tile:812`. */
export function keyOf(sel) {
  if (!sel) return '';
  return sel.kind === 'unit' ? `unit:${sel.id}` : sel.kind === 'capital' ? `capital:${sel.civ}` : `tile:${sel.tile}`;
}

/** The tile a selection stands on, or -1. */
export function tileOf(ctx, sel) {
  if (!sel) return -1;
  if (sel.kind === 'tile') return sel.tile;
  if (sel.kind === 'capital') return ctx.v.civs[sel.civ]?.capital ?? -1;
  return ctx.v.units[sel.id]?.tile ?? -1;
}

/** Whether the selection still exists (a disbanded unit does not). */
export function alive(ctx, sel) {
  if (!sel) return false;
  if (sel.kind === 'unit') return sel.id in ctx.v.units;
  if (sel.kind === 'capital') return sel.civ in ctx.v.civs;
  return true;
}

/** Everything a left-click on tile `i` can pick, in cycle order: the player's units (by id), the
 *  capital, other civs' units, then the bare tile. */
export function candidates(ctx, i) {
  const { v } = ctx;
  const here = Object.values(v.units).filter((u) => u.tile === i).sort((a, b) => a.id - b.id);
  const mine = here.filter((u) => u.civ === v.player);
  const out = mine.map((u) => ({ kind: 'unit', id: u.id }));
  for (const c of Object.values(v.civs)) if (c.capital === i) out.push({ kind: 'capital', civ: c.id });
  for (const u of here) if (u.civ !== v.player) out.push({ kind: 'unit', id: u.id });
  out.push({ kind: 'tile', tile: i, clan: null, market: null });
  return out;
}

export function terrainLabel(ctx, i) {
  const l = ctx.layout;
  const t = ctx.terrain[l.terrain[i]];
  if (l.terrain[i] === 0) return l.lake[i] ? 'lake' : 'ocean';
  if (t.name === 'marsh') return 'marsh (delta)';
  return t.name;
}

/** The terrain name and what's on it: river, floodplain, pass, zone. */
export function tileKind(ctx, i) {
  const l = ctx.layout;
  const bits = [terrainLabel(ctx, i)];
  if (l.river[i] > 0) bits.push(`river (${l.river[i]} upstream)`);
  if (l.flood[i]) bits.push('floodplain');
  if (l.pass[i]) bits.push('pass');
  if (l.zone[i]) bits.push(ZONES[l.zone[i]]);
  return bits.join(', ');
}

export const marketOn = (ctx, i) => Object.values(ctx.v.markets).find((m) => m.tile === i);

function clanHtml(c, how) {
  const cost = c.routes.reduce((a, r) => a + r.cost, 0);
  const festival = c.market != null ? `festival ${c.market}, route ${f2(cost)} labour a year` : 'no festival in reach';
  return `<p><b style="color:${clanHue(c.id)}">Clan ${c.id}</b> <span class="dim">(${how})</span><br>${f2(c.people)} people · territory ${c.territory.length} tiles<br>${festival}<br>fed ${f2(c.fed_ratio)}${c.starving ? ' · <b style="color:#ff7b72">starving</b>' : ''}</p>`;
}

function marketHtml(ctx, m, how) {
  const { v, goods } = ctx;
  const prices = goods.map((g, f) => `${g} ${f2(m.prices[f])}`).join(', ');
  return `<p><b>Market ${m.id}</b> <span class="dim">(${how})</span>${v.end && v.end.centres.includes(m.id) ? ' · <b style="color:#ffc800">centre</b>' : ''}<br>age ${v.turn - m.founded} · ${m.traders} traders · activity ${f2(m.activity)}<br><span class="dim">prices ${prices}</span></p>`;
}

const unitName = (u) => `${u.kind[0].toUpperCase()}${u.kind.slice(1)} ${u.id}`;

/** A row of buttons that select units: `Warrior 41 (at 3, 4)`. */
function unitList(ctx, units, label, withTile) {
  if (!units.length) return '';
  const rows = units
    .map(
      (u) =>
        `<button type="button" class="link" data-select="unit:${u.id}">${unitName(u)}${withTile ? ` <span class="dim">(${hex(ctx, u.tile)})</span>` : ''}</button>`,
    )
    .join(' ');
  return `<p class="units"><span class="dim">${label}</span><br>${rows}</p>`;
}

function unitPanel(ctx, id) {
  const { v, sim } = ctx;
  const u = v.units[id];
  if (!u) return null;
  const mine = u.civ === v.player;
  let html = `<p class="dim">${civName(ctx, u.civ)} · at ${hex(ctx, u.tile)}</p>`;
  if (u.hungry > 0) html += `<p><b style="color:#ff7b72">Hungry for ${u.hungry} turn${u.hungry === 1 ? '' : 's'}</b> <span class="dim">(disbands at ${ctx.rules.hungry_turns})</span></p>`;
  const actions = [];
  if (u.dest != null) {
    const r = JSON.parse(sim.route(id));
    html += r
      ? `<p>Going to ${hex(ctx, u.dest)}: cost ${f2(r.cost)}, ${r.turns} turn${r.turns === 1 ? '' : 's'}</p>`
      : `<p>Going to ${hex(ctx, u.dest)}: <b>no route</b></p>`;
  } else if (mine) {
    html += '<p class="dim">Right-click a tile to move it there.</p>';
  } else {
    html += '<p class="dim">Standing still.</p>';
  }
  const others = Object.values(v.units).filter((o) => o.tile === u.tile && o.id !== id).sort((a, b) => a.id - b.id);
  html += unitList(ctx, others, 'Units here', false);
  if (mine) {
    if (u.dest != null) actions.push({ label: 'Stop', action: { type: 'moveCamp', to: u.tile } });
    actions.push({ label: 'Disband', action: { type: 'disband', n: null } });
  }
  return { title: `${unitName(u)} (${v.civs[u.civ]?.name ?? '?'})`, html, actions };
}

function capitalPanel(ctx, id) {
  const { v, goods } = ctx;
  const c = v.civs[id];
  if (!c) return null;
  const mine = id === v.player;
  let html = `<p class="dim">${civName(ctx, id)} · capital at ${hex(ctx, c.capital)} · founded turn ${c.founded}</p>`;
  html += `<p>${f2(c.people)} people in ${c.clans} clan${c.clans === 1 ? '' : 's'}<br>land: ${c.territory.length} tiles · work tiles: ${c.work.length}</p>`;
  html += '<table>';
  goods.forEach((g, f) => {
    if (c.store[f] > 0.005) html += `<tr><td>${g}</td><td>${f2(c.store[f])}</td></tr>`;
  });
  html += `<tr><td class="dim">store</td><td class="dim">${f2(c.store.reduce((a, b) => a + b, 0))} / ${ctx.rules.store_cap}${c.lost > 0.005 ? `, lost ${f2(c.lost)} last turn` : ''}</td></tr>`;
  html += '</table>';
  const units = Object.values(v.units).filter((u) => u.civ === id).sort((a, b) => a.id - b.id);
  html += units.length ? unitList(ctx, units, `Units (${units.length})`, true) : '<p class="dim">No units.</p>';
  const actions = mine ? [{ label: 'Raise warrior', action: { type: 'raise', n: 1 } }] : [];
  return { title: `${c.name}'s capital`, html, actions };
}

function tilePanel(ctx, sel) {
  const { v, layout, terrain, goods, rules, sim, view } = ctx;
  const i = sel.tile;
  const t = JSON.parse(sim.tile(i));
  const land = terrain[t.terrain].land;
  let html = `<p class="dim">${tileKind(ctx, i)}${land ? ` · fertility ${f2(t.fertility)} · trail ${t.trail ? 'yes' : 'no'}` : ''}</p>`;
  const civ = sim.tile_civ(i);
  if (civ >= 0) html += `<p>Land of ${civName(ctx, civ)}</p>`;
  if (land) {
    html += '<table>';
    goods.forEach((g, f) => {
      if (t.cap[f] > 0) html += `<tr><td>${g}</td><td>${f2(t.stock[f])} / ${f2(t.cap[f])}</td></tr>`;
    });
    if (t.wild_grain > 0) html += `<tr><td>tame</td><td>${Math.round((100 * t.tame) / rules.tame_turns)}%</td></tr>`;
    html += '</table>';
  }
  const shown = new Set();
  const addClan = (id, how) => {
    if (id == null || shown.has(id) || !v.clans[id]) return;
    shown.add(id);
    html += clanHtml(v.clans[id], how);
  };
  addClan(sel.clan, 'chosen');
  addClan(view.clanAt(i), 'in its territory');
  const m = sel.market != null ? v.markets[sel.market] : marketOn(ctx, i);
  if (m) html += marketHtml(ctx, m, 'here');
  else if (sel.market != null) html += `<p class="dim">Market ${sel.market} is gone.</p>`;
  const here = Object.values(v.units).filter((u) => u.tile === i).sort((a, b) => a.id - b.id);
  html += unitList(ctx, here, 'Units here', false);
  void layout;
  return { title: `Tile ${hex(ctx, i)}`, html, actions: [] };
}

/** The panel for a selection: `{title, html, actions: [{label, action}]}`, or null if it is gone. */
export function describe(ctx, sel) {
  if (!sel) return null;
  if (sel.kind === 'unit') return unitPanel(ctx, sel.id);
  if (sel.kind === 'capital') return capitalPanel(ctx, sel.civ);
  return tilePanel(ctx, sel);
}
