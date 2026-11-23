# #441 build-1 (easel): the atlas — new cut ops, the grounds, the shore and fringe cells

Read `relay/specs/441/spec.md` first (the keys, the decisions, how pieces are chosen) and
`relay/specs/441/shores.md` (the source: where its water, bank and hills are). Run
`relay/specs/441/proto/shore-cut.mjs` and look at `proto/patch3x.png`: it is the fill and the turned
bank working. Build-2 draws the cells you make on the map; build-3, the canvas, judges how they look and
may change any recipe. Your job is ops that are exact and first cuts that are visible and the right
kind of ground. No Rust changes: nothing under `engine/core`, `engine/cli` or `engine/web`. No map
code either: `www/map/` is build-2's.

Read before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js,png.mjs}`,
`www/map/layers/terrain.js` (how a ground and its `flat` are drawn), `www/map/geom.js` (`CORNER`,
`EDGE_DIR`), `www/play/art.mjs`, and `relay/specs/440/easel.md` (the atlas's height moves a few pixels
of old views: expected).

## 1. The ops, in `www/assets/cut.mjs`

Keep every existing region's cut identical to the pixel (`--check` proves it on the 17 cut cells).

- **fill** (hex regions): a hex region may name `fill: { rect: [x, y, w, h], scale, offset: [ox, oy] }`
  instead of mapping its hex onto a source hex. A cell sample at cell px `(x, y)` reads source
  `rect.x + fold(floor(x / scale + ox), w)`, `rect.y + fold(floor(y / scale + oy), h)`, where `fold`
  reflects every other copy (`proto/shore-cut.mjs`'s `fold`), so copies seam. Alpha is the hex's
  coverage, as for any hex region. `patches` work on a filled region too (they paste over the source
  box before the fill reads it).
- **ramp** (an op on any cell, after the cut, before `mirror`): `{ op: 'ramp', dark: [x, y, w, h],
  light: [x, y, w, h], mix }`. `dark` and `light` are source rectangles of the region's file, each
  taken as its mean colour. Each pixel's luminance (Rec. 601) is normalised over the cell's opaque
  pixels' range, mapped onto the line from `dark` to `light`, and blended with the pixel's own colour
  by `mix` (1: all ramp). Alpha is untouched. Ops become strings (`'mirror'`) or objects; an unknown
  op throws, as now.
- **rotate** (sprite regions): `rotate: deg` turns the cut about the ground point: a cell sample
  maps to source `ground + Rot(−deg) · (cell − anchor) / scale`. `rotate: 0` and no `rotate` give
  identical cells.
- **sprite keys**: add a key `bank` beside `ground` that clears what is not the bank's tan blocks
  (start from the proto's keep test, `r > 0.9 g && r > b + 30`, with alpha above 200), with its note.
- **derived cells**: a `CELLS` entry may be `{ from: 'terrain.desert.0', fringe: { dir, variant } }`
  instead of naming a region: the cut of the cell `from` names (at its own layout size), its alpha
  multiplied by a fringe mask, then cropped to the derived cell's layout rectangle (below). The mask,
  in the source cell's px: 1 inside the hex within depth `D(t)` of the hex's edge `dir` (the edge from
  corner `dir` to `dir + 1`, the one facing `EDGE_DIR[dir]`), falling to 0 over `FEATHER` px past it,
  and tapered to 0 over `TAPER` px at each end of the edge; `D(t) = DEPTH + AMP · n(t)`, `t` the
  position along the edge and `n` a smooth deterministic noise in [−1, 1] (a sum of two or three sines
  whose phases come from `dir` and `variant`), so the strip's inner edge is ragged and the two
  variants differ. Start at `DEPTH` 16, `AMP` 6, `FEATHER` 4, `TAPER` 12 (cell px); the constants
  live with `HEX_R` in `placeholders.js`, each with a note, so `layout()` and `cut.mjs` share them.

`cut()` returns `flat` for every cell as now (mean of its opaque pixels); a derived cell with no opaque
pixel may give `flat` null rather than throw, since it is never a flat fill. `atlas.json`'s `src` for a
derived cell is `from`'s region.

In `--check` (no Chrome, seconds), add three asserts beside the existing ones: a sprite region cut
with `rotate: 0` equals it without; a filled hex's samples on either side of a copy boundary are
mirror images; every derived cell's layout rectangle holds every pixel of its mask that is above 0.

## 2. The keys, in `www/assets/placeholders.js`

Append to `layout()` after the mountain pieces, in this order, so no older rectangle moves (check: every
main key keeps its `x`, `y`, `w`, `h`):

| Keys | Size | Anchor |
|---|---|---|
| `shore.edge.<d>.0`–`.1`, `d` 0–5 | 112×112 | (56, 56): the edge's midpoint |
| `shore.in.<c>`, `shore.out.<c>`, `c` 0–5 | 48×48 | (24, 24): the vertex |
| `fringe.<kind>.<d>.0`–`.1`, kind in `FRINGE` order (desert, marsh, forest, hills, mountain, coast), `d` 0–5 | the box of the mask for edge `d`, in a 128×148 hex cell, rounded out to whole px | the hex centre's place in that box (it may lie outside the box) |

Each fringe's box comes from the mask's geometry alone (the edge, `DEPTH + AMP + FEATHER`), not the
texture, so `layout()` computes it without cutting. Each new key is cut (`CELLS`), so its placeholder may
be empty; `play/art.mjs` still passes (no cut cell blank). Notes say what each is, as the mountain keys'
do.

## 3. The cells, in `www/assets/sources.js`

First cuts: each must show as the right kind of ground on the map (the terrain layer draws them as soon
as they exist) and carry no hex outline, neighbour's hill or halo. The recipes below are starting
points; adjust a rectangle that cuts the wrong thing and say so in `atlas.md`.

- **sea** `terrain.sea.0`–`.2`: a filled region on the clean block `[1270, 832, 137, 60]`, scale
  74/109 (the plains'), offsets `[0, 0]`, `[37, 23]`, `[71, 41]`.
- **lake** `terrain.lake.0`–`.2`: the sea's cells with a `ramp` toward lighter water (dark the block,
  light the bank's pale water rim or the ripples' highlights), so the lake's `flat` is clearly lighter
  than the sea's, as main's `#3f8fc4` is beside `#1f4f7a`.
