# #151 build-5: the economy group, from `sansfiat sweep --judge`

Run on the branch after build-5's economy commit (`engine/cli/src/sweep_judge.rs`'s `total economy`
line, the same text as the report):

```
total economy: store mean 8.14, store max 47.14, at pack 106.7 civ-turns a map, levy per turn 8.02, largest army 3.0, guard mean 2.14
```

The store and the levy are means over every civ-turn of the 200 turns past the pick, on the seven
retro maps, so they are not the first turns. The line is `Total.economy`'s means over maps (store
max and largest army are the largest over maps).

**Do the numbers agree with the spec-review's measure (#104)?** Partly. The store does start at 0.0
at the pick, as #104 had it (the pick's civ stores are 0.00 on seeds 2 and 3). The levy is not 3–4
food a turn on the first turns: measured on the retro picks of seeds 2 and 3 for four turns after the
pick, each civ's levy was 2–10 food a turn (seed 2: 6.2, 2.5, 10.3 on turn 1; seed 3: 4.3, 2.0, 8.4),
a mean of about 5–6 a civ. After most of those turns a civ's store still reads 0.00, so the levy's
food is gone from the store by the turn's end; I did not trace where it goes. The 200-turn store mean
(8.14) is the level the stores reach later, not the first turns'. The first-turn figures are from a
temporary test (not kept) that read each civ's `levied` and store after each turn.
