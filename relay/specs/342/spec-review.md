# #342 spec-review: the runs

The spec step's probe (`probe.diff` on 8d1b4ed, its switches as environment variables), with one
addition to its judge lines: each civ's animals eaten at +200 (`harvest[ANIMALS]`) beside its whole
food harvest, so a civ's **animal share** is what its herds feed it. Every row is one
`sweep --judge --json` on the seven retro maps against the probe with herds off (`NO_HERDS=1`),
read with `cmp.py`'s sums (the same as `probe.md`'s). Four sweeps ran at once on four cores, about
12 minutes each. Sweeps are deterministic: p4 here gives `probe.md`'s p4 number for number.

## Herds cost no time

The sweep with herds on (p4) took 740 s against herds off's 725 s on the same runner, run side by
side: 2%. The probe's 357 s against main's 152 s was the runner, not herds. The bench bound (1.25)
is not at risk from the notch's extra `value` calls.

## Tend makes land wide, and the days measure stops counting herders

`probe.md` reads narrow land as `K` bound by herding days, and tried `tend` 6 only where no clan
herded (p3) or where the offtake changed (p7). Raising `tend` at p4 tests it:

| Run | Setting | Herding civs (a tenth of days) | Their gain | Herders, animals | Tiles a person, herders / gatherers | Ratio | Farming civs | Starving | Worst map |
|---|---|---|---|---|---|---|---|---|---|
| p4 | probe.md's p4 (tend 3) | 9 (8 gain) | +0.033 | 195, 341 | 0.622 / 0.940 | 0.66 | 7, +0.047 | 1.15 | 1.54 |
| t12 | p4, tend 12 | 3 (0 gain) | −0.005 | 71, 396 | 0.921 / 0.744 | 1.24 | 8, +0.041 | 1.26 | 1.89 |
| t24 | p4, tend 24 | 1 (1 gain) | +0.009 | 12, 504 | 1.232 / 0.689 | 1.79 | 8, +0.027 | 1.16 | 1.53 |

- **Wide land comes when grass binds, not days.** At tend 24 a herder holds 1.79 times a gatherer's
  tiles a person (the bound is 1.5); at 12, 1.24. The diagnosis in `probe.md` holds, and its first
  lever is `tend` against `graze`, not the radius or the walk.
- **But the days measure then counts almost no one.** When a day tends many animals, a clan needs
  few days for all its grass: herding shares fall to 0.02–0.11 a civ, herders (share ≥ 0.2) to 12
  clans, and "herding civs" by a tenth of days to 1, while the animals held rise. 16 civs hold 0.05
  animals a person or more at t24. A bound counted in days fights the number that makes land wide.
  So the spec now counts a herding civ by what herds feed it (below).
- **Standing doesn't follow on its own.** Herding civs' gain falls with tend, fields' gain falls to
  +0.027 at tend 24 (under the +0.03 bound), and civs die on maps where the control keeps them
  (198, 200, 333 at t12): one draw each, but the glaze has to read the per-civ table, not only means.

## Herds feed almost nothing

Each civ's animal share at +200 (its clans' animals eaten over their food), highest first:

| Run | Setting | Animal shares, highest civs | Civs ≥ 0.05: n, gain, mean | Herding civs (days) | Herders, animals | Tiles a person ratio | Farming civs | Starving | Worst map |
|---|---|---|---|---|---|---|---|---|---|
| p4 | tend 3 | 0.02 0.02 0.01 … | 0 | 9 (8 gain), +0.033 | 195, 341 | 0.66 | 7, +0.047 | 1.15 | 1.54 |
| t12 | tend 12 | 0.04 0.03 0.02 … | 0 | 3, −0.005 | 71, 396 | 1.24 | 8, +0.041 | 1.26 | 1.89 |
| t24 | tend 24 | 0.03 0.03 0.03 … | 0 | 1, +0.009 | 12, 504 | 1.79 | 8, +0.027 | 1.16 | 1.53 |
| t12m | tend 12, eat only past K ÷ 2 (`H_MSY`) | 0.04 × 9 … | 0 | 2, +0.009 | 37, 1486 | 1.06 | 6, +0.095 | 1.81 | 2.31 |
| t24m | tend 24, eat only past K ÷ 2 | 0.10 0.08 0.06 0.06 0.05 × 4 … | 7, 5 gain, +0.024 | 2, +0.006 | 12, 1669 | 1.02 | 6, +0.110 | 1.79 | 2.27 |
| t24mf | t24m, only a fed clan raises its share (`H_FED`) | 0.07 0.05 0.04 … | 2, +0.002 | 0 | 2, 1073 | 0.78 | 8, +0.058 | 1.80 | 2.42 |
| t48mf | t24mf, tend 48 | 0.06 0.04 0.04 … | 1, +0.064 | 0 | 3, 926 | 0.76 | 6, +0.052 | 1.82 | 2.33 |

- **At p4 herds are 1–2% of any civ's food.** The nine civs that give herds a tenth of their days
  get almost nothing back: the notch reckons the herd's steady yield, `breed × K ÷ 4`, and the
  harvest gives the increase of a herd that never leaves its start. Their +0.033 is the days moved,
  not herds feeding. The arithmetic: feeding a person at breed 0.3 takes about 13 animals a person
  at `K ÷ 2`; a herd started at 0.5 a person that gives up its increase stays near 0.5.
- **Herds that grow feed a little and starve clans.** Eating only past `K ÷ 2` at tend 24 gives the
  most animals and the first civs near a tenth of their food from herds (7 civs at 5% or more, 5 of
  them gaining, +0.024), with starving at 1.79. Only a fed clan raising its share leaves starving at
  1.8, so the starving is not hungry clans taking up herds: it is clans that herd giving days to a
  herd that, below `K ÷ 2`, feeds them nothing, kept there by a reckoning that counts the herd's
  steady yield. The reckoning and the offtake are one rule to change together.
- **Wide land and growing herds pull apart.** Wide land comes at a high `tend` with the whole
  increase eaten (t24, 1.79); with herds kept to grow, herders' land falls back to gatherers'
  (1.02): a growing herd's `K` is not yet bound by the grass.

## What the plan changes

- **A herding civ is counted by its animal share** (a tenth of its clans' food), not its days. The
  days count rose and fell with `tend`, not with what herds feed. The herding share is reported
  beside it. No setting run yet has 4 such civs: the glaze has a rule to find, and the ask if it
  can't.
- **The wide-land bound needs 40 herder clans**, so 12 clans at tend 24 can't pass it alone.
- **The glaze's brief** gives it these runs and the arithmetic, and points it at the reckoning and
  the offtake together.
- **The clay's staged game** goes in `core/tests/herds.rs`: `core/stage/*.json` tests know only the
  scout's facts, and fields has none.
- **The page's layer list** moves out of `www/map/view.js` (605 lines) into
  `www/map/layers/index.js` before the herds layer joins it.
- **Animals' festival-turns** read `Market.ware[ANIMALS]`, which the market already keeps.
- **Herds cost no time**: 2% on the sweep.

The probe with these judge lines is `probe.diff` plus two sums (`harvest[ANIMALS]` and
the harvest's food per civ in `judge_fields.rs`'s probe lines); the runs' JSON was not kept; `spec-review-cmp.py` and `spec-review-share.py` read two sweeps into these tables.
