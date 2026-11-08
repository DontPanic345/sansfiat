# #155 build-4 (canvas): fields that pay

Every setting is one `sansfiat sweep --judge --json --floor` on the seven retro maps, each against the
no-fields control in the same sweep (raw JSON kept off the branch). The sweep is deterministic, and y8 gives
`judge.md`'s numbers to the last digit. Ratios are read less their floor, as the judge's text reads them.
Bound 2 is `1.5 × grain ≥ 1.125`, so every grain here (≥ 0.8) meets it. Every run had 0 panics and pre-games 7 of 7.

## The chosen setting

`FIELD`: `flood` 15 days, `river` 7.5 days, **`grain` 0.9** (was 0.8), **`notch` 0.05** (was 0.1),
**`margin` 0.02** (was 0.01), `farmer` 0.1, `tame` 1. No rule changed shape: the easels are as build-1 built them.

| Bound | Number | |
|---|---|---|
| 1 Standing | 10 farming civs gain +0.0461 against the control, 10 of 10 gaining; others −0.0013, over others +0.0474; aligned at +200 0.3162 against 0.2525 (floor 0.004) | pass |
| 2 Yield | 1.5 × 0.9 = 1.35 ≥ 1.125 (a field day at fertility 1 is 1.35 food, a gathering day 0.75) | pass |
| 3 Store | top farmer at a pack on half its turns on 5 of 7 maps; 2 civs never at a pack (control 4) | pass, nothing to spare |
| 4 Motion | settled farmers' home moves 0.042 per 100 clan-turns, gatherers 0.114; moves on 0.004 against 0.037 | pass |
| 5 Changed hands | clans 179 vs 127, people 939 vs 629, land 1050 vs 1022 | pass |
| 6 Starving | 1.262 times the control, floor 0.156, read 1.106; worst map read 1.298 (large 8); settled 0.26% against gatherers 2.24% | pass |
| 7 Health | pre-games 7 of 7, panics 0, AI orders 7.31 per 100 against 6.91 | pass |
| 8 Bench | `bench --size large 8 --after 50`: 81.7 ms a turn against the base's 118.9 (both while check.sh ran beside them) | pass |
| 9 Checks | `check.sh` green; `www/play/fields.mjs` passes | pass |

### The judge's `fields` block

```
fields (against the no-fields control):
  small  seed   2  civs (share, gain, store turns at a pack): Red 0.09 +0.034 200/200; Blue 0.11 +0.115 0/200; Gold 0.00 -0.020 200/200  starving 0.0304 vs 0.0421 = 0.72 (floor 0.122)  settled 0.0010 gatherers 0.0267
  small  seed   3  civs (share, gain, store turns at a pack): Red 0.00 -0.011 190/200; Blue 0.11 +0.046 194/200; Gold 0.00 -0.009 200/200  starving 0.0207 vs 0.0176 = 1.17 (floor 0.088)  settled 0.0025 gatherers 0.0214
  small  seed   6  civs (share, gain, store turns at a pack): Red 0.14 +0.049 173/200; Blue 0.13 +0.011 200/200; Gold 0.03 -0.007 198/200  starving 0.0223 vs 0.0197 = 1.13 (floor 0.220)  settled 0.0034 gatherers 0.0202
  small  seed   9  civs (share, gain, store turns at a pack): Red 0.10 +0.050 178/200; Blue 0.11 +0.106 0/200; Gold 0.16 +0.009 36/200  starving 0.0199 vs 0.0142 = 1.40 (floor 0.814)  settled 0.0052 gatherers 0.0166
  medium seed   8  civs (share, gain, store turns at a pack): Red 0.10 +0.012 250/250; Blue 0.10 +0.026 225/250; Gold 0.00 -0.004 250/250  starving 0.0243 vs 0.0148 = 1.65 (floor 0.913)  settled 0.0037 gatherers 0.0252
  medium seed   9  civs (share, gain, store turns at a pack): Red 0.12 +0.050 249/250; Blue 0.00 -0.014 4/250; Gold 0.00 -0.007 248/250  starving 0.0261 vs 0.0189 = 1.38 (floor 0.283)  settled 0.0066 gatherers 0.0214
  large  seed   8  civs (share, gain, store turns at a pack): Red 0.10 +0.014 274/300; Blue 0.06 -0.000 300/300; Gold 0.09 -0.002 300/300  starving 0.0202 vs 0.0152 = 1.33 (floor 0.028)  settled 0.0009 gatherers 0.0218
  standing: farming civs 10 of 21, gain +0.0461 (10 gaining), others -0.0013, over others +0.0474: gain ≥ +0.01 pass, ¾ gaining pass, over others ≥ 0.01 pass
  aligned at +200 0.3162 vs 0.2525, gain +0.0637 (floor 0.004): past the floor pass
  store: top farmer at a pack on half its turns on 5 of 7 maps (≥ 5 pass); civs never at a pack 2 (control 4) (≤ 2 pass)
  motion: home moves per 100 settled 0.042 gatherers 0.114 (under half pass); moves on settled 0.004 gatherers 0.037 (no more pass)
  changed hands: clans 179 vs 127 pass, people 939.4 vs 629.4 pass, land 1050 vs 1022 pass; joined 181 vs 125
  starving: 0.0224 vs 0.0178 = 1.262 (floor 0.156), read 1.106 (≤ 1.25 pass); worst map read 1.298 (floor 0.028) (≤ 1.6 pass); settled 0.0026 gatherers 0.0224 (lower pass)
  AI orders per 100 7.31 vs 6.91 (≥ half pass); lead changes 6 vs 9, spread +200 0.104 vs 0.050 (not bound)
```

