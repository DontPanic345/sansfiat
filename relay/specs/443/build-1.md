# #443 build-1 (easel): the unit figures and their masks in the atlas

Read `relay/specs/443/spec.md` first: the inventory, "How the figures are drawn" (the cells' size and anchor), the
decisions, and "Settled here, by running" for where the study's figures are. Build-2 draws your cells on the map,
and build-3, the canvas, judges how they look and may change any recipe. Your job is cells that are exact in their
keys, sizes and anchors, and first cuts that read: a spearman with a round shield and a walker with a pack, each
with a banner, and masks that hold only the cloth. Don't change Rust: nothing under `engine/core`, `engine/cli` or
`engine/web`. Don't change map code: `www/map/` and `www/atlas.js` are build-2's.

Read these before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js,png.mjs}`,
`www/play/art.mjs`, `relay/specs/442/atlas.md` and `relay/specs/442/proto/{compare-main.mjs,cells3x.mjs,regions.mjs}`
(how 442 cut, compared with main and showed its cells). Look at `docs/proposals/map-art/actors.png` and `ui.png`.
The trees in `sources.js` are the pattern: a sprite region with a `keep` polygon, a `key`, a `ground` point and a
`scale`, cut to a cell whose anchor is the ground point.

## 1. In `www/assets/cut.mjs` and `sources.js`

Every existing cut must stay identical to the pixel. `--check` proves it on the cells already cut.

- `FILES` gains `actors: 'docs/proposals/map-art/actors.png'`.
- **An `actor` key** beside `tree`: it clears nothing, because the study's ground is transparent and the cut already
  weights by source alpha. Its note says so.
- **A two-region cell.** A cell may name, besides its `region`, an `over` list of further sprite regions, each with
  the cell point its ground lands on (`at`, cell px). Each is cut as a sprite into the cell and drawn over what is
  there (source-over, in integers, the same on any machine). The banner is cut this way onto a figure.
- **A cloth mask.** A cell may name `cloth`: for each of its regions, polygons in that region's source px. A sample
  counts only when it is inside one of its region's cloth polygons. A mask cell is its figure's cell spec with
  `cloth` and the grey op, so it overlays its figure exactly.
- **A `grey` op.** Each pixel becomes its luminance, normalised over the cell's opaque pixels to a range
  (`[lo, 255]`, start `lo` at 90), alpha kept. White stays white, so a multiply by a colour gives that colour.
- Add an assert to `asserts()` for each: an `over` with no regions is the plain cut; a `cloth` polygon round the
  whole rectangle is the plain cut; `grey` on a white cell is white.

## 2. The cells

In `placeholders.js`'s `layout()`, the unit cells `unit.camp`, `unit.camp.mask`, `unit.scout` and `unit.scout.mask`
become 96×96 with anchor (48, 92), in their row. Their notes say what they are now (a figure, standing, its feet at
the anchor; a mask in grey, tinted by multiply and drawn over the figure). Their placeholder drawings must still
draw something at the new size, as `play/art.mjs` requires every cell to be non-blank. Every other key keeps its
size, and its pixels; its rectangle may move down with the row.

In `sources.js`, regions from the study, each a box with a margin of 3 px as its keep-polygon (each figure stands
alone in its box), and its ground point at the feet:

- `actor.camp`: the round-shield spearman, box about (583, 72) 129×187;
- `actor.scout`: the walker, box about (193, 107) 111×149;
- `actor.flag`: the flag, box about (1360, 100) 104×157, its ground point where it would sit on a staff top (the
  pole's top, so the flag hangs from the point).

Cells: `unit.camp` is `actor.camp` with `actor.flag` over it, its banner on the spear's shaft below the head;
`unit.scout` is `actor.scout` with the flag on its staff's top, as `ui.png`'s scout. Scale each figure so it stands
about 88 px tall in the 96 px cell; the flag about a third of the figure's height. `unit.<kind>.mask` is the same
spec with `cloth` round the flag's cloth (not its pole) and, for the camp, the round shield's face, and the `grey` op.

## 3. Checks and evidence

- `node www/assets/draw-atlas.mjs` twice gives the same bytes; `--check` passes, with the new asserts.
- Every key of main's `atlas.json` is still there, and every one but the four unit cells has main's pixels (adapt
  `relay/specs/442/proto/compare-main.mjs` into `relay/specs/443/proto/compare-main.mjs`: pixels by key, not
  rectangles). `node www/play/art.mjs` passes.
- `relay/specs/443/cells.png`: the four cells at 3×, on mid grey and on a forest-green patch, each mask beside its
  figure, and the figure with its mask multiplied in red, blue and gold (`#d64545`, `#3b6fd6`, `#d6a63b`) drawn
  over it. Its proto under `relay/specs/443/proto/`.
- `relay/specs/443/atlas.md`: the sha256 of `atlas.png` and `atlas.json`, the `--check` line, the compare line, each
  cell's regions, scale, anchor and cloth, and the share of each figure's opaque pixels its mask covers.
- `engine/CLAUDE.md`'s `assets/` row names the `actor` key, `over`, `cloth` and `grey`, and the unit cells' size and
  anchor, within its 8 KB.

Run `engine/check.sh` whole once before you finish.

Build-2 relies on: the four keys at 96×96 with anchor (48, 92); each mask overlaying its figure; a mask that is
transparent outside the cloth and grey inside it, so multiply gives the civ's colour shaded.
