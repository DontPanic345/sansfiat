# #444 Clans on the map

**Question:** none in the first four scores, because no rule moves. This is piece 6 of the illustrated map
(`docs/proposals/map-art.md`). It asks each of these against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web` is empty,
  so every digest, `SAVE_VERSION` (27) and every save stay as main's without a sweep. Every clan view records the
  game's digest, and it matches main's on the same view.
- **A clan's way of life is told by its shape.** On main every clan is a circle; a settled one carries a dark square.
  On the branch a clan that gathers is a tent, a settled clan is a hearth (a tent by a fire with its smoke), and a
  clan that has just arrived at its home is a file of walkers. At play zoom (k 18) and close (k 35) the three read
  as different things, and at the strategic zoom (phone k 6) the tent and the hearth still do.
- **A clan's civ is told by its colour.** On main the circle is filled in the civ's colour, pale for a clan of no civ
  (`page.md`, #352). On the branch the colour is the clan's cloth (the tent's hide, the hearth's tent, the walkers'
  clothes), multiplied over the art's shading as #443's units are, and the cloth of a clan of no civ is the same
  pale. An unaligned clan's leaning keeps main's disc in the colour of the civ it leans to, now on the ground under
  its tent. Measured: every clan's tinted pixels are nearest its own civ's colour, or the pale for a clan of no civ
  (in CI on the lineup, and on every clan view).
- **The signals stay.** A starving clan is edged red. People still read by size. Clans sharing a home all show, in a
  ring round its centre. A click on a shared home still cycles its clans. At the flat zoom (phone k 5) the clans are
  main's circles, pixel for pixel. Trees, rivers, borders, icons, the capital's house, towns and units sit where main
  has them.

The control is main at the branch's base, shot with the same tool: a main worktree with the branch's `shots.mjs`
copied in, as 439 to 443 did.

## What the live model exposes (the inventory)

A clan in `game.view().clans` carries (`web/src/lib.rs`'s `view`, `core/src/clans.rs`'s `Clan`): `home`, `people`,
`civ` (and `model.clanCiv`), `settled` (`settling.rs`: a clan whose field share has stood high enough), `starving`,
`since` (turns since it last moved), `leaning` (an unaligned clan's pull to each civ; `model.tintOf`), `territory`,
`market`. The map draws today (`www/map/layers/`):

- `clans.js`: a circle at home, its area by people (radius `0.16·√people`, at most 0.75), filled in its civ's colour
  or `UNALIGNED` (`#d8d2c4`); a dark square when settled; a red ring when starving. Clans on one home: small circles in
  a ring of radius 0.34 round the centre, at most 0.3 each.
- `leanings.js`, drawn after `clans`: an unaligned clan's disc (radius `0.22·√people`, at most 1.4) in the colour of
  the civ it leans to, at an alpha that grows as it nears joining.
- `hearths.js`: the yellow square on each tile whose grain has tamed long enough (`view.hearths`). That is the tile's
  tamed grain, not a clan. The piece leaves it as it is.
- `legend.js` names the clan marks: the circle, the red ring, the settled square, the civ fill, the leaning tint.

What the retro maps hold (`proto/clans.mjs`, `proto/moves.mjs`, `proto/clans.txt`):

- At the pick every clan has its own home, and the only settled clans are the civs' own, under the capitals' houses.
  By +16 turns, 6 to 44 clans have settled off the capitals, 4 to 24 homes are shared (up to four clans), and some
  shared homes hold a settled clan beside gathering ones. No home holds clans of two civs.
- **Moving.** Moving on is a jump: `move_on` sets `since` 0 and `eat` adds 1 the same turn, so a clan that moved on
  reads `since` 1 in that turn's view. A daughter sent off is born at `since` 0, after `eat`. A half from a split
  keeps its mother's `since`, and a home that drifts a tile with its territory keeps it too (most home changes are
  drifts, at `since` 100 to 400: nobody walked). So a clan "on the move" is one at `since ≤ 1`: it came to this home
  this turn (a daughter, this turn or last). 0 to 10 per map per turn; enough for real views.
- Starving clans: 0 to 29 per map. Clans of no civ: most of every map. Leaning clans: a few by +8.

