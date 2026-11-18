# #339 build-3: the page scripts move to the stage (moves.md)

Counts are the `need(` and `fail(` calls in each script, on main (before) and on this branch (after). None of the seven
opens a seeded page or leads a seed now: `grep -n seed www/play/{army,camp,civ,fields,goods,ring,scout}.mjs` finds nothing.

| Script | Before | After | Staged world | Checks whose meaning changed |
|---|---|---|---|---|
| `army.mjs` | 26 | 26 | Radius 8, forest at (-4,-1) and (-3,0), the led civ's capital at A (-4,0), its store 400, forty people a civ, grain 200 on every tile, a rival army of three at E (1,0). | The skirmish click (below). |
| `camp.mjs` | 41 | 43 | Panel and line home: radius 8 with a sea wall down q = 1, capital A (-7,0), store 400, grain on the land within three steps of A. The page paints X (-5,0) and Z (-5,1) into the claim. Hold: the same wall with a gap at (1,0), a rival camp of three on its post P (3,0), grain on every tile. | See below. +2 calls: `the claim takes the tiles painted into it` (in `paintClaim`) and the hold's `fail` in its catch. |
| `civ.mjs` | 23 | 23 | Radius 8, capitals at (-4,0) and (4,0), the led civ's store 50, an unaligned clan at K (0,3) whose market is the led civ's. | None. |
| `fields.mjs` | 18 | 18 | Radius 10, capitals at (-8,0) and (8,0), a floodplain P (0,1) and X (0,2) tamed to 100, the led civ's clan of twelve on P, 30 grain on P. | The farming clan. See below. |
| `goods.mjs` | 6 | 6 | Radius 4, capitals at (-2,0) and (2,0), salt at (0,-1), obsidian at (0,1), store 10. | None. |
| `ring.mjs` | 12 | 12 | Radius 8, capitals at (-4,0) and (4,0), an army of three on the capital, a clan of four at P (0,3). | None: the army is staged, not raised (`raiseArmy` took forty turns of store fill). |
| `scout.mjs` | 34 | 34 | Radius 8, capitals at (-6,0) and (6,0), forest at (-3,0) (mark T, the march's target), (-3,1), (-2,-1), (-5,0), hills at (-1,0) and (1,1), a scout of each civ with 30 grain. The second game puts a mountain at (0,0) with hidden trail cost 9. | The march's target (below). |
| `settle.mjs` | 19 | 19 | Its own staged game, moved onto `play-stage.mjs`. | None. |
| `towns.mjs` | 10 | 10 | Its own staged game, moved onto `play-stage.mjs`. | None. |
| `buttons.mjs` | 83 | 83 | Its staged game, its `gridText` now `grid`. Still opens `?size=small&seed=2` (build-4 opens `''`). | None. |

## The changed checks, and why

- **army: the skirmish click.** The feed's skirmish names the fight's first army, the lowest id present, which here is
  the rival's camp (it was staged first); the army that marched is not that id. So the click selects `unit:<first army>`
  (checked as `a click on the skirmish selects the fight's first army N`), and the script then selects the marching army
  by its tile before its panel checks. The skirmish is found by the army's tile (at turn start, or now) and the led civ,
  not by `contested` and the army's id. Why: the AI's camp roams off its post on turn one and comes back, and an army of
  ours that marches to it meets it on a tile where the feed names the rival; the page's behaviour is the same, only the
  check's words followed the engine's naming.
- **camp: the hold.** It runs and passes. In the staged game the rival camp fights our camp on the post: the check's
  `or` reads `cut false, hungry false, fought true`, so the hold passes on the fight. Its raise, its march and its ring
  are checked as before. The civ's home keeps five warriors raised from its panel, because a capital with no army of its
  civ falls to a rival's, and the staged rival does attack.
- **camp: the reasons.** `the camp lists its reasons` now comes after one turn: a staged camp has no reasons before a
  turn's notes are written.
- **fields: the farming clan.** The clan the script checks is the first of the led civ with a farmer's share (at least
  `field.farmer`), a laid field tile and a `clanFarming` feed line. The spec starts the share at none, so the notch
  emits the line on its rise (a clan staged with a share never emits it). The fields check now passes by turn four.
  Why: `clanFarming` is emitted only on a rise from none.
- **scout: the march's target.** The first forest or hills tile seven steps out is replaced by the forest mark T, three
  steps out, as the staged map has no tile at seven. The rest is as before.

## Engine and setup

- `StageSpec` gains three setup fields, each applied in `from_spec` and tested in `core/tests/stage.rs`: `sources`
  (goods by name or index, as `sources::place` sets `Tile.source`), `tame` (a tile's `Tile.tame`, the tending that makes
  its grain yield) and `ClanSpec.fields` (the clan's share of days to fields). `tame` is not in the brief's list: the
  fields script needs a field tile that yields, and a tile yields only once tamed.
- No `SAVE_VERSION` bump, no digest change (`engine/check.sh` passes, digest pins unchanged).
- `www/play-stage.mjs` holds `grid`, `plains`, `marks` and `stage`; `settle`, `towns` and `buttons` use it, `buttons`
  as `stageOn` because it has its own `stage` helper.
