# #443 build-2: the unit views, the probe and the checks

## Builds and checks

- `engine/check.sh`, whole, once: exit 0. Every touch-point script passes, `units.mjs` among them (1.8 s).
- `node www/play/units.mjs`: every `need` passes. Six figures on their tiles (gold's stack on 126); every unit's
  probe nearest its own civ with tinted pixels (tinted 75, 76, 76, 348, 344, 351 at k 35); a click on 126 selects
  one unit (`unit:15`) and the next click the other (`unit:18`), and the marked figure follows; the two numbers
  "2.0" and "4.0" have boxes that don't overlap (2 px apart).
- `node www/shots.mjs --out DIR --only …` on the branch: all 13 views set their cameras (the asked camera read
  back unclamped). The digests are below.
- `node www/shots.mjs --units DIR` on the branch: prints each unit's probe (the log is `easel-units.txt`). 12 of the 13
  views have every unit nearest its own civ. **`medium9-units-flat` does not**, and the run exits 1 (see Finding).
- `--units` on the main worktree (the branch's `shots.mjs` and `www/pkg`): every view prints `-` and the run exits 0.
  Its cameras and digests match the branch's for all 13 views (table below).

## Views

Each view's `digest` is `game.digest()` after its setup. Main's column is the digest of the same view on the main
worktree (the branch's `shots.mjs`).

| View | Window | k | Camera | Digest (branch) | Digest (main) |
|---|---|---|---|---|---|
| `medium9-units-play` | laptop | 18 | between 547, 780 | `306a92a41cc8fae0` | same |
| `medium9-stack-close` | laptop | 35 | at 547 | `306a92a41cc8fae0` | same |
| `medium9-wood-close` | laptop | 35 | at 780 | `2c0e18547e9eacd9` | same |
| `medium9-gold-play` | laptop | 18 | at 780 | `806f0958b1918a5b` | same |
| `medium9-gold-close` | laptop | 35 | at 780 | `806f0958b1918a5b` | same |
| `medium8-units-play` | laptop | 18 | between 1145, 639 | `c29a99b7dc21ccac` | same |
| `small2-camps-play` | laptop | 18 | at 527 | `ff55c006fe645483` | same |
| `small2-camps-close` | laptop | 35 | at 527 | `ff55c006fe645483` | same |
| `medium9-units-edge` | phone | 6 | between 547, 780 | `306a92a41cc8fae0` | same |
| `medium9-units-flat` | phone | 5 | between 547, 780 | `306a92a41cc8fae0` | same |
| `lineup-close` | laptop | 35 | between 125, 170 | `62bef9c95b4d1f08` | same |
| `lineup-play` | laptop | 18 | between 125, 170 | `62bef9c95b4d1f08` | same |
| `lineup-edge` | phone | 6 | between 125, 170 | `62bef9c95b4d1f08` | same |

The lineup's digest is `62bef9c95b4d1f08`, the value proto/lineup.mjs gives on main. The medium 9 views that raise
the same army share one digest: the camera doesn't move the digest.

## Units in each view

Each line: unit id, kind, civ, terrain under it, `opaque` (pixels with alpha), `tinted` (pixels the civ's colour
changes), the tinted share of the opaque pixels, `mean` (RGB of the tinted pixels), `nearest` (civ by chromaticity).

### `medium9-units-play`

```
unit 715  scout  civ 712  on plains  opaque 436  tinted 31  share 7.1%  mean 159,84,69  nearest 712  ok
unit 718  camp  civ 712  on plains  opaque 353  tinted 112  share 31.7%  mean 134,62,54  nearest 712  ok
unit 716  scout  civ 713  on coast  opaque 426  tinted 26  share 6.1%  mean 89,100,136  nearest 713  ok
unit 717  scout  civ 714  on forest  opaque 429  tinted 26  share 6.1%  mean 162,126,64  nearest 714  ok
unit 727  camp  civ 713  on coast  opaque 353  tinted 112  share 31.7%  mean 70,80,114  nearest 713  ok
unit 728  camp  civ 714  on hills  opaque 362  tinted 112  share 30.9%  mean 135,103,51  nearest 714  ok
```

### `medium9-stack-close`

```
unit 715  scout  civ 712  on plains  opaque 1616  tinted 75  share 4.6%  mean 168,74,67  nearest 712  ok
unit 718  camp  civ 712  on plains  opaque 1203  tinted 347  share 28.8%  mean 138,61,56  nearest 712  ok
```

### `medium9-wood-close`

```
unit 717  scout  civ 714  on forest  opaque 1616  tinted 75  share 4.6%  mean 168,132,61  nearest 714  ok
```

### `medium9-gold-play`

```
unit 716  scout  civ 713  on coast  opaque 428  tinted 32  share 7.5%  mean 89,94,122  nearest 713  ok
unit 717  scout  civ 714  on forest  opaque 428  tinted 32  share 7.5%  mean 151,116,62  nearest 714  ok
unit 718  camp  civ 714  on forest  opaque 349  tinted 113  share 32.4%  mean 133,102,50  nearest 714  ok
unit 727  camp  civ 712  on coast  opaque 674  tinted 105  share 15.6%  mean 142,63,55  nearest 712  ok
unit 728  camp  civ 713  on plains  opaque 674  tinted 105  share 15.6%  mean 75,82,118  nearest 713  ok
```

### `medium9-gold-close`

```
unit 717  scout  civ 714  on forest  opaque 1616  tinted 75  share 4.6%  mean 168,132,61  nearest 714  ok
unit 718  camp  civ 714  on forest  opaque 1203  tinted 347  share 28.8%  mean 138,108,51  nearest 714  ok
```

### `medium8-units-play`

```
unit 784  scout  civ 781  on plains  opaque 429  tinted 30  share 7.0%  mean 150,76,63  nearest 781  ok
unit 785  scout  civ 782  on forest  opaque 423  tinted 32  share 7.6%  mean 91,98,126  nearest 782  ok
unit 786  scout  civ 783  on forest  opaque 424  tinted 32  share 7.5%  mean 155,121,64  nearest 783  ok
```

### `small2-camps-play`

```
unit 470  scout  civ 467  on marsh  opaque 429  tinted 31  share 7.2%  mean 150,77,64  nearest 467  ok
unit 471  scout  civ 468  on plains  opaque 423  tinted 32  share 7.6%  mean 91,98,126  nearest 468  ok
unit 472  scout  civ 469  on coast  opaque 428  tinted 32  share 7.5%  mean 153,119,63  nearest 469  ok
unit 477  camp  civ 468  on coast  opaque 341  tinted 110  share 32.3%  mean 70,80,115  nearest 468  ok
unit 478  camp  civ 469  on plains  opaque 698  tinted 112  share 16.0%  mean 140,102,50  nearest 469  ok
```

### `small2-camps-close`

```
unit 471  scout  civ 468  on plains  opaque 1615  tinted 76  share 4.7%  mean 71,95,150  nearest 468  ok
unit 472  scout  civ 469  on coast  opaque 1617  tinted 75  share 4.6%  mean 170,134,62  nearest 469  ok
unit 477  camp  civ 468  on coast  opaque 1211  tinted 351  share 29.0%  mean 62,81,126  nearest 468  ok
unit 478  camp  civ 469  on plains  opaque 1911  tinted 352  share 18.4%  mean 138,106,50  nearest 469  ok
```

### `medium9-units-edge`

```
unit 715  scout  civ 712  on plains  opaque 145  tinted 14  share 9.7%  mean 152,80,63  nearest 712  ok
unit 718  camp  civ 712  on plains  opaque 127  tinted 56  share 44.1%  mean 119,61,50  nearest 712  ok
unit 716  scout  civ 713  on coast  opaque 148  tinted 14  share 9.5%  mean 90,95,115  nearest 713  ok
unit 717  scout  civ 714  on forest  opaque 144  tinted 11  share 7.6%  mean 154,120,59  nearest 714  ok
unit 727  camp  civ 713  on coast  opaque 126  tinted 55  share 43.7%  mean 77,75,94  nearest 713  ok
unit 728  camp  civ 714  on hills  opaque 126  tinted 56  share 44.4%  mean 120,89,48  nearest 714  ok
```

### `medium9-units-flat`

```
unit 715  scout  civ 712  on plains  opaque 152  tinted 15  share 9.9%  mean 155,86,69  nearest 714  NOT ITS OWN CIV
unit 718  camp  civ 712  on plains  opaque 128  tinted 53  share 41.4%  mean 128,66,54  nearest 712  ok
unit 716  scout  civ 713  on coast  opaque 142  tinted 16  share 11.3%  mean 98,99,119  nearest 713  ok
unit 717  scout  civ 714  on forest  opaque 138  tinted 12  share 8.7%  mean 150,115,63  nearest 714  ok
unit 727  camp  civ 713  on coast  opaque 120  tinted 54  share 45.0%  mean 80,78,95  nearest 713  ok
unit 728  camp  civ 714  on hills  opaque 126  tinted 56  share 44.4%  mean 121,90,48  nearest 714  ok
```

### `lineup-close`

```
unit 13  scout  civ 10  on forest  opaque 1616  tinted 75  share 4.6%  mean 166,72,65  nearest 10  ok
unit 14  scout  civ 11  on plains  opaque 1576  tinted 76  share 4.8%  mean 74,98,152  nearest 11  ok
unit 15  scout  civ 12  on plains  opaque 1576  tinted 76  share 4.8%  mean 166,131,60  nearest 12  ok
unit 18  camp  civ 12  on plains  opaque 1236  tinted 348  share 28.2%  mean 135,106,50  nearest 12  ok
unit 16  camp  civ 10  on plains  opaque 1220  tinted 344  share 28.2%  mean 136,59,54  nearest 10  ok
unit 17  camp  civ 11  on forest  opaque 1210  tinted 351  share 29.0%  mean 63,82,127  nearest 11  ok
```

### `lineup-play`

```
unit 13  scout  civ 10  on forest  opaque 423  tinted 32  share 7.6%  mean 155,81,68  nearest 10  ok
unit 14  scout  civ 11  on plains  opaque 429  tinted 32  share 7.5%  mean 91,98,126  nearest 11  ok
unit 15  scout  civ 12  on plains  opaque 429  tinted 32  share 7.5%  mean 154,120,63  nearest 12  ok
unit 18  camp  civ 12  on plains  opaque 341  tinted 110  share 32.3%  mean 135,104,51  nearest 12  ok
unit 16  camp  civ 10  on plains  opaque 330  tinted 101  share 30.6%  mean 139,64,57  nearest 10  ok
unit 17  camp  civ 11  on forest  opaque 341  tinted 110  share 32.3%  mean 70,80,115  nearest 11  ok
```

### `lineup-edge`

```
unit 13  scout  civ 10  on forest  opaque 159  tinted 12  share 7.5%  mean 160,86,72  nearest 10  ok
unit 14  scout  civ 11  on plains  opaque 158  tinted 12  share 7.6%  mean 99,103,128  nearest 11  ok
unit 15  scout  civ 12  on plains  opaque 158  tinted 12  share 7.6%  mean 159,124,67  nearest 12  ok
unit 18  camp  civ 12  on plains  opaque 126  tinted 54  share 42.9%  mean 127,95,51  nearest 12  ok
unit 16  camp  civ 10  on plains  opaque 129  tinted 53  share 41.1%  mean 128,66,54  nearest 10  ok
unit 17  camp  civ 11  on forest  opaque 129  tinted 54  share 41.9%  mean 81,79,96  nearest 11  ok
```


## Finding: the flat zoom

`medium9-units-flat` (phone, k 5): scout 715 (civ 712) has 15 tinted pixels, its mean is (155, 86, 69), and its
nearest colour is civ 714's. At k 5 the figure is about 14 px tall and its banner about 15 px, so the banner's edge
pixels, mixed with the art's brown, read off-hue. Every other unit in every view is nearest its own civ, at k 35,
k 18 and phone k 6. `--units` exits 1 on this view, and so on the run. That is the bound build-3 tunes against
(the banner's size at k 5, or the view's place in the bound); the probe measures it as it is.

## Measurement

- The probe draws each unit alone on a canvas of its own rectangle (plus 8 px for the outlines), so a figure the map
  cuts is measured whole. A unit's `clipped` says whether its rectangle reaches past the canvas.
- Label boxes come from `measureText` at 100× the font size: at world scale the browser rounds glyph bounds to whole
  pixels, which gave every box a height of one world unit.
