# #156 spec: what the spec step ran

All runs on `main` at 929b84a (Fields), release build, on the Actions runner. Nothing here is committed to
`engine/`. The diffs are kept beside this file.

## Where a large turn goes (main)

`sansfiat bench --size large 8 --after 50`: **54.8 ms a turn** past the pick (two runs: 54.8, 55.2). This
runner is slower than the one `architecture.md`'s 23 ms was measured on, so every scale bound in this piece is
read against main measured in the same session.

A scratch timer round each `TURN` step (50 turns past the pick, mean ms a turn):

| Step | ms | What |
|---|---|---|
| units | 31.5 | Almost all of it the scouts: about 5 ms on a quiet turn and 150–350 ms on a turn a scout picks rich ground |
| go | 15.9 | `worths`' cost map from each clan's home 13.4 ms, `path` to the chosen festival 2.5 ms; 700 clans on 700 home tiles, 72 festivals |
| eat | 3.9 | dividing and daughters (`sites`, `halve`) |
| move | 2.3 | |
| everything else | < 1 each | `ai` (the rival paths) 0.12 ms, `trade` 0.31, `people` 0.18, `markets` (forming) 0.04 |

**The scouts.** `scout::round_plan` runs `walk_costs`, a cost map over the whole map, for each tile it values.
`rich_ground` values every tile within `SCOUT.wander`, so it runs dozens of whole-map searches. But
`round_plan` skips any tile whose walk there and back costs the labour it has (`walk >= left`), so a cost map
bounded at `labour ÷ (2 × people)` (+1 day of slack) gives the same plan. `scout-bound.diff` is that one line.
It took the bench from **54.8 to 24.1 ms a turn**, and scouts from 31.5 to 0.3 ms. `digest --state` is equal at
turns 448–450 on large 8 and 399–400 on small 3.

**The festival search.** A crude early exit in `worths` (stop the cost map once every festival within
`most ÷ DAY` hex steps is settled) took `go` from 15.9 to 10.7 ms. It moved the digest, because a trail makes a
step cheaper than `DAY`, so a festival further than `most ÷ DAY` hexes can still be in reach. An exit that is
digest-safe has to bound by the cheapest step a tile can cost. With the scouts' bound and that exit, the bench
was 19.4 ms a turn.

## Farmers and festivals today (main, +200, every civ on the AI)

| Map | Clans | Farmers | Farmer's home to its festival: 0 / 1 / 2 / 3+ steps / none | Festival walk, share of a farmer's days | Gatherer's | Festivals | on field ground |
|---|---|---|---|---|---|---|---|
| small 2 | 157 | 19 | 0 / 3 / 2 / 13 / 1 | 0.275 | 0.285 | 18 | 5 |
| small 3 | 98 | 9 | 0 / 1 / 4 / 4 / 0 | 0.262 | 0.250 | 15 | 1 |
| small 6 | 126 | 27 | 0 / 4 / 5 / 18 / 0 | 0.263 | 0.218 | 19 | 1 |
| small 9 | 135 | 37 | 1 / 9 / 15 / 10 / 2 | 0.233 | 0.229 | 18 | 4 |
| medium 8 | 309 | 46 | 1 / 8 / 9 / 27 / 1 | 0.297 | 0.275 | 33 | 2 |
| medium 9 | 199 | 26 | 0 / 2 / 5 / 12 / 7 | 0.223 | 0.221 | 37 | 4 |
| large 8 | 708 | 90 | 5 / 15 / 25 / 35 / 10 | 0.194 | 0.260 | 72 | 16 |

A quarter of every clan's days go to the walk to its festival. No two clans share a home. Most farmers live
three or more steps from their festival, and few festivals stand on field ground. So a town needs homes at the
market and fields on the tiles round it: "fields along the water and homes at the market".

## A sketch of towns (`probe.diff`)

