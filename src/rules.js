// Core rules: the phase pipeline, the action table, and dispatch. All game logic is reached
// through dispatch(state, action) -> {state, error?}, which is pure: it clones, applies, and
// returns. Clan behaviour lives in clans.js; everything else lives in src/modules/*.
import {
  forage, foundCivs, formMarkets, hearths, nutrition, occupy, pregameOver, reaches, regrow, settle, trade,
} from './clans.js';
import { civOf, humanCiv, passable, sweepNotes, tileAt, total, totalPop } from './core.js';
import { MODULES } from './modules/index.js';

// Re-exported so older imports (tests, tools, UI) keep one entry point.
export { hearths, nutrition, passable, tileAt, total, totalPop, civOf, humanCiv };

// The turn order of DESIGN.md ("Turn order (main game)"). Each phase runs, in order:
// module steps with order 'before', the core step(s), then module steps with order 'after'.
export const PHASES = ['regrow', 'forage', 'trade', 'settle', 'occupy', 'markets', 'orders', 'armies', 'discoveries'];

// Core steps. when: 'always' runs in the pre-game and the main game; 'main' only in the main game.
const CORE_STEPS = [
  { phase: 'regrow', name: 'regrow', when: 'always', run: (s, ctx) => regrow(s, ctx) },
  { phase: 'forage', name: 'forage', when: 'always', run: (s) => forage(s, reaches(s)) },
  { phase: 'trade', name: 'trade', when: 'always', run: trade },
  { phase: 'settle', name: 'settle', when: 'always', run: (s, ctx) => settle(s, reaches(s), ctx.hooks) },
  { phase: 'occupy', name: 'occupy', when: 'always', run: occupy },
  { phase: 'markets', name: 'markets', when: 'always', run: formMarkets },
  { phase: 'orders', name: 'aiOrders', when: 'main', run: (s) => runAi(s) },
];

// ---- registry wiring -------------------------------------------------------------------------

function checkStep(mod, st) {
  if (!PHASES.includes(st.phase)) throw new Error(`${mod.name}: turn step ${st.name} has unknown phase ${st.phase}`);
  if (st.order && !['before', 'after'].includes(st.order)) throw new Error(`${mod.name}: bad order ${st.order}`);
  if (st.when && !['main', 'always'].includes(st.when)) throw new Error(`${mod.name}: bad when ${st.when}`);
}

// Flat list of {phase, name, module, when, run} in the order they run.
export const PIPELINE = PHASES.flatMap((phase) => {
  const of = (order) => MODULES.flatMap((m) => (m.turnSteps ?? [])
    .filter((st) => { checkStep(m, st); return st.phase === phase && (st.order ?? 'after') === order; })
    .map((st) => ({ when: 'main', ...st, module: m.name })));
  return [...of('before'), ...CORE_STEPS.filter((st) => st.phase === phase).map((st) => ({ ...st, module: 'core' })), ...of('after')];
});

// Hooks into the clan rules. daughterSite: the first module that returns something other than
// undefined decides. yieldFactor: the product of every module's factor.
const hookers = (name) => MODULES.filter((m) => m.hooks?.[name]);
const HOOKS = {};
if (hookers('daughterSite').length)
  HOOKS.daughterSite = (s, clan, opts) => {
    for (const m of hookers('daughterSite')) {
      const k = m.hooks.daughterSite(s, clan, opts);
      if (k !== undefined) return k;
    }
    return undefined;
  };
if (hookers('yieldFactor').length)
  HOOKS.yieldFactor = (s, t) => hookers('yieldFactor').reduce((k, m) => k * m.hooks.yieldFactor(s, t), 1);

// Called by newGame once the map and seed clans exist.
export function initModules(s) {
  for (const m of MODULES) m.init?.(s);
}

// Each AI civ asks every module for its actions, in registry order, and core applies the valid
// ones in place. AI is deterministic in state, so replays reproduce it without recording it.
function runAi(s) {
  for (const civ of Object.values(s.civs)) {
    if (civ.isHuman) continue;
    for (const m of MODULES)
      for (const a of m.ai?.(s, civ.id) ?? []) {
        const error = validate(s, a);
        if (!error) ACTIONS[a.type].apply(s, a);
      }
  }
}

// ---- actions ---------------------------------------------------------------------------------

// Runs one whole turn in place.
export function runTurn(s) {
  const ctx = { hooks: HOOKS };
  for (const st of PIPELINE) if (st.when === 'always' || s.phase === 'main') st.run(s, ctx);
  endOfTurn(s);
}

function endOfTurn(s) {
  s.stats.pop.push(totalPop(s));
  s.stats.clans.push(Object.keys(s.clans).length);
  s.stats.volume.push(Object.values(s.markets).reduce((a, m) => a + m.lastVolume, 0));
  for (const u of Object.values(s.units)) u.labour = u.labourMax;
  sweepNotes(s);
  if (s.phase === 'pregame' && pregameOver(s)) foundCivs(s);
  s.turn++;
}

const CORE_ACTIONS = {
  endTurn: {
    validate: (s) => (s.phase === 'choose' ? 'choose a civ to lead first' : null),
    apply: (s) => runTurn(s),
    candidates: () => [{ type: 'endTurn' }],
  },
  // Pick the civ the human leads (civ: id), or watch with every civ run by the AI (civ: null).
  // Starts the main game: every module's onStart runs, in registry order.
  chooseCiv: {
    validate: (s, a) => (s.phase !== 'choose' ? 'not choosing a civ now'
      : a.civ != null && !s.civs[a.civ] ? `no civ ${a.civ}` : null),
    apply: (s, a) => {
      if (a.civ != null) s.civs[a.civ].isHuman = true;
      s.phase = 'main';
      for (const m of MODULES) m.onStart?.(s);
    },
    candidates: (s) => (s.phase === 'choose' ? [...Object.keys(s.civs).map(Number), null].map((civ) => ({ type: 'chooseCiv', civ })) : []),
  },
};

// Module actions are wrapped so core checks the phase and the acting civ before the module does.
function wrap(mod, type, def) {
  return {
    validate: (s, a) => (s.phase !== 'main' ? `${type} needs the main game`
      : !s.civs[a.civ] ? `${type}: no civ ${a.civ}` : def.validate(s, a)),
    apply: def.apply,
    candidates: (s) => (s.phase !== 'main' ? []
      : Object.keys(s.civs).flatMap((civ) => def.candidates?.(s, Number(civ)) ?? [])),
    module: mod.name,
  };
}

export const ACTIONS = { ...CORE_ACTIONS };
for (const m of MODULES)
  for (const [type, def] of Object.entries(m.actions ?? {})) {
    if (ACTIONS[type]) throw new Error(`action ${type} from ${m.name} clashes with ${ACTIONS[type].module ?? 'core'}`);
    ACTIONS[type] = wrap(m, type, def);
  }

export function validate(s, a) {
  const def = ACTIONS[a.type];
  return def ? def.validate(s, a) : `unknown action ${a.type}`;
}

export function dispatch(s, a) {
  const error = validate(s, a);
  if (error) return { state: s, error };
  const next = structuredClone(s);
  next.events = [];
  ACTIONS[a.type].apply(next, a);
  return { state: next };
}

export const legalActions = (s) =>
  Object.values(ACTIONS).flatMap((def) => def.candidates(s)).filter((a) => !validate(s, a));
