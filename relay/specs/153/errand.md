# #153 build-5: the scout lives, and the flat trail on the finished branch

The feeding's numbers against the 28 errands (`core/tests/scout_errand.rs`, seven retro maps, each: routed to
the nearest rival capital, routed to `Lead`'s target, moved there, idle; 50 turns, no refill) and the AI's
scouts. Run with a temporary environment override of the `SCOUT` numbers (not kept), four settings at a
time, `cargo test --release -p engine --test scout_errand -- --ignored --nocapture`.

**Taken: `SCOUT { wander: 8, low: 1.5, fed: 2.5, rich: 1.2 }`, people 2, reach 1.** All 28 alive, no AI scout
lost by +200, and no AI person lost. `scout_errand` runs in `cargo test` with no `#[ignore]`.

## The settings

Columns: the numbers, scouts alive of 28, people lost (of 56), hungry scout-turns (of 1400), and on the 14
routed errands (700 scout-turns) the turns wandering, tiles revealed and trails built. `fill` is whether
the errand's own foraging fills the pack (yes) or covers the turn's eating (no, as built). A scout's pack
is `UNIT.pack`; `SCOUT.pack` was not needed.

Settings tried before the table counted work over all 21 non-idle errands (the move included), so their work
is not given:

| wander | low | fed | rich | reach | fill | people | alive | people lost | hungry |
|---|---|---|---|---|---|---|---|---|---|
| 4 | 2 | 4 | 1.5 | 1 | no | 2 | 22 | 15 | 250 |
| 6 | 2 | 4 | 1.5 | 1 | no | 2 | 24 | 8 | 176 |
| 8 | 2 | 4 | 1.5 | 1 | no | 2 | 27 | 3 | 64 |
| 4 | 2 | 4 | 1.2 | 1 | no | 2 | 26 | 5 | 98 |
| 4 | 2 | 4 | 2 | 1 | no | 2 | 23 | 11 | 232 |
| 4 | 2 | 4 | 1 | 1 | no | 2 | 27 | 4 | 55 |
| 4 | 3 | 5 | 1.5 | 1 | no | 2 | 23 | 14 | 236 |
| 4 | 1 | 3 | 1.5 | 1 | no | 2 | 23 | 12 | 219 |
| 4 | 2 | 4 | 1.5 | 1 | yes | 2 | 24 | 9 | 146 |
| 4 | 2 | 4 | 1.5 | 2 | no | 2 | 23 | 11 | 232 |
| 6 | 2 | 4 | 1.5 | 1 | yes | 2 | 23 | 11 | 222 |
| 4 | 2 | 4 | 1.5 | 2 | yes | 2 | 26 | 6 | 108 |
| 8 | 2 | 4 | 2 | 1 | no | 2 | 22 | 12 | 265 |
| 6 | 2 | 4 | 1 | 1 | no | 2 | 25 | 6 | 127 |
| 8 | 2 | 4 | 1 | 1 | no | 2 | 28 | 2 | 8 |
| 8 | 2 | 4 | 1.2 | 1 | no | 2 | 28 | 2 | 11 |
| 10 | 2 | 4 | 1.2 | 1 | no | 2 | 28 | 0 | 5 |

Then, with the work counted on the 14 routed errands:

