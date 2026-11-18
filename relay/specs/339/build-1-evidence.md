# #339 build-1: evidence

## The split

`core/src/stage.rs` gave its data form to `core/src/stage_spec.rs` as a pure move (commit 3b1a3cd):
`TileRef`, `ClanSpec`, `SignpostSpec`, `RetroSpec`, `HiddenSpec`, `FoodSpec`, `ArmySpec`, `StageSpec`
and `Stage::from_spec`. `stage` re-exports the types, so `stage::StageSpec` and its callers are unchanged.
`stage.rs` went from 585 lines to 427; `stage_spec.rs` is 170 lines before the new fields.

## The new fields

- `map` and `marks` take a string or an array of rows, joined with newlines (`text_or_rows`).
- `scouts: [{civ, at, pack, dest?}]`, applied after the armies: the scout's home and territory go to `at`,
  its pack to `[0, pack, …]`, and its `dest` if given.
- `stores: [{civ, food, everyTurn?}]`, applied last: the civ's store becomes `[0, food, …]`. `everyTurn`
  is read by the runner.

## Digest check (the errand's world)

`Stage::from_spec` of the errand's map text, with the forest and mountain glyphs, the four hidden costs at
`1.3333333333333333`, civs at (-10, 0) and (10, 0), 14 people and lead 0, but without the errand's `scouts`
and `stores` (the Rust test moves its scout after its digest), has `stats::state_digest`
`f81d47a26acde6cb`. The Rust test's own world (flat, civs, lead, `plain()`, the paint) has the same digest.
The bare flat world is `973a5aaf19d0d654`.

## The four scout tests as data

Each runs through `cargo test -p engine --test stage_list` and passes. Each hold is checked against the
Rust test it came from:

| test | holds |
| --- | --- |
| scout-errand | never hungry; one trip to the store; walks back; built by turn 10; a trail at each of the four forest tiles |
| scout-killed | the scout is gone and one `scoutKilled` event after turn 1 |
| scout-trapped | at turn 2: shut, on the tile, `trapped`, its line starts "trapped"; off the tile by turn 4 |
| scout-walks-round | never on a shut tile across six turns; at the end, at the far side |

Break check: the errand's `by 10` built hold changed to `by 2` fails with
`scout-errand: failed on turn 2: by turn 2: Red's scout has built its trails: it does not hold`.

## Judge and scout off the retro pick

- `core/tests/judge.rs`: the three economy tests on `civs_on_flat(3, 14.0)`. The levy test gives each
  capital 50 grain first: a flat map has no food, and the levy is paid from what clans gather.
- `core/tests/scout.rs`: eight tests on `lead()` = `Stage::flat(BIG)`, two civs at (0, 0) and (10, 0), the
  first led, fight off. The origin capital keeps the six-tile rays and the radius-3 ring on the map. The
  `plain()` calls go, as a flat map is bare. `open_ray` and `capital` move to `core/tests/common/mod.rs`.
- `a_route_order_replays_to_the_same_world` moves to `core/tests/stage.rs` as it is, on the retro pick
  (`retro_led(2)`), with the helpers it needs. A staged world cannot replay: its placements bypass the log.
- `core/tests/scout_feed.rs` is deleted; its four tests live in `core/stage/`.

## Tests run

- `cargo test -p engine --test stage_list`: 2 passed.
- `cargo test -p engine --test scout --test stage --test judge`: scout 8, stage 13 (one new:
  `rows_scouts_and_stores_of_a_spec_are_set`), judge 19.
- `node www/check.mjs`: passes, including the stage door case (the errand passes from its file; a copy with
  `"by": 2` on its built hold fails at turn 2 with the hold's words; a game with no test has no test state).
- `engine/check.sh`: exit 0 on the branch before the failed-hold message was reworded; after it,
  clippy, the stage, scout, judge and stage_list tests, and `check.mjs` were rerun and pass. The page
  scripts' seconds, from `all.mjs`, sum to 174.0 s (22 scripts, all pass; main's `allmjs-main.txt`
  total is 174.7 s).

## Size

`engine/CLAUDE.md` is 8123 bytes (budget 8192). Three rows were tightened to make room (towns, the
`www/play.mjs` row, the `www/play/` row, and `judge.rs`); the stage row and the invariant name `stage_spec.rs`,
`stage_test.rs` and `core/stage/`.