- **coast** `terrain.coast.0`–`.2`: the plains' three recipes with a `ramp` toward the bank's tan
  (`mix` about 0.35): grass, warmer.
- **forest** `terrain.forest.0`–`.2`: the plains' recipes with a `ramp` toward the shade under the
  source's woods (about x 330–960, y 560–640), darker and cooler than plains.
- **hills** `terrain.hills.0`–`.2`: hex regions on the bottom-left row's first two hexes (centres about
  (165, 854) and (358, 854), the plains hex's size and inset), each with grass patched over the hill
  that runs in from its left neighbour; `.2` the first mirrored.
- **mountain** `terrain.mountain.0`–`.2`: as now.
- **desert** `terrain.desert.0`–`.2`: the plains' clean recipe with a `ramp` onto the source's sand
  and ochre (dark the hills' shaded ochre, light the bank's top), `mix` 1.
- **marsh** `terrain.marsh.0`–`.2`: the plains' grass recipe with a `ramp` toward the woods' shade
  (`mix` about 0.4) and two or three pools patched in from the water block.
- **bank edges** `shore.edge.<d>.<v>`: a sprite region on the level run of bank (`proto/shore-cut.mjs`:
  x about 1040–1240, water line y 902, from 22 px above it to 4 px below), key `bank`, ground point on
  the water line at the strip's middle, scale 74/109, `rotate: 60 d − 90` (the source run faces south;
  `EDGE_DIR[d]` points from the land to the water). Variant 0 from the run's left part, 1 from its
  right part, each about 124 source px long (84 cell px: the edge and a little past each end).
- **bank corners** `shore.in.<c>`, `shore.out.<c>`: a disc of bank blocks about 12 cell px across from
  the same run, centred on the anchor, key `bank`; one cut serves all twelve to start.
- **fringes** `fringe.<kind>.<d>.<v>`: derived from `terrain.<kind>.<v>` with `fringe: { dir: d,
  variant: v }`.

## Done

1. `node www/assets/draw-atlas.mjs` twice: identical sha256; `--check` passes with the three new
   asserts, and fails when a byte of `shore.edge.0.0` in a scratch copy changes.
2. `atlas.json` has every main key at main's `x`, `y`, `w`, `h`, `ax`, `ay`, and every older cut
   cell's pixels and `flat` are main's.
3. `node www/shots.mjs --out /tmp/b1 --only small2-close,small2-play,small2-flat,large8-play` on the
   branch: every ground shows its first cut at k 35 and 18 (no placeholder hex left), and the flat fill at k 5 shows each kind's new `flat`.
4. `relay/specs/441/atlas.md`: the sha256 lines, the `--check` output, the sheet's size, each new
   key family's size and anchor, the eight grounds' and the lake's `flat` colours beside main's
   placeholder colours, any recipe you changed and why, and `relay/specs/441/cells.png`: every new
   and recut cell at 2× on a mid-grey sheet, labelled by key or in key order, for the canvas.
5. `engine/CLAUDE.md` (an 8 KB budget `check.sh` enforces): `cut.mjs`'s ops in a few words.

Run `engine/check.sh` once at the end. Commit as `#441 build-1: the ground and shore cells`.
