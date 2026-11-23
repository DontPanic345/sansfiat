# #441 spec: the shores and grounds on the retro maps

Measured on main (`f5f4620`) at the pick (`?size=S&seed=N&run=1`), from the page's `game.map`
(`proto/shore.mjs`, run from `engine/`; its output is `proto/shore.json`). Water is terrain 0 (sea, or a lake
where `l.lake`); off the map counts as water (the rim is sea on all seven). A tile's runs are its neighbours
read round the tile in `EDGE_DIR` order, split where land turns to water.

- **Neck**: a land tile with two or more water runs round it (land joined across water on two sides).
- **Strait**: a water tile with two or more land runs.
- **Bay** (sea): a water tile, not a lake, with at most two water neighbours, all in one run.
- **Cape**: a land tile with four or more water neighbours in one run.
- **Pond**: a lake of three tiles or fewer.
- **Mouth**: a river tile (`l.river > 0`) whose `l.flow` is a water tile; into the sea or a lake by `l.lake`.
- **Corners**: each hex vertex once (a tile's corner 1 with its E and SE neighbours, and corner 2 with its SE and
  SW), counted where one or two of its three tiles are water.

## Ground, by kind

| Map | Tiles | Sea | Lake | Coast | Plains | Forest | Hills | Mountain | Desert | Marsh |
|---|---|---|---|---|---|---|---|---|---|---|
| small2 | 817 | 210 | 84 | 196 | 118 | 33 | 72 | 62 | 34 | 8 |
| small3 | 817 | 351 | 35 | 220 | 47 | 49 | 42 | 42 | 31 | 0 |
| small6 | 817 | 235 | 69 | 208 | 100 | 61 | 53 | 52 | 39 | 0 |
| small9 | 817 | 184 | 133 | 201 | 67 | 48 | 77 | 70 | 37 | 0 |
| medium8 | 1657 | 386 | 230 | 422 | 201 | 85 | 126 | 124 | 83 | 0 |
| medium9 | 1657 | 351 | 193 | 400 | 87 | 213 | 169 | 147 | 97 | 0 |
| large8 | 3367 | 512 | 324 | 687 | 758 | 185 | 312 | 295 | 285 | 9 |

## Shores

| Map | Shore edges | One-water corners | Two-water corners | Necks | Straits | Sea bays | Capes | Lakes (sizes) | Mouths into sea / lake |
|---|---|---|---|---|---|---|---|---|---|
| small2 | 544 | 293 | 251 | 28 | 32 | 21 | 11 | 1, 1, 1, 6, 9, 12, 23, 31 | 6 / 0 |
| small3 | 562 | 281 | 281 | 27 | 22 | 23 | 26 | 1, 2, 4, 5, 10, 13 | 4 / 3 |
| small6 | 526 | 278 | 248 | 23 | 36 | 20 | 17 | 2, 2, 3, 7, 8, 21, 26 | 6 / 3 |
| small9 | 536 | 286 | 250 | 22 | 24 | 19 | 10 | 2, 3, 3, 3, 17, 33, 72 | 3 / 7 |
| medium8 | 1018 | 545 | 473 | 34 | 68 | 45 | 22 | 1, 1, 2, 2, 2, 3, 4, 6, 7, 8, 10, 19, 79, 86 | 6 / 8 |
| medium9 | 1032 | 546 | 486 | 33 | 68 | 37 | 21 | 1, 1, 1, 1, 8, 10, 14, 15, 22, 30, 35, 55 | 6 / 2 |
| large8 | 1710 | 927 | 783 | 39 | 145 | 67 | 27 | 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 4, 5, 6, 15, 18, 19, 22, 22, 27, 35, 44, 85 | 13 / 12 |

- Every map has dozens of necks and straits, bays and capes; ponds of one tile are on four of seven.
- Rivers reach water on every map: 3–13 into the sea, 0–12 into lakes.
- Corners are as many as shore edges, half of each kind: corner pieces are not a detail.

## Edges between kinds

