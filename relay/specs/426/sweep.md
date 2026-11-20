# #426 sweep: what a clan carries is what its days carry

`sansfiat sweep --judge` on main (0fd7835, with `--floor`) and on the branch (with `--floor`), seven
retro maps, every civ on the AI. Files: `main-sweep.json`, `branch-sweep.json`, `compare.txt`
(`sansfiat judge --compare`, every metric against the noise floor). `goal.py` makes the tables
(`python3 goal.py main main-sweep.json branch branch-sweep.json`).

The judge's `settling` group gained this piece's counts: people born a person a turn to settled and to
gathering clans (`Clan.born` over people before the turn's births and deaths), and the people crowding
killed (`Clan.crowded`), over the 200 turns past the pick. Main's numbers come from main with the same
counts patched in (judge-only: its digests are main's).

The rules swept: a clan brings its first load (2 food) whatever it has, and further loads from its
surplus while they pay their walker's days; a fed clan grows at most 15% a turn gathering and 25%
settled; a child costs its clan 4 days (gathering) or 1.5 days (settled) out of the next harvest; and
each turn a clan loses `0.02 × ((p − 8) / 32)²` of its people, `p` the people on its home tile.

## The Goal's scores

| score | main | branch | floor |
|---|---|---|---|
| towns at +200 | 10 | 19 | 1.0000 |
| capital towns at +200 | 5 | 3 | 0.0000 |
| clans in towns at +200 | 59 | 84 | 8.0000 |
| trade a person a turn, town clans | 0.0427 | 0.2101 | 0.0028 |
| trade a person a turn, outside towns | 0.0803 | 0.1560 | 0.0173 |
| trade a person a turn, farmers outside | 0.0380 | 0.0761 | 0.0057 |
| markets alive at +200 | 205 | 210 | 7.0000 |
| clans at a market at +200 | 0.9239 | 0.9510 | 0.0047 |
| born a person a turn, settled | 0.0034 | 0.0084 |  |
| born a person a turn, gathering | 0.0037 | 0.0041 |  |
| crowding deaths (people) | 0.0000 | 4137.8393 |  |
| people at +200 | 11416.0134 | 10693.6433 | 30.2978 |
| settled clans at +200 | 370 | 339 | 3.0000 |
| starving share, all | 0.0328 | 0.0271 | 0.0020 |
| starving share, gatherers | 0.0365 | 0.0274 | 0.0013 |
| starving share, settled farmers | 0.0040 | 0.0038 | 0.0008 |
| health: pre-games by centres | 7 | 7 |  |
| health: panics, digest mismatches | 0, 0 | 0, 0 |  |
| pre-game end turns | 287 214 274 226 283 467 362 | 255 177 227 248 274 285 314 |  |
| aligned share at +200 | 0.5216 | 0.4640 | 0.0079 |
| top civ at +200 | 0.2912 | 0.3060 | 0.0076 |

**The answer: yes, farmers come to markets on their own, but not to the capitals.** Towns at +200
nearly double (19 against 10, floor 1) and their clans rise from 59 to 84 (floor 8). A town clan trades
0.210 a person a turn against 0.076 for a farmer outside a town (main: 0.043 against 0.038). Festivals
hold: markets alive 210 against 205 (floor 7), clans at a market 0.951 against 0.924. A settled clan bears
0.0084 children a person a turn against 0.0041 for a gathering one (main: 0.0034 and 0.0037, the same).
Crowding killed 4138 people over the seven maps' 200 turns, most on large 8 (2629). Starving falls
below main (2.7% of clan-turns against 3.3%; gatherers 2.7% against 3.7%). Health holds: 7/7 pre-games
end by their centres, the latest at 314 (main's at 467), no panics or digest mismatches.

What does not come: capital towns at +200 are 3 against 5 (floor 0), and the aligned share at +200
falls from 0.52 to 0.46 (floor 0.008). Clans crowd a capital as they crowd any town, and the civ's own
clan stays there. Fewer AI orders (62.5 against 85.7 per 100 turns) and fewer claims taken (54 against
220) follow from a pre-game that ends sooner and a sparser world at the pick (people at +200 10694
against 11416). The civs' stores are a quarter lower (mean 31.4 against 41.7, floor 3.5; levy 8.7
against 10.5 a turn), so the AI raises fewer armies (5.9 raises per 100 turns against 10.3); on seed 9
the led civ's store cannot fill a scout's pack at the page script's turn, so `www/play/game.mjs` now
plays seed 2.

One sweep is one sample. Every pre-game ends on another turn here, so each map is another world; the
aligned share on one map moves from 0.99 to 0.60 (small 2) or 0.48 to 0.96 (small 3). Read the totals
against their floors, and the per-map rows as anecdotes. The floor is main's noise under one extra roll
at the pick, which understates a change that moves the pre-game: two variants with the same pre-game
(the first two crowding rows below share every pick) differ by 0.07 in aligned share.

## By map

| map | run | pick | towns 200 | capital towns 200 | town clans 200 | crowding deaths | aligned 200 |
|---|---|---|---|---|---|---|---|
| small 2 | main | 287 | 0 | 0 | 0 | 0 | 0.99 |
| small 2 | branch | 255 | 0 | 0 | 0 | 272 | 0.60 |
| small 3 | main | 214 | 0 | 0 | 0 | 0 | 0.48 |
| small 3 | branch | 177 | 2 | 0 | 7 | 20 | 0.96 |
| small 6 | main | 274 | 0 | 0 | 0 | 0 | 0.38 |
| small 6 | branch | 227 | 1 | 0 | 4 | 20 | 0.52 |
| small 9 | main | 226 | 1 | 1 | 3 | 0 | 0.80 |
| small 9 | branch | 248 | 5 | 1 | 21 | 428 | 0.60 |
| medium 8 | main | 283 | 2 | 1 | 10 | 0 | 0.26 |
| medium 8 | branch | 274 | 2 | 0 | 6 | 259 | 0.15 |
| medium 9 | main | 467 | 4 | 2 | 21 | 0 | 0.58 |
| medium 9 | branch | 285 | 3 | 1 | 14 | 509 | 0.20 |
| large 8 | main | 362 | 3 | 1 | 25 | 0 | 0.16 |
| large 8 | branch | 314 | 6 | 1 | 32 | 2629 | 0.22 |

## The numbers weighed

Each without `--floor`. Growth is "gathering/settled" percent; crowding is `crowd_rate`.

| score | crowd .05, 15/20 | crowd .02, 15/20 | no crowding, 15/20 | crowd .02, 15/25 (branch) | crowd .01, 15/20 | floor |
|---|---|---|---|---|---|---|
| towns at +200 | 14 | 17 | 17 | 19 | 17 |  |
| capital towns at +200 | 1 | 3 | 3 | 3 | 2 |  |
| clans in towns at +200 | 54 | 78 | 72 | 84 | 71 |  |
| trade a person a turn, town clans | 0.2063 | 0.2071 | 0.1791 | 0.2101 | 0.1889 |  |
| trade a person a turn, outside towns | 0.1670 | 0.1620 | 0.1478 | 0.1560 | 0.1582 |  |
| trade a person a turn, farmers outside | 0.0814 | 0.0728 | 0.0668 | 0.0761 | 0.0804 |  |
| markets alive at +200 | 208 | 213 | 222 | 210 | 214 |  |
| clans at a market at +200 | 0.9413 | 0.9470 | 0.9409 | 0.9510 | 0.9519 |  |
| born a person a turn, settled | 0.0120 | 0.0073 | 0.0041 | 0.0084 | 0.0055 |  |
| born a person a turn, gathering | 0.0072 | 0.0042 | 0.0033 | 0.0041 | 0.0039 |  |
| crowding deaths (people) | 11392.4360 | 4092.2405 | 0.0000 | 4137.8393 | 2957.5514 |  |
| people at +200 | 9998.4541 | 10742.7716 | 11059.0264 | 10693.6433 | 10854.4416 |  |
| settled clans at +200 | 290 | 326 | 359 | 339 | 367 |  |
| starving share, all | 0.0254 | 0.0247 | 0.0292 | 0.0271 | 0.0252 |  |
| starving share, gatherers | 0.0265 | 0.0251 | 0.0312 | 0.0274 | 0.0268 |  |
| starving share, settled farmers | 0.0033 | 0.0038 | 0.0038 | 0.0038 | 0.0029 |  |
| health: pre-games by centres | 7 | 7 | 7 | 7 | 7 |  |
| health: panics, digest mismatches | 0, 0 | 0, 0 | 0, 0 | 0, 0 | 0, 0 |  |
| pre-game end turns | 217 181 217 252 300 365 291 | 255 177 227 248 274 285 314 | 230 170 266 243 252 274 292 | 255 177 227 248 274 285 314 | 236 179 222 215 295 363 296 |  |
| aligned share at +200 | 0.3669 | 0.3938 | 0.4544 | 0.4640 | 0.5113 |  |
| top civ at +200 | 0.2067 | 0.2326 | 0.2943 | 0.3060 | 0.2776 |  |

- Crowding at 0.05 killed as many people as lived at +200 and left one capital town. At 0.01 the
  aligned share holds (0.51), but towns' clans fall back toward main (71) and capital towns to 2.
- Without crowding, towns at +200 are 17 too: crowding is not what draws farmers. The first load and the
  home step's trade gain draw them; crowding thins the towns and sends daughters out.
- A settled rate of 25% over 20% adds towns (19 against 17), town clans (84 against 78) and aligned
  share (0.46 against 0.39) on the same pre-game.
- Growth is the gathering rate in every pre-game (no clan settles before the pick), so a gathering rate
  under 15% slows it: at 10% (settled 15%, crowding 0.05) small 3's pre-game ended at 485 and medium 9's
  at 471, against 213 and 467 on main.
