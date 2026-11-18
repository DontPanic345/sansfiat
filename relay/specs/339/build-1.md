# #339 build-1 (ford): stage tests as data, the runner, the wasm door, two core files off the pick

Read `relay/specs/339/spec.md`: "The format, settled" is your contract, field by field. build-2
builds the Tests screen on your wasm door, and builds 3 and 4 move page scripts onto
`StageSpec` with the fields you add. Keep the format exactly as the spec writes it; where it
can't hold, make the smallest change that works and record it as a decision.

## 1. Split `stage.rs` first

`core/src/stage.rs` is 585 lines and this step adds to it. Move the data form out first, as a
pure move: `StageSpec` and its parts (`TileRef`, `ClanSpec`, `SignpostSpec`, `RetroSpec`,
`HiddenSpec`, `FoodSpec`, `ArmySpec`) and `Stage::from_spec` into `core/src/stage_spec.rs`,
re-exported from `stage` so every caller (`stage::StageSpec`, the web crate, the tests) builds
unchanged. Commit that alone.

## 2. `StageSpec` gains

- `map` and `marks` read a string or an array of rows, joined with `\n` (a serde
  `deserialize_with`); a string reads as now.
- `scouts: [{civ, at, pack, dest?}]`: moves civ `civ`'s scout to `at` (`actor.home` and
  `actor.territory`), sets its pack to `pack` grain (store index 1, as `scout_feed.rs` sets
  `[0.0, 10.0, 0, …]`), and its `dest` if given. Applied after armies.
- `stores: [{civ, food, everyTurn?}]`: civ `civ`'s store set to `food` grain (index 1) when the
  stage is built. `everyTurn` is read by the runner, not by `from_spec`.
- Document each field in the struct's doc, as the others are.

## 3. The runner: `core/src/stage_test.rs`

- `StageTest` (serde, camelCase): `name`, `claim`, the stage (`#[serde(flatten)] stage:
  StageSpec`), `orders: Vec<TestOrder>`, `turns`, `holds: Vec<Hold>`. `Hold` and `Fact` as the
  spec writes them; serde's externally tagged enums with `rename_all = "camelCase"` give that
  JSON nearly as is.
- `LIST: &[(&str, &str)]`: each test's name and its file, `include_str!("../stage/NAME.json")`.
  `pub fn list() -> Vec<StageTest>` parses them (a bad file panics with its name).
- `Run`: built from a `&StageTest` (`Run::new(test) -> Result<(World, Run), String>`, or holding
  the world itself: your choice, but the wasm `Sim` must own the world and step it). `step`
  plays one turn: keeps the `everyTurn` stores, gives the orders due (resolving `{"mark"}`,
  `{"tile"}`, `{"scout": k}`, `{"camp": k}` in the order's JSON, then `serde_json::from_value`
  into `Order`; the led civ's through `act(Action::Order …)`, as `common::led_order` does, the
  others through `World::order`), ends the turn, and updates every hold's state. `done()`,
  `states()`, `passed()`, and `run_all(test) -> Result<(), String>` that plays it to the end and
  names the first failed hold, its words and its turn.
- `Hold::words(&self, w: &World, marks)` writes the line the page shows ("by turn 10: Red's scout
  has built its trails"); a mark is written as its tile's `(q, r)`.
- Facts read the world through what exists (`units::scout`, `units::shut`, `Unit.hungry`, the
  scout state's `homing`, `built`, `doing`, `Tile.trail`, `World.events`). No new reads in
  `units/`.

## 4. The four scout tests as data, and `cargo test`

- `engine/core/stage/scout-errand.json`, `scout-killed.json`, `scout-trapped.json`,
  `scout-walks-round.json`: the four tests of `core/tests/scout_feed.rs`, in that order in
  `LIST`, each holding everything its Rust test asserts. The map is the `BIG` (radius 12) flat
  map as text: plains inside, sea on the ring (`Stage::flat`), with the errand's forest and
  mountains painted in as glyphs and its trail costs as `hidden`. `Stage::parse` and
  `Stage::flat` then `plain()` must give the same world: check that `stats::digest` of the two
  agree in a test before relying on it, or explain the difference.
- Run both forms before you delete anything: each data test passes, and with one hold changed
  so it can't hold (`"by": 2` on the errand's trails) it fails naming that hold. Then delete
  `core/tests/scout_feed.rs`.
- `core/tests/stage_list.rs`: `every_listed_test_holds` runs `list()` through `run_all` and
  fails with every failing test's message; `every_file_is_listed` reads `core/stage/`
  (`env!("CARGO_MANIFEST_DIR")`) and fails on a file not in `LIST`.

## 5. The wasm door, `web/src/lib.rs`

- `tests() -> String`: the list as JSON: per test its `name`, `claim`, `turns`, the kept stores
  as words, and its holds' words.
- `Sim::stage_test(json: &str) -> Result<Sim, String>`: a test's JSON (one entry of the list's
  files, or a changed copy), its world staged, its `Run` attached; `staged()` is true.
- `test_step(&mut self) -> String` plays one turn and returns `test_state()`; `test_state() ->
  String`: `{turn, turns, done, passed, holds: [{words, state, turn?, why?}]}`. Both refuse
  (`Err`) on a `Sim` with no run.
- The page's existing reads (`view`, `layout`) work on a test's `Sim` as on any staged one.
- `www/check.mjs`: one case that builds `scout-errand` through the door, steps it to the end,
  and reads `passed: true`; and a copy with `"by": 2` that reads `passed: false` with the
  failed hold at turn 2. (`check.mjs` runs under Node with the wasm, so this proves the door
  without Chrome.)

## 6. `judge.rs` and `scout.rs` off the retro pick

- `core/tests/judge.rs`: the three economy tests (`a_staged_store_at_a_pack_is_a_turn_at_pack`,
  `a_staged_army_sets_the_largest_army`, `the_levy_pays_in_the_first_turns_after_the_pick`) on
  a built stage: `civs_on_flat` in the same file, or `Stage::flat` with civs and clans enough
  for the levy to pay. Same assertions.
- `core/tests/scout.rs`: `lead(seed)` becomes a built stage (`Stage::flat(BIG)` with civs,
  led, fight off), and each test paints what it needs, as it already paints the retro land.
  Every assertion stays; a test that relied on the real map's terrain paints it. If one can't
  hold on a built map without changing what it claims, leave that test on `Stage::retro`, say
  which and why in your summary.

## Done

- `cargo test -p engine --test stage_list` runs the four scout tests from data and passes;
  `scout_feed.rs` is gone; `judge.rs` and `scout.rs` read no `Stage::retro`.
- The `check.mjs` door case passes.
- `engine/check.sh` passes; `stats::digest` pins and `SAVE_VERSION` unchanged.
- `engine/CLAUDE.md`: the stage row names `stage_spec.rs` and `stage_test.rs` and
  `core/stage/`; one line under Invariants: a rule's watched proof is a test in `core/stage/`
  (keep the file under 8 KB).
- Your summary gives build-2 the door's exact JSON shapes.

About 10 files, 700 lines: `stage_spec.rs` (moved ~320, +60), `stage_test.rs` (~300),
four JSON files (~40 each), `stage_list.rs` (~40), `web/src/lib.rs` (~50), `check.mjs` (~30),
`judge.rs` and `scout.rs` (~60 changed), `engine/CLAUDE.md` (2).
