# Sweeps for #55: is joining a lever worth pulling?

## The answer

**Yes, up to a point a player can see.** With `CIV.store_cap` 60 (was 20), a civ that raises its
gift draws clearly more clans than its AI rivals: at gift 6 the led civ joins 12.0 clans on
average to its rivals' 6.8, at 15 it joins 14.7 to 8.1, and at the default 2 it joins 8.4 to 8.9
(no edge). Past about 15 the lever is spent: at 30 it joins 14.4, the same, for about twice the
food given (1627 against 857 on large seed 8), since the log buys less per food and only the clans
at the market can be reached. Gifts matter: 183 clans join on the defaults against 32 with
`--no-gifts`, and the clans' people end at 1395 against 1054. The prize lasts: 1364 of 1630 clans
are still unaligned at 200 turns. The cost is the food: the store sits near the keep, and what
goes in gifts isn't posted. The default keep holds the camp: every pack ends full, and 21 of 21
camps hold at every gift, so giving more never starves the camp. That is the one part of "at a
cost" that fails: the keep guards the camp so well that the only cost is food given and a lower
store, not hunger. **Where it fails:** a civ whose market draws no unaligned clans can't buy any
(small seed 3's led civ gives 0.5 food in 200 turns at every gift, and joins none); and at 20 the
store left too little beyond the keep for any gift over 5 to differ (build-1's lever 6, 10 and 20
were identical, 9.6 to 7.9), which is why the cap rose.

Speed holds: the default sweep takes 13.9 s against 13.8 s on `main` before the piece (same
machine, release), so ms a turn is within 1.5×.

## Numbers, before and after

| Number | Before | After | Why |
|---|---|---|---|
| `CIV.store_cap` | 20 | 60 | At 20 the store held at most 5 beyond the default keep of 15, so every gift over 5 was the same; at 60 the levy fills it enough to give 15 a turn. |
| `CIV.levy`, `CIV.endowment` | 0.2, 20 grain | unchanged | The levy already refills the store faster than the gifts drain it. |
| `LEANING.trade` 1.0, `gift` 0.3, `gift_scale` 0.5, `starving` 5.0, `fade` 0.03, `join` 1.0, `gift_default` 2.0, `floor` 0.001 | as left | unchanged | At these the defaults draw a median 6 clans a civ, gifts-off 0, and the lever pays and saturates near 15; no change of form was needed. |
| Default keep, AI gift | camp's full pack, 2 | unchanged | The keep holds every camp at any gift; the AI's 2 leaves the lever its room. |

The tests on the cap (`core/tests/civs.rs`) now fill the store by multiples of `CIV.store_cap`
instead of 40 and 60 food.

## Final runs (store_cap 60)

### `sansfiat sweep --after 200 --maps retro`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.4  kept yes  joined 4 gifted 400.0 leaning 6
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   87.9  kept yes  joined 3 gifted 28.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.7  kept yes  joined 0 gifted 1.9 leaning 0
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.1  kept yes  joined 0 gifted 2.1 leaning 0
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   42.4  kept yes  joined 3 gifted 46.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.1  kept yes  joined 5 gifted 50.0 leaning 0
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.9  kept yes  joined 8 gifted 98.0 leaning 1
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.3  kept yes  joined 4 gifted 28.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.5  kept yes  joined 15 gifted 226.0 leaning 3
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.6  kept yes  joined 3 gifted 50.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  106.6  kept yes  joined 6 gifted 50.0 leaning 0
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   84.5  kept yes  joined 23 gifted 264.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   93.3  kept yes  joined 12 gifted 166.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  124.9  kept yes  joined 9 gifted 58.0 leaning 0
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   73.9  kept yes  joined 17 gifted 175.4 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.9  kept yes  joined 7 gifted 98.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  124.3  kept yes  joined 20 gifted 202.0 leaning 0
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   96.5  kept yes  joined 20 gifted 222.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.1  kept yes  joined 24 gifted 274.0 leaning 1
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 183 (per civ, min/median/max 0/6/24)
unaligned clans at the end: 1364 of 1630
clans' people at the end, all civs (army): 1394.8
```

### `sansfiat sweep --after 200 --maps retro --no-gifts`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   79.8  kept yes  joined 2 gifted 0.0 leaning 0
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   86.0  kept yes  joined 2 gifted 0.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.1  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 212: camp alive warriors  3.0 pack  3.26 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged  26 tiles pillaged   0 tiles clans' people   14.5  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.7  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   52.1  kept yes  joined 3 gifted 0.0 leaning 0
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   29.8  kept yes  joined 1 gifted 0.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   39.1  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   21.5  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   38.2  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   89.2  kept yes  joined 3 gifted 0.0 leaning 1
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   36.6  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.4  kept yes  joined 0 gifted 0.0 leaning 2
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.3  kept yes  joined 8 gifted 0.0 leaning 2
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   31.2  kept yes  joined 2 gifted 0.0 leaning 3
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   40.2  kept yes  joined 0 gifted 0.0 leaning 1
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  114.9  kept yes  joined 5 gifted 0.0 leaning 0
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   88.6  kept yes  joined 5 gifted 0.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   73.0  kept yes  joined 1 gifted 0.0 leaning 2
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 32 (per civ, min/median/max 0/0/8)
unaligned clans at the end: 1443 of 1641
clans' people at the end, all civs (control, no gifts): 1054.1
```

### `sansfiat sweep --after 200 --maps retro --lever 2`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.4  kept yes  joined 4 gifted 400.0 leaning 6  lead
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   87.9  kept yes  joined 3 gifted 28.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.7  kept yes  joined 0 gifted 1.9 leaning 0
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 0.5 leaning 0  lead
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.1  kept yes  joined 4 gifted 72.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   53.6  kept yes  joined 4 gifted 40.0 leaning 0  lead
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.7  kept yes  joined 7 gifted 82.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   46.3  kept yes  joined 4 gifted 28.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   60.0  kept yes  joined 17 gifted 224.0 leaning 0  lead
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.6  kept yes  joined 3 gifted 50.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  106.6  kept yes  joined 6 gifted 50.0 leaning 0  lead
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   84.5  kept yes  joined 23 gifted 264.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   93.3  kept yes  joined 12 gifted 166.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  114.2  kept yes  joined 8 gifted 62.0 leaning 0  lead
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   61.2  kept yes  joined 20 gifted 191.4 leaning 1
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.5  kept yes  joined 5 gifted 60.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  124.3  kept yes  joined 20 gifted 202.0 leaning 0  lead
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   96.5  kept yes  joined 20 gifted 222.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.1  kept yes  joined 24 gifted 274.0 leaning 1
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 184 (per civ, min/median/max 0/5/24)
unaligned clans at the end: 1366 of 1629
clans' people at the end, all civs (army): 1382.7
led civ joined 8.4, its rivals 8.9 on average
```

### `sansfiat sweep --after 200 --maps retro --lever 6`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  128.2  kept yes  joined 8 gifted 114.0 leaning 0  lead
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   84.1  kept yes  joined 2 gifted 16.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   21.0  kept yes  joined 0 gifted 1.4 leaning 0
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 0.5 leaning 0  lead
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.1  kept yes  joined 4 gifted 72.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   70.6  kept yes  joined 11 gifted 152.6 leaning 0  lead
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.3  kept yes  joined 13 gifted 146.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.5  kept yes  joined 6 gifted 54.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   71.8  kept yes  joined 8 gifted 116.7 leaning 0  lead
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   52.6  kept yes  joined 3 gifted 50.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  149.7  kept yes  joined 20 gifted 318.0 leaning 0  lead
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.9  kept yes  joined 4 gifted 80.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   95.5  kept yes  joined 9 gifted 182.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  110.6  kept yes  joined 16 gifted 132.0 leaning 0  lead
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   76.8  kept yes  joined 22 gifted 197.4 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.9  kept yes  joined 10 gifted 126.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  130.0  kept yes  joined 21 gifted 215.8 leaning 0  lead
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  101.6  kept yes  joined 15 gifted 132.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   96.9  kept yes  joined 7 gifted 82.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 179 (per civ, min/median/max 0/8/22)
unaligned clans at the end: 1340 of 1630
clans' people at the end, all civs (army): 1524.4
led civ joined 12.0, its rivals 6.8 on average
```

### `sansfiat sweep --after 200 --maps retro --lever 15`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  122.7  kept yes  joined 12 gifted 176.5 leaning 0  lead
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   85.2  kept yes  joined 3 gifted 18.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   21.0  kept yes  joined 0 gifted 63.7 leaning 4
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 0.5 leaning 0  lead
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.1  kept yes  joined 4 gifted 72.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   47.8  kept yes  joined 6 gifted 130.2 leaning 0  lead
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   47.6  kept yes  joined 4 gifted 60.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   43.0  kept yes  joined 3 gifted 24.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   57.2  kept yes  joined 13 gifted 294.6 leaning 0  lead
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.5  kept yes  joined 3 gifted 52.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  154.8  kept yes  joined 23 gifted 552.7 leaning 0  lead
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   61.9  kept yes  joined 4 gifted 72.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   89.4  kept yes  joined 11 gifted 152.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  110.8  kept yes  joined 9 gifted 124.3 leaning 0  lead
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   70.5  kept yes  joined 33 gifted 249.4 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.3  kept yes  joined 5 gifted 74.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  146.3  kept yes  joined 40 gifted 856.6 leaning 0  lead
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  101.6  kept yes  joined 23 gifted 244.0 leaning 2
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   98.2  kept yes  joined 20 gifted 236.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 216 (per civ, min/median/max 0/5/40)
unaligned clans at the end: 1342 of 1627
clans' people at the end, all civs (army): 1469.1
led civ joined 14.7, its rivals 8.1 on average
```

### `sansfiat sweep --after 200 --maps retro --lever 30`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  125.2  kept yes  joined 10 gifted 131.5 leaning 0  lead
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   85.2  kept yes  joined 3 gifted 18.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   39.7  kept yes  joined 6 gifted 141.7 leaning 1
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 0.5 leaning 0  lead
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.1  kept yes  joined 4 gifted 72.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   60.6  kept yes  joined 10 gifted 364.4 leaning 0  lead
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.2  kept yes  joined 8 gifted 92.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   43.3  kept yes  joined 3 gifted 24.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   70.2  kept yes  joined 8 gifted 192.0 leaning 0  lead
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   50.8  kept yes  joined 3 gifted 52.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  159.4  kept yes  joined 22 gifted 963.8 leaning 0  lead
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   58.1  kept yes  joined 9 gifted 110.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   95.5  kept yes  joined 9 gifted 182.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  105.9  kept yes  joined 9 gifted 130.6 leaning 0  lead
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   72.9  kept yes  joined 24 gifted 238.8 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.5  kept yes  joined 7 gifted 82.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  149.0  kept yes  joined 42 gifted 1626.6 leaning 0  lead
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  110.7  kept yes  joined 28 gifted 298.0 leaning 3
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  112.2  kept yes  joined 15 gifted 186.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 220 (per civ, min/median/max 0/8/42)
unaligned clans at the end: 1329 of 1621
clans' people at the end, all civs (army): 1556.6
led civ joined 14.4, its rivals 8.5 on average
```

## Play

`node www/play-lean.mjs` (small seed 2, the first civ led, gift raised to 5): on turn 277 the
civ's panel shows the store 15.8 / 60, 'kept 15 · posted 4.49 · given 5 last turn' and the gift
and keep controls high in the panel, and eight clans leaning to Red, the strongest at 87% of join;
the clan's own panel shows its 'Leans to'. A player can see a clan coming as its share of join
climbs, and the 'given' line shows what a bigger gift costs. All eight join by turn 306. The
leaning list sits just under the panel's fold at 1000 px and needs a scroll. Screens in
`screens/`.

# Build-1's first run (the canvas's baseline, not a judgement)

Numbers: `LEANING` trade 1.0, gift 0.3, gift_scale 0.5, starving 5.0, fade 0.03, join 1.0, gift_default 2.0, floor 0.001; `CIV.*` untouched.

Each run: the command, its output, and one line on what it shows.

## `sansfiat sweep --after 200 --maps retro `

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   89.2  kept yes  joined 2 gifted 400.0 leaning 6
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   81.2  kept yes  joined 1 gifted 16.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 93.0 leaning 4
small  seed   3 civ 212: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.1  kept yes  joined 0 gifted 2.1 leaning 0
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   50.0  kept yes  joined 3 gifted 46.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   50.6  kept yes  joined 3 gifted 26.0 leaning 0
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.0  kept yes  joined 5 gifted 52.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.0  kept yes  joined 3 gifted 24.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   71.9  kept yes  joined 11 gifted 158.0 leaning 0
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   62.3  kept yes  joined 4 gifted 52.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  114.8  kept yes  joined 7 gifted 64.0 leaning 0
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   88.3  kept yes  joined 16 gifted 220.4 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.7  kept yes  joined 7 gifted 108.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  112.1  kept yes  joined 7 gifted 46.0 leaning 0
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   80.7  kept yes  joined 26 gifted 231.4 leaning 2
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   48.4  kept yes  joined 2 gifted 42.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  131.7  kept yes  joined 18 gifted 194.0 leaning 0
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  105.9  kept yes  joined 12 gifted 142.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   92.9  kept yes  joined 14 gifted 190.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 141 (per civ, min/median/max 0/4/26)
unaligned clans at the end: 1361 of 1640
clans' people at the end, all civs (army): 1451.1
```

On the AI defaults 141 clans join over 21 civs (median 4, max 26), 21 of 21 camps held, the clans' people reach 1451, and 1361 of 1640 clans are still unaligned, so the prize lasts.

## `sansfiat sweep --after 200 --maps retro --no-gifts`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   89.2  kept yes  joined 2 gifted 0.0 leaning 0
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   81.6  kept yes  joined 1 gifted 0.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 11.40 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   18.9  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 212: camp alive warriors  3.0 pack  3.26 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged  26 tiles pillaged   0 tiles clans' people   14.5  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   27.0  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   53.7  kept yes  joined 2 gifted 0.0 leaning 0
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   37.4  kept yes  joined 1 gifted 0.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   38.5  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.1  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   39.5  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   92.4  kept yes  joined 3 gifted 0.0 leaning 1
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   36.0  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.4  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  109.0  kept yes  joined 0 gifted 0.0 leaning 0
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.1  kept yes  joined 1 gifted 0.0 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   26.5  kept yes  joined 0 gifted 0.0 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   91.8  kept yes  joined 6 gifted 0.0 leaning 0
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   68.6  kept yes  joined 3 gifted 0.0 leaning 0
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   75.0  kept yes  joined 2 gifted 0.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 21 (per civ, min/median/max 0/0/6)
unaligned clans at the end: 1444 of 1648
clans' people at the end, all civs (control, no gifts): 1045.2
```

With gifts off 21 clans join (median 0, max 6) and the clans' people fall to 1045: the gifts do most of the drawing.

## `sansfiat sweep --after 200 --maps retro --lever 6`

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  126.3  kept yes  joined 8 gifted 140.0 leaning 0  lead
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   81.2  kept yes  joined 1 gifted 16.0 leaning 0
small  seed   2 civ 512: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   20.3  kept yes  joined 0 gifted 78.3 leaning 4
small  seed   3 civ 212: camp alive warriors  3.0 pack 10.93 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   16.7  kept yes  joined 0 gifted 0.5 leaning 0  lead
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes  joined 0 gifted 0.0 leaning 0
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.2  kept yes  joined 3 gifted 46.9 leaning 0
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.3  kept yes  joined 3 gifted 37.6 leaning 0  lead
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.8  kept yes  joined 4 gifted 44.0 leaning 0
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.5  kept yes  joined 3 gifted 24.0 leaning 0
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   73.0  kept yes  joined 9 gifted 135.7 leaning 0  lead
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   58.9  kept yes  joined 3 gifted 48.7 leaning 0
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes  joined 0 gifted 20.6 leaning 0
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  115.6  kept yes  joined 7 gifted 90.0 leaning 0  lead
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   86.6  kept yes  joined 17 gifted 228.4 leaning 1
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   83.7  kept yes  joined 7 gifted 108.0 leaning 0
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  118.7  kept yes  joined 9 gifted 90.0 leaning 0  lead
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   79.2  kept yes  joined 23 gifted 209.4 leaning 0
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   60.8  kept yes  joined 5 gifted 60.3 leaning 0
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  106.6  kept yes  joined 31 gifted 420.0 leaning 0  lead
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   95.0  kept yes  joined 26 gifted 296.0 leaning 1
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  107.7  kept yes  joined 18 gifted 208.0 leaning 0
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans joined: 177 (per civ, min/median/max 0/5/31)
unaligned clans at the end: 1353 of 1633
clans' people at the end, all civs (army): 1473.1
led civ joined 9.6, its rivals 7.9 on average
```

The led civ (gift 6) joins 9.6 clans on average against its rivals' 7.9; 177 join in all and all 21 camps hold.
