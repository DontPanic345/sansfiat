# #151 build-5 (ford): the judge's split, the idle control, the economy group

Three things in the judge's area, in this order, each its own commit. Read `spec.md` items
15–17, `cli/src/judge.rs` (833 lines: the sweep's judge and the file's judge in one),
`core/src/judge.rs` (`Watch`, `Scores`, `civ_lands`), `core/src/civs.rs` (`levy`, `upkeep`),
`core/src/units/ai.rs` (it skips the civ the player leads) and `cli/tests/judge.rs`.

## 1. The split, byte for byte

`cli/src/judge.rs` becomes two files (`mod` lines and `use` lines in `main.rs` change; shared
helpers like `mean` go in whichever file owns them, and the other imports them):

- `cli/src/sweep_judge.rs`: `Health`, `Armies`, `SoloGain`, `MapJudged`, `Panicked`, the
  `Total*` structs, `Report`, `played`, `played_all`, `at_200`, `judge_map`, `message`, `mean`,
  `mean_some`, `civ_means`, `total_land`, `total_solo`, `total`, `judge`, `print_text`.
- `cli/src/game_judge.rs`: `GameJudged`, `Ending`, `Lands`, `Player`, `rank_of`, `judge_game`,
  `print_game`.

**Proof, before any other change:** save `sansfiat sweep --judge --json` and
`sansfiat sweep --judge` and `sansfiat judge FILE [--json]` (a game from `sansfiat story
skirmish --size small --seed 2 --save /tmp/g.json`, any short one) from main's binary, then
from yours after the split: identical but for the `seconds` field. Say so in your summary.

## 2. The idle control

`judge FILE` (in `game_judge.rs`) plays a third run: the same config to the same pick
(`retro::to_pick`), `ChooseCiv { civ: Some(led) }`, then `EndTurn` for the game's turns, no
order, under its own `Watch`. Add to `GameJudged` an `idle: Scores` and to `Ending` the idle
Standing and rank (`idle`, `idle_rank`), and print both in `print_game` beside the game's and
the AI control's, labelled "idle led civ". Update `cli/tests/judge.rs` for the field, and
add a test: the idle run is deterministic (two runs of the same file give the same
`idle` Standing) and its led civ has an empty order log (assert on the run's `w.log`).

## 3. The economy group

In `core/src/judge.rs`: `EconomyScores { store_mean, store_max, turns_at_pack, levy_per_turn,
largest_army, guard_mean }` as defined in `spec.md` item 17, added to `Scores` as `economy`,
read by `Watch::turn` each turn over the alive civs, finished in `Watch::scores`.
- `levy_per_turn` needs `Civ.levied: f64`, `#[serde(skip)]`, set in `civs::levy` where the
  civ's store is credited (the total of what it paid in; zero it at the start of the `civs`
  turn step, or set it unconditionally where `levy` is called so a turn with no levy reads 0,
  whichever keeps one place that writes it). **No digest may move:** run
  `cargo test` (the pins in `cli/tests/save.rs` stay) and `./cross-build.sh`. Say in the doc
  comment that it is derived and not saved.
- `turns_at_pack` uses `units::pack_cap(UNIT.start as f64)`.
- `sweep_judge.rs`'s `Total` gets `economy` (means over maps; `store_max` and `largest_army`
  are the largest over maps) and `print_text` prints one line for it under the groups it
  already prints, with the same layout.
- A test in `core/tests/judge.rs`: on a staged world (`Stage`), a civ whose store is set to
  a pack's worth for a turn counts one turn at pack; `largest_army` follows a staged army's
  people; `levy_per_turn` is positive on a retro map's first turns after the pick.

## Record

Run `sweep --judge` once on the branch and put the economy group's lines (the text) in
`relay/specs/151/economy.md`, with one sentence on what they say about the stores: the
spec-review's measure in #104 was that a civ's store is 0.0 at the pick and about 3–4 food a
turn comes in by the levy. Say if the numbers agree.

## Done

`cargo test`, `clippy`, `fmt`; `./cross-build.sh`; every digest unchanged; the split
byte-identical (item 1's proof); `engine/CLAUDE.md`'s `cli/src/` row names the two files.
