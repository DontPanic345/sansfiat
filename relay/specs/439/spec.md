# #439 The map-art atlas, and the first plains tile

**Question:** none in the first four scores: no rule moves. This is the first piece of the
illustrated map (`docs/proposals/map-art.md`, piece 1). It asks two things, each against main:

- **Health: nothing but the art moved.** No file under `engine/core`, `engine/cli` or
  `engine/web` changes (`git diff main --stat -- engine/core engine/cli engine/web` is empty),
  so every digest, `SAVE_VERSION` (27) and every save stay main's without a sweep. Selection,
  pan and zoom are `map/view.js`, which the piece doesn't touch.
- **The map shows one illustrated tile, and the atlas build is reproducible.** On identical
  views of the retro maps (same seed, same camera, same zoom, same window), main's plains are
  the flat placeholder and the branch's are the grass cut from `terrain.png`; every other tile,
  border, river, icon and unit draws in the same place. Two builds of the atlas in one session
  give byte-identical `atlas.png` and `atlas.json`, and `check.sh` holds the checked-in atlas to
  its sources.

The control is main at the same commit the branch starts from, captured with the same tool.

## What the piece delivers

1. **The source art in the branch** (done in this step). `docs/proposals/map-art.md` and
   `docs/proposals/map-art/{terrain,actors,ui}.png` from c9ae527. They stay there, outside
   `engine/www`, so the site never ships them.
2. **The atlas build** (build-1). `node www/assets/draw-atlas.mjs` renders the placeholder
   sheet in headless Chrome exactly as now, then, in plain Node, cuts every cell a source names
   from its PNG and writes it over the placeholder at the same key and rectangle:
   - `www/assets/png.mjs`: decode (8-bit RGB and RGBA, non-interlaced, all five filters) and
     encode (RGBA, filter 0, fixed zlib level) through `node:zlib`. No npm packages.
   - `www/assets/sources.js`: the named regions. A region names its file (a repo path), an
     explicit source hex (centre, half-width, circumradius, in source px), an inset that drops
     the drawn outline, patches (a source rectangle pasted over a destination rectangle with a
     feathered edge) and a note. A cell names its atlas key, its region, its ops (`mirror`, for
     now) and its anchor.
   - `www/assets/cut.mjs`: region → cell: the source's alpha is dropped and replaced by the
     cell's own hex mask (the placeholders' hex, circumradius 74 in a 128×148 cell, antialiased
     by coverage), patches applied, the hex resampled by area to the cell, then the ops.
   - `atlas.json` keeps every key, rectangle and note it has today, and every cell gains `ax`,
     `ay` (its anchor in cell px: the point that sits on the tile's centre; the centre for every
     placeholder). A cut cell also gains `src` (its region's name) and `flat` (`#rrggbb`, the
     mean colour of its opaque pixels).
   - `node www/assets/draw-atlas.mjs --check`, without Chrome, recuts every sourced cell and
     compares it pixel for pixel with the checked-in `atlas.png`, and checks `atlas.json` holds
     every key `placeholders.js` lays out. `check.sh` runs it beside `www/check.mjs`.