Callers: only `map/view.js` draws `clans.js` and `leanings.js` (`LAYERS`). `play/towns.mjs` counts pixels within 24 of
each town clan's flat colour (`UNALIGNED` imported from `clans.js`). `play/goods.mjs` turns the `clans` layer off.
Selection is by tile (`screens/play.js`'s `candidates`, a click cycling what is on the tile, clans among them), so
the drawing can't move it. `view.focus` (a selected clan's territory bright) is the territories' layer, not this one.

## What the piece delivers

1. **The clan cells** (build-1). The actor study's second row becomes three cells, each with a mask:
   - `clan.tent`: the hide tent (study box about (79, 355) 372×194 holds the tent, a drying rack, baskets and a
     barrel; the tent alone is about x 79 to 330). A clan that gathers.
   - `clan.hearth`: the left tent, the seated man and the fire with its pot and smoke (the blob at (1172, 339)
     279×214; the right tent and the woman are the blob at (1408, 400) 182×147). A settled clan.
   - `clan.walk`: the man, the woman and the child walking (boxes (617, 386) 111×148, (732, 393) 80×144,
     (813, 443) 53×94; the donkey is (874, 407) 162×130). A clan on the move.
   - `clan.<kind>.mask`: same size and anchor as its cell, transparent except the cloth, in grey (#443's `cloth`
     polygons and `{ op: 'grey', lo }`).

   The row's art has a soft fringe the top row lacks: 17,000 to 25,000 pixels of alpha 1 to 127 round each group,
   some of them red (`proto/boxes.mjs`). `cut.mjs` gains a floor: source samples under an alpha count as clear.
2. **The clans on the map** (build-2). `clans.js` draws each clan's cell by its state, with its mask multiplied in its
   colour, above the flat zoom; main's circles at it. `leanings.js` draws at the flat zoom only, and `clans.js` draws
   the leaning discs under the cells above it. The legend's clan rows name the new marks.
3. **The views and checks** (build-2). `shots.mjs` gains the clan views below, a `states` option for the lineup, and
   `--clans`. `play/clans.mjs` checks the lineup in CI.
4. **The look, judged** (build-3): the cuts, the cloth, the floor, the sizes, the ring, the threshold, on the views
   beside main.

## How the clans are drawn

In world units (a hex has circumradius 1). Every number is in `CLAN`, a table at the top of
`www/map/layers/clans.js`, a `note` beside each. Build-2 starts it at the values below; build-3 tunes it.

- **The cells.** `clan.tent`, `clan.hearth`, `clan.walk` and their masks are 112×96, anchor (56, 90): the middle of
  the ground the group stands on. Each group is scaled to fit about 108×88. A mask has its cell's size and anchor.
- **The kind.** `since ≤ 1`: `walk`. Else `settled`: `hearth`. Else `tent`. (A settled clan that moved on keeps
  `settled`, `settling.rs` says; it walks this turn and shows its hearth the next.)
- **The size.** A lone clan's cell is drawn `max(CLAN.minPx / k, min(CLAN.max, CLAN.per · √people))` world units
  tall (`per` 0.42, `max` 1.5, `minPx` 10 css px), its width in the cell's proportion. So 5 people stand 0.94 tall,
  13 or more 1.5. At phone k 6 the floor wins for every clan (`10 / 6` = 1.67 > `max`): every clan stands 10 css px,
  a little wider than its hex (10.4 px), and size doesn't tell people there. Build-3 judges that with `from`. Pre-scaled to the device pixel (`atlas.scaled` and `atlas.shaded`, as `units.js` draws).
- **The place.** A lone clan's anchor is at `(px, py + CLAN.feet)` (`feet` 0.4), so the group's middle sits near the
  tile's centre, where main's circle is. Clans on one home stand on a ring of radius `CLAN.ring` (0.42) round the
  centre, in id order from the top as main's, each anchor at its ring point plus `(0, CLAN.feet)` as a lone clan's is
  (so the four anchors lie between 0.02 above the centre and 0.82 below it, inside the hex), each at most `CLAN.shared` (0.8) tall, drawn back to front (by their
  y), so a lower one overlaps an upper one and none hides another whole.
- **The colour.** The mask, pre-scaled, multiplied (`atlas.shaded`) by the civ's colour, or by `UNALIGNED` for a clan
  of no civ. The cell's own art shows outside the cloth.
