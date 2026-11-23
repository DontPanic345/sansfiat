# #442 build-3 (canvas): the woods and the marsh, judged

Shot with `node www/shots.mjs` on the branch and on a worktree of main at the base (`7ae72cc`, `./build-web.sh` there) with
the branch's `shots.mjs` copied in. The 14 PNGs are in `shots/` as `<view>-main.png` and `<view>-branch.png`.

## Evidence

**Atlas.** `node www/assets/draw-atlas.mjs`, twice, the same both times; `--check` passes (143 cut cells, 157 cells match
`layout()`); `play/art.mjs` passes (157 cells, sheet 1024×2288).

```
atlas.png  b34f973998795ef6ad063c87f817154248663ba30e8ed35bb1715dc66d504127
atlas.json 62fae1383012b54377e08fe57588b54b19048abc97550cd677b38a598e36e90f
```

**`--diff` main branch** (all 48 views, exit 0: every camera and digest matches; the share of differing pixels is the art).
Window sizes are as in `views.json`.

| View | % differ |
|---|---|
| small2-close | 7.278 |
| small2-play | 4.083 |
| small2-edge | 1.634 |
| small2-flat | 0.837 |
| small3-play | 5.237 |
| small6-play | 6.046 |
| small9-play | 4.631 |
| medium8-play | 5.208 |
| medium9-play | 14.605 |
| large8-play | 7.772 |
| small9-fork-close | 4.645 |
| small9-fork-play | 4.205 |
| small2-bend-close | 8.576 |
| small2-bend-play | 4.222 |
| medium9-blob-play | 15.786 |
| large8-bend-play | 8.263 |
| large8-fork-play | 8.097 |
| small9-edge-bottom | 2.371 |
| small9-edge-right | 3.358 |
| small9-fork-edge | 1.813 |
| small9-fork-flat | 0.984 |
| small2-neck-close | 4.817 |
| small2-delta-close | 8.799 |
| small2-delta-play | 4.222 |
| small6-bay-close | 13.219 |
| small9-pond-close | 3.63 |
| small9-lake-play | 3.904 |
| medium8-strait-close | 4.865 |
| medium9-neck-play | 17.604 |
| large8-delta-play | 7.055 |
| large8-pond-play | 6.383 |
| small2-neck-edge | 1.637 |
| small2-neck-flat | 0.839 |
| medium9-woods-close | 19.445 |
| medium9-woods-play | 16.053 |
| medium9-woods-near | 9.836 |
| medium9-woods-far | 8.371 |
| medium9-woods-edge | 7.627 |
| medium9-woods-flat | 4.571 |
| small2-lone-close | 6.693 |
| small2-lone-play | 3.876 |
| small9-river-close | 9.089 |
| small2-delta-near | 2.001 |
| small2-delta-far | 1.663 |
| small2-delta-edge | 1.637 |
| small2-delta-flat | 0.846 |
| medium9-fields-close | 27.293 |
| small2-marsh-fields-close | 7.005 |

(Even the flat views differ, by 0.8 to 1.0 %: forest and marsh flat colours moved with the grounds.)

**`--cull`** (branch): 38 views with an `on`, all `identical`, none `DIFFER`, exit 0. For example
`medium9-fields-close  stats.woods 306 culled, 1209 unculled  identical`; `small2-marsh-fields-close  stats.woods 80 culled, 260 unculled  identical`.

**`--time`** (`stats.terrainMs`, median of five static redraws; three runs, branch and main alternating, on this runner):

| View | Branch medians (ms) | Main medians (ms) | Difference |
|---|---|---|---|
| medium9-woods-play | 26.4, 25.3, 25.0 (`stats.woods` 918) | 23.8, 23.7, 24.1 | +2.6, +1.6, +0.9 |
| large8-play | 26.7, 26.9, 27.3 (`stats.woods` 479) | 26.7, 26.2, 26.8 | +0.0, +0.7, +0.5 |

The woods layer alone, measured by turning it off in the page, costs about 1 to 2 ms on `medium9-woods-play`. Both are inside the 3 ms bar.
(The first cut, 1646 pieces at gap 0.42, cost 2.4 ms by that measure and +2.6 to +3.9 over main.)

**Flat colours** (`flat` of `terrain.<kind>.0`; mountain's is the peak's, as `layers/terrain.js` reads it):

| sea | lake | coast | plains | forest | hills | mountain | desert | marsh |
|---|---|---|---|---|---|---|---|---|
| #013d66 | #206793 | #978550 | #7b882d | #34442e | #b39f47 | #777370 | #c59c65 | #3f5f42 |

Closest pair: hills and desert, RGB distance 35.1 (bar 28). Forest and marsh, the pair this piece moved, are 35.4 apart.

**The page** (`proto/live.mjs`, run on both trees; the same staged map with a camp in a seven-tile wood and marsh on the sea). I clicked the unit
standing in the wood, a forest tile and a marsh tile, then zoomed 20 → 14 → 12 → 11 → 8 → 12 → 18, panned, and clicked the unit again.
Selection (`selected`, `selectedUnit`, `dataset.selected`), the camera and the zoom-to-selection are the same on both, page errors none. On the
branch `stats.woods` is 86 at k ≥ 12 (cell height 25 at k 12) and 0 at k 11 (cell height 23) and k 8, 86 again on the way back.

