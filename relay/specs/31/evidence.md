# Evidence: the pre-game's end on three sizes (#31 build-4)

Run on 2026-10-25 at commit 71b9f35 (the build-4 code), release build, `sansfiat sweep --size all`
(seeds 1..=10, 500-turn limit), 4 threads. The `ms a turn` figures are with four runs in parallel.

Columns: size, seed, end turn, how it ended (`centres`: three centres and 80% cover;
`limit`; `limit+topup`: the turn limit with fewer than three centres, topped up), centres at the
end, topped up (and, in brackets, how many came from tier a existing markets / b tended land /
c other grain land, told apart by founding turn and tame), cover, clans, people, markets,
hearths, closed country (valleys / sealed valleys / necks) and the time.

```
small  seed   1: ended 382 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 119  people  610.3  markets  8  hearths  30  closed 1/1/2  5.8 ms a turn
small  seed   2: ended 276 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 149  people  752.0  markets  9  hearths  28  closed 2/1/2  6.0 ms a turn
small  seed   3: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 1.00  clans 113  people  493.8  markets  7  hearths  30  closed 1/1/1  5.2 ms a turn
small  seed   4: ended 311 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 125  people  668.8  markets  9  hearths  42  closed 2/1/1  6.8 ms a turn
small  seed   5: ended 499 by limit+topup centres 3 topped up 2 (1/0/1)  cover 1.00  clans  87  people  443.6  markets  6  hearths  29  closed 2/1/1  6.3 ms a turn
small  seed   6: ended 308 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 118  people  638.2  markets  9  hearths  40  closed 1/1/2  6.8 ms a turn
small  seed   7: ended 380 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 113  people  540.5  markets  8  hearths  20  closed 2/1/2  4.5 ms a turn
small  seed   8: ended 433 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 137  people  682.1  markets  7  hearths  42  closed 2/1/2  5.2 ms a turn
small  seed   9: ended 299 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 116  people  668.4  markets  9  hearths  42  closed 2/1/1  5.2 ms a turn
small  seed  10: ended 383 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 143  people  743.7  markets  8  hearths  69  closed 1/1/2  6.8 ms a turn
medium seed   1: ended 247 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 245  people 1364.9  markets 16  hearths  24  closed 4/2/4  11.7 ms a turn
medium seed   2: ended 372 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 213  people 1107.7  markets 13  hearths  67  closed 4/2/4  10.5 ms a turn
medium seed   3: ended 336 by centres     centres 3 topped up 0 (0/0/0)  cover 0.85  clans 165  people  963.3  markets 10  hearths  43  closed 4/2/3  8.5 ms a turn
medium seed   4: ended 231 by centres     centres 3 topped up 0 (0/0/0)  cover 0.88  clans 204  people 1115.2  markets 15  hearths  25  closed 4/2/4  12.6 ms a turn
medium seed   5: ended 251 by centres     centres 3 topped up 0 (0/0/0)  cover 0.94  clans 233  people 1255.6  markets 15  hearths  16  closed 4/2/3  11.4 ms a turn
medium seed   6: ended 257 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 246  people 1422.8  markets 15  hearths  69  closed 4/2/3  13.8 ms a turn
medium seed   7: ended 442 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 212  people 1115.3  markets 15  hearths  91  closed 4/2/4  11.6 ms a turn
medium seed   8: ended 244 by centres     centres 3 topped up 0 (0/0/0)  cover 1.00  clans 265  people 1372.1  markets 17  hearths  25  closed 4/2/3  11.0 ms a turn
medium seed   9: ended 499 by limit+topup centres 3 topped up 2 (2/0/0)  cover 1.00  clans 173  people 1071.9  markets 11  hearths  77  closed 4/2/3  11.2 ms a turn
medium seed  10: ended 248 by centres     centres 3 topped up 0 (0/0/0)  cover 0.87  clans 211  people 1249.7  markets 16  hearths  30  closed 4/2/4  5.9 ms a turn
large  seed   1: ended 219 by centres     centres 3 topped up 0 (0/0/0)  cover 0.91  clans 351  people 2202.4  markets 20  hearths  15  closed 8/4/8  17.1 ms a turn
large  seed   2: ended 273 by centres     centres 3 topped up 0 (0/0/0)  cover 0.81  clans 356  people 1855.3  markets 24  hearths  47  closed 8/4/8  18.1 ms a turn
large  seed   3: ended 267 by centres     centres 3 topped up 0 (0/0/0)  cover 0.83  clans 408  people 2327.7  markets 26  hearths  30  closed 8/4/8  18.6 ms a turn
large  seed   4: ended 436 by centres     centres 3 topped up 0 (0/0/0)  cover 0.96  clans 435  people 2325.2  markets 25  hearths 172  closed 8/4/8  19.3 ms a turn
large  seed   5: ended 367 by centres     centres 3 topped up 0 (0/0/0)  cover 0.83  clans 335  people 1932.5  markets 22  hearths  59  closed 8/4/8  15.7 ms a turn
large  seed   6: ended 313 by centres     centres 3 topped up 0 (0/0/0)  cover 0.97  clans 428  people 2396.7  markets 28  hearths  52  closed 8/4/8  16.9 ms a turn
large  seed   7: ended 398 by centres     centres 3 topped up 0 (0/0/0)  cover 0.80  clans 394  people 1966.8  markets 22  hearths 117  closed 8/4/8  16.0 ms a turn
large  seed   8: ended 201 by centres     centres 3 topped up 0 (0/0/0)  cover 0.84  clans 378  people 2204.8  markets 24  hearths   7  closed 8/4/8  14.0 ms a turn
large  seed   9: ended 258 by centres     centres 3 topped up 0 (0/0/0)  cover 0.91  clans 419  people 2507.2  markets 28  hearths  38  closed 8/4/8  14.6 ms a turn
large  seed  10: ended 305 by centres     centres 3 topped up 0 (0/0/0)  cover 0.82  clans 361  people 2102.8  markets 23  hearths  64  closed 8/4/8  11.5 ms a turn
```

