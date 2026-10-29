# #28 sweep: the pre-game with real trail costs

Measured at spec-review, before any build, on a scratch copy of `434b465` with one change:
`land::step_cost` brings a trailed tile to `min(bare, trail_cost)` (bare when it has none)
instead of `min(bare, TRAIL.cost = 1.0)`. In the pre-game no mountain has a trail and no
trail is built, so this is all build-1's change does to it.

Result: the pre-game still settles on every size, and the retro maps all found three civs.
**No fallback is needed**: worn trails go to the hidden cost. build-1 re-runs both commands
after its change and appends its lines below; they should match the "after" lines here.

| | before (`TRAIL.cost` 1.0) | after (hidden cost) |
|---|---|---|
| small: ended on their own, levelled off, median end | 10/10, 10/10, 270 | 10/10, 10/10, 261 |
| medium | 10/10, 10/10, 345 | 10/10, 10/10, 355 |
| large | 10/10, 10/10, 378 | 10/10, 10/10, 373 |
| retro: civs founded / kept their first camp | 21 / 21 of 21 | 21 / 21 of 21 |
| retro: clans' people 200 turns after the pick | 1025.5 | 847.2 |

The clans' people fall by 17% 200 turns after the pick. The maps diverge from the first
changed step, and the per-civ figures swing both ways between the two runs, so this is partly
noise. But forest rolled at 2 and marsh (no hidden cost) no longer get cheaper with a trail,
so it is partly real. It is noted for the review; nothing in this piece tunes it.

## Commands

```
cargo run --release -p sansfiat -- sweep --size all
cargo run --release -p sansfiat -- sweep --after 200 --maps retro
```

## before: sweep --size all (summary lines)

```
small: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
small: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 270
small: people over the last 100 turns before the end changed by +3% +4% +3% +3% +3% +3% +3% +3% +3% +3%: 10 of 10 levelled off (under 10%)
medium: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
medium: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 345
medium: people over the last 100 turns before the end changed by +4% +4% +4% +4% +4% +4% +4% +4% +4% +4%: 10 of 10 levelled off (under 10%)
large: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
large: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 378
large: people over the last 100 turns before the end changed by +4% +5% +5% +4% +4% +5% +5% +5% +5% +5%: 10 of 10 levelled off (under 10%)
```

## after: sweep --size all (summary lines)

```
small: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
small: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 261
small: people over the last 100 turns before the end changed by +3% +3% +2% +3% +2% +3% +3% +3% +3% +4%: 10 of 10 levelled off (under 10%)
medium: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
medium: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 355
medium: people over the last 100 turns before the end changed by +4% +4% +4% +4% +4% +4% +4% +4% +4% +4%: 10 of 10 levelled off (under 10%)
large: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
large: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 373
large: people over the last 100 turns before the end changed by +4% +4% +5% +5% +4% +4% +5% +5% +5% +5%: 10 of 10 levelled off (under 10%)
```

## before: sweep --after 200 --maps retro

```
small  seed   2 civ 510: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   77.3  kept yes
small  seed   2 civ 511: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   82.1  kept yes
small  seed   2 civ 512: camp alive warriors  3.0 pack  8.83 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   17.3  kept yes
small  seed   3 civ 212: camp alive warriors  3.0 pack  3.30 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged  26 tiles pillaged   0 tiles clans' people   14.5  kept yes
small  seed   3 civ 213: camp alive warriors  3.0 pack 11.88 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.6  kept yes
small  seed   3 civ 214: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   26.9  kept yes
small  seed   6 civ 363: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.3  kept yes
small  seed   6 civ 364: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   29.8  kept yes
small  seed   6 civ 365: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   39.8  kept yes
small  seed   9 civ 275: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   66.1  kept yes
small  seed   9 civ 276: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   40.2  kept yes
small  seed   9 civ 277: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.3  kept yes
medium seed   8 civ 677: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   78.9  kept yes
medium seed   8 civ 678: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.6  kept yes
medium seed   8 civ 679: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.7  kept yes
medium seed   9 civ 919: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people  105.7  kept yes
medium seed   9 civ 920: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   37.6  kept yes
medium seed   9 civ 921: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   27.3  kept yes
large  seed   8 civ 2158: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.9  kept yes
large  seed   8 civ 2159: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.1  kept yes
large  seed   8 civ 2160: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   88.6  kept yes
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (army): 1025.5
```

## after: sweep --after 200 --maps retro

```
small  seed   2 civ 713: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   22.2  kept yes
small  seed   2 civ 714: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   75.3  kept yes
small  seed   2 civ 715: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.7  kept yes
small  seed   3 civ 243: camp alive warriors  3.0 pack  8.51 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.3  kept yes
small  seed   3 civ 244: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.0  kept yes
small  seed   3 civ 245: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.4  kept yes
small  seed   6 civ 432: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   36.0  kept yes
small  seed   6 civ 433: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   42.7  kept yes
small  seed   6 civ 434: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.1  kept yes
small  seed   9 civ 290: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   52.7  kept yes
small  seed   9 civ 291: camp alive warriors  3.0 pack  9.38 hungry    0 starved   0 lost 0 line cost 8.00 carry 3.94 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.4  kept yes
small  seed   9 civ 292: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   28.1  kept yes
medium seed   8 civ 648: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.4  kept yes
medium seed   8 civ 649: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   42.2  kept yes
medium seed   8 civ 650: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.6  kept yes
medium seed   9 civ 849: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   44.9  kept yes
medium seed   9 civ 850: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.4  kept yes
medium seed   9 civ 851: camp alive warriors  3.0 pack  3.78 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   13.6  kept yes
large  seed   8 civ 2748: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.2  kept yes
large  seed   8 civ 2749: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.2  kept yes
large  seed   8 civ 2750: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.9  kept yes
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (army): 847.2
```

