# #102 build-1 (easel): festivals that stick, trails by traffic, desert

You build the second easel first: the festival and trail rules and desert's cost, with the measures
that show them, tuned to the targets below. The land rules (build-2) are tuned on top of what you
leave, so your numbers must be final for them. `spec.md` items 1–6 are the rules; `probe.md` has
what main and the spec's variants measured. Read both first.

## Before you change anything

Add the measures (below) and run `cargo run --release -p sansfiat -- sweep --judge` on main's rules.
Save it as `relay/specs/102/build-1-main.txt`: the baseline your targets are read against (main's
`flips`, `markets`, `held_trailed`).

## What to build, and where

1. **The measures** (spec item 5), in `core/src/judge.rs`'s `Watch`/`Scores` as a `land` group, and
   `cli/src/judge.rs`'s text and `--json`: `homes_trailed`, `held_trailed`, `route_trailed`, `markets`
   and `trading` at the pick and +200; `festival_changes` and `flips` per 100 clan-turns after the pick.
   The trail shares are pure functions of a world: put them in `stats.rs` (`stats::trails(w)`) so the
   view and tests can read them. `Watch` keeps each clan's last few festivals to count flips; it stays
   read-only.
2. **Desert** (spec item 1): `TERRAIN[DESERT].move_cost` 2.0, its note saying why (owner note 1: clans
   routed through desert).
3. **Festivals stick** (spec item 2) in `core/src/markets.rs` (`choose`, `go`), with `Clan.market_turns`
   in `core/src/clans.rs` (saved, `#[serde(default)]`). `MARKET` gains `keep_turns` (start 5) and
   `stay_margin` (start 3.0). The note on `go` when a clan stops says the walk wasn't worth it by the
   margin; a clan held by `keep_turns` writes no note.
4. **Spacing and reach** (spec item 3): `MARKET.spacing` 6, `MARKET.reach_walk` 14 to start; move within
   5–7 and 12–16 if the targets need it.
5. **Trails by traffic** (spec item 4) in `core/src/land.rs` (`tend`), `core/src/actors.rs` (what
   `mark_walked` becomes: adding a walker's people to the tiles of its walks, home included),
   `core/src/clans.rs` (the harvest step's call), `core/src/units/camp.rs` (the line on a turn it
   resupplies, and keeping its trail) and `core/src/units/scout.rs` (its walk and route). `Tile.traffic`
   replaces `walked`, `walk_run` and `rest_run` (`core/src/map.rs`); grep for every reader. `TRAIL` gains
   `keep`, `on` and `off` in place of `after` and `fade`; starting numbers are yours, from the traffic a
   home and a shared route carry on a retro map (print a histogram once; don't commit the printer).
6. **The save**: `SAVE_VERSION` 3, the pinned digests in `cli/tests/save.rs` re-pinned. The test in
   `cli/tests/play.rs` that reads `relay/specs/101/trial/g.json` (a version-2 save) is rebuilt on a game
   it makes itself, keeping what it checks; #101's file stays as it is.
7. **The page and the text view** that read the old trail fields (`web/src/lib.rs`, `www/`, `cli/src/view.rs`)
   keep working; nothing new is drawn here.
8. `engine/CLAUDE.md`: the rows that change (`land.rs`, `actors.rs`, `markets.rs`, `judge.rs`).

## Tests (`core/tests/festivals.rs`, `core/tests/wear.rs`; built maps from `tests/common`)

- A clan that just started at a festival keeps it for `keep_turns` though another is worth more; after
  that it keeps one worth a little under 0 and stops at one worth less than `−stay_margin`.
- A clan's home gains traffic every turn it walks anywhere, and wears a trail; a tile one small clan
  walks to alone stays below `on`; a tile on two clans' routes wears one; a trail falls below `off` and
  goes; a used scout route keeps its trail with no other traffic.
- Desert costs 2 to enter.

## Done (the targets, spec item 6)

On the seven retro maps, from `sweep --judge`:

- pre-games by centres 7 of 7, and the run's health otherwise as main's (no panics, no digest mismatch);
- `flips` at least 80% below `build-1-main.txt`'s;
- `homes_trailed` ≥ 0.8, `held_trailed` ≤ 0.35, `route_trailed` ≥ 0.5, at the pick and at +200;
- `markets` at the pick at least main's.

Save the final run as `relay/specs/102/build-1-judge.txt` and a table of what you tried (variant, the
measures, centres) as `relay/specs/102/build-1-tuning.md`. If a target can't be met without breaking
another, keep the pre-game and say which you missed and by how much; don't tune clans changing civ or
lead changes: those are the canvas's, on top of build-2's rules (`probe.md`: nearer festivals cut
today's joining by three quarters, and that's expected). `./check.sh` passes.

Build-2 relies on: `Clan.market_turns`, `Tile.traffic` and the trail rule settled, `SAVE_VERSION` 3,
the `land` group in `Scores` to add its own measures beside.
