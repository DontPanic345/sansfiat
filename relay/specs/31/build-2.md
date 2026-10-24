# Build 2: rivers, crossing, floodplains and deltas

Second of five builds for #31. Read `spec.md` (decisions 5–8), `docs/design/map.md` (Rivers,
Also), and the earlier steps' results. Work in `engine/`; checks: `cargo test`,
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `./build-web.sh`. This is the
step with the most judgment: rivers that run to the sea, and a crossing cost that touches
every path the clans walk. merge4's `carveRivers` follows the steepest drop on noisy height
and wanders round pits; **don't port it**, `map.md` says why.

## 1. Rivers (`map.rs`, a function `carve_rivers`, called after coast/`lake` and before caps)

Start values in `MAP` (per 817 tiles): `rivers` 16 attempts (floor 3), `river_min` 6 (steps
a source is from water), `river_wiggle` 0.25.

- **Distance field.** Multi-source BFS from every `SEA` tile (ocean and lake) over tiles a
  river may enter: land tiles that are not mountains (`LAND` terrains incl. desert, hills,
  coast, marsh). `dist[t]` = steps to the nearest water. Tiles with no entry (walled off from
  all water) have no dist and can't carry a river.
- **Sources.** Hills tiles and tiles beside a mountain with `dist ≥ river_min`, picked without
  replacement from the main rng; at most `rivers` (scaled) attempts. A source already on a
  river is skipped.
- **Walk.** From the source, repeatedly choose the next tile among the unvisited neighbours
  with `dist ≤ dist(here)`: lower-distance neighbours are chosen with probability
  `1 − river_wiggle`, otherwise (if any) an equal-distance sidestep; ties between candidates
  go to the lower noisy height `h` (keep `h` in a working array), then the lower index. Stop
  when the next tile is water (the river ends there: `flow = Some(water tile)`) or is already
  a river tile (it **joins**: `flow = Some(that tile)`). By construction it never revisits a
  tile and can't double back; a test proves it.
  Every attempt ends in water or a join, so every kept river is a success; no river is
  dropped.
- **Fields.** Along the path set `flow` to the next tile, and recompute `river` (upstream
  tile count incl. self) for the whole network after all rivers are carved (topological: sum
  into the tile downstream; rivers are a forest of trees pointing to the sea, so a simple
  accumulation by processing sources first / sorting by decreasing `dist` works).
- **Lakes fed and drained.** After the rivers: a lake (inland water body) that no river
  reaches gets, with probability ½ each, an outflow: a river that starts on a land tile
  beside the lake and runs to the *ocean*: BFS from ocean only for this one, since the lake
  is the source. Where the outflow can't reach the ocean (walled in) it gets none. Rivers
  that run into a lake feed it. Inland seas count as lakes here.
- Rivers never cross a mountain (not in the BFS) and a river tile keeps its terrain (a river
  on a coast tile stays coast; floodplains below change this).

## 2. Fertility

`finish_caps` (build-1) adds a river's food: `FEATURES.river` = fish +5, grain +1 added to
the terrain's caps before the fertility multiplier (as merge4). Test: over 20 seeds on each
size, the mean total cap of a river tile (non-coast, non-floodplain) is above the mean total
cap of its non-river land neighbours.

## 3. Floodplains and deltas

Start values in `MAP`: `big_river` 8, `flood_reach` 6, `delta_river` 12,
`flood_fertility` 2.2.

- **Path to the mouth** of a river tile = steps following `flow` until the next tile is water.
- **Floodplain**: a river tile with `river ≥ big_river` whose path to the mouth ≤
  `flood_reach`, and every non-mountain land neighbour of those tiles (not already marsh).
  `flood = true`, fertility set to `flood_fertility` (above the normal maximum, so floodplains
  are the richest land), terrain becomes PLAINS if it was COAST or DESERT (so it can grow
  grain); other terrain stays. The mouth ends of lake rivers count too (the last stretch near
  *its water*), but a delta is ocean-only.
- **Delta**: the mouth tile of a river with `river ≥ delta_river` whose water is the *ocean*
  (`!lake`), plus those of its land neighbours that touch the ocean: terrain `MARSH`,
  `flood = true`, fertility `flood_fertility`. Also keep their `river`/`flow` if they had it.
- Add to `ASCII`: `r` river, `z` floodplain, `%` marsh.

## 4. Crossing (`land.rs` and callers)

- `step_cost(w, from, to) -> Option<f64>` now takes the tile left: the terrain/trail cost of
  `to` as today, plus `RIVER.cross` (new `data::RIVER`, starting value 2.0) when `to` is a
  river tile (`river > 0`), the tile has no trail, and the move is not along the course:
  `from.flow == Some(to)` or `to.flow == Some(from)`. A river-to-river step that is not along a
  single course (two tributaries side by side) costs the extra; a confluence step along a
  `flow` is free.
- Add `land::passable(w, i) -> bool` (`step_cost` without the river: terrain can be entered)
  and use it where code asked `step_cost(w, i).is_some()` (`markets.rs`, `round.rs`,
  `clans.rs`, `pregame.rs`). `cost_map` uses `step_cost(w, i, j)`.
- A trail on a river tile bridges it: the check is `t.trail`, which both worn trails (the
  `hold` step) and built trails set. A trail on a river tile at the *course* does nothing
  more. Don't change what trails cost otherwise.
- Cost symmetry: a leg's cost can differ by direction only when an endpoint is a river tile
  (entering costs, leaving doesn't). Make sure rounds and `Legs` still compute `leg(a, b)` and
  `leg(b, a)` consistently with how they used to (look at `round.rs`: if it caches one
  direction and doubles it, the doubling is still the rule: say what you did); add a test
  that a round and its reverse cost the same total.
- Pre-game ending: `pregame::land_cover` uses passable land; rivers don't block it.

## Tests (`core/tests/rivers.rs`, `land.rs`)

- Rivers: on every size × 15 seeds: ≥ 1 river per size with `river ≥ 5`; every river tile's
  chain of `flow` ends in a water tile with no repeated tile; along each chain `dist` to
  water never rises by a step (recompute dist in the test); a river never touches mountain;
  at least one pair of rivers joins (some tile has two upstream tiles) on large maps; lakes:
  across 20 seeds at least one lake is fed and one drained.
- Floodplains/deltas: floodplain tiles' min fertility ≥ every non-flood tile's fertility;
  every delta tile is MARSH next to ocean-`SEA` (not a lake); a delta's river has
  `river ≥ delta_river`; over all sizes at least a few seeds have a floodplain and a delta (say
  how many).
- Movement (hand-built world on `flat()`): entering a river tile from the bank costs
  `1 + RIVER.cross`; along the course costs 1; from the downstream tile back up to it costs 1
  (the `to.flow == from` case); with `trail = true` it costs the trail cost; a path across a
  one-tile-wide river costs more than the same path over plains, and a path along the river is
  not dearer; `cost_map` out of a tile reflects it.
- A clan's round or a market's reach changes with a trail on the river tile (construct it).
- All earlier tests green. List every test you changed and why.

## Report

`sansfiat map --size large --seed 3` (and a small one) shown in your result as ASCII, plus
how many rivers, floodplain tiles and delta tiles per size over seeds 1..=10. Run the small
sweep for seeds 1..=10 and put the table in your result (info for build-4).

Commit when checks pass: "#31 build-2: rivers to the sea, crossing, floodplains and deltas".
