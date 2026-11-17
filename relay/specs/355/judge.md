# #355 Goods: the judge

Verdict: **not shown**. The sweeps are `sweep-main.json` (main, da69498) and `sweep-branch.json` (the branch at
the last engine change); `compare.txt` is `sansfiat judge --compare` of the two (re-run in review: identical).
The floors are the compare's own.

## The piece's question: motion of the goods, against the no-sources control

| | control | branch |
|---|---|---|
| festivals where salt or obsidian trades | 0 (no sources) | 9612 of 38241 = 25.1% (per map 10.1% to 47.1%) |
| salt, mean / furthest steps from a source at +200 | | 3.5 / 11; 449 stores hold it, 372 away from a source |
| obsidian, same | | 5.4 / 18; 534 stores, 485 away; reaches 18 civ stores |
| both leave their source | | 7 maps of 7 |

The control is zero because the good does not exist there: it shows the goods exist and move, not that they move
a score. Obsidian trades at 524 festival-turns (1.4% of festivals; 2 on large 8), so it moves mostly by the civ's
levy, not its festival.

## The four scores, main to branch (7 retro maps, `compare.txt`)

| score | main | branch | floor | |
|---|---|---|---|---|
| Standing: top civ at +200 | 0.234 | 0.291 | 0.018 | up |
| Standing: led civ | 0.126 | 0.167 | 0.015 | up |
| Doubt: spread at +200 | 0.173 | 0.243 | 0.006 | wider: a civ runs further ahead (worse) |
| Doubt: maps with a lead change | 6 | 3 | 0 | worse |
| Doubt: lead changes | 10 | 7 | 3 | within the floor |
| Decisions: AI orders per 100 | 71.5 | 85.7 | 5.2 | up (raise 6.9 to 10.3) |
| Motion: land changed | 481 | 1024 | 209 | up |
| Motion: clans changed | 274 | 410 | 66 | up |
| Motion: claimed tiles taken | 21 | 220 | 79 | up |
| Health: starving share (all clan-turns) | 0.0279 | 0.0328 | 0.0020 | up; 1.23 times the no-sources control (0.0265 against 0.0216) |
| Health: town clans at +200 | 77 | 59 | 10 | down a quarter (town farmers 58 to 44) |
| Health: digest mismatches, panics, pregame by centres | 0, 0, 7 | 0, 0, 7 | | in band |
| Led over its idle self | 0.040 | 0.069 | 0.006 | the led civ beats idle in 16 of 21, as on main |

## Why it is not better

- The question moved on every retro map, and the mechanism is the one claimed: salt is dug at 137 of 145 sources,
  walks off them, keeps food (civ store mean 34.0 to 41.7) and 33 armies are armed.
- But the scores moved with it are not shown to be the sources': the branch's no-sources control already starves
  more than main (the wider baskets, the store cap and the levy), the report does not export the control's
  Standing, doubt or towns, and the largest rise (war: tiles taken 1 to 25, claims taken 21 to 220) follows the richer
  stores and larger armies. Doubt got worse past its floor (one civ runs ahead; fewer maps change leader), and towns
  fell a quarter, a cost to an earlier milestone's score that no step traced.
- A led player beats idle in the sweep. The played game (`play.md`, small 2, one seed): 28.7% against the AI's 36.6% and
  27.6% idle, 8 orders in 100 turns; salt never reached Blue's store (no Blue clan held a pan).
