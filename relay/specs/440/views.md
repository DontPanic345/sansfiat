# #440 build-2 (canvas): the ranges, on the views

Main is `55ed47b` (the branch's base; no engine change before this step), built in `/tmp/main` with this branch's
`www/shots.mjs` copied in. Shots one run each, on one machine.

## Atlas

`node engine/www/assets/draw-atlas.mjs` run twice, the same both times and as committed:

```
atlas.png   54c95775f4ad4e397eef3c758509cb9a793ba19fbd230d62173d7645a10ee7c8
atlas.json  1af6f7228d029a6e10401befb504c6e9c32b4e085b64a945c44f8812b1186155
```

`--check`: `atlas: 17 cut cells match their sources, 52 cells match layout()`. The sheet is 1024×1300.

## `--diff`, main against the branch

Every camera matches (exit 0). The share of pixels that differ is the mountains (the plains views' 33-64 px shift
is build-1's, `easel.md`).

| view | differ |
|---|---|
| small2-close | 14.765% |
| small2-play | 6.781% |
| small2-edge | 2.585% |
| small2-flat | 1.445% |
| small3-play | 4.63% |
| small6-play | 5.731% |
| small9-play | 7.692% |
| medium8-play | 11.041% |
| medium9-play | 12.001% |
| large8-play | 12.694% |
| small9-fork-close | 15.012% |
| small9-fork-play | 7.144% |
| small2-bend-close | 11.863% |
| small2-bend-play | 6.033% |
| medium9-blob-play | 12.017% |
| large8-bend-play | 11.475% |
| large8-fork-play | 15.095% |
| small9-edge-bottom | 3.993% |
| small9-edge-right | 16.906% |
| small9-fork-edge | 3.01% |
| small9-fork-flat | 1.644% |

## `--cull`

Exit 0: every mountain view is identical culled and unculled.

```
small9-fork-close  tile 469  stats.ridges 76 culled, 133 unculled  identical
small9-fork-play  tile 469  stats.ridges 125 culled, 133 unculled  identical
small2-bend-close  tile 377  stats.ridges 53 culled, 111 unculled  identical
small2-bend-play  tile 377  stats.ridges 98 culled, 111 unculled  identical
medium9-blob-play  tile 777  stats.ridges 229 culled, 293 unculled  identical
large8-bend-play  tile 1937  stats.ridges 211 culled, 574 unculled  identical
large8-fork-play  tile 1953  stats.ridges 312 culled, 574 unculled  identical
small9-edge-bottom  tile 474  stats.ridges 73 culled, 133 unculled  identical
small9-edge-right  tile 474  stats.ridges 88 culled, 133 unculled  identical
small9-fork-edge  tile 469  stats.ridges 133 culled, 133 unculled  identical
small9-fork-flat  tile 469  stats.ridges 0 culled, 0 unculled  identical
```

## `stats.terrainMs`, main beside the branch

Single runs: noisy at this size (the swing between runs is as big as the gap). The branch's static draw stays within a
few ms of main's.

| view | main | branch |
|---|---|---|
| small2-edge | 2.1 | 2.9 |
| small9-fork-close | 6.1 | 6.9 |
| small9-fork-play | 5.9 | 5.9 |
| small2-bend-close | 7.3 | 5.1 |
| small2-bend-play | 4.7 | 5.2 |
| medium9-blob-play | 7.8 | 9.1 |
| large8-bend-play | 11.2 | 11.8 |
| large8-fork-play | 11.1 | 11.3 |
| small9-edge-bottom | 3.6 | 3.6 |
| small9-edge-right | 3.7 | 6.4 |
| small9-fork-edge | 2.1 | 4.6 |
| small9-fork-flat | 1.3 | 1.1 |

## What changed from the easel, and why

- **The mountain cells are 224×144, anchored at (224/2, 128)** (`placeholders.js` `MOUNTAIN`), not 192×128 at (96, 112):
  the peaks are drawn a little bigger than the plains' scale (the target's are), and the left peak at 0.74 cell px per
  source px is 203 px wide and 109 tall, which does not fit 192×128. Four a row, three rows; every older key keeps its rectangle.
- **The scales are per region** (`sources.js`): the left peak 0.74, the right 0.72, the front 0.70, the low peaks
  0.70 and 0.66 for the east ridges, 0.52 for the diagonal ones. A peak's tip is 1.4 circumradii above its ground
  point, so it reaches the middle of the tile north of it and no further.
- **The art.** `mountain.peak.0` is the range's left peak (as build-1 cut it), `.1` the right peak (x 1130-1385, its
  neighbour's flank masked by the keep-polygon), `.2` the front peak (x 745-1050, its tip trimmed so the big peak behind it
  does not show). `mountain.lone.0` is the left peak (its boulders and low spurs are the foothills), `.1` the right peak mirrored.
  `mountain.ridge.e.0` is the low peak at x 540-700, `.e.1` the lower peak at x 590-850, both cut to a triangle
  (a keep-polygon on the peak's own flanks) so a ridge has a peak's outline, not a rectangle's. `.se.0` is the lower peak
  and `.se.1` the low peak, each small and mirrored for `.sw.*`, so `sw` is `se` mirrored. A ridge is drawn behind the peaks
  it joins and shows as the saddle between them.
- **The ground key** (`cut.mjs` `KEYS.ground.tuftBlue`) is 0.72, was 0.6: pale yellow tuft pixels (b about 0.65 g)
  remained at the peaks' feet and are cleared now; the rock's browns (b about 0.8 g and up) are not.
- **No new keys and no change to `ridgePieces`**: the CI checks are untouched. Caps and forks are what the composition
  makes, and they read (below).

## How each point reads

- **Ridges.** In `small9-fork-close` and `small2-bend-play` a straight run reads as one chain: the low peak between two big
  ones fills the notch and breaks the line of the skyline, so the run is a range, not three stamps. The diagonal ridges
  show as a spur between a peak's foot and the next peak down. At k 35 the grass shows between peaks at their feet; that is the
  tile's own ground and sits as the target's does.
- **Peaks and lone peaks.** Three different peaks over `i % 3` break the stamp; they are the same family, and at play zoom
  two neighbours rarely look alike. A lone peak (the left peak with its boulders) has foothills at its foot and reads
  as a peak, not a cut range.
- **Caps and forks.** A cap is a peak with one low ridge and ends the run without a blunt edge; a fork is a knot of three
  or four overlapping peaks (`small9-fork-close`, right of centre), dense but still a mass of one grey, which is what a
  fork is. `medium9-blob-play`'s 4+ tile reads as a clump.
- **Depth and overhang.** Southern pieces draw over northern ones with no seam (they are all alpha-edged rock, and the overlap
  is of rock on rock). A peak rises into the tile north of it, to its middle; rivers, borders, icons and units draw over it
  and read on the grey and the snow (the red border, the clan dots and the river in `small9-fork-close` all stay clear).
- **Beside the rest.** The grey and the white sit with the grass tiles; against the placeholder desert tan and hills brown they
  stand out as mountains should. A pass (hills) between two caps shows as a plain hills hex between two ends.
- **Zoom.** `small9-fork-edge` (k 6) reads the ranges as grey chains of peaks; `small9-fork-flat` (k 5) as grey lines on the
  flat fill, as on main but with the range's grey.
- **The edges.** `small9-edge-bottom` (k 18) and `small9-edge-right` (k 35) show the peaks reaching onto the canvas from
  tiles off it; `--cull` proves culled and unculled the same.

## The page

On the branch and on main, `?size=small&seed=9&run=1` after leading the first civ (script in `/tmp`, not committed):
a click selects the mountain tile 469 (`tile:469`), the tile north of it 468 (`tile:468`: the click picks its tile where a
peak overhangs it) and the unit on 472 (`unit:334`); a drag pans by the same amount (cx 0 to 0.87, cy -6 to -4.5) and
zoom to k 30 and 8 changes only k. Identical on both, no page errors.
