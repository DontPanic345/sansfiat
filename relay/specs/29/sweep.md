# #29 sweep: the camps on the retro's seven maps

Run by build-2 (the sweep's first run, before build-3 tunes the numbers): `sansfiat sweep --after 200 --maps retro`
(army) and the same with `--no-army` (the control, `Config.army` off). All three civs on the AI, each raising
its camp at the pick (`UNIT.start` warriors), run 200 turns past the pick. One line per civ. A civ "kept" its
first camp when no camp was lost to hunger and it ends with at least `UNIT.start` warriors.

The numbers are the starting ones (`CIV`, `UNIT` in `data.rs` as build-1 left them: `levy` 0.3,
`endowment`, `pack` 5, `carry` 1, `forage` 0.5, `pillage` 1, `start` 3, `post` 3). Build-1 warned that at
these numbers the default AI game loses every camp within about ten turns (the levy brings in almost
nothing and the pack starves the camp). This run confirms it: no civ keeps its camp.

## Army

```
small  seed   2 civ 510: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   77.5  kept no
small  seed   2 civ 511: camp lost warriors  0.0 pack  0.00 hungry    6 starved   3 lost 1 line no line cut off   13 foraged   2 tiles pillaged   0 tiles clans' people   81.1  kept no
small  seed   2 civ 512: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   2 tiles pillaged   0 tiles clans' people   36.0  kept no
small  seed   3 civ 212: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   10 foraged   1 tiles pillaged   0 tiles clans' people   45.8  kept no
small  seed   3 civ 213: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   22.3  kept no
small  seed   3 civ 214: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   36.0  kept no
small  seed   6 civ 363: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   12 foraged   1 tiles pillaged   0 tiles clans' people   59.1  kept no
small  seed   6 civ 364: camp lost warriors  0.0 pack  0.00 hungry    7 starved   3 lost 1 line no line cut off   14 foraged   2 tiles pillaged   0 tiles clans' people   43.0  kept no
small  seed   6 civ 365: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   12 foraged   2 tiles pillaged   0 tiles clans' people   42.8  kept no
small  seed   9 civ 275: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   12 foraged   1 tiles pillaged   0 tiles clans' people   64.5  kept no
small  seed   9 civ 276: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   52.4  kept no
small  seed   9 civ 277: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   10 foraged   1 tiles pillaged   0 tiles clans' people   32.5  kept no
medium seed   8 civ 677: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   77.9  kept no
medium seed   8 civ 678: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   57.4  kept no
medium seed   8 civ 679: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   57.9  kept no
medium seed   9 civ 919: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   12 foraged   2 tiles pillaged   0 tiles clans' people   92.6  kept no
medium seed   9 civ 920: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   2 tiles pillaged   0 tiles clans' people   50.9  kept no
medium seed   9 civ 921: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   0 tiles pillaged   0 tiles clans' people   48.3  kept no
large  seed   8 civ 2158: camp lost warriors  0.0 pack  0.00 hungry   11 starved   3 lost 1 line no line cut off   22 foraged   5 tiles pillaged   0 tiles clans' people   85.6  kept no
large  seed   8 civ 2159: camp lost warriors  0.0 pack  0.00 hungry    5 starved   3 lost 1 line no line cut off   11 foraged   1 tiles pillaged   0 tiles clans' people   51.3  kept no
large  seed   8 civ 2160: camp lost warriors  0.0 pack  0.00 hungry    7 starved   3 lost 1 line no line cut off   14 foraged   3 tiles pillaged   0 tiles clans' people   89.8  kept no
civs that kept their first camp: 0 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (army): 1204.7
```

## Control (no army)

```
small  seed   2 civ 510: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   78.0  kept no
small  seed   2 civ 511: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   81.2  kept no
small  seed   2 civ 512: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   30.9  kept no
small  seed   3 civ 212: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.3  kept no
small  seed   3 civ 213: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   25.5  kept no
small  seed   3 civ 214: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   32.5  kept no
small  seed   6 civ 363: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   59.1  kept no
small  seed   6 civ 364: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   35.4  kept no
small  seed   6 civ 365: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   51.3  kept no
small  seed   9 civ 275: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   66.2  kept no
small  seed   9 civ 276: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   49.0  kept no
small  seed   9 civ 277: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   32.8  kept no
medium seed   8 civ 677: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   80.1  kept no
medium seed   8 civ 678: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   58.8  kept no
medium seed   8 civ 679: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   58.0  kept no
medium seed   9 civ 919: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   90.7  kept no
medium seed   9 civ 920: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   52.4  kept no
medium seed   9 civ 921: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   45.3  kept no
large  seed   8 civ 2158: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   75.0  kept no
large  seed   8 civ 2159: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   48.4  kept no
large  seed   8 civ 2160: camp none warriors  0.0 pack  0.00 hungry    0 starved   0 lost 0 line no line cut off    0 foraged   0 tiles pillaged   0 tiles clans' people   84.1  kept no
civs that kept their first camp: 0 of 21 (no camp lost, at least 3 warriors at the end)
clans' people at the end, all civs (control, no army): 1179.9
```

## Reading it

- Every army camp is starved out (`lost` 1 on each civ, its warriors gone by the end of the run) and the
  AI cannot raise another: the store is empty, so no pack can be filled. The control keeps no camp at all
  and its clans keep their people.
- Each army camp is cut off on most of the turns it is out (the "cut off" column), as the spec's
  arithmetic predicts at carry 1.0: a camp of three is fed one plains step out and cut off two out.
- Clans' people at the end are close in the two runs (1204.7 against 1179.9 over 21 civs), but that
  margin is measured after every camp has already failed, so it says nothing yet about what a kept army
  costs the clans. Build-3 has to make the camps survive before this comparison means anything.

This is the baseline for build-3 to tune against. Nothing here is a verdict on the design.
