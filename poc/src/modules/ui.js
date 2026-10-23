// UI half of the module registry, imported by render.js and main.js only (it may touch the
// canvas through the hooks; game rules never import this file). Sorted by each module's z.
import * as civ from './civ/ui.js';
import * as scout from './scout/ui.js';
import * as army from './army/ui.js';
import * as supply from './supply/ui.js';

export const MODULE_UIS = Object.entries({ civ, scout, army, supply })
  .map(([name, ui]) => ({ name, ...ui }))
  .sort((a, b) => (a.z ?? 0) - (b.z ?? 0));
