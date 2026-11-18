# #356 Exchange: the judge's verdict — worse

From `sweep-main.json` (8d1b4ed) and `sweep-final.json` (the branch, run with `--floor`); full table:
`sansfiat judge --compare relay/specs/356/sweep-main.json relay/specs/356/sweep-final.json`.

| score | main / control | branch | floor | |
|---|---|---|---|---|
| army-turns fed at a market not the civ's (7 maps, to +200) | 0 | 35 (5 of 7 maps) | | moved; 8.4 food bought of 120.6 eaten |
| largest good's share of trades at +200, by count | .19–.21 (a food) | .19–.21 (a food) | | same as control |
| largest share by worth | grain .35–.37 | grain .34–.43 | | same |
| starving clan-turns | 6,194 | 7,773 | 335 | past |
| starving share | .0328 | .0359 | .0014 | past |
| goods people over control | 1.0127 | 1.0236 | .0131 | within |
| armies people over control | 0.9991 | 0.9934 | .0274 | within |
| claim_taken / largest_army / fallen | 220 / 14.4 / 52 | 71 / 4.0 / 26 | 11 / 7 / 5 | past, unexplained |
| led beats idle (ahead_of_idle) | 16 | 19 | 3 | at the floor |
| obsidian steps mean / furthest | 5.35 / 18 | 5.40 / 18 | .08 | same |
| salt steps mean / furthest | 3.52 / 11 | 3.39 / 11 | .09 | down |

Play note: Gold on small 2 for 100 turns, Standing 5.2% against 9.7% on the AI and 2.9% idle. The lever never offered itself: the store held no salt.
