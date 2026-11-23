# #441 build-3 (canvas): the shores and grounds, judged beside main

Main is `f5f4620` in a worktree (`/tmp/main`), its wasm built, with this branch's `shots.mjs` copied in. All 33 views
shot on both; the branch is shot at the final atlas. The captures of the six named views are in `shots/` as
`<view>-main.png` and `<view>-branch.png`.

## `--diff`: main against the branch

Exit 0: the cameras match on all 33 views. The shares are large because every ground is art now, so they measure the
new ground, not the shore alone.

| View | Share of pixels differing |
|---|---|
| small2-close | 67.618% |
| small2-play | 48.793% |
| small2-edge | 19.573% |
| small2-flat | 12.959% |
| small3-play | 55.998% |
| small6-play | 50.788% |
| small9-play | 52.003% |
| medium8-play | 75.162% |
| medium9-play | 76.078% |
| large8-play | 66.871% |
| small9-fork-close | 79.194% |
| small9-fork-play | 49.533% |
| small2-bend-close | 73.12% |
| small2-bend-play | 49.508% |
| medium9-blob-play | 75.771% |
| large8-bend-play | 68.253% |
| large8-fork-play | 67.875% |
| small9-edge-bottom | 31.128% |
| small9-edge-right | 60.426% |
| small9-fork-edge | 20.665% |
| small9-fork-flat | 13.911% |
| small2-neck-close | 59.271% |
| small2-delta-close | 72.65% |
| small2-delta-play | 49.485% |
| small6-bay-close | 69.108% |
| small9-pond-close | 75.548% |
| small9-lake-play | 51.793% |
| medium8-strait-close | 82.309% |
| medium9-neck-play | 77.53% |
| large8-delta-play | 58.068% |
| large8-pond-play | 69.208% |
| small2-neck-edge | 19.527% |
| small2-neck-flat | 12.991% |

## The atlas, twice

`node www/assets/draw-atlas.mjs` run twice, the same bytes both times; `--check` passes
(`atlas: 134 cut cells match their sources, 148 cells match layout()`).

```
9f6916c9ffc03169fdecfaebaf807c3e445ff1f7a12874419a526944811eed28  www/assets/atlas.png
cc3d3a36a2a12b0d0c54d318e38dc325e145d81914eb5126475511f2c4fe9a3c  www/assets/atlas.json
```

## `--cull`

Exit 0: culled and unculled draws are byte-identical on all 23 views with an `on`.

```
small9-fork-close  tile 469  stats.ridges 76 culled, 133 unculled  stats.edges 784 culled, 1774 unculled  identical
small9-fork-play  tile 469  stats.ridges 125 culled, 133 unculled  stats.edges 1504 culled, 1774 unculled  identical
small2-bend-close  tile 377  stats.ridges 53 culled, 111 unculled  stats.edges 767 culled, 1801 unculled  identical
small2-bend-play  tile 377  stats.ridges 98 culled, 111 unculled  stats.edges 1616 culled, 1801 unculled  identical
medium9-blob-play  tile 777  stats.ridges 229 culled, 293 unculled  stats.edges 2356 culled, 3632 unculled  identical
large8-bend-play  tile 1937  stats.ridges 211 culled, 574 unculled  stats.edges 2544 culled, 6982 unculled  identical
large8-fork-play  tile 1953  stats.ridges 312 culled, 574 unculled  stats.edges 2345 culled, 6982 unculled  identical
small9-edge-bottom  tile 474  stats.ridges 73 culled, 133 unculled  stats.edges 948 culled, 1774 unculled  identical
small9-edge-right  tile 474  stats.ridges 88 culled, 133 unculled  stats.edges 566 culled, 1774 unculled  identical
small9-fork-edge  tile 469  stats.ridges 133 culled, 133 unculled  stats.edges 1774 culled, 1774 unculled  identical
small9-fork-flat  tile 469  stats.ridges 0 culled, 0 unculled  stats.edges 0 culled, 0 unculled  identical
small2-neck-close  tile 402  stats.ridges 85 culled, 111 unculled  stats.edges 711 culled, 1801 unculled  identical
small2-delta-close  tile 506  stats.ridges 52 culled, 111 unculled  stats.edges 703 culled, 1801 unculled  identical
small2-delta-play  tile 506  stats.ridges 98 culled, 111 unculled  stats.edges 1616 culled, 1801 unculled  identical
small6-bay-close  tile 376  stats.ridges 59 culled, 91 unculled  stats.edges 722 culled, 1723 unculled  identical
small9-pond-close  tile 341  stats.ridges 91 culled, 133 unculled  stats.edges 774 culled, 1774 unculled  identical
small9-lake-play  tile 377  stats.ridges 133 culled, 133 unculled  stats.edges 1625 culled, 1774 unculled  identical
medium8-strait-close  tile 875  stats.ridges 52 culled, 231 unculled  stats.edges 799 culled, 3515 unculled  identical
medium9-neck-play  tile 1004  stats.ridges 214 culled, 293 unculled  stats.edges 2347 culled, 3632 unculled  identical
large8-delta-play  tile 2639  stats.ridges 235 culled, 574 unculled  stats.edges 1933 culled, 6982 unculled  identical
large8-pond-play  tile 1289  stats.ridges 139 culled, 574 unculled  stats.edges 2443 culled, 6982 unculled  identical
small2-neck-edge  tile 402  stats.ridges 111 culled, 111 unculled  stats.edges 1801 culled, 1801 unculled  identical
small2-neck-flat  tile 402  stats.ridges 0 culled, 0 unculled  stats.edges 0 culled, 0 unculled  identical
```

