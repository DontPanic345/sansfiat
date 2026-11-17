# #355 Goods: scores

The branch as it leaves build-1 (`SOURCE.share` 0.05) beside `main` (da69498), on the seven retro maps:
`sweep-branch.txt/json`, `sweep-main.txt/json`, `compare.txt` (the two json reports have a noise floor
from one extra sweep each; `compare.txt` flags the 114 of 232 metrics past it).

## The goods section

| | whole's first sweep (`sweep-1.txt`) | share 0.1 | **final, share 0.05** | control |
|---|---|---|---|---|
| festivals where salt or obsidian trades | 32.0% | 28.9% | **25.1%** | 0 |
| salt: mean / furthest steps from a source at +200 | 4.6 / 13 | 4.1 / 14 | **3.5 / 11** | |
| obsidian: mean / furthest steps from a source at +200 | 5.2 / 20 | 5.0 / 20 | **5.4 / 18** | |
| armies armed | 30 | 40 | **33** | |
| people over control | 1.039 | 0.984 | **1.013** | 1 |
| starving clan-turns | 0.034 | 0.0278 | **0.0265** | 0.0216 |

Both goods leave their source on all 7 maps. Salt moves a lot (9346 festival-turns; 449 stores hold it, 372
of them away from any source); obsidian trades at only 524 festival-turns but reaches 18 civ stores.

**The cost.** Starving is 1.23 times the control (0.0265 against 0.0216), under a quarter above it. The
whole step's first sweep had 1.55 times; the store's fill and a starving clan leaving its source brought it to
1.28 at share 0.1; the one allowed try at `SOURCE.share` 0.05 gave 1.23 with the festival share at 25.1%
(over the 20% floor) and was kept. No other number was tuned. The goods section's starving and `main`'s are not the same count (the judge's clan-turns differ), so
the rise over `main` is read in `compare.txt`: the no-sources control on this branch already starves more than
`main` does (towns group, all clan-turns: 0.0183 on `main`, 0.0223 on the branch; the goods control reads 0.0216), so most of the rise
over `main` is the wider baskets, the store's cap and the levy, not the sources.

## Scores that moved past their noise floor (main to branch)

War is live in the AI games. The AI's civs hold more (store mean 34.0 to 41.7, from salt keeping food), raise
more (raise orders per 100, 6.9 to 10.3) and field bigger armies, and armed warriors fight 1.25 times as strong:

| score | main | branch | floor |
|---|---|---|---|
| largest army | 4 | 14.4 | 7.6 |
| tiles taken by a rival army (`fight.taken`) | 1 | 25 | 38 |
| claimed tiles taken (`motion.claim_taken`) | 21 | 220 | 147 |
| people fallen | 16.8 | 52.5 | 76.9 |
| camps lost | 11 | 8 | |
| Standing top at +200 | 0.234 | 0.291 | 0.033 |
| spread at +200 (the lead is closer) | 0.173 | 0.243 | 0.027 |
| lead changes | 10 | 14 | 3 |
| led civ's Standing | 0.125 | 0.167 | 0.040 |
| led civ won | 5 | 7 | 4 |
| towns at +100 | 12 | 10 | 1 |
| town clans at +200 | 77 | 38 | 10 |
| settlements founded | 6 | 15 | 3 |

Towns halve (town clans 77 to 38). The cause is not traced; it is a cost of the piece to the towns score. Everything else flagged is in
`compare.txt`; most of it is the same war and motion (clans changed 274 to 595 before the share change).

## Tests and scripts moved by the new worlds (not by a rule)

Real maps changed with the sources, so a few tests moved to a seed that keeps their subject:
`notes` (seed 2), `script` (seed 2: on seeds 1, 3, 4 the led civ fell within 15 turns of the pick),
`civ.mjs` (seed 6), `game.mjs` (seed 9), `army.mjs` (seed 6, Red). The led digest pin is at turn 300 (the pick
on seed 2 is 287, so turn 279 never ran the lead). Seed 1's pre-game now ends by `PREGAME.max_turns` (500).
