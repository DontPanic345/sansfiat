# #341 spec: what the maps say before any build

Measured at 8d1b4ed (main, after #340) on the spec's runner. `sweep --judge` takes 2 m 55 s.
`towns.py` reads `sweep --judge --json` reports into the per-map table in `tables.txt`:

- `towns200`: towns at +200; `clans`: clans living in them; `formed`: `TownFormed` events.
- `trade_town`: food traded per person a turn by town clans, over the clan-turns after +100.
- `farm_out`: the same for farmers outside towns (the judge's `farmers_outside`).
- `outside`: the same for every clan outside a town (Σ traded ÷ Σ clan-turns of `all` less `town`).

## Main is not the issue's main

The issue's "13 towns on 4 maps, none on small 3, small 9 or medium 8" is from before M5. Main
now has **10 towns at +200 on 4 maps** (small 9, medium 8, medium 9, large 8), none on small 2, 3 or 6.
Five of the 10 stand on a capital (a probe that ran each map to +200 and read `towns()` against
`Civ.capital`): small 9 (472), medium 8 (1145), medium 9 (547, 780), large 8 (2156). **So a
capital's market already counts for the town rule** (it is in `World.markets`, which `towns()`
reads), and one small map, small 9, already has a capital town.

## Why farmers by a capital don't move in

A probe at +150 on small 6 weighed `homes::step` for the farmers one tile from civ 263's capital
(tile 378, no field ground), whose festival is the capital's market:

| clan | stay | onto the capital | festival walk there |
|---|---|---|---|
| 105 | 0.8142 | 0.8142 | 2.0 → 0 |
| 308 | 1.0008 | 1.0008 | 2.0 → 0 |
| 310 | 1.0340 | 1.0340 | 2.0 → 0 |

The walk it saves buys nothing in `could_reap`: the value with no route at all is the same to four
places. These clans' fields are capped by their ground (`fields::room`) and their gathering by the
food on their tiles, so freed days have nowhere to go. A step onto the festival ties with staying,
and a tie loses to `TOWN.margin`. Farmers two or more tiles out lose their fields when they step
(the territory is trimmed to the radius around the new home), so they stay too.

The same probe found civ 263's own clan gone by +20. `civs::repair` passes the civ to its biggest
clan wherever that clan lives, and that clan is then pinned where it is, so the capital has no fixed
resident. Small 3's civ 198 lost its own clan the same way. On small 6 with rule A below, three
farmers live on civ 263's capital at +30 (a town), and one at +200: 316 left by +50, and 304 stopped
farming at +60 and left by +140.

Small 3's capitals and their rings have no field ground and no farmer within four tiles: no
farming rule makes a town there.

## The rules tried

| run | towns at +200 | maps with one | capital towns at +200 (small maps) | trade per person: town / outside |
|---|---|---|---|---|
| main | 10 | 4 | 5 (1: small 9) | 0.0427 / 0.0803 |
| **A**: the door step | **18** | **6** | 6 (1: small 9) | 0.0476 / 0.1006 |
| A + B: a clan on its festival's tile brings its whole basket | 11 | 5 | not read | 0.2413 / 0.1179 |
| A + C: `repair` passes the civ to a clan on the capital first | 18 | 6 | not read | 0.0493 / 0.1074 |

- **A** (`door.diff`): in `homes::step`, a step whose home is the clan's festival's tile is taken
  when it feeds the clan at least as well as staying and its festival walk from there is less,
  besides the usual "beats staying by `TOWN.margin`". The existing towns, fields and clans tests
  pass with it.
- **B** (in `markets::trade`, `most` unbounded for a clan whose home is the market's tile) makes town
  clans trade five times as much per person, but towns fall from 18 to 11. Not taken: see spec.md.
- **C** changes small 9 and medium 9 a little and nothing else at +200. Left to the canvas.

The towns A adds are all off a capital but one, large 8's 2689 (the probe's list at +200 under A:
small 2 531, small 6 82, small 9 151, medium 8 753, large 8 2778, 1380, 205 and 2689; medium 8's 178
grows from 7 to 10 clans). Under A, town clans still trade about half as much per person as clans
outside a town (0.048 against 0.101).
