# #439 build-2 (canvas): the plains tile on the map

## `--diff`, main (0fd7835) against the branch

Both shot with `www/shots.mjs` (main's from a worktree with this branch's copy and the same wasm: no Rust
changed). The cameras (tile, `cx`, `cy`, `k`) match on all ten views.

| View | Changed share |
|---|---|
| small2-close | 19.206% |
| small2-play | 8.786% |
| small2-edge | 3.162% |
| small2-flat | 2.297% |
| small3-play | 3.086% |
| small6-play | 7.706% |
| small9-play | 4.914% |
| medium8-play | 10.642% |
| medium9-play | 5.169% |
| large8-play | 25.553% |

Every view has plains on screen, so there is no placeholder-only view. To show nothing but plains changed, `CELLS`
was emptied in a scratch edit and the atlas rebuilt: `small2-play`, `small2-flat` and `medium8-play` then differ
from main by 0 pixels, so the anchor and flat-colour code in `terrain.js` moves nothing else. (Reverted.)
Two branch runs of all ten views give byte-identical PNGs.

## The atlas repeats

```
6a0b0558a02b378c0a9cfa92a17263288c2627e38eef1cf5d358ab56e6175365  www/assets/atlas.png
b36d382c64a2c71a47c2ea6bbbfc7053aaf5d50e10bc27c7919218d701845f68  www/assets/atlas.json
```

Two runs of `node www/assets/draw-atlas.mjs`, the same hashes; `--check` passes.

## What changed from the easel's cut, and why

- **Rocks on one tile in three.** `.1` (the `plains.grass` region, rocks in) stays; `.0` and `.2` come from a new
  region `plains.clean`: the same hex, the same hill patch, and a second patch (source (541,806), 99×58, feather 6,
  over (530,862)) that covers the rocks with grass from above them. `.2` is `.0` mirrored. At k 18 the rocks were
  grey blobs the size of the map's hearths and grain circles on every plains tile; on one in three they read as
  small rocks, and the plain tiles dominate. `.0` is the flat colour's source and is the rock-free mean (`#7b882d`),
  so the flat fill is the colour the land mostly is.
- **The right edge inset is 5** (`insetRight`, `cut.mjs` moves the hex's centre by half the difference): at 3 a
  dotted dark line stayed on the hex's right edge, at 5 it is gone and the left edge, still at 3, is clean.
- **The rocks patch starts 7 px further left** than the rocks, because a 2-px grey remnant of the rock showed to
  its left at 537.
- `terrain.js` draws by the anchor (read once per draw, per distinct cell, scaled to the drawn size) and fills a
  kind's flat colour from its variant-0 cell's `flat` where it has one.

## How each point reads in the captures

- **Edge, outline, hill.** No outline, no hill, no seam in `small2-close` (k 35) or `small2-play`; neighbours
  meet as the placeholders do. The k ≥ 14 hex outline is main's and is the only line between plains.
- **Variants.** Three in a row read as grass with tufts, one with rocks; the tufts sit in different places in `.0`
  and `.2` (mirror), so a run of plains doesn't read as a stamp. At k 18 the rocks are small grey clusters on a
  third of the tiles, lower right, and are not mistaken for a hearth (white ring) or a grain circle (round, with a
  stalk).
- **Rivers, trails, fields, borders, clans, icons, units.** In `small2-play` the river is as easy to see on the
  olive as on the lime (blue against green both ways); field squares and the clan circles keep their contrast;
  squares.
- **Zoom boundary.** `small2-edge` (cells) and `small2-flat` (fill) give plains the same olive at a glance; the
  fill had been the placeholder's bright green.
- **Beside the placeholders.** The olive is darker and more yellow than the placeholder forest/hill/desert
  neighbours, but sits between them without clashing; sea and mountains are unchanged.

## Selection and zoom

Page check, `?size=small&seed=2&run=1` led to the main game, selecting a plains tile (585), then a clan's tile
(37, focus on clan 25), zooming 0.4×, 0.4×, 3×, 3× (k 20 → 8 → 6.6 → 19.9 → 35): the same `selected`, `focus`,
`k` and cell counts as main's worktree, no page errors. The map crosses no flat boundary on a laptop.
