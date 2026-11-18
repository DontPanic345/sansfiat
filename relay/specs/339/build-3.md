# #339 build-3 (ford): the page scripts of units and the civ move to the stage

Read `relay/specs/339/spec.md` (7) and `count.md`. Seven scripts in `engine/www/play/` read a
seeded pick though their point is a unit or the civ's panel: **army, camp, civ, fields, goods,
ring, scout**. Each moves to a staged game through `game.stage(spec)`, as `settle.mjs`,
`towns.mjs` and `buttons.mjs` already do. build-4 moves the map and bar scripts the same way
on what you leave.

## The rule for each move

- The script opens `open('')` and stages its world; no `?size=…&seed=`, no `lead(page, {seed})`.
- **Every check it makes now, it still makes**, with the same words: a `need`/`fail` stays a
  `need`/`fail`, never an `if`. A check that held only because of a seed's luck (the store
  filling on seed 6, a clan that happens to farm on seed 1) holds because the stage puts it
  there. The header comment says what the stage is, not the seed.
- Turns played fall to what the claim needs: a staged world needs no 40 turns of waiting for
  a store to fill when the spec gives it a store.
- Run each script on its own as you move it, and once more under `all.mjs` at the end.

## Shared helpers: `www/play-stage.mjs`

`grid(R, cell)` (the map-text builder `settle.mjs`, `towns.mjs` and `buttons.mjs` each copy),
`plains(R)` (sea on the ring, plains inside), `marks(R, {A: [q, r], …})`, and
`stage(page, spec)` (stage, wait for `phase === 'main'`, two frames). Move `settle.mjs`,
`towns.mjs` and `buttons.mjs` onto it; their checks don't change.

It lives in `www/`, beside `play.mjs`, not in `www/play/`: `all.mjs` runs every `.mjs` in
`www/play/` as a script, and the recount counts them.

## What the stage needs, and where it grows

`StageSpec` (`core/src/stage_spec.rs`, from build-1) already has the map text, marks, hidden
costs, food, civs, lead, armies (on the civ's camp), clans (with `civ` and `market`),
signposts, `scouts` and `stores`. Add only what a script needs, each as a field documented
beside the others, applied in `from_spec`, with a `core/tests/stage.rs` case each:

- **goods**: `sources: [{at, good}]` (salt or obsidian by name or index), set as
  `sources::place` sets `Tile.source`.
- **fields**: a clan's `fields` share (`ClanSpec.fields`, the share of its days to fields,
  `Clan.fields`), so a staged farmer farms from the first turn (the farming shows as the
  script's checks need: the clanFarming feed item may need one turn).
- **camp's hold**: a rival army at a post is `armies` with a rival civ; the hold's march and
  cut run on it. The hold is no longer skipped: it runs and must pass. If the cut needs a
  setting no field gives (a zone, a line), add the field, not a `world_mut` back door.
- **civ, army, ring, scout**: `stores` gives the store; a clan of no civ at the civ's market
  (`ClanSpec.market`) gives the clan that leans to a gift; terrain glyphs in the map give the
  forest and hills a scout routes through and the mountain it leaves unrevealed (hidden costs
  as `hidden`).

A field is setup, never a rule: no `SAVE_VERSION` bump, no digest change.

## Done

- The seven scripts and the three moved onto `play-stage.mjs` pass alone and under
  `node www/play/all.mjs`; none of the seven opens a seeded page or leads a seed
  (`grep -n "seed" www/play/{army,camp,civ,fields,goods,ring,scout}.mjs` finds none but in
  comments, if those).
- `camp.mjs`'s hold runs and passes.
- In `relay/specs/339/moves.md`, a row per script: its checks before and after, counted (the
  `need`/`fail` calls), and any check whose meaning changed, with why. The count after is never
  lower without a line saying which check went and why it was never the script's point.
- `engine/check.sh` passes; digest pins and `SAVE_VERSION` unchanged. `engine/CLAUDE.md`'s
  `www/play/` row says the scripts stage their worlds through `../play-stage.mjs`.

About 12 files, 600 changed lines: seven scripts (~40-80 each of setup replaced),
`play-stage.mjs` (~60), three scripts onto it (~-60), `stage_spec.rs` (~40), `core/tests/stage.rs`
(~40), `moves.md`.
