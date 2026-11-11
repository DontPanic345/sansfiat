# #162 Fixes for M4

**Question:** does each `fix` finding now work as the design line it names says, with its repro a test that fails on main and passes here?
Nothing in this piece is judged by the scores: it restores lines, it adds no rule. No play step.

**Shape: Henry Ford**, kept. Every finding is settled by the line it names; two builds cut by area (the engine, then the page and the docs).

## Findings and what settles each

The findings list five items; two are the same bug (the Keep buttons, #151 build-7 and #151 review). That is one fix.

1. **Scout trails on the possible path** (`core/src/units/search.rs`, the `trail_on` match near line 278). `scouting.md`: trails make a route cheaper, and the scout finds and builds the cheapest.
   Today, once `plan.candidates` is empty, `trail_on` is the known path. When the possible path is cheaper than the known one by more than `EPS` and no candidate is left (the differing step is on a revealed tile, e.g. a river bridge), the scout settles at the known cost.
   Fix: with no candidates, `trail_on` is the possible path when `possible < known - EPS` (or known is `None`), else the known path. Building those trails brings the known cost down to the possible one; the existing "settled" tests (`known <= possible + EPS`, `heard`, `built`) then fire as they do today. `keep_route` reads the known path, so it keeps the trails alive once settled and needs no change.
   The repro is `core/tests/cases.rs` `a_settled_route_has_no_cheaper_trail`. It is `#[should_panic]` and `#[ignore]` today because it was red on main. The build drops both attributes, so it asserts green, and reads its expected numbers from the run: small 2 to the rival capital, small 3 to Lead's target. Where the ignore note says the picks moved, the build restages to the current map and keeps the test's check (`possible < known - EPS` at settle is a failure).
   The test must fail on main with the attributes dropped (check by stashing the fix), and pass here.
2. **Keep ±1** (`www/panels.js` `civShape`, lines 459-460; `web/src/lib.rs` civ view). `c.keep` is `civs::keep`: the set amount plus the guard's food. A press must move the set amount by one.
   Fix: the view gains `keep_guard`, the guard's food in `civs::keep` (`GUARD.keep_turns * UNIT.food * claim::guard_people`; pull it into a small `civs::guard_keep` fn that `keep` also uses, so the two can't drift). The buttons set `base ± 1` where `base = c.keep - c.keep_guard` (equal to `keep_set` when set, else the default). Keep −1 clamps at 0. Drop both `expectBug` wrappers in `www/play/buttons.mjs` (and the `expectBug` helper if nothing else uses it, and the comment at line 5, which lists it as red). Update the docs of the view fields in `web/src/lib.rs` (line ~104) and wherever `engine/CLAUDE.md` or `docs/` list them.
   Repro: `cd engine && node www/play/buttons.mjs`: the Keep cases go from "ok (red on main)" to plain ok, with 26 -> 27 and 27 -> 26 style steps. On main the dropped wrappers fail, which is the red.
3. **"leaves for X, 0 steps away"** (`core/src/units/scout.rs`, `choose_ground`, the `leaves` closure). When `n == 0` the scout already stands on the tile: the note reads `forages at {at}`. The `leaves its route to forage` variant (route, no dest) with n == 0 also says `forages at {at}`. The "waits on rich ground" texts are unchanged. Repro: `relay/specs/153/play/small-2.json` plus `{"type":"move","unit":514,"to":300}`, `end 50`, `why 514`: turns 389 and 402 read `leaves for (-7, -8), 0 steps away` on main. The test is a core test that stages a scout standing on rich ground with food short and reads the note, or a `cli` test of that game file if one exists for such files (the build picks the nearer pattern: `core/tests/notes.rs`).
4. **`engine/CLAUDE.md` view.rs line**: the table says "the text view" with "under 80 lines" in some form; `cli/src/view.rs` is about 416 lines. Drop the size claim (say only what the file is). Repro: `wc -l engine/cli/src/view.rs` against the table. A test for a doc: none needed beyond the grep in the brief; the build records the before and after wording in its summary. Check the table in `engine/CLAUDE.md` still reads "under 80" nowhere (`grep -n "under 80" engine/CLAUDE.md`; the current table row at line 50 may already have dropped it, in which case say so and move on).

## Measuring

No scores move by design, but item 1 changes scout trails, so a scout's route can settle at a lower cost on the retro maps. The build runs `sansfiat sweep --judge` on main and here and puts both in `relay/specs/162/sweep.md` (the control is main). Expected: scores equal, or a trail count that moves a little. If a score moves, say it in the PR with old and new numbers (`CLAUDE.md`). The pins in `cli/tests/save.rs` and elsewhere may move with the trails; update them and say which.

## Decisions I made

- Design says "the scout finds and builds the cheapest" route: so when possible is cheaper than known with no candidates, trails go on the possible path. No new number or rule.
- The two Keep findings are one fix, one build step.
- The page learns the guard's food through a new view field `keep_guard`, not a new lever; the page can't otherwise tell the default from the guard's share.
- At 0 steps the note says "forages at X" (text only; no event kind or rule changes).
- `engine/CLAUDE.md`'s view.rs claim is cut, not the file: the size is not the doc's business.

## Unsettled

Nothing. Every build has an exact done: the named tests fail on main and pass here, `engine/check.sh` passes, and `node www/play/buttons.mjs` passes with no `expectBug`.

## For the review

Run `engine/check.sh`, `cd engine && node www/play/buttons.mjs`, and read `relay/specs/162/sweep.md`. Press Keep +1 and Keep −1 on the staged game and confirm the shown keep moves by exactly one each way.
