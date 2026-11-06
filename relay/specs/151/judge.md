# #151 review: the judge

**Verdict: same.** The piece adds tools and changes no rule, so no score moves. `sweep --judge --json` on main
(`origin/main` at 4e143c4) against the branch: 0 of 1,811 metrics changed (totals and per map),
0 flagged; the 6 economy totals (132 per-map paths) are only in B. main against itself: 0 changed.

Commands (release builds, 2026-11-06):

    sansfiat sweep --judge --json > A.json                 # main worktree
    sansfiat sweep --judge --json > B.json                 # branch
    sansfiat sweep --judge --json --floor > Bf.json        # branch, one null
    sansfiat judge --compare A.json B.json [--all]
    sansfiat judge --compare A.json Bf.json

Floors (one null): 20 of 58 totals are 0. The four scores' floors: standing top_200 0.0138,
aligned_200 0.0125, on_land_200 0.0006; doubt lead_changes 1, swings 2, spread_200 0.0171; motion
clans_changed 21, people_changed 120.3, land_changed 176. With floors, A against Bf flags 0.

The branch's economy group, per map:

| map | store_mean | store_max | turns_at_pack | levy_per_turn | largest_army | guard_mean |
|---|---|---|---|---|---|---|
| small 2 | 0.67 | 7.58 | 0 | 6.51 | 3.0 | 2.45 |
| small 3 | 3.73 | 6.95 | 0 | 5.00 | 3.0 | 0.07 |
| small 6 | 0.68 | 6.08 | 0 | 3.39 | 3.0 | 0.34 |
| small 9 | 14.90 | 35.41 | 247 | 11.21 | 3.0 | 3.56 |
| medium 8 | 2.14 | 15.44 | 2 | 5.51 | 3.0 | 1.72 |
| medium 9 | 8.67 | 42.80 | 144 | 9.75 | 3.0 | 2.27 |
| large 8 | 26.18 | 47.14 | 354 | 14.79 | 3.0 | 4.58 |

`largest_army` is 3.0 on every map: no AI camp grows past its founding size in 200 turns.

## The full compare (totals, A = main, B = branch with floor)

