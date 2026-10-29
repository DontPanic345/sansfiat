# #28 review: a scout on real maps

`engine/check.sh` passes, and `node www/play-scout.mjs` plays the route through the page
(seed 2, small, capital 166 to tile 58: known 11, possible 7, found in 2 turns, one trail built).

Measured at review on a scratch test (not kept): seeds 1-8, radius 16, each led civ's scout sent from its
capital to each rival capital, run until its route ended (300 turns at most).

| Result | Routes |
|---|---|
| Ended in the first turn with nothing revealed (seed 5 checked: no path at all, so no route can exist) | 7 of 16 |
| Searched, the scout starved to nothing before the route was found | 5 of 9 |
| Searched, the scout lost one of its two people | 2 of 9 |
| Searched, the route ended with both people alive | 2 of 9 |

Why: the scout starts with 5 food for 2 people (the store after the camp's pack), forages one tile a turn
(tiles hold 2-3 food in clan country, then fall fallow for 5 turns), and its line to the capital carries
less than one person's food from about 8 steps out (cost 24 a turn for 0.875 food). A mountain reveal takes
a whole turn on one spot, so it can't forage fresh ground in that turn. Past about 5-8 steps from the capital
a scout can't keep itself fed and searches. The painted-map proofs in `core/tests/scout.rs` all sit near the
capital, so they don't show it.

Two things tried and dropped (no effect on the 16 routes): keeping a part-paid step's banked labour out of
the feeding, and capping the line at half the labour left. The fix is tuning (a bigger starting pack or
larger forage, or the scout's pack filled before the camp's), which the owner may want to choose: the design
says its numbers are a place to start from.

Also seen: after a reveal the nearest candidate can be back toward the capital, so the scout walks home
and out again (seed 1, turns 4-13), which is the back and forth `scouting.md` asks to avoid; it was not
chased further here.
