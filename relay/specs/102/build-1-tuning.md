# #102 build-1: what was tried

Every row is `sweep --judge` on the seven retro maps with desert at 2. Shares are pick / +200 (homes
trailed, worked trailed, route trailed); `flips` is per 100 clan-turns after the pick; "centres" is
pre-games ended by centres. Main (`build-1-main.txt`): markets 142, flips 4.35, homes 0.25, worked 0.99,
route 0.93, centres 7 of 7. Targets: flips ≤ 0.87, homes ≥ 0.8, worked ≤ 0.35, route ≥ 0.5, markets ≥ 142,
centres 7 of 7.

Trail traffic at the pick (people-walks, keep 0.9; a histogram printed once, not committed): homes of
clans of 4+ people median ~185 (quarter under ~140), tiles on two clans' routes ~225, worked tiles ~70
(quarter over ~110).

| spacing / reach_walk | keep_turns / stay | keep / on / off | markets (pick) | flips | homes | worked | route | centres |
|---|---|---|---|---|---|---|---|---|
| 6 / 14 | 5 / 3.0 | 0.9 / 40 / 15 | 203 | 0.04 | 1.00 | 0.98 | 1.00 | **6 of 7** (medium 9 ended on the limit, cover 0.68) |
| 8 / 20 (main's) | 5 / 3.0 | 0.9 / 40 / 15 | n/a | n/a | n/a | n/a | n/a | medium 8, 9 end by centres (242, 378) |
| 6 / 14 | 0 / 3.0 | 0.9 / 40 / 15 | n/a | n/a | n/a | n/a | n/a | medium 8, 9 end by centres (290, 369) |
| 6 / 14 | 5 / 100 | 0.9 / 40 / 15 | n/a | n/a | n/a | n/a | n/a | medium 8, 9 end by centres (299, 405) |
| 6 / 14 | 5 / 3.0 | 0.9 / 100 / 80 | 207 | 0.13 | 0.87 | 0.34 | 0.98 | 7 of 7 |
| 6 / 14 | 5 / 3.0 | 0.9 / 120 / 100 | 208 | 0.08 | 0.83 | 0.27 | 0.96 | 7 of 7 |
| 5 / 12 | 5 / 3.0 | 0.9 / 110 / 90 | 279 | 0.03 | 0.84 | 0.27 | 0.99 | 7 of 7 |
| 6 / 14 | 5 / 3.0 | 0.9 / 110 / 70 | 213 | 0.12 | 0.88 | 0.36 | 0.97 | 7 of 7 (worked misses by 0.01) |
| **6 / 14** | **5 / 3.0** | **0.9 / 110 / 90** | **218** | **0.08** | **0.84** | **0.31** | **0.97** | **7 of 7** (final, `build-1-judge.txt`) |

(Rows marked n/a were pre-game checks on the medium maps only, to find which of the new rules moved
medium seed 9 off the centres.) The first row's 6 of 7 is the same festivals and trail thresholds as
the last but for `on`/`off`: the trail rule changes the world, so whether medium 9 spreads over 80% of
its land by turn 500 is chaotic in the thresholds; the final numbers, and `5 / 12` and `100 / 80`,
`120 / 100`, all end 7 of 7.

All targets are met. Final numbers: flips 0.08 (98% under main's 4.35), markets 218 vs 142, homes
0.84 / 0.84, worked 0.31 / 0.30, route 0.97 / 0.98.

Side effects on the judge's other groups (not tuned): clans changing civ 82 → 32, lead changes 7 on 4
maps → 7 on 3, aligned at +200 0.184 → 0.145, warriors starved 19 → 12, camps lost 3 → 1. The civs'
stores take in less at the pick than main's (about 3 a turn at small seed 2, falling to nothing by turn 9
under a camp and a scout), because nearer festivals draw clans from the capital's market: leaning by land
(build-2) is meant to fill that.
