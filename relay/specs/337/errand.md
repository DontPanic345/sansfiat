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
| branch (36, 3, 0.25), fight off | 24 | 25 | 27 | 0 | 0 | 0 |
| branch (36, 3, 0.25), fight on | 17 | 17 | 19 | 0 | 9 | 16 |

The branch builds 27 of the 42 routes where main builds 8, and no scout starves (main: 33 people). Fight on, the led civ is
idle and Blue or Gold takes its capital on some maps: that is most of the 16 gone (9 are killed by a rival army on the
scout's tile; the rest are scouts whose civ fell). The scout is off its errand (walking to a store or back) 2,259 turns of
the 8,400 fight off (27%): that is the price of eating from a store.

## A bug this step found: the way home wore away

The first runs lost three medium 8 scouts to hunger in every setting (6 people). The scout builds a pass, walks on, and a
trail nobody walks wears away: with the pass gone behind it the scout had no path to any store (`store_walk` is `None`),
never turned for one, and starved on the far side at "searching". `scout::keep_way` now counts the built passes between a
scout and its nearest store as walked each turn, and `core/tests/pass.rs`
(`a_scout_past_its_pass_keeps_the_way_home`) holds it: without the fix the scout is gone, with it it walks home across the
pass. Every table here is after the fix; the one in the first runs (spare 1 and 2 losing scouts on small 9 at 36) was this
bug, not the spare.

## What each setting gave

All fight off, 42 routed errands; `pack, spare, out`. "Starved" is people lost to hunger.

| pack | spare | out | built +50 / +100 / +200 | off | filling | too far | starved |
|---|---|---|---|---|---|---|---|
| 16 | 2 | 0.5 | 20 / 20 / 20 | 3788 | 104 | 2279 | 0 |
| 20 | 2 | 0.5 | 21 / 22 / 22 | 3185 | 154 | 1483 | 0 |
| 24 | 2 | 0.5 | 22 / 23 / 23 | 2889 | 282 | 1087 | 0 |
| 30 | 2 | 0.5 | 23 / 25 / 25 | 2275 | 409 | 663 | 0 |
| 36 | 2 | 0.5 | 24 / 24 / 26 | 2104 | 537 | 346 | 0 |
| 48 | 2 | 0.5 | 22 / 24 / 26 | 1980 | 672 | 52 | 1 |
| 24 | 2 | 0.25 | 22 / 23 / 23 | 2830 | 0 | 1087 | 0 |
| 36 | 2 | 0.1 | 23 / 25 / 27 | 2105 | 0 | 218 | 0 |
| 36 | 2 | 0.75 | 23 / 24 / 26 | 1988 | 779 | 374 | 0 |
| 48 | 2 | 0.25 | 23 / 25 / 28 | 2134 | 282 | 5 | 0 |
| 36 | 1 | 0.25 | 24 / 25 / 28 | 2078 | 151 | 49 | 0 |
| 36 | 3 | 0.25 | 24 / 25 / 27 | 2259 | 106 | 342 | 0 |
| 36 | 4 | 0.25 | 24 / 25 / 27 | 2265 | 43 | 353 | 0 |
| 30 | 3 | 0.25 | 24 / 24 / 25 | 2424 | 0 | 511 | 0 |
| 33 | 3 | 0.25 | 24 / 25 / 26 | 2222 | 44 | 323 | 0 |
| 42 | 3 | 0.25 | 24 / 25 / 27 | 2164 | 145 | 65 | 0 |
| 48 | 3 | 0.25 | 24 / 25 / 27 | 2210 | 216 | 5 | 0 |
| 24 | 3 | 0.25 | 23 / 23 / 23 | 2839 | 0 | 1037 | 0 |
| 36 | 3 | 0.5 | 24 / 24 / 27 | 2113 | 520 | 352 | 0 |

- **pack.** The built count climbs with the pack and stops: 16 gives 20, 24 gives 23, 30 gives 25, 36 gives 26 to 28, and
  42 and 48 give 27 and 28. The routes lost at 24 are mostly the ones further than a pack there and back (1,087 turns at
  `TooFar` at 24 and 2, 346 at 36, 52 at 48). 36 is the smallest pack at the top. A civ's store holds 60 food, 30 turns of a
  scout's eating, so a pack of 36 (72 food) is never filled from one visit: the scout takes what the store holds and fills
  the rest over the turns it stands there (or, with `out`, sets out on a quarter).
- **out.** The share of a pack a scout waits for. At 0.5 and 0.75 scouts wait at `Filling` (537 turns at 36 and 0.5, 779 at
  0.75); at 0.25 they wait 106 to 151 turns and build the same routes as at 0.1 (which waits 0 and sends the scout out on a
  pack it must refill after a few turns' work: 218 turns at `TooFar` at 36 and 2, against 346 at 0.5). 0.25.
- **spare.** With the way-home bug fixed no scout starves at a spare of 1, 2, 3 or 4, and 36 builds 28, 27, 27 and 27; turns
  at `TooFar` fall as the spare falls (49 at 1, 342 at 3). The errand a spare of 1 finishes is one route more; it leaves the
  scout one turn of food over the walk home, where the turn it spends working before it looks again and the turn it arrives
  on take two. 3 keeps the margin the note beside `SCOUT` gives; the one route is within the noise of any neighbouring
  setting.
- The number is flat to a route or two across neighbouring settings (a pack of 33 builds 26, of 36 27, of 42 27), and main's 8
  is far under any of them: the pack and the store matter, the third decimal does not.

## The store

- A pack of 36 turns (72 food) from a store of 60 is the piece's main risk, and it holds on six of the seven maps: the scouts
  take what the store holds, which the civ's levy refills in about 6 turns (medium 9, civ 845: store 53 before the first
  fill, 0 after, 53 again 7 turns later); armies are raised from what the store holds, and in the sweep `scores.md` counts
  what that cost. `filling` is 106 of 8,400 scout-turns at 0.25: waiting on a store is rare.
- Medium 9, civ 846: its store gains about a food a turn and the scout eats two. At 36, 3 and 0.25 its scout waits 41 and 65
  turns at `Filling` and builds neither route. A poor store cannot carry a pack, and that scout cannot do an errand: this is
  the store's limit, not a number's. The other two medium 9 civs have stores that fill; their routes (to the rival,
  well over 14 turns' walk) fail because the scout spends 155 to 177 of the 200 turns walking between the store and the
  errand (`off`): fight off, none of medium 9's six routes is built by +200.
- Small 9, civ 311, and large 8, civ 1719: the rival is further than a full pack takes the scout and brings it back (`TooFar`
  for 119 to 139 and 84 turns); no setting under 48 finishes them, and at 48 `TooFar` is 5 turns for the whole report.
- The AI's own scouts never run an errand in the sweep, so they only eat at stores: `scores.md`.
