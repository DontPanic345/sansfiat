# #156 Towns: the review's judge

`sansfiat sweep --judge --json --floor` on the branch (merged with main at 79a9847) and on main (79a9847),
in one session on the Relay runner, then `sansfiat judge --compare main.json branch.json`.

## Verdict: better

The question (towns form where farmers and a festival meet, their clans trade more and walk less, a large
map runs under a bound) moves past the floor against the no-towns control on four maps, in motion.
Standing rises past the floor. Health stays in band. Two of the spec's bounds miss, narrowly (below).

## The towns block (against the no-towns control, same sweep)

- MISS: towns form: 14 towns at +200 over the seven (≥ 10), on 4 of 7 maps (≥ 5)
- pass: farmers meet the festival: town clans at +200 80, of them farmers 60 (≥ ¾), towns working fields 14 of 14
- pass: town clans walk less: walk per turn 0.000 vs farmers outside 0.194 (under half)
- pass: town clans trade more: traded per person a turn 0.0451 vs farmers outside 0.0357 (≥ 1.1 times)
- MISS: traded over all clans: 0.0601 vs the control's 0.0605, less the floor 0.0002
- pass: clans change hands: 218.0 vs 177.0, less the floor 21.0
- pass: people change hands: 1124.0 vs 953.3, less the floor 91.6
- pass: land changes hands: 1453.0 vs 1150.0, less the floor 205.0
- pass: settled farmers stay: settled home moves less town steps 0.027 per 100 (town steps 0.141); town clans' moves off 0.000; gatherers 0.111 (each under half)
- pass: the store: maps with the top farmer at a pack 6 vs control 5 (no worse by more than 1); civs never at a pack 1 vs 1 (no worse by more than 1)

Per map, towns at +200: small 2 2, small 3 0, small 6 1, small 9 0, medium 8 0, medium 9 2, large 8 9.

