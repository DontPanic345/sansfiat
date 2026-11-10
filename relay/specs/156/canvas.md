# #156 build-4 (canvas, Opus low): towns that form and feed

## The landed setting's towns block

`sansfiat sweep --judge --json --floor`, the seven retro maps against the no-towns control in the same
sweep (setting 6 below, the code as committed):

```
ok   towns form | 14 towns at +200 over the seven (≥ 10), on 5 of 7 maps (≥ 5)
ok   farmers meet the festival | town clans at +200 79, of them farmers 66 (≥ ¾), towns working fields 14 of 14
ok   town clans walk less | walk per turn 0.000 vs farmers outside 0.180 (under half)
ok   town clans trade more | traded per person a turn 0.0487 vs farmers outside 0.0378 (≥ 1.1 times)
MISS traded over all clans | 0.0600 vs the control's 0.0605, less the floor 0.0000
ok   clans change hands | 201.0 vs 177.0, less the floor 11.0
ok   people change hands | 1021.9 vs 953.3, less the floor 73.3
ok   land changes hands | 1864.0 vs 1150.0, less the floor 348.0
MISS settled farmers stay | settled home moves less town steps 0.004 per 100 (town steps 0.367); town clans' moves off 0.097; gatherers 0.123 (each under half)
ok   the store | maps with the top farmer at a pack 5 vs control 5 (no worse by more than 1); civs never at a pack 2 vs 1 (no worse by more than 1)
starving town 0.033101356174161316 outside 0.0077703808796245775 all 0.01657849779491886 control all 0.01324997771240082
moves {'settled_turns': 69476, 'settled_town_steps': 255, 'settled_other_moves': 3, 'town_turns': 15501, 'town_moves_off': 15} aligned 0.3298714303206976 0.31826667017546834 maps 5
```

Per map (towns at +200, town clans, starving share of clan-turns after +100, town clans | farmers outside):

```
small 2 towns 2 clans 11 starving 0.060 | 0.019
small 3 towns 0 clans 0 starving 0.000 | 0.010
small 6 towns 1 clans 4 starving 0.000 | 0.009
small 9 towns 3 clans 12 starving 0.014 | 0.020
medium 8 towns 0 clans 0 starving 0.000 | 0.006
medium 9 towns 2 clans 18 starving 0.057 | 0.006
large 8 towns 6 clans 34 starving 0.024 | 0.004
```

