# #162 build-2 (ford): the Keep buttons and the doc

Read `spec.md` items 2 and 4.

## Code
`core/src/civs.rs` (`keep`, ~line 465), `web/src/lib.rs` (the civ view, ~lines 104 and 178), `www/panels.js` (`civShape`, ~lines 436-461), `www/play/buttons.mjs` (`expectBug`, `keepStep`, the Keep cases, the comment at line 5), `engine/CLAUDE.md`.

## Build
1. In `civs.rs` add `pub fn guard_keep(w, civ) -> f64` = `GUARD.keep_turns * UNIT.food * claim::guard_people(w, civ)`; `keep` uses it. In the web civ view add `"keep_guard": civs::guard_keep(w, c.id)` and document it with the other fields. Add it to any view type or schema check that lists the fields (`grep -rn keep_set engine`).
2. In `panels.js`, `const base = c.keep - c.keep_guard;` Keep −1 sets `Math.max(0, base - 1)`, Keep +1 sets `base + 1`.
3. In `buttons.mjs` the Keep −1 and Keep +1 cases call `keepStep` directly; drop `expectBug` and its comment, and the line 5 note, if nothing else uses them. `keepStep`'s comment drops "Red on main".
4. Check it red on main: with the buttons.mjs change and no `panels.js` change the Keep cases fail (26 -> 30). Then green with the fix. Record this in your summary.
5. `engine/CLAUDE.md`: the layout table's `view.rs` entry must not claim a line count (`grep -n "under 80" engine/CLAUDE.md`; also `grep -rn "under 80" docs engine`). If the claim is already gone, say so.

## Done
`engine/check.sh` passes; `cd engine && node www/play.mjs`-style run of `node www/play/buttons.mjs` passes with no `expectBug` left for the Keep cases; one note in the summary on the doc line.
