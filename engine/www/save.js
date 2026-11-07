// Autosave: the page's one browser entry, `localStorage['sansfiat.autosave']`, written after each turn
// the player ends and each action, and at most once a second while a run goes (and once when it stops).
// A write that throws (storage refused or full) turns autosave off for the session; the Game menu says so.
// A staged game (`Sim.stage`) is not a save: it is never written.
// The entry holds {version, saved, seed, size, turn, civ} (civ is the led civ's name, or null) and the save
// (`Sim.save()`, which carries the Standing history by replay). Continue loads it through `Sim.load`.

import { app } from './app.js';

export const KEY = 'sansfiat.autosave';

/** The size name of a save's radius. */
const sizeOf = (radius) => app.data.SIZES.find((s) => s.radius === radius)?.name ?? `radius ${radius}`;

export class Autosave {
  constructor() {
    this.on = true;
    this.last = -Infinity; // performance.now() of the last write
    this.onChange = () => {}; // told when autosave goes off, or a write lands
  }

  /** The stored entry, or null (none, or unreadable). */
  read() {
    try {
      const text = globalThis.localStorage?.getItem(KEY);
      return text ? JSON.parse(text) : null;
    } catch {
      return null;
    }
  }

  /** Writes the game as it stands now. A refusal turns autosave off for the session. */
  write(sim, model) {
    if (!this.on || !sim || !model || sim.staged()) return;
    try {
      const save = JSON.parse(sim.save());
      const v = model.v;
      const civ = v.player != null ? v.civs[v.player]?.name ?? null : null;
      const entry = {
        version: app.data.RULES.save_version,
        saved: new Date().toISOString(),
        seed: save.config.seed,
        size: sizeOf(save.config.radius),
        turn: v.turn,
        civ,
        save,
      };
      globalThis.localStorage.setItem(KEY, JSON.stringify(entry));
      this.last = performance.now();
    } catch {
      this.on = false;
    }
    this.onChange();
  }

  /** While a run goes: a write if a second has passed since the last. */
  tick(sim, model) {
    if (performance.now() - this.last >= 1000) this.write(sim, model);
  }

  /** The Game menu's line: the last write, or why there is none. */
  line() {
    if (!this.on) return 'Autosave off: this browser refuses storage';
    const e = this.read();
    return e ? `Autosaved turn ${e.turn}` : 'Not autosaved yet';
  }
}

/** The line Continue shows for an entry: "Red, turn 312, small seed 2", or "Watching, …". */
export const continueLine = (e) => `${e.civ ?? 'Watching'}, turn ${e.turn}, ${e.size} seed ${e.seed}`;

export const autosave = new Autosave();
