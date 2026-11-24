# #444 build-1: the clan cells in the atlas

Build-1 (easel) cuts the three clan cells and their masks from the actor study's second row, adds the `floor` to
`cut.mjs`, and lays out the six cells. Keys `clan.tent`, `clan.hearth`, `clan.walk` and `.mask` each: 112×96,
anchor (56, 90). Nothing under `engine/core`, `engine/cli` or `engine/web` changed, nor any map code.

## Checks

- `node www/assets/draw-atlas.mjs` twice from `engine/`: the same bytes both times.
  - `engine/www/assets/atlas.png` sha256 `5407c34890bd0c3c9102603a596d17456a6140c332f5d7a0c7f48d89d63a3538`
  - `engine/www/assets/atlas.json` sha256 `cb264c8ccba4605d688b48eed1cdfb40a6a53f44f4b2342022e172d5bb44c49b`
- `node www/assets/draw-atlas.mjs --check`: `atlas: 153 cut cells match their sources, 163 cells match layout()`.
  The new assert (`floor: 0` cuts as the region without `floor`) is among its asserts.
- Compare with the branch's base (`proto/compare-main.mjs`, `proto/compare.txt`):
  `main keys 157, keys whose pixels differ 0`. The control is the branch's base atlas (58426b6), not the `main`
  branch: `main` predates #474, which gave the unit cells their 96×96 size on this branch. Every one of the 157 keys
  keeps its pixels; the six clan keys are new; 123 rectangles moved down with the rows below the unit row.
- `node www/play/art.mjs`: passes (163 cells, every cell non-blank).
- `engine/check.sh` whole: see the end of this file.

## The cells

Every region is in `engine/www/assets/sources.js`, key `actor`, `floor: 128`, its keep-polygon its box with a 3 px margin
less what the box holds that is not the group. `scale` is cell px per source px (as the unit regions').

| Cell (and mask) | Region (source rect) | Ground (source) | Scale | Anchor | Floor | Cloth (source px, read off 4× crops) |
| --- | --- | --- | --- | --- | --- | --- |
| `clan.tent` | `clan.tent`: rect (76, 349) 257×194; keep is the box, notched at x 316 for y 390 to 492 so the drying rack's pole stays out | (205, 528) | 0.43 | (56, 90) | 128 | two polygons: the left flap (from the apex at (242, 388) round to the hem at (185, 530), the opening's dark and the pole at its foot left out) and the right flap ((250, 400) to (322, 486)); `where` lit (luminance ≥ 60) |
| `clan.hearth` | `clan.hearth`: rect (1169, 335) 286×221; keep follows the right tent's slope from (1452, 400) to (1404, 488), so the right tent and the woman stay out | (1320, 540) | 0.38 | (56, 90) | 128 | the left tent's canvas, one polygon from the apex (1295, 401) round the flanks to the hem at y 462 to 490; `where` lit. Poles, the man, the fire and the smoke are outside it |
| `clan.walk` | `clan.walk`: rect (614, 383) 255×157 (the donkey from x 874 is outside it) | (742, 534) | 0.44 | (56, 90) | 128 | three polygons: the man's tunic and trousers (647 to 698, 428 to 503), the woman's dress and shawl (733 to 811, 437 to 519), the child's tunic (815 to 863, 478 to 521); `where` lit and `b < 0.5 r`, which leaves out skin, hair, packs, staff and boots |

Each mask is its cell's spec with `cloth` and `{ op: 'grey', lo: 190 }`, as the unit masks are.

Each group fits about 108×88: the tent is 0.43 × 188 px high, the hearth 0.38 × 218, the walkers 0.44 × 151.

## The rim (floor against no floor)

`proto/rim.mjs` (`proto/rim.txt`). Opaque = cell alpha ≥ 128. Rim = an opaque pixel with a clear pixel (alpha < 128)
among its 8 neighbours. Red = r > 2g and r > 2b. Mask share = the share of a cell's opaque pixels its mask covers.

| cell | opaque, floor 128 | rim, floor 128 | opaque, no floor | rim, no floor | mask covers, floor 128 |
| --- | --- | --- | --- | --- | --- |
| clan.tent | 4754 | 621 (61 red) | 4811 | 622 (56 red) | 46.6% |
| clan.hearth | 3510 | 749 (73 red) | 3579 | 735 (61 red) | 23.2% |
| clan.walk | 4436 | 755 (148 red) | 4523 | 749 (115 red) | 37.3% |

What the floor did (`proto/floor.mjs`, `proto/floor.txt`): it cleared 2,065 (tent), 2,352 (hearth) and 2,351 (walk)
cell pixels' alpha. But the red-dominant opaque pixels barely moved: 222 to 218 (tent), 182 to 185 (hearth), 562 to 548
(walk). The rim's red sits at alpha 128 and up in the source; the floor does not reach it. Build-3 needs to see this
on the captures: a higher floor or a colour key would have to clear it, and the floor at 128 is the one that was asked for.

## What the cells show (`relay/specs/444/cells.png`, `proto/cells.mjs`)

Three rows at 3×: each cell and its mask on mid grey and forest green, then each cell with its mask multiplied in red,
blue, gold and the pale, drawn over it. The tent's hide and the hearth's canvas read; the walkers' tunics, dress and
child's tunic take the colour, their skin and packs don't. Known for build-3:

- The tent's right foot is cut at the notch (x 316), and the second basket (green apples, x 302 to 333) is clipped
  at the cell's right edge. The first basket reads.
- The hearth's mask is the canvas alone (23% of its opaque pixels): the fire, the man and the smoke are not cloth.

## Whole check

`engine/check.sh` whole, run once after the last edit: exit 0 (every script in `www/play/all.mjs` passes; `draw-atlas.mjs --check` passes).

## Build-3: final hashes and recipes

Build-3 judged the cells on the map beside main (`views.md`) and left every recipe as cut: the scales 0.43 (tent), 0.38 (hearth), 0.44 (walk),
the ground points, the keep polygons, the cloth polygons and `{ op: 'grey', lo: 190 }`, and the floor 128. The atlas is the build-1 one:
`atlas.png` sha256 `5407c34890bd0c3c9102603a596d17456a6140c332f5d7a0c7f48d89d63a3538`, `atlas.json` sha256
`cb264c8ccba4605d688b48eed1cdfb40a6a53f44f4b2342022e172d5bb44c49b` (rechecked; `--check` passes). What the captures showed of the cuts: the tent's
clipped second basket and the hearth's canvas-only mask (23% to 35% of its cell) do not hurt the read at k 18 or 35; the red rim at alpha 128 and up
shows as a faint warm edge on a pale tent, hidden by the dark edge `clans.js` now draws under every cell. The look was fixed in `layers/clans.js`
(the dark edge, `CLAN.from`, the walkers' height), not in the cuts.