## build-1 re-run: sweep --size all (summary lines)

```
small: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
small: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 261
small: people over the last 100 turns before the end changed by +3% +3% +2% +3% +2% +3% +3% +3% +3% +4%: 10 of 10 levelled off (under 10%)
medium: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
medium: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 355
medium: people over the last 100 turns before the end changed by +4% +4% +4% +4% +4% +4% +4% +4% +4% +4%: 10 of 10 levelled off (under 10%)
large: the top-up took 0 existing markets, 0 tended tiles, 0 other grain tiles
large: 10 of 10 ended on their own, 0 needed the top-up, 0 ended on the limit with three centres already (cover short or people not levelled off), median end turn 373
large: people over the last 100 turns before the end changed by +4% +4% +5% +5% +4% +4% +5% +5% +5% +5%: 10 of 10 levelled off (under 10%)
```

## build-1 re-run: sweep --after 200 --maps retro

```
small  seed   2 civ 713: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   22.2  kept yes
small  seed   2 civ 714: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   75.3  kept yes
small  seed   2 civ 715: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.7  kept yes
small  seed   3 civ 243: camp alive warriors  3.0 pack  8.51 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   19.3  kept yes
small  seed   3 civ 244: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   23.0  kept yes
small  seed   3 civ 245: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   24.4  kept yes
small  seed   6 civ 432: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   36.0  kept yes
small  seed   6 civ 433: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   42.7  kept yes
small  seed   6 civ 434: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   55.1  kept yes
small  seed   9 civ 290: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   52.7  kept yes
small  seed   9 civ 291: camp alive warriors  3.0 pack  9.38 hungry    0 starved   0 lost 0 line cost 8.00 carry 3.94 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   33.4  kept yes
small  seed   9 civ 292: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   28.1  kept yes
medium seed   8 civ 648: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.4  kept yes
medium seed   8 civ 649: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   42.2  kept yes
medium seed   8 civ 650: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.6  kept yes
medium seed   9 civ 849: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   44.9  kept yes
medium seed   9 civ 850: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   41.4  kept yes
medium seed   9 civ 851: camp alive warriors  3.0 pack  3.78 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   13.6  kept yes
large  seed   8 civ 2748: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   54.2  kept yes
large  seed   8 civ 2749: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 4.00 carry 7.88 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.2  kept yes
large  seed   8 civ 2750: camp alive warriors  3.0 pack 12.00 hungry    0 starved   0 lost 0 line cost 6.00 carry 5.25 cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   56.9  kept yes
civs that kept their first camp: 21 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (army): 847.2
```

Both match the "after" blocks above line for line.

## Owner's change: a centre is a clan, and the capital gets its own market

A centre is now a clan: the biggest first, settled at home `centre_age` turns, going to a
festival with `centre_clans` traders last turn, with a hearth on its own territory, its home
`centre_spacing` from the other centres'. The site is its home. Founding no longer moves the
festival: a market is raised on the site unless one is there. This fixes the spaced-centres
failure (seed 2, radius 16), since the capital is where the spacing was checked.

`sweep --size all`: unchanged, 10/10 ended on their own on every size, median end 261, 355, 373.

`sweep --after 200 --maps retro`:

| | civs kept their first camp | clans' people |
|---|---|---|
| build-1 (festival moves to the clan's home) | 21 of 21 | 847.2 |
| clan centres, festival moved to the home (experiment) | 18 of 21 | 811.6 |
| clan centres, new market on the site, `CIV.pull` 3 (committed) | 11 of 21 | 866.1 |
| the same, `CIV.pull` 6 | 17 of 21 | 846.6 |
| the same, `CIV.pull` 10 | 19 of 21 | 841.9 |

Why camps starve: when the centre clan's festival is not on its home, the civ's other clans keep
going to that festival, busy and with its prices settled, and only the centre clan comes to the
new capital market (small seed 3: two of three capitals with one trader for 60 turns). The levy
is paid only at the capital's market, so the store stays empty and the camp's line brings nothing.
`CIV.pull` is left at 3 for the owner to decide.

The owner's answer: not a regression, since a camp is only a cost until the features that use it
are built; and at founding a clan switches to a capital's market whose path costs no more than
its festival's (`civs::switch_to_capitals`), the markets' traders counted again so the next turn's
choice sees the switch. After that clans choose as ever. `CIV.pull` stays at 3.

| | civs kept their first camp | clans' people |
|---|---|---|
| with the switch at founding | 15 of 21 | 882.1 |

`sweep --size all` is unchanged by it (founding comes after the end).
