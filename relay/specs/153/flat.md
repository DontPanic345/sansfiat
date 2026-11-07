# #153 build-1: the flat trail, as built

Plains and coast carry `trail_cost = bare × TRAIL.flat` (0.75) on every generated, parsed and
plain map. The rule is `land::flat_cost`; `roll_trails`, `maptext::parse`, `Stage::plain` and
`Stage::flat` call it. `Stage::paint` stays literal.

## Status

`check.sh` is **not** green. Everything up to and including `play/all.mjs` passes: `cargo fmt`,
clippy, `cargo test` (all binaries), `build-web.sh`, `cross-build.sh` and `www/check.mjs`. Seven
of the touch-point scripts in `www/play/` fail on the new maps (listed below, with the cause found).
They were tuned to main's pick and were not retuned here.

## Decisions taken while building (for the owner)

- **`trailable` means buildable.** It now requires rough ground or a mountain (`TRAIL.work`) as
  well as a hidden cost. A flat tile has a hidden cost but no trail can be built on it by labour.
  This keeps the scout's `tile_cost`, `may_trail` and `needs_trail` flat-blind with no separate
  helper: a flat tile prices the same known and possible, so none is a route candidate (the
  retro-errand test asserts it on small 2). Used by `build_trail` and the mountain-pass event.
- **`land::flat_cost` lives in `land.rs`**, as the brief says for the helper; the brief's
  `map::flat_cost` calls are `land::flat_cost` (map.rs has no such item).
- **`Stage::flat` carries the flat cost too.** It is a plain map built from `maptext::bare`, which
  clears trail costs; without it `Stage::parse(ascii)` and `Stage::flat` disagreed in digest.
- **`SAVE_VERSION` 8 → 9**, and `www/check.mjs`'s version pin with it.

## Assertions changed, and why

Rule assertions changed because the rule changed (plains and coast now carry a trail cost):

| file | assertion | change |
|---|---|---|
| `core/tests/map.rs` `hidden_costs_follow_the_terrain` | "every terrain but forest, hills, mountain has no `trail_cost`" | plains and coast expect `land::flat_cost` |
| `core/tests/stage.rs` `plain_makes_the_land_bare…` | "no `trail_cost`" | land carries `flat_cost(terrain)`, sea none |
| `core/tests/rivers.rs` `a_trail_on_the_river…` | a trailed ford walks `2.0`; reach `2.0`; the course step `1.0` | `1.0 + TRAIL.flat`, `1.0 + TRAIL.flat`, `TRAIL.flat`: the ford and the course are plains |
| `core/tests/land.rs` `step_costs` | "a trail with no hidden cost gains nothing" | setup now clears the flat cost first (`trail_cost = None`), so the assertion still tests a trail with no hidden cost |
| `cli/tests/save.rs`, `core/tests/save.rs` | version 8 | version 9 |
| `cli/tests/save.rs` | the four digests and the led state digest | new hashes (see below) |

Assertions whose number comes from the map, moved with comments:

| file | assertion | change |
|---|---|---|
| `core/tests/units.rs` `the_control_run_has_no_army_and_never_levies` | `units.is_empty()` at turn 400 | only scouts: founding raises a scout with or without an army, and on this map the control's scout lives to turn 400 (on main it did not). The check is that no camp or guard is raised. |
| `core/tests/notes.rs` `a_led_scouts_reasons_follow_its_route` | the scout exists at turn 60 | reads the scout's notes each turn it lives. On seed 2 the led scout now forages its own tile first, spends its labour there, never walks, and starves at turn 9 (route set, "has its route: cost 14.8", then hunger). The route-set and has-its-route notes still hold. This is the feeding problem #153 build-3 fixes. |
| `core/tests/cases.rs` `a_scout_searches_a_range_by_best_case` | `expected = "small 9 turn"` | `"small 9 turn 22: the scout revealed mountain 131"`: small 9 to the rival now fails at turn 22 (131 at 32.0 while 178 at 28.0 is open). Doc numbers updated. Medium 8 is not reached. |
| `core/tests/cases.rs` `a_settled_route_has_no_cheaper_trail` | `expected = "small 2 to the rival turn"` | `"small 3 to Lead's target turn 52: the settled route costs 18.8 known and 17.8 possible"`: small 2 to the rival now settles with nothing cheaper; small 3 to Lead's target fails at turn 52, the step into 436 at 2.0 where known it costs 4.0. Doc updated. |
| `cli/tests/play.rs` `the_view_says_whether_a_gift_reaches_each_clan_in_reach` | seed 1, `end 5` | seed 16, `end 25`. On seed 1 no unaligned clan is in reach at any turn 1–30 (the flat trail moved the pick from 314 to 271, and the reach differs). Seed 16 at turn 25 is the first seed and turn that shows a clan in reach trading at another market. The comment says so. |
| `cli/tests/save.rs` digests | turns 334, 256, 264 (seeds 1–3), led at 256 | turns 290, 279, 308 and led at 279: the picks move (seed 1 271, seed 2 260, seed 3 289 as `play` reports them). At turn 256 seed 2 is before its pick here, so its led digest equalled the plain one; 279 is 20 turns past it. |
| `cli/tests/save.rs` `the_led_game_state_digest…` | seed 2 state digest at 256 | at 279 |

