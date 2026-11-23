# #427 Settling, read: the judge

`judge --compare` of main's sweep (A, `relay/specs/341/main-floor.json.gz`) and the branch's (B, `branch.json.gz`):
`compare-main.txt`. The engine is unchanged since (the review's commits change only a feed line's wording and the
legend), so no sweep was run again.

Verdict: **same**. Settling is a reading: no rule reads it, so the branch plays main's game to the digit.

| score | main | branch | floor |
|---|---|---|---|
| Standing: top civ at +200 | 0.2912 | 0.2912 | 0.0076 |
| Standing: aligned at +200 | 0.5216 | 0.5216 | 0.0079 |
| doubt: lead changes | 7 | 7 | 3 |
| doubt: swings | 14 | 14 | 4 |
| decisions: AI orders per 100 | 85.74 | 85.74 | 5.23 |
| motion: clans changed | 410 | 410 | 14 |
| motion: people changed | 2282.3 | 2282.3 | 125.8 |
| health: warriors starved | 329 | 329 | 16 |

The piece's own question, which main can't count (it has no settled state): at +200, 181 of 629 civ clans (1814 of
4460 people) and 189 of 1160 unaligned clans (1697 of 6956) are settled; 10 towns and 5 capital towns of 18
capitals; 368 settles and 61 go-backs, 0.095 changes per 100 clan-turns, 16 of 413 changed clans changing twice
(`sweep.md`). Every map has settled clans in the civs at both +100 and +200.

A seed no step ran: small 5, led civ 1 and left idle for 210 turns. `judge` gives settled at +100 14/23 clans
(88/136 people) in the game against 13/26 in the control, at +200 13/22 against 13/26, settles 14 / 15, back 1 / 1.
Its screenshots are `shots/settled-clan.png` and `shots/civ-living.png` (the save is loaded into the page at turn 464).

Play note (small 2, 110 turns): led 32.0% against 37.5% with every civ on the AI and 28.0% for the idle led civ.