About 40 lines in `clans::adjust`. After the usual home step, a farmer weighs stepping its home one tile toward
its festival's tile, onto any passable land tile, whoever holds it. It values the move with `worth` from the
new home: the territory with the new tile added and trimmed to the radius, and the festival route repriced
from there. It steps if that beats staying by 0.01 food a person. Claims are written only on unheld tiles, so a
home on another clan's tile doesn't take it. The sketch's flaws: a clan still gathers on its home tile when
another clan holds it, so a shared home tile is gathered twice; the field step and the town step both move
the home in one turn and can pull against each other; the festival search and `in_reach` still find homes
through the claim.

Towns counted as a tile with 3 or more homes and a festival on it (`probe_towns.rs.txt`, +200):

| Map | Clans | People | Towns | First town (turn past the pick) | Tiles with 2+ homes | Most homes on a tile | Town clans | Of them starving | Clans starving | Control: clans, people, starving |
|---|---|---|---|---|---|---|---|---|---|---|
| small 2 | 191 | 1223 | 2 | 8 | 19 | 8 | 7 | 0 | 14 | 157, 1107, 5 |
| small 3 | 100 | 511 | 0 | 9 | 3 | 3 | 0 | 0 | 6 | 98, 499, 1 |
| small 6 | 129 | 677 | 1 | 13 | 6 | 3 | 3 | 2 | 9 | 126, 659, 2 |
| small 9 | 167 | 1061 | 4 | 8 | 20 | 6 | 19 | 7 | 14 | 135, 1001, 0 |
| medium 8 | 332 | 1788 | 0 | 17 | 19 | 4 | 0 | 0 | 15 | 309, 1661, 2 |
| medium 9 | 240 | 1444 | 2 | 12 | 16 | 16 | 19 | 3 | 19 | 199, 1347, 5 |
| large 8 | 812 | 4910 | 9 | 2 | 51 | 11 | 41 | 5 | 49 | 708, 4458, 8 |

The control is the same sketch with the town step off: it equals main. Town clans walk nothing to their
festival, where other farmers spend 0.16–0.23 of their days.

`sweep --judge --json` with the sketch against main (`probe-compare.txt`):

| | Main | Sketch |
|---|---|---|
| Aligned Standing at +200 | 0.318 | 0.341 |
| People changed hands | 953 | 1420 |
| Land changed hands | 1150 | 2353 |
| Clans joined | 177 | 285 |
| Starving clan-turns against the no-fields control | 1.22 | 3.97 (every map 1.9–6.7) |
| Settled farmers starving | 0.13% | 6.9% |
| Settled farmers' home moves per 100 clan-turns | 0.03 | 7.8 |
| Maps where the top farmer holds a pack half its turns | 5 | 3 |
| Civs never at a pack | 1 | 4 |
| AI orders a hundred turns (claims) | 8.3 | 23.6 |
| Pre-games by centres, panics | 7, 0 | 7, 0 |
| `bench --size large 8 --after 50` | 54.7 ms | 75.2 ms |

So towns form, people and land move, and Standing rises. But town clans don't feed: starving runs at three to
four times the control. Settled farmers hop between their fields and the market. The crowd costs 37% more a
turn. Feeding the town is the canvas's work, on easels that remove the double gathering and give a
clan its share of the field ground.

## Every territory tile is its clan's claim today

Build-1's proof rests on this: if every tile of every territory is its own clan's claim, then "a clan gathers
only what it claims" changes nothing until a home is shared. A scratch check over 200 turns past the pick on
the seven retro maps, every clan, every turn:

| Map | Territory tiles checked | Not the clan's own claim | In two territories |
|---|---|---|---|
| small 2 | 77,765 | 0 | 0 |
| small 3 | 61,863 | 0 | 0 |
| small 6 | 79,424 | 0 | 0 |
| small 9 | 75,920 | 0 | 0 |
| medium 8 | 153,573 | 0 | 0 |
| medium 9 | 157,049 | 0 | 0 |
| large 8 | 356,454 | 0 | 0 |