Starving over clan-turns after +100: town clans 0.07%, farmers outside towns 0.38%, all clans 0.0143 against
the control's 0.0132. Not past the floor: the floor of the control's starving count is 577 clan-turns over the seven
maps (one harmless extra roll at the pick), and the run has 546 more (4,113 of 287,053 clan-turns against 3,567 of 269,208, with 7% more clan-turns). Small 9, where build-4 left
the extra starving unexplained (195 starving clan-turns against the control's 7), has a floor of 251 on its
control's count. It is the drift of a world that diverges after the pick, not a rule.

## Scale

`sansfiat bench --size large 8 --after 50`, alternated in one session:

```
main   65.336 ms a turn
branch 25.530 ms a turn
main   65.215 ms a turn
branch 25.797 ms a turn
```

0.39 to 0.40 of main's, under the bound of 0.45.

## Towns off equals main

The branch's no-towns run (the seventh run) equals main's run in every map's `standing`, `doubt`,
`decisions`, `motion`, `land`, `economy`, `supply` and `fields` block: 56 of 56. The no-fields control
equals main's on every map. The no-claims and no-fight controls differ, as they should: they run with towns on.

## judge --compare, the flagged rows (main = A, branch = B)

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.3183      0.3280      0.0097      0.0005 *
doubt
  total.doubt.no_claims_maps_with_lead_change                 3.0000      4.0000      1.0000      0.0000 *
decisions
  total.decisions.ai_orders_per_100                           8.2603     12.6921      4.4317      2.9508 *
  total.decisions.by_kind.claim                               4.3159      9.0254      4.7095      3.0048 *
  total.decisions.by_kind.disband                             0.3095      0.1429     -0.1667      0.0238 *
  total.decisions.by_kind.raise                               3.6349      3.5238     -0.1111      0.0778 *
motion
  total.motion.clans_changed                                177.0000    218.0000     41.0000     24.0000 *
  total.motion.clans_joined                                 177.0000    219.0000     42.0000     25.0000 *
  total.motion.land_changed                                1150.0000   1453.0000    303.0000    205.0000 *
  total.motion.people_changed                               953.3499   1123.9574    170.6075     91.6198 *
land
  total.land.flips_per_100                                    0.0871      0.1041      0.0170      0.0152 *
  total.land.held_200                                        34.0000     33.1905     -0.8095      0.4286 *
  total.land.homes_trailed.1                                  0.8163      0.8702      0.0539      0.0109 *
  total.land.trading_200                                      0.9172      0.9350      0.0178      0.0066 *
  total.land.worked_trailed.1                                 0.3122      0.3219      0.0097      0.0051 *
economy
  total.economy.turns_at_pack                               524.0000    546.1429     22.1429      8.0000 *
armies
  total.armies.people_vs_control                              0.9672      0.9890      0.0218      0.0083 *
fields_vs_control
  total.fields_vs_control.ai_orders_per_100                   8.2603     12.6921      4.4317      2.9508 *
  total.fields_vs_control.aligned_200                         0.3183      0.3280      0.0097      0.0005 *
  total.fields_vs_control.aligned_gain                        0.0658      0.0754      0.0097      0.0053 *
  total.fields_vs_control.clans_changed                     177.0000    218.0000     41.0000     24.0000 *
  total.fields_vs_control.clans_joined                      177.0000    219.0000     42.0000     25.0000 *
  total.fields_vs_control.farming_gaining                    12.0000     14.0000      2.0000      1.0000 *
  total.fields_vs_control.farming_over_others                 0.0493      0.0561      0.0068      0.0036 *
  total.fields_vs_control.land_changed                     1150.0000   1453.0000    303.0000    205.0000 *
  total.fields_vs_control.others_gain                        -0.0086     -0.0123     -0.0037      0.0022 *
  total.fields_vs_control.people_changed                    953.3499   1123.9574    170.6075     91.6198 *
  total.fields_vs_control.settled_home_moves_per_100          0.0307      0.1675      0.1368      0.0031 *
supply
  total.supply.counts.scout_outside                        1416.0000   1566.0000    150.0000     77.0000 *
listed 164, changed 62, flagged 29, only in A 0, only in B 50
```

`settled_home_moves_per_100` (#155's farmers-stay count, counted whole) rises 0.031 → 0.168 against
gatherers' 0.111: the town steps. The spec counts it less town steps (0.027, a pass), as spec-review set it.

---

# #156 build-3: the judge's towns block, and the sweep's time

Run on this runner (release build) with `sansfiat sweep --judge --json --floor`, before (build-2, 561d8a1) and after (build-3). The table gives the sweep's own clock (`seconds` in the report) and the wall time of the whole command, which includes the floor's nulls:

| | sweep's seconds | wall time |
|---|---|---|
| before (561d8a1) | 44.4 | 88.2 |
| after (build-3) | 52.6 | 105.3 |

The seventh run (no towns) adds one run in six: about 17% more time on the wall clock, 18% on the sweep's clock.

Every block of the report except the towns keys equals before's, on each map and in the total: `standing`, `motion`, `economy`, `land`, `fields`, `supply`, `decisions`, `doubt`, `end`, `fight` and `armies`. The floors of every path that existed before are unchanged. Panics 0, digest mismatches 0, pre-games by centres 7 of 7.

## The towns block (the text, with the floor)

```
towns (against the no-towns control, Config.towns off):
  small  seed   2  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.245; traded 0.0000 vs 0.0390; starving 0.0251 vs control 0.0251
  small  seed   3  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.281; traded 0.0000 vs 0.0658; starving 0.0127 vs control 0.0127
  small  seed   6  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.241; traded 0.0000 vs 0.0470; starving 0.0019 vs control 0.0019
  small  seed   9  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.220; traded 0.0000 vs 0.0400; starving 0.0005 vs control 0.0005
  medium seed   8  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.300; traded 0.0000 vs 0.0469; starving 0.0116 vs control 0.0116
  medium seed   9  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.213; traded 0.0000 vs 0.0331; starving 0.0244 vs control 0.0244
  large  seed   8  towns at +200 0 (clans 0, farmers 0, fields 0), formed 0, most homes 1; walk 0.000 vs farmers 0.196; traded 0.0000 vs 0.0284; starving 0.0124 vs control 0.0124
  towns form: 0 towns at +200 over the seven (≥ 10), on 0 of 7 maps (≥ 5) MISS
  farmers meet the festival: town clans at +200 0, of them farmers 0 (≥ ¾), towns working fields 0 of 0 MISS
  town clans walk less: walk per turn 0.000 vs farmers outside 0.227 (under half) MISS
  town clans trade more: traded per person a turn 0.0000 vs farmers outside 0.0362 (≥ 1.1 times) MISS
  traded over all clans: 0.0605 vs the control's 0.0605, less the floor 0.0000 pass
  clans change hands: 177.0 vs 177.0, less the floor 24.0 pass
  people change hands: 953.3 vs 953.3, less the floor 89.1 pass
  land changes hands: 1150.0 vs 1150.0, less the floor 79.0 pass
  settled farmers stay: settled home moves less town steps 0.029 per 100 (town steps 0.002); town clans' moves off 0.000; gatherers 0.122 (each under half) MISS
  the store: maps with the top farmer at a pack 5 vs control 5 (no worse by more than 1); civs never at a pack 1 vs 1 (no worse by more than 1) pass
  health: town clans starving 0.0000, farmers outside towns 0.0054, the control's 0.0132 (reported, not bound); lead changes and the spread: reported
```


With no home shared (build-4 shares them), every map reads 0 towns, run and control equal, so the towns bounds miss, as the brief expects: towns form, farmers meet the festival, town clans walk less and trade more. Settled farmers stay misses only on town clans' moves off, which has no town clans to count; its settled part passes (0.029 per 100, under half the gatherers' 0.122). The bounds that hold are the rows that need no town: the traded and the changes of hands against the control, and the store.
