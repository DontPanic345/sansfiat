# #153 review: Move to… stays

## What was wrong

The piece decides "Move to… walks there and stays, foraging". As built, a scout that arrived cleared its `dest`
(build-2), so it became idle, and an idle scout walks to rich ground up to `SCOUT.wander` (8) steps away
(build-3/5). On the retro maps the moved scout reached Lead's target and left a turn later, drifting 5 to 18
steps away for good:

| map | arrived | steps from the target, turns 1-12 (before) | after the fix |
|---|---|---|---|
| small 2 | turn 2 | 3 1 0 3 4 4 4 4 4 4 4 4 … 5 to the end | as before: it can't stay fed, and comes back now and then |
| small 6 | turn 1 | 2 0 6 6 6 3 3 3 3 8 7 11 … 11 to the end | 2 0 0 0 6 … back at 16, 29, 43 |
| small 9 | turn 2 | 4 1 0 3 7 7 7 10 … 10 to the end | back within 5 steps every 8 turns |
| medium 8 | turn 2 | 8 2 0 6 6 6 7 13 … 18 at the end | back at 19, 41, 49 |
| medium 9 | turn 2 | 7 1 0 7 8 6 6 6 9 … | back at 9, 15, 21, 22, 27, 49 |
| large 8 | turn 1 | (not measured before) | 3 0 0 5 7 … 7 |
| small 3 | never | 8 11 12 11 … | as before |

## The fix

A scout keeps its `dest` after it arrives: the tile stays its errand until another order (Stop, Route, Move).
It forages round it, leaves it only when its pack runs under `SCOUT.low`, and comes back fed ("fed: back to X"),
as spec item 16 already says. `line::route` draws no march for a scout standing on its `dest`, and the panel
reads "At X, foraging". Stop (a move to its own tile) ends the errand, and the scout then waits on rich ground.

`scout_errand` after the fix: all 28 alive, 0 people lost (1 before), 61 hungry scout-turns; the routed work is
unchanged (33 tiles revealed, 5 trails built, 329 turns wandering). The AI never moves a scout, so the sweep
and the digest pins don't move.

Where Lead's target is bare and past the line's reach (small 2, small 3, large 8) the scout spends most of its
turns feeding away from it: the rule keeps it alive rather than at its post. That is the feeding rule working,
not Move to…; the reach warning tells the player so when they give the order.