## `WOODS` as left

```js
gap: { tree: 0.56, reed: 0.6 },       // was 0.42, 0.36
inset: { tree: 0.06, reed: -0.05 },   // was one number, 0.1
jitter: 0.14,                         // was 0.07
band: 0.34,                           // was 0.32
sparse: 0.6,                          // was 0.4
river: 0.22,                          // kept
clearing: { x: 0.6, top: 0.0, bottom: 0.8 },  // kept
from: 24,                             // kept: k 12 draws (cell 25 high), k 11 does not (23)
```

What changed and why:

- **Trees are bigger and fewer.** The big cells are cut at scale 0.9 (was 0.7) in an 80×100 cell (anchor 40,95); the small at 0.85 and 0.65 in 48×64
  (anchor 24,60). At the old size the wood read as small pines; at 1.29× with `gap` 0.56 the crowns overlap as the target's do, and the
  layer has 918 pieces on `medium9-woods-play` instead of 1646, which is what the time bar needed.
- **`jitter` 0.14** breaks the lattice's rows: at 0.07 an edge tile showed a row of trees along its hex edge.
- **`band` 0.34, `sparse` 0.6.** 0.4 and above left a lone hex of two trees; 0.34 gives the lone tile 2 to 3 big trees and 3 saplings in the
  staged check (more big ones and the staged lone tile loses its small trees), and the margin of an edge tile thins over a third of a hex.
- **Reeds stand past the edge (`inset.reed` -0.05, `slotsOf(gap, inset)`).** With the bank gone the marsh met the sea along a straight hex edge; reeds
  that overhang it by up to 0.05 + `jitter` break that edge, on land edges as well. `check-map.mjs` takes the reed lattice with `WOODS.inset.reed`;
  `play/woods.mjs` needed no change. This is a piece rule.
- **`gap.reed` 0.6** so the pools show between the reeds (13 slots a marsh tile).
- Cells: reeds are now 48×60 (anchor 24,56), cut at scale 0.75 / 1.1 and scaleY 1.2 / 2.0 (reed.2 is reed.0 mirrored) and ramped from `treeLeaf`
  to `straw`, not `treeDark`, so they read against the dark marsh ground. `cut.mjs`'s `ramp` gained `keepWater`.

## The grounds

- **Forest ground lightens toward the grass.** `ramp(treeDark, grassDark, 0.7)`, from `ramp(treeDark, treeLeaf, 0.95)`, ground mean (89,106,45) at mix 0.55
  and darker at 0.7. The target has pines on grass; the dark floor made each wooded hex a dark hexagon with a few trees on it,
  worst on a margin tile and a lone one. On the lighter floor the wood's edge is the fringe 441 draws on the neighbour. It still reads as forest
  without trees at `-far` (a dark green patch against the plains' yellow-green), and the forest's `flat` stays the dark `treeFloor` rectangle, 35 from the marsh.
- **Marsh ground is the clean plains hex with four large pools** (up to 120×44 source px, feather 5, from the sea block) and the ramp at mix 0.45 with
  `keepWater`, so the pools keep the sea's blue. 441's pools were 2–3 px in a 148 px cell. The marsh is the plains' pixels toward the dark olive
  with blue patches and straw reeds standing in and over them.

## How each point reads

- **Interior** (`medium9-woods-close`, `-play`): dense overlapping crowns, no hex-shaped gaps. The hex edges show through between trees; that is fine.
- **Margin**: the wood thins into small scattered trees over the band, on the lighter floor, with 441's fringe outside; no hex outline. Where one hex row of
  trees runs along a north edge there is a faint line of trees, softened by `jitter`.
- **Lone hex** (`small2-lone-close`, `-play`): 2 big trees and 3 saplings on the dark patch, a clump about half a hex across. It is smaller than the hex; the
  band can't grow it without losing its saplings on the staged map.
- **Rivers** (`small9-river-close`): the river runs through the wood with no tree on its course, trees stand either side.
- **Fields** (`medium9-fields-close`, `small2-marsh-fields-close`): the clearing leaves the furrows whole on the forest and marsh tiles, but both views have the
  neighbouring peaks and the floodplain's green over most of the field tile, so the furrows read thinly; the clearing was not changed.
- **Marsh** (`small2-delta-close`, `-play`, `large8-delta-play`): reeds standing over dark wet ground with blue pools, a ragged rim on the sea.
  It reads apart from forest (lighter, teal-olive, blue patches, straw reeds) and from coast. The floodplain's bright green covers the marsh in these views,
  as it does on main. The pools are rectangles with a soft rim; at play zoom they read as water, up close as blocky ponds.
- **Over**: peaks stand over trees (ridges draw after woods), rivers, floodplains, borders, fields, clans, units, icons and the ring draw over the wood.
- **Zoom boundary**: `WOODS.from` stays 24. At `-near` (k 12) trees are 8 to 17 px, still trees, not noise. At `-far` (k 11) the forest reads from its ground;
  the mean colour of the wood's ground is within a few units of the near view's (trees against floor). `-edge` (k 6) and `-flat` (k 5) tell every kind apart.
