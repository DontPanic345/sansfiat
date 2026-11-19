# #342 build-1 (clay): the herd on the actor, read on the retro maps

`sansfiat sweep --judge --json` on the branch with herds on (`on.json`) beside main at 8d1b4ed (`main.json`,
the same session; `main.md` is the spec's reading of it), each 4 to 5 minutes. A temporary print in the
judge's fields group read the herders, animals and harvests (not committed: the judge's herds group is
build-2's). Standing is the civ's at +200 (0 where its capital fell and it has none); the gain is the branch's
less main's, one draw each, so a civ's swing of half a point is divergence, not herds (small 2's 467 and 468).

## Herds off equals main

With `herds: false` in `Config::default()` (not committed), `sweep --judge --json` equals main's in all
19,953 numbers (0 differ, 0 missing; seconds left out). The new `animals` slot, the herding job's share (a
job with share 0 gets nothing and the shares keep their products) and the budget's order are exact.

## Herds on

Columns: herders at +200 on the map (clans with a herding share from `HERD.herder`) and animals held; the
civ's herding share (Σ people × herds ÷ Σ people) at +200; its animal share (Σ `harvest[ANIMALS]` ÷ Σ the
harvest's food over its clans) at +200 and summed over every turn from the pick to +200; its Standing
beside main's.

| Map | Herders | Animals | Civ | Herding share | Animal share (+200) | Animal share (pick to +200) | Standing | Main Standing | Gain |
|---|---|---|---|---|---|---|---|---|---|
| small 2 | 39 | 137 | 467 | 0.14 | 0.029 | 0.023 | +0.345 | +0.522 | -0.177 |
| small 2 |  | 137 | 468 | 0.00 | 0.000 | 0.023 | +0.000 | +0.391 | -0.391 |
| small 2 |  | 137 | 469 | 0.19 | 0.042 | 0.051 | +0.087 | +0.079 | +0.008 |
| small 3 | 17 | 94 | 198 | 0.13 | 0.036 | 0.037 | +0.348 | +0.209 | +0.138 |
| small 3 |  | 94 | 199 | 0.00 | 0.000 | 0.041 | +0.000 | +0.000 | +0.000 |
| small 3 |  | 94 | 200 | 0.14 | 0.028 | 0.035 | +0.265 | +0.272 | -0.006 |
| small 6 | 16 | 67 | 263 | 0.00 | 0.000 | 0.018 | +0.000 | +0.000 | +0.000 |
| small 6 |  | 67 | 264 | 0.06 | 0.013 | 0.013 | +0.183 | +0.192 | -0.009 |
| small 6 |  | 67 | 265 | 0.17 | 0.039 | 0.041 | +0.195 | +0.186 | +0.009 |
| small 9 | 46 | 129 | 331 | 0.07 | 0.013 | 0.016 | +0.496 | +0.554 | -0.058 |
| small 9 |  | 129 | 332 | 0.12 | 0.024 | 0.024 | +0.177 | +0.131 | +0.046 |
| small 9 |  | 129 | 333 | 0.21 | 0.046 | 0.052 | +0.165 | +0.116 | +0.049 |
| medium 8 | 29 | 102 | 781 | 0.18 | 0.034 | 0.036 | +0.098 | +0.140 | -0.042 |
| medium 8 |  | 102 | 782 | 0.19 | 0.037 | 0.038 | +0.060 | +0.058 | +0.002 |
| medium 8 |  | 102 | 783 | 0.13 | 0.025 | 0.026 | +0.061 | +0.065 | -0.004 |
| medium 9 | 22 | 85 | 712 | 0.12 | 0.021 | 0.021 | +0.127 | +0.218 | -0.091 |
| medium 9 |  | 85 | 713 | 0.13 | 0.029 | 0.027 | +0.093 | +0.082 | +0.011 |
| medium 9 |  | 85 | 714 | 0.07 | 0.009 | 0.012 | +0.224 | +0.276 | -0.051 |
| large 8 | 82 | 282 | 1885 | 0.10 | 0.018 | 0.019 | +0.100 | +0.000 | +0.100 |
| large 8 |  | 282 | 1886 | 0.11 | 0.024 | 0.022 | +0.126 | +0.083 | +0.043 |
| large 8 |  | 282 | 1887 | 0.11 | 0.014 | 0.015 | +0.075 | +0.078 | -0.003 |

herder clans 251, animals 897 at +200 over seven maps
civs giving a tenth of days to herds: 14; gain vs main:  -0.001
animal share pick-to-+200: max 0.052 mean 0.028; at +200: max 0.046 mean 0.023
civs with animal share >= 0.1 (pick to +200): 0 ; >=0.05: 2
starving 0.0204 vs main 0.0328 ratio 0.62
mean Standing at +200 over civs: branch 0.1537 main 0.1739

## Reading

- **Against p4** (`probe.md`: 9 civs a tenth of their days on herds, about 200 herder clans, 341 animals,
  1–2% of food, starving 1.15): the clay gives 14 civs a tenth of their days, 251 herder clans and 897
  animals; animals are 1–5% of a civ's food (mean 2.8% over the run, 5.2% at most), no civ a tenth
  (so no civ **herds** by the plan's measure: animals a tenth of its food). Near p4's shape, a little
  more herd: animals are about three times p4's, and the steady yield, not the increase, still decides
  who herds.
- **Starving is 0.62 of main's**, not p4's 1.15 (clan-turns starving 0.0204 against 0.0328). Not
  isolated (no run of its own); two things differ from the probe that may feed a clan: a clan short of `CLAN.stable_at` eats from its herd (up to
  `HERD.lean`), with the animals it left in its basket counted at the diet's own values; and the herd
  starts at half an animal a person from beyond its land. The probe counted the herd in the notch's
  nutrition (up to `diet_share × need`); the clay leaves the breeding stock out of every food sum
  (`worth`, the budget, the basket). The glaze reads whether the drop is the lean buffer or the start.
- **Standing**: mean at +200 over the 21 civs 0.154 against main's 0.174, from small 2 (467 −0.18; 468's
  capital fell, 0.39 to 0), and gains on small 3's 198 (+0.14), large 8's 1885 (+0.10), small 9's 333 and
  332. The civs giving a tenth of their days to herds average −0.001 against main. One draw each, no
  floor: the glaze reads this with `--floor` and the no-herds control.
- Wide land was not read here (the judge's tiles a person by kind is build-2's); the probe found herders narrower than gatherers at these numbers, and nothing in the clay changes why (days bind `K`).
