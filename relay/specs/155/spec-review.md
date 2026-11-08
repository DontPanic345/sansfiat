# #155 spec-review: the runs

The spec step's probe (`probe.diff`) with two additions, `probe-2.diff` in full: `F_TAME` adds steps of tame a
turn to a tile that yielded field grain (held past `TENDING.after`), as build-1's tame under fields does; and
the judge's `PROBEW` line adds **settled farmers**: clans with `fields ≥ F_FARMER` (0.1) for ten turns in a row
before the turn, against gatherers (share 0). Every run is `sansfiat sweep --judge --json --floor` on main
(85868ed) with the probe applied, all civs on the AI, the seven retro maps. The control is `probe.md`'s `off`
run, which equals main.

## Starving is noisy per map

`--floor` plays the whole sweep once more with one harmless roll after the pick: the null. The probe and the
spec's table read only the first sweep. Starving clan-turns over the control's share, per map, the sweep / the
null (both against the one control in `probe.md`):

| setting | s2 | s3 | s6 | s9 | m8 | m9 | l8 | all, sweep | all, mean of the two |
|---|---|---|---|---|---|---|---|---|---|
| y8 (probe.md, reproduced) | 0.76/0.82 | 1.13/1.92 | 0.93/0.84 | 1.41/2.31 | 1.38/1.33 | 1.53/1.44 | 1.48/1.44 | 1.296 | 1.321 |
| y8, tame 1 (build-1's numbers) | 0.75/0.68 | 1.30/1.78 | 0.68/0.96 | 1.48/1.19 | 1.42/1.52 | 1.32/1.86 | 1.58/1.51 | 1.312 | 1.343 |
| y8, tame 1, margin 0.03 | 1.02/0.67 | 1.91/1.75 | 1.17/1.02 | 1.26/1.77 | 1.14/1.36 | 1.17/1.47 | 1.55/1.42 | 1.333 | 1.316 |
| y8, tame 1, no home rule | 0.74/0.71 | 1.20/1.27 | 0.89/1.17 | 1.89/1.16 | 1.19/1.46 | 1.76/2.05 | 1.53/1.40 | 1.339 | 1.339 |
| y8, tame 1, flood 10 days | 0.74/0.83 | 1.30/1.78 | 0.68/0.96 | 1.85/1.57 | 1.22/1.46 | 1.36/1.80 | 1.42/1.42 | 1.239 | 1.304 |
| y6, tame 1 | 0.66/0.66 | 1.10/1.07 | 1.14/1.33 | 1.69/1.80 | 1.36/1.46 | 1.60/1.91 | 1.23/1.12 | 1.210 | 1.220 |
| y4, tame 1 | 0.63/0.81 | 1.77/1.92 | 1.48/1.06 | 0.99/1.48 | 1.43/1.44 | 1.37/1.53 | 1.11/1.03 | 1.153 | 1.164 |

One map's two plays of the same setting differ by up to 1.6 times (y8 small 9: 1.41 against 2.31). Over the
seven maps they differ by about 5%. So a per-map bound read from one play is mostly noise, and flood 10's 1.239
"pass" is gone in the null (1.37) and on the mean of the two (1.304). The spec reads the starving bound past the
floor, as it reads every move: the floor's null has its own no-fields control, so the floor holds the
control's noise too.

## The rest of each setting (the sweep, not the null)

| setting | farming civs: n, mean gain, gained | others' mean | never at a pack | gatherers: home moves / 100 clan-turns, starving | settled farmers: moves, starving |
|---|---|---|---|---|---|
| y8 | 12, +0.0362, 11 | −0.0017 | 2 | not measured | not measured |
| y8, tame 1 | 13, +0.0377, 12 | −0.0083 | 1 | 0.232, 2.93% | 0.017, 0.23% |
| y8, tame 1, margin 0.03 | 10, +0.0460, 10 | +0.0008 | 1 | 0.192, 2.95% | 0.025, 0.21% |
| y8, tame 1, no home rule | 11, +0.0424, 11 | −0.0019 | 2 | 0.145, 2.97% | 0.082, 0.29% |
| y8, tame 1, flood 10 | 12, +0.0384, 11 | −0.0034 | 0 | 0.206, 2.76% | 0.013, 0.21% |
| y6, tame 1 | 14, +0.0344, 13 | −0.0095 | 2 | 0.146, 2.65% | 0.025, 0.29% |
| y4, tame 1 | 14, +0.0313, 12 | −0.0071 | 2 | 0.153, 2.50% | 0.034, 0.25% |

The control's starving share over all clans is 1.77%.

## What it says

- **Tame under fields barely moves the probe.** Build-1's numbers (y8, tame 1) give farming civs +0.038, 12 of
  13 gaining, 1 civ never at a pack, starving 1.31 times the control. Build-1's "near y8" check holds.
- **Settled farmers stop moving.** Bound 4 passes with room on every setting: settled farmers move a tenth of
  the gatherers' rate or less, and the home rule's walks fall before the tenth farming turn.
- **The starving is the gatherers'.** Settled farmers starve at 0.2–0.3% of clan-turns; gatherers at 2.5–3.0%,
  above the control's 1.77% over all clans.
- **Only the yield moves it.** Margin 0.03, no home rule and half the floodplain ground all leave starving at
  1.30–1.34 on the mean of the two. Yield 0.6 gives 1.22 and yield 0.4 gives 1.16, which meet bound 6, but bound 2
  needs `FIELD.grain ≥ 0.75` (a field day at a hearth 1.5 times a gathering day's 0.75 food). So bounds 2 and 6
  pull against each other through the yield: with grain at 0.8, every setting is at 1.30 or more on the mean of
  the two. Read past the floor, as the spec now reads it, y8 with tame sits on the line, so a pass there may be
  noise. The canvas has to find why
  gatherers beside farmers starve and change the rule that causes it, or send the ask with the nearest.
- **Time.** A sweep takes 5 min 15 s with two running at once on the runner's four cores (five runs a map).
  With build-2's sixth run, expect about 6 minutes a pair.
