# #156 build-2: a turn that scales

All runs on the runner, release builds, in one session: `main` (a worktree at dc26b06), build-1 (4c8424b)
and this branch. Digests and the sweep are build-1's; the bench is `sansfiat bench --size large 8 --after 50`.

## Result

| Engine | ms a turn (run 1, run 2) | Of main |
|---|---|---|
| main | 64.2, 64.5 | 1.00 |
| build-1 | 76.5, 76.1 | 1.19 |
| build-2 (this branch) | 20.8, 20.8 | **0.32** |

Done is at most 0.35 times main's ms a turn: met (0.32).

## Step table (ms a turn past the pick, large 8, mean of 50 turns; scratch timer round each `TURN` step, not committed)

| Step | main | build-1 | build-2 |
|---|---|---|---|
| units (scouts) | 36.7 | 36.8 | 0.7 |
| go | 18.7 | 18.6 | 9.9 |
| eat | 4.4 | 9.5 | 1.8 |
| move | 2.7 | 8.3 | 5.9 |
| harvest | 0.6 | 1.6 | 1.5 |
| trade, people, ai, lean, the rest | < 0.5 each | < 0.5 each | < 0.3 each |
| **total** | **64.2** | **76.5** | **20.8** |

## What each change did

1. **Scouts' bound** (`scout::round_plan`): the cost map runs out to `labour ÷ (2 × people)` plus a day of
   slack, not the whole map. A walk costs twice its path (`actors::walk_cost`), so a tile past that bound costs
   more labour than the scout has, and the plan skips it either way. Units 36.8 → 0.7 ms.
2. **Flat search** (`core/src/reach.rs`): `Scratch` holds a grid cost and predecessor per tile, the touched
   tiles to reset, a heap and the cheapest step on the map, and is made once a turn in `go`. It is owned by
   the turn, not a `thread_local`, and is not a `World` field. `cost_map_by` keeps its `BTreeMap` for its other
   callers; the saving is in the festival search only.
3. **Per home tile** (`markets::go`): `festival_costs(w, scratch, home)` runs once for each home tile a turn
   (a `BTreeMap` cache in `go`), and every clan on it reads the same festivals through `worths_from`. The worth
   stays per clan. Today no home is shared, so this saves nothing on the bench by itself.
4. **Early exit**: the search stops once every candidate festival that could be in reach is settled. A
   candidate `n` hex steps off costs at least `n × cheapest`, and `cheapest` is the least a tile can cost to
   enter: its terrain's cost or its hidden trail cost, the minimum over the map (computed in `Scratch::new`).
   On large 8 that minimum is 1.0 = `TRAIL.flat × DAY`, the figure the brief gives. As spec-review found, the
   exit saves little, since most searches run to `most` anyway.
5. **One search, not two**: the chosen festival's route is read from the same search's predecessors. They give
   exactly `land::path`'s tiles and cost: the same passability, the same grid costs and tie-breaks, and every
   tile on the chain is settled before the festival is. `go` still calls `land::path` when the festival is not
   among the search's reads (a civ's capital market beyond reach). `path` took 2.5 ms of `go` in `probe.md`.
6. **Clans by market** (`markets::by_market`): `trade`, `people`, `merge` and the view's `markets.clans` read it.
   `merge` removes the clans it merges from its copy. `civs::levy`, which `trade` calls for each market, still
   scans every clan; that is outside this step's list and is the next per-market scan to index.
7. **The AI's paths** (`units::ai::post`): `ai` keeps one `Paths` map for its turn and reads each rival's and
   farthest clan's path through it. It is a local of `ai`, not a `World` field: it cannot outlive the turn, is not
   saved, and the save and digest never see it. `walkable` depends only on terrain and trails, which `ai` does
   not change, so a path read early in the turn is the one read later. The cache saves about 0.1 ms.

Two changes outside the brief's list, made to reach 0.35:

8. **`sites`** (`clans::sites`, the move and dividing steps): build-1 built `homes::homes(w)` for each call, a
   `BTreeMap` with a `Vec` per home, about 90 µs a call over 10,000 calls. It now marks the homes in a flag array,
   and the search's `seen.contains` is a visited array, the same tiles in the same order. Eat 4.4 → 1.8 ms
   (build-1: 9.5).
9. **`fields::room`** reads the clans that sowed last turn from `World.sowers`, a derived, `#[serde(skip)]` list
   that `clans::harvest` rebuilds at its end (the only writer of `Clan.sown`). It removes a scan of every clan on
   each call. The move step still costs 5.9 ms against main's 2.7: the value calls and the reaps they make
   (`value`, `site_value`, `fields::room`) are build-1's extra work, the next target.

## Digests (`sansfiat digest --state`, build-1's pins unchanged, every turn)

| Run | Turn | build-1 | build-2 |
|---|---|---|---|
| large 8, 450 turns | 0 to 450 | identical at every turn | `66199895119607c8` at 450 |
| small 3, 400 turns | 0 to 400 | identical at every turn | `f42bd739cac676a8` at 400 |

`diff` of the two `--state` outputs is empty for both maps (`digest --seed 8 --size large --turns 450 --state`
and `digest --seed 3 --size small --turns 400 --state`).

## Sweep

`sweep --judge --json` against build-1's report: identical in every block; the only difference is the
`seconds` wall-clock field, which is the run's own timer and differs with the load on the runner (192.2 s for build-1's run, 61.5 s for this one).

## Tests

- `core/tests/towns.rs`: `clans_on_one_home_read_one_festival_search_and_walk_the_land_path` (two clans on
  one home read one search's costs and walk `land::path`'s route), and `by_market_lists_each_markets_clans_in_id_order`.
- `core/tests/reach.rs`: `the_festival_search_gives_cost_map_costs_and_land_paths`, on a radius-10 map with
  hills, forest, trails east and north-west; festivals at one step, at 8 steps along a trail (past `most ÷ DAY`
  = 7 steps), and two more. For two homes it checks that every festival `cost_map` puts in reach gets its cost,
  and that nothing else is returned, and that each route equals `land::path`.
- `engine/check.sh`: see the record in the summary.
