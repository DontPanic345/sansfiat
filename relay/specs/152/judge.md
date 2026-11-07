# #152 build-3: the judge reads the end

`sansfiat sweep --judge` on `relay/152` after this build (`engine/cli/src/sweep_judge.rs` plays each run to its end and at least to +200). Wall time: 33.8 s here, against 29.1 s for the same binary before the change this session (main's reports 32–34 s). Every run now plays to its limit; the +200 readings are the same as before the change (checked line by line against the pre-change text output, which differs only in the end lines and the whole-watch rates below).

```
small  seed   2 pick 237  standing +200 top 0.153 aligned 0.259  lead changes 0  spread +100 0.095 +200 0.102 swings 0  orders/100 3.83 levers 1.7  motion: clans changed 11 people 70.5 land 87  armies: camps kept 3/3 starved 0 lost 0 people vs control 0.99  land: markets 20/18 trading 0.89/0.89 homes 0.80/0.82 worked 0.29/0.26 route 1.00/0.98 changes 0.44 flips 0.07  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +200, winner Gold; no fight: limit +200; capitals fallen 0 (0)
small  seed   3 pick 245  standing +200 top 0.153 aligned 0.275  lead changes 0  spread +100 0.115 +200 0.115 swings 1  orders/100 1.83 levers 0.7  motion: clans changed 3 people 15.5 land 54  armies: camps kept 3/3 starved 0 lost 0 people vs control 1.00  land: markets 17/16 trading 0.95/0.93 homes 0.87/0.88 worked 0.34/0.31 route 0.94/0.94 changes 0.36 flips 0.01  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +200, winner Gold; no fight: limit +200; capitals fallen 0 (0)
small  seed   6 pick 318  standing +200 top 0.116 aligned 0.233  lead changes 1  spread +100 0.089 +200 0.084 swings 1  orders/100 10.67 levers 2.7  motion: clans changed 5 people 23.9 land 227  armies: camps kept 3/3 starved 0 lost 0 people vs control 0.95  land: markets 20/19 trading 0.94/0.94 homes 0.89/0.91 worked 0.31/0.28 route 0.98/0.98 changes 0.39 flips 0.07  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +200, winner Red; no fight: limit +200; capitals fallen 0 (0)
small  seed   9 pick 240  standing +200 top 0.149 aligned 0.325  lead changes 5  spread +100 0.094 +200 0.105 swings 2  orders/100 24.00 levers 1.3  motion: clans changed 11 people 60.2 land 557  armies: camps kept 3/3 starved 0 lost 0 people vs control 0.97  land: markets 18/15 trading 0.88/0.85 homes 0.88/0.86 worked 0.30/0.29 route 0.95/1.00 changes 0.60 flips 0.01  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +200, winner Blue; no fight: limit +200; capitals fallen 0 (0)
medium seed   8 pick 273  standing +200 top 0.046 aligned 0.106  lead changes 0  spread +100 0.026 +200 0.026 swings 0  orders/100 1.87 levers 1.3  motion: clans changed 0 people 0.0 land 66  armies: camps kept 3/3 starved 0 lost 0 people vs control 1.00  land: markets 36/36 trading 0.96/0.96 homes 0.86/0.85 worked 0.32/0.32 route 0.97/0.98 changes 0.21 flips 0.02  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +250, winner Blue; no fight: limit +250; capitals fallen 0 (0)
medium seed   9 pick 431  standing +200 top 0.081 aligned 0.166  lead changes 2  spread +100 0.049 +200 0.047 swings 3  orders/100 13.60 levers 1.7  motion: clans changed 10 people 61.3 land 292  armies: camps kept 3/3 starved 0 lost 0 people vs control 0.96  land: markets 34/35 trading 0.74/0.80 homes 0.82/0.82 worked 0.28/0.30 route 0.94/0.98 changes 1.18 flips 0.16  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +250, winner Red; no fight: limit +250; capitals fallen 0 (0)
large  seed   8 pick 369  standing +200 top 0.041 aligned 0.097  lead changes 2  spread +100 0.013 +200 0.014 swings 0  orders/100 37.44 levers 2.0  motion: clans changed 82 people 338.1 land 1165  armies: camps kept 3/3 starved 0 lost 0 people vs control 1.00  land: markets 73/72 trading 0.93/0.91 homes 0.81/0.82 worked 0.32/0.31 route 0.99/0.99 changes 0.78 flips 0.13  fight (control): fallen 0.00 (0.00) taken 0 (0) surrendered 0 (0)  health: centres 1 digests differ 0
        end: limit +300, winner Gold; no fight: limit +300; capitals fallen 0 (0)
total end: games ended before the limit 0 of 7 (no fight 0 of 7), turns to the end 228.6 (228.6), capitals fallen 0 (0)
```

## What moves, and why

Every standing, spread and top-share reading at +200 is unchanged: the diff against the pre-change text output (`sweep --judge`, this session) differs only in the end lines, the total end line, the seconds, and the measures that cover the whole watch. A watch now runs to the limit (250 and 300 on medium and large), so those cover more turns: per map, `orders/100` (medium 8: 2.33 → 1.87; medium 9: 11.17 → 13.60; large 8: 38.83 → 37.44), the motion counts, the land changes and flips, the swings, and `people vs control` (medium 9: 0.97 → 0.96, since the summary reads the world at the watch's end). In the total: AI orders per 100 turns 13.24 → 13.32; clans changed 102 → 122; people changed 498.5 → 569.5; land tiles changed 1961 → 2448; swings 5 → 7; store mean 16.90 → 17.11; civ-turns at pack 228.9 → 284.6; levy per turn 7.77 → 7.83; people vs control 0.992 → 0.990; festival changes 0.66 → 0.65 per 100.

A `judge --compare` of main against this branch flags these as moves from the longer watch, not from a rule. Whether they should stay over +200 is the owner's call (a `Watch` that stops scoring at +200 while the world runs on would hold them where they were).

## `judge FILE` on the stories' saves (the end beside the controls)

```
surrender, small seed 2, led civ Red, to the end: end (game / control / idle): limit +200 rank 2 / limit +200 rank 3 / limit +200 rank 3
skirmish, small seed 2, led civ 1, 200 turns: end (game / control / idle): limit +200 rank 2 / limit +200 rank 3 / limit +200 rank 3
```
