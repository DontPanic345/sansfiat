# #152 build-2 (easel): the end

The game ends. Read `spec.md` items 8–15 and its decisions 1–8. The code: `core/src/world.rs`
(`Phase`, `TURN`, `validate`, `act`, `replay`, the derived fields `orders` and `picked`),
`core/src/civs.rs` (`alive`, `fall`, `is_out`), `core/src/stats.rs` (`standing`),
`core/src/events.rs`, every `Phase::Main` check (`grep -rn "Phase::Main" core cli web`),
`core/src/stage.rs` (`StageSpec`, `from_spec`), `core/src/data.rs` (`Config`, `SIZES`),
`cli/src/play.rs`, `cli/src/view.rs`, `cli/src/story.rs`, `docs/design/vision.md`.

## What to build

1. **`core/src/over.rs`**, a module of its own (`lib.rs` exports it): `Over { winner: u32, why:
   Why, turn: u32 }` and `Why::{Conquest, Limit, Fell { by: u32 }}`, both `Copy`, serde
   camelCase; the `over` step; `limit(w) -> Option<u32>` (the limit in force, `None` before
   the pick); a `winner_by_standing(w)` helper (the standing civ with the highest Standing, a
   tie to the lower id).
2. **`Phase::Over(Over)`** in `world.rs`, and `Phase::playing(&self) -> bool` (`Main` or
   `Over`). Every rule that runs only in `Main` runs while `playing()`: `leaning::lean`,
   `markets` (the capital's market), `units::step`, `units::ai`, `script.rs`'s `Lead` (it then
   gives orders the world refuses, which it ignores: make it stop at `Over` instead), and
   `orders.rs`'s phase check (the AI's `World::order` goes on in `Over`). Check each
   `Phase::Main` in `cli` and `web` too. `turn` in `Over` is the turns since the pick.
3. **The step**, last in `TURN` (`Step { name: "over", .. }`), as `spec.md` item 9 says: in
   `Main` after the pick, push the turn's Standing, then end on Conquest, then Fell, then
   Limit. `Fell` takes the civ that took the led capital from `Civ.fallen`, which is
   `Some((turn, by))`. Emit `EventKind::GameOver`
   (`civ` the winner; `other` the taker for Fell), worded in `events.rs` ("Blue won: the last
   capital standing" / "...: the highest Standing at the turn limit" / "Red's capital fell to
   Blue; Gold leads"), an alert for the led civ (`alert_key`). Note the winner.
4. **The limit.** `END: EndRules { limit: [(i32, u32); 3] }` in `data.rs`, by radius as
   `SIZES`, starting at `[(16, 200), (23, 250), (33, 300)]`, with a note that it counts turns
   after the pick and that the canvas sets it. A radius between sizes takes the next size up's;
   past the largest, the largest's. `Config.limit: Option<u32>`, `#[serde(default)]`, `None`
   for the size's: a stage, a test or a script sets a short one. `Config::default()` gives
   `None`.
5. **Standing over time.** `World.standing: Vec<BTreeMap<u32, f64>>`, `#[serde(skip)]`, doc as
   `orders`' says derived: `ChooseCiv` pushes `stats::standing` (index 0, at the pick), the
   `over` step one a turn after, in `Main` and `Over`. A replay rebuilds it, so its length is
   the turns since the pick plus one.
6. **Refusals.** In `Over`, `validate` refuses `Action::Order`, `SetKeep` and `SetGift` with
   `"the game is over"`. `EndTurn` goes on.
7. **The CLI.** `view.rs`'s first line says the phase and, in `Over`, the end ("the game is
   over: Blue won, the highest Standing at the turn limit, turn +200"); otherwise it shows the
   turns left ("… 37 of 200 turns since the pick …"). `play.rs` accepts `end` in `Over` and
   prints the refusal for an order. `story.rs` stops at `Over` or at `--turns`, whichever
   comes first, `--turns 0` meaning to the end, and its summary line ends with the end ("game
   over at +200: Blue won by the limit") or "the game goes on".
8. **The stage.** `StageSpec.limit: Option<u32>` sets `config.limit`; `Stage::over() ->
   Option<Over>`.
9. **`SAVE_VERSION`** is bumped once by the piece (build-1). The digests move again here (a
   field on `Config` and a new phase are in the world's JSON): move the pins in
   `cli/tests/save.rs` and keep build-1's version number unless main has released it (it
   hasn't: one bump per piece).
10. **`docs/design/vision.md`, Winning**: replace "Not decided yet. These are the usual
    options, held equally open:" with the first way to win, in the file's own voice (plain,
    present tense): a civ loses when its capital falls; the game is won by the last capital
    standing, or by the most Standing when the turn limit comes. The list stays, introduced as
    the ways that may come later; drop "Time limit" from it or say the first way covers it.
    Edit nothing else in `docs/design/`.

## Tests (`core/tests/over.rs`, on the stage)

- A two-civ stage, led by the first: its army on the rival's capital, the rival's guard
  disbanded and its capital clan small enough to fall in a turn or two (read `fight.rs`'s
  militia rule): within a few turns `Phase::Over` with `Why::Conquest`, the winner the led civ,
  a `GameOver` event, and an `Order` refused with "the game is over"; `EndTurn` still works and
  the turn advances.
- A three-civ stage led by the first, a rival army taking the led capital: `Why::Fell { by }`
  with `by` the taker, the winner the highest-Standing civ left. The same with no led civ
  (watching): one capital falling does not end it.
- A stage with `limit: 3` after the pick: `Over` at +3 with `Why::Limit` and the winner the
  highest Standing; two civs set to equal Standing (or a constructed tie) go to the lower id.
- `World.standing` has `turns since the pick + 1` entries, in `Main` and after the end.
- A world with one civ never ends by conquest.
- A game played past its end saves and replays to the same `stats::digest` and the same
  `standing` (the save tests' pattern).
- After `Over` the world runs on: an AI civ's orders still go through `World::order`, and
  clans keep moving.

## Done

`engine/check.sh` green; `engine/CLAUDE.md`'s layout row names `over.rs` and the invariant
that the end is a phase; `www/check.mjs` still passes (the page reads `phase`: the page's own
change is build-4, so make sure only that `"main"` still reads as before).