- **The leaning.** Above the flat zoom, `clans.js` draws every on-screen leaning disc first (main's shape, radius
  and alpha, from a function `leanings.js` exports), then the cells. So the disc lies on the ground under the tent.
  At the flat zoom `leanings.js` draws as on main and `clans.js` draws none.
- **Starving.** The cell's outline (`atlas.outline`) in `#ff2a2a`, `CLAN.starving` (3.5 css px) wide, drawn under the
  cell, as #443's hungry unit.
- **The flat zoom.** Where the terrain is flat (`isFlat(cellHeight(view), view)` from `terrain.js`, as `woods.js`
  reads it), `clans.js` draws main's circles by main's code, unchanged. `CLAN.from` holds the threshold. It starts at
  the terrain's own, and build-3 may raise it to a minimum cell height in device px if the cells don't read at k 6.
- **What it reports.** `view.stats.clans` lists each clan drawn: `{ clan, civ, kind, x, y, w, h, starving }` in
  canvas px, `kind` one of `tent`, `hearth`, `walk`, or `dot` at the flat zoom (`civ` null for none). `MapView.clanProbe()`
  draws each clan alone (its cell and mask, no disc, no outline) on transparent canvases, once in its colour and once
  with every colour black, and returns per clan its rectangle, `opaque`, `tinted`, `mean`, and `nearest`: of the
  living civs' colours and `UNALIGNED`, the one nearest `mean` by chromaticity (`units.js`'s `chroma`), as a civ id or
  `'none'`. A circle at the flat zoom probes the same way.

## Shape: Easel and Canvas

The cells' keys and sizes, the kind rule, the colour rule, the leaning disc, the flat zoom, the drawing order, the
probe, the views and the checks are settled here to the rule. The first cuts need only be in place and read as a
tent, a hearth and walkers. How the clans look and read is the piece, judged by eye on real maps, so it belongs to one
canvas. This is 443's shape, cut by area as 443's was: the atlas, then the map.

- **build-1, easel (the atlas):** the floor in `cut.mjs`, the three regions and six cells, first cuts and cloth,
  `--check` holding them, the evidence.
- **build-2, easel (the map):** `clans.js`'s cells, the leaning disc and the flat path, the stats and probe, the
  legend, the views and `--clans` in `shots.mjs`, the lineup in `play-stage.mjs`, and `play/clans.mjs` in CI.
- **build-3, canvas:** how the clans look and read: `CLAN`'s numbers, the cuts, the cloth and grey, the floor, the
  captures on main and the branch, and the evidence.

## Settled here, by running

- **The inventory** above (`proto/clans.mjs` and `proto/moves.mjs` on the retro maps; `proto/clans.txt`).
- **What moving is** (`proto/moves.mjs`): `since ≤ 1`, from the turn order and the born and moved rows.
- **The views' cameras** (`proto/scene.mjs`, `proto/scenes.txt`): each real view below set on main and read back
  unclamped, with its digest. Main's captures of them are what build-3 compares against.
- **The lineup** (`proto/lineup.mjs`): a built map with clans of red, blue, gold and of no civ on plains and in a
  wood, a home shared by four, and a clan of no civ on gold's units' tile. It stages on main with no error, digest
  `4157b18e28b9bd5c`. The stage can't set `settled`, `since`, `starving` or `leaning`, and every staged clan is at
  `since` 0, so the lineup's states are set on the drawn model (`game.map.model.v.clans`), as #443's `bow.mjs` set a
  stack: the drawing reads the state, and the real views prove the game makes it.
- **The source** (`proto/boxes.mjs`, `proto/crop.mjs`): the boxes above at alpha ≥ 128, and the soft fringe.
- **The open idea** "The unit bound at the flat zoom" is already answered: #443's build-3 `units.txt` shows
  `medium9-units-flat`'s scouts at 65 tinted pixels, above the bar, and `--units` exits 0. Not taken.

## Decisions I made

- **Three kinds: tent, hearth, walkers.** They are the three distinctions the live model makes about how a clan
  lives: `settled`, `since`, neither. The study's hamlet, palisade and civic groups are towns and capitals (piece 7).
  The walkers' donkey, the tent's rack and barrel, and the hearth's second tent are left to build-3's eye.
