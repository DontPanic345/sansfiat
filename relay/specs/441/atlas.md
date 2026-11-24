# #441 build-1: the atlas

## Builds and checks

- `node engine/www/assets/draw-atlas.mjs`, run three times: `atlas.png`
  `20b9177931bf07dcc767ce2a6fc4807cc4cba822b14ba56f24726240525bd3de`, `atlas.json`
  `fa5faceb72a8b48cecbf79d91f9cd2c9002b8223e3303af10e79bb19af6dcfba`, the same both times.
- `node engine/www/assets/draw-atlas.mjs --check`:
  `atlas: 134 cut cells match their sources, 148 cells match layout()`.
  The three new asserts (in `asserts()`) pass: a sprite cut with `rotate: 0` equals the cut without (`shore.edge.0.0`,
  `mountain.peak.0`); a fill's copies mirror across the copy boundary at x 40 (a synthetic 40-px fill through the real
  `cut()`); every derived cell's rectangle holds every fringe mask pixel above 0.
- Mutation: one byte of `shore.edge.0.0` changed in a scratch copy makes `--check` exit 1 (`1 pixels differ`).
- The sheet is 1024 × 2064 (main 1024 × 1300). Its 52 main keys keep `x`, `y`, `w`, `h`, `ax`, `ay`; the 17 older cut
  cells have main's pixels and `flat` exactly (compared against main's `atlas.png` and `atlas.json`, not just
  `--check`). `atlas.png` is 1.1 MB (main 0.38 MB): the new cells are real art.

## New key families (`layout()`, after the mountain pieces)

| Keys | Size | Anchor |
|---|---|---|
| `shore.edge.<d>.<v>`, d 0–5, v 0–1 (12) | 112 × 112 | (56, 56) |
| `shore.in.<c>`, `shore.out.<c>`, c 0–5 (12) | 48 × 48 | (24, 24) |
| `fringe.<kind>.<d>.<v>`, 6 kinds × d 0–5 × v 0–1 (72) | d0 26×74 (−38, 37); d1 77×59 (13, −15); d2 77×59 (64, −15); d3 26×74 (64, 37); d4 77×59 (64, 74); d5 77×59 (13, 74) | the hex centre in the box |

Fringe constants (`placeholders.js`): `DEPTH` 16, `AMP` 6, `FEATHER` 4, `TAPER` 12. The box is the mask's
reach (depth + AMP + FEATHER inside the hex, tapered ends), not its texture.

## Flat colours

The map's flat fill is the cell's `flat` (`terrain.js` `flatColour`); the mountain keeps `mountain.peak.0`.

| Kind | Flat now | Main's placeholder colour |
|---|---|---|
| sea | #04436c | #1f4f7a |
| lake | #226995 | #3f8fc4 |
| coast | #978550 | #d6c487 |
| plains | #7b882d | (cut, unchanged) |
| forest | #616c26 | #3f7a3a |
| hills | #988d3b | #8a7a5c |
| mountain | #777370 | #7c7a85 |
| desert | #6e6d3a | #efe3b8 |
| marsh | #69752c | #5f8a6a |

Closest pairs of the nine (RGB distance; build-3 holds 28): **forest–marsh 13.5**, **desert–marsh 16.9**,
**coast–hills 22.5**, **forest–desert 23.9**, **plains–marsh 26.2**. The next pair is plains–hills at 32.6; main's closest is
hills–mountain at 28 or more. Build-3 owns these five: the marsh is the grass with woods' shade, close to the forest
and to the plains, and the desert's ramp lands near the marsh.

Lake is lighter than sea, as main's is (`#226995` against `#04436c`).

## Recipes changed from the brief, and why

- **Lake**: dark is a 16×8 window of the sea's lighter water (919, 902), light a 4×4 highlight of it (919, 902). The
  brief's light (a pale rim) and dark (the block) left the lake 1.7 from the sea: the ramp normalises over the cell's
  luminance range, and the sea block's bright specks stretch the range so that most pixels stay near the dark end.
  A light end that is itself the palest blue lifts the mean.
- **Coast**: the ramp (grass dark to bank tan, mix 0.35) left the coast 9.5 from the plains. The coast's flat is the
  bank's tan rectangle (1080, 884, 100, 12), so the flat is warm (`#978550`); the cells still show grass warmed by the
  ramp.
- **Bank tan**: `bankTan` is (1080, 884, 100, 12). The brief's row 879 averages with grass above the bank to olive.
- **Desert**: the plains' three recipes (clean, grass, clean mirrored), each ramped from the hill's shade to the bank's
  tan at mix 1, not the clean recipe alone (the variants would repeat).
- **Hills**: hex centres at 164.5 and 358, the plains' spacing (193.5) from 551.5. `hills.0` takes no patch: nothing runs
  in from the left of the first hex. `hills.1` has the first hill's tail patched out (grass from (262, 760) over
  (262, 820), 46 × 80). `hills.2` is `hills.0` mirrored. Flat: the mound's ochre (160, 845, 60, 25), `#988d3b`.
- **Marsh**: the grass hex with two pools patched in from the water block, (520, 826) and (585, 880), then the
  woods-shade ramp at mix 0.4. The rocks of the grass hex are still in it.
- **Forest**: the plains' three recipes ramped from the woods' shade toward grass light at mix 0.7 (the brief gave no mix).
- **Rotate** is on the cell (`shore.edge.<d>.<v>`: `60 d − 90`), so one region serves all six edges; a region's `rotate`
  is also honoured.
- **Bank strips**: the run is (1040, 878, 200, 29); variant 0's ground is x 1102, variant 1's x 1178 (the strips are
  124 source px, their halves of the run). The corner disc is a 32-gon of radius 8.8 source px (12 cell px) at (1102, 898).

## The cells

`relay/specs/441/cells.png`: every new and recut ground, edge, corner and fringe at 2×, on mid grey, labelled by key
and flat (rendered from `atlas.png` in headless Chrome). Water, lake, coast, forest, hills, desert and marsh read as
their kinds; the bank follows its edge and sits on the land side of it; the corners are small discs; the fringes are
ragged strips along their edges. The look is build-3's to judge.

## Views

`node www/shots.mjs --out /tmp/b1 --only small2-close,small2-play,small2-flat,large8-play` on the branch: each view
draws its grounds from the first cuts, with no placeholder hexes left. The shore edges are not drawn yet (build-2).
