# #439 build-1 (easel): evidence

## The build is reproducible

Two runs of `node www/assets/draw-atlas.mjs` in `engine/` on this runner:

```
714d1872d09678f193036eb03be3c325aebdebc47bb9ca53a3f3dc004ee0254c  www/assets/atlas.png
ca78d70bc8ec0b8a9e5f1e2dd6eb56302a02bbe083f87899446e788415ef08e6  www/assets/atlas.json
```

Both runs gave the same hashes. `node www/assets/draw-atlas.mjs --check` passes.

## With `CELLS` empty, the build is main's sheet

With `CELLS` emptied in a scratch edit (reverted after), the build's `atlas.png` decodes to
main's pixels exactly (the bytes differ: Node encodes now). Main's `atlas.png` is
`061bfeb9…e313a6`, its `atlas.json` `aefb7a64…5ca0`. The empty build's `atlas.json` has the same
keys, `x`, `y`, `w`, `h` and `note` as main's, plus `ax`, `ay` (the centre) on every cell.

On the cut build, every placeholder cell's pixels are main's; only the three plains cells differ.

## The plains cut against the prototype

`terrain.plains.0` against `relay/specs/439/proto/plains.png`, over the opaque pixels in both
(14,056 pixels): largest channel difference 18, with 365 channel values over 3. The differences
sit where the patch is feathered (the new feather is on all four edges, where the prototype
left the left edge sharp) and where the hex's half-width differs (the prototype's coverage is
64 px, the placeholders' hex is 64.09). Partly-covered edge pixels differ by more, as alpha does
(largest alpha difference 16). Mean channel difference over the hex: 0.5.

`terrain.plains.1` is `terrain.plains.0` mirrored, pixel for pixel.

## The views tool

`node www/shots.mjs --out DIR` on the branch, twice, then `--diff`:

```
small2-close  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
small2-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
small2-edge  0% differ  (0 of 306540 pixels; 390×786 vs 390×786)
small2-flat  0% differ  (0 of 306540 pixels; 390×786 vs 390×786)
small3-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
small6-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
small9-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
medium8-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
medium9-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
large8-play  0% differ  (0 of 971520 pixels; 1280×759 vs 1280×759)
```

Main, at `0fd7835` in a worktree (`/tmp/main`, its wasm built from main's `engine/`), shot
with the same `shots.mjs` and compared from the branch with `--diff`. The cameras match on all
ten views (tile, `cx`, `cy`, `k`; tile 440 on small 2, 1680 on large 8):

```
small2-close  19.204% differ  (186572 of 971520 pixels; 1280×759 vs 1280×759)
small2-play  8.784% differ  (85337 of 971520 pixels; 1280×759 vs 1280×759)
small2-edge  3.16% differ  (9688 of 306540 pixels; 390×786 vs 390×786)
small2-flat  0% differ  (0 of 306540 pixels; 390×786 vs 390×786)
small3-play  3.086% differ  (29981 of 971520 pixels; 1280×759 vs 1280×759)
small6-play  7.706% differ  (74866 of 971520 pixels; 1280×759 vs 1280×759)
small9-play  4.915% differ  (47747 of 971520 pixels; 1280×759 vs 1280×759)
medium8-play  10.644% differ  (103412 of 971520 pixels; 1280×759 vs 1280×759)
medium9-play  5.169% differ  (50216 of 971520 pixels; 1280×759 vs 1280×759)
large8-play  25.552% differ  (248242 of 971520 pixels; 1280×759 vs 1280×759)
```

`small2-flat` is unchanged: the flat fill still uses `TERRAIN_COLOR` until build-2 draws plains
with the cell's `flat` colour. The others differ where the plains are drawn; build-2 checks
that nothing else moved.

`views.json` carries `game.map.stats` verbatim, so its `terrainMs` and `overlayMs` are timings
and differ between runs; the PNGs are what repeat byte for byte.

## Corrections to the brief

- The patches are applied to a copy of the region's box, and the box is the hex's bounding box
  widened to hold each patch's source and destination rectangles. The brief's box is too small
  for `terrain.plains`: the patch's destination starts at x 452, left of the hex's box (x 455).
- A patch reads its source from the copy as the patches before it left it, so the order is
  defined where rectangles overlap.
