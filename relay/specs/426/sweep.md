# #426 sweep: what a clan carries is what its days carry

`sansfiat sweep --judge` on main (59cd80f, with `--floor`) and on the branch, seven retro maps, every civ
on the AI. Files: `main-sweep.json`, `branch-sweep.json`; `table.py` makes the tables
(`python3 table.py main main-sweep.json branch branch-sweep.json`). `sansfiat judge --compare
main-sweep.json branch-sweep.json` gives every metric against main's noise floor.

The judge gained one count for this piece, `capital_towns_200` (towns at +200 on a civ's capital
market); main's numbers were made with the same judge patched in, which changes nothing in play.

## The Goal's scores

| score | main | branch | floor |
|---|---|---|---|
| towns at +200 | 10 | **17** | 1 |
| capital towns at +200 | 5 | **7** | 0 |
| clans living in towns at +200 | 59 | 72 | |
| towns formed after the pick | 23 | 29 | 0 |
| maps with a town | 4 | 5 | 0 |
| trade a person a turn, town clans | 0.043 | **0.144** | 0.003 |
| trade a person a turn, farmers outside towns | 0.038 | 0.048 | 0.006 |
| trade a person a turn, all clans | 0.079 | 0.073 | |
| markets alive at +200 | 205 | 212 | 7 |
| clans at a market at +200 | 0.924 | 0.936 | 0.005 |
| health: panics, digest mismatches | 0, 0 | 0, 0 | 0 |
| health: pre-games ended by centres | 7 | 7 | 0 |

Farmers come to markets on their own: twice as many towns hold at +200 against main, on five maps of
seven, and two more of them are capitals. A town clan trades three times what a farmer outside a town
does (main: about the same). Festivals hold: more markets at +200, and more clans at them. Health is at
main's floor.

What it costs: clans that have no surplus bring no food, so the trade over all clans falls a little
(0.079 to 0.073), and starving rises a little: 2.6% of clan-turns against 2.2% (gatherers 4.2% against
3.7%; town clans 1.6% against 0.8%, most of it small 9's five towns; farmers outside towns fall, 0.6%
against 0.9%).

## By map

| map | run | towns 200 | capital towns 200 | town clans 200 | town traded/p | farmers outside traded/p | all traded/p | town starving | markets 200 | trading 200 |
|---|---|---|---|---|---|---|---|---|---|---|
| small 2 | main | 0 | 0 | 0 | 0.050 | 0.038 | 0.059 | 4 | 18 | 0.891 |
| small 2 | branch | 0 | 0 | 0 | 0.000 | 0.043 | 0.084 | 0 | 21 | 0.969 |
| small 3 | main | 0 | 0 | 0 | 0.000 | 0.053 | 0.060 | 0 | 13 | 0.941 |
| small 3 | branch | 1 | 0 | 3 | 0.049 | 0.167 | 0.064 | 0 | 17 | 0.980 |
| small 6 | main | 0 | 0 | 0 | 0.000 | 0.061 | 0.069 | 0 | 16 | 0.933 |
| small 6 | branch | 0 | 0 | 0 | 0.127 | 0.053 | 0.055 | 1 | 19 | 0.926 |
| small 9 | main | 1 | 1 | 3 | 0.038 | 0.035 | 0.053 | 9 | 20 | 0.912 |
| small 9 | branch | 5 | 2 | 19 | 0.119 | 0.057 | 0.076 | 114 | 18 | 0.936 |
| medium 8 | main | 2 | 1 | 10 | 0.037 | 0.038 | 0.074 | 2 | 33 | 0.965 |
| medium 8 | branch | 4 | 3 | 18 | 0.169 | 0.062 | 0.087 | 24 | 34 | 0.925 |
| medium 9 | main | 4 | 2 | 21 | 0.034 | 0.044 | 0.104 | 0 | 30 | 0.870 |
| medium 9 | branch | 2 | 1 | 8 | 0.070 | 0.028 | 0.039 | 23 | 34 | 0.871 |
| large 8 | main | 3 | 1 | 25 | 0.050 | 0.033 | 0.082 | 63 | 75 | 0.956 |
| large 8 | branch | 5 | 1 | 24 | 0.168 | 0.039 | 0.077 | 3 | 69 | 0.946 |
| total | main | 10 | 5 | 59 | 0.043 | 0.038 | 0.079 | 78 | 205 | 0.924 |
| total | branch | 17 | 7 | 72 | 0.144 | 0.048 | 0.073 | 165 | 212 | 0.936 |

## The variants weighed

The last row of each: the branch as built ("branch"), a clan bringing loads from its whole basket
("whole basket", its food to eat included), and a clan bringing its first load whatever it has and only
its surplus past it ("first load + surplus").

| map | run | towns 200 | capital towns 200 | town clans 200 | town traded/p | farmers outside traded/p | all traded/p | town starving | markets 200 | trading 200 |
|---|---|---|---|---|---|---|---|---|---|---|
| total | main | 10 | 5 | 59 | 0.043 | 0.038 | 0.079 | 78 | 205 | 0.924 |
| total | branch | 17 | 7 | 72 | 0.144 | 0.048 | 0.073 | 165 | 212 | 0.936 |
| total | whole basket | 12 | 4 | 40 | 0.231 | 0.074 | 0.155 | 210 | 208 | 0.942 |
| total | first load + surplus | 14 | 5 | 61 | 0.181 | 0.073 | 0.155 | 222 | 205 | 0.924 |

- **Whole basket**: a town clan trades most (0.23), but it brings the food it needs and the clearing at
  ratios hands some back as less food: town clans starve at 3.4% of their turns, towns at +200 rise only
  to 12, and capital towns fall to 4. The Decided line says a clan at the door brings its *surplus*.
- **First load + surplus**: one map's pre-game no longer ends by centres (health 7 to 6), and aligned
  share at +200 falls from 0.52 to 0.40. Ruled out by health.
- Before the home step counted the load's trade (the cap gone, homes valued by their walk alone), towns
  at +200 fell to 7: a clan outside a town now pays one walker where it paid one for every 2 food of its
  0.2 a person, so the walk alone pulled less to the door. That run is not kept; the step is in
  `progress.md`.
