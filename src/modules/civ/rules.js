// The civ module. Owns: Founding (onStart turns founding clans into civ clans and the market site into the first city), civ clans and cities, expansion by division (hooks.daughterSite), materials (wood, stone, clay) and pottery, labour+goods projects (granary, storehouse, wall), civ trading at markets (m.posts / m.received), good and bad years (hooks.yieldFactor), clan leanings, joining and food gifts, discoveries (herding, pottery), victory.
// Contract (see CLAUDE.md, "Module registry"): import helpers from ../../core.js and
// ../../clans.js only, never from ../../rules.js. Keep module-wide state in s.mods.civ
// and per-unit state under u.civ.
// Suggested phases: trade (order 'before' to post the civ store, 'after' to take m.received), settle (cities, labour),
// markets (joining), orders (projects, expansion), discoveries (discoveries, victory).
// import { note, event, tileAt, pathCost, unitsAt, civOf } from '../../core.js';
// import { CIV } from './data.js';

export const name = 'civ';

// Called once from newGame, after the map and seed clans exist. Set up s.mods.civ here.
export function init(s) {
  s.mods.civ = {};
}

// Called once when the main game begins (after chooseCiv), in registry order civ, scout,
// army, supply. s.civs exists and the human civ (if any) has isHuman = true.
export function onStart(s) {}

// type -> { validate(s, a) -> error string | null, apply(s, a), candidates(s, civId) -> [actions] }.
// Every action carries `civ` (the acting civ's id); core rejects it outside the main game or
// for an unknown civ before your validate runs. Action type names must be unique across modules.
export const actions = {};

// { phase, name, run(s, ctx), order?: 'before' | 'after' (default 'after' the core step of that
// phase), when?: 'main' | 'always' (default 'main') }. Phases: see PHASES in src/rules.js.
export const turnSteps = [];

// Optional seams into the clan rules (see CLAUDE.md): daughterSite, yieldFactor.
export const hooks = {};

// Actions this module's AI wants for civ `civId` this turn; core applies them in the orders phase.
export function ai(s, civId) {
  return [];
}