| Map | Land edges between differing kinds | Water edges by land kind |
|---|---|---|
| small2 | coast|plains 95, hills|mountain 83, mountain|plains 77, coast|forest 65, coast|hills 61, hills|plains 61, coast|desert 58, desert|plains 45, forest|plains 42, coast|mountain 35, desert|hills 22, forest|hills 21, desert|mountain 17, forest|mountain 15, marsh|plains 11, coast|marsh 4, forest|marsh 3, hills|marsh 2, marsh|mountain 2 | coast|sea 240, coast|lake 174, hills|lake 39, lake|mountain 36, marsh|sea 16, plains|sea 12, hills|sea 11, mountain|sea 9, lake|plains 7 |
| small3 | coast|plains 115, coast|forest 82, coast|hills 56, hills|mountain 53, coast|mountain 38, coast|desert 35, desert|plains 29, forest|plains 28, desert|mountain 27, forest|mountain 25, forest|hills 20, hills|plains 19, desert|hills 14, mountain|plains 13, desert|forest 11 | coast|sea 416, coast|lake 84, hills|sea 27, mountain|sea 19, lake|mountain 9, hills|lake 7 |
| small6 | coast|plains 109, coast|forest 93, coast|hills 64, forest|plains 55, coast|desert 51, desert|plains 47, hills|mountain 46, coast|mountain 45, mountain|plains 41, desert|mountain 35, forest|hills 28, forest|mountain 28, hills|plains 27, desert|hills 7, desert|forest 4 | coast|sea 297, coast|lake 141, hills|lake 29, lake|mountain 23, mountain|sea 16, hills|sea 15, lake|plains 5 |
| small9 | coast|plains 99, coast|hills 91, hills|mountain 82, coast|forest 63, coast|desert 55, coast|mountain 51, hills|plains 51, desert|mountain 40, mountain|plains 39, forest|hills 34, forest|mountain 29, forest|plains 25, desert|plains 22, desert|forest 17, desert|hills 14 | coast|sea 238, coast|lake 187, lake|mountain 47, hills|lake 38, lake|plains 14, hills|sea 6, mountain|sea 6 |
| medium8 | coast|plains 279, coast|forest 155, hills|plains 152, hills|mountain 146, mountain|plains 117, coast|desert 115, coast|mountain 102, forest|plains 83, desert|hills 77, desert|plains 69, coast|hills 65, desert|mountain 51, forest|hills 41, forest|mountain 27, desert|forest 14 | coast|lake 499, coast|sea 357, lake|mountain 60, hills|lake 41, mountain|sea 27, hills|sea 16, plains|sea 12, lake|plains 6 |
| medium9 | coast|forest 271, hills|mountain 203, coast|hills 142, forest|hills 139, forest|mountain 132, coast|desert 126, forest|plains 104, coast|mountain 98, coast|plains 80, desert|plains 77, hills|plains 52, desert|hills 51, desert|mountain 38, desert|forest 36, mountain|plains 27 | coast|sea 495, coast|lake 326, hills|sea 63, lake|mountain 56, hills|lake 44, mountain|sea 36, plains|sea 10, lake|plains 2 |
| large8 | coast|plains 684, desert|plains 409, hills|mountain 390, mountain|plains 344, hills|plains 301, forest|plains 294, coast|desert 200, desert|hills 172, coast|hills 163, desert|mountain 160, coast|forest 145, forest|mountain 117, coast|mountain 99, forest|hills 67, desert|forest 20, marsh|plains 15, coast|marsh 3, desert|marsh 1, hills|marsh 1, marsh|mountain 1, forest|marsh 1 | coast|lake 712, coast|sea 664, hills|lake 93, hills|sea 67, lake|mountain 67, lake|plains 38, mountain|sea 34, marsh|sea 18, plains|sea 17 |

- Water meets coast mostly, but also hills, mountain, marsh and plains (a delta turns coast to plains), and lakes
  meet hills and mountain often: the bank goes on every land kind, not only coast.
- Marsh is on two maps (small 2, large 8), at deltas beside the sea.

## The source

`docs/proposals/map-art/terrain.png` has grass (the plains hex), two grass hexes with a hill (bottom-left row, the
hexes centred about x 165 and 358, y 854, the same size as the plains hex; each hill runs into the hex to its
right), the range, three woods, and a coastline: deep water with ripples behind a bank of tan blocks on grass
(x 700–1490, y 740–975). It has no desert, marsh, sand or lake-coloured water.

- No hex of water fits: the source's water is opaque only in bands. The largest clean blocks (blue, no ripple)
  are about 137×60 at (1270, 832) and 126×70 at (873, 838); with ripples, the band x 740–1290 × y 905–945 is
  water too. Mean water `#03416a`. Water has to be filled by repeating a block.
- The bank is a run of tan blocks about 15–20 px thick on the land side of the water, with a light rim on the
  water side. It runs roughly level at about x 1040–1240, y 880–905, and turns at about (1000, 870), (1250, 790)
  and (870, 830).

## The views (cameras checked on main)

The tile of each class nearest world (0, 0), ties to the lower index; each camera, set on the tile at the zoom
named, comes back unclamped (`proto/cams.mjs`).

| Map | Class | Tile | World x, y |
|---|---|---|---|
| small 2 | neck | 402 | −5.2, −9 |
| small 2 | mouth into sea (a delta: marsh, river ≥ 9) | 506 | 6.93, 3 |
| small 6 | bay | 376 | −0.87, 1.5 |
| small 9 | pond (also a lake bay) | 341 | −5.2, −3 |
| small 9 | mouth into lake (river ≥ 9) | 377 | 0, 3 |
| medium 8 | strait | 875 | 1.73, 0 |
| medium 9 | neck (a mouth, a lake bay, a strait and a cape within two tiles) | 1004 | 1.73, −9 |
| large 8 | marsh | 2639 | 31.18, 6 |
| large 8 | pond | 1289 | −16.45, −10.5 |

Every view in the spec's list was set at its tile, zoom and window on main and read back unclamped: small 2's neck
at k 35 (laptop) and k 6 and 5 (phone), the delta at k 35 and 18, the bay, the small-9 pond and the strait at k 35,
the small-9 mouth, medium 9's neck and large 8's marsh and pond at k 18.
