# #440 build-1 (easel): the evidence

The machinery of `build-1.md`, checked. Main is the branch's base (`ae506b1`, no engine change before this step),
shot with the branch's `shots.mjs` copied into a worktree of it.

## Atlas

- `node engine/www/assets/draw-atlas.mjs` run twice: `atlas.png` `168bc5b1…c7738`, `atlas.json` `0d987c45…5585e`,
  the same both times and as committed. `--check` passes: 11 cut cells match their sources, 52 cells match `layout()`.
- In a scratch copy, one byte of `mountain.peak.0` changed: `--check` exits 1 (`1 pixels differ from the cut; first at (96, 60)`).
- The sheet is 1024×1252 (main's was 1024×868): 11 new cells, 5 a row, after the icons.
- `mountain.peak.0`, cut from `peak.left`, equals `relay/specs/440/proto/peak.png` byte for byte (the cell's RGBA),
  and so does its cell in `atlas.png`. Anchor 96, 112; flat `#77736f`.

## Plains views: not byte-identical (Done 2 not met)

With the mountain `CELLS` entries and the ridge layer removed in a scratch copy (sources, view.js), the ten plains
views were shot and compared with main's:

| view | vs main |
|---|---|
| small2-edge, small2-flat | identical |
| small2-close | 64 px differ, max 20 levels |
| small2-play | 49 px, max 54 |
| small3-play | 49 px, max 54 |
| small6-play | 50 px, max 54 |
| small9-play | 50 px, max 58 |
| medium8-play | 33 px, max 56 |
| medium9-play | 34 px, max 54 |
| large8-play | 33 px, max 54 |

Traced as far as this step went. Every cell of main's atlas is pixel-identical in the new sheet (all 41 keys, same
`x`, `y`, `w`, `h`, `ax`, `ay`, `flat`), and the terrain layer and the view are unchanged for plains. What differs is
the atlas *image's height*: with main's 868-row PNG and the new `atlas.json`, the view matches main byte for byte;
with the same pixels padded with transparent rows to 1268, it doesn't. So Chrome samples the grown image a little
differently at a few pixels. I did not find which icons or edges. No placement that keeps the old height is
possible: the new cells need 384 px below the icon row, and a gap under the icons didn't change the result
(tried, reverted). Build-2 and the review should know the plains views are main's to within 64 pixels of a
1280×759 canvas, not to the byte.

## Views: cameras match, `--diff` (main | branch)

`node www/shots.mjs --diff` on the two folders: every camera matched (exit 0). The share of differing pixels, which
is the ridges, not a bug (the plains rows above are the only pixels that move without a ridge):

| view | differ |
|---|---|
| small2-close | 13.48% |
| small2-play | 6.24% |
| small2-edge | 2.22% |
| small2-flat | 1.45% |
| small3-play | 4.19% |
| small6-play | 5.26% |
| small9-play | 7.09% |
| medium8-play | 9.98% |
| medium9-play | 11.05% |
| large8-play | 11.48% |
| small9-fork-close | 13.79% |
| small9-fork-play | 6.59% |
| small2-bend-close | 10.74% |
| small2-bend-play | 5.53% |
| medium9-blob-play | 11.00% |
| large8-bend-play | 10.44% |
| large8-fork-play | 13.79% |
| small9-edge-bottom | 3.70% |
| small9-edge-right | 15.56% |
| small9-fork-edge | 2.58% |
| small9-fork-flat | 1.65% |

Tiles: 469, 377, 777, 1937, 1953, 474 as `ranges.md` chose them (the shots throw otherwise). Edge cameras:
`small9-edge-bottom` at k 18 (cy −20.58 unclamped), `small9-edge-right` at k 35.

## Culling: `--cull`

`node www/shots.mjs --cull DIR`: each mountain view in one page, culled, then `noCull = true; dirty = true`,
unculled. All eleven identical:

| view | stats.ridges culled | unculled |
|---|---|---|
| small9-fork-close | 76 | 133 |
| small9-fork-play | 125 | 133 |
| small2-bend-close | 53 | 111 |
| small2-bend-play | 98 | 111 |
| medium9-blob-play | 229 | 293 |
| large8-bend-play | 211 | 574 |
| large8-fork-play | 312 | 574 |
| small9-edge-bottom | 73 | 133 |
| small9-edge-right | 88 | 133 |
| small9-fork-edge | 133 | 133 |
| small9-fork-flat | 0 | 0 (flat fill: no pieces) |

## `stats.terrainMs`, main beside the branch

One run each, on the laptop play views, with other shots running on the same machine: single timings, noisy.
A rerun on a quiet machine is worth doing before the review reads these as a cost.

| view | main | branch |
|---|---|---|
| small2-play | 17.4 | 10.9 |
| small9-play | 12.5 | 15.9 |
| small6-play | 16.5 | 20.8 |
| medium9-play | 22.8 | 26.1 |
| large8-play | 12.5 | 14.9 |

## Checks

- `node www/check.mjs`: the ridge classes over every mountain tile on the medium world match a count made from the
  neighbours in the check; a peak per mountain tile; a ridge per shared edge at its midpoint with its variant; draw order.
- `node www/play/ridges.mjs`: on a staged radius-8 map with a lone peak, a fork, a straight run with two caps, a bend
  and a lone peak far south: every class, peak and ridge key is the spec's; the draw order holds; the camera with the
  straight run's middle peak 1.0 world unit below the canvas is not clamped; `stats.ridges` counts the 18 pieces whose
  rectangle meets the canvas (of 19); the culled and unculled PNGs are identical. Output in `/tmp`, not committed.
- `node www/play/art.mjs`: `{"cells":52,"sheet":"1024x1252","terrainCells":127,"kinds":3}`, every cell inside the sheet and not blank.

## Corrections to `build-1.md`

- **Opaque pixels.** The source's range is opaque at 239–251 alpha, never 255, so the brief's "alpha-255 pixels" leaves
  no pixel for `mountain.peak.0` and the cut throws. `flat` now counts pixels at 250 alpha and up (`OPAQUE` in `cut.mjs`).
  Hex cells are unchanged: their full cover is 255, which is still the only value at or above 250 in them.
- **The plains views** are not byte-identical; see above. The cause is the atlas's height, not a cell's pixels.
- **Terrain mountain placeholder.** `terrain.mountain.*` keeps main's grey triangle in the placeholder drawing, so a
  scratch build without its cuts is main's sheet in those cells.
- **Gap under the icons.** Tried to see whether the icons' bottom edge sampled the new keys; it did not change the
  result, so the layout has no gap.