## `stats.terrainMs`: the static draw

Medians of nine redraws of the same camera (the views' cameras from `views.json`, laptop window), main against the branch.

| View | Main ms | Branch ms |
|---|---|---|
| small2-play | 4.3 | 6.4 |
| medium9-play | 9.6 | 10.2 |
| large8-play | 12.0 | 18.3 |
| small9-lake-play | 4.6 | 5.9 |
| medium9-neck-play | 8.1 | 10.3 |
| large8-delta-play | 6.8 | 8.5 |

Within 1 to 3 ms of main on every view but `large8-play` (+6 ms): build-2's known miss, two draws a tile there (the
bank and fringe pieces beside the cell). This step adds no piece, so it stays; the review may take the per-tile
composite.

## The nine flat colours

The flat fill's colour is each kind's cell `flat` (the mountain's is its peak's). No pair is closer than 28 in RGB; the
closest is **hills–desert, 35.1**. Hills' flat is the mound's lit ochre; desert's the bank's lit sand; coast's the bank's
shaded sand; forest's the trees' dark floor; marsh, the cell's own mean.

| Kind | Flat | RGB |
|---|---|---|
| sea | #013d66 | 1, 61, 102 |
| lake | #206793 | 32, 103, 147 |
| coast | #978550 | 151, 133, 80 |
| plains | #7b882d | 123, 136, 45 |
| forest | #2d4735 | 45, 71, 53 |
| hills | #b39f47 | 179, 159, 71 |
| mountain | #777370 | 119, 115, 112 |
| desert | #c59c65 | 197, 156, 101 |
| marsh | #57672e | 87, 103, 46 |

Six nearest pairs: hills–desert 35.1, coast–hills 39.3, coast–plains 44.9, coast–mountain 48.7, plains–marsh 48.8,
forest–marsh 53.3. Build-1's five close pairs (forest–marsh 13.5, desert–marsh 16.9, coast–hills 22.5, forest–desert
23.9, plains–marsh 26.2) are gone: the cells' recolours were re-aimed at sheet colours that are far apart (below), and a
cell's flat is a rectangle of a ground's own colour wherever the cell's mean sits near another kind's.

## What changed from the easels, and why

- **The bank is built, not cut flat and turned.** Edge pieces stop at the edge's two vertices (`shape: edge`, half
  `HEX_R / 2 + 0.5`), so a piece never reaches past its edge onto the next tile (the first cut ran 7 px past each vertex). The
  `bank` scale is 0.95 (was 74/109 ≈ 0.68): the bank is about 4 px thick at k 18 and 8 px at k 35, it was 3 and 5.
- **`in` and `out` are their own cells** (`shape: in`, `shape: out` in `cut.mjs`'s `spriteData`). `in` (water pokes
  into land) fans the run round the vertex between the two edges' outward normals, depth the radius and arc length the
  angle, so the blocks follow the turn; it fills the wedge the two strips leave at the bend and stays on the land side.
  `out` (land pokes into water) mitres the two edges' strips on the bisector, each serving the half nearer its edge, so
  a cape's tip is one bank. The easel's one bank disc, which lumped half onto the water, is gone (`bank.disc`).
