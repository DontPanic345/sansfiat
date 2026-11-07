# #153 build-1 (easel): the flat trail

A trail on plains or coast brings the tile to three quarters of its bare cost. Read `spec.md`
items 1–6 and `probe.md`, where the spec step measured it. The code: `core/src/data.rs`
(`TrailRules`, `TRAIL`), `core/src/map.rs` (`roll_trails`, `Tile`), `core/src/maptext.rs`
(`parse`, where it clears `trail_cost`), `core/src/stage.rs` (`plain`, `paint`),
`core/src/land.rs` (`step_cost`, `trailable`, `tend`), `core/src/units/scout.rs`
(`tile_cost`, `needs_trail`: read them to confirm the scout ignores flat land),
`core/tests/cases.rs`, `cli/tests/save.rs`, `www/play/scout.mjs`, `core/src/script.rs`.

## What to build

1. **`data.rs`**: `TrailRules` gains `flat: f64` and `TRAIL.flat` is `0.75`, with a note: the
   share of its bare cost that a trail brings plains and coast to (M4 answer 4); the same on
   every flat tile, so nothing about it is hidden. Leave `TRAIL.work` as it is: the scout
   builds no flat trail (spec decision 3).
2. **`map::flat_cost(terrain: usize) -> Option<f64>`**: `Some(move_cost × TRAIL.flat)` for
   `PLAINS` and `COAST`, else `None`. Call it in `roll_trails` for plains and coast tiles
   without drawing from `sub`, so every other roll stays where it was. Call it in `map::parse`
   where it clears `trail_cost` (set `trail_cost = map::flat_cost(terrain)` once the terrain is
   known), and in `Stage::plain` (its doc says the plains carry the flat trail cost and no
   hidden costs). `Stage::paint` stays literal.
3. **Check, don't change**: `step_cost` already takes the lower of bare and `trail_cost` on a
   trailed tile, and `tend` wears and clears trails on every terrain but mountain. Confirm the
   scout's `tile_cost` prices a flat tile the same known and possible, so no flat tile becomes
   a candidate (`route_plan`'s candidates on a retro errand contain no plains or coast tile:
   assert it in the test of item 4). The hidden-costs layer (`www/map/layers/hidden-costs.js`)
   already skips flat land.
4. **A stage test** in `core/tests/wear.rs` (or `land.rs`): on a `Stage::flat` or `plain`
   map, a plains tile and a coast tile whose `traffic` is set to `TRAIL.on` get a trail at
   `tend` and then cost `0.75` to enter (`land::step_cost`), and a marsh tile keeps `3.0`. A
   clan's walk over five worn plains tiles costs a quarter less than over five bare ones.
   Also assert that on one retro errand (small 2, the first civ's capital to the nearest rival
   capital) the plan's candidates hold no flat tile.
5. **The fallout.** Run `cargo test`. The pre-game on every retro map changes, so tests pinned
   to a retro map's numbers move:
   - `SAVE_VERSION` bumps by one, and the pins in `cli/tests/save.rs` move.
   - **An assertion of a rule may not change.** An assertion whose number comes from the map
     (a tile id, a count at the pick, a turn a route is found) may, with a comment saying the
     number follows the map. List every assertion you changed, and why, in
     `relay/specs/153/flat.md`.
   - **The red cases** in `core/tests/cases.rs` keep failing for their own reason. For each of
     `a_scout_searches_a_range_by_best_case` and `a_settled_route_has_no_cheaper_trail`: if it
     still fails, update the `expected` text to the new failure's start and the doc comment's
     numbers. If it no longer fails on its maps, run the same check over all 14 errands (seven
     maps, both targets), pin the first failure, and update the doc. If none of the 14 fails:
     the search-order case keeps its body and becomes a plain green test (build-2 would have
     removed the attribute anyway; say so in `flat.md`), and the bridge case becomes a green
     guard with its doc saying so. The green cases must stay green.
   - `www/play/scout.mjs`'s `TARGET` (58) and the `Lead` target are "the first forest or hills
     tile seven steps out" on small 2. If the tile moved, update `TARGET` and its comment, and
     check the route is still found.
6. **`docs/design/scouting.md`**, Hidden costs: replace "Flat land gains nothing from a trail."
   with "A trail brings flat land down a little; walking wears one there, and a scout builds
   trails only on rough ground and mountains." Change nothing else in the file.

## Proof and record

- `engine/check.sh` green (it runs `cargo test`, clippy, fmt, the cross-build, `check.mjs` and
  `www/play/all.mjs`).
- `sansfiat sweep --judge --json --floor` on main (a worktree at the branch's base:
  `git worktree add /tmp/main origin/main`) and on the branch, then
  `sansfiat judge --compare main.json branch.json`. Put the totals of the standing, doubt,
  decisions, motion and land groups, and `health.pregame_by_centres`, in `flat.md`, beside
  `probe.md`'s 0.75 column. They should match it closely: the probe ran the same rule. Say
  what differs.
- `sansfiat sweep --size all` (the pre-game sweep) on the branch: the endings line per size in
  `flat.md`.

## What later steps rely on

Plains and coast carry `trail_cost = bare × TRAIL.flat` on every generated, parsed and plain
map; the scout neither reveals nor builds them; every check is green on the new maps; and the
red cases' texts are the new maps' texts.

## Done

`engine/check.sh` green; `flat.md` written; `engine/CLAUDE.md` changed only if a line in it is
now wrong (`map.rs`'s line names "hidden trail costs": add "and the flat trail cost" if it
reads wrong).
