# #156 build-2 (easel): a turn that scales

Towns put many clans on few tiles, and the engine must carry them: per-clan work must not grow with the number
of clans (`clans.md`, Scale). This step makes four refactors, each digest-safe: `stats::digest` stays exactly
build-1's on every seed and turn. Read `spec.md` item 5 and decision 8, and `probe.md`'s "Where a large turn
goes": the step timings, the scouts' bound and why a bound of `DAY` a step moved the digest.

The code: `core/src/markets.rs` (`worths`, `choose`, `go`, `trade`, `people`, `merge`, `in_reach`),
`core/src/land.rs` (`cost_map`, `cost_map_by`, `path`, `path_by`, `step_cost`, `grid`),
`core/src/units/ai.rs` (`post`), `core/src/units/scout.rs` (`round_plan`, `rich_ground`, `keep_route`),
`core/src/units/search.rs` (`walk_costs`, `route_plan`), `core/src/units/line.rs` (`path_between`),
`core/src/homes.rs` (build-1's `homes`), `web/src/lib.rs` (the view's markets), `cli/src/bench.rs`.

## What to build

1. **The festival search once per home tile.** In `go`, the costs from a home tile to the festivals in reach
   (`worths`' cost map) are worked out once a turn for each home tile, and every clan living there reads them
   (`homes::homes`). The worth itself stays per clan: it reads the clan's basket, walkers and civ. Today no
   home is shared, so the saving has to come from the search too:
   - **Flat scratch arrays, not a map.** `cost_map_by` keeps its costs in a `BTreeMap`. Spec-review measured
     (`spec-review.md`, `search-flat.diff`): the festival search on reused flat arrays (a cost per tile and a
     list of touched tiles to reset, kept between calls) took the bench from 27.9 to 22.0 ms a turn on top of
     the scouts' bound, against main's 64.3 in the same session: **0.34 times**, digest equal on large 8 and
     small 3. That is where this step's saving is. Keep the scratch in a struct the turn owns or passes, not a
     `thread_local`, and make it `cost_map_by`'s own store if that serves every caller.
   - **Stop early.** The search may stop once every festival that could be in reach is settled, bounded by the
     cheapest step any tile can cost (a trail on flat land, `TRAIL.flat × DAY`), not by `DAY`: `probe.md` shows
     a `DAY` bound moves the digest. A festival is settled when it is popped, as `cost_map_by` settles tiles.
     Measured, this exit buys nothing on its own (27.9 ms with or without it), since most homes have a candidate
     festival the search reaches only near `most`. Keep it only if it costs nothing to read.
   - **One search, not two.** The chosen festival's route comes from the same search's predecessors, if they
     give exactly the path and cost `land::path` gives with its tie-breaks. Otherwise keep `path` and say why
     in `scale.md`. `path` was 2.5 of 15.9 ms of `go` in `probe.md`.
   - Whatever structure the search keeps its costs in, the costs and tie-breaks must be `cost_map_by`'s: the
     cheapest grid cost, then the lowest tile, with the first predecessor at an equal grid cost.
2. **Clans indexed by market once a turn.** `markets::by_market(w) -> BTreeMap<u32, Vec<u32>>`: each market's
   clans in id order. `trade`, `people`, `merge` and the view's `markets.clans` read it instead of scanning
   every clan for each market. Build it where each step starts, since `go` and `merge` change it.
3. **The AI's paths cached per turn.** `ai::post`'s `path_between` to each rival capital and to the farthest
   clan go through a cache that lives for one turn (keyed by from and to, cleared at the turn's start,
   `#[serde(skip)]` and never read across turns, so the save and the digest don't see it).
4. **The scouts' search bounded by their labour.** `scout::round_plan` builds its cost map out to the most
   its labour could walk there and back (`labour ÷ (2 × people)`, plus a day of slack), not over the whole map.
   It skips any tile whose walk costs the labour it has, so the plan is the same. `scout-bound.diff` is the
   change as the spec step ran it: 31.5 → 0.3 ms a turn on large 8, digest equal. Build it properly, with a
   doc line saying why the bound gives the same plan. Alone it takes the bench to 0.43 of main's
   (spec-review: 27.9 against 64.3 ms), so the flat search above is what reaches 0.35.

`markets.rs` is 428 lines and `land.rs` 353, so both may grow. If the search grows past about 80 lines, put it
in `core/src/reach.rs`.

## Tests

- In `core/tests/towns.rs` (build-1's): two clans sharing a home on a staged map choose from the same
  festival costs, and each gets the route `land::path` would give.
- A test that the early-exit search gives the same costs to every festival as `cost_map` on a map with trails,
  including a festival further than `most ÷ DAY` hex steps reached along a trail.
- `by_market` lists each market's clans in id order.

## Proof and record

- `engine/check.sh` green, with the pins in `cli/tests/save.rs` unchanged from build-1's: the digest moves
  nowhere.
- `sansfiat digest --seed 8 --size large --turns 450 --state` and `--seed 3 --turns 400 --state` equal
  build-1's at every turn. `sansfiat sweep --judge --json` equal to build-1's in every map's blocks.
- **The bench.** `sansfiat bench --size large 8 --after 50`, two runs each, on this branch, on build-1's commit
  and on main (a worktree), in the same session. **Done is at most 0.35 times main's ms a turn.** The spec
  step reached 0.35 with the scouts' bound and a crude exit that moved the digest; spec-review reached 0.34
  with the scouts' bound and the flat search, digest equal. Time the steps with a
  scratch timer round each `TURN` step (don't commit it) and give the table before and after.
- Write it all to `relay/specs/156/scale.md`: the step table before and after, the bench, the digests.

## What later steps rely on

The per-home-tile search: build-4's home rule prices a farmer's festival route from a candidate home with it,
so give it a function a caller can ask for the costs from one tile to the festivals in reach
(`markets::festival_costs(w, tile)` or the like). Also `markets::by_market` and the bench numbers in `scale.md`.
Build-4 must keep the large turn under 0.45 times main's with towns on.

## Done

`check.sh` green with build-1's pins, digests equal, the bench at or under 0.35 times main's, and `scale.md`
written.