- **Water is three clean blocks without the foam arc.** The first fill's block (1270, 832, 137×60) held a foam arc at its
  left edge, which showed as a pale seam down the left of every hex, and a speck pair that the mirror turned into a diamond
  repeated per hex. Variants are now open water with faint streaks (1296, 814, 70×28), a block with one speck pair
  (1312, 852, 70×40) and a clean block (1304, 846, 74×26), at scale 1.0 with different offsets, so a bay has no repeated
  lattice and a third of the hexes carry sparkle.
- **Lake is the sea's cells ramped at a fixed luminance range** (`lum: [45, 110]` on the ramp op): the cell's own range
  made the clean variant lighter than the other two. Lake is still a ramp from a window of the sea's lighter water to a
  highlight of it.
- **Grounds recoloured onto colours far apart in the sheet** (`RAMP`: the trees' floor, dark and leaf, the bank's shaded
  and lit sand): coast is grass at mix 0.8 toward the sand, a pale warm band; forest is the grass at mix 0.95 on the trees'
  dark teal to leaf green; desert is the grass on the bank's shaded to lit sand at mix 1; marsh is the grass with four
  pools of the sea (two more than the easel) murked to the trees' dark teal at mix 0.6. Hills are unchanged but their
  flat is the lit ochre (170, 845, 40, 10).
- **The desert stipple no longer draws** (`terrain.js` drops `overlay.desert-dots`; the cell stays in the atlas). The
  sand ground reads as desert on its own, the dots were placeholder strokes (`#b39a55` squares) that the sheet does not
  hold, and they drew over the new sand as noise.
- **Not changed:** `FRINGE`, `DEPTH`/`AMP`/`FEATHER`/`TAPER`, the fringe order and the piece rules; there are no corner
  pieces for land fringes, since no notch shows where two fringes of one kind meet at a vertex (the forest and desert
  blobs read as one shape at k 18 and k 35). The hex outline over every tile at k 14 and up (`terrain.js`, main's) is
  left as main has it, so water keeps its faint hex lines.

## How each point reads in the captures

- **Water.** Sea is deep blue with sparse light streaks and specks, no wallpaper lattice; lake is clearly lighter (flat
  #206793 against #013d66) and both are far from the coast's pale olive. Seen in `small6-bay-close`, `small9-lake-play`.
- **The bank.** One continuous line of tan blocks about 4 px thick at k 18 and 8 px at k 35, on the land side of every shore, with no
  gap at the twelve corners. The `out` corner at a cape and a neck is a clean mitre; the `in` corner in a bay or round a
  pond is a smooth turn (`small6-bay-close`: the island at the lower left and the bay's rim).
- **Necks, straits, bays, ponds, mouths.** `small2-neck-close` and `medium9-neck-play`: the neck is land with a bank
  down both sides; `medium8-strait-close`: the strait is open water between two banks; `small6-bay-close` and
  `small9-pond-close`: the water is held all round; `small2-delta-close` and `small9-lake-play`: the river's line runs
  through the gap where the bank is left out, the corners either side frame it.
- **The grounds.** Coast is a pale warm olive band beside the water, land and not sand; forest is a dark green ground that
  reads as wood without trees; hills are the source's mounds on grass; desert is flat tan sand; marsh is murky teal-olive
  with pools; mountain ground is grass under the peaks. No ground looks pasted beside another: the fringes cover the hex
  outline where kinds meet.
- **Fringes.** Desert, forest and coast bleed into neighbouring plains in a ragged edge of 16 px or so, soft where the
  mask feathers, neither a hex outline nor a smear (`small2-delta-close`, right).
- **Over it.** Rivers, floodplains, passes, borders, fields, clans, units and icons read on every ground and on the bank;
  the red and blue borders and the white rings sit on forest, desert and water as they do on main.
- **Zoom.** `small2-neck-edge` (k 6, cells) and `small2-neck-flat` (k 5, the flat fill) tell sea, lake, coast, plains,
  forest, hills, mountain, desert and marsh apart; so does `small2-flat`. The flat zoom shows hills and desert as warm
  tans against the plains' olive, forest as the darkest land and coast as the pale ring round the shore.

## The branch page

`play/live.mjs` on the branch and on main (`?size=small&seed=2&run=1`), the same commands on each: lead Red, click the
coast tile 402 (selects clan 176's home), click the sea tile 5 (panel "ocean"), click (3, 2) (Red's scout), drag 402 to
380, zoom in (`Equal`), tip 402. The panels and tooltips are identical on both after the first command (the branch
session had already led Red), so selection, pan and zoom behave as on main.
