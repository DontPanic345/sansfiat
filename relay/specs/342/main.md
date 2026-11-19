# #342 main before herds

`sansfiat sweep --judge --json` on main at 8d1b4ed, in the spec session: 152 s (3 min 16 s with the build). The fields group against the no-fields control, per civ at +200: its farming share, whether it farms (a tenth of its days or more), and its Standing gain over the control.

| Map | Civ | Farming share | Farms | Standing gain |
|---|---|---|---|---|
| small 2 | 467 | 0.06 | no | +0.522 |
| small 2 | 468 | 0.12 | yes | +0.096 |
| small 2 | 469 | 0.01 | no | -0.031 |
| small 3 | 198 | 0.00 | no | -0.094 |
| small 3 | 199 | 0.00 | no | +0.000 |
| small 3 | 200 | 0.07 | no | +0.001 |
| small 6 | 263 | 0.00 | no | +0.000 |
| small 6 | 264 | 0.17 | yes | +0.030 |
| small 6 | 265 | 0.02 | no | -0.015 |
| small 9 | 331 | 0.16 | yes | +0.166 |
| small 9 | 332 | 0.13 | yes | -0.043 |
| small 9 | 333 | 0.01 | no | -0.071 |
| medium 8 | 781 | 0.12 | yes | +0.039 |
| medium 8 | 782 | 0.00 | no | -0.011 |
| medium 8 | 783 | 0.11 | yes | -0.011 |
| medium 9 | 712 | 0.19 | yes | +0.097 |
| medium 9 | 713 | 0.00 | no | -0.042 |
| medium 9 | 714 | 0.13 | yes | +0.137 |
| large 8 | 1885 | 0.00 | no | -0.123 |
| large 8 | 1886 | 0.14 | yes | -0.036 |
| large 8 | 1887 | 0.13 | yes | +0.020 |

Total: 10 of 21 civs farm, gaining +0.049 on average (7 of them gain); the civs that don't farm gain +0.012. Aligned Standing 0.522 against the control's 0.432. Starving 0.033 of clan-turns against 0.039 (ratio 0.85, worst map 1.40).

Eleven civs don't farm: they are the land herds are for. A single civ's gain swings by up to half a point of Standing (small 2, civ 467: +0.52 without farming), so the herds group is read with `--floor`, as fields was.
