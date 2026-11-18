# #339 build-4 (ford): the page scripts of the map and the bar move to the stage; the recount

Read `relay/specs/339/spec.md` (7), `count.md`, and build-3's summary and `moves.md`. build-3
moved the unit and civ scripts and wrote `www/play-stage.mjs`; follow its rule ("The rule for
each move" in `build-3.md`, copied here):

- The script opens `open('')` and stages its world; no `?size=…&seed=`, no `lead(page, {seed})`.
- Every check it makes now, it still makes, with the same words: a `need`/`fail` stays a
  `need`/`fail`, never an `if`. A check that held because of a seed's luck holds because the
  stage puts it there. The header comment says what the stage is, not the seed.

## The scripts

- **art**: a staged map with every terrain glyph `maptext::parse` reads, so the map draws each
  tile kind from the atlas; the atlas checks don't change.
- **bar**: the chips, End turn, space, Play, speed and the menus on a staged game of three civs
  (laptop and phone). The pre-game's counts need a pre-game, which a stage hasn't: that check
  keeps a seeded pre-game open (`?size=small&seed=2` without `run=1`); if so, `bar` still
  counts as reading a seed, and `recount.md` says that one check is why.
- **buttons**: opens `open('')`, not small 2; it stages already.
- **camera**: a staged map with the capital (its scout on it) off the centre and not on the
  rim; the click and the second click as now.
- **feed**: a staged game whose first turns make feed items and an alert the script clicks
  (a clan of no civ that joins when the claim reaches it, a hungry army: whatever the stage
  can place that makes an alert on turn 1 or 2).
- **select**: a staged map with a clan's home, a festival (a market with clans at it: `clans`
  with `market`), a bare tile, and a stack (the capital with its camp and scout).
- **views**: a staged map with hidden country if `H` needs it (`hidden` costs, or the closed
  glyphs `V`/`S`/`N`); pan, zoom, fit and the layers as now.

Add a `StageSpec` field only where a script can't be staged without one, as build-3 did.

## The recount: `relay/specs/339/recount.md`

`count.md`'s two tables, recounted on the branch with its rule, with build-3's and your
scripts' check counts before and after (append your rows to `moves.md`). The totals, beside
the control: page scripts **19 of 22** on main, and on the branch (the target is 5 of 23);
core files **10 of 45**, and on the branch (the target 8 of 45). Then time
`node www/play/all.mjs` once and give its total seconds beside `allmjs-main.txt`'s.

## Done

The seven scripts pass alone and under `all.mjs`; `recount.md` and `moves.md` are written;
`engine/check.sh` passes; digest pins and `SAVE_VERSION` unchanged.

About 9 files, 450 changed lines: seven scripts (~30-70 each), `stage_spec.rs` if a field
is needed, `recount.md`, `moves.md`.
