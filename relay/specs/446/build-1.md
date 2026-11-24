# #446 build-1 (easel): the harness and the baseline

Read `relay/specs/446/spec.md` first: the bars, the controls, "Settled here", "The verify set" and the pop cases. Build-2
speeds the static layer and build-3, the canvas, fixes selection, popping and legibility. Both judge with your tools, on
all three trees. Your job is tools that measure exactly what the spec measures, and a baseline that reproduces the spec's
numbers. Don't change Rust (nothing under `engine/core`, `engine/cli`, `engine/web`) and don't change how anything draws:
every view's PNG stays byte-identical to main's.

Read these before you edit: `www/shots.mjs` (508 lines: `setUp`, `cull`, `time`, the dispatch at the end),
`www/shots-views.mjs`, `www/map/view.js` (`onScreen`, `noCull`, `tileAt`, `drawNow`), and the spec step's protos under
`relay/specs/446/proto/`: `hits-tail.mjs`, `pop-tail.mjs`, `overlays-tail.mjs`, `layers-tail.mjs` and `control.sh`. Each
tail ran appended to `shots.mjs`'s first 431 lines. They are your pattern.

## 1. `www/shots-probe.mjs`

`shots.mjs` would pass 600 lines with these modes, so they go in a new file, `www/shots-probe.mjs`. Export `setUp` (and
whatever it needs) from `shots.mjs` for it to import, and keep `shots.mjs`'s dispatch working when run directly: guard its
command line with an `import.meta` main check, or move `setUp` and `CAMERA` into a module both import (`shots-setup.mjs`).
Your call; say which in `shots.mjs`'s head. Every `shots*.mjs` file copies onto a control tree together (`control.sh`
copies `shots*.mjs`).

- `node www/shots-probe.mjs --hits [NAME,…]`: the verify set and every view with `units`, `clans` or `places` (or the names
  given). Per view, each unit, clan and place in `game.map.stats` with a drawn box (a place's `role` not `mark`), the tile
  under its box's centre by `tileAt` (canvas px divided by `dpr`) against its own tile (a unit's `tile`, a clan's `home`, a
  place's `tile`). Print each view's drawn count and every miss; exit 1 on any miss. On the pre-art tree, with no
  `stats.units` or `stats.clans`, it prints what it finds.
- `node www/shots-probe.mjs --pop [DIR]`: each case in `POPS`, its tile put 90 and 130 css px past each edge, drawn with
  culling on and off (`dirty` set each time); print the pixels that differ per placement; exit 1 on any. With `DIR`, write
  each differing placement's two PNGs.
- `node www/shots-probe.mjs --overlays DIR [NAME,…]`: each layer in `view.js`'s `OFF` set (territories, fertility, country,
  hidden-costs, worn, festival-routes) turned on alone over each named view (default `medium9-play`, `medium9-places-close`,
  `small2-edge`): `DIR/<view>-<layer>.png`, and the share of pixels it moves by more than 48 in a channel.
- `node www/shots-probe.mjs --layers [NAME,…]`: `layers-tail.mjs`'s per-static-layer median ms (the verify set by default).
- `--time` in `shots.mjs` takes `verify` as a name for the whole verify set, and prints each view's median frame
  (terrain + overlay) as a third number.

## 2. `www/shots-views.mjs` and `view.js`

- `VERIFY`: the spec's 21 names, exported. `POPS`: the spec's seven cases, exported. Each must name a view that exists.
- `MapView.onScreen` returns true when `this.noCull`, so `--pop` and `--cull` see what the 80 px margin hides. Nothing
  draws differently with `noCull` false.

## 3. The baseline, `relay/specs/446/baseline.md`

Run `relay/specs/446/proto/control.sh` (after `engine/build-web.sh` or `check.sh` has built `www/pkg`), then on the
branch, `/tmp/main` and `/tmp/preart`:

- `--out` on the verify set; `--diff` branch against main (0% on every view: you changed no drawing) and branch against
  pre-art (cameras and digests match: the table of shares that differ).
- `--time verify` on all three: each view's median terrain, overlay and frame ms, the ratio to pre-art, and the bar
  (`max(1.5 × pre-art, 8)`), marking each view over it. Run it twice on the branch and say how far the two runs differ, so
  build-2 knows the noise.
- `--layers` on main and pre-art.
- `--hits` and `--pop` on the branch (which draws as main). The spec's `hits.txt` and `pop.txt` misses must show; if a
  number differs, say why.
- `--overlays DIR` on main and pre-art: the shares side by side.
- `--units`, `--clans`, `--places`, `--cull` on the branch: each one's exit code and failures (main's `--units` fails on
  `medium9-units-flat`, the spec says why).

Each table in `baseline.md`, each command's raw output in a `.txt` beside it.

## 4. Checks

`engine/CLAUDE.md` is 8,086 bytes against `check.sh`'s 8,192. Name `shots-probe.mjs` and its four modes by folding them into
the existing `--units|--clans|--places` line, and tighten a line of your own `shots` rows to pay for it. Run
`engine/check.sh` whole once before you finish.

Build-2 and build-3 rely on: `--time verify` with the frame column; `--hits` and `--pop` exiting 1 on any miss; `--layers`;
`control.sh` making both controls; `baseline.md`'s tables as the before.
