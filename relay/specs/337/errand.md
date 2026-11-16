# #337 errands on the seven retro maps

`core/tests/scout_routes.rs` (a report: `cargo test --release -p engine --test scout_routes -- --ignored --nocapture`,
`FIGHT=on|off` picks one half). Each map's civs are in turn the led civ, idle past the pick, with the others on the AI; its
scout gets one order, `Route` from its capital to each rival capital, and the game runs 200 turns. 3 civs a map, so 6
errands a map and 42 in all, fight off and fight on. `errand-main.txt` is main at 4fea294 (the same test minus the columns
main has no state for: turns off the errand, turns waiting, kills), `errand-branch.txt` the branch at the numbers in
`SCOUT`, every errand a row.

"Built" is the scout's `built` flag: its route needs no more trails. "Off" is turns on `homing` or `back`; "filling" and
"too far" are turns at a store with `Doing::Filling` (the pack not yet at `out`) or `Doing::TooFar` (the errand is
further than a full pack there and back). "Starved" is people lost to hunger (2 a scout); "gone" is scouts not alive at +200
(starved, killed, or the civ fell).

## Main against the branch

| 42 routed errands | built +50 | +100 | +200 | starved (people) | killed | gone at +200 |
|---|---|---|---|---|---|---|
| main, fight off | 7 | 8 | 8 | 33 | - | 11 |
| main, fight on | 5 | 8 | 8 | 34 | - | 20 |
| branch (36, 3, 0.25), fight off | 24 | 24 | 27 | 6 | 0 | 3 |
| branch (36, 3, 0.25), fight on | 16 | 17 | 19 | 8 | 7 | 18 |

The branch builds 27 of the 42 routes where main builds 8. Fight on, the led civ is idle and Blue or Gold takes its
capital on some maps: that is most of the 18 gone (7 are killed by a rival army on the scout's tile; the rest are scouts whose
civ fell or starved). The scout turns off its errand 2,200 turns of the 8,400 fight off (26%): that is the price of eating
from a store.

## What each setting gave

All fight off, 42 routed errands; `pack, spare, out`. "Starved" is people, "gone" scouts at +200 (3 of them, the first three
people-pairs, are the medium 8 scouts of civs 816 and 818, who starve in every setting; below).

| pack | spare | out | built +50 / +100 / +200 | off | filling | too far | starved | gone |
|---|---|---|---|---|---|---|---|---|
| 16 | 2 | 0.5 | 20 / 20 / 20 | 3693 | 104 | 2265 | 4 | 2 |
| 20 | 2 | 0.5 | 21 / 22 / 22 | 3072 | 154 | 1554 | 6 | 3 |
| 24 | 2 | 0.5 | 22 / 23 / 23 | 2761 | 282 | 1129 | 6 | 3 |
| 30 | 2 | 0.5 | 22 / 23 / 23 | 2262 | 409 | 735 | 10 | 5 |
| 36 | 2 | 0.5 | 24 / 24 / 24 | 2008 | 537 | 345 | 8 | 4 |
| 48 | 2 | 0.5 | 22 / 24 / 26 | 1802 | 672 | 0 | 9 | 4 |
| 24 | 2 | 0.1 | 22 / 23 / 23 | 2702 | 0 | 1129 | 6 | 3 |
| 24 | 2 | 0.25 | 22 / 23 / 23 | 2702 | 0 | 1129 | 6 | 3 |
| 24 | 2 | 0.75 | 23 / 23 / 24 | 2488 | 537 | 1036 | 6 | 3 |
| 36 | 2 | 0.1 | 23 / 24 / 25 | 1993 | 0 | 221 | 8 | 4 |
| 36 | 2 | 0.25 | 23 / 24 / 25 | 2024 | 134 | 221 | 8 | 4 |
| 36 | 2 | 0.75 | 23 / 24 / 25 | 1845 | 779 | 378 | 6 | 3 |
| 48 | 2 | 0.1 | 23 / 24 / 26 | 1909 | 0 | 5 | 10 | 5 |
| 48 | 2 | 0.25 | 23 / 24 / 26 | 1998 | 282 | 5 | 10 | 5 |
| 48 | 2 | 0.75 | 23 / 24 / 26 | 1407 | 905 | 0 | 10 | 5 |
| 36 | 1 | 0.25 | 24 / 24 / 26 | 1989 | 151 | 90 | 8 | 4 |
| 36 | 3 | 0.25 | 24 / 24 / 27 | 2200 | 106 | 364 | 6 | 3 |
| 36 | 4 | 0.25 | 24 / 24 / 25 | 2151 | 43 | 379 | 6 | 3 |
| 30 | 3 | 0.25 | 24 / 24 / 24 | 2398 | 0 | 562 | 8 | 4 |
| 33 | 3 | 0.25 | 24 / 24 / 24 | 2295 | 44 | 455 | 6 | 3 |
| 42 | 3 | 0.25 | 24 / 24 / 27 | 2115 | 145 | 65 | 8 | 4 |
| 48 | 3 | 0.25 | 24 / 24 / 27 | 2127 | 216 | 5 | 8 | 4 |

- **pack.** The built count climbs with the pack and stops: 16 gives 20, 24 gives 23, 36 gives 24 to 27, and 42 and 48
  give 27 too. The routes lost at 24 are mostly the ones further than a pack there and back (1,129 turns at `TooFar`
  at 24, 345 at 36, none at 48). 36 is the smallest pack at the top. A civ's store holds 60 food, 30 turns of a scout's
  eating, so a pack of 36 (72 food) is never filled from one visit: the scout fills over the turns it stands there.
- **out.** The share of a pack a scout waits for. A poor store (medium 9, a food a turn) holds a scout at `Filling` for
  tens of turns at 0.5 and 0.75 (905 turns at 48 and 0.75); 0.25 sets it out sooner and builds the same routes as
  0.1, which sends it out on a pack that it then has to refill after a few turns' work. 0.25.
- **spare.** At 1 and 2 turns a scout turns for its store with the walk home a turn short on some maps (small 9, civ 310 to
  311, at 36 and 2: gone). At 3 none of the 39 scouts that had a store is lost on the walk, at 4 the same, with fewer routes
  built (25 against 27): 3.
- The number is flat to a route or two across neighbouring settings (a tuned pack at 33 builds 24, at 36 27, at 42 27), and
  main's 8 is far under any of them: the pack and the store matter, the third decimal does not.

MEDIUM8_PLACEHOLDER

## The store

- Medium 9: the led civs' stores gain about a food a turn, and the scout eats two. At 36, 3 and 0.25 civ 846's scout waits
  41 and 65 turns at `Filling` and builds neither route; fight off every medium 9 errand fails (6 of 6). At the pick the
  civ's store is empty and the scout takes everything that arrives (`cargo test` scratch: store 0.0 for 80 turns at `out`
  0.5). A poor store cannot carry a pack, and the scout cannot do an errand there: this is the piece's main risk and it
  is the store's, not a number's.
- Small 9, civ 311, and large 8, civ 1719: the rival is further than a full pack takes the scout and brings it back
  (`TooFar` for 119 to 139 and 84 turns); no setting under 48 finishes them, and at 48 they still don't (the store holds 30
  turns of the pack).
- The AI's own scouts never run an errand in the sweep, so they only eat at stores: `scores.md`.
