// The supply module. Owns: Unit food stores (u.supply), supply lines, resupply, forage and pillage (tile.fallow), porters, supply caches (tile.cache), auto-supply and auto-forage toggles.
// Contract (see CLAUDE.md, "Module registry"): import helpers from ../../core.js and
// ../../clans.js only, never from ../../rules.js. Keep module-wide state in s.mods.supply
// and per-unit state under u.supply.
// Suggested phases: armies (order 'after': units eat and resupply), orders (porters walk).
// import { note, event, tileAt, pathCost, unitsAt, civOf } from '../../core.js';
// import { SUPPLY } from './data.js';

export const name = 'supply';

// Called once from newGame, after the map and seed clans exist. Set up s.mods.supply here.
export function init(s) {
  s.mods.supply = {};
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
