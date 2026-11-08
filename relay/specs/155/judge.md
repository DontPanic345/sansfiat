# #155 build-2: the judge's fields group on build-1's rules

`sansfiat sweep --judge --json --floor` and `sansfiat sweep --judge --floor` on `relay/155` at build-2, with
build-1's rules and numbers (FIELD at the probe's y8, tame under fields). Both come from the same code, and the
sweep is deterministic, so the text's numbers are the JSON's. These are build-4's starting numbers.

## The text's `fields` block

Per map: each civ's farming share at +200, its Standing at +200 less the no-fields control's, and its store's
turns at a pack of its turns. The bounds are the spec's, marked pass or MISS. The starving ratios are read less their floor.

```
fields (against the no-fields control):
  small  seed   2  civs (share, gain, store turns at a pack): Red 0.13 +0.056 200/200; Blue 0.11 +0.101 0/200; Gold 0.00 -0.014 200/200  starving 0.0325 vs 0.0421 = 0.77 (floor 0.150)  settled 0.0006 gatherers 0.0345
  small  seed   3  civs (share, gain, store turns at a pack): Red 0.00 -0.009 190/200; Blue 0.12 +0.037 194/200; Gold 0.00 -0.011 200/200  starving 0.0253 vs 0.0176 = 1.44 (floor 0.412)  settled 0.0171 gatherers 0.0252
  small  seed   6  civs (share, gain, store turns at a pack): Red 0.19 +0.049 51/200; Blue 0.13 +0.019 200/200; Gold 0.03 -0.009 198/200  starving 0.0226 vs 0.0197 = 1.15 (floor 0.431)  settled 0.0024 gatherers 0.0250
  small  seed   9  civs (share, gain, store turns at a pack): Red 0.14 +0.057 170/200; Blue 0.12 +0.099 0/200; Gold 0.17 +0.011 49/200  starving 0.0235 vs 0.0142 = 1.66 (floor 0.844)  settled 0.0061 gatherers 0.0183
  medium seed   8  civs (share, gain, store turns at a pack): Red 0.13 +0.012 250/250; Blue 0.13 +0.028 95/250; Gold 0.00 -0.008 250/250  starving 0.0133 vs 0.0148 = 0.90 (floor 1.031)  settled 0.0040 gatherers 0.0122
  medium seed   9  civs (share, gain, store turns at a pack): Red 0.13 +0.070 180/250; Blue 0.00 -0.015 4/250; Gold 0.00 -0.007 248/250  starving 0.0316 vs 0.0189 = 1.67 (floor 0.287)  settled 0.0052 gatherers 0.0317
  large  seed   8  civs (share, gain, store turns at a pack): Red 0.12 +0.011 275/300; Blue 0.10 +0.006 300/300; Gold 0.11 +0.002 300/300  starving 0.0197 vs 0.0152 = 1.29 (floor 0.029)  settled 0.0008 gatherers 0.0219
  standing: farming civs 13 of 21, gain +0.0426 (13 gaining), others -0.0082, over others +0.0508: gain ≥ +0.01 pass, ¾ gaining pass, over others ≥ 0.01 pass
  aligned at +200 0.3221 vs 0.2525, gain +0.0696 (floor 0.001): past the floor pass
  store: top farmer at a pack on half its turns on 4 of 7 maps (≥ 5 MISS); civs never at a pack 2 (control 4) (≤ 2 pass)
  motion: home moves per 100 settled 0.066 gatherers 0.138 (under half pass); moves on settled 0.005 gatherers 0.038 (no more pass)
  changed hands: clans 182 vs 127 pass, people 979.5 vs 629.4 pass, land 1195 vs 1022 pass; joined 195 vs 125
  starving: 0.0215 vs 0.0178 = 1.209 (floor 0.079), read 1.130 (≤ 1.25 pass); worst map read 1.385 (floor 0.287) (≤ 1.6 pass); settled 0.0027 gatherers 0.0223 (lower pass)
  AI orders per 100 8.44 vs 6.91 (≥ half pass); lead changes 4 vs 9, spread +200 0.101 vs 0.050 (not bound)
```

## `total.fields_vs_control`, with each number's floor

