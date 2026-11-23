# #442 build-1: the atlas's tree and reed cells

## Builds and checks

- `node engine/www/assets/draw-atlas.mjs`, run twice: `atlas.png`
  `d14706fc95f2155b6096a6af7738df5c79cfdd937dab8adb837bca61a94995ce`, `atlas.json`
  `eaa638fbe448577423c08e65bfc35915148b4ac330a8cc5b0f0b9ccd7d8dacf5`, the same both times.
- `node engine/www/assets/draw-atlas.mjs --check`: `atlas: 143 cut cells match their sources, 157 cells match layout()`.
  The new assert (in `asserts()`) passes: `tree.big.0` cut with `scaleY` equal to its `scale` equals the cut without it.
- Main keys: `node relay/specs/442/proto/compare-main.mjs /tmp/mainatlas` (from the repo root, with main's `atlas.json`
  and `atlas.png` in that folder, from `git show main:engine/www/assets/…`): all 148 keys of main keep their `x`, `y`,
  `w`, `h`, and their pixels, 0 rectangles and 0 pixels differ. The nine new keys are `tree.big.0–2`,
  `tree.small.0–2`, `reed.0–2`. The sheet is 1024 × 2236 (main 1024 × 2064); the new rows sit after the fringe rows.
- `node engine/www/play/art.mjs`: passes, `{"cells":157,"sheet":"1024x2236","terrainCells":127,"kinds":3}`, exit 0.
- `cells.png`: the nine new cells at 3×, on mid grey, labelled, rendered from `atlas.png` in headless Chrome
  (`proto/cells3x.mjs`). `regions.png`: each sprite region's source rectangle at 3×, its keep-polygon outlined in magenta
  and its ground point in cyan (`proto/regions.mjs`).

## Cells

| Key | Size | Anchor | Region | Scale | Key | Ground point |
|---|---|---|---|---|---|---|
| `tree.big.0` | 64×80 | 32,76 | `tree.big.0`: the left clump's front tree, rect 130,556 68×92 | 0.7 | `tree` | 160,637 |
| `tree.big.1` | 64×80 | 32,76 | `tree.big.1`: the band's front tree, rect 448,562 50×76 | 0.7 | `tree` | 473,632 |
| `tree.big.2` | 64×80 | 32,76 | `tree.big.2`: the band's front tree at the right end, rect 600,542 62×96 | 0.7 | `tree` | 627,632 |
| `tree.small.0` | 40×52 | 20,49 | the right group's tail, rect 1396,558 36×50 | 0.7 | `tree` | 1412,603 |
| `tree.small.1` | 40×52 | 20,49 | the right group's tail, slim, rect 1432,538 48×66 | 0.7 | `tree` | 1452,600 |
| `tree.small.2` | 40×52 | 20,49 | the mid tree, rect 1336,528 52×82 | 0.5 | `tree` | 1362,598 |
| `reed.0` | 32×40 | 16,37 | a plains tuft, rect 496,800 60×40 | 0.5 (scaleY 1.0) | `tuft` | 524,836 |
| `reed.1` | 32×40 | 16,37 | the small tuft by the plains grass, rect 556,786 48×30 | 0.8 (scaleY 1.6) | `tuft` | 578,811 |
| `reed.2` | 32×40 | 16,37 | `reed.0`, mirrored | 0.5 (scaleY 1.0) | `tuft` | 524,836 |

Every tree is cut from the studies' `terrain.png` with a keep-polygon round its own crown, the trees behind it
left out. Each `reed` is ramped from `RAMP.treeDark` to `RAMP.straw` (the brightest tuft core, rect 520,817 8×8),
at mix 1. Reeds stretch `scaleY` at twice `scale`, as the brief asks.

## Decisions taken in the build

- **The trees use a new key, `tree`, not `ground`.** The brief said `ground`, but `ground` clears any pixel with
  low blue (`b < 0.72 g`, the tuft test) and any green over red and blue. Sampled on the source, a bright leaf is
  `(82,102,43)` and a yellow highlight `(222,207,66)`, both cleared, so the crown came out as dark teal fragments
  (the first build; the render was rejected). `tree` clears nothing: the keep-polygon alone takes the tree out of the
  grass. `build-1.md` now names `tree`.
- **The reeds use a new key, `tuft`.** Its thresholds sit beside it in `cut.mjs`: a blade has red at least
  0.98 of green and blue under 0.4 of green (blades 0.29–0.37 of green in blue, grass 0.34–0.56, grass red 0.9–0.98).
- **`reed.1` is larger than `reed.0`** (scale 0.8, from 0.5): its tuft is 37 source px wide, and at 0.5 it was faint.
  It keeps `scaleY` at twice `scale`.
- **`reed.2` is `reed.0` mirrored**, as the brief allows; `reed.0` and `reed.1` come from different tufts.

## What reads and what doesn't (first cuts, read at 3×)

- `tree.big.0` and `tree.big.2` read as full trees. `tree.big.1` is narrow: its keep-polygon cuts the crown's left
  side, and the leaves at its left lower edge are lost. Build-3 can widen its keep.
- The three `tree.small` cells read as saplings and small trees. `tree.small.1` keeps a dark streak at its
  lower right, the grass's shade from the source.
- The reeds read as yellow-green clumps on a dark base. They are a first cut: build-3 may recut them.

## Source coordinates (what was settled by cropping `terrain.png` at 3×)

The crop tool is `proto/crop.mjs` (`node relay/specs/442/proto/crop.mjs x y w h 3 out.png`, from the repo root).
The coordinates in `spec.md` hold: the left clump's front tree has its base at (160, 637); the right group's tail
has trunks at (1412, 603) and (1452, 600); the plains tuft clusters are at about (500–550, 808–837) and (560–597,
792–812). The band's front row has trunks at (473, 632), (540, 623) and (627, 632); the first is used as
`tree.big.1`, the last as `tree.big.2`.

## Run notes

- The game page needs the wasm build (`engine/build-web.sh`, into `www/pkg`, git-ignored) before `play.mjs` can open it.
  Until that was built, every page check, `art.mjs` included, timed out waiting for `window.game`, on main too.
  Stale headless Chrome from killed runs was also stopped first.
- `node engine/check.sh` is run once at the end of the build; its result is in the summary of `result.json`.
