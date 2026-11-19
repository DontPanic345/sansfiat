# #342 probe: a herd sketched on main and run on the retro maps

A sketch of the clay (`probe.diff`, about 600 lines of diff on 8d1b4ed, not for merging: its
switches are environment variables) run as `sweep --judge --json` on the seven retro maps, each
setting against the probe with herds off (`NO_HERDS=1`, `p0`). Every setting is one draw; one civ
swings by up to half a point (small 2's civ 467 reads −0.522 in every herds run: it is that
divergence, not herds). The judge's fields group carried the herd counts for the probe.

**Herds off equals main.** The new `animals` slot alone, and the whole probe with herds off, give
`sweep --judge --json` equal to main's (`main.md`) in all 19,953 numbers. So the clay's "herds off
equals main" is a done it can meet: with a herding share of 0 every product keeps its order.

**Why no herd started** (p1–p3, and a traced run, `H_DEBUG`): by the pick the game on a clan's own
tiles is hunted out, 0.1 to 7 game for a clan of four to seven, so a herd caught from game starts
at 0 to 3 animals. And the notch weighs this turn's increase: a herd of two on new land yields a
tenth of an animal, which never beats the days it takes. Clans sit near their need (value 0.8 to
1.25 food a person), not at the target, so days are dear.

Columns: herding civs (a tenth of their people's days or more at +200) and how many gain; their
mean Standing gain against herds off; the other civs' mean gain; herder clans and animals held at
+200 over the seven maps; territory tiles a person, herders / gatherers; farming civs and their
gain against no fields (main: 10, +0.049); starving against herds off, overall and the worst map.

| Run | Setting | Herding civs | Their gain | Others | Herders, animals | Tiles a person | Farming civs | Starving | Worst map |
|---|---|---|---|---|---|---|---|---|---|
| p1 | The spec's start: a herd caught from game; the notch weighs this turn's increase | 0 (0 gain) | +nan | -0.018 | 13, 107 | 0.476 / 0.762 | 8, +0.063 | 1.04 | 1.63 |
| p2 | p1, and value counts the increase the target leaves in the herd | 0 (0 gain) | +nan | +0.002 | 13, 119 | 0.364 / 0.704 | 9, +0.075 | 1.05 | 1.48 |
| p3 | p2, breed 0.5, tend 6 | 0 (0 gain) | +nan | -0.015 | 13, 179 | 0.402 / 0.719 | 6, +0.081 | 1.08 | 1.52 |
| p4 | The herd reckoned at its steady yield (breed × K ÷ 4); start 0.5 a person whatever the game | 9 (8 gain) | +0.033 | -0.050 | 195, 341 | 0.622 / 0.940 | 7, +0.047 | 1.15 | 1.54 |
| p5 | p4, breed 0.5 | 12 (6 gain) | +0.009 | -0.057 | 217, 403 | 0.640 / 0.765 | 8, +0.060 | 1.79 | 2.03 |
| p6 | p4, and a clan eats only what its herd grows past K ÷ 2 | 7 (4 gain) | +0.032 | -0.060 | 137, 860 | 0.504 / 0.791 | 5, +0.054 | 1.86 | 2.54 |
| p7 | p6, tend 6 | 6 (4 gain) | +0.041 | -0.017 | 156, 1417 | 0.573 / 0.816 | 5, +0.043 | 2.02 | 2.78 |
| p8 | p6, and only a fed clan raises its herd share | 2 (2 gain) | +0.064 | -0.039 | 73, 557 | 0.436 / 0.750 | 6, +0.062 | 1.50 | 2.14 |
| p9 | p4, only a fed clan raises it, and herding walks at 0.25 | 1 (0 gain) | -0.007 | -0.013 | 96, 210 | 0.545 / 0.696 | 9, +0.084 | 1.07 | 1.30 |

Start numbers in every run unless the row says: graze plains 6, hills 4, coast, forest and desert
1, marsh 2; tend 3; breed 0.3; start 0.5; lean 0.25; notch 0.1; margin 0.01; herder 0.2; radius 3;
walk 0.5; animals weigh 0.5.

## What it settles

- **The herd's start** pays from game as far as it goes and isn't capped by it (p4 against p1).
- **The notch reckons the steady yield**, `breed × K ÷ 4`: the step from 0 civs herding to 9 (p4).
- **p4 is the clay's start**: 9 civs herd and 8 gain, +0.033 on average; starving 1.15 times herds
  off (worst map 1.54); farming civs still gain +0.047. The others' −0.050 is mostly civ 467.

## What it leaves to the glaze

- **The herds are a trickle.** In p4, 195 herder clans hold 341 animals: eating every increase
  keeps a herd at its start. Eating only what grows past `K ÷ 2` (p6, p7) grows herds three to
  four times, but starving doubles (1.86 to 2.02), since a clan gives days to a herd that feeds it
  later. Only a fed clan raising its share (p8, p9) cuts herding to one or two civs.
- **No setting gives wide land.** Herders hold 0.36 to 0.64 tiles a person against gatherers' 0.70
  to 0.94. A herd share of a tenth or a fifth gives few herding days, so `K` is bound by the days,
  not the grass, and more grass adds nothing to the steady yield; the radius and the walk don't
  bind. The glaze's first look: the days against the grass (`tend`, `graze`), and whether a
  herder's added tile is weighed by the grass it adds.
- **Breed 0.5** brings more civs in but starving to 1.79 (p5).

Per-civ table for p4: `probe-p4.txt`.