```
metric                                                             A           B      change       floor
standing
  total.standing.aligned_200                                  0.2001      0.2001      0.0000      0.0125
  total.standing.on_land_200                                  0.0013      0.0013      0.0000      0.0006
  total.standing.top_200                                      0.0957      0.0957      0.0000      0.0138
doubt
  total.doubt.lead_changes                                   10.0000     10.0000      0.0000      1.0000
  total.doubt.maps_with_lead_change                           6.0000      6.0000      0.0000      0.0000
  total.doubt.no_claims_lead_changes                          5.0000      5.0000      0.0000      1.0000
  total.doubt.no_claims_maps_with_lead_change                 3.0000      3.0000      0.0000      0.0000
  total.doubt.spread_100                                      0.0614      0.0614      0.0000      0.0032
  total.doubt.spread_200                                      0.0625      0.0625      0.0000      0.0171
  total.doubt.swings                                         20.0000     20.0000      0.0000      2.0000
decisions
  total.decisions.ai_orders_per_100                          21.8571     21.8571      0.0000      1.5000
  total.decisions.by_kind.claim                              15.9762     15.9762      0.0000      1.9048
  total.decisions.by_kind.disband                             1.1190      1.1190      0.0000      0.1905
  total.decisions.by_kind.move                                3.0952      3.0952      0.0000      0.3571
  total.decisions.by_kind.raise                               1.0476      1.0476      0.0000      0.3810
  total.decisions.by_kind.zone                                0.6190      0.6190      0.0000      0.1905
  total.decisions.levers_repeated                             3.0952      3.0952      0.0000      0.0476
motion
  total.motion.claim_taken                                    0.0000      0.0000      0.0000      0.0000
  total.motion.clans_changed                                127.0000    127.0000      0.0000     21.0000
  total.motion.clans_joined                                 130.0000    130.0000      0.0000     19.0000
  total.motion.clans_left                                     0.0000      0.0000      0.0000      0.0000
  total.motion.land_changed                                2425.0000   2425.0000      0.0000    176.0000
  total.motion.people_changed                               594.5137    594.5137      0.0000    120.2582
land
  total.land.claimed_200                                     32.9524     32.9524      0.0000      0.5238
  total.land.festival_changes_per_100                         0.8746      0.8746      0.0000      0.1384
  total.land.flips_per_100                                    0.0925      0.0925      0.0000      0.0062
  total.land.guard_200                                        2.4286      2.4286      0.0000      0.2381
  total.land.held_200                                        16.3333     16.3333      0.0000      2.4762
  total.land.homes_trailed.0                                  0.8458      0.8458      0.0000      0.0000
  total.land.homes_trailed.1                                  0.8320      0.8320      0.0000      0.0053
  total.land.markets_200                                    211.0000    211.0000      0.0000      2.0000
  total.land.markets_pick                                   218.0000    218.0000      0.0000      0.0000
  total.land.route_trailed.0                                  0.9652      0.9652      0.0000      0.0000
  total.land.route_trailed.1                                  0.9852      0.9852      0.0000      0.0044
  total.land.trading_200                                      0.8908      0.8908      0.0000      0.0121
  total.land.trading_pick                                     0.8979      0.8979      0.0000      0.0000
  total.land.worked_trailed.0                                 0.3096      0.3096      0.0000      0.0000
  total.land.worked_trailed.1                                 0.3026      0.3026      0.0000      0.0074
economy
  total.economy.guard_mean                                         -      2.1419   only in B      0.2307
  total.economy.largest_army                                       -      3.0000   only in B      0.0000
  total.economy.levy_per_turn                                      -      8.0224   only in B      0.2939
  total.economy.store_max                                          -     47.1351   only in B      0.9261
  total.economy.store_mean                                         -      8.1377   only in B      0.8653
  total.economy.turns_at_pack                                      -    106.7143   only in B     27.0000
armies
  total.armies.camps_kept                                    13.0000     13.0000      0.0000      2.0000
  total.armies.camps_lost                                     6.0000      6.0000      0.0000      1.0000
  total.armies.camps_of                                      21.0000     21.0000      0.0000      0.0000
  total.armies.people_vs_control                              0.9932      0.9932      0.0000      0.0043
  total.armies.warriors_starved                              26.0000     26.0000      0.0000      3.0000
fight
  total.fight.control_fallen                                  0.0000      0.0000      0.0000      0.0000
  total.fight.control_surrendered                             0.0000      0.0000      0.0000      0.0000
  total.fight.control_taken                                   0.0000      0.0000      0.0000      0.0000
  total.fight.fallen                                          0.0000      0.0000      0.0000      0.0000
  total.fight.surrendered                                     0.0000      0.0000      0.0000      0.0000
  total.fight.taken                                           0.0000      0.0000      0.0000      0.0000
health
  total.health.digest_mismatches                              0.0000      0.0000      0.0000      0.0000
  total.health.panics                                         0.0000      0.0000      0.0000      0.0000
  total.health.pregame_by_centres                             7.0000      7.0000      0.0000      0.0000
listed 58, changed 0, flagged 0, only in A 0, only in B 6
```

## The other checks the spec names

- `engine/check.sh`: exit 0 on the branch.
- `cargo test -p engine --test cases`: 5 pass. The search order (`small 9 turn`) and the river bridge
  (`small 2 to the rival turn`) are `should_panic`, red for their stated messages (cases.md).
- `node www/check.mjs`: the four Disband all page presses are `expectBug`, red on main.
- `node www/play/buttons.mjs`: exit 0, 34 cases over 25 labels. The review's control: Forage's order in
  `www/panels.js` changed to `pillage` makes it exit 1 with
  `camp Forage: Forage is pressed on the camp (it is greyed on the led civ's own land)`.
- `judge FILE` on `story skirmish --size small --seed 2`: `led civ 472: Standing at +80 10.7% (rank 2)
  in the game, 2.8% (rank 3) with every civ on the AI, 3.7% (rank 3) idle led civ`. Two runs' JSON are
  identical.
- `bench --size large 8 --after 50`: pre-game 4938.6 ms, 50 turns 1148.2 ms, 22.965 ms a turn.