- **The keys are `clan.tent`, `clan.hearth`, `clan.walk`.** The page calls an army a camp (`page.md`; `unit.camp`),
  so the clan's tent is not named a camp anywhere the player or the code reads.
- **A clan is on the move at `since ≤ 1`.** That is what arriving is in the model. A home drifting with its territory
  is not a move and draws no walkers.
- **Walkers before the hearth.** A settled clan that moved on shows as walkers that turn: what happened this turn
  reads over a way of life that hasn't changed.
- **The civ's colour is the clan's cloth, and a clan of no civ's cloth is `UNALIGNED`.** It keeps `page.md`'s "a clan
  in its civ's colour, pale for a clan of no civ" on the new mark, and the multiply keeps the art's shading, as
  #443's units.
- **The leaning stays main's disc, under the tent.** Dyed cloth says a clan is a civ's; a disc on the ground says it
  leans. Mixing the leaning into the cloth would make a clan about to join look like one that has.
- **Size by people stays,** as main's circle area does, within a range so a big clan doesn't fill its hex.
- **Shared homes keep main's ring,** drawn back to front.
- **The flat zoom keeps main's circles,** as the woods and ridges leave the flat zoom to the flat fill. At 10 px a hex,
  a 6 px tent is a speck; a circle is not.
- **The tamed-grain hearth mark (`hearths.js`) stays.** It marks a tile, not a clan. The "hearth" in this piece is a
  settled clan's.
- **The legend's clan rows change to the new marks.** That is the legend telling the truth, not a HUD redesign.

## Unsettled, and which step holds it

- **How the clans look and read** (build-3, the canvas):
  - the cuts (keep polygons, scale, anchors, what of each group stays), the floor;
  - the cloth: which pixels each mask keeps, and the grey's floor;
  - `CLAN`: `per`, `max`, `minPx`, `feet`, `ring`, `shared`, `starving`, `from`.

  Proof: the captures of the views below, main beside the branch, read by eye; `shots.mjs --clans` on every clan view
  (every clan nearest its own colour, every kind the state's, the tinted pixels per clan at k 18 and phone k 6 above
  the bar build-3 sets); `--diff` (cameras and digests match, and the flat view differs from main in no pixel); and
  `play/clans.mjs`. All in `relay/specs/444/views.md`.

## Ideas taken

None. The one open idea is already answered by #443 (above).

## The views

`www/shots.mjs` keeps its views and adds those below, each with `clans: true`. New option: `states: true` (the lineup
only) sets the lineup's states on the drawn model after staging (`CLAN_STATES` below). The real views use the
existing `turns` and `on: 'at'`. Laptop is 1280×800 and phone 390×844.

| View | Map | Window | k | Camera | What it shows |
|---|---|---|---|---|---|
| `medium9-clans-play` | medium 9, 16 turns | laptop | 18 | between 734, 874 | Gold's land: the town at 734 (two settled, two gathering), walkers at 874 and 919, tents in the woods, the capital at 780; clans of no civ |
| `medium9-clans-close` | medium 9, 16 turns | laptop | 35 | between 734, 874 | the same, close |
| `medium9-clans-edge` | medium 9, 16 turns | phone | 6 | between 734, 874 | strategic zoom |
| `medium9-clans-flat` | medium 9, 16 turns | phone | 5 | between 734, 874 | the flat zoom: main's circles |
| `small2-clans-play` | small 2, 16 turns | laptop | 18 | between 410, 503 | Red's settled clans on plains (410, 443, 502), the mixed home 503, three at 475, the forest town 505, starving clans |
| `small2-clans-close` | small 2, 16 turns | laptop | 35 | at 503 | a settled clan and a gathering one on one home, close |
| `medium8-clans-play` | medium 8, 16 turns | laptop | 18 | at 401 | two settled clans of no civ in the desert; walkers at 725, 771, 200, 679 |
| `medium8-lean-play` | medium 8, 16 turns | laptop | 18 | at 1368 | a clan of no civ leaning to Red |
| `small9-lean-close` | small 9, 8 turns | laptop | 35 | at 417 | a clan of no civ leaning to Gold at 0.57 |
| `clans-lineup-close` | the clan lineup, states | laptop | 35 | between 93, 170 | each civ's tent and none's on plains and in a wood; the shared home; the starving, walking, leaning clans; the clan beside gold's units |
| `clans-lineup-play` | the same | laptop | 18 | between 93, 170 | the same at play zoom |
| `clans-lineup-edge` | the same | phone | 6 | between 93, 170 | the same at strategic zoom |

