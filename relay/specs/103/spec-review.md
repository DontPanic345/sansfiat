# #103 spec-review: what was run

All on this runner, headless Chrome through `www/play.mjs`, main's page (the branch has only the
spec on top of main).

## The old play scripts

| Script | Result | Seconds |
|---|---|---|
| play-camp | pass | 4.2 |
| play-hold | **fails**: "raised only 5 warriors" | 5.7 |
| play-lean | pass | 4.6 |
| play-read | pass | 3.8 |
| play-scout | pass | 3.0 |
| play-claim | pass | 3.8 |

`play-hold` clicks a button labelled "Raise 1"; the civ bar's button is now "Raise warrior".
With the label changed in a copy, it raised 3 of 10 warriors in 60 tries: the page shows Raise
warrior enabled, and the engine refuses it. At the pick on small seed 7 the button is disabled
with an empty tooltip (the store is empty), and it turns on by turn 265.

## The map's share of the window, main, camp selected (small seed 2)

| Size | Map (px) | Share | Bar height | Side |
|---|---|---|---|---|
| phone 390×844 | 390×602 | 71% | 242 px | 176 px drawer over the map |
| laptop 1280×800 | 940×726 | 67% | 74 px (wraps) | 340 px |
| wide 1920×1080 | 1580×1039 | 79% | 41 px | 340 px |

With the planned layout at laptop (a 340 px feed beside the map, a 40 px bar, the panel below),
60% leaves the panel 106 px. At wide it leaves 253 px.

## Terrain drawing (seed 2, laptop size, at fit)

| Map | Tiles | main `terrainMs` (steady) | a bare `drawImage` per tile |
|---|---|---|---|
| small | 817 | 3.4 ms | 0.7–2.5 ms |
| large | 3,367 | 8.5 ms | 10–12 ms |

Zoomed in (k = 40, ~200 tiles visible), both are under 0.5 ms. The atlas's per-tile draw alone
costs more than main's whole terrain pass on a large map at fit, so build-1's bar is an absolute
16 ms with the cache as the expected way under it.

## Saves and turns

A save at the pick on large seed 2 is 8.5 kB (440 log entries), and 8.7 kB ten turns later, so
autosaving after every action costs nothing worth throttling beyond spec item 14's once a second.
Ten End turns on large take 0.9 s through the driver.

## A save's digest

No CLI command prints a save's digest: `replay` prints the report, and `digest` only runs a
fresh seed. The canvas's and the review's played-game proof needs one, so build-1 adds
`digest --game FILE`.