### Per civ (id: farming share at +200 / Standing gain against the control / store turns at a pack), the top farmer and whether it held a pack, the map's starving ratio

```
    small 2 424:0.09/+0.034/200 425:0.11/+0.115/0 426:0.00/-0.020/200 top 425 0 st 0.72
    small 3 247:0.00/-0.011/190 248:0.11/+0.046/194 249:0.00/-0.009/200 top 248 1 st 1.17
    small 6 245:0.14/+0.049/173 246:0.13/+0.011/200 247:0.03/-0.007/198 top 245 1 st 1.13
    small 9 309:0.10/+0.050/178 310:0.11/+0.106/0 311:0.16/+0.009/36 top 311 0 st 1.40
    medium 8 816:0.10/+0.012/250 817:0.10/+0.026/225 818:0.00/-0.004/250 top 816 1 st 1.65
    medium 9 845:0.12/+0.050/249 846:0.00/-0.014/4 847:0.00/-0.007/248 top 845 1 st 1.38
    large 8 1718:0.10/+0.014/274 1719:0.06/-0.000/300 1720:0.09/-0.002/300 top 1718 1 st 1.33
```

## Every setting run

Bound 2 holds in every row. "Misses" names each bound the row misses.

| Setting | 1 farm civs, gain (gaining), over others, aligned | 3 maps top at pack, never | 4 home moves settled/gatherers, moves on | 5 clans, people, land vs control | 6 starving ratio (read), worst map read, settled/gatherers | 7 AI orders vs control | Misses |
|---|---|---|---|---|---|---|---|
| y8 (build-1's) | 13, +0.0426 (13), +0.0508, 0.3221 vs 0.2525 | 4/7, 2 (control 4) | 0.066/0.138, 0.005/0.038 | 182/127, 980/629, 1195/1022 | 1.209 (1.130), 1.385 (medium 9), 0.0027/0.0223 | 8.44/6.91 | 3 |
| grain 0.9 | 9, +0.0424 (9), +0.0372, 0.3160 vs 0.2525 | 6/7, 1 (control 4) | 0.050/0.173, 0.006/0.052 | 177/127, 965/629, 1011/1022 | 1.419 (1.392), 1.809 (medium 9), 0.0028/0.0260 | 7.59/6.91 | 6 |
| grain 0.85 | 10, +0.0459 (9), +0.0462, 0.3175 vs 0.2525 | 4/7, 1 (control 4) | 0.035/0.173, 0.001/0.043 | 176/127, 951/629, 1169/1022 | 1.309 (1.265), 1.759 (medium 9), 0.0025/0.0241 | 8.58/6.91 | 3, 6 |
| farmer 0.2 | 12, +0.0439 (12), +0.0517, 0.3176 vs 0.2525 | 4/7, 2 (control 4) | 0.113/0.133, 0.008/0.060 | 185/127, 989/629, 1069/1022 | 1.381 (1.329), 1.754 (medium 9), 0.0030/0.0256 | 8.16/6.91 | 3, 4, 6 |
| margin 0.02 | 10, +0.0464 (10), +0.0490, 0.3147 vs 0.2525 | 4/7, 2 (control 4) | 0.034/0.112, 0.001/0.035 | 177/127, 980/629, 1073/1022 | 1.125 (1.024), 1.817 (small 9), 0.0022/0.0208 | 7.66/6.91 | 3, 6 |
| notch 0.05 | 12, +0.0424 (12), +0.0504, 0.3148 vs 0.2525 | 4/7, 0 (control 4) | 0.037/0.106, 0.002/0.037 | 169/127, 920/629, 1109/1022 | 1.174 (1.079), 1.417 (medium 9), 0.0028/0.0205 | 7.49/6.91 | 3 |
| grain 0.9, margin 0.02 | 12, +0.0431 (12), +0.0459, 0.3230 vs 0.2525 | 4/7, 1 (control 4) | 0.049/0.145, 0.001/0.039 | 208/127, 1087/629, 1054/1022 | 1.265 (1.257), 1.445 (medium 9), 0.0025/0.0239 | 7.07/6.91 | 3, 6 |
| grain 0.9, margin 0.03 | 11, +0.0364 (11), +0.0362, 0.3099 vs 0.2525 | 3/7, 1 (control 4) | 0.042/0.148, 0.002/0.042 | 193/127, 1009/629, 1267/1022 | 1.245 (1.222), 1.429 (small 9), 0.0024/0.0242 | 8.75/6.91 | 3 |
| grain 0.9, notch 0.05 | 11, +0.0387 (11), +0.0383, 0.3139 vs 0.2525 | 5/7, 1 (control 4) | 0.040/0.121, 0.000/0.036 | 191/127, 985/629, 1069/1022 | 1.251 (1.209), 1.819 (small 9), 0.0033/0.0222 | 6.93/6.91 | 6 |
| grain 0.875, notch 0.05 | 11, +0.0339 (11), +0.0269, 0.3158 vs 0.2525 | 4/7, 2 (control 4) | 0.050/0.115, 0.010/0.038 | 199/127, 1028/629, 1074/1022 | 1.267 (1.187), 1.646 (medium 9), 0.0034/0.0223 | 6.82/6.91 | 3, 6 |
| **grain 0.9, notch 0.05, margin 0.02 (chosen)** | 10, +0.0461 (10), +0.0474, 0.3162 vs 0.2525 | 5/7, 2 (control 4) | 0.042/0.114, 0.004/0.037 | 179/127, 939/629, 1050/1022 | 1.262 (1.106), 1.298 (large 8), 0.0026/0.0224 | 7.31/6.91 | none |

What the table says:

- **The starving swings by map more than it moves by setting.** Grain 0.9 alone puts starving at 1.39 read.
  Grain 0.9 with a slower split (notch 0.05, margin 0.02) puts it at 1.11. Only that combination is under every
  line. A slower split keeps a clan from crowding onto its floodplain in a few turns, so its gatherers aren't
  pushed off the river.
- **The store bound is set by the guard, not the fields.** The same civs miss in every row: small 2's 425,
  small 9's 311 and 310, small 6's 245, medium 8's 817. Their guard eats the levy. Grain 0.9 gives the top
  farmer enough levy to fill its store on two more maps. The chosen setting passes with nothing to spare: 5 of
  7, and medium 8's top farmer (816 at 0.10) leads 817 by a hair.
- **`farmer` 0.2 is worse.** Settled farmers move almost as often as gatherers (0.113 against 0.133), because
  a clan at a share of 0.1 still farms but no longer counts as settled, and the home rule lets it go.

## The store flow at y8

A one-off probe (not committed) logged each civ's store per turn on the army run. Values are the mean a turn:
levy in, gifts given, food posted to the clearing, guard people, and the store.

| Map | Civ | Levy | Given | Posted | Guard | Store | At a pack |
|---|---|---|---|---|---|---|---|
| small 2 | 424 | 20.97 | 0.15 | 32.43 | 12.38 | 39.0 | 200 |
| small 2 | 425 | 15.45 | 0.00 | 0.04 | 14.88 | 2.2 | 0 |
| small 2 | 426 | 10.21 | 0.07 | 36.89 | 4.27 | 46.1 | 200 |
| small 3 | 247 | 5.69 | 0.06 | 14.19 | 1.00 | 24.6 | 190 |
| small 3 | 248 | 12.85 | 0.05 | 38.08 | 4.93 | 45.4 | 194 |
| small 3 | 249 | 8.12 | 0.04 | 41.53 | 2.00 | 50.6 | 200 |
| small 6 | 245 | 25.28 | 0.04 | 4.86 | 19.86 | 12.2 | 51 |
| small 6 | 246 | 9.00 | 0.24 | 41.90 | 2.00 | 50.3 | 200 |
| small 6 | 247 | 10.99 | 0.01 | 41.51 | 2.00 | 47.7 | 198 |
| small 9 | 309 | 10.77 | 0.00 | 36.04 | 1.95 | 42.1 | 170 |
| small 9 | 310 | 7.92 | 0.00 | 0.03 | 3.97 | 3.9 | 0 |
| small 9 | 311 | 16.60 | 0.09 | 1.94 | 12.95 | 9.9 | 49 |
| medium 8 | 816 | 30.97 | 0.05 | 21.73 | 21.99 | 27.8 | 250 |
| medium 8 | 817 | 21.93 | 0.06 | 6.11 | 17.70 | 16.9 | 95 |
| medium 8 | 818 | 12.13 | 0.11 | 41.51 | 2.98 | 47.6 | 250 |
| medium 9 | 845 | 18.08 | 0.08 | 8.29 | 13.86 | 19.1 | 180 |
| medium 9 | 846 | 2.60 | 0.00 | 0.14 | 0.98 | 13.4 | 4 |
| medium 9 | 847 | 3.15 | 0.00 | 5.27 | 0.99 | 18.1 | 248 |
| large 8 | 1718 | 6.87 | 0.00 | 8.65 | 3.98 | 20.4 | 275 |
| large 8 | 1719 | 27.69 | 0.26 | 26.47 | 18.20 | 32.2 | 300 |
| large 8 | 1720 | 12.77 | 0.09 | 41.94 | 2.86 | 47.2 | 300 |

The civs that miss keep a guard of 13–20 people on a levy of 15–25 food a turn and post almost nothing. The civs
that fill their store keep a guard of 2–5. Spoilage and the cap aren't logged separately: a full store sits at
`CIV.store_cap` 60 and pays a cut levy.

## Base against branch

`judge --compare` of `main` at the piece's base (85868ed, a worktree) against the chosen setting. These are
the totals, and every metric past the floor is starred:

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2525      0.3162      0.0637      0.0048 *
  total.standing.on_land_200                                  0.0003      0.0000     -0.0003      0.0000 *
  total.standing.top_200                                      0.1069      0.1607      0.0538      0.0029 *
doubt
  total.doubt.lead_changes                                    9.0000      6.0000     -3.0000      6.0000
  total.doubt.maps_with_lead_change                           4.0000      4.0000      0.0000      1.0000
  total.doubt.no_claims_lead_changes                          6.0000      5.0000     -1.0000      1.0000
  total.doubt.no_claims_maps_with_lead_change                 4.0000      4.0000      0.0000      1.0000
  total.doubt.spread_100                                      0.0551      0.0983      0.0432      0.0049 *
  total.doubt.spread_200                                      0.0497      0.1038      0.0542      0.0071 *
  total.doubt.swings                                          7.0000      4.0000     -3.0000      4.0000
decisions
  total.decisions.ai_orders_per_100                           6.9111      7.3095      0.3984      1.1937
  total.decisions.by_kind.claim                               4.3222      3.6206     -0.7016      1.0524
  total.decisions.by_kind.disband                             0.3063      0.1429     -0.1635      0.1095 *
  total.decisions.by_kind.raise                               2.2825      3.5460      1.2635      0.0810 *
  total.decisions.levers_repeated                             1.5238      1.7619      0.2381      0.0476 *
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                127.0000    179.0000     52.0000     25.0000 *
  total.motion.clans_joined                                 125.0000    181.0000     56.0000     22.0000 *
  total.motion.clans_left                                     2.0000      0.0000     -2.0000      4.0000
  total.motion.land_changed                                1022.0000   1050.0000     28.0000    124.0000
  total.motion.people_changed                               629.3674    939.3724    310.0050    108.9791 *
land
  total.land.claimed_200                                     35.9524     36.0000      0.0476      0.4286
  total.land.festival_changes_per_100                         0.3071      0.5096      0.2025      0.0277 *
  total.land.flips_per_100                                    0.0347      0.1045      0.0698      0.0241 *
  total.land.guard_200                                        4.4762      7.6667      3.1905      0.4762 *
  total.land.held_200                                        33.2857     33.7619      0.4762      1.5714
  total.land.homes_trailed.0                                  0.8785      0.8785      0.0000      0.0000
  total.land.homes_trailed.1                                  0.8693      0.8257     -0.0437      0.0070 *
  total.land.markets_200                                    211.0000    211.0000      0.0000      1.0000
  total.land.markets_pick                                   217.0000    217.0000      0.0000      0.0000
  total.land.route_trailed.0                                  0.9904      0.9904      0.0000      0.0000
  total.land.route_trailed.1                                  0.9951      0.9987      0.0036      0.0029 *
  total.land.trading_200                                      0.9334      0.9219     -0.0115      0.0007 *
  total.land.trading_pick                                     0.9377      0.9377      0.0000      0.0000
  total.land.worked_trailed.0                                 0.3084      0.3084      0.0000      0.0000
  total.land.worked_trailed.1                                 0.3093      0.3122      0.0029      0.0027 *
economy
  total.economy.guard_mean                                    4.0548      7.2747      3.2199      0.3897 *
  total.economy.largest_army                                  0.0000      0.0000      0.0000      0.0000
  total.economy.levy_per_turn                                 9.1784     13.1677      3.9893      0.2681 *
  total.economy.store_max                                    52.1718     52.0639     -0.1079      0.1954
  total.economy.store_mean                                   25.8845     31.4191      5.5346      0.5679 *
  total.economy.turns_at_pack                               425.5714    552.7143    127.1429     14.0000 *
armies
  total.armies.camps_kept                                    21.0000     21.0000      0.0000      0.0000
  total.armies.camps_lost                                     0.0000      0.0000      0.0000      0.0000
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9883      0.9672     -0.0211      0.0086 *
  total.armies.warriors_starved                               0.0000      0.0000      0.0000      0.0000
fight
  total.fight.control_fallen                                  0.0000      0.0000      0.0000      0.0000
  total.fight.control_surrendered                             0.0000      0.0000      0.0000      0.0000
  total.fight.control_taken                                   0.0000      0.0000      0.0000      0.0000
  total.fight.fallen                                          0.0000      0.0000      0.0000      0.0000
  total.fight.surrendered                                     0.0000      0.0000      0.0000      0.0000
  total.fight.taken                                           0.0000      0.0000      0.0000      0.0000
end
  total.end.capitals_fallen                                   0.0000      0.0000      0.0000      0.0000
  total.end.early                                             0.0000      0.0000      0.0000      0.0000
  total.end.no_fight_capitals_fallen                          0.0000      0.0000      0.0000      0.0000
  total.end.no_fight_early                                    0.0000      0.0000      0.0000      0.0000
  total.end.no_fight_turns_mean                             228.5714    228.5714      0.0000      0.0000
  total.end.of                                                7.0000      7.0000      0.0000      0.0000
  total.end.turns_mean                                      228.5714    228.5714      0.0000      0.0000
health
  total.health.digest_mismatches                              0.0000      0.0000      0.0000      0.0000
  total.health.panics                                         0.0000      0.0000      0.0000      0.0000
  total.health.pregame_by_centres                             7.0000      7.0000      0.0000      0.0000
fields_vs_control
  total.fields_vs_control.ai_orders_per_100                        -      7.3095   only in B      0.9667
  total.fields_vs_control.aligned_200                              -      0.3162   only in B      0.0006
  total.fields_vs_control.aligned_gain                             -      0.0637   only in B      0.0041
  total.fields_vs_control.civs                                     -     21.0000   only in B      0.0000
  total.fields_vs_control.civs_never_at_pack                       -      2.0000   only in B      1.0000
  total.fields_vs_control.clans_changed                            -    179.0000   only in B     25.0000
  total.fields_vs_control.clans_joined                             -    181.0000   only in B     22.0000
  total.fields_vs_control.control_ai_orders_per_100                -      6.9111   only in B      1.1937
  total.fields_vs_control.control_aligned_200                      -      0.2525   only in B      0.0048
  total.fields_vs_control.control_civs_never_at_pack               -      4.0000   only in B      1.0000
  total.fields_vs_control.control_clans_changed                    -    127.0000   only in B     17.0000
  total.fields_vs_control.control_clans_joined                     -    125.0000   only in B     16.0000
  total.fields_vs_control.control_land_changed                     -   1022.0000   only in B     28.0000
  total.fields_vs_control.control_lead_changes                     -      9.0000   only in B      6.0000
  total.fields_vs_control.control_people_changed                   -    629.3674   only in B     48.9407
  total.fields_vs_control.control_spread_200                       -      0.0497   only in B      0.0071
  total.fields_vs_control.control_starving_share                   -      0.0178   only in B      0.0005
  total.fields_vs_control.farming_civs                             -     10.0000   only in B      1.0000
  total.fields_vs_control.farming_gain                             -      0.0461   only in B      0.0040
  total.fields_vs_control.farming_gaining                          -     10.0000   only in B      1.0000
  total.fields_vs_control.farming_over_others                      -      0.0474   only in B      0.0006
  total.fields_vs_control.gatherer_home_moves_per_100              -      0.1139   only in B      0.0204
  total.fields_vs_control.gatherer_moved_on_per_100                -      0.0367   only in B      0.0066
  total.fields_vs_control.gatherer_starving_share                  -      0.0224   only in B      0.0019
  total.fields_vs_control.land_changed                             -   1050.0000   only in B    124.0000
  total.fields_vs_control.lead_changes                             -      6.0000   only in B      2.0000
  total.fields_vs_control.maps_top_farmer_at_pack                  -      5.0000   only in B      2.0000
  total.fields_vs_control.maps_with_top_farmer                     -      7.0000   only in B      0.0000
  total.fields_vs_control.others_gain                              -     -0.0013   only in B      0.0034
  total.fields_vs_control.people_changed                           -    939.3724   only in B    108.9791
  total.fields_vs_control.settled_home_moves_per_100               -      0.0425   only in B      0.0061
  total.fields_vs_control.settled_moved_on_per_100                 -      0.0045   only in B      0.0034
  total.fields_vs_control.settled_starving_share                   -      0.0026   only in B      0.0001
  total.fields_vs_control.spread_200                               -      0.1038   only in B      0.0049
  total.fields_vs_control.starving_ratio                           -      1.2619   only in B      0.1560
  total.fields_vs_control.starving_share                           -      0.0224   only in B      0.0021
  total.fields_vs_control.worst_map_starving_ratio                 -      1.6451   only in B      0.9126
supply
  total.supply.clan_territory_200                             3.2752      2.9430     -0.3323      0.0191 *
  total.supply.clan_territory_pick                            3.2887      3.2887      0.0000      0.0000
  total.supply.counts.army_inside                             0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_inside_hungry                      0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_outside                            0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_outside_hungry                     0.0000      0.0000      0.0000      0.0000
  total.supply.counts.held_ring_200                         585.0000    571.0000    -14.0000     35.0000
  total.supply.counts.held_ring_pick                         21.0000     21.0000      0.0000      0.0000
  total.supply.counts.ring_200                              737.0000    749.0000     12.0000      8.0000 *
  total.supply.counts.ring_pick                             705.0000    705.0000      0.0000      0.0000
  total.supply.counts.scout_inside                         2811.0000   2169.0000   -642.0000    280.0000 *
  total.supply.counts.scout_inside_hungry                     3.0000      1.0000     -2.0000      3.0000
  total.supply.counts.scout_outside                        1555.0000   1623.0000     68.0000    182.0000
  total.supply.counts.scout_outside_hungry                   23.0000     23.0000      0.0000      9.0000
listed 114, changed 39, flagged 26, only in A 0, only in B 37
```
