# #439 build-2 (canvas): the plains tile on the map

Read `relay/specs/439/spec.md` and build-1's summary and `relay/specs/439/easel.md`. You are
the painter: the first illustrated tile on the map, judged by eye on the retro maps beside
main. No Rust changes; nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easel gives you

- `www/assets/sources.js`: the region `plains.grass` (an explicit hex in
  `docs/proposals/map-art/terrain.png`, an inset, patches) and the cells
  `terrain.plains.0`–`.2` cut from it (`.0` the cut, `.1` mirrored, `.2` the cut).
- `www/assets/cut.mjs` (region → cell), `png.mjs`, and `draw-atlas.mjs`, which writes
  `atlas.png` and `atlas.json` (each cell's `ax`, `ay`; a cut cell's `src` and `flat`) and
  `--check`s them in `check.sh`.
- `www/shots.mjs`: `--out DIR` shoots the spec's views (fixed seed, camera, zoom and window),
  `--diff A B` checks the cameras match and gives each view's changed-pixel share.

You may change any of it; record why in your summary. The keys and rectangles of every
existing cell stay, and `--check` and a byte-identical repeat build must still hold.

## 1. The terrain layer reads the atlas's anchor and flat colour

In `www/map/layers/terrain.js`:

- Place each cell by its anchor: the anchor (scaled from cell px to the drawn size) lands on
  the tile's centre, where the code now subtracts `img.width / 2` and `H / 2`. Every anchor
  today is the centre, so every view other than plains must come out pixel for pixel main's.
- In the flat path, a kind whose variant-0 cell has a `flat` colour fills with it; the others
  keep `TERRAIN_COLOR`. (Lakes and the rest are untouched.)

Read the anchor and colour once per draw, not per tile. Nothing else in the page changes.

## 2. The look

Starting from the easel's cut, make the plains read as the grass of `ui.png` (the approved
target in `docs/proposals/map-art/`) while it sits beside placeholder sea, forest, hills,
desert and the rest, on the views in `spec.md`. What you judge, in the captures:

- No outline, no hill, no seam at the hex edge (the prototype left a faint dark line on the
  right edge: a pixel or two more inset there).
- The variants: three plains side by side shouldn't read as a tiled stamp, and rocks on
  every plains tile shouldn't make the land busy. Look hard at the rocks at play zoom (k 18,
  a cell about 37 px tall): there they shrink to grey blobs the size of the map's grey
  hearth and wild-grain circles, which sit on plains, and must not be mistaken for them. A variant with the rocks patched out (a patch
  from clean grass in the same hex) is one way; changing which variant is which is another.
  Only regions of `terrain.png`; every pixel from a named rectangle.
- Rivers, trails, fields, borders, clans, icons and units stay as easy to read on the new
  plains as on main's (look hardest at `small2-play`: rivers and fields sit on plains most).
- No colour jump at the zoom boundary: `small2-edge` (cells) and `small2-flat` (fill) give
  plains the same colour at a glance.

What you don't do: other terrains, transitions between terrains, colour changes to the
placeholders, the HUD. They are later pieces.

## 3. The evidence

1. `node www/assets/draw-atlas.mjs` twice: same sha256 both times; `--check` passes.
2. A main worktree (`git worktree add /tmp/main main`, `./build-web.sh` there), with this
   branch's `www/shots.mjs` copied in. Shoot every view on main and on the branch, and
   `--diff` them: cameras match for all ten.
3. The shots are the map canvas alone (no bar, no pick panel). Commit main's and the branch's PNGs for `small2-close`, `small2-play`, `small2-flat`
   and `medium8-play` under `relay/specs/439/shots/` as `<view>-main.png` and
   `<view>-branch.png` (only these eight: the shots are large).
4. `relay/specs/439/views.md`: the `--diff` table (view, changed share), the repeat-build
   sha256 lines, what you changed from the easel's cut and why, and a few lines on how each
   point under 2 reads in the captures. If a placeholder-only view (`--only` a view with no
   plains on screen, if any) or a tile away from plains changed, say why or fix it.
5. A page-level check that selection and zoom are unchanged: on the branch page
   (`node www/play.mjs "?size=small&seed=2&run=1"` or `play/live.mjs`), select a plains
   tile, a clan and zoom in and out; it behaves as on main. A line in `views.md`.

Run `engine/check.sh` once at the end. Commit as `#439 build-2: the plains tile`.
