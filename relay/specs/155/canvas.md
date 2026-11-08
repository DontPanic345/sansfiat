# #155 build-4 (canvas): the field rules tuned on the retro maps

Every bound is met. The chosen `FIELD` is build-1's y8 with two changes: a farmer from `farmer` 0.2 of its
days (was 0.1), and a rule change: **a field is sown only on a tile that yields**. Every number below is read
from `sansfiat sweep --judge --json --floor` against the no-fields control in the same sweep (the chosen
setting's text, JSON and `judge --compare` are in `/tmp`-free form below). Sweeps are deterministic.

## What the starving was

The sweep's starving is a growth transient, not a rule fault. In a per-turn trace of small 9 the control's
population stays at 576 while the fields run grows 600 → 934 in 60 turns, 118 → 137 clans. Each new daughter
clan starts on one tile and starves for a few turns. By turns 175–200 the starving rate equals the control's
(0.0118 against 0.0117 of clan-turns), and the whole excess is in the first 100 turns after the pick (turns
0–25: 0.050 against 0.036; 25–50: 0.038 against 0.028; 50–75: 0.025 against 0.023).

The "starving gatherers" are:
- **clans that farmed and fell back**, 4% of clan-turns and 27% of starving at a 13–15% rate. They are mostly
  daughters (a daughter inherits its parent's share, `d.fields = c.fields`) and clans that moved on after
  starving (`m.fields = 0` on a move). A daughter has one tile and no field tile on it, so its inherited share
  worked nothing: of the starving clans with fields in the trace, most had `fieldtiles 0` and `terr 1`.
- the unaligned's and the civs' alike: among small clans, unaligned clans starve at 1.7% (never farmed) to 2.6% (farming), a civ's at 3.1% to 2.4%.
- Settled farmers starve at 0.13% of clan-turns, a tenth of gatherers' 2.3%.

Nothing in the field rule turns that transient off, and none of these moved it past the noise: no starving
fall-back (1.42 raw, worse), the fall-back after 2 or 3 turns, daughters starting at 0 fields (1.32), margin
0.015, 0.02 and 0.03, notch 0.05, grain 0.75 to 1.0 and tame 2. The sweep-to-sweep noise is ±0.1 on the ratio,
so the settings are one draw each and the table reads them with y8's floor (0.079) unless said. The chosen one
is read on its own floor (0.187): its raw 1.222 reads 1.036.

## The rule change

`fields::jobs` takes as field tiles those with ground **and a yield above 0**. Before, a tile on wild grain took
days it could not pay. A daughter that inherits a share onto such ground spent that share of its days on
nothing, and a farmer's days sown on wild grain did the same. Now with no yielding field tile all labour
gathers, as it does with no ground. `home_toward` and `plots` read the same job, so the farmer's home follows
yielding fields only. Settled farmers' home moves fall (0.066 → 0.022 per 100 clan-turns at `farmer` 0.1).
Recorded as a decision.

## Settings

Each row is one `sweep --judge --json` (the chosen one with `--floor`). Columns: farming civs' mean Standing
gain at +200 (gaining/farming); maps where the top farmer holds a pack on half its turns / civs never at a
pack; settled / gatherers' home moves per 100 clan-turns; clans, people and land changed hands (control: 127,
629, 1022); starving against the control, raw → read less its floor; the worst map's ratio less its floor; AI
orders per 100 (control 6.9). Bound 2 holds for every grain ≥ 0.75.

| Setting | Farming gain | Store | Home moves | Changed hands | Starving | Worst map | AI |
|---|---|---|---|---|---|---|---|
| y8: grain 0.8, notch 0.1, margin 0.01, farmer 0.1, flood 15 d, tame 1 (build-2's start) | +0.043 (13/13) | 4/7, 2 | 0.066 / 0.138 | 182, 980, 1195 | 1.209 → 1.130 | 0.84 | 8.4 |
| y8, grain 1.0 | +0.042 (10/10) | 5/7, 1 | 0.029 / 0.188 | 201, 1049, 1000 | 1.283 → 1.204 | 0.76 | 6.6 |
| y8, grain 0.9, tame 2 | +0.045 (8/8) | 3/7, 1 | 0.071 / 0.160 | 180, 965, 991 | 1.282 → 1.204 | 1.26 | 7.1 |
| y8, margin 0.03 | +0.044 (10/11) | 4/7, 1 | 0.036 / 0.162 | 176, 966, 1093 | 1.310 → 1.232 | 0.88 | 8.7 |
| y8, no starving fall-back | +0.040 (12/13) | 5/7, 2 | 0.107 / 0.115 | 186, 999, 1100 | 1.416 → 1.337 | 0.78 | 7.5 |
| y8, the starving fall-back only after 2 starving turns | +0.041 (11/12) | 3/7, 2 | 0.075 / 0.140 | 180, 963, 1098 | 1.301 → 1.222 | 0.65 | 7.5 |
| y8, the starving fall-back only after 3 starving turns | +0.041 (11/12) | 4/7, 2 | 0.067 / 0.125 | 191, 1007, 1071 | 1.282 → 1.203 | 0.81 | 7.0 |
| yield filter, farmer 0.1 | +0.040 (13/13) | 5/7, 2 | 0.022 / 0.146 | 190, 1002, 1047 | 1.270 → 1.192 | 0.83 | 8.0 |
| yield filter, grain 1.0 | +0.035 (8/8) | 3/7, 1 | 0.036 / 0.200 | 223, 1163, 1120 | 1.447 → 1.368 | 0.97 | 7.9 |
| yield filter, daughters start at 0 fields | +0.039 (12/12) | 4/7, 2 | 0.021 / 0.184 | 167, 906, 1072 | 1.321 → 1.243 | 0.95 | 8.0 |
| yield filter, grain 0.75 | +0.040 (12/13) | 6/7, 2 | 0.038 / 0.183 | 197, 1035, 1206 | 1.339 → 1.261 | 0.94 | 8.7 |
| yield filter, grain 0.77 | +0.037 (13/13) | 5/7, 1 | 0.048 / 0.153 | 188, 985, 1044 | 1.311 → 1.233 | 0.86 | 7.5 |
| yield filter, margin 0.02 | +0.037 (13/13) | 4/7, 1 | 0.023 / 0.119 | 169, 921, 1102 | 1.201 → 1.123 | 0.86 | 7.8 |
| yield filter, margin 0.015 | +0.038 (12/13) | 4/7, 2 | 0.042 / 0.149 | 182, 973, 1009 | 1.320 → 1.241 | 0.78 | 6.8 |
| yield filter, notch 0.05 | +0.040 (12/12) | 5/7, 0 | 0.052 / 0.133 | 164, 895, 984 | 1.289 → 1.211 | 1.34 | 7.1 |
| yield filter, farmer 0.2 | +0.041 (12/13) | 5/7, 1 | 0.031 / 0.122 | 177, 953, 1150 | 1.222 → 1.144 | 0.91 | 8.3 |
| yield filter, farmer 0.3 | +0.043 (12/12) | 5/7, 2 | 0.051 / 0.120 | 178, 973, 1108 | 1.274 → 1.196 | 1.17 | 8.2 |
| yield filter, farmer 0.2, grain 0.9 | +0.041 (12/12) | 5/7, 0 | 0.032 / 0.111 | 180, 951, 1092 | 1.225 → 1.146 | 0.86 | 8.6 |
| **yield filter, farmer 0.2, grain 0.8 (chosen; read on its own floor)** | +0.041 (12/13) | 5/7, 1 | 0.031 / 0.122 | 177, 953, 1150 | 1.222 → 1.036 | 0.89 | 8.3 |

Bounds: farming gain ≥ +0.01 with ¾ gaining; store ≥ 5/7 maps and ≤ 2 never; settled under half of gatherers;
changed hands ≥ control less floor; starving read ≤ 1.25 and worst map read ≤ 1.6; AI orders ≥ 3.5.

y8 misses only the store (4 of 7). Settings that raise the store (grain 1.0, 0.75) raise starving past 1.25, or
pass it by noise alone. `farmer` 0.2 holds both: the store on 5 of 7 maps, starving 1.222 (1.036 on its own
floor, 0.19 of room), and settled farmers move 0.031 against gatherers' 0.122.

## The chosen setting

`FIELD`: flood 15 days, river 7.5 days, grain 0.8, notch 0.1, margin 0.01, **farmer 0.2**, tame 1, with the
yield filter. `SAVE_VERSION` 16 (the digests in `cli/tests/save.rs` move).

| Bound | Number | Verdict |
|---|---|---|
| 1 Standing | farming civs +0.0407 (12 of 13 gain), others −0.0086, over others +0.0493; aligned 0.2525 → 0.3183, gain +0.0658 (floor 0.0042) | pass |
| 2 Fields pay | 1.5 × 0.8 = 1.2 ≥ 1.125 | pass |
| 3 Store | top farmer at a pack on half its turns on 5 of 7 maps (small 3, small 6, medium 8, medium 9, large 8); 1 civ never at a pack (control 4) | pass |
| 4 Motion | settled farmers 0.031 home moves per 100 clan-turns (floor 0.003) against gatherers' 0.122; moves on 0.000 against 0.051 | pass |
| 5 Changed hands | clans 177 vs 127 (floor 24), people 953 vs 629 (floor 89), land 1150 vs 1022; joined 177 vs 125 | pass |
| 6 Starving | 0.0217 vs 0.0178 = 1.222, floor 0.187, read 1.036 (≤ 1.25); worst map read 1.211 in the text (≤ 1.6); settled 0.0013, gatherers 0.0226 | pass |
| 7 Health | pre-games by centres 7 of 7, panics 0, digest mismatches 0; AI orders 8.26 vs 6.91 per 100 | pass |
| 8 Bench | `bench --size large 8 --after 50`: 53.2 ms a turn, base 121.7 ms | pass |
| 9 Checks | `engine/check.sh` and `www/play/fields.mjs` | see the summary |

The store misses on small 2 and small 9, where the civs with the most fields are not at a pack: 425 on small 2
holds one 7 of 200 turns; on small 9 the three civs hold one 0, 60 and 5 of 200 turns. These are the civs the
probe's store flow found the guard eating the levy from (`probe.md`); the flow was not measured again here,
because the bound is met. One civ never holds a pack (309 on small 9).

### The `fields` block, the text's

```
fields (against the no-fields control):
  small  seed   2  civs (share, gain, store turns at a pack): Red 0.11 +0.031 200/200; Blue 0.12 +0.117 7/200; Gold 0.00 -0.020 200/200  starving 0.0311 vs 0.0421 = 0.74 (floor 0.041)  settled 0.0027 gatherers 0.0319
  small  seed   3  civs (share, gain, store turns at a pack): Red 0.00 -0.009 190/200; Blue 0.12 +0.043 194/200; Gold 0.00 -0.009 200/200  starving 0.0229 vs 0.0176 = 1.30 (floor 0.148)  settled 0.0032 gatherers 0.0238
  small  seed   6  civs (share, gain, store turns at a pack): Red 0.17 +0.037 153/200; Blue 0.14 +0.010 200/200; Gold 0.03 -0.007 198/200  starving 0.0187 vs 0.0197 = 0.95 (floor 0.365)  settled 0.0013 gatherers 0.0208
  small  seed   9  civs (share, gain, store turns at a pack): Red 0.14 +0.065 0/200; Blue 0.13 +0.091 60/200; Gold 0.14 +0.005 5/200  starving 0.0187 vs 0.0142 = 1.32 (floor 1.271)  settled 0.0000 gatherers 0.0206
  medium seed   8  civs (share, gain, store turns at a pack): Red 0.13 +0.018 250/250; Blue 0.14 +0.022 210/250; Gold 0.00 -0.006 250/250  starving 0.0187 vs 0.0148 = 1.27 (floor 0.800)  settled 0.0027 gatherers 0.0181
  medium seed   9  civs (share, gain, store turns at a pack): Red 0.13 +0.080 222/250; Blue 0.00 -0.014 4/250; Gold 0.00 -0.007 248/250  starving 0.0329 vs 0.0189 = 1.74 (floor 0.608)  settled 0.0009 gatherers 0.0342
  large  seed   8  civs (share, gain, store turns at a pack): Red 0.12 +0.013 277/300; Blue 0.10 +0.003 300/300; Gold 0.11 -0.002 300/300  starving 0.0194 vs 0.0152 = 1.27 (floor 0.064)  settled 0.0009 gatherers 0.0204
  standing: farming civs 13 of 21, gain +0.0407 (12 gaining), others -0.0086, over others +0.0493: gain ≥ +0.01 pass, ¾ gaining pass, over others ≥ 0.01 pass
  aligned at +200 0.3183 vs 0.2525, gain +0.0658 (floor 0.004): past the floor pass
  store: top farmer at a pack on half its turns on 5 of 7 maps (≥ 5 pass); civs never at a pack 1 (control 4) (≤ 2 pass)
  motion: home moves per 100 settled 0.031 gatherers 0.122 (under half pass); moves on settled 0.000 gatherers 0.051 (no more pass)
  changed hands: clans 177 vs 127 pass, people 953.3 vs 629.4 pass, land 1150 vs 1022 pass; joined 177 vs 125
  starving: 0.0217 vs 0.0178 = 1.222 (floor 0.187), read 1.036 (≤ 1.25 pass); worst map read 1.211 (floor 0.064) (≤ 1.6 pass); settled 0.0013 gatherers 0.0226 (lower pass)
  AI orders per 100 8.26 vs 6.91 (≥ half pass); lead changes 4 vs 9, spread +200 0.105 vs 0.050 (not bound)
```

### Per civ

| map | civ | share +200 | gain vs control | store turns at a pack | farms |
|---|---|---|---|---|---|
| small 2 | 424 | 0.112 | +0.031 | 200/200 | yes |
| small 2 | 425 (top farmer) | 0.116 | +0.117 | 7/200 | yes |
| small 2 | 426 | 0.000 | -0.020 | 200/200 | no |
| small 3 | 247 | 0.000 | -0.009 | 190/200 | no |
| small 3 | 248 (top farmer) | 0.122 | +0.043 | 194/200 | yes |
| small 3 | 249 | 0.000 | -0.009 | 200/200 | no |
| small 6 | 245 (top farmer) | 0.168 | +0.037 | 153/200 | yes |
| small 6 | 246 | 0.143 | +0.010 | 200/200 | yes |
| small 6 | 247 | 0.029 | -0.007 | 198/200 | no |
| small 9 | 309 | 0.136 | +0.065 | 0/200 | yes |
| small 9 | 310 | 0.129 | +0.091 | 60/200 | yes |
| small 9 | 311 (top farmer) | 0.145 | +0.005 | 5/200 | yes |
| medium 8 | 816 | 0.129 | +0.018 | 250/250 | yes |
| medium 8 | 817 (top farmer) | 0.135 | +0.022 | 210/250 | yes |
| medium 8 | 818 | 0.000 | -0.006 | 250/250 | no |
| medium 9 | 845 (top farmer) | 0.132 | +0.080 | 222/250 | yes |
| medium 9 | 846 | 0.000 | -0.014 | 4/250 | no |
| medium 9 | 847 | 0.000 | -0.007 | 248/250 | no |
| large 8 | 1718 (top farmer) | 0.120 | +0.013 | 277/300 | yes |
| large 8 | 1719 | 0.096 | +0.003 | 300/300 | no |
| large 8 | 1720 | 0.109 | -0.002 | 300/300 | yes |

## `judge --compare` of the base against the branch

The base is `main` at the piece's base, 85868ed (a worktree), and the branch is this commit. Both are
`sweep --judge --json --floor`. A `*` marks a change past the floor. The `fields_vs_control` rows exist only on
the branch. Full output:

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2525      0.3183      0.0658      0.0048 *
  total.standing.on_land_200                                  0.0003      0.0000     -0.0003      0.0000 *
  total.standing.top_200                                      0.1069      0.1619      0.0550      0.0018 *
doubt
  total.doubt.lead_changes                                    9.0000      4.0000     -5.0000      6.0000
  total.doubt.maps_with_lead_change                           4.0000      4.0000      0.0000      1.0000
  total.doubt.no_claims_lead_changes                          6.0000      3.0000     -3.0000      1.0000 *
  total.doubt.no_claims_maps_with_lead_change                 4.0000      3.0000     -1.0000      1.0000
  total.doubt.spread_100                                      0.0551      0.1016      0.0465      0.0008 *
  total.doubt.spread_200                                      0.0497      0.1051      0.0554      0.0071 *
  total.doubt.swings                                          7.0000      3.0000     -4.0000      4.0000
decisions
  total.decisions.ai_orders_per_100                           6.9111      8.2603      1.3492      1.1937 *
  total.decisions.by_kind.claim                               4.3222      4.3159     -0.0063      1.0524
  total.decisions.by_kind.disband                             0.3063      0.3095      0.0032      0.0603
  total.decisions.by_kind.raise                               2.2825      3.6349      1.3524      0.0810 *
  total.decisions.levers_repeated                             1.5238      1.7143      0.1905      0.0476 *
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                127.0000    177.0000     50.0000     24.0000 *
  total.motion.clans_joined                                 125.0000    177.0000     52.0000     25.0000 *
  total.motion.clans_left                                     2.0000      1.0000     -1.0000      1.0000
  total.motion.land_changed                                1022.0000   1150.0000    128.0000     79.0000 *
  total.motion.people_changed                               629.3674    953.3499    323.9824     89.1467 *
land
  total.land.claimed_200                                     35.9524     36.6667      0.7143      1.0952
  total.land.festival_changes_per_100                         0.3071      0.4988      0.1917      0.0277 *
  total.land.flips_per_100                                    0.0347      0.0871      0.0524      0.0105 *
  total.land.guard_200                                        4.4762      7.3810      2.9048      0.1429 *
  total.land.held_200                                        33.2857     34.0000      0.7143      1.5714
  total.land.homes_trailed.0                                  0.8785      0.8785      0.0000      0.0000
  total.land.homes_trailed.1                                  0.8693      0.8163     -0.0530      0.0060 *
  total.land.markets_200                                    211.0000    212.0000      1.0000      1.0000
  total.land.markets_pick                                   217.0000    217.0000      0.0000      0.0000
  total.land.route_trailed.0                                  0.9904      0.9904      0.0000      0.0000
  total.land.route_trailed.1                                  0.9951      0.9991      0.0040      0.0029 *
  total.land.trading_200                                      0.9334      0.9172     -0.0162      0.0042 *
  total.land.trading_pick                                     0.9377      0.9377      0.0000      0.0000
  total.land.worked_trailed.0                                 0.3084      0.3084      0.0000      0.0000
  total.land.worked_trailed.1                                 0.3093      0.3122      0.0029      0.0051
economy
  total.economy.guard_mean                                    4.0548      7.2543      3.1994      0.1216 *
  total.economy.largest_army                                  0.0000      0.0000      0.0000      0.0000
  total.economy.levy_per_turn                                 9.1784     12.6959      3.5175      0.1188 *
  total.economy.store_max                                    52.1718     52.1897      0.0180      0.0818
  total.economy.store_mean                                   25.8845     29.1273      3.2428      1.9396 *
  total.economy.turns_at_pack                               425.5714    524.0000     98.4286      7.5714 *
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
  total.fields_vs_control.ai_orders_per_100                        -      8.2603   only in B      1.0508
  total.fields_vs_control.aligned_200                              -      0.3183   only in B      0.0005
  total.fields_vs_control.aligned_gain                             -      0.0658   only in B      0.0042
  total.fields_vs_control.civs                                     -     21.0000   only in B      0.0000
  total.fields_vs_control.civs_never_at_pack                       -      1.0000   only in B      1.0000
  total.fields_vs_control.clans_changed                            -    177.0000   only in B     24.0000
  total.fields_vs_control.clans_joined                             -    177.0000   only in B     25.0000
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
  total.fields_vs_control.farming_civs                             -     13.0000   only in B      0.0000
  total.fields_vs_control.farming_gain                             -      0.0407   only in B      0.0024
  total.fields_vs_control.farming_gaining                          -     12.0000   only in B      1.0000
  total.fields_vs_control.farming_over_others                      -      0.0493   only in B      0.0025
  total.fields_vs_control.gatherer_home_moves_per_100              -      0.1221   only in B      0.0027
  total.fields_vs_control.gatherer_moved_on_per_100                -      0.0511   only in B      0.0025
  total.fields_vs_control.gatherer_starving_share                  -      0.0226   only in B      0.0032
  total.fields_vs_control.land_changed                             -   1150.0000   only in B     79.0000
  total.fields_vs_control.lead_changes                             -      4.0000   only in B      1.0000
  total.fields_vs_control.maps_top_farmer_at_pack                  -      5.0000   only in B      1.0000
  total.fields_vs_control.maps_with_top_farmer                     -      7.0000   only in B      0.0000
  total.fields_vs_control.others_gain                              -     -0.0086   only in B      0.0002
  total.fields_vs_control.people_changed                           -    953.3499   only in B     89.1467
  total.fields_vs_control.settled_home_moves_per_100               -      0.0307   only in B      0.0031
  total.fields_vs_control.settled_moved_on_per_100                 -      0.0000   only in B      0.0000
  total.fields_vs_control.settled_starving_share                   -      0.0013   only in B      0.0006
  total.fields_vs_control.spread_200                               -      0.1051   only in B      0.0014
  total.fields_vs_control.starving_ratio                           -      1.2221   only in B      0.1865
  total.fields_vs_control.starving_share                           -      0.0217   only in B      0.0026
  total.fields_vs_control.worst_map_starving_ratio                 -      1.7419   only in B      0.8492
supply
  total.supply.clan_territory_200                             3.2752      2.9469     -0.3284      0.0075 *
  total.supply.clan_territory_pick                            3.2887      3.2887      0.0000      0.0000
  total.supply.counts.army_inside                             0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_inside_hungry                      0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_outside                            0.0000      0.0000      0.0000      0.0000
  total.supply.counts.army_outside_hungry                     0.0000      0.0000      0.0000      0.0000
  total.supply.counts.held_ring_200                         585.0000    580.0000     -5.0000     35.0000
  total.supply.counts.held_ring_pick                         21.0000     21.0000      0.0000      0.0000
  total.supply.counts.ring_200                              737.0000    752.0000     15.0000     17.0000
  total.supply.counts.ring_pick                             705.0000    705.0000      0.0000      0.0000
  total.supply.counts.scout_inside                         2811.0000   2488.0000   -323.0000     56.0000 *
  total.supply.counts.scout_inside_hungry                     3.0000      4.0000      1.0000      2.0000
  total.supply.counts.scout_outside                        1555.0000   1416.0000   -139.0000    182.0000
  total.supply.counts.scout_outside_hungry                   23.0000     36.0000     13.0000     10.0000 *
listed 114, changed 42, flagged 27, only in A 0, only in B 37
```
