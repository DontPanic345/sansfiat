# #444 build-2: the clans on the map, the views and the checks

Build-2 (easel) draws the clans in `www/map/layers/clans.js` from the `CLAN` table, with the leaning disc under each
cell (`leanings.js` exports `disc`), `stats.clans`, `clanProbe()`, the legend's clan rows, the 12 clan views and
`--clans` in `shots.mjs`, the lineup (`CLANS`, `CLAN_STATES`) in `play-stage.mjs` and `play/clans.mjs` in CI.
Nothing under `engine/core`, `engine/cli` or `engine/web` changed; `www/assets/` is untouched.

## Checks

- `node www/play/clans.mjs`: passes. Lineup: 16 of 16 clans drawn once: four hearths (the three civs' own clans on
  their capitals, and the shared home's first), eleven tents, one walker (the shared home's third). Every cloth nearest its own civ's colour (or the pale), tinted above zero. The starving clan has 92 red
  pixels in the ring round its cell with the edge, 0 without. The shared home's four clans are at four places, each
  anchor on the home's tile. A click on the home reaches `clan:24`–`clan:27`. Phone k 5: 16 of 16 dots, `stats.atlas`
  false; phone k 6: cells.
- `node www/shots.mjs --clans DIR`: exits 0. Every clan's `nearest` is its own civ (or `none`), every kind is its
  state's. Kinds per view (`--clans` output):

  | View | Kinds |
  |---|---|
  | medium9-clans-play | tent 115, hearth 3, walk 3 |
  | medium9-clans-close | tent 43, walk 2, hearth 1 |
  | medium9-clans-edge | tent 192, hearth 6, walk 3 |
  | medium9-clans-flat | dot 201 |
  | small2-clans-play | tent 119, hearth 5, walk 2 |
  | small2-clans-close | tent 64, hearth 3, walk 1 |
  | medium8-clans-play | tent 129, walk 4, hearth 5 |
  | medium8-lean-play | tent 105, walk 2, hearth 9 |
  | small9-lean-close | tent 54, walk 1, hearth 1 |
  | clans-lineup-close / -play / -edge | hearth 4, tent 11, walk 1 |

- `node www/play/towns.mjs`, `goods.mjs`: pass. `towns.mjs` now reads `clanProbe()` (see below).
- `node www/play/all.mjs` and `engine/check.sh` whole: see the end of this file.

## Digests and cameras

Every view's camera matches its spec (`proto/scenes.txt`, `proto/lineup.mjs`). The digests do not match the spec's
recorded values, on either tree. On this runner the branch's views and main (f5ea782, a worktree with the branch's
`shots.mjs` copied in) give the same digest for every view they share:

| View | Branch | Main (f5ea782) | Spec's (`proto/clans.txt`) |
|---|---|---|---|
| medium9 +16 (play, close, edge, flat) | eb3b37678eeed1ca | eb3b37678eeed1ca | 68a2c856313bf378 |
| small2 +16 (play, close) | 919c83c12c830549 | 919c83c12c830549 | 0a19bbf2eb1dce5c |
| medium8 +16 (play, lean) | 1ff941beb36c4a0e | 1ff941beb36c4a0e | f2bcde40e4686b25 |
| small9 +4 (lean-close) | 1567d510b5fe7188 | — | a742b8a68f9a8081 (+8) |
| clan lineup (states) | ed6628347def86e3 | ed6628347def86e3 | 4157b18e28b9bd5c |

The game itself differs from the spec's runs on this runner: the pick comes at a different turn (small 9 at 249, the
spec says 226; medium 9 at 294, the spec says 467), and small 9's led game is over at 254, five turns after its pick.
So `small9-lean-close` is four turns (it was eight, which this runner can't reach), and the lean views show no leaning
clan on this runner: none on medium 8 at +16, none on small 2 at +16, none near tile 417 on small 9 at any turn before
its end. The lineup stages its own leaning clans, so the disc is checked there. The spec's digests should be rechecked
on a runner that reproduces them; the health claim (the same digest on both trees) holds here.

## Pixels against main

`node www/shots.mjs --diff` (main's captures from the worktree, the branch's from `--out`):

| View | Share of pixels that differ |
|---|---|
| medium9-clans-flat (phone k 5, flat) | 0% (identical) |
| medium9-clans-edge | 10.07% |
| medium9-clans-play | 3.66% |
| medium9-clans-close | 4.53% |
| small2-clans-play | 4.66% |
| small2-clans-close | 7.81% |
| medium8-clans-play | 4.14% |
| medium8-lean-play | 3.08% |
| clans-lineup-close | 1.15% |
| clans-lineup-play | 0.34% |
| clans-lineup-edge | 0.30% |

The flat view is identical to main; every other difference is the clans' cells and their cloth, the layers at which
clans draw. Cameras match in every case (`--diff` reports no camera or digest difference).

## Decisions and changes this step made

- The probe's `mean` is the mean of each tinted pixel's cloth alone: its colour drawing less its black drawing. The
  first version averaged the colour drawing, and the art under the cloth's soft edge (at phone k 6 most of a cell)
  pulled a blue civ's mean to the pale: that first `--clans` run failed on 20 clans (17 at medium 9's phone k 6, the
  rest on small 2 at k 18 and medium 8 at k 18). Units' probe is unchanged.
- `towns.mjs`'s check counted flat-colour pixels within 24 of each town clan (at least 8): clan 5, the capital's,
  had 6 pixels, because the cloth is shaded. The check now reads `clanProbe()`: each town clan's tinted pixels at
  least 8 and its nearest its own colour. The same claim, measured the way the piece measures it.
- `CLAN.from` is 12 (the terrain's own threshold, `FLAT_BELOW`): a clan is a circle when the cell is under it or the
  terrain is flat.
- A clan's ground is drawn in one order across all homes, by its ground's y (spec: back to front by y), not per home.
- A shared home's clan is `min(CLAN.shared, max(CLAN.minPx / k, CLAN.per · √people))` tall; a lone clan's is
  `max(CLAN.minPx / k, min(CLAN.max, CLAN.per · √people))`.
- The legend's clan rows are plain CSS shapes, not the atlas cells: a triangle for a tent, a triangle with a fire
  square for a hearth, a narrow block for walkers, a red ring for starving, the pale triangle for no civ. The tamed-grain
  row is renamed "tamed grain" so it isn't read as a clan's hearth.
- `leanings.js` and `clans.js` import each other: `disc` from leanings, `clansFlat` from clans. Both are function
  declarations used only at draw time.

## Files

`www/map/layers/clans.js`, `www/map/layers/leanings.js`, `www/map/layers/units.js` (`chroma` and `hexRgb` exported),
`www/map/view.js` (`stats.clans`, `clanProbe()`), `www/legend.js`, `www/shots.mjs` (12 views, `stage: 'clans'`,
`states`, `--clans`), `www/play-stage.mjs` (`CLANS`, `CLAN_STATES`), `www/play/clans.mjs` (new), `www/play/towns.mjs`,
`engine/CLAUDE.md` (two lines).
