# #102 review: what I played and measured

## The judge

`sweep --judge --solo` on the merged branch (`review-judge.txt`) matches build-4's `canvas-judge.txt` line for line.

| | main | build-3 | final | no-claims control |
|---|---|---|---|---|
| solo gain, mean (clans, land) | – | -0.0012 | **+0.0170** (+0.0169, +0.0001) | 0 by definition |
| civs above +0.005 | – | 6 of 21 | **14 of 21** | – |
| lead changes (maps) | 7 (4) | 7 (3) | **12 (5)** | 3 (3) |
| camps lost / warriors starved | 3 / 19 | 2 / 16 | **18 / 53** | – |
| people vs `--no-army` | 0.984 | 0.976 | 0.982 | – |
| flips per 100 clan-turns | 4.35 | 0.08 | 0.07 | – |
| markets at the pick | 142 | 218 | 218 | – |
| homes / worked / route trailed | – | 0.84 / 0.31 / 0.97 | 0.85 / 0.31 / 0.97 | – |

Unaligned people on held land at +200 is 0.000 on every map: a held home's clan joins within about 11 turns, so the land half of Standing counts nobody. The lowest Standing of any AI civ at +200 is 0.010.

## Played: the wide claim against the ring (`sansfiat play` and `sansfiat judge`)

**Small seed 2, leading Red.** The founding guard (3) starves by 7 turns after the pick, even with its clans feeding it: the store is at 0, and so are the clans'. The claim falls to the capital whether it is the stand-in (24 tiles) or everything within 6 steps (51). At +100 held is 1 and Standing is 3.7% either way. Red's solo gain on this map is also negative (-0.0048). Seed 2 can't show width costing anything, so I played seed 9.

**Small seed 9, leading Blue (civ 486), +100 turns:**

| game | claim | held at +100 | guard | Standing at +100 |
|---|---|---|---|---|
| stand-in ring, 3 guards | 34 | 32 | 3 | 13.2% (rank 1) |
| all land within 6 steps (58), 3 guards | 58 | **5** | 2 | 8.2% (rank 2) |
| the same, with 3 more guards raised at once (6) | 58 | 52 | 6 | 13.5% (rank 1) |
| the same, with 6 more raised (7 survive) | 58 | 57 | 7 | 15.7% (rank 1) |
| control: every civ on the AI | 40 | 40 | 6 | 14.2% |

So a wide claim on the starting guard is thin. It holds less than the ring, and the Standing it loses goes with that. It takes about three more guards (six in all) to hold what the ring holds, and on seed 9 the store and the clans feed them: the store sits at 27 against the ring's 45. The cost of width is people and food, and on this map paying it pays back.

In the wide-claim game, AI Red (civ 485) fell to 0.0% Standing by +50: its clans starved and merged away, while it kept claiming. In the ring game the same civ dipped to 3.2% and recovered to 9.3%. The sweep's AI-only runs never go below 1.0%, but a played game can wipe a civ out through hunger, with no war.

## The page

`play-claim.mjs` passes on the final numbers (small seed 4). Red claims and holds 33 of 33 tiles with 6 guards, and two unaligned clans lean to Gold. Its screenshots are in `screens/`. The border reads at a glance, and the capital's circles and the market ring are gone. The selected festival's walkers are listed in the panel, but their routes are hard to pick out on the map.

`play-hold.mjs` still fails ("raised only 5 warriors", where it needs 8): the founding guard takes the clans' people and food first. `play-lean.mjs` now runs to the end, though its civ gave nothing in 30 turns: given 0, with keep 15 against a store that never clears it.
