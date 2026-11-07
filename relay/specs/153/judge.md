# #153 judge: `judge --compare` main (A, b377337) against the branch (B)

`sansfiat sweep --judge --json --floor` on each, run by the review. `*` is a change past its floor.

## Totals

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2086      0.2556      0.0471      0.0082 *
  total.standing.on_land_200                                  0.0004      0.0000     -0.0004      0.0012
  total.standing.top_200                                      0.1055      0.1101      0.0046      0.0083
doubt
  total.doubt.lead_changes                                   10.0000      6.0000     -4.0000      5.0000
  total.doubt.maps_with_lead_change                           4.0000      3.0000     -1.0000      1.0000
  total.doubt.no_claims_lead_changes                          4.0000      2.0000     -2.0000      1.0000 *
  total.doubt.no_claims_maps_with_lead_change                 3.0000      2.0000     -1.0000      1.0000
  total.doubt.spread_100                                      0.0687      0.0613     -0.0073      0.0065 *
  total.doubt.spread_200                                      0.0705      0.0576     -0.0128      0.0107 *
  total.doubt.swings                                          7.0000     15.0000      8.0000      9.0000
decisions
  total.decisions.ai_orders_per_100                          13.3206      8.3397     -4.9810      1.4444 *
  total.decisions.by_kind.claim                              11.7397      6.3683     -5.3714      1.3302 *
  total.decisions.by_kind.disband                             0.1905      0.1873     -0.0032      0.0714
  total.decisions.by_kind.raise                               1.3905      1.7841      0.3937      0.0794 *
  total.decisions.levers_repeated                             1.6190      1.6667      0.0476      0.1429
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                122.0000    130.0000      8.0000     17.0000
  total.motion.clans_joined                                 123.0000    131.0000      8.0000     16.0000
  total.motion.clans_left                                     0.0000      0.0000      0.0000      0.0000
  total.motion.land_changed                                2448.0000   1591.0000   -857.0000    392.0000 *
  total.motion.people_changed                               569.5084    660.5567     91.0483     66.6352 *
land
  total.land.claimed_200                                     32.4286     35.1429      2.7143      1.6190 *
  total.land.festival_changes_per_100                         0.6546      0.3459     -0.3087      0.0976 *
  total.land.flips_per_100                                    0.0950      0.0504     -0.0446      0.0196 *
  total.land.guard_200                                        2.7619      3.4286      0.6667      0.0476 *
  total.land.held_200                                        21.3333     26.1905      4.8571      1.6190 *
  total.land.homes_trailed.0                                  0.8456      0.8651      0.0195      0.0000 *
  total.land.homes_trailed.1                                  0.8505      0.8625      0.0121      0.0157
  total.land.markets_200                                    211.0000    212.0000      1.0000      3.0000
  total.land.markets_pick                                   218.0000    211.0000     -7.0000      0.0000 *
  total.land.route_trailed.0                                  0.9652      0.9884      0.0232      0.0000 *
  total.land.route_trailed.1                                  0.9782      0.9948      0.0166      0.0056 *
  total.land.trading_200                                      0.8968      0.9274      0.0306      0.0013 *
  total.land.trading_pick                                     0.8979      0.9292      0.0314      0.0000 *
  total.land.worked_trailed.0                                 0.3096      0.3171      0.0075      0.0000 *
  total.land.worked_trailed.1                                 0.2964      0.3000      0.0036      0.0061
economy
  total.economy.guard_mean                                    2.6409      3.3168      0.6759      0.0223 *
  total.economy.largest_army                                  0.0000      0.0000      0.0000      0.0000
  total.economy.levy_per_turn                                 7.8296      7.2091     -0.6205      0.1382 *
  total.economy.store_max                                    48.0917     52.0085      3.9168      0.0114 *
  total.economy.store_mean                                   17.1076     23.6401      6.5325      0.2567 *
  total.economy.turns_at_pack                               284.5714    390.4286    105.8571     26.7143 *
armies
  total.armies.camps_kept                                    21.0000     21.0000      0.0000      0.0000
  total.armies.camps_lost                                     0.0000      0.0000      0.0000      0.0000
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9897      0.9825     -0.0072      0.0078
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
listed 63, changed 40, flagged 26, only in A 0, only in B 0
```

## By map: the piece's scores

```
  maps.large 8.standing.aligned_200                           0.0967      0.1028      0.0061      0.0117
  maps.medium 8.standing.aligned_200                          0.1058      0.1622      0.0564      0.0165 *
  maps.medium 9.standing.aligned_200                          0.1656      0.2136      0.0480      0.0474 *
  maps.small 2.standing.aligned_200                           0.2588      0.4832      0.2244      0.0186 *
  maps.small 3.standing.aligned_200                           0.2754      0.2808      0.0054      0.0041 *
  maps.small 6.standing.aligned_200                           0.2325      0.3347      0.1022      0.0254 *
  maps.small 9.standing.aligned_200                           0.3250      0.2122     -0.1128      0.0274 *
  maps.large 8.motion.land_changed                         1165.0000    261.0000   -904.0000    464.0000 *
  maps.medium 8.motion.land_changed                          66.0000    193.0000    127.0000     37.0000 *
  maps.medium 9.motion.land_changed                         292.0000    551.0000    259.0000     63.0000 *
  maps.small 2.motion.land_changed                           87.0000    172.0000     85.0000     67.0000 *
  maps.small 3.motion.land_changed                           54.0000     66.0000     12.0000     17.0000
  maps.small 6.motion.land_changed                          227.0000    166.0000    -61.0000     92.0000
  maps.small 9.motion.land_changed                          557.0000    182.0000   -375.0000    157.0000 *
  maps.large 8.health.pregame_by_centres                      1.0000      1.0000      0.0000      0.0000
  maps.medium 8.health.pregame_by_centres                     1.0000      1.0000      0.0000      0.0000
  maps.medium 9.health.pregame_by_centres                     1.0000      1.0000      0.0000      0.0000
  maps.small 2.health.pregame_by_centres                      1.0000      1.0000      0.0000      0.0000
  maps.small 3.health.pregame_by_centres                      1.0000      1.0000      0.0000      0.0000
  maps.small 6.health.pregame_by_centres                      1.0000      1.0000      0.0000      0.0000
  maps.small 9.health.pregame_by_centres                      1.0000      1.0000      0.0000      0.0000
```
