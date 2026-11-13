# #336 scores

`sansfiat sweep --judge --json` on the seven retro maps, every civ on the AI, 200 turns past the pick.
Control: `main-sweep.json` (main at 92939e3). Branch: `branch-sweep.json` (the planner's `PLAN.hold` at 15).
Read with `sansfiat judge --compare main-sweep.json branch-sweep.json [--all]`.

## The question: do games end before the limit, and do capitals fall?

| | main | branch |
|---|---|---|
| all-AI games ending before the limit | 0 of 7 | 0 of 7 |
| capitals fallen, all-AI | 0 | 2 (small 3, small 6) |
| led games (the planner leads one civ) ending early | 0 of 21 | 3 of 21 |
| led civs whose capital fell | 0 of 21 | 3 of 21 |
| idle led civs whose capital fell | 1 of 21 | 2 of 21 |
| tiles of claim taken by an army (`motion.claim_taken`) | 0 | 36 (small 2: 24, small 9: 9, small 3: 2, small 6: 1) |
| lead changes | 8 on 5 maps | 13 on 6 maps |
| swings | 17 | 16 |
| spread of Standing at 200 | 0.098 | 0.177 |
| aligned Standing at 200 | 0.35 | 0.47 |
| top civ's Standing at 200 | 0.16 | 0.23 |
| clans joined | 273 | 405 |

Capitals fall and land changes hands; no game with every civ on the AI ends early, since that takes two
of three capitals. On small 3 Blue's capital falls 7 turns after the pick: no civ has an army at the
pick, and Gold's first party walks 10 steps onto a capital nobody stands on. The two capitals left are 22
steps apart, and a party of 4 that sets out over that goes hungry on the way, halts to forage and
never arrives. On the medium and large maps the capitals are too far apart for any army to reach one,
and no tile is taken.

## What else moved

| | main | branch |
|---|---|---|
| largest army (mean of maps) | 14.1 | 4.0 |
| levy a turn | 15.7 | 9.5 |
| store, mean | 18.9 | 39.0 |
| warriors starved | 90 | 170 |
| fallen in fights | 34.0 | 26.5 |
| tiles won in a fight (`fight.taken`) | 14 | 6 |
| armies lost | 0 | 5 |
| claim, mean tiles at 200 | 35.2 | 50.5 |
| panics, digest mismatches | 0, 0 | 0, 0 |

Armies stay parties of 4: the planner raises more only when a rival capital is within 4 turns' walk or a
rival army is on its land (`PLAN.march`, `PLAN.party`, as on main), and on main the larger figure was
mostly the guard's stand-in for it. Warriors starve more because parties spend more turns abroad.

## The planner's `PLAN.hold` (tiles of claim a person of its armies answers for)

| hold | capitals fallen | claim tiles taken | lead changes | swings | claim at 200 | aligned at 200 | warriors starved |
|---|---|---|---|---|---|---|---|
| 6 | 2 | 8 | 5 | 8 | 32.2 | 0.29 | 139 |
| 15 (kept) | 2 | 36 | 13 | 16 | 50.5 | 0.47 | 170 |
| 30 | 1 | 63 | 16 | 17 | 95.5 | 0.73 | 272 |

At 6 a party of 4 answers for 24 tiles, under the claim a civ is founded with, so the claim never
widens. Also tried at hold 6: an army marches on a capital only within `PLAN.march` turns, party or
not: 1 capital fell, 7 tiles taken, 178 warriors starved. Not kept.
