# #445 build-1: the place cells in the atlas

Build-1 (easel) cuts the three place cells and two masks from the actor study's third row, adds one row to the atlas
(`PLACE` in `placeholders.js`, the regions and cells in `sources.js`), and writes `cells.png`. Nothing under
`engine/core`, `engine/cli` or `engine/web` changed, nor any map code (`www/map/`, `www/atlas.js`, `www/legend.js`).

## Checks

- `node www/assets/draw-atlas.mjs` twice from `engine/`: the same bytes both times.
  - `engine/www/assets/atlas.png` sha256 `e28fc88631d119f1750a1f219d0f3c82663771eb33a6186e3b5e24b131193f0d`
  - `engine/www/assets/atlas.json` sha256 `35beec0ece4955345f5777ce5b7b71945111f8d010eec751ee07f2e4635efb8a`
  - The sheet is 1024×2544 (main's was 1024×2416): one 128 px row.
- `node www/assets/draw-atlas.mjs --check`: `atlas: 158 cut cells match their sources, 168 cells match layout()`.
- `relay/specs/445/proto/compare-main.mjs` (`proto/compare.txt`), from the repo root against main's atlas:
  `main keys 163, keys whose pixels differ 0, sheet 1024x2544 (main 1024x2416)`. Rectangles moved 123 (the rows below
  the new row); no size changed; new keys are the five place keys.
- `node www/play/art.mjs` (after `build-web.sh` put `www/pkg` in place, as `check.sh` does): passes, 168 cells, every
  cell non-blank.

## The cells

Every region is in `engine/www/assets/sources.js`, key `actor`, `floor: 128`, each rectangle the group's box (alpha ≥ 128)
with a 3 px margin, its keep-polygon the same rectangle. Anchor (88, 118) for all five, `PLACE` 176×128.

| Cell (and mask) | Region (source rect) | Ground (source) | Scale | Cloth (source px, read off 6× crops) |
| --- | --- | --- | --- | --- |
| `place.town` | `place.town`: rect (110, 657) 305×206 | (262, 843) | 0.56 | none (no owner: a town carries no colour) |
| `place.settlement` | `place.settlement`: rect (606, 592) 385×302 | (798, 870) | 0.40 | none (the palisade and the huts are not cloth) |
| `place.settlement.mask` | as `place.settlement`, with `cloth` and `GREY` | (798, 870) | 0.40 | the flag: one 12-point polygon round the cream cloth, from (887, 650) to (941, 698); the pole and the smoke left out; the chevron kept in it |
| `place.capital` | `place.capital`: rect (1145, 611) 456×282 | (1373, 866) | 0.37 | none |
| `place.capital.mask` | as `place.capital`, with `cloth` and `GREY` | (1373, 866) | 0.37 | two banners on the hall's front: the left (1325–1346, 695–731) and the right (1424–1447, 696–735), 10-point polygons each; the posts, roof and walls left out |

The ground points are where each group's front stands, not the middle of its footprint. At the middle the groups' feet
ran past the cell (the palisade's ring 118 px down at 0.40), so the ground point sits at the front of the ring and
the hall's front, and each group's foot lands at the cell's foot. The first ground points were (798, 866) and (1373,
862); those clipped 14 and 30 px of the bottom row, so they moved to 870 and 866. The scales are about the spec's.

`GREY` is `{ op: 'grey', lo: 190 }`, as the clans' and units' masks are. The mask pixels that are not cloth are clear.

## Each cell's numbers (`proto/metrics.mjs`, `proto/metrics.txt`)

Opaque = alpha ≥ 128. Rim = an opaque pixel with a clear (alpha < 128) pixel among its 8 neighbours. Red = r > 2g and r > 2b.
Mask covers = the share of the owned cell's opaque pixels the mask covers (mask alpha ≥ 128).

| cell | rect (x, y) | opaque | edge rows or columns with opaque | rim (red) | mask covers |
| --- | --- | --- | --- | --- | --- |
| place.town | 0, 932 | 10723 | none | 757 (21) | |
| place.settlement | 176, 932 | 10041 | none | 1501 (40) | |
| place.settlement.mask | 352, 932 | 348 | none | 76 (0) | 3.5% |
| place.capital | 528, 932 | 11872 | none | 803 (34) | |
| place.capital.mask | 704, 932 | 586 | none | 156 (0) | 4.9% |

No cell has an opaque pixel on any edge: each group sits inside its 176×128 cell. The rim's red counts are small
(0 to 40 red pixels per cell); the hamlet has 21, the palisade 40, the hall 34. Build-3 reads the captures for any fringe.

## What the cells show (`cells.png`, `proto/cells.mjs`)

Five cells at 2× on mid grey and on forest green (row 1, 2), then the settlement and the hall each with its mask multiplied
in red, blue and gold drawn over it (rows 3, 4). The hamlet reads as three thatched huts; the palisade as a ring with
its huts, smoke and the flag; the hall as a civic hall on its mound with its houses. The masks show only the flag's and
the banners' cloth. The chevron reads dark in the tint. Build-3 judges the look and the cloth.

Known for build-3:
- The palisade's ring sits low in its cell (its front at the foot), so a unit beside it at the flat foot may cover it.
- The hall's mask keeps only the two banners; at the cell's size they are small (1.7% of the hall's pixels).
- Ground points are the fronts, not the footprints' middles; a crowded place's clans may need the anchor moved.

## Placeholders

Each cell has a placeholder in `placeholders.js` (`placePiece`): three huts for the town; a fence ring, a hut and a flag
for the settlement; a mound and a hall-and-houses row for the capital; the masks are the cloth in white. `play/art.mjs`
requires every cell non-blank, and passes.

## Decisions

- The ground points are at the front of the group (see above). Build-3 may move them with the scales.
- The palisade's chevron is kept in its mask (inside the flag's cloth), so the mark stays dark under the civ's colour.
- The town has no mask: a town carries no colour (the model gives it no civ).

## Build-3: the hall's pennant (a variant authored from the study's pixels)

The hall's two banners are 1.7% of its pixels and small at every zoom: Gold's vanish against the thatch, and the town diamond
(which covers the roof's peak on a capital that reads as a town) hides what is near the centre. `place.capital` and its mask
gain a **pennant**: the study's own flag (the unit flag's pixels, `FLAG_CLOTH` as its cloth) at 0.3, its whole pole, planted
on the roof's right shoulder, `over` the hall at (112, 3) of the cell (region `place.flag`, a rectangle of the actors sheet,
no new key and no new pixels from outside). It stands to the right of the diamond's reach, so a crowded capital's colour
still shows. The keys, size and anchor are as before; the two banners stay. Both hashes above are after it
(`--check` passes, `compare-main`: `main keys 163, keys whose pixels differ 0`, `proto/compare.txt` as before).
`proto/metrics.txt` is re-run: the capital 11,872 opaque, its mask 586 (4.9%). `cells.png` shows it.

The palisade's cell, the hamlet and the settlement's mask are as build-1 cut them.
