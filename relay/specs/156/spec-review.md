# #156 spec-review: what was run

All on the branch at b23b14b, where the engine is main's (929b84a, Fields), release build, on the Actions
runner. Nothing here is committed to `engine/`.

## The scale target (build-2)

`sansfiat bench --size large 8 --after 50`, mean ms a turn past the pick:

| Engine | ms a turn | Of main | Digest (large 8 t450, small 3 t400, `--state`) |
|---|---|---|---|
| main | 64.3 | 1.00 | ba98dd5e9639e61c, 4d2a65dba173a448 |
| scouts' bound (`scout-bound.diff`) | 27.9, 27.8 | 0.43 | equal |
| + early exit bounded by the cheapest step (`TRAIL.flat × DAY`), `BTreeMap` | 27.9 | 0.43 | equal |
| + the same search on flat scratch arrays (`search-flat.diff`) | 22.0, 22.0 | 0.34 | equal |

Steps with the scouts' bound and the early exit: go 18.5, eat 4.4, move 2.7, units 0.7, everything else under
0.6. With the flat search: go 12.4.

So the digest-safe early exit buys nothing: most homes have a festival within the cheapest-step radius that
the search reaches only near `most`, so it runs to the bound anyway. The saving is the data structure. The
`BTreeMap` in `cost_map_by` costs more than the search itself. 0.35 is reachable digest-safe, and build-2's
brief now says where the saving is. `path` to the chosen festival is still a `BTreeMap` search in that
diff. Taking it from the same search's predecessors would cut more.

## Food brought (bound 4)

At turn 450, every clan with a festival on small 9, medium 8 and small 2 (34 + 84, 41 + 252, 19 + 125
farmers + gatherers) brings `trade`'s cap, `MARKET.carry` (0.2) a person. Brought per person is 0.200 for
every clan, so "town clans bring 1.1 times" can't pass, whatever towns do.

What the clearing swaps does move (half of Σ |after − brought| per person; every clan-turn at a festival
during play to turn 500):

| | small 9 | medium 8 |
|---|---|---|
| farmers | 0.042 (n 7,743) | 0.047 (n 8,572) |
| gatherers | 0.065 (n 20,826) | 0.074 (n 52,291) |
| festival of 2–3 clans | 0.033 | 0.043 |
| 4–6 | 0.053 | 0.056 |
| 7–10 | 0.058 | 0.072 |
| 11+ | 0.077 | 0.075 |

The nutrition a person gains in the clearing (the trade leaning's measure) is noisy and negative for
farmers (−0.009, −0.017), so it is a poor score. Bound 4 is now on **traded**, what the clearing swaps.
`World.traded` is `#[serde(skip)]`, derived and read only by the judge.

## Settled farmers' home moves (bound 8)

`sweep --judge --json` on main (2 min 15 s): 52,054 settled-farmer clan-turns with 16 home moves (0.031 per
100), and 326,661 gatherer clan-turns with 399 (0.122 per 100). Under half the gatherers' rate, the seven
maps allow about 32 settled-farmer home moves in all. Bound 1 asks for at least 10 towns. Ten towns of three
clans, mostly farmers who live three or more steps from their festival (`probe.md`), take 60 to 100 steps.
So bound 8 as written ("a farmer's move into a town counts") can't hold beside bound 1. It now counts home
moves less **town steps** (a step that brings the home a tile nearer its festival's tile), and town clans'
moves off their town. Town steps are reported. Hopping is still caught: a step away from the festival is not
a town step, and nor is a step off the town.

## Read in the code

- `harvest` zeroes `Tile.field` at its start, so `fields::room` as the brief first read it ("`Tile.field` less
  the clan's own `sown`") would see 0 in `reap` and give every clan the whole ground. Build-1 now moves that
  reset into pass two.
- `reap` reads only tile stocks, fallow and ground, never `traffic`, `take` or `field` as harvest writes them,
  so reaping every clan before any take gives today's numbers while gathering stays exclusive.
- `StageSpec`, which the page's `Sim.stage` reads, places only civs and armies, so `towns.mjs` couldn't stage
  a town. Build-3 adds `clans: [{home, tiles, people}]`.