`cases.rs`'s search-order case keeps its body and its `should_panic`: it is still red for its own
reason (small 9 turn 22), so build-2 still removes the attribute. The bridge case is still red for
its own reason too. Neither needed the "green guard" fallback.

`www/play/scout.mjs`: `TARGET` (58) still is the first forest or hills tile seven steps out on
small 2, and the scout route is still found; the script passes. No change.

## Proof

Judge (`sansfiat sweep --judge --json --floor`, main at `b377337` and the branch), totals from
`sansfiat judge --compare main.json branch.json`. The probe's 0.75 column is in `probe.md`.

| group | metric | main | branch | probe 0.75 |
|---|---|---|---|---|
| standing | aligned share at +200 | 0.2086 | 0.2364 | 0.236 |
| standing | top share at +200 | 0.1055 | 0.1076 | 0.108 |
| doubt | lead changes (maps) | 10 (4) | 3 (1) | 3 (1) |
| doubt | spread at +200 | 0.0705 | 0.0583 | 0.058 |
| decisions | AI orders per 100 turns | 13.32 | 9.04 (−32%) | 9.0 |
| decisions | claims per 100 turns | 11.74 | 6.90 | 6.9 |
| motion | clans joined | 123 | 90 | 90 |
| motion | people changing hands | 569.5 | 460.8 | 461 |
| motion | land tiles changing hands | 2448 | 1612 | 1612 |
| land | trading share at the pick | 0.8979 | 0.9292 | 0.929 |
| land | trading share at +200 | 0.8968 | 0.9250 | 0.925 |
| land | festival changes per 100 | 0.6546 | 0.3899 | 0.39 |
| land | markets at the pick | 218 | 211 | 211 |
| land | held tiles a civ at +200 | 21.33 | 21.76 | 21.8 |
| economy | store mean | 17.11 | 16.06 | 16.1 |
| health | pregame_by_centres | 7 | 7 | 7 of 7 |

The branch matches the probe to the probe's own precision: the probe ran the same rule. The
differences are in metrics the probe did not print: `doubt.swings` 7 → 16 and the trailed-share
groups (`land.route_trailed`, `land.homes_trailed`, `worked_trailed`) moved by under 0.03.

Pre-game sweep (`sansfiat sweep --size all`, seeds 1–10 per size), the endings lines:

- small: 10 of 10 ended on their own, 0 needed the top-up, median end turn 262
- medium: 10 of 10 ended on their own, 0 needed the top-up, median end turn 351
- large: 9 of 10 ended on their own, 0 needed the top-up, 1 ended on the limit with three centres
  already, median end turn 383

Every pre-game ends by centres, as the probe found. People levelled off on all 30 seeds.

## Open: seven page scripts fail (`www/play/all.mjs`)

`army`, `camp`, `civ`, `feed`, `game`, `layout` and `select` fail. Their setup is `seed=2` (or 4, 9)
and `raiseArmy`, which raises three warriors at the pick and three guards. On the branch, seed 2's
leader has a store of 10 food at the pick, and a one-person camp raised there drains the civ's
store to nothing by turn 11 (traced in the page). So the three-warrior army never forms, and the
scripts' `find(camp)` finds nothing. Fixing it means choosing seeds (or turns) for each script
and checking each script's own assertions, which were tied to main's pick. That is the next
attempt's work. The page scripts were not retuned in this step.
