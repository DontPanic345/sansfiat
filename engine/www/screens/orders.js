// The armed map clicks of the play screen: a unit's Move to… (`'move'`), a route's start (`'start'`) and its end
// (`'to'`), the right-click that moves the selected unit, and the tile tooltip's order lines. The screen hands
// each call its host: {view (MapView), v (Sim.view()), order (sends an order), panelDraw (redraws the panel)}.

import { f2, unitOrder } from '../panels.js';

/** The selected unit's armed click: `arm` is 'start', 'to' or null; `start` is a route's start tile (or null);
 *  `unit` is the unit they belong to. */
export const routing = { arm: null, start: null, unit: null };

/** Forgets the armed click and the route's start, and takes `unit` as the one they belong to. */
export function resetRouting(unit = null) {
  routing.arm = null;
  routing.start = null;
  routing.unit = unit;
}

/** A route or a clear went through: the armed click and the start are done. */
export function routeDone() {
  routing.arm = null;
  routing.start = null;
}

/** The armed click's state on the page: the crosshair cursor, and `body.dataset.armed`. */
export function syncArm(h) {
  document.body.dataset.armed = routing.arm ?? '';
  h.view.setArmed(routing.arm != null);
}

/** Arms (or disarms, if it is armed already) a map click: 'start' or 'to'. */
export function arm(kind, h) {
  routing.arm = routing.arm === kind ? null : kind;
  syncArm(h);
  h.panelDraw();
}

/** Disarms the armed click (Escape). */
export function disarm(h) {
  routing.arm = null;
  syncArm(h);
  h.panelDraw();
}

/** The map click after an armed button: a start, a move, or a route's end, which sends `route` from the start (or
 *  the unit's tile). A click off the map only disarms. */
export function armedPick(i, h) {
  const kind = routing.arm;
  routing.arm = null;
  if (i < 0) {
    h.panelDraw();
    syncArm(h);
    return;
  }
  if (kind === 'start') {
    routing.start = i;
    h.panelDraw();
    syncArm(h);
    return;
  }
  const u = h.v.units[routing.unit];
  if (!u) return;
  if (u.kind === 'camp') h.order(unitOrder(u, { type: 'move', to: i }));
  else h.order(unitOrder(u, { type: 'route', from: routing.start ?? u.tile, to: i }));
}

/** A right-click: the selected camp of the player's goes to the tile. */
export function onCommand(i, h) {
  const u = routing.unit != null ? h.v.units[routing.unit] : null;
  if (i < 0 || !u || u.civ !== h.v.player || u.kind !== 'camp') return;
  h.order(unitOrder(u, { type: 'move', to: i }));
}

/** The order lines of the tile tooltip for tile `i` and the selected unit `u` (or null): "Move here" for the
 *  player's camp, or for a unit while a route's end is armed. */
export function orderTip(sim, u, i, v) {
  if (!u || u.civ !== v.player || !(u.kind === 'camp' || routing.arm === 'to')) return [];
  const r = JSON.parse(sim.path(u.tile, i));
  return [r ? `Move here: cost ${f2(r.cost)}, ${r.turns} turn${r.turns === 1 ? '' : 's'}` : 'No route'];
}
