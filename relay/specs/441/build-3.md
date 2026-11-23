# #441 build-3 (canvas): the shores and grounds on the map

Read `relay/specs/441/spec.md`, `relay/specs/441/shores.md`, build-1's and build-2's summaries, and
`relay/specs/441/atlas.md` and `easel.md`. You are the painter: every ground as art, and water held by a
shore, judged by eye on the retro maps beside main. No Rust changes; nothing under `engine/core`,
`engine/cli` or `engine/web`.

## What the easels give you

- `www/assets/cut.mjs`: hex regions (with `fill`, a repeated source block), sprite regions (with
  `rotate` and the keys `ground` and `bank`), `patches`, and the ops `mirror` and `ramp` (luminance onto
  a ramp between two source rectangles' means); derived cells, a cut cell masked to a ragged edge
  strip (`DEPTH`, `AMP`, `FEATHER`, `TAPER` in `placeholders.js`).
- `www/assets/sources.js`: first cuts of `terrain.<kind>.*` for sea, lake, coast, forest, hills,
  desert and marsh; `shore.edge.<d>.<v>` from the bank's level run, turned; one bank disc for all
  twelve `shore.in.<c>` / `shore.out.<c>`; `fringe.<kind>.<d>.<v>` from each kind's ground.
  `relay/specs/441/cells.png` shows them.
- `www/map/edges.js` (`edgePieces`, `FRINGE`): fringes, shore edges (none at a mouth) and shore
  corners, in order. `www/map/layers/edges.js` draws them after `terrain`, culled; `view.noCull`
  turns culling off.
- `www/shots.mjs`: the 21 older views and the 12 shore views (`--out`, `--diff`, `--cull`).
  `www/check-map.mjs` and `www/play/edges.mjs` check the pieces in CI.

You may change any of it; record why in your summary. Every key and rectangle older than this piece
stays; you may change the new keys' sizes and anchors and add keys after them. `--check`, a
byte-identical repeat build, `--cull` and the CI checks must hold; if you change a piece rule (corner
pieces for land fringes, say, or `FRINGE`), change `check-map.mjs` and `play/edges.mjs` to the new
rule and record it as a decision.

## The look

Make the map read as `docs/proposals/map-art/ui.png` (the approved target) does on the spec's views:
grass to a bank of tan blocks, deep water beyond, hills as mounds on grass. Every pixel comes from a
named rectangle of `terrain.png`, recoloured at most. What you judge, in the captures:

- **Water.** Sea that reads as water, not a wallpaper: the fill's block, scale and per-variant offset
  so the ripples don't repeat in a visible grid across a bay. A lake clearly lighter than the sea, and
  both distinct from the coast at a glance.
- **The bank.** It follows every shore as one line: thickness, how far onto the land, the strip it is
  cut from. The twelve corners join the edges with no gap, notch or lump, both the `out` corner at a
  cape or a neck and the `in` corner in a bay or round a pond. Cut corners from the coastline's turns
  (about (1000, 870), (1250, 790), (870, 830)) or build them from the run; give `in` and `out` their own
  cells.
- **Necks, straits, bays, ponds, mouths.** In `small2-neck-close` and `medium9-neck-play` a neck is a
  strip of land with a bank on both sides; in `medium8-strait-close` a strait is open water between
  two banks; in `small6-bay-close` and the ponds the water is held all round; in
  `small2-delta-close` and `small9-lake-play` the river runs through a gap in the bank into the water.
- **The grounds.** Coast reads as land by water, not beach and not plains; forest ground reads as
  forest without its trees (piece 4 plants them); hills as hills; desert as desert (decide whether the
  stipple, `overlay.desert-dots`, still draws, and say why); marsh as wet ground; mountain ground under
  the peaks. All beside each other and the peaks without one looking pasted on.
- **Fringes.** Neighbouring kinds meet in a ragged edge, not a hex outline and not a smear: their
  depth, raggedness, taper and `FRINGE`. Where two fringes of one kind meet at a vertex, no notch.
- **Over it.** Rivers, floodplains, passes, borders, fields, clans, units and icons still read on
  every ground and on the bank.
- **Zoom.** `small2-neck-edge` (phone, k 6, cells) and `small2-neck-flat` (k 5, the flat fill) tell
  sea, lake, coast, plains, forest, hills, mountain, desert and marsh apart at a glance; so does
  `small2-flat`. No two of the nine kinds' `flat` colours closer than 28 in RGB
  (main's closest pair, hills and mountain): cell means of recoloured grass fall well under that, so set
  a cell's `flat` rectangle where they do, on a colour of that ground in the sheet, and say how you
  separated them.

What you don't do: trees, reeds, units, settlements, the HUD and the legend. They are later pieces.

## The evidence

1. `node www/assets/draw-atlas.mjs` twice: the same sha256 both times; `--check` passes.
2. A main worktree (`git worktree add /tmp/main <base>`, `./build-web.sh` there) with this branch's
   `www/shots.mjs` copied in. Shoot every view on main and the branch and `--diff` them: the cameras
   match on all. `--cull` passes on the branch.
3. Commit main's and the branch's PNGs of `small2-neck-close`, `small2-delta-close`,
   `small6-bay-close`, `small9-lake-play`, `medium9-neck-play` and `small2-neck-flat` under
   `relay/specs/441/shots/` as `<view>-main.png` and `<view>-branch.png` (only these twelve).
4. `relay/specs/441/views.md`: the `--diff` table, the sha256 lines, the `--cull` output, each play
   view's `stats.terrainMs` on main and the branch, the nine flat colours and the closest pair's RGB distance (28 or more), what you changed from the easels
   and why, and a few lines on how each point above reads in the captures.
5. On the branch page (`node www/play.mjs "?size=small&seed=2&run=1"` or `play/live.mjs`), select a
   coast tile, a water tile and a unit, then pan and zoom: they behave as on main. A line in
   `views.md`.

Run `engine/check.sh` once at the end. Commit as `#441 build-3: the shores`.
