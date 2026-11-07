# #153 build-5: the flat trail on the finished branch

`sansfiat sweep --judge --json --floor` on main (a worktree at `b377337`, the branch's base) and on the
branch's head (the flat trail, the search's best-case order, the feeding at `SCOUT { wander 8, low 1.5, fed 2.5,
rich 1.2 }`), then `sansfiat judge --compare main.json branch.json`. `TRAIL.flat` stays **0.75**: every bound
holds, so 0.85 and 0.65 were not run.

## The bounds

| Bound (`spec.md`) | Main | Branch | Change | Floor | Holds |
|---|---|---|---|---|---|
| Trading share at the pick rises past the floor | 0.898 | 0.929 | +0.031 | 0.000 | yes |
| Trading share at +200 rises past the floor | 0.897 | 0.927 | +0.031 | 0.001 | yes |
| Aligned share at +200 (Standing) rises past the floor | 0.209 | 0.256 | +0.047 | 0.008 | yes |
| Pre-games ended by centres | 7 of 7 | 7 of 7 | | | yes |
| AI orders per 100 turns above half of main's | 13.3 | 8.3 | 63% of main | | yes |
| Land tiles changing hands above half of main's | 2448 | 1591 | 65% of main | 392 | yes |

## The totals, main against the branch

`probe.md`'s 0.75 column (the spec step's rough run, before the search and the feeding) is beside them:
the branch differs from it by the search's order and the feeding (aligned share 0.236 to 0.256, orders 9.0 to 8.3,
land 1612 to 1591), and every bound still holds.

| Total | Main | Branch | Floor | `probe.md` at 0.75 |
|---|---|---|---|---|
| **Standing** | | | | |
| Aligned share at +200 | 0.209 | **0.256** | 0.008 | 0.236 |
| Top Standing at +200 | 0.106 | 0.110 | 0.008 | 0.108 |
| On land at +200 | 0.0004 | 0.0000 | 0.0012 | |
| **Doubt** (reported, not a bound) | | | | |
| Lead changes (maps) | 10 (4) | 6 (3) | 5 (1) | 3 (1) |
| Lead changes with no claims (maps) | 4 (3) | 2 (2) | 1 (1) | |
| Spread at +100 | 0.069 | 0.061 | 0.007 | |
| Spread at +200 | 0.071 | 0.058 | 0.011 | 0.058 |
| Swings | 7 | 15 | 9 | |
| **Decisions** | | | | |
| AI orders per 100 turns | 13.3 | **8.3** (-37%) | 1.4 | 9.0 (-32%) |
| of which claims | 11.7 | 6.4 | 1.3 | |
| of which raise | 1.39 | 1.78 | 0.08 | |
| of which disband | 0.19 | 0.19 | 0.07 | |
| **Motion** | | | | |
| Land tiles changing hands | 2448 | **1591** (-35%) | 392 | 1612 |
| Clans joined | 123 | 131 | 16 | 90 |
| People changing hands | 570 | 661 | 67 | 461 |
| **Land** | | | | |
| Trading share at the pick | 0.898 | **0.929** | 0.000 | 0.929 |
| Trading share at +200 | 0.897 | **0.927** | 0.001 | 0.925 |
| Festival changes per 100 clan-turns | 0.65 | 0.35 | 0.10 | 0.39 |
| Festivals at the pick / at +200 | 218 / 211 | 211 / 212 | 0 / 3 | 211 |
| Held tiles a civ at +200 | 21.3 | 26.2 | 1.6 | 21.8 |
| Claimed tiles a civ at +200 | 32.4 | 35.1 | 1.6 | |
| Flips per 100 turns | 0.095 | 0.050 | 0.020 | |
| **Health** | | | | |
| Pre-games ended by centres | 7 of 7 | 7 of 7 | | 7 of 7 |
| Store mean | 17.1 | 23.6 | 0.26 | 16.1 |
| Panics, digest mismatches | 0, 0 | 0, 0 | | |
| Camps kept / lost | 21 / 0 | 21 / 0 | | |

Lead changes fall from 10 to 6 and the spread from 0.071 to 0.058 (the spread past its floor). Both are
reported, not bounds (`probe.md`: noise at this count).

## The pre-game sweep (`sansfiat sweep --size all`, seeds 1 to 10 each)

| Size | Main: ended on their own, top-up, median end | Branch | People levelled off |
|---|---|---|---|
| small | 10 of 10, 0, turn 256 | 10 of 10, 0, turn 262 | 10 of 10 on both |
| medium | 10 of 10, 0, turn 375 | 10 of 10, 0, turn 351 | 10 of 10 on both |
| large | 9 of 10 (one on the limit with three centres), 0, turn 369 | 9 of 10 (the same), 0, turn 383 | 9 of 10 on main, 10 of 10 on the branch (the seed at +11% is +10%) |

Every size ends as on main: no top-up anywhere, and the same one large seed reaches its limit with three
centres already.

## The whole `judge --compare` output

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
