# #426 sweep: what a clan carries is what its days carry

`sansfiat sweep --judge` on main (59cd80f, with `--floor`) and on the branch, seven retro maps, every civ
on the AI. Files: `main-sweep.json`, `branch-sweep.json`, `compare.txt` (`sansfiat judge --compare`,
every metric against main's noise floor); `table.py` makes the tables
(`python3 table.py main main-sweep.json branch branch-sweep.json`).

The judge gained one count for this piece, `capital_towns_200` (towns at +200 on a civ's capital
market). Main's numbers come from main with that count patched in; it changes nothing in play.

## The Goal's scores

| score | main | branch | floor |
|---|---|---|---|
| towns at +200 | 10 | 11 | 1 |
| capital towns at +200 | 5 | 3 | 0 |
| clans living in towns at +200 | 59 | 58 | 8 |
| towns formed after the pick | 23 | 34 | 0 |
| trade a person a turn, town clans | 0.043 | **0.161** | 0.003 |
| trade a person a turn, farmers outside towns | 0.038 | 0.049 | 0.006 |
| trade a person a turn, all clans | 0.079 | 0.079 | |
| markets alive at +200 | 205 | 213 | 7 |
| clans at a market at +200 | 0.924 | 0.951 | 0.005 |
| health: panics, digest mismatches | 0, 0 | 0, 0 | 0 |
| health: pre-games ended by centres | 7 | 7 | 0 |
| starving, share of clan-turns | 2.2% | 3.3% | |
| starving, gatherers | 3.7% | 5.0% | 0.1% |
| starving, town clans | 0.8% | 1.7% | |
| starving, farmers outside towns | 0.9% | 0.6% | |

**The answer: a market at the door pays, but it does not yet draw farmers.** A town clan now trades
3.3 times what a farmer outside a town does (main: about the same), and festivals hold better: more
markets at +200 and more clans at them. But towns at +200 are within the floor of main (11 against 10),
the clans in them the same, and capital towns fewer (3 against 5). More towns form (34 against 23)
and fewer last. Health is at main's floor.

What it costs: a clan with no surplus brings no food, so it no longer swaps food for variety, and the
gatherers starve more (5.0% of their turns against 3.7%).

One sweep is one sample: the maps' outcomes move a long way with any change to the pre-game (small 2's
aligned share at +200 is 0.99 on main and 1.00 here, 0.44 and 0.47 under the variants below). Read the
counts above against their floors, and the per-map rows as anecdotes.

## By map

| map | run | towns 200 | capital towns 200 | town clans 200 | town traded/p | farmers outside traded/p | all traded/p | town starving | markets 200 | trading 200 |
|---|---|---|---|---|---|---|---|---|---|---|
| small 2 | main | 0 | 0 | 0 | 0.050 | 0.038 | 0.059 | 4 | 18 | 0.891 |
| small 2 | branch | 0 | 0 | 0 | 0.000 | 0.045 | 0.069 | 0 | 22 | 0.994 |
| small 3 | main | 0 | 0 | 0 | 0.000 | 0.053 | 0.060 | 0 | 13 | 0.941 |
| small 3 | branch | 1 | 0 | 3 | 0.030 | 0.043 | 0.064 | 3 | 19 | 0.969 |
| small 6 | main | 0 | 0 | 0 | 0.000 | 0.061 | 0.069 | 0 | 16 | 0.933 |
| small 6 | branch | 0 | 0 | 0 | 0.127 | 0.053 | 0.055 | 1 | 19 | 0.926 |
| small 9 | main | 1 | 1 | 3 | 0.038 | 0.035 | 0.053 | 9 | 20 | 0.912 |
| small 9 | branch | 1 | 0 | 4 | 0.055 | 0.047 | 0.061 | 0 | 20 | 0.955 |
| medium 8 | main | 2 | 1 | 10 | 0.037 | 0.038 | 0.074 | 2 | 33 | 0.965 |
| medium 8 | branch | 0 | 0 | 0 | 0.000 | 0.058 | 0.090 | 0 | 33 | 0.987 |
| medium 9 | main | 4 | 2 | 21 | 0.034 | 0.044 | 0.104 | 0 | 30 | 0.870 |
| medium 9 | branch | 2 | 1 | 18 | 0.186 | 0.052 | 0.053 | 82 | 32 | 0.884 |
| large 8 | main | 3 | 1 | 25 | 0.050 | 0.033 | 0.082 | 63 | 75 | 0.956 |
| large 8 | branch | 7 | 2 | 33 | 0.163 | 0.047 | 0.087 | 71 | 68 | 0.944 |
| total | main | 10 | 5 | 59 | 0.043 | 0.038 | 0.079 | 78 | 205 | 0.924 |
| total | branch | 11 | 3 | 58 | 0.161 | 0.049 | 0.079 | 157 | 213 | 0.951 |

## The variants weighed

Each with prices bounded as on the branch. The last rows of each: the branch ("branch", the surplus
only), a clan bringing loads from its whole basket ("whole basket": the food it eats too), and a clan
bringing its first load whatever it has and only its surplus past it ("first load + surplus").

| map | run | towns 200 | capital towns 200 | town clans 200 | town traded/p | farmers outside traded/p | all traded/p | town starving | markets 200 | trading 200 |
|---|---|---|---|---|---|---|---|---|---|---|
| total | main | 10 | 5 | 59 | 0.043 | 0.038 | 0.079 | 78 | 205 | 0.924 |
| total | branch | 11 | 3 | 58 | 0.161 | 0.049 | 0.079 | 157 | 213 | 0.951 |
| total | whole basket | 14 | 3 | 49 | 0.262 | 0.079 | 0.157 | 123 | 205 | 0.938 |
| total | first load + surplus | 15 | 5 | 63 | 0.180 | 0.072 | 0.157 | 46 | 209 | 0.937 |

| | main | branch | whole basket | first load + surplus |
|---|---|---|---|---|
| starving, all clans | 2.2% | 3.3% | 1.6% | 1.6% |
| starving, town clans | 0.8% | 1.7% | 1.7% | 0.5% |
| aligned share at +200 | 0.52 | 0.56 | 0.41 | 0.36 |
| towns formed | 23 | 34 | 34 | 22 |
| pre-games ended by centres | 7 | 7 | **6** | **6** |

- **First load + surplus** answers the question best: 15 towns at +200, 5 capital towns, 63 town clans,
  less starving than main everywhere, a town clan trading 2.5 times a farmer outside. But medium 9's
  pre-game runs to the 500-turn backstop (main ends it at 467, already near), so health falls below
  main's floor, and aligned share falls from 0.52 to 0.36. It breaks the Goal's health line, so the
  branch does not take it; it is an idea for the owner.
- **Whole basket**: a clan at the door brings the food it eats too, against the Decided line's "whole
  surplus". It also ends medium 9 by the backstop.
- **Prices**: a first run without the price bound read 17 towns at +200 for the branch. In that run,
  markets where clans brought salt and next to no food went to NaN prices within 80 turns and stopped
  clearing, and a NaN broke the save's replay. `market::clear` now bounds every price within 10^12 of
  the goods' mean (`PRICE_SPAN`), and every number here was made with the bound.
- **The home step**: with the cap gone and homes valued by their walk alone, towns at +200 fell to 7. A
  clan outside a town now pays one walker where it paid one for each 2 food of its 0.2 a person, so
  the walk alone pulled less to the door. The home step counts the load's trade (`homes::step`).

## Bench

`sansfiat bench --size large 8 --after 50`: 35.9 ms a turn on the branch, 33.8 on main (different
worlds past the pick).
