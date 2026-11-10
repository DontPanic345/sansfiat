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
