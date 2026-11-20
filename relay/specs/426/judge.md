# #426 review: the judge

Sweeps read from `main-sweep.json` (main 0fd7835, with this piece's settling counts patched in, judge-only)
and `branch-sweep.json` (1bb905e; nothing in `engine/core` or `engine/cli` changed after it).
`compare.txt` is `sansfiat judge --compare` on the two. The Goal's table is in `sweep.md`.

## Verdict: worse in the four scores; the question's answer is yes for towns, no for capitals

| score | main | branch | floor | past floor |
|---|---|---|---|---|
| Standing: aligned share at +200 | 0.522 | 0.464 | 0.008 | down |
| Standing: top civ at +200 | 0.291 | 0.306 | 0.008 | up |
| doubt: spread at +200 | 0.243 | 0.270 | 0.010 | wider (less doubt) |
| doubt: swings | 14 | 6 | 6 | down |
| doubt: lead changes | 7 | 8 | 3 | no |
| decisions: AI orders per 100 turns | 85.7 | 62.5 | 5.2 | down |
| motion: claims taken | 220 | 54 | 79 | down |
| motion: land changed | 1024 | 719 | 79 | down |
| motion: clans changed | 410 | 408 | 14 | no |
| health: pre-games by centres | 7/7 | 7/7 | | latest 314 (main 467) |
| health: panics, digest mismatches | 0, 0 | 0, 0 | | |
| health: starving share | 3.3% | 2.7% | 0.2% | down (better) |

The Goal's scores (`sweep.md`):

| score | main | branch | floor |
|---|---|---|---|
| towns at +200 | 10 | 19 | 1 |
| capital towns at +200 | 5 | 3 | 0 |
| clans in towns at +200 | 59 | 84 | 8 |
| trade a person a turn: town clans / farmers outside | 0.043 / 0.038 | 0.210 / 0.076 | 0.003 / 0.006 |
| markets alive at +200 | 205 | 210 | 8 |
| clans at a market at +200 | 0.924 | 0.951 | 0.005 |
| born a person a turn: settled / gathering | 0.0034 / 0.0037 | 0.0084 / 0.0041 | |
| crowding deaths (people, 7 maps) | 0 | 4138 | |

Towns gain on four maps (small 3, 6, 9, large 8), so the move is past the floor on more than one
map. The mechanism is the claimed one: with crowding off, towns at +200 are 17 too (`sweep.md`,
variants), so the first load and the home step's trade gain draw the farmers. Crowding thins them.
Play: none, so no led player was measured against an idle one.

## Read cold

- `markets::load` matches the labour.md line: the first load of 2 goes whatever the clan has, each
  further load of 2 comes from the surplus while its gain × `harvest_labour` is more than the extra
  walker's days, the clan stops at the first load that doesn't pay, and at the door it brings
  `max(first, surplus)`. Walkers are `max(weight / 2, 1)`. No cap.
- `clans::growth(settled)`, `child_days` (out of the next harvest, in `fields::jobs`), and `crowding`
  (home-tile people counted once a turn, before growth, fed or not) do what the Decided lines say.
- No defects found. `trade` reads the walk back as route cost ÷ `walkers.max(1)`, so an old save's
  `walkers` of 0 can't divide by zero.
- farming.md's #427 ledger line ("Settling is a reading … no rule reads it") is superseded by clans.md's
  #426 growth line. The ledger is the owner's to strike.

## Seeds no step ran

`sansfiat sweep --after 100 1 4 5 7 --size medium` on main and on the branch: every seed reaches its pick
and plays 100 turns with no panic. Clans that joined civs: 218 on the branch against 262 on main, and
civs that kept their camps 9 of 12 against 7 of 12. Fewer joins match the lower aligned share on the
retro maps.