3. **Identical views** (build-1). `node www/shots.mjs --out DIR` shoots a fixed list of map
   views through `play.mjs`, each at its window (below), device pixel ratio 1, with the camera
   set directly (`cx`, `cy`, `k`) on the tile the view names. A shot is the map canvas itself
   (`game.map.canvas.toDataURL()` after `drawNow()`), not a page screenshot: at the pick the
   "Choose your civ" panel covers the left 390 px of a laptop map and nearly all of a phone's.
   It's named `shots.mjs`, not `views.mjs`, because `www/play/views.mjs` is already the map
   views' play script; it sits in `www/`, not `www/play/`, so `play/all.mjs` doesn't run it in
   CI. It writes
   `views.json` (each view's url, tile, `cx`, `cy`, `k` and `game.map.stats`). It reads only
   `play.mjs` and the page, so a copy of it runs on a main worktree. `--diff A B` compares two
   output folders: the cameras must match, and it prints each view's share of changed pixels.
4. **The plains tile, judged** (build-2). The grass hex at the right of `terrain.png`'s
   bottom-left row becomes `terrain.plains.0`–`.2`, its look judged on the retro maps beside
   main; the terrain layer draws by the anchor and fills a far-zoomed plains tile with the
   cell's `flat` colour, so crossing the zoom boundary doesn't flash.

## Shape: Easel and Canvas

The tooling is settled to the byte, and a cheap model builds it; the one thing a player sees,
how the grass looks on the map beside the placeholders, is judged by eye. No `Shape:` line was
given.

- **build-1, easel:** the PNG codec, the regions, the cutter, the build and its check, the
  views tool. It ships the prototype's plains (below) so the canvas starts from a working
  tile.
- **build-2, canvas:** the plains look, the terrain layer's anchor and flat colour, the
  captures on main and the branch, the evidence.

## Settled here, by running

- **The placeholder sheet is reproducible.** `node www/assets/draw-atlas.mjs`, run twice on
  this runner, writes `atlas.png` byte for byte the checked-in one
  (`061bfeb9…e313a6`) and the same `atlas.json` (`aefb7a64…5ca0`). So the whole atlas can be
  held to identical bytes in one session, and the Chrome stage needs no change.
- **The source.** `terrain.png` is 1536×1024 RGBA with real alpha: 57% transparent, the
  sprites opaque. The bottom-left row holds three pointy-top grass hexes side by side, their
  tips at x ≈ 165, 357 and 550. The first two carry a hill; the third, x 455–648 and y 745–963
  (centre 551.5, 854; half-width 96.5; circumradius 109), is grass, tufts and a group of rocks:
  the plains tile. Its source hex is 2% wider than regular, close enough to map straight onto
  the cell's hex. Each hex has a dark outline about 3 px wide, and hex 2's hill runs about 60 px
  into hex 3's lower left (x 452–520, y 835–895).
- **A method that works** (`proto/`, run from `relay/specs/439/proto/` with
  `node proto.cjs`): drop the source alpha; paste the source rectangle at (555, 790), 72×70,
  over (452, 828) with an 8 px feathered edge, which replaces the hill with grass and a tuft;
  inset the hex by 3 px; resample the inset hex onto the 128×148 cell by 4×4 supersampling,
  alpha the hex coverage. `proto/plains.png` is the cell and `proto/plains3x.png` it at 3× on
  the placeholder plains green. The hill and the outline are gone; a faint dark line is left on
  the right edge, so the right inset wants a pixel or two more. The grass is darker and more
  olive than the placeholder `#9cbf5a`. `proto/png.cjs` is a working decoder and encoder,
  Chrome's PNGs included.
- **The views hold** (this review, `small 2` and `large 8` on main, probes not committed). The
  pick is up when `document.body.dataset.phase` leaves `pregame` (it reads `choose`), a few
  seconds after `open` at every size. The plains tile nearest (0, 0) is tile 440 on small 2
  and 1680 on large 8. Laptop map canvas 1280×759, `minK` 7.47; phone 390×786, `minK` 3.34.
  On the phone, k 6 draws cells (`stats.atlas` true) and k 5 the flat fill; laptop k 35 and
  18 are not clamped. Setting `cx`, `cy`, `k`, calling `viewChanged()` and waiting on
  `!game.map.anim && !game.map.frame` gives byte-identical page shots and canvas PNGs across
  two runs.
- **Callers.** `terrain.js` draws `terrain.<kind>.<i % 3>` through `atlas.scaled(name, H)`,
  centred by `img.width / 2` and `H / 2`, and fills a flat colour from `placeholders.js`'s
  `TERRAIN_COLOR` below 12 device px. `fields.js`, `passes.js` and `icon.js` read
  `overlay.fields`, `icon.*` and `unit.*` cells by rectangle. `play/art.mjs` wants at least 41
  cells and none blank. Nothing else reads the atlas; nothing in Rust does.

## Decisions I made

- The game's **plains** is the grassland: the piece's grassland cell is `terrain.plains.*`.
- **Every existing key stays, at its rectangle.** A cut cell replaces a placeholder's pixels in
  place; the sheet's layout is still `placeholders.js`'s `layout()`. Later pieces add keys.
- **One source region fills all three plains variants**, since `i % 3` picks among them and a
  map of one illustrated variant beside two placeholders would read as three terrains. The easel
  ships `.0` the cut, `.1` its mirror, `.2` the cut; the canvas may change the recipe (a variant
  with the rocks patched out, say), by eye, within the one region.
- **The source's alpha is replaced by the cell's hex mask.** The source hexes share edges and
  carry outlines and a neighbour's hill, so their alpha can't be the tile's. The tile fills the
  same hex as the placeholders, so it seams with the neighbours as they do today.
- **Missing pixels are patches from the same sheet**, named in `sources.js` with their
  rectangles, not painted: the build stays reproducible from the checked-in PNG.
- **The flat zoom path fills plains with the cell's mean colour** (`flat` in `atlas.json`),
  not the placeholder green, so a tile doesn't change colour when it crosses 12 px. Other
  terrains keep `TERRAIN_COLOR`.
- **Anchors are recorded for every cell** (`ax`, `ay`) and the terrain layer draws by them.
  For every cell today they are the centre, so nothing moves; mountains (piece 2) will need a
  ground point that isn't.
- **The source PNGs stay in `docs/proposals/map-art/`**, where the owner put them and where
  they can change by ordinary commits; `sources.js` names them by repo path. Nothing under
  `engine/www` loads them, so the site doesn't serve them at runtime.
- **The tooling is plain Node**: no npm packages, no Python (the runner has neither Pillow
  nor numpy). The Chrome stage is kept for the placeholders only.
- **`check.sh` holds the cut cells, not the whole PNG**, and compares decoded pixels, not
  bytes: zlib's bytes can differ between Node versions, the pixels can't. The whole file's
  byte-for-byte repeat is the review's run.
- **A shot is the map canvas, not the page**: the pick panel would hide the map, and the
  piece changes only the map. The PR's captures are the map alone.
- **The evidence is a tool, not a one-off**: `www/shots.mjs` serves pieces 2–8 too, which
  each compare identical views.

## Unsettled, and which step holds it

- **How the plains tile looks** (build-2, the canvas): the inset, the patch, the variants, how
  the art's colour sits beside the placeholder terrains, whether the rocks (grey blobs at k 18, like the map's grey hearth circles) on every plains
  tile hide rivers, trails and fields. Proof: the captures of the views below, main beside the
  branch, read by eye, and the changed-pixel shares from `--diff`, in
  `relay/specs/439/views.md`.

