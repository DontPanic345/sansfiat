// The module registry. Core (src/rules.js) imports this file once and wires every module's
// actions, turn steps, hooks, AI and lifecycle calls. Order here is the order modules run in
// within a phase, and the order of onStart. Adding a module: one import and one entry.
import * as civ from './civ/rules.js';
import * as scout from './scout/rules.js';
import * as army from './army/rules.js';
import * as supply from './supply/rules.js';

export const MODULES = [civ, scout, army, supply];
