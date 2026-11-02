# #102 spec: what main does, and what the pre-game takes

Taken by the spec on main at `5496695`, release build, a 4-core runner. Each variant edited
`core/src/data.rs`, rebuilt, ran, and put it back; nothing of it is committed.

## The judge on main

`sweep --judge` on main is `main-judge.txt` (37.8 s). The totals the piece is read against:

| Measure | Main |
|---|---|
| Top Standing at +200 (mean over maps) | 0.091 |
| Aligned (the civs' share of the world) at +200 | 0.184 |
| Lead changes | 7, on 4 maps of 7 |
| Spread +100 / +200 | 0.039 / 0.053 |
| Clans changing civ after the pick | 82 |
| Land tiles changing civ | 2522 (the ring plus clans' territories, so it moves with every clan's claim) |
| Camps kept / warriors starved / camps lost | 15 of 21 / 19 / 3 |
| Pre-games by centres | 7 of 7 |

## The pre-game with desert at 2 and nearer festivals

`sweep SEEDS --size S` on the retro maps (small 2 3 6 9, medium 8 9, large 8): the pre-game's end.

| Variant | Small (4) | Medium (2) | Large (1) |
|---|---|---|---|
| Main | 4 by centres, median turn 266 | 2, median 379 | 1, turn 373 |
| Desert 2 | 4, 262 | 2, 431 | 1, 361 |
| Desert 2, spacing 6, reach_walk 14 | 4, 264 | 2, 406 | 1, 348 |
| Desert 2, spacing 5, reach_walk 12 | 4, 262 | 2, 454 | 1, 313 |

Every variant ends 7 of 7 by centres, with nobody needing the top-up. So **desert's move cost is 2**
(settled), and spacing 6 with `reach_walk` 14 is build-1's starting point; 5 and 12 also end and
push medium later.

## What nearer festivals do after the pick (today's joining)

`sweep --judge` with desert 2, spacing 6, reach_walk 14 (`probe-6-14-judge.txt`), every other rule
as main:

| Measure | Main | 6 / 14 |
|---|---|---|
| Clans changing civ | 82 | 20 |
| Lead changes | 7 on 4 maps | 3 on 2 |
| Aligned at +200 | 0.184 | 0.171 |
| Camps lost | 3 | 9 |

Nearer festivals take clans away from the capital's market, which is where today's gifts and trade
win them: joining falls by three quarters and the lead moves less. That is the gap leaning by land
is meant to fill, and why the canvas tunes the land rules with build-1's festivals in place, not
main's. Build-1 reports these numbers again on its own rules; it doesn't tune them back up.