| Path | Value | Floor |
|---|---|---|
| `civs` | 21 | 0 |
| `farming_civs` | 13 | 1 |
| `farming_gain` | 0.0426 | 0.0026 |
| `farming_gaining` | 13 | 2 |
| `others_gain` | -0.0082 | 0.0018 |
| `farming_over_others` | 0.0508 | 0.0008 |
| `aligned_200` | 0.3221 | 0.0053 |
| `control_aligned_200` | 0.2525 | 0.0048 |
| `aligned_gain` | 0.0696 | 0.0006 |
| `maps_top_farmer_at_pack` | 4 | 1 |
| `maps_with_top_farmer` | 7 | 0 |
| `civs_never_at_pack` | 2 | 0 |
| `control_civs_never_at_pack` | 4 | 1 |
| `settled_home_moves_per_100` | 0.0660 | 0.0193 |
| `gatherer_home_moves_per_100` | 0.1382 | 0.0309 |
| `settled_moved_on_per_100` | 0.0051 | 0.0041 |
| `gatherer_moved_on_per_100` | 0.0384 | 0.0159 |
| `starving_share` | 0.0215 | 0.0008 |
| `control_starving_share` | 0.0178 | 0.0005 |
| `starving_ratio` | 1.2086 | 0.0785 |
| `worst_map_starving_ratio` | 1.6719 | 0.8306 |
| `settled_starving_share` | 0.0027 | 0.0004 |
| `gatherer_starving_share` | 0.0223 | 0.0002 |
| `clans_changed` | 182 | 4 |
| `control_clans_changed` | 127 | 17 |
| `people_changed` | 979.5178 | 20.3751 |
| `control_people_changed` | 629.3674 | 48.9407 |
| `land_changed` | 1195 | 118 |
| `control_land_changed` | 1022 | 28 |
| `clans_joined` | 195 | 14 |
| `control_clans_joined` | 125 | 16 |
| `ai_orders_per_100` | 8.4381 | 0.6254 |
| `control_ai_orders_per_100` | 6.9111 | 1.1937 |
| `lead_changes` | 4 | 0 |
| `control_lead_changes` | 9 | 6 |
| `spread_200` | 0.1012 | 0.0027 |
| `control_spread_200` | 0.0497 | 0.0071 |

## Against spec-review's "y8, tame 1"

| Count | spec-review (the probe, y8 with tame) | build-2 (build-1's rules) |
|---|---|---|
| Farming civs: n, mean gain, gaining | 13, +0.0377, 12 | 13, +0.0426, 13 |
| Others' mean gain | −0.0083 | −0.0082 |
| Civs never at a pack (control) | 1 (4) | 2 (4) |
| Starving against the control | 1.312 | 1.209 (floor 0.079) |
| Control's starving share | 1.77% | 1.78% |
| Settled farmers: home moves / 100, starving | 0.017, 0.23% | 0.066, 0.27% |
| Gatherers: home moves / 100, starving | 0.232, 2.93% | 0.138, 2.23% |
| Aligned at +200 | 0.253 → 0.312 (y8) | 0.2525 → 0.3221 |

Most counts sit near spec-review's. The one far from it is settled farmers' home moves: 0.066 against 0.017.
That gap comes from build-1's rules, not from the count. `judge_fields` defines settled farmers, gatherers and home
moves line for line as probe-2's `Probe` does: a home move is a home that differs from the turn before, a settled
farmer had `FIELD.farmer` or more of its days on fields at the end of each of the ten turns before, and a gatherer
had no fields the turn before. A throwaway trace of every settled farmer's home move over the sweep's fields-on runs
(the main run and the no-army, no-claims and no-fight controls; not committed) found 174 distinct moves over 108 clans. 64 of the clans moved once, and
7 moves went back to the tile just left. So it isn't oscillation. Build-1's farmer home follows the field tile with
the most field days on the turn's split (`fields::home_toward`), where the probe's followed ground times yield.
As the share notches between 0.4 and 0.5, the best tile changes, and a farmer settled for ten turns takes another
step. Gatherers move less than in the probe as well (0.138 against 0.232). Bound 4 still passes (0.066 under half of 0.138),
but the floors (0.019 and 0.031) leave it close.

Starving runs lower than spec-review's sweep: 1.21 against 1.31, and 1.13 read less its floor. The per-map
starving floors are large (medium 8: 1.03; small 9: 0.84), so the per-map bound of 1.6 reads mostly noise.
Store: the top farmer holds a pack on half its turns on 4 of 7 maps, against the bound's 5. Blue on small 2 and
small 9 farms most (or second most) and never holds a pack, and Red on small 6 holds one on 51 of 200 turns.

## Time

`sweep --judge --json` without the floor, the base sweep's `seconds`: 80.3 s on main's five runs a map (build-1's
HEAD) and 88.2 s with the sixth, the no-fields control: 1.10 times, under the fifth more the brief expected, because the runs go side by side on four cores. With
`--floor` the whole command takes 2 min 42 s on six runs a map, against 2 min 20 s on five at build-1's HEAD
(during that run a test build used the cores for about half a minute).
