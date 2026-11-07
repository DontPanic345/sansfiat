# #152 build-3 (easel): the judge reads the end

Read `spec.md` items 16–19. The code: `core/src/judge.rs` (`Watch`, `Scores`, the groups),
`core/src/over.rs` and `World.standing` (build-2), `cli/src/sweep_judge.rs` (`judge_map`,
`played`, `played_all`, `total`, `print_text`), `cli/src/game_judge.rs`, `cli/src/compare.rs`
(the group order), `cli/tests/judge.rs`, `core/tests/judge.rs`.

## What to build

1. **The split first, byte for byte.** `sweep_judge.rs` is 664 lines. Move `print_text` and
   the helpers only it uses into `cli/src/sweep_text.rs`. Proof: `sansfiat sweep --judge`
   and `--json` from before and after the move are identical but for `seconds`. Commit it on
   its own.
2. **`EndScores`** in `core/src/judge.rs`, a field `end` on `Scores`:
   - `over: Option<EndOver>`, `EndOver { why: String ("conquest" | "limit" | "fell"), winner:
     u32, turn: u32 }`, `turn` counted from the pick;
   - `early: bool`: over by conquest or fell (before the limit);
   - `capitals_fallen: u32`: `CapitalFell` events after the pick (count them in
     `Watch::turn` from the events, as `clans_left` is);
   - `ranks: BTreeMap<u32, usize>`: each civ's rank in Standing (1 the largest, ties sharing
     the better rank, as `game_judge::rank_of`) at `Over` if the game ended, else at the
     watch's last turn: read `World.standing` at the over turn's index;
   - `led_rank: Option<usize>`: `ranks[player]`, `None` when watching.
   Move `rank_of` into `core/src/judge.rs` so both judges use one.
3. **The sweep plays to the end.** In `sweep_judge.rs`, `played` runs each config until the
   world is `Over` and at least to +200 (the `AFTER` constant stays as the floor), so every
   +200 reading is where it was. Guard the loop with a backstop (the largest limit plus a
   margin) that panics with a message if a game never ends: a sweep that doesn't end is the
   failure the piece guards against, and a panic shows in `panics`.
4. **The sweep's report.** `MapJudged` keeps `scores` and `no_fight`; both now carry `end`.
   `Total` gains `end: TotalEnd { early, of, no_fight_early, turns_mean, no_fight_turns_mean,
   capitals_fallen, no_fight_capitals_fallen }`. The text (in `sweep_text.rs`) prints per map
   one line after the fight line, "end: limit +200, winner Blue; no fight: limit +200; capitals
   fallen 0 (0)", and a total line after `total fight`: "total end: games ended before the
   limit 0 of 7 (no fight 0 of 7), turns to the end 216.4 (216.4), capitals fallen 0 (0)".
   `compare.rs`'s group order gains `end` after `fight`.
5. **`judge FILE`.** `GameJudged`'s three `Scores` carry `end`; `Ending` gains `game_end`,
   `control_end` and `idle_end` (how each ended, or "goes on", and the led civ's rank at its
   end). The controls are played to the game's turns, as now: a control that hasn't ended by
   then reads "goes on" with its rank at its last turn; a game played on past its end reads its
   ranks at `Over`. `print_game` prints "end (game / control / idle): fell +88 rank 3 / goes on
   +88 rank 2 / goes on +88 rank 2" or "limit +200 rank 1 / limit +200 rank 2 / limit +200 rank
   2", beside its Standing lines.
6. **Tests.** `core/tests/judge.rs`: on a stage with `limit: 3`, `Watch` reads `over` with
   why "limit" and `turn` 3, `early` false, and `ranks` from `World.standing`; on the
   build-2 conquest stage, why "conquest", `early` true and `capitals_fallen` 1.
   `cli/tests/judge.rs`: `judge FILE` on a story save prints the end line; the sweep's JSON
   (one small map is enough if the tests already build one) carries `end` on `scores` and
   `no_fight`.

## Record

`sansfiat sweep --judge` on the branch: put the end lines and the total block in
`relay/specs/152/judge.md`, with the wall time beside main's (32–34 s), since every run now
plays to its limit (expect little: the spec-review probe ran 600 turns on all seven maps in
19 s).

## Done

`engine/check.sh` green; the split byte-identical (item 1's proof, said in your summary);
`engine/CLAUDE.md`'s `cli/src/` row names `sweep_text.rs`.