The lineup, `CLANS` (`proto/lineup.mjs`): radius 8, sea on the rim, plains with the forest block of #443's lineup
(q 3 to 5, r −2 to 1). Civs on marks A (−5, 0) red led, B (0, −5) blue, C (0, 5) gold, 12 people. Gold's army (4) and
scout on mark c (1, 1). Clans, each holding only its home: red, blue, gold and none on plains at (0, 2), (1, 2), (2, 2),
(3, 2); the same in the wood at (3, −1), (4, −1), (5, −1), (4, 0); a shared home at (−1, 2) of red 9 (holding the
tile), red 5, red 3 and blue 4; a clan of no civ (4) on c. People 6 unless named. `CLAN_STATES`: every clan at
`since` 5, then on the shared home its first clan `settled` and its third at `since` 0; gold's plains clan
`starving`; the plains clan of no civ leaning to blue at 0.6 of `rules.join`, the wood's to gold at 0.2. The staged
clans' tiles: shared home 93, plains 110, 127, 143, 158, wood 155, 169, 182, 170, gold's units 126.

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty, and `SAVE_VERSION` is 27. On both trees,
  every clan view's `digest` in `views.json` is the same.
- Run `node www/assets/draw-atlas.mjs` twice: the sha256 is the same both times and as committed, and `--check`
  passes. Every key of main's `atlas.json` is still there, and every one keeps main's pixels (the six clan cells are
  new).
- `node www/play/clans.mjs` (in CI) on the lineup with its states:
  - one cell per clan on screen, each of its state's kind: `walk` for the shared home's third clan, `hearth` for its
    first and for the three civs' own clans (7, 8, 9, staged settled on the capitals 33, 103, 113, on screen under the
    capitals' houses), `tent` for the rest;
  - every clan's `nearest` in `clanProbe()` is its own civ, or `none` for a clan of no civ (the leaning ones too), with
    `tinted` above zero;
  - the starving clan is marked `starving` in `stats.clans`, and red pixels (`#ff2a2a` within 24) lie round its cell;
  - the shared home's four cells stand at four different places, each anchor inside the home's hex;
  - clicking the shared home's tile reaches each of its four clans (`data().selected` `clan:<id>`), as on main;
  - at the flat zoom (the lineup on a phone at k 5: on a laptop its `minK` is 7.47) every entry of `stats.clans` is
    a `dot`.
- `node www/play/all.mjs` passes, `towns.mjs`, `goods.mjs`, `units.mjs` and `views.mjs` among them.
- Make a main worktree with the branch's `shots.mjs`, shoot every clan view on both and `--diff`: cameras and digests
  match, and `medium9-clans-flat` differs in no pixel. Run `--clans` on the branch. Run `--time medium9-clans-edge`
  on both trees and record the two medians (185 clans on screen). Then read the pairs:
  - tent, hearth and walkers read as three things at k 35 and k 18, and tent and hearth at phone k 6;
  - in `clans-lineup-*`, red's, blue's, gold's and none's clans are told apart on plains and in the wood; the leaning
    clans read as unaligned with a pull, not as a civ's;
  - in `medium9-clans-play`, the town at 734 shows its four clans, two hearths and two tents, beside the capital,
    its ring, the town mark and Gold's units;
  - `small2-clans-*` shows the starving clans' red edges and the settled clans as hearths;
  - trees, rivers, borders, icons, the capital's house, towns and units sit where main has them.
- On the branch page, click a clan, cycle a shared home, pan and zoom across k 5 to 35. They behave as on main.
- The PR carries main's and the branch's captures of `clans-lineup-close`, `clans-lineup-play`, `clans-lineup-edge`,
  `medium9-clans-play`, `medium9-clans-close`, `small2-clans-close` and `medium8-clans-play`, from the files build-3
  commits under `relay/specs/444/shots/`.