| wander | low | fed | rich | reach | fill | people | alive of 28 | people lost | hungry | wandering | tiles revealed | trails built |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 10 | 2 | 4 | 1.5 | 1 | no | 1 | 26 | 2 | 84 | 461 | 19 | 0 |
| 10 | 1.25 | 2.5 | 1 | 1 | no | 1 | 25 | 3 | 155 | 363 | 26 | 3 |
| 10 | 1.5 | 2.5 | 1.2 | 1 | no | 1 | 24 | 4 | 241 | 320 | 24 | 0 |
| 6 | 1.5 | 2.5 | 1.2 | 1 | no | 1 | 20 | 8 | 345 | 292 | 23 | 0 |
| 10 | 1.5 | 2.5 | 1.2 | 1 | no | 2 | 28 | 0 | 43 | 332 | 36 | 5 |
| 10 | 1.5 | 2 | 1.2 | 1 | no | 2 | 28 | 0 | 43 | 174 | 36 | 5 |
| 10 | 1.25 | 2 | 1.2 | 1 | no | 2 | 28 | 0 | 57 | 231 | 36 | 5 |
| 12 | 1.5 | 2.5 | 1.2 | 1 | no | 2 | 28 | 0 | 44 | 332 | 36 | 5 |
| 10 | 1.25 | 2.5 | 1.2 | 1 | no | 2 | 28 | 1 | 54 | 401 | 35 | 5 |
| 10 | 1.25 | 3 | 1.2 | 1 | no | 2 | 28 | 0 | 27 | 462 | 34 | 5 |
| 10 | 1.25 | 2.5 | 1 | 1 | no | 2 | 28 | 0 | 34 | 441 | 33 | 5 |
| 8 | 1.5 | 2.5 | 1.2 | 1 | no | 2 | 28 | 1 | 46 | 329 | 33 | 5 |
| 10 | 1.5 | 2.5 | 1 | 1 | no | 2 | 28 | 0 | 24 | 396 | 33 | 5 |
| 10 | 1.5 | 3 | 1.2 | 1 | no | 2 | 28 | 0 | 22 | 463 | 32 | 5 |
| 8 | 1.5 | 3 | 1 | 1 | no | 2 | 28 | 1 | 18 | 504 | 31 | 5 |
| 10 | 1.5 | 3 | 1 | 1 | no | 2 | 28 | 0 | 16 | 504 | 31 | 5 |
| 8 | 1.5 | 3 | 1.2 | 1 | no | 2 | 28 | 1 | 26 | 454 | 30 | 5 |
| 10 | 2 | 4 | 1.5 | 1 | no | 2 | 28 | 0 | 15 | 482 | 34 | 2 |
| 10 | 1.5 | 3 | 1.2 | 1 | yes | 2 | 28 | 0 | 12 | 163 | 28 | 5 |
| 10 | 1.5 | 2.5 | 1.2 | 1 | yes | 2 | 28 | 0 | 11 | 96 | 28 | 5 |
| 8 | 2 | 4 | 1.2 | 1 | no | 2 | 28 | 2 | 11 | 617 | 25 | 2 |
| 8 | 2 | 4 | 1 | 1 | no | 2 | 28 | 2 | 8 | 630 | 25 | 2 |
| 10 | 2 | 4 | 1 | 1 | no | 2 | 28 | 0 | 3 | 632 | 25 | 2 |
| 10 | 2 | 4 | 1.2 | 1 | no | 2 | 28 | 0 | 5 | 631 | 25 | 2 |
| 10 | 2 | 3 | 1.5 | 1 | no | 2 | 28 | 0 | 26 | 358 | 23 | 2 |
| 10 | 1 | 3 | 1.2 | 1 | no | 2 | 27 | 4 | 65 | 417 | 45 | 5 |
| 8 | 1 | 3 | 1.2 | 1 | no | 2 | 27 | 5 | 69 | 407 | 43 | 5 |
| 8 | 1 | 3 | 1 | 1 | no | 2 | 27 | 3 | 52 | 423 | 39 | 5 |
| 10 | 1 | 3 | 1 | 1 | no | 2 | 27 | 2 | 51 | 463 | 39 | 5 |
| 10 | 1.1 | 2.5 | 1.2 | 1 | no | 2 | 27 | 2 | 80 | 387 | 35 | 5 |
| 10 | 1.5 | 3 | 1.2 | 2 | no | 2 | 27 | 2 | 68 | 458 | 33 | 5 |
| 8 | 2 | 4 | 1.5 | 1 | no | 2 | 27 | 3 | 64 | 461 | 33 | 2 |
| 10 | 2 | 3 | 1.5 | 1 | yes | 2 | 27 | 2 | 57 | 78 | 21 | 4 |
| 10 | 1 | 3 | 1.5 | 1 | no | 2 | 26 | 8 | 160 | 325 | 53 | 9 |
| 10 | 1.5 | 3 | 1.5 | 1 | no | 2 | 26 | 5 | 145 | 323 | 30 | 5 |
| 10 | 1.5 | 2.5 | 1.5 | 1 | no | 2 | 26 | 5 | 150 | 262 | 29 | 5 |
| 8 | 1 | 3 | 1.5 | 1 | no | 2 | 24 | 11 | 248 | 309 | 53 | 10 |
| 8 | 1.5 | 3 | 1.5 | 1 | no | 2 | 24 | 9 | 234 | 309 | 30 | 5 |