## Ideas taken

None: no open idea is about the map's art.

## The views

The list `www/shots.mjs` shoots, each at the pick (`?size=S&seed=N&run=1`), with the camera
on the plains tile nearest world (0, 0), ties to the lower index. The window is `play.mjs`'s
laptop (1280×800) unless the view says phone (390×844): at laptop size the least zoom is
k 7.47 (`MapView.minK`, measured), so the flat fill (a cell under 12 device px, k below about
5.8) is only ever drawn on a phone, where `minK` is 3.34 and k 5 draws it (measured). The tool reads `game.map.k` back and fails a view whose k was clamped.

| View | Map | Window | k |
|---|---|---|---|
| `small2-close` | small 2 | laptop | 35 (the most zoom) |
| `small2-play` | small 2 | laptop | 18 |
| `small2-edge` | small 2 | phone | 6 (the last zoom that draws cells) |
| `small2-flat` | small 2 | phone | 5 (the flat fill) |
| `<size><seed>-play` | small 3, 6, 9; medium 8, 9; large 8 | laptop | 18 |

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty. `SAVE_VERSION` 27.
- Run `node www/assets/draw-atlas.mjs` twice (Chrome) from `engine/`: `sha256sum
  www/assets/atlas.png www/assets/atlas.json` the same both times and the same as committed.
  `node www/assets/draw-atlas.mjs --check` passes, and fails after a byte of a sourced cell's
  rectangle in a scratch copy is changed (or a patch in `sources.js` is nudged).
- `atlas.json` has every key main's has, each with main's `x`, `y`, `w`, `h`.
- `git worktree add /tmp/main main`, build its wasm, copy `www/shots.mjs` into it, and shoot
  `small2-play` and one other view on both (`--out` there needs only `shots.mjs`); run
  `--diff` from the branch: the cameras match. Look at the
  pairs: only plains changed, and rivers, borders, fields, icons and units sit where they did.
  Shoot `small2-edge` and `small2-flat` on the branch (phone): plains are the same colour in both.
- On the branch page, select a tile and a unit, pan and zoom: they behave as on main.
- `docs/proposals/map-art/*.png` are not under `engine/`, and `engine/www` names them only in
  `sources.js`.
- The PR carries main's and the branch's captures of `small2-close` and `small2-play` and one
  medium or large view, from the files build-2 commits under `relay/specs/439/shots/`.
