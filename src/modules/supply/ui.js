// UI hooks for the supply module. render.js and main.js call these generically. Draw through the ctx
// you are handed and return HTML strings; never touch document directly.

// Draw order among module layers: lower first. Fog of war (scout) should be last.
export const z = 30;

// Draws on the map canvas after the base map. view: { size, at(hex) -> {x, y}, hexPath(ctx, center, size),
// selected, mode, viewer (human civ id or null), local (this module's UI-only state) }.
export function drawLayer(ctx, s, view) {}

// HTML for the side panel. selection is the selected hex {q, r} or null. Buttons with
// data-action='{"type":"...","civ":N}' (JSON) dispatch that action when clicked.
export function panel(s, selection, ui) {
  return '';
}

// A click on hex. ui: { act(action) -> bool, local, selection }. Return true to consume the click
// (the default tile selection still happens unless consumed).
export function onClick(s, hex, ui) {
  return false;
}
