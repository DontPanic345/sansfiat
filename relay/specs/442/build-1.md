# #442 build-1 (easel): the atlas's tree and reed cells

Read `relay/specs/442/spec.md` first: its keys, its decisions, and "Settled here, by running" for where
the source's trees and tufts are. Build-2 places your cells on the map, and build-3, the canvas, judges
how they look and may change any recipe. Your job is cells that are exact in their keys, sizes and
anchors, and first cuts that are visible: a tree that reads as a tree, a reed clump that reads as
reeds. Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`. Don't change map
code: `www/map/` is build-2's.

Read these before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js,png.mjs}`,
`www/play/art.mjs`, `relay/specs/441/atlas.md` (how 441 cut and checked), and
`relay/specs/440/easel.md` (the atlas's height moves a few pixels of old views, which is expected).
The peaks in `sources.js` are the pattern to follow: a sprite region with a `keep` polygon, a `key`,
a `ground` point and a `scale`.

## 1. In `www/assets/cut.mjs`

Every existing cut must stay identical to the pixel. `--check` proves it on the cells already cut.

- **`scaleY`**: a sprite region may name `scaleY`, its vertical scale in cell px per source px. The
  default is `scale`. With it, a cell sample maps to source `ground + (dx / scale, dy / scaleY)`
  before any turn. Use it only on regions without `rotate` or `shape`: throw if a region has both.
- **A `tuft` key** beside `ground` and `bank`. It keeps the yellow tuft blades of the plains hex (low
  blue, red near green) and clears the green grass around them. Give it a note, and put its thresholds
  beside it, as the other keys have.

## 2. The cells

In `www/assets/placeholders.js`'s `layout()`, after the fringe rows, start a new row and put:

- `tree.big.0`–`.2`: 64×80, anchor (32, 76);
- `tree.small.0`–`.2`: 40×52, anchor (20, 49);
- `reed.0`–`.2`: 32×40, anchor (16, 37).

Each gets a note saying what it is and that its ground point is at the anchor, transparent outside.
These sizes are starting values. Change them if a cut needs room, but keep a cell's anchor at its
ground point. Give each a placeholder drawing in `draw()`, the way the mountain pieces have one: a dark
green cone on a trunk, and a few upright strokes.

In `www/assets/sources.js`, add the regions and `CELLS` entries:

- **`tree.big.*`**: three whole front trees, from the left clump's front row (bases along y 600–640,
  one about (130–192, 555–640) with its base at (160, 637)) or from the band's front row. Each gets a
  `keep` polygon round that tree's own silhouette, so no tree behind it comes along, key `tree`
  (a new key that clears nothing: `ground` cut the leaves' yellow and green, see `atlas.md`), and `ground` on the trunk's base. Start the scale near the
  peaks' (0.7). A variant may be another's `mirror`.
- **`tree.small.*`**: the right group's isolated small trees (about (1400–1428, 563–606) with its base
  at (1412, 603), and (1435–1470, 541–603) with its base at (1452, 600)), and the mid tree at about
  (1338–1386, 530–606) cut smaller. Same method.
- **`reed.*`**: a tuft from the plains hex (around (500–540, 810–830) or (560–590, 790–806)), key
  `tuft`, stretched with `scaleY` about twice `scale`, then a `ramp` onto the source's reed colours. A
  ramp end is a source rectangle's mean: start from `RAMP.treeDark` up to a straw taken from a tuft.
  Three variants, from different tufts, or one mirrored.

Coordinates are approximate. Find the real silhouettes by cropping the source at 3× (440's and 441's
`proto/` scripts show how). Write the regions you settle on, with a crop of each, into
`relay/specs/442/atlas.md`, and commit a 3× sheet of the nine cells as `relay/specs/442/cells.png`.

## 3. `--check` and the build

- `node www/assets/draw-atlas.mjs` builds the sheet. Run it twice: both runs give the same sha256.
  Record it in `atlas.md`.
- `node www/assets/draw-atlas.mjs --check` passes. Add one assert to its `asserts()`: a region with
  `scaleY` equal to its `scale` cuts identically to the same region without `scaleY`.
- `atlas.json` has every key main has, each at main's `x`, `y`, `w` and `h`. Check this with a few
  lines of node against `git show main:engine/www/assets/atlas.json`, and say so in `atlas.md`.
- `node www/play/art.mjs` passes: every cell lies inside the sheet and is not blank.
- Add the new keys and `scaleY` to `engine/CLAUDE.md`'s atlas lines.
- Run `engine/check.sh` once, before you finish.

## What later steps rely on

Build-2 draws `tree.big.<v>`, `tree.small.<v>` and `reed.<v>` (v 0–2) by key, at the terrain's scale,
with the anchor on a slot. The keys and their anchors-as-ground-points must hold. Build-3 may recut any
of them, resize them, or add keys after them.

## Size

About 6 files and 200 lines, plus the PNG: `cut.mjs` (378 lines, plus about 15), `sources.js` (204,
plus about 60), `placeholders.js` (360, plus about 25), `draw-atlas.mjs` (plus about 10),
`atlas.png`/`atlas.json`, `engine/CLAUDE.md`, `atlas.md` and `cells.png`.
