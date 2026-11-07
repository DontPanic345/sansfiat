# #154 review: the judge

`sansfiat sweep --judge --json --floor` on the branch (59aa5c7 plus the review's ledger) and on `main`
(270d0d3), then `sansfiat judge --compare main.json branch.json`. The branch reproduces build-4's setting J in
`walk.md` exactly. The two last-bit nulls (`CLAN.work` × (1 ± 10⁻¹²)) are build-4's runs, in `walk.md`. The
review didn't re-run them.

## Verdict: not shown

The piece's motion question, "a walk costs more for a clan that carries more", holds on a built stage
(`core/tests/markets.rs`) but not on the retro maps. At +100, 99.8% of clans with a festival route send one walker
(`walk.md`, The walk), so the per-person festival walk changes nothing measurable there. The health questions hold:
- build-1's digests match main for seeds 1, 2 and 9;
- `day.rs` shows no reach changes;
- under the AI, armies inside the ring don't starve, and from share 1.25 up they do (0.000 against 0.53);
- scouts inside the ring are hungry on 3 of 2,811 turns.

| score | main | branch | floor | nulls (walk.md) |
|---|---|---|---|---|
| Standing: aligned at +200 | 0.256 | 0.253 | 0.005 | |
| Standing: top at +200 | 0.110 | 0.107 | 0.008 | |
| doubt: lead changes | 6 | 9 | 6 | |
| doubt: spread at +100 | 0.061 | 0.055 | 0.003 * | |
| decisions: AI orders per 100 | 8.34 | 6.91 | 1.44 | 6.42–8.82 |
| motion: clans joined | 131 | 125 | 16 | 106–141 |
| motion: people changed | 661 | 629 | 49 | 590–648 |
| motion: land changed | 1591 | 1022 | 345 * | 1264–1694 |
| land: held at +200 | 26.2 | 33.3 | 1.6 * | 23.6–26.1 |
| land: guard at +200 | 3.43 | 4.48 | 0.14 * | 2.86–3.42 |
| economy: levy a turn | 7.21 | 9.18 | 0.12 * | 6.77–7.23 |
| health: pre-games by centres | 7 | 7 | 0 | 7, 7 |
| health: panics, digest mismatches | 0, 0 | 0, 0 | 0 | |

Most of the fall in land changed comes from medium 9: 551 to 74 tiles, against a floor of 63. A last-bit null puts
that map at 35, so the number moves by chance there. On the other six maps it is 948 against main's 1040, inside the floor.
Held land, the guard and the levy rise past both nulls. The cause is the guard's walk: each guard now pays it, at the
camp's weight of 0.2 (it was 0.02), and the AI raises more people to hold its claim. That is a real move, and a side
effect rather than the piece's question.

## The full compare

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2556      0.2525     -0.0031      0.0048
  total.standing.on_land_200                                  0.0000      0.0003      0.0003      0.0012
  total.standing.top_200                                      0.1101      0.1069     -0.0033      0.0083
doubt
  total.doubt.lead_changes                                    6.0000      9.0000      3.0000      6.0000
  total.doubt.maps_with_lead_change                           3.0000      4.0000      1.0000      1.0000
  total.doubt.no_claims_lead_changes                          2.0000      6.0000      4.0000      1.0000 *
  total.doubt.no_claims_maps_with_lead_change                 2.0000      4.0000      2.0000      1.0000 *
  total.doubt.spread_100                                      0.0613      0.0551     -0.0062      0.0031 *
  total.doubt.spread_200                                      0.0576      0.0497     -0.0080      0.0107
  total.doubt.swings                                         15.0000      7.0000     -8.0000      9.0000
decisions
  total.decisions.ai_orders_per_100                           8.3397      6.9111     -1.4286      1.4444
  total.decisions.by_kind.claim                               6.3683      4.3222     -2.0460      1.3302 *
  total.decisions.by_kind.disband                             0.1873      0.3063      0.1190      0.0603 *
  total.decisions.by_kind.raise                               1.7841      2.2825      0.4984      0.0810 *
  total.decisions.levers_repeated                             1.6667      1.5238     -0.1429      0.1429 *
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                130.0000    127.0000     -3.0000     17.0000
  total.motion.clans_joined                                 131.0000    125.0000     -6.0000     16.0000
  total.motion.clans_left                                     0.0000      2.0000      2.0000      1.0000 *
  total.motion.land_changed                                1591.0000   1022.0000   -569.0000    345.0000 *
  total.motion.people_changed                               660.5567    629.3674    -31.1893     48.9407
land
  total.land.claimed_200                                     35.1429     35.9524      0.8095      1.6190
  total.land.festival_changes_per_100                         0.3459      0.3071     -0.0389      0.0976
  total.land.flips_per_100                                    0.0504      0.0347     -0.0157      0.0196
  total.land.guard_200                                        3.4286      4.4762      1.0476      0.1429 *
  total.land.held_200                                        26.1905     33.2857      7.0952      1.6190 *
  total.land.homes_trailed.0                                  0.8651      0.8785      0.0134      0.0000 *
  total.land.homes_trailed.1                                  0.8625      0.8693      0.0068      0.0029 *
  total.land.markets_200                                    212.0000    211.0000     -1.0000      3.0000
  total.land.markets_pick                                   211.0000    217.0000      6.0000      0.0000 *
  total.land.route_trailed.0                                  0.9884      0.9904      0.0020      0.0000 *
  total.land.route_trailed.1                                  0.9948      0.9951      0.0002      0.0038
  total.land.trading_200                                      0.9274      0.9334      0.0060      0.0013 *
  total.land.trading_pick                                     0.9292      0.9377      0.0085      0.0000 *
  total.land.worked_trailed.0                                 0.3171      0.3084     -0.0087      0.0000 *
  total.land.worked_trailed.1                                 0.3000      0.3093      0.0093      0.0010 *
economy
  total.economy.guard_mean                                    3.3168      4.0548      0.7380      0.0947 *
  total.economy.largest_army                                  0.0000      0.0000      0.0000      0.0000
  total.economy.levy_per_turn                                 7.2091      9.1784      1.9692      0.1188 *
  total.economy.store_max                                    52.0085     52.1718      0.1633      0.0345 *
  total.economy.store_mean                                   23.6401     25.8845      2.2444      0.5679 *
  total.economy.turns_at_pack                               390.4286    425.5714     35.1429     26.7143 *
armies
  total.armies.camps_kept                                    21.0000     21.0000      0.0000      0.0000
  total.armies.camps_lost                                     0.0000      0.0000      0.0000      0.0000
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9825      0.9883      0.0058      0.0086
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
supply
  total.supply.clan_territory_200                                  -      3.2752   only in B      0.0047
  total.supply.clan_territory_pick                                 -      3.2887   only in B      0.0000
  total.supply.counts.army_inside                                  -      0.0000   only in B      0.0000
  total.supply.counts.army_inside_hungry                           -      0.0000   only in B      0.0000
  total.supply.counts.army_outside                                 -      0.0000   only in B      0.0000
  total.supply.counts.army_outside_hungry                          -      0.0000   only in B      0.0000
  total.supply.counts.held_ring_200                                -    585.0000   only in B     35.0000
  total.supply.counts.held_ring_pick                               -     21.0000   only in B      0.0000
  total.supply.counts.ring_200                                     -    737.0000   only in B      2.0000
  total.supply.counts.ring_pick                                    -    705.0000   only in B      0.0000
  total.supply.counts.scout_inside                                 -   2811.0000   only in B     44.0000
  total.supply.counts.scout_inside_hungry                          -      3.0000   only in B      2.0000
  total.supply.counts.scout_outside                                -   1555.0000   only in B    182.0000
  total.supply.counts.scout_outside_hungry                         -     23.0000   only in B      1.0000
listed 77, changed 41, flagged 24, only in A 0, only in B 14
```
