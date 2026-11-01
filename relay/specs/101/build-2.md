# #101 build-2 (easel): `sansfiat play`, the text view, `sansfiat judge`

You build the playing half of the easel: a terminal game a human or a model plays a line at a time,
the text view it reads, and the judge over a played game. `spec.md` in this folder items 13–17 are the
full rules; this brief says where they go and what done is. Build-1 gave you `stats::standing`,
`World.orders`, `core/src/judge.rs`'s `Watch` and the report printer in `cli/src/judge.rs`: read its
summary first, and use them rather than measuring again.

## Rules that hold everything

- **Orders go through `World::act`**, the door the page uses, so through `validate`. `play` never
  calls `World::order` or changes a world any other way. The only thing `play` adds to the world is
  actions in its log.
- **No digest moves**; `compare.sh` (`relay/specs/100/baseline/`) prints six `ok` lines when you finish.
- The game file is a plain `Save` (`World::save`), so `sansfiat replay g.json` reads it.
- The view is read by a model every decision, so it is short: under 80 lines on small seed 2 at the
  pick and at +100. Prefer one line per thing, and numbers to one decimal (Standing as a percentage).

## What to build, and where

1. **`cli/src/play.rs`** (new): `play --game FILE [--seed N --size S [--civ K]]` (spec items 13–15).
   Making a game: run the pre-game with `act(EndTurn)` while the phase is `Pregame` (not `World::run`,
   which watches at the pick), then `ChooseCiv` the `K`-th civ in id order. Without `--civ`, print the
   cards and stop. With the file there, `World::replay` it; flags that disagree with the file are an
   error. Read stdin a line at a time (it may be a pipe or a person), act on each as item 14 says,
   rewrite the game file and append to `FILE` with `.json` replaced by `.lines.jsonl` after each line.
   Parse a JSON line as `Action` first, then as `Order` (sent as `Action::Order` for the led civ).
   "The feed since the last call" is the events after the turn of the lines file's last entry (or the
   pick, for a new game). `map` reuses `cli/src/map.rs`, with the led civ's capital and units marked.
2. **`cli/src/view.rs`** (new): `text(w: &World, since: usize) -> String`, the view of spec item 16, in
   that order. Reuse what `web/src/lib.rs`'s `view()` and `report.rs` already compute (`units::line`,
   `cut_off`, `cut_at`, `pack_cap`, `zone_cap`, `post`, `civs::keep`, `markets::reach`, the clans'
   `leaning`, `Event::text`); move a computation into core only when both the page and the view need
   it and it isn't there. The legal orders come from `validate` (try each order with what the view
   would fill in, and show it only if it passes); a tile order is shown with a placeholder tile and the
   places it lists.
3. **`cli/src/judge.rs`**: `judge FILE [--json]` (spec item 17): replay the save one action at a time,
   calling `watch.turn` after each `EndTurn`; play the control (`World::new` with the save's config,
   to the same pick, `ChooseCiv { civ: None }`, every civ on the AI, to the same turn) with its own
   `Watch`; print as item 17 says, using build-1's printer for the shared groups. The player's orders
   come from the save's log; refusals and notes from the lines file when it is there.
4. **`cli/src/main.rs`**: dispatch `play` and `judge`; USAGE lines for both.
5. **`engine/CLAUDE.md`**: rows for `play.rs` and `view.rs`, and the `play` and `judge` command lines.

## Tests (`cli/tests/play.rs`)

Run the binary (as the other CLI tests do) on small seed 2:

- `play` without `--civ` prints three cards and writes no file.
- A session piped in: make the game with `--civ 1`; a refused order (a move for a unit that isn't
  yours) prints `refused:` and leaves the log as it was; an accepted `move` of its camp; `note test`;
  `why` on the camp; `end 3`. The turn is the pick + 3.
- A second call with `view` on the same file resumes at the same turn and the same `stats::digest`, and
  the lines file has every line of both calls with its result.
- The view at the pick is under 80 lines.
- `judge` on that game prints a score line and the control's Standing for the led civ.

## Done

- `./check.sh` passes; `compare.sh` six `ok`.
- A transcript in `relay/specs/101/build-2-play.txt`: small seed 2, `--civ 1`, the view at the pick, a
  move toward the AI's post, a zone, a gift, `end 10`, `why` on the camp, a refused order, `end 90`,
  and `sansfiat judge`'s output on the game. Your summary names it and says the view's length in lines
  at the pick and at +100, and how long one call takes at +100 on small and on large seed 8.

Build-3 (the canvas) relies on: a game file a shell can resume one call at a time, the lines file with
notes and refusals, the view under 80 lines, and `sansfiat judge` on a game. It may change the view
where the trial shows a gap.
