# #156 build-4 (canvas): towns that form and feed

## The landed setting's towns block

From `sansfiat sweep --judge --floor` on the landed commit, against the no-towns control in the same sweep:

```
towns (against the no-towns control, Config.towns off):
  small  seed   2  towns at +200 2 (clans 12, farmers 9, fields 2), formed 2, most homes 9; walk 0.000 vs farmers 0.206; traded 0.0497 vs 0.0456; starving 0.0187 vs control 0.0251
  small  seed   3  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 2; walk 0.000 vs farmers 0.209; traded 0.0000 vs 0.0600; starving 0.0087 vs control 0.0127
  small  seed   6  towns at +200 1 (clans 4, farmers 3, fields 1), formed 1, most homes 4; walk 0.000 vs farmers 0.234; traded 0.0306 vs 0.0422; starving 0.0104 vs control 0.0019
  small  seed   9  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 4; walk 0.000 vs farmers 0.204; traded 0.0000 vs 0.0396; starving 0.0141 vs control 0.0005
  medium seed   8  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 2; walk 0.000 vs farmers 0.232; traded 0.0000 vs 0.0403; starving 0.0116 vs control 0.0116
  medium seed   9  towns at +200 2 (clans 14, farmers 12, fields 2), formed 2, most homes 9; walk 0.000 vs farmers 0.189; traded 0.0484 vs 0.0339; starving 0.0220 vs control 0.0244
  large  seed   8  towns at +200 9 (clans 50, farmers 36, fields 9), formed 14, most homes 10; walk 0.000 vs farmers 0.175; traded 0.0445 vs 0.0308; starving 0.0137 vs control 0.0124
  towns form: 14 towns at +200 over the seven (≥ 10), on 4 of 7 maps (≥ 5) MISS
  farmers meet the festival: town clans at +200 80, of them farmers 60 (≥ ¾), towns working fields 14 of 14 pass
  town clans walk less: walk per turn 0.000 vs farmers outside 0.194 (under half) pass
  town clans trade more: traded per person a turn 0.0451 vs farmers outside 0.0357 (≥ 1.1 times) pass
  traded over all clans: 0.0601 vs the control's 0.0605, less the floor 0.0002 MISS
  clans change hands: 218.0 vs 177.0, less the floor 21.0 pass
  people change hands: 1124.0 vs 953.3, less the floor 91.6 pass
  land changes hands: 1453.0 vs 1150.0, less the floor 205.0 pass
  settled farmers stay: settled home moves less town steps 0.027 per 100 (town steps 0.141); town clans' moves off 0.000; gatherers 0.111 (each under half) pass
  the store: maps with the top farmer at a pack 6 vs control 5 (no worse by more than 1); civs never at a pack 1 vs 1 (no worse by more than 1) pass
  health: town clans starving 0.0007, farmers outside towns 0.0038, the control's 0.0132 (reported, not bound); lead changes and the spread: reported
```

Two bounds miss: **towns form** (14 towns over the seven, which passes, but on 4 of 7 maps, not 5) and
**traded over all clans** (0.0601 against the control's 0.0605 less a floor of 0.0002). Aligned Standing at +200
is 0.328 against the control's 0.318 (floor 0.0005), which passes. Pre-games by centres 7 of 7, panics 0,
digest mismatches 0.

## The rule

`homes::step`, called from `clans::adjust` with `Config.towns` on and fields open, weighs one home step a turn
for a farmer that isn't its civ's own clan: staying, the step toward its busiest field (`fields::home_toward`),
or the step one tile toward its festival's tile onto any passable land. Each option's territory is the changed
territory with the new home in it, less tiles another clan holds (other than the home), trimmed to the radius.
Its festival route is priced from the new home by build-2's `festival_costs`, cached per tile for the turn
(`homes::Prices`). A step is taken when it beats staying by `TOWN.margin` (0.01), the better of the two if
both do. With towns off the rule is not called, and the move step is today's.

Three changes to rules came out of the painting, each recorded as a decision:

1. **A clan divides only the land it holds** (`dividing::divide_land`, `clans::eat`). The first sweep (a1) put
   clusters of 5 to 10 homes on tiles with no festival, 1 to 4 steps short of it. These were farmers that had
   stepped onto a held tile and then divided. `halve` gave the daughter the one tile they held and left the
   mother on a home it didn't hold, gathering nothing and unable to step on (its next tile had no ground). Now
   a home another clan holds is not land to divide. A clan divides when it holds 2 or more tiles, the daughter
   takes the half away from home, and otherwise it splits. With towns off every home is its own claim, so
   nothing changes.
2. **The home step weighs what its labour could reap, not the store's cap** (`clans::could_reap`). With the
   cap, a farmer at its store target gains nothing from a shorter walk, so its value from a step toward the
   festival equalled staying's exactly (the probe: route 4.0 to 2.0, value 0.9564 both ways, in about a third
   of the weighings). The surplus that "no longer walks to market" was invisible. Weighing the uncapped
   harvest counts the labour a shorter walk frees.
