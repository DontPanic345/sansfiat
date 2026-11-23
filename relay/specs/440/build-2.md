# #440 build-2 (canvas): the mountain ranges on the map

Read `relay/specs/440/spec.md`, `relay/specs/440/ranges.md`, build-1's summary and
`relay/specs/440/easel.md`. You are the painter: mountains that join into ranges, judged by eye on the
retro maps beside main. No Rust changes; nothing under `engine/core`, `engine/cli` or `engine/web`.

## What the easel gives you

- `www/assets/cut.mjs`: hex regions and sprite regions (a source rectangle, a keep-polygon, a named
  ground key, the source's alpha, a ground point and a scale), `patches`, `mirror`.
- `www/assets/sources.js`: `peak.left` (the range's left peak) cut into `mountain.peak.*` and
  `mountain.lone.*`; `terrain.mountain.*` the plains' grass; the ridges `mountain.ridge.{e,se,sw}.*`
  still placeholders. Every mountain key is 192×128, anchored at (96, 112).
- `www/map/ridges.js` (`ridgePieces`, `ridgeClass`): a peak a mountain tile, a ridge a shared edge, in
  depth order. `www/map/layers/ridges.js` draws them after `terrain`, culled by each piece's drawn
  rectangle; `view.noCull` turns that off.
- `www/shots.mjs`: the plains and mountain views (`--out`, `--diff`, `--cull`). `www/play/ridges.mjs`
  and `www/check.mjs` check the pieces in CI.

You may change any of it; record why in your summary. Every key and rectangle older than this piece
stays; you may change the mountain keys' sizes and anchors (they are after the icons). `--check`, a
byte-identical repeat build, `--cull` and the CI checks must still hold, and the pieces' rules stay the
spec's unless you add a piece (below).

## The look

Make a chain of mountains read as one range, as the peaks in `docs/proposals/map-art/ui.png` (the
approved target) do, on the spec's views. Every pixel comes from a named rectangle of `terrain.png`.
What you judge, in the captures:

- **The ridges.** Cut `mountain.ridge.e.*` and `.se.*` (`.sw` mirrored from `.se`) from the range's
  lower peaks and spurs: a ridge must fill the gap between two peaks so a straight run, a bend and a
  fork read as one mass, not beads. The low peaks between the big ones (about x 520–700, y 190–270)
  and the spurs at the range's foot are candidates; look along the whole range.
- **The peaks and the lone peaks.** One source peak on every range tile reads as a stamp: use two or
  three of the range's peaks (the right one, x ≈ 1140–1400, has its neighbour's flank to mask off), and
  give a lone peak foothills or boulders so it doesn't read as a range cut short.
- **Caps and forks.** A cap is a peak with one ridge, a fork a peak with three. If a cap ends too
  bluntly or a fork reads as a knot, you may add keys (after the existing mountain keys) and teach
  `ridgePieces` to choose them by class or by which edges are open; say so in `views.md` and keep the
  CI checks true to the new rule.
- **Depth and overhang.** Southern pieces overlap northern ones without a seam showing; a peak rises
  into the tile north of it, but not so far that a river, pass, field, clan or unit on that tile is
  hard to read (they draw over it; check that they still read on rock and snow). The scale (74/109 cell
  px per source px, the plains') is a starting point: the target's peaks are a little bigger.
- **Beside the rest.** The peaks' grey and snow sit with the grass, placeholder hills and forest; a
  hills pass between two caps reads as a gap.
- **Zoom.** `small9-fork-edge` (phone, k 6, cells) and `small9-fork-flat` (k 5, flat fill) show the
  range grey at a glance at both.

What you don't do: other terrains, transitions, the hills, the HUD. They are later pieces.

## The evidence

1. `node www/assets/draw-atlas.mjs` twice: the same sha256 both times; `--check` passes.
2. A main worktree (`git worktree add /tmp/main <base>`, `./build-web.sh` there) with this branch's
   `www/shots.mjs` copied in. Shoot every view on main and the branch and `--diff` them: the cameras
   match on all. `--cull` passes on the branch.
3. Commit main's and the branch's PNGs of `small9-fork-close`, `small9-fork-play`, `small2-bend-play`,
   `medium9-blob-play` and `small9-edge-bottom` under `relay/specs/440/shots/` as `<view>-main.png` and
   `<view>-branch.png` (only these ten).
4. `relay/specs/440/views.md`: the `--diff` table, the sha256 lines, the `--cull` output, each
   mountain view's `stats.terrainMs` on main and the branch, what you changed from the easel and why,
   and a few lines on how each point above reads in the captures.
5. On the branch page (`play/live.mjs` or `node www/play.mjs "?size=small&seed=9&run=1"`), select a
   mountain tile, the tile north of a peak and a unit, then pan and zoom: they behave as on main. A line
   in `views.md`.

Run `engine/check.sh` once at the end. Commit as `#440 build-2: the ranges`.
