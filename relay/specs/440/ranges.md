# #440 spec: the mountains on the retro maps

Measured on main (55ed47b) at the pick (`?size=S&seed=N&run=1`), from the page's `game.map`
(`proto/classes.mjs`, `proto/nearest.mjs`, run from `engine/`). A tile's class is how many of its six
neighbours are mountain; with two, the turn between them: opposite is straight, two apart a bend (120°),
adjacent a sharp bend (60°).

## Every mountain tile, by class

| Map | Mountains | Lone | End | Straight | Bend | Sharp | Fork (3) | 4+ | On the rim |
|---|---|---|---|---|---|---|---|---|---|
| small 2 | 62 | 1 | 24 | 25 | 12 | 0 | 0 | 0 | 0 |
| small 3 | 42 | 4 | 12 | 17 | 5 | 0 | 4 | 0 | 0 |
| small 6 | 52 | 3 | 20 | 22 | 7 | 0 | 0 | 0 | 0 |
| small 9 | 70 | 2 | 18 | 26 | 16 | 1 | 6 | 1 | 0 |
| medium 8 | 124 | 7 | 37 | 45 | 19 | 2 | 11 | 3 | 0 |
| medium 9 | 147 | 6 | 43 | 41 | 16 | 9 | 18 | 14 | 0 |
| large 8 | 295 | 11 | 60 | 130 | 47 | 7 | 30 | 10 | 0 |

- No mountain touches the world's rim on any of the seven (the rim is sea), so the piece's "map-edge
  culling" is the canvas's edge: a piece whose tile is off screen and whose art reaches onto it.
- Forks and 4+ blobs are on five of the seven maps; valley walls and crossing chains make them. Sharp
  bends are rare but real (medium 9 has 9).
- Passes are hills (`map.rs`, `raise_ridges`: "a pass is hills where the chain would have been
  mountain"), so a ridge already breaks at its pass: the tiles either side are end caps.
- Tile indices are not in row order (`py` is not sorted by index), so depth needs a sort.

## The tiles the views centre on (nearest world (0, 0) of their class)

| Map | Class | Tile | World x, y | Mountain neighbours (0 E, 1 SE, 2 SW, 3 W, 4 NW, 5 NE) |
|---|---|---|---|---|
| small 2 | bend | 377 | 0, 3 | 3, 5 |
| small 2 | straight | 441 | 1.73, 0 | 2, 5 |
| small 9 | fork | 469 | 0, −6 | 0, 3, 5 |
| small 9 | bend | 474 | 4.33, 1.5 | 1, 3 |
| small 9 | end | 442 | 2.6, 1.5 | 0 |
| medium 9 | 4+ | 777 | −5.2, −6 | 2, 3, 4, 5 |
| medium 9 | fork | 732 | −6.06, −4.5 | 2, 4, 5 |
| large 8 | bend | 1937 | 0, −12 | 3, 5 |
| large 8 | fork | 1953 | 13.86, 12 | 0, 1, 3 |

The laptop map canvas is 1280×759 and at k 18 shows ±35.6 by ±21.1 world units, so each view's tile
and its range are on screen.

## A peak cut that works

`proto/peak.mjs` (run from the repo root) cuts the left peak of `terrain.png`'s range: rectangle
(110, 140) 280×180, a keep-polygon `[[110,140],[300,140],[300,150],[385,258],[372,300],[110,312]]`
that drops the next peak to its right and the hill below it, and a ground key that makes a pixel clear
when it is grass (`g > r + 6 && g > b + 25`) or yellow tufts and hill (`b < 0.6 g`). It area-resamples
(4×4 samples, alpha-weighted colour) at the plains' scale, 74/109 cell px per source px, into a 192×128
cell with the source's ground point (262, 292) on the cell's (96, 112). `proto/peak.png` is the cell and
`peak3x.png` it at 3× over the plains olive, the ground point in red: snow, rock and the foot's boulders,
no grass halo, no tufts. The peak is 170 cell px wide (1.3 hexes) and its tip 97 px above the ground
point (1.3 circumradii), so it rises about a quarter hex into the tile to its north.

The source's range is one opaque sprite (alpha ≥ 250 inside, 0 around it; the dark glow is colour under
alpha 0), so every piece is a rectangle, a polygon and the key: the peaks overlap each other in the art.