3. **A clan on a shared home weighs its step to the middle the same way** (`homes::step`). Setting a2 had
   town clans moving off their town at 0.39 per 100 (half the gatherers' rate is 0.066). These were clans
   whose field share notched under `FIELD.farmer` and the tile's original holder, re-centring to the middle
   of their land every time it changed. Now a clan that isn't a farmer but lives on a tile with another home,
   or on a tile it doesn't hold, steps to the middle only when that feeds it better by the margin. Elsewhere it
   re-centres as before.

## Every setting run

Each row is one `sweep --judge --json --floor`. "Traded all" is per person a turn over all clans. Moves are
per 100 clan-turns. The bounds: settled other moves and town moves off are each under half the gatherers'
rate; store is (maps with the top farmer at a pack, civs never at a pack), control (5, 1).

| Setting | What | Towns +200 (maps) | Farmers of town clans | Walk town / outside | Traded town / outside | Traded all / control | Hands: clans, people, land (control 177, 953, 1150) | Settled other moves / town moves off / gatherers (per 100) | Store | Starving town / outside / all / control |
|---|---|---|---|---|---|---|---|---|---|---|
| a1 | the home rule as the brief writes it (worth, capped at the store target) | 6 (4) | 23/30 | 0.000 / 0.192 | 0.0479 / 0.0394 | 0.0606 / 0.0605 | 195, 1024, 1223 | 0.067 / 0.232 / 0.106 | 4, 2 | 0.0177 / 0.0070 / 0.0145 / 0.0132 |
| a2 | + divide_land halves only held land; the step weighs what labour could reap | 12 (4) | 45/59 | 0.004 / 0.184 | 0.0418 / 0.0366 | 0.0598 / 0.0605 | 204, 1078, 1456 | 0.069 / 0.391 / 0.133 | 5, 1 | 0.0033 / 0.0028 / 0.0107 / 0.0132 |
| a3 | + clans on a shared home weigh their step to the middle | 14 (4) | 60/80 | 0.000 / 0.194 | 0.0451 / 0.0357 | 0.0601 / 0.0605 | 218, 1124, 1453 | 0.027 / 0.000 / 0.111 | 6, 1 | 0.0007 / 0.0038 / 0.0143 / 0.0132 |
| m0.005 | a3, TOWN.margin 0.005 | 14 (4) | 57/75 | 0.001 / 0.189 | 0.0444 / 0.0359 | 0.0599 / 0.0605 | 196, 1039, 1289 | 0.072 / 0.029 / 0.131 | 6, 1 | 0.0026 / 0.0050 / 0.0144 / 0.0132 |
| m0.02 | a3, TOWN.margin 0.02 | 13 (4) | 58/71 | 0.001 / 0.177 | 0.0483 / 0.0350 | 0.0593 / 0.0605 | 184, 993, 1224 | 0.012 / 0.018 / 0.125 | 6, 2 | 0.0008 / 0.0022 / 0.0108 / 0.0132 |
| r3 | a3, a farmer at its festival holds radius 3 | 12 (4) | 48/65 | 0.000 / 0.189 | 0.0436 / 0.0362 | 0.0598 / 0.0605 | 217, 1121, 1528 | 0.033 / 0.013 / 0.104 | 6, 1 | 0.0007 / 0.0037 / 0.0121 / 0.0132 |

a3 is the landed rule. Its sweep was rerun on the landed commit, which adds only the floor key fix below, and
reads the same. The bench was measured on the landed commit. Margins of 0.005 and 0.02 form no more towns:
0.02 moves the fifth map's town from small 6 to small 9 and loses traded. Radius 3 for a farmer living at
its festival (the spec's Unsettled allows it) made towns rarer and cut the farmers' share of town clans to
48 of 65, so it was reverted.

**A fix to build-3's judge.** `sweep_towns::bounds` read the floor of "traded over all clans" from
`total.towns_vs_control.all.traded_per_turn`, a path that doesn't exist, so it was always 0. It now reads
`run.all.traded_per_turn` (0.0002 here).

## Who starves round a town, and why

From a probe of the landed rule: every map, every clan-turn from +101 to +200, each clan classed by where it
lives at the turn's end. "Holds ≤1 tile" is a clan gathering at most one tile.

```
towns true: clan-turns after +100, starving, share
  daughter of a town clan: 3838 118 0.0307
  elsewhere, farmer: 21739 115 0.0053
  elsewhere, gatherer: 137489 2390 0.0174
  town clan: 7746 6 0.0008
  within 2 of a town, farmer: 4982 6 0.0012
  within 2 of a town, gatherer: 6966 137 0.0197
towns false: clan-turns after +100, starving, share
  elsewhere, farmer: 26262 146 0.0056
  elsewhere, gatherer: 146168 2241 0.0153
```

- **Town clans hardly starve** (6 of 7,746 clan-turns, 0.08%; the judge reads 10 of 13,416, 0.07%). They work
  the shared ground of the town's tile and walk nothing to market. Farmers outside towns starve at 0.38%, about
  as farmers do in the control (0.56% in this probe).
- **Daughters of town clans** starve at 3.1%, about 4% of those holding one tile. A town clan holds the town's
  tile or none, so it usually splits rather than divides. Its daughter goes to an unheld site within
  `scout_range`, which round a town is crowded. Gatherers anywhere holding one tile starve at the same rate
  (3.7–4.0%). This is a new clan's start in a crowd, the world's consequence (`docs/process.md`, Health).
- **Gatherers within two steps of a town** starve at 2.0%, against 1.7% elsewhere in the same runs and 1.5% for
  gatherers in the control. The towns run has 7% more clan-turns (287,053 against 269,208), so more people want
  the same land. That is crowding from growth.
- **Not explained: small 6 and small 9.** Small 9 has no town and starves at 0.0141 against its control's 0.0005.
  The extra starving falls on gatherers far from any shared home (181 of 9,325 clan-turns against 6 of 9,718).
  The run's world diverges from the control after the pick, and I didn't find the rule behind it in time. On
  small 6, one or two gatherers living on a shared home they don't hold (not a town) starve 48 of their 56
  clan-turns. They were farmers that stepped into a crowd and lost their field share, and neither `move_on` nor
  the home step finds them better ground. Whether that is a misjudged step or crowding isn't settled, so bound
  7 is not shown to hold for these two maps.

## Why town clans trade more but all clans don't

Town clans trade 0.0451 a person a turn against 0.0357 for farmers outside towns (1.26 times). Towns' festivals
have bigger crowds, and what the clearing swaps grows with the crowd. Over all clans the run trades 0.0601
against the control's 0.0605. Most town clans are farmers (60 of 80), and farmers bring mostly grain, which the clearing swaps less of than a
gatherer's mixed basket (farmers 0.036, all clans 0.060). My reading is that trading falls with the share of grain brought, not
from any town rule, but I haven't measured it. Setting a1, which had the divide fault and weighed the capped value, traded 0.0606. It
formed only 6 towns and starved town clans at 1.8%.

## The landed setting against main

`judge --compare main.json branch.json` (main's sweep in the same session), the headline rows:

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.3183      0.3280      0.0097      0.0005 *
  total.standing.on_land_200                                  0.0000      0.0000      0.0000      0.0000
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                177.0000    218.0000     41.0000     24.0000 *
  total.motion.clans_joined                                 177.0000    219.0000     42.0000     25.0000 *
  total.motion.clans_left                                     1.0000      2.0000      1.0000      1.0000
  total.motion.land_changed                                1150.0000   1453.0000    303.0000    205.0000 *
  total.motion.people_changed                               953.3499   1123.9574    170.6075     91.6198 *
fields_vs_control
  total.fields_vs_control.ai_orders_per_100                   8.2603     12.6921      4.4317      2.9508 *
  total.fields_vs_control.aligned_200                         0.3183      0.3280      0.0097      0.0005 *
  total.fields_vs_control.aligned_gain                        0.0658      0.0754      0.0097      0.0053 *
  total.fields_vs_control.civs                               21.0000     21.0000      0.0000      0.0000
  total.fields_vs_control.civs_never_at_pack                  1.0000      1.0000      0.0000      1.0000
  total.fields_vs_control.clans_changed                     177.0000    218.0000     41.0000     24.0000 *
  total.fields_vs_control.clans_joined                      177.0000    219.0000     42.0000     25.0000 *
  total.fields_vs_control.control_ai_orders_per_100           6.9111      6.9111      0.0000      1.1937
  total.fields_vs_control.control_aligned_200                 0.2525      0.2525      0.0000      0.0048
  total.fields_vs_control.control_civs_never_at_pack          4.0000      4.0000      0.0000      1.0000
  total.fields_vs_control.control_clans_changed             127.0000    127.0000      0.0000     17.0000
  total.fields_vs_control.control_clans_joined              125.0000    125.0000      0.0000     16.0000
  total.fields_vs_control.control_land_changed             1022.0000   1022.0000      0.0000     28.0000
  total.fields_vs_control.control_lead_changes                9.0000      9.0000      0.0000      6.0000
  total.fields_vs_control.control_people_changed            629.3674    629.3674      0.0000     48.9407
  total.fields_vs_control.control_spread_200                  0.0497      0.0497      0.0000      0.0071
  total.fields_vs_control.control_starving_share              0.0178      0.0178      0.0000      0.0005
  total.fields_vs_control.farming_civs                       13.0000     14.0000      1.0000      1.0000
  total.fields_vs_control.farming_gain                        0.0407      0.0438      0.0032      0.0058
  total.fields_vs_control.farming_gaining                    12.0000     14.0000      2.0000      1.0000 *
  total.fields_vs_control.farming_over_others                 0.0493      0.0561      0.0068      0.0036 *
  total.fields_vs_control.gatherer_home_moves_per_100         0.1221      0.1114     -0.0107      0.0389
  total.fields_vs_control.gatherer_moved_on_per_100           0.0511      0.0454     -0.0057      0.0212
  total.fields_vs_control.gatherer_starving_share             0.0226      0.0242      0.0016      0.0032
  total.fields_vs_control.land_changed                     1150.0000   1453.0000    303.0000    205.0000 *
  total.fields_vs_control.lead_changes                        4.0000      4.0000      0.0000      3.0000
  total.fields_vs_control.maps_top_farmer_at_pack             5.0000      6.0000      1.0000      1.0000
  total.fields_vs_control.maps_with_top_farmer                7.0000      7.0000      0.0000      0.0000
  total.fields_vs_control.others_gain                        -0.0086     -0.0123     -0.0037      0.0022 *
  total.fields_vs_control.people_changed                    953.3499   1123.9574    170.6075     91.6198 *
  total.fields_vs_control.settled_home_moves_per_100          0.0307      0.1675      0.1368      0.0031 *
listed 164, changed 62, flagged 29, only in A 0, only in B 50
```

With towns off, every map's `standing`, `motion`, `economy`, `land`, `fields`, `supply`, `decisions` and
`doubt` blocks equal main's (56 blocks over the seven maps, checked key by key from the no-towns run against
main's sweep). Build-3 equalled main in the same blocks, so towns off equals build-3.

## The bench

`sansfiat bench --size large 8 --after 50`, main and the landed commit alternately in the same session:

```
main1: seed 8: world 107.6 ms, pre-game 4623.7 ms, 50 turns past the pick 3220.3 ms (64.405 ms a turn)
branch1: seed 8: world 108.2 ms, pre-game 2970.8 ms, 50 turns past the pick 1283.8 ms (25.676 ms a turn)
main2: seed 8: world 106.6 ms, pre-game 4634.8 ms, 50 turns past the pick 3222.8 ms (64.455 ms a turn)
branch2: seed 8: world 108.6 ms, pre-game 2979.8 ms, 50 turns past the pick 1289.5 ms (25.790 ms a turn)
```

The branch runs at 25.7 ms a turn against main's 64.4, which is **0.40 times**, under the 0.45 bound.
