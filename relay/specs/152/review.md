# #152 review: the judge's table

`sweep --judge --json` on main (2441f7c's base, origin/main) and on relay/152, the branch run with `--floor`
(one extra sweep), then `judge --compare main.json branch.json`. Main 77 s and the branch ~70 s for the
floored pair, run side by side.

## Verdict: not shown

- **The question (early ends):** games ended before the limit 0 of 7 in the sweep (no fight 0 of 7), and
  0 of 21 `conquest` games on the stories' maps (control 0 of 21). The conquest story takes one capital
  in 3 of 21 games and never a second. Reproduced here: small 6 civ 2 (fall at +14, rank 1 at the end
  against 2 in the AI and idle controls) and small 3 civ 3 (fall at +51); both `--no-fight` controls 0.
- **Every game ends:** all 7 AI runs and all 7 no-fight runs end by the limit (+200 small, +250 medium,
  +300 large; turns to the end 228.6 both).
- **Doubt fell in AI play**, from the start (no AI army): maps with a lead change 6 → 4 (floor 1),
  swings 20 → 7 (floor 4), spread +200 0.063 → 0.071 (floor 0.002). Lead changes 10 → 10.
- **Standing** top +200 0.096 → 0.106 (floor 0.004): the start, not a lever.
- **Decisions:** AI orders per 100 turns 21.9 → 13.3 (floor 0.12): the AI raises and moves no army.
- **Motion:** clans changed 127 → 122 (floor 17), within the drift.
- **Health:** digest mismatches 0, panics 0, `check.sh` green.

## Also run

- `play --game` on small 2 with `limit: 3` written into the save: the view's first line says
  "the game is over: Gold won, the highest Standing at the turn limit, turn +3"; a raise and a setKeep
  are refused with "the game is over"; `end` plays on; `judge` prints
  "end (game / control / idle): limit +3 rank 3 / limit +3 rank 3 / limit +3 rank 3".
- `www/play/end.mjs`, `pick.mjs`, `buttons.mjs` and the other 13 scripts pass (in `check.sh`).

## The compare

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2001      0.2086      0.0084      0.0082 *
  total.standing.on_land_200                                  0.0013      0.0004     -0.0009      0.0004 *
  total.standing.top_200                                      0.0957      0.1055      0.0098      0.0036 *
doubt
  total.doubt.lead_changes                                   10.0000     10.0000      0.0000      2.0000
  total.doubt.maps_with_lead_change                           6.0000      4.0000     -2.0000      1.0000 *
  total.doubt.no_claims_lead_changes                          5.0000      4.0000     -1.0000      1.0000
  total.doubt.no_claims_maps_with_lead_change                 3.0000      3.0000      0.0000      1.0000
  total.doubt.spread_100                                      0.0614      0.0687      0.0073      0.0065 *
  total.doubt.spread_200                                      0.0625      0.0705      0.0080      0.0020 *
  total.doubt.swings                                         20.0000      7.0000    -13.0000      4.0000 *
decisions
  total.decisions.ai_orders_per_100                          21.8571     13.3206     -8.5365      0.1238 *
  total.decisions.by_kind.claim                              15.9762     11.7397     -4.2365      0.2746 *
  total.decisions.by_kind.disband                             1.1190      0.1905     -0.9286      0.0714 *
  total.decisions.by_kind.move                                3.0952           -   only in A      0.0000
  total.decisions.by_kind.raise                               1.0476      1.3905      0.3429      0.0794 *
  total.decisions.by_kind.zone                                0.6190           -   only in A      0.0000
  total.decisions.levers_repeated                             3.0952      1.6190     -1.4762      0.0000 *
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                127.0000    122.0000     -5.0000     17.0000
  total.motion.clans_joined                                 130.0000    123.0000     -7.0000     16.0000
  total.motion.clans_left                                     0.0000      0.0000      0.0000      0.0000
  total.motion.land_changed                                2425.0000   2448.0000     23.0000    392.0000
  total.motion.people_changed                               594.5137    569.5084    -25.0052     66.6352
land
  total.land.claimed_200                                     32.9524     32.4286     -0.5238      0.4286 *
  total.land.festival_changes_per_100                         0.8746      0.6546     -0.2201      0.0119 *
  total.land.flips_per_100                                    0.0925      0.0950      0.0025      0.0083
  total.land.guard_200                                        2.4286      2.7619      0.3333      0.0000 *
  total.land.held_200                                        16.3333     21.3333      5.0000      1.1429 *
  total.land.homes_trailed.0                                  0.8458      0.8456     -0.0002      0.0000 *
  total.land.homes_trailed.1                                  0.8320      0.8505      0.0185      0.0157 *
  total.land.markets_200                                    211.0000    211.0000      0.0000      2.0000
  total.land.markets_pick                                   218.0000    218.0000      0.0000      0.0000
  total.land.route_trailed.0                                  0.9652      0.9652      0.0000      0.0000
  total.land.route_trailed.1                                  0.9852      0.9782     -0.0070      0.0056 *
  total.land.trading_200                                      0.8908      0.8968      0.0060      0.0006 *
  total.land.trading_pick                                     0.8979      0.8979      0.0000      0.0000
  total.land.worked_trailed.0                                 0.3096      0.3096      0.0000      0.0000
  total.land.worked_trailed.1                                 0.3026      0.2964     -0.0062      0.0061 *
economy
  total.economy.guard_mean                                    2.1419      2.6409      0.4990      0.0223 *
  total.economy.largest_army                                  3.0000      0.0000     -3.0000      0.0000 *
  total.economy.levy_per_turn                                 8.0224      7.8296     -0.1927      0.1382 *
  total.economy.store_max                                    47.1351     48.0917      0.9565      0.0000 *
  total.economy.store_mean                                    8.1377     17.1076      8.9698      0.1988 *
  total.economy.turns_at_pack                               106.7143    284.5714    177.8571     26.0000 *
armies
  total.armies.camps_kept                                    13.0000     21.0000      8.0000      0.0000 *
  total.armies.camps_lost                                     6.0000      0.0000     -6.0000      0.0000 *
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9932      0.9897     -0.0035      0.0078
  total.armies.warriors_starved                              26.0000      0.0000    -26.0000      0.0000 *
fight
  total.fight.control_fallen                                  0.0000      0.0000      0.0000      0.0000
  total.fight.control_surrendered                             0.0000      0.0000      0.0000      0.0000
  total.fight.control_taken                                   0.0000      0.0000      0.0000      0.0000
  total.fight.fallen                                          0.0000      0.0000      0.0000      0.0000
  total.fight.surrendered                                     0.0000      0.0000      0.0000      0.0000
  total.fight.taken                                           0.0000      0.0000      0.0000      0.0000
end
  total.end.capitals_fallen                                        -      0.0000   only in B      0.0000
  total.end.early                                                  -      0.0000   only in B      0.0000
  total.end.no_fight_capitals_fallen                               -      0.0000   only in B      0.0000
  total.end.no_fight_early                                         -      0.0000   only in B      0.0000
  total.end.no_fight_turns_mean                                    -    228.5714   only in B      0.0000
  total.end.of                                                     -      7.0000   only in B      0.0000
  total.end.turns_mean                                             -    228.5714   only in B      0.0000
health
  total.health.digest_mismatches                              0.0000      0.0000      0.0000      0.0000
  total.health.panics                                         0.0000      0.0000      0.0000      0.0000
  total.health.pregame_by_centres                             7.0000      7.0000      0.0000      0.0000
listed 65, changed 37, flagged 30, only in A 2, only in B 7
```
