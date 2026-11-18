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

# #339 build-4: the map and bar scripts move to the stage (moves.md, continued)

Counts are the `need(` and `fail(` calls, as build-3 counted them. The before is the branch's HEAD before this step
(`git show HEAD:engine/www/play/X.mjs`): camera.mjs is not on main's tree (main, 148528b, has no camera.mjs), so its row
counts the branch's copy. No count goes down.

| Script | Before | After | Staged world | Checks whose meaning changed |
|---|---|---|---|---|
| `art.mjs` | 10 | 10 | Radius 6: sea on the rim, coast inside it, the other glyphs cycled over the rest (`.`, `f`, `h`, `^`, `:`, `%`, `r`, `z`, `P`, `V`, `S`, `N`, `=`, `,`), two capitals on plains. | None. The atlas and sound checks run on it as on the pick. |
| `bar.mjs` | 20 | 20 | Radius 8, three civs on plains (the led one first), staged on `''`; the phone copy stages the same spec. The pre-game's counts stay on a seeded pre-game (`?size=small&seed=2`, phase `pregame`). | The pre-game check is the one that reads a seed (below). |
| `buttons.mjs` | 83 | 83 | Its own staged game, as before. Now opens `''`, not `?size=small&seed=2`. | None. |
| `camera.mjs` | 8 | 8 | Radius 8, capitals at (-4, 0) and (4, 0), zoomed in one step off the fit. | None: the zoom is a setup, and the centring check runs on the zoomed map (below). |
| `feed.mjs` | 16 | 16 | Radius 12, capitals at (-4, 0) and (8, 0), a clan of no civ at (-2, 1), a camp of three on the led capital, grain on every tile. | The alert is `cut off from its capital`, not a hungry army: the camp marches out past its line and the alert is the first the stage makes. The clan's item is clan 6's starvation line. |
| `select.mjs` | 23 | 23 | Radius 8, capitals at (-4, 0) and (4, 0), a clan of no civ at (0, 3), a clan of no civ at (0, -3) going to the led civ's market (the festival), a camp of three on the capital. | None. |
| `views.mjs` | 15 | 15 | Radius 12, capitals at (-3, 0) and (3, 0), a river at (0, -2) flowing to (1, -2), a clan at (-2, 1) going to the led civ's festival, grain not set, after one End turn. | The rivers layer needs `Tile.river` and `Tile.flow`, which the glyph `r` does not set: the river is the new `rivers` field (below). The festival routes are drawn from the walks, so the script ends a turn first. |

## The changed checks, and why

- **bar: the pre-game.** `the chips show before the pick` and the counts' tooltips read the seeded pre-game, the only
  check here a stage cannot make (a stage is not a pre-game). That is why `bar` still counts as reading a seed in
  `recount.md`. Every other check runs on the staged game of three civs.
- **camera: the centring.** A panel that changes the map's box refits a map sitting at its fit, and a refit drops a
  centring under way. So the staged map is zoomed in one step (`zoomBy(2)`), and a refit only clamps it. The three
  centring checks then run as they did, on the zoomed map; the zoom check reads its own `k` before the click.
- **feed: the alert.** The brief named a hungry army as one example; the stage's first alert is the camp cut off from its
  capital after it marches out (`cut off from its capital`). The check is `an alert shows in the alerts box`, which
  the cut-off alert satisfies, and the three-turns-on and click checks run on it unchanged.
- **views: the rivers layer.** `rivers` is a setup field (`RiverSpec`, applied after the tame counts, with a case in
  `core/tests/stage.rs`), not a rule: `Tile.river` 1 and `Tile.flow` set as the carve step sets them. No `SAVE_VERSION`
  bump, no digest change.

## Engine and setup

- `StageSpec` gains `rivers: [{at, flow}]` (`RiverSpec`), beside `tame`; `core/tests/stage.rs` checks a river tile's
  `river` and `flow` in the sources test. This is the one field build-4 needed.
- `engine/www/play/*.mjs` that stage: art, bar, buttons, camera, feed, select and views now open `''` (bar's pre-game
  excepted), and none of the seven leads a seed; `grep -n "seed" www/play/{art,camera,feed,select,views}.mjs` finds
  nothing but comments.
- `engine/www/pkg` was rebuilt (`build-web.sh`) for the `rivers` field; the page reads the wasm.