```
small: the top-up took 3 existing markets, 0 tended tiles, 1 other grain tiles
small: 8 of 10 ended on their own, 2 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 382
medium: the top-up took 2 existing markets, 0 tended tiles, 0 other grain tiles
medium: 9 of 10 ended on their own, 1 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 257
large: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
large: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 305
```

More seeds, same code, for a sharper rate (not in the table above):

```
seeds 11..=40: small: the top-up took 21 existing markets, 0 tended tiles, 0 other grain tiles
seeds 11..=40: small: 22 of 30 ended on their own, 8 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 367
seeds 11..=30: medium: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
seeds 11..=30: medium: 20 of 20 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 290
seeds 11..=20: large: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
seeds 11..=20: large: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short), median end turn 257
```

Over all the seeds run: small 30 of 40 end on their own (75%), medium 29 of 30 (97%), large 20 of 20.
Every one of the 90 runs ends with exactly three centres at least 10 steps apart (the test
`*_pregames_always_end_with_three_spaced_centres` holds this for a few seeds on each size).

## What this says

**How often it ends on its own.** The bigger the map the better: large 20/20 (median end turn
about 260-305), medium 29/30 (median 257-290), small 30/40 (median 367-382). Medium and large
maps have more land for the same 500 turns and more markets (11-28), so three centres far
apart form earlier; on a small map the three centres 10 apart have little room and the
seeds looked at (small 3 and 5) are ones where only one market gets the four traders it needs.

**How often it needs the top-up.** Small 10 of 40 seeds, medium 1 of 30, large 0 of 20. No
run ended on the limit with three centres already (so the cover was never what held it up
at the limit); every limit end was a shortage of centres.

**What the top-up picks.** Almost always tier (a), existing markets: 26 of the 27 sites added across all
90 runs were markets that already stood, just short of four traders or of a hearth in reach. One was a market raised on
other grain land (tier c, seed 5 small, which had only one centre and one other spaced
market). Tier (b) (tended land with no market) never fired: wherever people had tamed grain, a
market was already near. So the cheap tier carries the work, and the other two are the safety net.
Why the small seeds fall short: in the cases looked at (small 3 and 5) the map is fine, with seven
markets and cover 1.00, but only one market has four or more traders. The others are small
because clans prefer the nearest market, and the 10-step spacing leaves few places for three
big ones. That is the economy, not the map or the unreachable grain, so no tuning of `MAP` was
needed. Decision 17 (scaling the clan seeds with area) was not needed either: medium and large
maps do not fail to spread, so the clan rules and the generator numbers are unchanged.

**Odd things.**
- Small seed 36 first ended with *two* centres. The top-up needs its sites in one region of
  passable land, and with no centre at all it took the region of the most populated clan's
  home, which was the small northern landmass with room for two sites 10 apart only. The
  open-land guarantee holds on the map (it has three sites in its largest open region), but the
  clan was elsewhere. The top-up now tries that region first, then the others by population, and
  keeps the first that holds three sites; seed 36 now ends with three (all tier a). With one or
  more greedy centres already taken the region is theirs, as the brief says; if one sat in a
  poor region the end could still be short. No seed of the 90 did. A test
  (`with_no_centres_a_region_too_small_for_three_is_passed_over`) holds the change.
- No map has fewer than three sites in the sweeps; the open-sites guarantee of build-3 holds
  over the 90 seeds run.
- Cover on many medium and large ends is 0.80-0.97 when the centres are met, not 1.00: the
  cover threshold (0.8) is what ends them, not the people running out of land to spread on.
- Time: a large map is 11-20 ms a turn with four runs at once; all 90 runs took 81 s here.
