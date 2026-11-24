# #445 Towns, cities and capitals on the map

**Question:** none in the first four scores, because no rule moves. This is piece 7 of the illustrated map
(`docs/proposals/map-art.md`). It asks each of these against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web` is empty, so every
  digest, `SAVE_VERSION` and every save stay as main's without a sweep. Every place view records the game's digest, and
  it matches main's on the same view.
- **A place's role is told by its shape.** On main a capital is a house in its civ's colour (`capitals.js`), a
  settlement a smaller plain house (`settle.js`), and a town only a white diamond over its clans (`towns.js`). On the
  branch, at play zoom (k 18) and close (k 35), a town is a hamlet of thatched huts, a settlement a palisade round its
  huts with a flag, and a capital the civic hall on its mound among its houses: three things told apart at a glance.
- **Ownership stays.** A settlement's flag and a capital's banners are cloth in the civ's colour, multiplied over the art's
  shading as #443's units and #444's clans are. Measured: every capital's and settlement's tinted pixels are nearest its
  own civ's colour (in CI on the lineup, and on every place view). A town has no owner in the model (it is a festival
  that clans crowd), so the hamlet carries no colour; the land border round it says whose land it stands in, as on main.
- **The markers stay.** The town diamond, the gold centre ring round each capital's festival, the land borders and the
  signposts (the frontier) draw where and as main draws them, at every zoom. The capital is still the one place that
  reads as a civ's seat: its hall is the largest place and its banners its civ's.
- **The crowd shows.** A crowded place (a town of 8 clans, a capital or settlement that reads as a town) shows its place
  and every clan living on it: the clans stand in front, in its yard, so each clan's tent, hearth or walkers and a starving
  clan's red edge still show. A unit beside it keeps its figure and number readable, and a forest and a mountain ridge
  beside it draw as on main.
- **The flat zoom keeps main's marks.** Below the zoom where the clans turn to circles (#444's `CLAN.from`, 28 css px of
  terrain cell, k 13.6), the capital's house, the settlement's house and the town diamond are main's, pixel for pixel.

The control is main at the branch's base, shot with the same tool: a main worktree with the branch's `shots.mjs` and
`shots-views.mjs` copied in, as 439 to 444 did.

## What the live model exposes (the inventory)

The roles (`web/src/lib.rs`'s `view`):

- **Capital:** `view.civs[id].capital`, a tile, one per living civ. Its market never fades (`founding.md`).
- **Settlement** (#353): `view.settlements[]`, each `{ id, civ, tile, market, … }` (`settle::panels`): a civ's market
  that never fades, with a store. Founded by the `settle` step on a civ's signpost; taken with its store by a rival army.
- **Town:** `view.towns[]`, each `{ tile, market, clans }` (`towns.rs`): a festival whose tile holds `TOWN.homes` (3) or
  more homes. No rule reads it; it has no civ. A capital's or a settlement's market reads as a town too when 3 clans live
  on its tile.
- **Signpost:** `view.signposts[]`, a civ's planned site (`settle.js` draws a post with a board in the civ's colour).

So a tile is one of: a capital, a settlement, a town of no civ, or nothing. The design calls a settlement a city beyond the
capital ("Cities beyond the capital" in `founding.md`'s out of scope, then #353); the piece's "cities" are settlements.

What the map draws today (`www/map/view.js`'s `LAYERS`: `… held, lines, flow, clans, leanings, centres, capitals, towns,
settle, ring, units …`):

- `capitals.js`: `icon.capital` with `icon.capital.mask` tinted, half-width `max(0.95, 8 px)`, at the tile's centre,
  drawn over the clans (so the capital's own clans are hidden under it on main).
- `settle.js`: signposts (post and board), then each settlement's house in its civ's colour, `u = max(0.5, 14 px)`.
- `towns.js`: `icon.town` (a white diamond, 1.1 wide) at `py − 0.7`, over the capitals.
- `centres.js`: `icon.centre`, the gold ring, on `view.centres` (the capitals' festivals).
- `legend.js`: rows for capital, settlement, signpost.

Callers: only `map/view.js` draws these layers. Selection is by tile (`panels.js`'s `candidates`; `play/settle.mjs`
clicks a settlement's tile and cycles to it), so the drawing can't move it. `play/towns.mjs` checks the town mark and
the town clans' colours; `play/settle.mjs` checks the settlement is drawn where the signpost was (pixels differ by > 20);
`play/views.mjs` checks the legend. `play/goods.mjs` and `play/exchange.mjs` turn the marks' layers off by name to read the pixels near a
capital (a source, the flow); `places` joins their lists.

What the retro maps hold, leading the first civ and ending turns (`proto/places.mjs`, `proto/places.jsonl`,
`proto/near.mjs`, `proto/near.jsonl`):

| Map | +16 | +40 | +80 |
|---|---|---|---|
| small 2 | 3 capitals | + settlement 288 (Blue, no clans); town 505 (4, forest) beside capital 506 and its scout | 3 settlements, 2 towns |
| medium 8 | 3 capitals; town 754 (3) | the same | capital 1184 reads as a town (5) |
| medium 9 | settlement 602; town 676 (5) | settlements 602, 919 (a town of 3, in forest under mountains), 46; towns 676 (12), 735 (8, forest, 3 mountain neighbours) | the same |
| large 8 | capital 2524 a town (5); towns 2787, 820, 547 | capital 2524 a town (9); 6 towns, 2787 with 8 beside capital 2689 and its scout | 7 towns |

Small 3, 6 and 9 timed out ending turns on this runner (the turn didn't advance in 60 s) and aren't in the table; the
four maps above hold every role, crowded towns, and places beside units, forests and ridges.

## What the piece delivers

1. **The place cells** (build-1). The actor study's third row becomes cells:
   - `place.town`: the thatched hamlet (blob at (113, 660) 299×200, `proto/boxes.txt`). No mask.
   - `place.settlement`: the palisade, its huts, the smoke and the flag (blob at (609, 595) 379×296).
   - `place.capital`: the civic hall on its mound among its houses (blob at (1148, 614) 450×276).
   - `place.settlement.mask` and `place.capital.mask`: same size and anchor as the cell, transparent except the cloth (the
     flag; the hall's two banners), in grey (#443's `cloth` and `{ op: 'grey', lo }`).

   The third row has the soft red fringe the second had (1,900 to 3,400 halo pixels per group, 11 to 439 of them solid);
   #444's `floor` in `cut.mjs` handles it.
2. **The places on the map** (build-2). A new layer, `www/map/layers/places.js`, draws each place's cell by its role above
   the flat zoom, before the clans. `capitals.js` and `settle.js`'s settlement houses draw only at the flat zoom. The
   legend's capital and settlement rows show the new marks, and a town row is added.
3. **The views and checks** (build-2). `shots.mjs` gains the place views, `stage: 'places'` with `states`, and
   `--places`. `play/places.mjs` checks the lineup in CI.
4. **The look, judged** (build-3): the cuts, the cloth, the sizes, the place in the hex, the threshold, on the views
   beside main; and any variant the art lacks, authored in the study's palette.

## How the places are drawn

In world units (a hex has circumradius 1). Every number is in `PLACE`, a table at the top of
`www/map/layers/places.js`, a note beside each. Build-2 starts it at the values below; build-3 tunes it.

- **The cells.** All five are 176×128, anchor (88, 118): the ground the place stands on. Build-1 puts a tall group's ground point at the front of its foot (the palisade at (798, 870), the hall at (1373, 866)), since its middle would run past the cell; see `atlas.md`. Each group is
  scaled to fit about 168×120 (the hamlet at about 0.56, the palisade 0.40, the civic 0.37). A mask has its cell's size
  and anchor. They take one new row of the atlas (5 × 176 = 880 of its 1024 px).
- **The role.** Per tile, the first that holds: a capital of a civ that hasn't fallen (`civ.fallen` null) → `capital`; a settlement → `settlement`; a town
  → `town`. A capital or settlement that reads as a town keeps its own cell, and the town diamond still draws over it.
- **The size.** Each role's cell is drawn `PLACE.w[role]` world units wide (start: town 1.6, settlement 1.8, capital
  2.0), its height in the cell's proportion, pre-scaled to the device pixel (`atlas.scaled` and `atlas.shaded`, as
  `units.js` and `clans.js` draw).
- **The place in the hex.** The anchor at `(px, py + PLACE.feet)` (`feet` 0.3), so the place rises from just below the
  tile's centre into the hex's upper half, and a clan's ring (anchors 0.02 above to 0.82 below the centre) stands in
  front of it.
- **The order.** `places.js` sits in `LAYERS` just before `clans`, so every clan on a place's tile draws over the place:
  none is hidden, the starving edge shows, a click still cycles them. Centres, the town diamond, signposts, the ring and
  units draw over it, as they draw over the clans now.
- **The colour.** For `capital` and `settlement`, the mask pre-scaled and multiplied (`atlas.shaded`) by the civ's
  colour (`view.civs[civ].color`). `town` draws its cell alone.
- **The flat zoom.** `placesFlat(view)`: the terrain is flat, or its cell is under `PLACE.from` css px (start 28, the
  clans' own: `clans.js`'s `clansFlat` is the pattern, `dpr` divided out as #444's review fixed). There `places.js` draws
  nothing, and `capitals.js` and `settle.js` draw main's houses by main's code, unchanged. Above it they draw no house;
  `settle.js` still draws its signposts at every zoom, and `towns.js` its diamond.
- **What it reports.** `view.stats.places` lists each place drawn: `{ tile, role, civ, x, y, w, h }` in canvas px, `role`
  one of `town`, `settlement`, `capital`, or `mark` at the flat zoom (`civ` null for a town). `MapView.placeProbe()` draws
  each place alone (cell and mask) on transparent canvases, once in its colour and once with every colour black, and
  returns per place its rectangle, `opaque`, `tinted`, `mean` and `nearest`: of the living civs' colours, the one nearest
  `mean` by chromaticity (`units.js`'s `chroma`), as a civ id, or `null` for a town (no tint).

## Shape: Easel and Canvas

The cells' keys and sizes, the role rule, the colour rule, the order, the flat zoom, the probe, the views and the checks
are settled here to the rule. The first cuts need only be in place and read as a hamlet, a palisade and a hall. How the
places look and read, and how a crowded place sits with its clans, is the piece, judged by eye on real maps, so it
belongs to one canvas. This is 443's and 444's shape, cut by area as theirs were: the atlas, then the map.

- **build-1, easel (the atlas):** three regions and five cells, first cuts and cloth, `--check` holding them, the evidence.
- **build-2, easel (the map):** `places.js`, the flat path in `capitals.js` and `settle.js`, the stats and probe, the
  legend, the views (in `shots-views.mjs`, split from `shots.mjs`) and `--places`, the lineup in `play-stage.mjs`, and
  `play/places.mjs` in CI.
- **build-3, canvas:** how the places look and read: `PLACE`'s numbers, the cuts, the cloth, any variant the art lacks,
  the captures on main and the branch, and the evidence.

## Settled here, by running

- **The inventory** above (`proto/places.mjs`, `proto/near.mjs` on the retro maps).
- **The source boxes** (`proto/boxes.mjs`, `proto/boxes.txt`): the third row's three blobs at alpha ≥ 128 and their halos.
- **The views' cameras** (`proto/scene.mjs`, `proto/scenes.txt`): each real view below set on main and read back
  unclamped, with its digest and the places and units on screen. Digests: small 2 +40 `4ee21ce6517e1cc5`, medium 9 +40
  `c38cfa6eb0163d07`, large 8 +40 `37104de33cd8e31b`.
- **The lineup** (`proto/lineup.mjs`, `proto/lineup.txt`): staged on main, one turn ended, digest `514f01f255b6d28f`.
  The first turn founds Blue's settlement on its signpost (tile 140), and it reads as a town with its three clans; Red's
  capital (47) reads as a town with three. Red's army stays on c (124), beside the settlement; Red's scout walks to 74 on
  the turn. The stage makes no market of no civ, so the town of no civ (four clans on tile 79) is set on the drawn model
  (`PLACE_STATES`), as #444's lineup set its clans' states: the drawing reads `v.towns`, and the real views prove the game
  makes towns.
- **The open idea** "The unit bound at the flat zoom" is about unit banners at k 5, not places. Not taken.

## Decisions I made

- **Thatch is a town, palisade a settlement, civic the capital.** The three roles the model has, by rank: a festival clans
  crowd, a civ's walled outpost, the civ's seat. The study's palisade carries a flag (an owner); the hamlet none.
- **A capital or settlement that is also a town keeps its own shape.** Rank wins; the diamond still says "town".
- **A town carries no colour.** The model gives a town no civ. Tinting it by the land under it would claim ownership the
  model doesn't have; the land border already says whose land it is.
- **The place stands behind its clans.** Main draws the capital's house over its clans and hides them. The piece's goal
  is a crowded settlement that reads as crowded, so the clans stand in its yard, in front, and none is hidden.
- **The civic hall is the capital's marker above the flat zoom.** Main's house is replaced, not drawn beside it: one mark
  per capital. The gold centre ring stays round it.
- **A fallen civ's capital has no hall.** A fallen civ draws no units (`units.js`); its old capital is a town's hamlet
  if it reads as a town, else nothing above the flat zoom. At the flat zoom `capitals.js` stays main's, unchanged.
- **The places are in View → Civs.** `places` joins the `civs` group with `capitals` and `settle`, so a town's hamlet
  goes with them while its diamond (`towns`, its own layer) stays.
- **The flat zoom keeps main's marks, switching with the clans.** At a 12 px hex a palisade is a speck, and main's house
  reads. The places switch at the clans' zoom so the map changes register in one step.
- **The town diamond, signposts, borders and centre rings stay main's at every zoom.** They are the labels and frontier
  the goal names.
- **The legend's capital and settlement rows show the new marks, and a town row is added.** The legend telling the truth,
  not a HUD redesign.
- **No new variants are planned.** The three groups and two masks cover every role the model has. A taken settlement
  changes civ, so its flag's colour follows. Build-3 authors a variant only where the views show one missing (for
  example, a hamlet that hides a crowded town's clans needing a smaller variant), in the study's palette.

## Unsettled, and which step holds it

- **How the places look and read** (build-3, the canvas):
  - the cuts (keep polygons, scale, what of each group stays: the palisade's smoke, the hall's outer houses), the floor;
  - the cloth: which pixels each mask keeps, and the grey's floor;
  - `PLACE`: `w` per role, `feet`, `from`;
  - how a crowded place sits with its clans (the hamlet's size against 8 clans in a ring), and any variant that needs.

  Proof: the captures of the views below, main beside the branch, read by eye; `shots.mjs --places` on every place view
  (every capital and settlement nearest its own civ, every role the tile's, the tinted pixels per owned place at k 18
  above the bar build-3 sets); `--diff` (cameras and digests match, and `medium9-places-flat` and, while `PLACE.from`
  holds at 28, `medium9-places-edge` differ from main in no pixel); and `play/places.mjs`. All in
  `relay/specs/445/views.md`.

## Ideas taken

None. The one open idea is about units at the flat zoom.

## The views

`www/shots.mjs`'s views table moves to `www/shots-views.mjs` (`shots.mjs` is 563 lines). Each view below has
`places: true`. New: `stage: 'places'` stages `PLACES`; `turns` on a staged view ends that many turns after staging;
`states: true` then sets `PLACE_STATES`. Laptop is 1280×800, phone 390×844.

| View | Map | Window | k | Camera | What it shows |
|---|---|---|---|---|---|
| `medium9-places-play` | medium 9, 40 turns | laptop | 18 | between 735, 566 | Gold's capital 566 with its scout, its settlements 602 and 919, the town 735 (8 clans) in a forest under a ridge, camps 116 and 447 |
| `medium9-places-close` | medium 9, 40 turns | laptop | 35 | between 735, 919 | the crowded town on its forest ridge beside the settlement 919 (a town of 3) and 602 |
| `medium9-places-edge` | medium 9, 40 turns | phone | 6 | between 735, 566 | strategic zoom: main's marks while `from` is 28 |
| `medium9-places-flat` | medium 9, 40 turns | phone | 5 | between 735, 566 | the flat zoom: main's marks |
| `small2-places-close` | small 2, 40 turns | laptop | 35 | between 505, 506 | Blue's capital 506 and its scout beside the forest town 505 (4 clans), the settlement 288, a signpost |
| `large8-places-close` | large 8, 40 turns | laptop | 35 | between 2787, 2689 | the town 2787 (8 clans) beside capital 2689 and its scout, hills |
| `large8-places-play` | large 8, 40 turns | laptop | 18 | at 2524 | the fallen capital 2524 (its civ's capital fell by turn 40, so its tile is a town of 9 clans: the hamlet), capital 2689, the forest town 2871 under mountains, camp 2731 |
| `places-lineup-close` | the place lineup, 1 turn, states | laptop | 35 | between 47, 140 | each role; Red's capital with its clans; Blue's settlement beside the wood, the ridge and Red's army; the town of no civ |
| `places-lineup-play` | the same | laptop | 18 | between 47, 140 | the same at play zoom |
| `places-lineup-edge` | the same | phone | 6 | between 47, 140 | the same at strategic zoom |

The lineup, `PLACES` (`proto/lineup.mjs`, copy it as it is): radius 8, sea on the rim, plains with #443's forest block
(q 3 to 5, r −2 to 1) and a ridge of mountains at r −2, q 0 to 2. Civs on A (−4, 1) red led, B (0, −6) blue, C (−1, 6)
gold, 12 people. Red's army (4) and scout on c (1, −1). Clans: three red on A (trading at Red's market), three blue on S
(2, −1), four of no civ on T (−2, 4), with food on their tiles. A blue signpost on S. After one turn: capitals 47, 102,
97; Blue's settlement on 140 (a town of 3); Red's capital a town of 3; the four of no civ on 79; Red's camp on 124.
`PLACE_STATES`: push `{ tile: 79, market: null, clans: <the ids living on 79> }` into `game.map.model.v.towns`, set every
clan's `starving` false and `since` 5 (so the lineup's clans are tents and hearths, not walkers), and redraw.

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty, and `SAVE_VERSION` is main's. On both trees every
  place view's `digest` in `views.json` is the same.
- Run `node www/assets/draw-atlas.mjs` twice: the sha256 is the same both times and as committed, and `--check` passes.
  Every key of main's `atlas.json` is still there, with main's pixels (the five place cells are new).
- `node www/play/places.mjs` (in CI) on the lineup with its states:
  - one cell per place on screen, each of its tile's role: `capital` on 47 (a town too), `settlement` on 140 (a town
    too), `town` on 79; the town diamond drawn on all three (`towns` on);
  - every capital's and settlement's `nearest` in `placeProbe()` is its own civ, with `tinted` above zero; the town's
    `tinted` is 0;
  - every clan on 47, 140 and 79 is in `stats.clans` with a cell, and the clans draw after the places;
  - a click on 140 reaches the settlement (`settlement:<id>`) and each of its clans, as on main;
  - at the flat zoom (phone, k 5) every entry of `stats.places` is a `mark`, and the house on 47 and on 140 is drawn
    (pixels in the civ's colour at the tile's centre).
- `node www/play/all.mjs` passes, `towns.mjs`, `settle.mjs`, `clans.mjs` and `views.mjs` among them.
- Make a main worktree with the branch's `shots.mjs` and `shots-views.mjs`, shoot every place view on both and `--diff`:
  cameras and digests match; `medium9-places-flat` differs in no pixel, and `medium9-places-edge` too unless build-3
  moved `from` below k 6. Run `--places` on the branch. Run `--time medium9-places-play` on both trees and record the two
  medians. Then read the pairs:
  - hamlet, palisade and hall read as three things at k 35 and k 18;
  - each settlement's flag and capital's banners read as its civ's, Gold's too;
  - in `medium9-places-close` and `large8-places-close`, the town of 8 shows as a town with every clan visible;
  - in `places-lineup-close` and `small2-places-close`, the settlement or town beside a unit leaves the unit's figure and
    number readable, beside a forest and a ridge;
  - the town diamond, centre rings, signposts, borders, trees, ridges and units sit where main has them.
- On the branch page, click a capital, a settlement and a town, cycle a crowded tile, pan and zoom across k 5 to 35.
  They behave as on main.
- The PR carries main's and the branch's captures of `places-lineup-close`, `places-lineup-play`, `medium9-places-play`,
  `medium9-places-close`, `small2-places-close`, `large8-places-close` and `large8-places-play`, from the files build-3
  commits under `relay/specs/445/shots/`.