The first table's rows from the single levers at the starting numbers show the order the levers worked in:
**wander** was the lever (4 to 8 to 10 lost 22 to 24 to 27 to 28 of 28), then **rich** (1.5 passed over ground that
only just fed a scout: 1 and 1.2 keep scouts alive where 1.5 does not), then **low** and **fed**, which
spend fewer turns off the errand for the same lives. **Fill** (the errand's own foraging fills the pack)
cut the turns wandering by a half to three quarters and did not raise the work: at the same lives it
revealed fewer tiles (28 against 36 at wander 10, rich 1.2, low 1.5, fed 2.5). **Reach 2** lost one errand
more than reach 1 at the same numbers. **One person** lost 2 to 8 of 28 at every setting tried (the last four
rows): half the labour comes with half the eating, so the person count is still not the lever. **A
`SCOUT.pack`** was not needed: the pack holds `fed` turns at most, so a bigger one only matters to the
wander's room, and the settings that pass don't leave the pack full.

Work among the settings that keep all 28 alive tops out at **36 tiles and 5 trails** (wander 10 to 12, rich 1.2,
low 1.25 to 1.5, fed 2 to 2.5). Only low 1 (a pack of one turn) goes past it at rich 1 or 1.2 (39 to 45 tiles, 5 trails), and
loses medium 9's scout to the rival every time; at rich 1.5 it reveals 53 and builds 9 or 10 and loses 2 to 4 others. Of the 36-and-5 settings, wander 10 and 12 each lose a person
of one AI idle scout by +200 (medium 9, below), so the one taken is wander 8: 33 tiles and 5 trails, the AI's
scouts untouched.

## The one taken: `wander 8, low 1.5, fed 2.5, rich 1.2, reach 1, people 2`

All 28 alive; people lost 1 (small 6 moved to `Lead`'s target); 46 hungry scout-turns of 1400.
`cargo test --release -p engine --test scout_errand -- --nocapture`. "Turns wandering" is the turns the
scout holds a wander tile at the end of its turn: a routed scout away from its route feeding, and any scout
that has nothing left to do (a moved scout that has arrived, an idle one) waiting on rich ground.
"Route found" is the turn the scout's `ScoutRouteFound` event fell, from the pick.

| map | errand | hungry turns | people lost | turns wandering | tiles revealed | trails built | route found | alive |
|---|---|---|---|---|---|---|---|---|
| small 2 | route to the rival | 2 | 0 | 32 | 1 | 0 | - | yes |
| small 2 | route to Lead's target | 0 | 0 | 9 | 2 | 0 | turn 2 | yes |
| small 2 | move to Lead's target | 1 | 0 | 47 | 0 | 0 | - | yes |
| small 2 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| small 3 | route to the rival | 0 | 0 | 27 | 1 | 0 | turn 1 | yes |
| small 3 | route to Lead's target | 3 | 0 | 35 | 2 | 0 | - | yes |
| small 3 | move to Lead's target | 4 | 0 | 35 | 0 | 0 | - | yes |
| small 3 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| small 6 | route to the rival | 1 | 0 | 23 | 1 | 0 | - | yes |
| small 6 | route to Lead's target | 0 | 0 | 5 | 2 | 2 | turn 1 | yes |
| small 6 | move to Lead's target | 3 | 1 | 48 | 0 | 0 | - | yes |
| small 6 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| small 9 | route to the rival | 11 | 0 | 32 | 1 | 0 | - | yes |
| small 9 | route to Lead's target | 1 | 0 | 27 | 3 | 0 | - | yes |
| small 9 | move to Lead's target | 1 | 0 | 47 | 0 | 0 | - | yes |
| small 9 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| medium 8 | route to the rival | 4 | 0 | 26 | 3 | 0 | - | yes |
| medium 8 | route to Lead's target | 1 | 0 | 27 | 6 | 0 | - | yes |
| medium 8 | move to Lead's target | 1 | 0 | 47 | 0 | 0 | - | yes |
| medium 8 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| medium 9 | route to the rival | 5 | 0 | 27 | 3 | 0 | - | yes |
| medium 9 | route to Lead's target | 0 | 0 | 2 | 5 | 2 | turn 8 | yes |
| medium 9 | move to Lead's target | 0 | 0 | 47 | 0 | 0 | - | yes |
| medium 9 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |
| large 8 | route to the rival | 6 | 0 | 30 | 0 | 0 | - | yes |
| large 8 | route to Lead's target | 1 | 0 | 27 | 3 | 1 | turn 1 | yes |
| large 8 | move to Lead's target | 1 | 0 | 48 | 0 | 0 | - | yes |
| large 8 | idle | 0 | 0 | 50 | 0 | 0 | - | yes |

### The work beside the refilled control

The control is `the_refilled_scouts_work` (`--ignored`): the same 14 routed errands with the pack topped up
each turn, so the scout never feeds itself:

| map | errand | hungry turns | people lost | turns wandering | tiles revealed | trails built | route found | alive |
|---|---|---|---|---|---|---|---|---|
| small 2 | route to the rival | 0 | 0 | 0 | 11 | 4 | turn 9 | yes |
| small 2 | route to Lead's target | 0 | 0 | 0 | 2 | 0 | turn 2 | yes |
| small 3 | route to the rival | 0 | 0 | 0 | 1 | 0 | turn 1 | yes |
| small 3 | route to Lead's target | 0 | 0 | 0 | 10 | 2 | turn 24 | yes |
| small 6 | route to the rival | 0 | 0 | 0 | 8 | 5 | turn 16 | yes |
| small 6 | route to Lead's target | 0 | 0 | 0 | 2 | 2 | turn 1 | yes |
| small 9 | route to the rival | 0 | 0 | 0 | 15 | 0 | - | yes |
| small 9 | route to Lead's target | 0 | 0 | 0 | 14 | 4 | turn 16 | yes |
| medium 8 | route to the rival | 0 | 0 | 0 | 13 | 2 | turn 36 | yes |
| medium 8 | route to Lead's target | 0 | 0 | 0 | 11 | 3 | turn 24 | yes |
| medium 9 | route to the rival | 0 | 0 | 0 | 23 | 0 | - | yes |
| medium 9 | route to Lead's target | 0 | 0 | 0 | 5 | 2 | turn 5 | yes |
| large 8 | route to the rival | 0 | 0 | 0 | 22 | 0 | - | yes |
| large 8 | route to Lead's target | 0 | 0 | 0 | 3 | 1 | turn 1 | yes |

| 14 routed errands, 50 turns | tiles revealed | trails built | routes found | turns wandering |
|---|---|---|---|---|
| The setting taken | 33 | 5 | 5 | 329 of 700 |
| Refilled control | 140 | 25 | 11 | 0 |
| Taken as a share | 24% | 20% | 5 of 11 | |

Under a quarter of the control's work on both counts (33 of 140 is 23.6%, 5 of 25 is 20%), so an `ask` goes
to the owner: past the line's reach the scout's labour goes on its food, and D's supply ring or a cheaper
scout forage is the owner's call. The best setting that keeps all 28 alive does 36 tiles (26%) and 5 trails
(20%): the shortfall is the trails.

### The search's progress

The route is found (`ScoutRouteFound`) in 5 of the 14 routed errands in 50 turns (`Lead`'s target on small 2 (turn 2), small 6, large 8 and
medium 9 (turn 8), and the rival on small 3 (turn 1)); the control finds 11 of 14 (the three it misses are the long rival errands on small 9, medium 9 and
large 8). The other nine go unfound in 50 turns: the scout reveals 33 tiles and builds 5 trails in all 14 and spends the
rest of its labour on its food.

## The AI's scouts

`the_ais_scouts_live_to_200` (`#[ignore]`, `--ignored --nocapture`): the seven retro maps with every civ on
the AI, 200 turns past the pick: **21 scouts at the pick, 21 alive at +200, 0 lost, 0 people lost to hunger.**
At wander 10 and 12 the same report shows one person lost to hunger (medium 9, civ 806's scout, turn 609)
and no scout gone; wander 8 loses none.
