# #442 build-3 (canvas): the woods and the marsh on the map

Read these first: `relay/specs/442/spec.md`, build-1's and build-2's summaries,
`relay/specs/442/atlas.md`, `easel.md` and `proto/woods.json`. You are the painter. Your job is a
forest that reads as a wood, dense inside and sparse at its margins, a lone wooded hex that reads as a
clump, and a marsh that reads as reeds and water. All of it is judged by eye on the retro maps beside
main. Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easels give you

- From `www/assets/cut.mjs` and `sources.js`:
  - sprite regions with `scaleY` and the keys `ground`, `bank` and `tuft`, plus the ops `mirror` and
    `ramp`;
  - first cuts of `tree.big.0`–`.2`, `tree.small.0`–`.2` and `reed.0`–`.2`, which
    `relay/specs/442/cells.png` shows;
  - the forest and marsh grounds as 441 left them (`terrain.forest.*`, a ramped grass, and
    `terrain.marsh.*`, grass with four small pools).
- `www/map/woods.js`: `WOODS` (every number of the rule), `slotsOf`, `woodPieces` and
  `riverSegments`. `www/map/layers/woods.js` draws the pieces after `edges` and before `ridges`,
  culled, from `WOODS.from`, and redraws when the field tiles on forest and marsh change (`stale`).
- `www/map/edges.js` draws no bank on a marsh shore.
- `www/shots.mjs` holds 48 views (`--out`, `--diff`, `--cull`, `--time`), and the views with `turns`
  record the digest. `www/check-map.mjs` and `www/play/woods.mjs` check the pieces in CI.

You may change any of it. Record why in your summary. Every key and rectangle older than this piece
stays, but you may resize, recut or add after the new keys. `--check`, a byte-identical repeat build,
`--cull` and the CI checks must hold. If you change a piece rule (say a size between big and small, or
reeds denser along the water), change `check-map.mjs` and `play/woods.mjs` to the new rule and record
it as a decision.

## The look

Make the map read as `docs/proposals/map-art/ui.png` (the approved target) does: woods of the
source's pines standing on grass, round-crowned and overlapping, with saplings at their edge. Every
pixel comes from a named rectangle of `terrain.png`, recoloured at most. Judge these in the captures:

- **The interior** (`medium9-woods-close` and `-play`): dense wood, crowns overlapping across hex
  edges with no hex-shaped gaps or rows. Tune `gap`, `jitter` and the tree scale. Add a third size,
  or a clump cut from the band, if single trees can't read as dense. A canopy that hides every hex
  edge is right for a wood.
- **The margin** (any edge tile, and every play view): the wood thins over `band` into small,
  scattered trees, and 441's forest fringe on the neighbour still reads as the floor running out. It
  must not end in a hard hex outline or a row of saplings.
- **The lone hex** (`small2-lone-close` and `-play`): a clump with saplings round it, about the size
  of the hex, not a filled hexagon.
- **Rivers** (`small9-river-close`, `medium9-fields-close`): the river runs through the trees in the
  open, and the trees on either bank don't cover it. Tune `river`.
- **Fields** (`medium9-fields-close`, `small2-marsh-fields-close`): the furrows show on cleared ground.
  Tune the clearing so the furrows are whole and the wood around the clearing still reads as a wood.
- **The marsh** (`small2-delta-close`, `-play` and `large8-delta-play`): reeds standing in wet ground,
  water you can see at play zoom (make the marsh's pools larger: `441/notes.md`), and reeds rather
  than a bank along the sea. Tune the reed recipe, `gap` for reeds, and the marsh ground. It must
  read apart from forest and from coast at a glance.
- **The ground under the trees.** Decide whether forest ground stays the dark floor or lightens toward
  the source's grass under its trees, and say why.
- **Over it.** Peaks stand over trees. Rivers, floodplains, borders, fields, clans, units, icons and
  the selection ring read on top of a wood.
- **The zoom boundary.** Set `WOODS.from`, and move the `-near` and `-far` views' k to straddle it.
  At `-near` the trees still read as trees and not as noise. At `-far` the forest and the marsh read
  from their grounds alone, with no jump in colour as the trees go. At `-edge` and `-flat` (phone k 6
  and k 5), every kind is told apart. No two of the nine kinds' `flat` colours may be closer than 28 in
  RGB.
- **The time.** `--time medium9-woods-play,large8-play`: the branch's median `stats.terrainMs` is at
  most 3 ms over main's. If it isn't, cut slots (a larger `gap` and fewer sparse trees) before
  anything cleverer.

What you don't do: units, settlements, the HUD and the legend. They are later pieces.

## The evidence

1. Run `node www/assets/draw-atlas.mjs` twice: the sha256 is the same both times, and `--check`
   passes.
2. Make a main worktree (`git worktree add /tmp/main <base>`, then `./build-web.sh` there) with this
   branch's `www/shots.mjs` copied in. Shoot every view on main and on the branch and `--diff` them:
   the cameras and digests match on all. `--cull` passes on the branch. `--time` both.
3. Commit main's and the branch's PNGs of `medium9-woods-close`, `small2-lone-close`,
   `small9-river-close`, `small2-delta-close`, `medium9-fields-close`, `medium9-woods-near` and
   `medium9-woods-far` under `relay/specs/442/shots/` as `<view>-main.png` and `<view>-branch.png`
   (these fourteen only).
4. Write `relay/specs/442/views.md` with:
   - the `--diff` table, the sha256 lines and the `--cull` output;
   - the `--time` medians on both trees;
   - the nine flat colours and the closest pair's RGB distance;
   - `WOODS` as you left it, with what you changed from the easels and why;
   - a few lines on how each point above reads in the captures.
5. On the branch page (`node www/play.mjs`, or `play/live.mjs`), select a forest tile, a marsh tile
   and a unit standing in a wood, then pan and zoom across `WOODS.from`. Check they behave as on main,
   and put a line about it in `views.md`.

Run `engine/check.sh` once at the end. Commit as `#442 build-3: the woods`.

## Size

About 7 files and 14 committed PNGs, 250 lines: `assets/sources.js`, maybe `cut.mjs` and
`placeholders.js`, `atlas.png`/`atlas.json`, `map/woods.js` (`WOODS`), `shots.mjs` (the `-near` and
`-far` k), maybe `check-map.mjs` and `play/woods.mjs`, `views.md`, and `shots/`.
