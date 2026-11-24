# #443 build-1: the unit figures and their masks in the atlas

## Builds and checks

- `node engine/www/assets/draw-atlas.mjs`, run twice: the bytes are the same both times (`cmp`), and the sha256 is
  `atlas.png` `541a12f9d12c3c65a8061b868e285c3ee7a96dd6c9bfbc0cdafa70beda72eb44`, `atlas.json`
  `47be6d6d43cc9e6612b611a8516454fae443ac1f72e20dbd85f432d23a8319fd`. The sheet is 1024 × 2320 (main 1024 × 2288):
  the unit row is 96 px tall, so the rows below it move down by 32.
- `node engine/www/assets/draw-atlas.mjs --check`: `atlas: 147 cut cells match their sources, 157 cells match layout()`.
  The new asserts in `asserts()` pass: an `over` of no regions is the plain cut; a cloth polygon round the whole
  rectangle is the plain cut; `grey` on a white cell is white.
- Main's keys (`node relay/specs/443/proto/compare-main.mjs /tmp/mainatlas`, with main's `atlas.json` and `atlas.png`
  from `git show main:engine/www/assets/…`): all 157 keys of main keep their pixels, 0 keys differ. Compared by key,
  each at its own rectangle. 126 keys moved down with their rows; the four unit cells are the only ones that changed
  size (64 to 96) and are excluded from the pixel check.
- `node engine/www/play/art.mjs` (after `engine/build-web.sh`): `{"cells":157,"sheet":"1024x2320","terrainCells":127,"kinds":3}`, exit 0.
- `engine/check.sh`, whole, once: exit 0, 26 touch-point scripts pass. The last edit after it changed one cell note (the atlas.json hash above); the atlas was rebuilt twice and `--check` and `art.mjs` rerun after it.
- `cells.png` (`proto/cells.mjs`, headless Chrome): the four cells at 3×, on mid grey and on forest green, the anchor
  marked in magenta; then each figure with its mask multiplied in red, blue and gold (`#d64545`, `#3b6fd6`,
  `#d6a63b`) drawn over it.

## Cells

| Key | Size | Anchor | Figure | Scale | Over | Cloth | Ops |
|---|---|---|---|---|---|---|---|
| `unit.camp` | 96×96 | 48,92 | `actor.camp`: rect 580,69 135×193, ground (feet) 645,256 | 0.48 | `actor.flag` at 25,34 | none | none |
| `unit.camp.mask` | 96×96 | 48,92 | `actor.camp` as above | 0.48 | `actor.flag` at 25,34 | `actor.camp`: the shield face (a 24-point ellipse 31 × 34 about (675.5, 193.5)) and `CAMP_TUNIC` (a 12-point polygon round the body, samples of luminance 60 and up); `actor.flag`: the cloth, 14 points | `grey` from 190 |
| `unit.scout` | 96×96 | 48,92 | `actor.scout`: rect 190,104 117×155, ground (feet) 246,253 | 0.6 | `actor.flag` at 75.5,30 | none | none |
| `unit.scout.mask` | 96×96 | 48,92 | `actor.scout` as above | 0.6 | `actor.flag` at 75.5,30 | `actor.scout`: `SCOUT_BODY` (13 points) and `SCOUT_SHORTS` (5 points), samples of luminance 60 and up; `actor.flag`: the cloth | `grey` from 190 |

`actor.flag` is rect 1357,97 110×163, ground (the pole's top) 1372,106, scale 0.2. Each `actor.*` region's keep-polygon
is its rect (the study's box with a 3 px margin); no other figure falls inside any of the four margins. The camp's
figure stands 89 px tall in the cell (rows 4 to 92), the walker 89 (5 to 93); the flag is 31 px tall with its pole
(a third of a figure). The mask cloth is the flag's cloth (the pole is out of it).

Measured by `proto/shares.mjs`, the share of each figure's opaque pixels (alpha 128 and up) its mask covers:
the camp's mask covers 1001 of 3315 (30.2%), the scout's 200 of 4114 (4.9%); the masks have no pixel off their figure.

## Decisions taken in the build

- **A `grey` op normalises over the cell's opaque pixels to [90, 255]**, as the brief says. A flat cell (the brightest
  pixel is also the darkest) maps to 255, so white stays white.
- **`cloth` names its regions**: `cloth: { 'actor.flag': [...] }` keeps samples of that region only. A region a
  `cloth` cell does not name keeps none, so the scout's walker leaves no pixel in its mask.
- **The scout's banner is at 75.5, 30, not 77.** At 77 the cloth ran to the cell's right edge (its last opaque column
  was 95 of 96 and the edge was clipped). At 75.5 it ends at column 94 and the pole sits half a pixel left of the staff.
- **The camp's shield face is an ellipse that includes the boss.** The boss is grey metal, so it is lighter in the
  grey and comes out lighter in the tint; build-3 may cut it out with a second polygon.

## Run notes

- `art.mjs` opens the page, so it needs the wasm build (`engine/build-web.sh`, into `www/pkg`, git-ignored) first; without
  it, it times out waiting for `window.game`, as 442 did.
- The map (`www/map/layers/units.js`) still draws each unit cell at the old size, so on the branch the figures stand
  larger than main's shields until build-2 lays them out. That is the piece's order, not a regression to fix here.

## Build-3's final recipes

- Hashes above are build-3's final atlas (`draw-atlas.mjs` run twice: the same; `--check` passes with the new assert; all
  157 of main's keys keep their pixels, 0 differ by `proto/compare-main.mjs`). `cells.png` is regenerated.
- **The tunic is cloth.** A cloth entry is a polygon or `{ poly, where }` (the polygon's samples whose colour `where(r, g, b)` accepts).
  `CAMP_TUNIC`, `SCOUT_BODY` and `SCOUT_SHORTS` are polygons read off a 6× crop of each figure (`crop6(ox, oy, points)`, the
  crop's px over 6 from the region's origin) with `where` luminance 60 and up (the outline stays black). Tinted share of the opaque pixels at k 35 (`units.txt`):
  the scout 26% (412 of 1595 px), the camp 40% (485 of 1202).
- **The grey runs from 190 to 255** (`GREY = { op: 'grey', lo: 190 }` in `sources.js`; the op without `lo` keeps `GREY_LO` 90). 
  The banner's cloth and the shield's face share it, so the camp's shield boss is still lighter than its face.
- A hue test (ochre by r, g, b) was tried for the tunic and dropped: the study's tunic ranges too far in shading.
