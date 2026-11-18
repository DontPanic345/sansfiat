# #338 build-1 (easel): the live driver, the click's camera, the gift line, the words

Read `relay/specs/338/spec.md` (What build-1 settles) and `probe.md`. build-2 plays the page
through what you build here, as a player, for 100 turns, then fixes what it finds: build the
driver for that session first, and make it quick to use and honest about what the page shows.

## 1. The live driver: `engine/www/play/live.mjs`

One page held across many shell calls. Built on `open` from `www/play.mjs` (reuse its clicks,
keys, `panel()`, `stepper()`, `tip()`, `shot()`; don't copy them).

- `node www/play/live.mjs start [QUERY] [--dir D] [--size laptop|wide|phone]` opens the page
  (default query `?size=small&seed=2&run=1`), listens on a local HTTP port, writes the port to
  `D/port` (default `D` is `/tmp/sansfiat-live`), and stays up until `stop`. A session runs it
  in the background.
- `node www/play/live.mjs CMD ARGS…` sends one command to it and prints the result, then exits.
  Commands, each a player's touch point:
  - `look`: nothing done; print the state.
  - `lead NAME` (a pick card, or `watch`), `click T`, `right T` (the selected unit's order),
    `drag T1 T2`, `key NAME` (`Space`, `p`, `u`, `h`, `Escape`), `button LABEL` (the first
    enabled visible button starting with it), `stepper LABEL N [OPTION] [go]`, `tip T|SELECTOR`.
  - A tile `T` is written as the page writes it, `q,r` (the panel's and the tooltip's
    coordinates), or as a tile index (the text view's), or `@x,y` in a screenshot's pixels at the
    scale that screenshot was taken.
  - `end [N]`: End turn up to N times (1 by default), stopping early on a new alert or the end
    screen, and saying where it stopped and why.
  - `shot NAME [SCALE]`: a PNG into `D` (half size by default), its path printed so a session can
    read it as an image.
  - `save FILE`: the game as the page saves it (`Sim.save()`, through the Game menu's Save to
    file if that works headless, else `game.save()`), a file `sansfiat judge FILE` reads.
  - `load FILE`: the start screen's load a file (`play.mjs` `file('#load', …)`), for a saved
    game.
  - `stop`.
- After every command it prints, compactly, what a player sees: the turn and phase, the top
  bar's Standing, the panel (name, doing line, numbers as `label value`, buttons with greyed ones
  and their tooltips, reasons, refusal, hint), the feed items new since the last command, any
  open alert, and page errors. Nothing from `game.view()` the page doesn't draw.
- Every command and its printout go as one JSON line to `D/live.lines.jsonl`, so a game's
  repro is that file and its turns.
- The header comment says all of this in a few lines, with the background start for a Relay
  session (`run_in_background`) and the judge call. Add one line to `engine/CLAUDE.md`'s table
  row for `www/play.mjs` naming `live.mjs` (keep the file under 8 KB).

Proof: a short session through the CLI only, start to stop: lead on small 61288, click the
capital, click it again, `stepper` a raise, `end 10`, a shot, `save`, `sansfiat judge` on the
save. Put the commands and the trimmed outputs in `relay/specs/338/live.md`. No test in
`all.mjs` for the driver: the scripts already cover `play.mjs`.

## 2. A click centres the camera

In `www/screens/play.js` `onPick`, a left click on a tile (`i >= 0`), armed or not, eases the
view to that tile's middle with the zoom unchanged: `view.centreOn(i, { smooth: true })` with no
`minK`. Drags, paint strokes, right-clicks and clicks off the map don't move it.

Proof: a play script `www/play/camera.mjs`, added to `all.mjs`: click a tile that is off the
centre and check the map's centre (`game.map.cx`, `cy` after `!game.map.anim`) is that tile's
middle (`game.map.px[i]`, `py[i]`) to a pixel; click the same tile again with two things on it
(the capital with its scout, at the pick) and check the selection changed and the centre didn't.
It fails on main. `clampView` holds the centre inside the map's extent, so pick a tile whose
middle the view can centre on (not on the rim). Other scripts that read screen positions after a click may need to wait on
`!game.map.anim` (`play.mjs` `tile()` already does); fix any that break.

## 3. The gift line

- The web civ view (`web/src/lib.rs`, ~line 181) gains `gift_reach`: the number of clans not the
  civ's own whose market is now its capital market (`c.market`, the field the text view reads at
  `cli/src/view.rs` ~line 174). Derived, no saved field: the digest and `SAVE_VERSION` must not
  move. Put the count in one core function (`civs::gift_reach(w, civ)`) both front ends call.
  Document the field with the others.
- The civ's panel (`www/panels.js` `civShape`, ~line 448): the Gift row reads
  `6.0 a turn · reaches 3 clans · gave 4.2 · left 1.8`, where left is `gift − given` (never
  below 0). With no clan reached it reads `reaches no clan`, and the tooltip says why: no clan at
  your market is not already yours, and a gift reaches only clans that trade there. When it
  reaches clans and leaves some, the tooltip says the store held less than keep plus gift (the
  pool is `gift.min(store − keep)`, `markets.rs` `trade`). Under the AI the gift reaches no clan
  on either small map (`probe-review.md`), so the "reaches no clan" form is the one players read:
  make it plain. The "Last
  turn" row drops `given` (it's in the Gift row now). `panels.js` is 570 lines: if this takes it
  past 600, move the civ's shape to `www/panels-civ.js`.
- The text view (`cli/src/view.rs` ~line 110): the same three numbers in the civ's line:
  `gift 6.0 a turn reaches 3 clans: gave 4.2, left 1.8` (or `reaches no clan at your market:
  gave 0.0, left 6.0`). `view.rs` is 546 lines: if it passes 600, move the civ's lines to
  `cli/src/view_civ.rs`.

Proof: a `check.mjs` (or `check-stage.mjs`) case on a staged `Sim` with a clan not the civ's at
its market and one without, reading `gift_reach` and the panel's Gift row; a `cli` test or a
`core/tests` test for `gift_reach` on a stage, whichever pattern is nearer. Then the real map:
a copy of `relay/specs/336/play/small-2.json` (whose turns 285-377 read "gave 0.0"), opened in
`sansfiat play --game COPY` at its last turn (never write into `relay/specs/336/`), and the same
file loaded in the page through the driver: record both new lines in `live.md`.

## 4. The words

- `core/src/claim.rs` `check` (~line 79): the refusal names the first tile of `tiles` not in the
  connected part: `a claim must be joined to the capital or a settlement: (q, r) is not`, with the
  tile written by `World::at`, as `claim::change` writes its tiles. A core test asserts the tile is named.
- `cli/src/view.rs` legal orders, the scout's route (~line 527): `to: any tile it may route to,
  such as …` with the near list as the examples.
- The keep's default label, in the panel (`civShape`, the Keep row) and the text view: say the
  default is the camp's full pack, as now, and add "now" with the figure, so a figure that moves
  with the camp reads as the default moving: `45.0 (not set: the default, a full pack for its
  camps)`. Exact words yours.

## Done

`engine/check.sh` passes (it runs `all.mjs`, with `camera.mjs` in it); the digest pins and
`SAVE_VERSION` are unchanged; `live.md` holds the driver's session, the judge's lines and the
small 2 gift line. Your summary tells build-2 how to start the driver and anything awkward in it.

About 8 files, 400 lines with tests: `live.mjs` (~200), `camera.mjs` (~40), `all.mjs`,
`screens/play.js` (~3), `civs.rs` (~15), `web/src/lib.rs` (~3), `panels.js` (~15), `view.rs`
(~15), `claim.rs` (~5), tests (~60), `engine/CLAUDE.md` (1).
