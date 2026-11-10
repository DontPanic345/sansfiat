# #157 build-1 (easel): the judge's led games, and the fight's panic

The piece makes the AI civs play for Standing with the player's levers, and asks whether a led game is ever
in doubt. Before any planner exists, give the judge the reading that answers it, measure it on today's AI
(the stand-in, which only claims and guards), and fix a panic that fighting armies will hit. Nothing here
changes a rule: every digest stays as it is, except the fix, which no retro map reaches today.

Read `relay/specs/157/spec.md` (the question, the table of bounds) and `probe.md`.

## 1. The fight's panic

`core/src/units/fight.rs`, in `step`: `wiped` indexes `lost`, which holds only the civs with more than
`FIGHT.floor` people on the tile, and it is called on `holder` (`holder.is_some_and(|h| h != by && (wiped(h)
|| …))`). When the tile's holder (by claim, `holder_of`) has no people on it and two other civs fight there,
`lost[&h]` panics with "no entry found for key". The spec step's probe hit it on small 2 and small 6 once
every AI civ marched armies. A civ with no people on the tile has none of its people left there: `wiped` reads
a civ missing from `lost` (or `sides`) as wiped.

Test in `core/tests/fight.rs` on a stage (`core/src/stage.rs`): three civs; a tile of civ A's claim that A's
guard doesn't stand on (or stands on under `FIGHT.floor`), and armies of B and C on it. The turn runs; the tile
goes to the winner as the rules say. It panics without the fix.

## 2. The led games (`core/src/judge_led.rs`)

A **led game** is one civ's view of a watched run: the game as it would go if the player led that civ and the
AI played it, which is the same world (`spec.md`, decision 2), with its own end: the first turn after the pick
on which that civ's capital has fallen, or at most one capital stands, else the limit (`over::limit`). Its
winner is as `vision.md` says: the last capital, else the standing civ with the most Standing (ties to the
older civ).

The `Watch` already keeps every civ's Standing per turn (`standing_at`). It gains each civ's fall turn (from
`CapitalFell` events, or `is_out`), so a led game's end is read without replaying. From a `Watch`, for each
civ `k`, `LedGame`:

- `civ`, `rank_pick`, `rank_end` (1 the most Standing);
- `rank_changes`: how many times `k`'s rank changes after the pick, **with the lead's margin**: `k` counts
  another civ `j` as above it until `k` is ahead of `j` by more than `JUDGE.lead_margin`, and as below it until
  `j` is ahead by more than the margin; its rank is one more than the civs above it. At the pick, the order is
  the plain order of Standing. A fallen civ is below every standing civ;
- `end_turn` (turns since the pick), `why` (`fell`, `conquest` or `limit`), `early` (not the limit), `winner`;
- `standing_end`: `k`'s Standing at its end;
- `in_doubt`: `rank_changes >= 1 || early`.

A test in `core/tests/judge.rs` on a hand-built `Watch` or a staged run: the margin holds a rank (two civs
within the margin trade places with no change counted), a fall ends the led game at its turn, and the end's
rank and winner are as `vision.md` says.

## 3. The idle control and the sweep (`cli/src/sweep_led.rs`)

The idle control for civ `k` is the same map to the same pick with `k` led and given no orders, every other
civ on the AI: `game_judge.rs`'s `idle_run`. Move it where both can use it (`retro.rs` is a fine home). The
sweep plays it for each civ on each map, three runs a map beside the seven it plays, in the same parallel
`played_all`, for as many turns as the watched run. Each map gains `led`:

- the three `LedGame`s from the watched run, and the three from the idle runs (each idle run's own led civ);
- per civ, **beats idle**: `standing_end` less the idle run's Standing at the same turn;
- the map **in doubt**: at least two of its three led games in doubt.

`total.led`: led games in doubt (of 21), maps in doubt (of 7), led games ended early (and how many before
+50), rank changes in all, the mean beats-idle and how many are positive, and the idle runs' rank changes and
in-doubt count beside. Lead changes and clans left are already in the report: print them in the same block.
The text output prints the block per map and in total. `judge --compare` reads the new numbers through
`flatten`, as it does every number; check `--floor`'s nulls give them a floor.

`sweep_judge.rs` (376 lines) gets only the call and the field. If `sweep_text.rs` or `sweep_total.rs` would
pass 300 lines, the led text and totals go in `sweep_led.rs`.

## 4. `judge FILE`

`cli/src/game_judge.rs` already plays the AI control and the idle control. Its report gains, for the game, the
control and the idle control: the led civ's rank changes, in doubt, and lead changes; and beats-idle (the
game's Standing at its end less the idle control's at the same turn), with whether it is past the floor
(`JUDGE`'s, or the sweep's floor for Standing if one is at hand: say which in `judge.md`).

## Proof, and what later steps rely on

- `engine/check.sh` green. Every digest pin unchanged (`cli/tests/save.rs`).
- `sansfiat sweep --judge --json --floor` on the branch: every map's existing blocks equal the base's
  (`git stash` or a worktree of the parent commit), and the `led` block present. Write
  `relay/specs/157/judge.md`: the per-map and total led block on the stand-in, as the base every later step
  reads against (led games in doubt, maps in doubt, early ends, rank changes, beats-idle and its count, the
  idle runs beside, lead changes, clans left, orders per 100 by kind, levers repeated), the sweep's seconds
  before and after, and the floor of each new number.
- `sansfiat judge FILE` on one saved game (`sansfiat play --game g.json`, a few `end` lines) prints the new
  lines.
- Build-2, build-3 and the canvas read `total.led` and the per-map `led` in the JSON. Name the JSON paths in
  `judge.md`.

`engine/CLAUDE.md`'s layout table gains `judge_led.rs` and `sweep_led.rs`.
