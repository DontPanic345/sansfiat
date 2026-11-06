# #151 build-7 (ford): `www/play/buttons.mjs` and the docs

One script presses every button of every panel once on a staged game and checks the world
changed as the button says. Read `spec.md` item 21 (the table of cases: follow it),
build-4's summary (`window.game.stage(spec)`, `Sim.staged`), `www/panels.js` (every button
is data: `{label, action}`, `{label, arm}`, `{label, paint}` or `{label, stepper: {options:
[{label, cap, make(n)}]}}`), `www/panel.js` (how a button and a stepper are rendered and
clicked), `www/play.mjs` (`open`, `page.button`, `page.click`, `page.eval`, `page.until`,
`stepper()`), and `www/play/camp.mjs` and `www/play/scout.mjs` (scripts that already press
camp and scout buttons: reuse their helpers' shape, `selectKey`, `need`, `fail`).

## Do

1. **`www/play/buttons.mjs`**, in the style of the other scripts (`--out DIR`, exits non-zero
   on any failure, `need(ok, msg)` printing `ok` lines). `all.mjs` runs every `.mjs` in the
   folder, so `check.sh` runs it with no edit to either.
2. **The staged game**: a `StageSpec` in the script: a small hand-written map text (plains
   and sea, a forest and a hill, a short walk across), food on a few tiles, two civs, the
   first led, its camp placed beside food, a rival tile with food next to it (for Pillage),
   its scout and guard from founding. Load it with `page.eval('game.stage(...)')` and wait
   for `document.body.dataset.phase === 'main'`. Every case starts from a **fresh** stage
   (the call is cheap), so a case's effect never hides another's.
3. **The inventory**: for the led civ's panels, enumerate the buttons through the page's own
   `describe` (import `www/panels.js` in the page via `page.eval`, or read the panel with
   `page.panel()` per selection) for these selections: the led civ, a rival civ, a clan, the
   camp, the scout, the guard, a market if there is one, a tile. Collect every button label
   (a stepper's options included, each as `Label/option`). Each label must have a case in the
   script's table; a label without one fails the script with "button X has no case". A rival
   civ's panel and the clan and tile panels have no buttons for the led player: assert that
   (it is part of the check: a button appearing there is a failure to look at).
4. **The cases**, as in `spec.md` item 21. Each case has `setup` (a selection, and a staged
   change if one is needed, such as a marching army for Stop), `press` (click the button; for
   an `arm` button click the tile after; for a `paint` button drag or click the tiles; for a
   stepper choose the option, set the count to 1 and press Go) and `check` (read `game.view()`
   and, where a turn must pass, press End turn and read it again). The exact view fields are
   the build's to find by reading `Sim.view()` in `web/src/lib.rs` and the model; write each
   check as an assertion on the world, never on the button's own state.
5. **Disband all** is the one red case: press `Disband…`, choose `all`, and press Go at the
   count the stepper opens at (do not set the count: the bug is that it opens at 1 and sends
   it), on the camp of 4.6 people; assert the army is gone and the clan is up by 4.6. It fails
   today (one person goes home). Wrap that case in `expectBug` (copy build-4's helper into the
   script, same doc comment) so the script stays green and T drops the wrapper. The scout's
   and the guard's `all` cases the same way, each in its own wrapper. T makes `all` a button,
   so its case's press changes then; its check does not.
6. **The control:** a self-test flag `--break forage` that makes the script press `Pillage`
   in place of `Forage`'s case; run it once and put "script goes red (exit 1) with: …" and the
   message in `relay/specs/151/buttons.md`. The review runs a different change to `panels.js`.

## Docs

- `engine/CLAUDE.md`: a `core/src/stage.rs`/`maptext.rs` row (if build-1 didn't add it),
  `www/play/buttons.mjs` in the `www/play/` row (and "every button of every panel" in its
  words), `Sim.stage` in the `web/src/lib.rs` row, `judge --compare`/`bench --after` in the
  command block (if build-6 didn't), and one line in Invariants: "A rule's proof is a stage
  (`core/src/stage.rs`), and a button's is `www/play/buttons.mjs`". Under 8 KB.
- `docs/architecture.md`: the **Stage** row's "On main" is "Holds: `Stage` and `StageSpec`,
  `map::parse`, the retro cache; `tests/common` is built on it", "Built by" "Done in M4 S";
  **Scale**'s "On main" has `bench --after`'s number (build-6 wrote it) and "Built by" keeps W;
  **The CLI** row: "Holds; `judge` is `sweep_judge.rs` and `game_judge.rs`", done in M4 S;
  **The judge** row's "On main" names the compare, the floor, the idle control and the economy
  group, "Built by" "Done in M4 S"; **Play scripts**: "The driver, the touch points and
  `buttons.mjs`", done in M4 S. Do not touch `docs/design/`.

## Done

`node www/play/buttons.mjs` passes (the Disband case by `expectBug`) and `node www/play/all.mjs`
passes; `engine/check.sh` passes in full; `buttons.md` holds the control run and the list of
every label the script covers (a table: panel, button, check). The script's runtime is in
your summary.
