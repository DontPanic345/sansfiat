# Build 7 (ford): the CLI split, shared test fixtures and the map

Piece #100. Read `relay/specs/100/spec.md` items 18, 19 and 20. Nothing here changes what the engine does;
this step makes the repo easier to read for the sessions after.

## The work

1. **`cli/src/main.rs` (≈1,000 lines) → a module per command** (item 19): `main.rs` keeps `USAGE`, `size`, `main` and
   the dispatch; `run.rs` (`run`, `Output`, `print_events`, `print_notes`), `map.rs`, `digest.rs`, `sweep.rs` (`Run`,
   `Point`, `sweep_run`, `sweep`, `RETRO`, `retro_maps`, `CivArmy`, `MapRun`, `after_run`, `sweep_after`), `replay.rs`,
   `bench.rs`, `clear.rs` (`basket_of`, `clear_file`), `report.rs` (`line`, `centres_text`, `report`, `civs_text`).
   Output is byte-identical: diff `run`, `run --why`, `sweep`, `sweep --after 200 --maps retro`, `replay`, `map` and
   `bench` output (timings aside) before and after on seeds 2 and 3, small.
2. **`core/tests/common/mod.rs`** (item 18): the helpers the test files define for themselves move there, once: at
   least `three`, `led`, `markets3`, `fill_stores`, `found`, `put_camp`, `camp_id`, `end_turn`, `total`, `kinds` and
   the `land`/`made`/`walk`/`nbrs`/`centre` helpers where two files define the same thing. Compare bodies before you
   merge two; where they differ, keep both under different names. Delete the copies; `#![allow(dead_code)]` on the
   module as it has now. `cargo test` counts the same number of tests (record before and after) and none is `#[ignore]`d.
3. **`engine/CLAUDE.md`** (item 20): the layout table has one row per file in `core/src` (including
   `core/src/units/*`, `orders.rs`, `script.rs`, `events.rs`), `cli/src`, `web/src` and `www`, one clause each; under
   *Adding a mechanic*, add "Shared actor behaviour (spoil, eat, walk) goes in `actors.rs`; a new actor is a new state on
   it, not new machinery", change item 3 to say a thing the player or an AI does to a unit is an `Order`
   (`orders.rs`, through `World::order`; the player's go through `Action::Order` and are logged), and put "(and the AI's
   orders, which a replay re-derives)" into the *world changes only through `act`* invariant. The command list gains
   `digest [--lead] [--state]`.

## Done

`./check.sh` green; the five baseline runs identical and the retro sweep `diff` empty (run them; say so); the number
of tests is the number before; `wc -l cli/src/*.rs` shows no file over about 450 lines; every file in
`core/src`, `core/src/units`, `cli/src` and `web/src` has a row in `engine/CLAUDE.md`.

The comparison with the baselines is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines.