Lands with two misses, both reported below: traded per person over all clans (0.0600 against the
control's 0.0605, a floor of 0.0000 read from the sweep), and town clans' moves off their town (0.097 per
100 against the half of gatherers', 0.062: 15 moves in 15,501 town clan-turns).

## The bench

`sansfiat bench --size large 8 --after 50`, two runs each, same session:

| | run 1 | run 2 |
|---|---|---|
| main | 65.158 ms a turn | 64.580 ms a turn |
| this | 25.920 ms a turn | 25.195 ms a turn |

0.39 of main's (bound 0.45).

## Every setting

| # | Rule | towns +200 (maps) | town farmers | walk town/outside | traded town/outside | traded all (ctrl 0.0605) | settled other /100 | town steps /100 | town moves off /100 | gatherers /100 | store (pack maps; never-pack civs) | aligned (ctrl 0.318) | starving town / outside / all (ctrl all 0.0132) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | town step weighed against the field step, field step taken as before | 6 (4) | 29/34 | 0 / 0.208 | 0.0487 / 0.0399 | 0.0611 | 1.450 | 1.706 | 1.711 | 0.133 | 5 v 5; 2 v 1 | 0.338 | 0.035 / 0.011 / 0.016 |
| 2 | one choice: stay, field step or town step, each by value beating stay by TOWN.margin, routes priced from each home (festival-goers only) | 7 (4) | 32/39 | 0 / 0.195 | 0.0471 / 0.0388 | 0.0597 | 0.118 | 0.528 | 0.133 | 0.119 | 5 v 5; 3 v 1 | 0.337 | 0.031 / 0.010 / 0.017 |
| 3 | 2, and every farmer weighs (no festival: field step vs stay) | 10 (5) | 37/46 | 0 / 0.189 | 0.0443 / 0.0392 | 0.0603 | 0.090 | 0.496 | 0.307 | 0.106 | 4 v 5; 3 v 1 | 0.335 | 0.038 / 0.010 / 0.015 |
| 4 | 3, and a clan living on its festival's tile leaves it only when that feeds it better (`homes::keep`) | 7 (4) | 29/41 | 0 / 0.186 | 0.0449 / 0.0384 | 0.0594 | 0.085 | 0.473 | 0.083 | 0.112 | 5 v 5; 2 v 1 | 0.336 | 0.047 / 0.008 / 0.015 |
| 5 | 4, and steps are weighed on what the clan gathers and sows, its store left out | 13 (6) | 70/82 | 0 / 0.195 | 0.0494 / 0.0377 | 0.0598 | 0.055 | 0.392 | 0.117 | 0.113 | 4 v 5; 2 v 1 | 0.332 | 0.047 / 0.009 / 0.017 |
| **6** | 5, and a farmer with a festival never steps toward its fields away from it | **14 (5)** | **66/79** | **0 / 0.180** | **0.0487 / 0.0378** | **0.0600** | **0.004** | **0.367** | **0.097** | **0.123** | **5 v 5; 2 v 1** | **0.330** | **0.033 / 0.008 / 0.017** |

Changes of hands pass in every setting (setting 6: clans 201 v 177, people 1021.9 v 953.3, land 1864 v
1150). Towns differ by a few from setting to setting on the same rule family: the count is noisy at 10.

## Who starves round a town, and why

Small 2 at turn 400 (setting 3) shows it whole. Its two towns' clans each hold one tile, their home,
which another clan of the town claims. They gather nothing (gathering is by claim), so they live on
their share of the home's field ground (shared by area through `fields::room`) and on what the festival
swaps them: a grain-heavy diet, every clan on a tile at the same fed ratio, 0.875 on (57) and 0.877 on
(506). They stepped in while it fed them better, shedding the tiles beyond the radius of the new home;
the tiles round the festival are all held, so they cannot claim more. As more farmers step in, the
ground each gets shrinks. That is crowding on one tile's ground: the world's consequence the design
means (a home is not a claim, gathering stays exclusive, ground is shared by area), and starving
falls on medium 9 and small 2 where towns are biggest (18 and 11 clans).

One rule that can starve a clan by mistake was changed: a step was weighed with the clan's store
counted (`clans::value_at` adds the store to this turn's harvest, and the value caps at full), so a
clan with a store could not tell a home that feeds it less from the one it left, and could step onto
it and starve once the store ran out. Steps are now weighed on what the clan gathers and sows alone
(setting 5). This is reasoned from the rule, not traced clan by clan; its measured effect is towns
13 (6 maps) from 7 (4) and settled other moves 0.085 → 0.055, with the town share of starving level. The town clans' share of starving moved 0.047 → 0.033 when the
field step stopped walking farmers away from their festival (setting 6), since a farmer that left a
town lost its shared ground there without a full field to go to.

Starving shares, reported not bound (setting 6): town clans 0.033 of clan-turns, farmers outside towns
0.008, all clans 0.017 against the no-towns control's 0.013.

## The misses

- **Town clans' moves off**: the moves left are clans that stopped farming on a town (a starving clan
  drops a field notch; a clan under `FIELD.farmer` is not weighed as a farmer) and step to the middle
  of their land because it feeds them better by `homes::keep`. On large 8 and small 2 at 350 turns every
  move off a market tile was a non-farmer's step, none a move-on. Meeting the bound means a clan that
  stops farming stays at the town it doesn't farm from, which no rule in the piece's Decided gives.
- **Traded over all clans**: 0.0600 against 0.0605, and the floor reads 0.0000 for this metric. Town
  clans trade 1.29 times what farmers outside do; the all-clans figure is lower because towns are
  fewer, bigger festivals for fewer clans. With a nonzero floor (it is 0 in every sweep here, which looks
  like the floor not being computed for `traded`) the miss would likely be inside it.

## Towns off, health, checks

- With `Config.towns` off the home rule is never read (`homes::weighs` and `homes::keep` return the old
  step): every map's `no_towns` block in this sweep equals build-3's sweep's, key for key, on all seven.
- Pre-games by centres 7 of 7, panics 0, digest mismatches 0. `engine/check.sh` passes through
  `cargo test` (save-version pins at 18 and the four digests moved), clippy, fmt, the web build and
  `www/check.mjs` and `www/play/all.mjs` (towns.mjs passing).
- Starving against the no-fields control moved 1.22 → 1.38 (inside its floor 0.19); settled farmers'
  starving share 0.0013 → 0.0113 is the town clans' crowding.

## judge --compare against main (A = main, B = this, totals; * past the floor)

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.3183      0.3299      0.0116      0.0086 *
  total.standing.on_land_200                                  0.0000      0.0000      0.0000      0.0000
  total.standing.top_200                                      0.1619      0.1662      0.0043      0.0064
doubt
  total.doubt.lead_changes                                    4.0000      8.0000      4.0000      4.0000
  total.doubt.maps_with_lead_change                           4.0000      4.0000      0.0000      1.0000
  total.doubt.no_claims_lead_changes                          3.0000      7.0000      4.0000      1.0000 *
  total.doubt.no_claims_maps_with_lead_change                 3.0000      5.0000      2.0000      0.0000 *
  total.doubt.spread_100                                      0.1016      0.1091      0.0076      0.0052 *
  total.doubt.spread_200                                      0.1051      0.1080      0.0029      0.0100
  total.doubt.swings                                          3.0000      7.0000      4.0000      3.0000 *
decisions
  total.decisions.ai_orders_per_100                           8.2603     11.6413      3.3810      1.0508 *
  total.decisions.by_kind.claim                               4.3159      7.7857      3.4698      0.9841 *
  total.decisions.by_kind.disband                             0.3095      0.0476     -0.2619      0.0000 *
  total.decisions.by_kind.raise                               3.6349      3.8079      0.1730      0.2159
  total.decisions.levers_repeated                             1.7143      1.7143      0.0000      0.0476
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                177.0000    201.0000     24.0000     24.0000
  total.motion.clans_joined                                 177.0000    201.0000     24.0000     25.0000
  total.motion.clans_left                                     1.0000      2.0000      1.0000      2.0000
  total.motion.land_changed                                1150.0000   1864.0000    714.0000    348.0000 *
  total.motion.people_changed                               953.3499   1021.8570     68.5071     89.1467
land
  total.land.claimed_200                                     36.6667     34.9524     -1.7143      1.0952 *
  total.land.festival_changes_per_100                         0.4988      0.6359      0.1371      0.0374 *
  total.land.flips_per_100                                    0.0871      0.1118      0.0246      0.0196 *
  total.land.guard_200                                        7.3810      8.5714      1.1905      0.3810 *
  total.land.held_200                                        34.0000     32.5238     -1.4762      0.4286 *
  total.land.homes_trailed.0                                  0.8785      0.8785      0.0000      0.0000
  total.land.homes_trailed.1                                  0.8163      0.8539      0.0376      0.0088 *
  total.land.markets_200                                    212.0000    214.0000      2.0000      3.0000
  total.land.markets_pick                                   217.0000    217.0000      0.0000      0.0000
  total.land.route_trailed.0                                  0.9904      0.9904      0.0000      0.0000
  total.land.route_trailed.1                                  0.9991      0.9982     -0.0009      0.0008 *
  total.land.trading_200                                      0.9172      0.9307      0.0134      0.0042 *
  total.land.trading_pick                                     0.9377      0.9377      0.0000      0.0000
  total.land.worked_trailed.0                                 0.3084      0.3084      0.0000      0.0000
  total.land.worked_trailed.1                                 0.3122      0.3124      0.0002      0.0051
economy
  total.economy.guard_mean                                    7.2543      7.9333      0.6791      0.2295 *
  total.economy.largest_army                                  0.0000      0.0000      0.0000      0.0000
  total.economy.levy_per_turn                                12.6959     13.2751      0.5792      0.2596 *
  total.economy.store_max                                    52.1897     52.1897      0.0000      0.1386
  total.economy.store_mean                                   29.1273     30.1466      1.0193      1.9396
  total.economy.turns_at_pack                               524.0000    533.4286      9.4286      8.0000 *
armies
  total.armies.camps_kept                                    21.0000     21.0000      0.0000      0.0000
  total.armies.camps_lost                                     0.0000      0.0000      0.0000      0.0000
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9672      0.9752      0.0080      0.0081
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
  total.fields_vs_control.ai_orders_per_100                   8.2603     11.6413      3.3810      1.0508 *
  total.fields_vs_control.aligned_200                         0.3183      0.3299      0.0116      0.0086 *
  total.fields_vs_control.aligned_gain                        0.0658      0.0774      0.0116      0.0133
  total.fields_vs_control.civs                               21.0000     21.0000      0.0000      0.0000
  total.fields_vs_control.civs_never_at_pack                  1.0000      2.0000      1.0000      1.0000
```
