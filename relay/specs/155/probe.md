# #155 spec: the probe

The spec step built a sketch of the field job on main (85868ed) to settle what running could settle: `probe.diff`, against `engine/`. It is a sketch, not the build: the knobs read environment variables, the judge prints `PROBE` lines to stderr, and nothing is tested. Run as `sansfiat sweep --judge --json --floor`, with every civ on the AI, on the seven retro maps.

What it does: after founding, each clan splits its days left after its routes between gathering (over its territory, as now) and fields (over its territory's tiles with field ground), by a share on the clan (`fields`). The share moves 0.1 a turn toward whichever of ±0.1 feeds the clan better by `value`, by at least 0.01 food a person; a starving clan with fields drops 0.1. Field ground in days: a floodplain 20, a river tile 10 × min(river ÷ 8, 1), any land terrain. A field day yields `F_YIELD × fertility × (domestic(tame) − 1)` grain, from no stock and with no wear; a tile takes no more field days than its ground. The harvest budget (`CLAN.surplus_target`) caps fields and gathering together. A clan with fields steps its home toward its field tile of most ground × yield (`F_HOME`). Tame under fields: unchanged.

Runs: **main** (85868ed); **off**: the probe with `F_ON=0`; **on**: `F_YIELD=0.4`; **y8**: `F_YIELD=0.8`; **nohome**: `F_YIELD=0.4` with the home rule off. A civ farms when its clans put a tenth of their people's days to fields at +200. The judge's farmer for moves and starving is a clan with `fields ≥ 0.2`. Store turns at a pack count the turns a civ's store holds `units::pack_cap(UNIT.first)`, 15 food.

## Fields off is main

The `off` sweep's `total` block equals main's byte for byte. Fields open at the pick, so the pre-game is untouched, and the no-fields control differs from a fields run only from the pick, like the floor's null.

## y8: yield 0.8 (the clay's starting numbers)

| map | civ | farming share +200 | Standing +200 off | on | change | store turns at a pack off | on |
|---|---|---|---|---|---|---|---|
| small 2 | 424 | 0.122 | 0.2162 | 0.2497 | +0.0335 | 200 | 200 |
| small 2 | 425 | 0.126 | 0.2141 | 0.2910 | +0.0769 | 0 | 0 |
| small 2 | 426 | 0.000 | 0.0649 | 0.0469 | -0.0180 | 200 | 200 |
| small 3 | 247 | 0.029 | 0.0811 | 0.0729 | -0.0082 | 190 | 192 |
| small 3 | 248 | 0.094 | 0.1149 | 0.1598 | +0.0449 | 192 | 197 |
| small 3 | 249 | 0.000 | 0.0805 | 0.0737 | -0.0068 | 200 | 200 |
| small 6 | 245 | 0.194 | 0.1245 | 0.1647 | +0.0402 | 167 | 69 |
| small 6 | 246 | 0.133 | 0.0931 | 0.1071 | +0.0140 | 200 | 200 |
| small 6 | 247 | 0.030 | 0.0916 | 0.0830 | -0.0086 | 198 | 198 |
| small 9 | 309 | 0.148 | 0.0657 | 0.1334 | +0.0677 | 0 | 189 |
| small 9 | 310 | 0.124 | 0.1111 | 0.1906 | +0.0795 | 0 | 0 |
| small 9 | 311 | 0.185 | 0.0965 | 0.1024 | +0.0059 | 84 | 108 |
| medium 8 | 816 | 0.132 | 0.0826 | 0.0937 | +0.0111 | 25 | 250 |
| medium 8 | 817 | 0.136 | 0.0495 | 0.0756 | +0.0261 | 24 | 208 |
| medium 8 | 818 | 0.008 | 0.0568 | 0.0547 | -0.0021 | 250 | 250 |
| medium 9 | 845 | 0.177 | 0.0586 | 0.1297 | +0.0711 | 197 | 248 |
| medium 9 | 846 | 0.000 | 0.0483 | 0.0353 | -0.0130 | 4 | 4 |
| medium 9 | 847 | 0.000 | 0.0229 | 0.0167 | -0.0062 | 248 | 248 |
| large 8 | 1718 | 0.120 | 0.0288 | 0.0399 | +0.0111 | 0 | 278 |
| large 8 | 1719 | 0.083 | 0.0256 | 0.0282 | +0.0026 | 300 | 300 |
| large 8 | 1720 | 0.112 | 0.0404 | 0.0373 | -0.0031 | 300 | 300 |

farm: n=12 mean change +0.0362, gained 11

no: n=9 mean change -0.0017, gained 2

never at a pack: off 4 on 2 ; under 50 turns: off 7 on 3

| map | gatherer home moves / clan-turns | farmer (fields ≥ 0.2) | starving clan-turns off | on | field tiles +200 | farming share of people |
|---|---|---|---|---|---|---|
| small 2 | 103/26286 (0.39%) | 1/4505 (0.02%) | 1173 of 27919 | 980 of 30791 | 47 | 0.082 |
| small 3 | 34/17119 (0.20%) | 5/2207 (0.23%) | 314 of 17992 | 382 of 19326 | 19 | 0.040 |
| small 6 | 25/18907 (0.13%) | 15/5492 (0.27%) | 419 of 21510 | 443 of 24399 | 37 | 0.090 |
| small 9 | 43/19569 (0.22%) | 12/7068 (0.17%) | 320 of 22651 | 532 of 26637 | 60 | 0.115 |
| medium 8 | 118/65051 (0.18%) | 24/11626 (0.21%) | 1012 of 69101 | 1547 of 76677 | 71 | 0.064 |
| medium 9 | 85/42327 (0.20%) | 17/6548 (0.26%) | 850 of 45213 | 1407 of 48875 | 52 | 0.081 |
| large 8 | 609/180516 (0.34%) | 28/29638 (0.09%) | 2910 of 191725 | 4709 of 210154 | 185 | 0.075 |

```
  total.standing.aligned_200                                  0.2525      0.3124      0.0598      0.0048 *
  total.standing.top_200                                      0.1069      0.1528      0.0459      0.0018 *
  total.doubt.lead_changes                                    9.0000      4.0000     -5.0000      6.0000
  total.doubt.no_claims_lead_changes                          6.0000      5.0000     -1.0000      1.0000
  total.doubt.spread_100                                      0.0551      0.0900      0.0349      0.0056 *
  total.doubt.spread_200                                      0.0497      0.0949      0.0453      0.0071 *
  total.decisions.ai_orders_per_100                           6.9111      7.3921      0.4810      1.1937
  total.motion.clans_joined                                 125.0000    172.0000     47.0000     16.0000 *
  total.motion.land_changed                                1022.0000   1105.0000     83.0000     28.0000 *
  total.motion.people_changed                               629.3674    915.3191    285.9517     48.9407 *
  total.economy.guard_mean                                    4.0548      7.1743      3.1195      0.0947 *
  total.economy.levy_per_turn                                 9.1784     13.3537      4.1753      0.3744 *
  total.economy.store_mean                                   25.8845     31.8751      5.9906      0.6958 *
  total.economy.turns_at_pack                               425.5714    548.4286    122.8571     10.7143 *
  total.health.panics                                         0.0000      0.0000      0.0000      0.0000
  total.health.pregame_by_centres                             7.0000      7.0000      0.0000      0.0000
  total.supply.clan_territory_200                             3.2752      2.9482     -0.3270      0.0047 *
```

Store flow at y8, a turn over the game (levy in, given, posted, guard people):

```
PROBE seed 2 r 16 civ 424 pack 200 st200 0.2497 flow Some([17.64, 0.17, 34.98, 9.78])
PROBE seed 2 r 16 civ 425 pack 0 st200 0.2910 flow Some([14.73, 0.0, 0.04, 14.18])
PROBE seed 2 r 16 civ 426 pack 200 st200 0.0469 flow Some([8.3, 0.05, 41.97, 2.0])
PROBE seed 3 r 16 civ 247 pack 192 st200 0.0729 flow Some([5.69, 0.04, 14.34, 1.0])
PROBE seed 3 r 16 civ 248 pack 197 st200 0.1598 flow Some([10.33, 0.06, 38.97, 3.92])
PROBE seed 3 r 16 civ 249 pack 200 st200 0.0737 flow Some([8.29, 0.02, 33.72, 2.9])
PROBE seed 6 r 16 civ 245 pack 69 st200 0.1647 flow Some([24.13, 0.04, 6.89, 18.07])
PROBE seed 6 r 16 civ 246 pack 200 st200 0.1071 flow Some([10.97, 0.02, 41.31, 2.81])
PROBE seed 6 r 16 civ 247 pack 198 st200 0.0830 flow Some([10.99, 0.01, 41.51, 2.0])
PROBE seed 8 r 23 civ 816 pack 250 st200 0.0937 flow Some([32.17, 0.05, 21.25, 22.89])
PROBE seed 8 r 23 civ 817 pack 208 st200 0.0756 flow Some([23.17, 0.12, 14.85, 17.75])
PROBE seed 8 r 23 civ 818 pack 250 st200 0.0547 flow Some([12.62, 0.35, 41.41, 2.85])
PROBE seed 8 r 33 civ 1718 pack 278 st200 0.0399 flow Some([6.83, 0.0, 8.98, 3.98])
PROBE seed 8 r 33 civ 1719 pack 300 st200 0.0282 flow Some([24.82, 0.24, 29.65, 15.02])
PROBE seed 8 r 33 civ 1720 pack 300 st200 0.0373 flow Some([14.54, 0.05, 40.6, 4.26])
PROBE seed 9 r 16 civ 309 pack 189 st200 0.1334 flow Some([9.11, 0.06, 39.44, 1.85])
PROBE seed 9 r 16 civ 310 pack 0 st200 0.1906 flow Some([7.57, 0.0, 0.98, 2.99])
PROBE seed 9 r 16 civ 311 pack 108 st200 0.1024 flow Some([16.45, 0.09, 6.08, 10.78])
PROBE seed 9 r 23 civ 845 pack 248 st200 0.1297 flow Some([16.34, 0.07, 33.91, 9.72])
PROBE seed 9 r 23 civ 846 pack 4 st200 0.0353 flow Some([2.6, 0.0, 0.14, 0.98])
PROBE seed 9 r 23 civ 847 pack 248 st200 0.0167 flow Some([3.15, 0.0, 5.27, 0.99])
```

## on: yield 0.4

| map | civ | farming share +200 | Standing +200 off | on | change | store turns at a pack off | on |
|---|---|---|---|---|---|---|---|
| small 2 | 424 | 0.152 | 0.2162 | 0.2379 | +0.0217 | 200 | 200 |
| small 2 | 425 | 0.215 | 0.2141 | 0.3144 | +0.1003 | 0 | 0 |
| small 2 | 426 | 0.000 | 0.0649 | 0.0510 | -0.0139 | 200 | 200 |
| small 3 | 247 | 0.029 | 0.0811 | 0.0771 | -0.0040 | 190 | 192 |
| small 3 | 248 | 0.115 | 0.1149 | 0.1428 | +0.0279 | 192 | 195 |
| small 3 | 249 | 0.000 | 0.0805 | 0.0754 | -0.0051 | 200 | 200 |
| small 6 | 245 | 0.199 | 0.1245 | 0.1502 | +0.0257 | 167 | 22 |
| small 6 | 246 | 0.117 | 0.0931 | 0.1026 | +0.0095 | 200 | 200 |
| small 6 | 247 | 0.040 | 0.0916 | 0.0851 | -0.0065 | 198 | 198 |
| small 9 | 309 | 0.228 | 0.0657 | 0.1339 | +0.0682 | 0 | 164 |
| small 9 | 310 | 0.238 | 0.1111 | 0.2183 | +0.1072 | 0 | 0 |
| small 9 | 311 | 0.220 | 0.0965 | 0.1027 | +0.0062 | 84 | 14 |
| medium 8 | 816 | 0.138 | 0.0826 | 0.0845 | +0.0019 | 25 | 42 |
| medium 8 | 817 | 0.225 | 0.0495 | 0.0849 | +0.0354 | 24 | 141 |
| medium 8 | 818 | 0.000 | 0.0568 | 0.0519 | -0.0049 | 250 | 250 |
| medium 9 | 845 | 0.224 | 0.0586 | 0.1121 | +0.0535 | 197 | 206 |
| medium 9 | 846 | 0.000 | 0.0483 | 0.0370 | -0.0113 | 4 | 4 |
| medium 9 | 847 | 0.000 | 0.0229 | 0.0175 | -0.0054 | 248 | 248 |
| large 8 | 1718 | 0.166 | 0.0288 | 0.0382 | +0.0094 | 0 | 280 |
| large 8 | 1719 | 0.115 | 0.0256 | 0.0229 | -0.0027 | 300 | 300 |
| large 8 | 1720 | 0.141 | 0.0404 | 0.0418 | +0.0014 | 300 | 300 |

farm: n=14 mean change +0.0333, gained 13

no: n=7 mean change -0.0073, gained 0

never at a pack: off 4 on 2 ; under 50 turns: off 7 on 6

| map | gatherer home moves / clan-turns | farmer (fields ≥ 0.2) | starving clan-turns off | on | field tiles +200 | farming share of people |
|---|---|---|---|---|---|---|
| small 2 | 33/22868 (0.14%) | 2/6876 (0.03%) | 1173 of 27919 | 719 of 29744 | 49 | 0.119 |
| small 3 | 49/16631 (0.29%) | 3/2254 (0.13%) | 314 of 17992 | 490 of 18885 | 16 | 0.044 |
| small 6 | 45/18690 (0.24%) | 27/5015 (0.54%) | 419 of 21510 | 438 of 23705 | 35 | 0.080 |
| small 9 | 74/18214 (0.41%) | 14/7740 (0.18%) | 320 of 22651 | 754 of 25954 | 60 | 0.154 |
| medium 8 | 80/61083 (0.13%) | 17/12380 (0.14%) | 1012 of 69101 | 1023 of 73463 | 70 | 0.085 |
| medium 9 | 82/40501 (0.20%) | 13/7019 (0.19%) | 850 of 45213 | 1334 of 47520 | 43 | 0.109 |
| large 8 | 229/166598 (0.14%) | 28/41307 (0.07%) | 2910 of 191725 | 3836 of 207905 | 188 | 0.108 |

```
  total.standing.aligned_200                                  0.2525      0.3117      0.0592      0.0048 *
  total.standing.top_200                                      0.1069      0.1521      0.0452      0.0018 *
  total.doubt.lead_changes                                    9.0000      4.0000     -5.0000      6.0000
  total.doubt.no_claims_lead_changes                          6.0000      3.0000     -3.0000      1.0000 *
  total.doubt.spread_100                                      0.0551      0.0874      0.0323      0.0008 *
  total.doubt.spread_200                                      0.0497      0.0940      0.0443      0.0071 *
  total.decisions.ai_orders_per_100                           6.9111      9.0016      2.0905      1.6825 *
  total.motion.clans_joined                                 125.0000    163.0000     38.0000     16.0000 *
  total.motion.land_changed                                1022.0000   1129.0000    107.0000     51.0000 *
  total.motion.people_changed                               629.3674    904.1024    274.7349     48.9407 *
  total.economy.guard_mean                                    4.0548      6.1796      2.1248      0.2281 *
  total.economy.levy_per_turn                                 9.1784     11.4659      2.2875      0.1188 *
  total.economy.store_mean                                   25.8845     29.2446      3.3601      1.1282 *
  total.economy.turns_at_pack                               425.5714    479.4286     53.8571      7.5714 *
  total.health.panics                                         0.0000      0.0000      0.0000      0.0000
  total.health.pregame_by_centres                             7.0000      7.0000      0.0000      0.0000
  total.supply.clan_territory_200                             3.2752      3.0399     -0.2354      0.0367 *
```

## nohome: yield 0.4, no home rule

| map | gatherer home moves / clan-turns | farmer (fields ≥ 0.2) | starving clan-turns off | on | field tiles +200 | farming share of people |
|---|---|---|---|---|---|---|
| small 2 | 58/22230 (0.26%) | 0/6537 (0.00%) | 1173 of 27919 | 663 of 28767 | 46 | 0.120 |
| small 3 | 44/17013 (0.26%) | 1/2331 (0.04%) | 314 of 17992 | 359 of 19344 | 16 | 0.044 |
| small 6 | 15/17718 (0.08%) | 1/5563 (0.02%) | 419 of 21510 | 402 of 23281 | 36 | 0.088 |
| small 9 | 41/18215 (0.23%) | 11/8443 (0.13%) | 320 of 22651 | 742 of 26658 | 60 | 0.153 |
| medium 8 | 55/61959 (0.09%) | 9/10963 (0.08%) | 1012 of 69101 | 1279 of 72922 | 66 | 0.080 |
| medium 9 | 56/42149 (0.13%) | 11/6642 (0.17%) | 850 of 45213 | 2158 of 48791 | 43 | 0.107 |
| large 8 | 189/163972 (0.12%) | 12/39515 (0.03%) | 2910 of 191725 | 3657 of 203487 | 188 | 0.106 |

## Bench

`sansfiat bench --size large 8 --after 50`: main 158.8 ms a turn past the pick; y8 90.8 ms. Farmers hold fewer tiles, and `change` weighs fewer.

## What it says

- **Standing.** At y8, 12 civs farm. They gain +0.036 Standing at +200 on average against `off`, and 11 of 12 gain. The 9 that don't farm lose −0.002, and 2 of 9 gain. Aligned Standing at +200 rises from 0.253 to 0.312, past a floor of 0.005. The question's first part holds with the probe's rules.
- **The store.** The proposal's "14 of 21 never hold a pack" is the M3 retro's count. On main today, 4 of 21 civs never hold one and 7 hold one on under 50 turns. At y8, 2 never do and 3 hold one on under 50 turns. The two that never do, small 2's 425 and small 9's 310, levy as much as their neighbours, but their guard eats it: 425 keeps 14 guards on 14.7 food a turn of levy. The store bound is the canvas's to meet. Its levers are the field rules: yield, ground, the notch and the farmer's line. The levy, the gifts and the guard work as now. If the field rules can't meet the bound, it sends an ask.
- **Moving.** Clans hardly move their homes after the pick: 0.1 to 0.4 moves per 100 clan-turns on main. The home rule makes a new farmer walk to its fields. At y8, farmers move more than gatherers on 4 of 7 maps. Without the rule, farmers move less on 6 of 7. The judge therefore counts settled farmers, those farming ten turns in a row, whose walk to the fields is behind them. The canvas tunes when a clan counts as a farmer.
- **Starving.** Clan-turns starving rise from 1.77% (`off`) to 2.01% (`on`) and 2.29% (y8). Most of it falls on gatherers squeezed beside farmers. Large 8 at y8 goes from 1.52% to 2.24%. This is the canvas's health bound.
- **Doubt.** The spread at +200 nearly doubles: 0.050 to 0.095. Lead changes fall from 9 to 4. A civ with river ground pulls away. That is the piece's question showing, and doubt is reported, not bound.
