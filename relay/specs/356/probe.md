# #356 spec-review probes, on main at 943eac6 (= 8d1b4ed's engine)

Run with `relay/specs/356/probe.patch` applied (eprintln lines behind `PROBE=1`, reverted after):
`PROBE=1 sansfiat run --seed S --size Z --turns T --no-events`, T = the sweep's pick + 200
(+250 medium, +300 large). Every civ on the AI, as the sweep. Four of the seven retro maps.

## Salt into a civ's store

Salt in the capital's store before its spoil, the mean a turn past the pick. After the spoil it
is ~0 on every civ: the keeping uses every unit that arrives (a store of 50 food uses up to 5
salt a turn). So this is the store's whole inflow, levy and market together.

| map | civ | store food | salt in a turn |
|---|---|---|---|
| small 2 | 467 | 49.5 | 0.630 |
| small 2 | 468 | 59.5 | 0.277 |
| small 2 | 469 | 29.5 | 0.000 |
| medium 9 | 712 | 59.0 | 0.092 |
| medium 9 | 713 | 43.0 | 0.337 |
| medium 9 | 714 | 50.4 | 0.000 |
| medium 8 | 781 | 59.6 | 0.400 |
| medium 8 | 782 | 35.2 | 0.007 |
| medium 8 | 783 | 56.4 | 0.422 |
| large 8 | 1885 | 17.4 | 0.000 |
| large 8 | 1886 | 58.0 | 0.007 |
| large 8 | 1887 | 60.9 | 0.004 |

The levy takes `CIV.levy` (0.2) of a clan's basket salt; the clan basket salt at its civ's market
is about 1 at best (civ 467: levied 0.18 of a mean 0.95). The levy's cap (`civ_holds`, 6 for a
store of 60) never binds. **So raising `civ_holds` by `KEEPING.packs` adds no salt to a store:**
salt for packs must be salt the keeping doesn't use. Even then, at these inflows a reserve of 10
takes 16 turns on the best civ and never fills on half of them.

Settlement stores: mean salt 0.000–0.013, obsidian 0.001–2.6 (small 2), over 148–468
settlement-turns a map.

## What salt buys

At festivals where salt cleared (small 2, 921 market-turns): salt's price over the mean food
price, median **0.34**, 90th percentile 0.74. A clan's worth for salt is at most 1 food, so salt
never buys much more than a food. A warrior eats 1 food a turn (`UNIT.food`), so a unit of salt
is about a third of a warrior-turn, not two.

## Carry against Lines: what each could move

Per clan-turn at the trade step, a clan whose salt or obsidian went home unsold, and whether
another festival in its reach (`festival_costs`) had that good in its `goods` last clearing:

| map | good | unsold clan-turns | amount | with another festival trading it | amount | a capital among them |
|---|---|---|---|---|---|---|
| small 2 | salt | 1,242 | 5,742 | 68 | 102 | 3 |
| small 2 | obsidian | 13,655 | 4,361 | 455 | 541 | 455 |
| medium 9 | salt | 6,694 | 39,313 | 310 | 4,838 | 6 |
| medium 9 | obsidian | 19,573 | 8,948 | 113 | 11 | 113 |
| medium 8 | salt | 1,828 | 5,682 | 956 | 2,345 | 12 |
| medium 8 | obsidian | 35,441 | 20,089 | 379 | 19 | 379 |
| large 8 | salt | 8,378 | 31,922 | 3,950 | 8,578 | 0 |
| large 8 | obsidian | 99,331 | 30,473 | 12 | 2 | 12 |

Obsidian trades only where a civ posts and wants it: a capital's market with room under
`ARMS.hold`. Every festival a clan could carry obsidian to is a capital's. Carry can move a few
hundred units of obsidian a map to capitals and thousands of salt between festivals.

Lines can move only what settlement stores hold: ~0 salt and at most 2.6 obsidian on average,
within one civ, on the 5 of 7 maps with a settlement, to a capital that mostly wants none
(capital obsidian 9.7 and 15.0 on the civs that hold any). Lines can't move obsidian further
than main by anything the sweep could see. **Carry is kept, unbuilt Lines is dropped.**
