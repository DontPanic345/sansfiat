# #427 Settling, read: the sweep

`sansfiat sweep --judge` on the branch (`branch.json.gz`, its text in `settling-lines.txt` and `towns-lines.txt`)
beside main's (`relay/specs/341/main-floor.json.gz` on `origin/relay/341`, main's engine at 8d1b4ed, with its noise
floors). `compare-main.txt` is `judge --compare main.json branch.json`.

## The control

Settling is a reading: no rule reads `Clan.settled`, so the branch plays main's game. Every metric main reports is
identical on the branch (no row moves, not by a hair; digest mismatches 0). What the branch adds is only in B: the
`settling` group, `capital_towns_*`, `capitals_200` and the `outside` group taken from #341's judge. Main's towns and
capital towns are therefore the branch's: 10 towns and 5 capital towns of the 18 capitals alive at +200. Main has no
settled state, so its settled counts are none.

## The question: settled clans and people, towns, and how often a clan changes

Settled clans of all clans (settled people of all people) at +100 and +200; settles and go-backs are over the whole
watch (each run goes at least 200 turns past the pick, on to the game's end), counted under the clan's civ at the change;
changes are both ways per 100 clan-turns over the watch.

| map | civs settled +100 | civs settled +200 | unaligned settled +100 | unaligned settled +200 | towns +200 | capital towns +200 | settles / back | changes per 100 clan-turns | clans changed (twice+) |
|---|---|---|---|---|---|---|---|---|---|
| small 2 | 28/140 (292/926 people) | 28/163 (364/1093 people) | 0/23 (0/112 people) | 0/2 (0/9 people) | 0 | 0 of 3 | 31 / 16 | 0.144 | 46 (1) |
| small 3 | 5/47 (34/239 people) | 5/49 (35/246 people) | 6/51 (37/277 people) | 6/52 (38/278 people) | 0 | 0 of 2 | 11 / 5 | 0.0812 | 15 (1) |
| small 6 | 14/49 (87/260 people) | 14/50 (86/266 people) | 22/86 (139/456 people) | 20/84 (137/456 people) | 0 | 0 of 2 | 33 / 6 | 0.148 | 36 (3) |
| small 9 | 40/100 (423/799 people) | 40/99 (471/849 people) | 7/39 (54/218 people) | 7/38 (54/215 people) | 1 | 1 of 3 | 46 / 4 | 0.183 | 50 (0) |
| medium 8 | 18/74 (138/448 people) | 18/77 (141/470 people) | 40/230 (323/1333 people) | 42/234 (340/1367 people) | 2 | 1 of 3 | 57 / 7 | 0.0833 | 60 (4) |
| medium 9 | 33/96 (311/720 people) | 40/98 (414/769 people) | 10/106 (50/561 people) | 9/109 (49/581 people) | 4 | 2 of 3 | 46 / 5 | 0.102 | 51 (0) |
| large 8 | 36/94 (291/727 people) | 36/93 (302/767 people) | 100/630 (1011/3921 people) | 105/641 (1079/4050 people) | 3 | 1 of 2 | 144 / 18 | 0.0742 | 155 (7) |
| **seven** | 174/600 (1576/4120 people) | 181/629 (1814/4460 people) | 185/1165 (1614/6878 people) | 189/1160 (1697/6956 people) | 10 | 5 of 18 | 368 / 61 | 0.0951 | 413 (16) |

- **Motion.** About three in ten civ clans (181 of 629) live settled at +200, holding 41% of the civs' people; one in six
  unaligned clans (189 of 1160, 24% of their people). Settled clans are larger than gatherers: 9.5 people against 5.6.
- **The shift is early.** Most clans settle in the first tens of turns after the pick, when fields open; between +100 and
  +200 the settled count barely moves (359 → 370 over all clans). Medium 9 is the one map that keeps settling (civs 33 → 40).
- **Rare changes.** 0.095 changes per 100 clan-turns: a clan changes about once in a thousand turns. Of the 413 clans that
  ever changed, 16 changed more than once. 61 went back to gathering, 14 of them founding clans (of 21) whose ground
  has no field: as the design says, the founding clan goes back as any clan does.
- **Small 2's civs**: 16 go-backs, the most of any map, and its unaligned clans nearly all join a civ by +200 (23 → 2).

## Tuning the turns (`SETTLING`)

A probe dumped every clan's field share each turn to +200 on the seven maps and replayed the state machine offline for
each pair of turns (settle after N turns at ≥ 0.2, go back after M turns under 0.1; changes over 351k clan-turns to +200):

| N / M | settled +100 | settled +200 | changes | clans changing twice or more | per 100 clan-turns |
|---|---|---|---|---|---|
| 1 / 1 | 378 | 385 | 1253 | 244 | 0.357 |
| 5 / 5 | 370 | 378 | 513 | 36 | 0.146 |
| **10 / 10** | 359 | 370 | 408 | 13 | 0.116 |
| 10 / 20 | 362 | 373 | 395 | 11 | 0.112 |
| 15 / 15 | 352 | 365 | 387 | 11 | 0.110 |
| 20 / 20 | 344 | 363 | 368 | 9 | 0.105 |
| 30 / 30 | 325 | 362 | 339 | 3 | 0.097 |

Past 10 the changes fall slowly while the clans that settle get later; at 10/10 a clan that changes twice is under 1%
of the clans that change. 18% of clan-turns are at a farmer's share, 72% under a tenth: the band between holds 10%.

## A played game

Small 2, led as civ 1 with no orders for 205 turns (`end 205`), judged with `sansfiat judge`:
`settling, led civ (game / control): settled at +100 21/38 clans (240/347 people) / 17/76 clans (155/471 people), at +200
20/37 clans (251/362 people) / 15/98 clans (167/575 people), settles 14 / 17, back to gathering 1 / 14; towns at +200 1 / 0,
capital towns 1 / 0`. The play step plays it for real.
