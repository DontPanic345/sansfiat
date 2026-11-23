# #443 Units on the map

**Question:** none in the first four scores, because no rule moves. This is piece 5 of the illustrated map
(`docs/proposals/map-art.md`). It asks each of these against main:

- **Health: nothing but the art moved.** `git diff main --stat -- engine/core engine/cli engine/web` is empty,
  so every digest, `SAVE_VERSION` (27) and every save stay as main's without a sweep. Every unit view records the
  game's digest, and it matches main's on the same view.
- **A camp and a scout are told apart by shape.** On main both are the same shield. On the branch the camp
  (an army) is the source study's spearman with a round shield, and the scout is its walker with a pack and staff.
  At play zoom (k 18) and at the strategic zoom (phone k 6) the two read as different figures.
- **A unit's civ is told by its colour, on grass and in forest.** On main the whole shield is the civ's colour.
  On the branch, the civ's colour is cloth on the figure (a banner on every unit, and the camp's shield face),
  shaded by the art. Your units and a rival's are told apart on plains and in a wood, at play and strategic zoom.
  Measured: every unit's tinted pixels are nearest its own civ's colour of the three (in CI), and the tinted area
  per figure at k 18 and phone k 6 (`views.md`).
- **The signals stay.** A selected unit is edged white and a hungry one red, as on main. Units on one tile still
  stand side by side, each with its number, and a click on the tile still cycles them. An army's archers are still
  shown ("6+2" and the bow). Selection, pan and zoom behave as on main.

The control is main at the branch's base, shot with the same tool: a main worktree with the branch's `shots.mjs`
copied in, as 439 to 442 did.

## What is on the map today (the inventory)

Main has two unit kinds (`UnitKind` in `core/src/units/mod.rs`): `camp`, an army of warriors with archers in its
stack (`u.stack = [warriors, archers]`), and `scout`. Both draw in `www/map/layers/units.js` as the atlas cells
`unit.<kind>` and `unit.<kind>.mask` (64×64 placeholders, both the same shield), the mask tinted in the civ's colour
by `atlas.tinted`. Only `units.js` reads these cells; no HUD or panel draws them. Clans, hearths, festivals,
capitals and towns are other layers, for pieces 6 and 7.

What the retro maps hold (`proto/units.mjs`, `proto/units.txt`, `proto/scenes.txt`):

- At the pick each civ has one scout, standing on its capital, so a unit always shares its tile with the capital's
  house, the centre's gold ring and the clans. By turn +8 the AI civs have raised one or two camps, often hungry.
  The led civ has a camp only when the player raises one, at the capital: then a camp and the scout share the tile,
  the only stack the real maps make early.
- Units stand on plains, coast, forest, hills, marsh and desert. On forest: rival scouts on medium 8 (639, 1312)
  and medium 9 (780, Gold's capital). On plains: the led civ's scout on small 9, medium 8 and medium 9.
- Leading Gold on medium 9 (the third card) and raising puts the led civ's camp and scout in the wood at 780. That
  is the one real "yours in forest" view, at turn +8 (the raise waits for the store).
- Small 3 and small 6 end the led game inside 20 turns when the player only raises and ends turns ("Play on, New
  game"), so no unit view uses them.
- Main draws the stack's numbers after each figure, so the second figure covers the first figure's number (the Gold
  stack in `lineup` and the Red stack in `medium9-stack-close`).

## What the piece delivers

1. **The figures and their masks** (build-1): the actor study `docs/proposals/map-art/actors.png` becomes a source
   file in `sources.js`, and the atlas cuts from it:
   - `unit.camp`: the spearman with the round shield (source box about (583, 72) 129×187). It carries a banner.
   - `unit.scout`: the walker with pack and staff (about (193, 107) 111×149). It carries a banner on its staff,
     as the scout in `ui.png` does.
   - The banner is the study's flag (about (1360, 100) 104×157), cut small and set on the staff or spear.
   - `unit.<kind>.mask`: same size and anchor as its figure, transparent except the civ's cloth (the banner's
     cloth, and the camp's shield face), in grey: the art's own shading, lifted so its light parts are near white.

   To build these, `cut.mjs` gains a key that keeps an actor (the study is on transparent ground), a cell made of
   two sprite regions (the figure and its banner), a mask polygon that keeps only the cloth, and a grey op.
2. **The figures on the map** (build-2): `units.js` stands each figure on its slot, draws it pre-scaled to the
   device pixel, then its mask tinted by multiply in its civ's colour, then the numbers. The selected and hungry
   edges are the figure's own outline, drawn behind it. `atlas.js` gains the multiply tint and the outline.
3. **The views and checks** (build-2): `www/shots.mjs` gains the unit views below, with `lead`, `raise`, `stage`
   and an `at`/`between` camera. `www/play/units.mjs` checks the lineup in CI. `shots.mjs --units` prints each
   unit's tinted area and nearest civ colour.
4. **The look, judged** (build-3): the cuts, the banner, the cloth, the grey, the figure's size, the outlines, the
   stack's spacing and numbers, all on the views beside main.

## How the figures are drawn

All in world units: a hex has circumradius 1. Every number is in `FIGURE`, a table at the top of
`www/map/layers/units.js` with a `note` per number. Build-2 starts it at the values below, and build-3 tunes it.

- **The cells.** `unit.camp`, `unit.scout` and their masks are 96×96, anchor (48, 92): the figure's feet. A mask
  has its figure's size and anchor, so the two overlay exactly. The keys stay `unit.<kind>` and `unit.<kind>.mask`.
- **The slot.** Today's shield centre is the figure's place: unit `n` on a tile (in id order) at
  `x = px + n·gap·s + 0.9 s`, `y = py + 1.1 s`, with `s = max(FIGURE.s, FIGURE.minPx / k)` (0.6, 6 css px). The
  figure's feet stand at `y + FIGURE.feet·s` (0.9) so its middle sits near today's shield. `gap` starts at 1.7.
- **The height.** A figure is drawn `FIGURE.h·s` world units tall (2.4, so 1.44 world units at play zoom, 26 px
  at k 18), its width in the cell's proportion. It draws from a canvas pre-scaled to its device height
  (`round(h·k·dpr)`), cached by height, as `atlas.scaled` does for the terrain, so a 96 px cell shrinks cleanly.
- **The order on a tile:** each unit's red outline if hungry, its white outline if selected, its figure, its
  tinted mask; then, after every figure on the tile, every number and archer's bow. So no figure covers a number.
- **The civ's colour.** The mask, pre-scaled, is tinted by multiply: the cloth's grey times the civ's colour, the
  mask's alpha kept. White gives the colour itself. `atlas.tinted` (source-in) stays for `icon.capital.mask`.
- **The outlines.** A figure's alpha, pre-scaled, dilated by a width in device px and filled in one colour: white
  `FIGURE.selected` (2.4 css px) for the selected unit, red `#ff2a2a` `FIGURE.hungry` (3.5 css px) for a hungry
  one, as main's shield strokes. Cached by name, height, width and colour.
- **What it reports.** Each entry of `view.stats.labels` keeps `{ unit, label }` (`play/army.mjs` reads it) and
  gains `box`, the number's text box in canvas px. `view.stats.units` lists each figure drawn: `{ unit, civ, kind, x, y, w, h }`, its rectangle
  in canvas px. `MapView.unitProbe()` draws the units alone on transparent canvases, once in their colours and once
  with every civ's colour black, and returns per unit its rectangle, its opaque pixels, its tinted pixels (those
  that differ), their mean colour, and the civ colour nearest it of the three by chromaticity (each colour divided
  by its r+g+b), so the art's shading doesn't count: in plain RGB, gold multiplied by a grey of 0.6 is nearer red
  than gold (spec-review, `review.md`). On main there is no probe.

## Shape: Easel and Canvas

The cells' keys, the drawing order, the slot, the multiply tint, the outlines, the probe, the views and the checks
are settled here to the rule. The first cuts only need to be in place and read as a spearman and a walker with a
banner. How the units look is the piece, judged by eye on real maps, so it belongs to one canvas. This is 439's to
442's shape, cut by area as 442's was: the atlas, then the map.

- **build-1, easel (the atlas):** the actor source, the actor key, two-region cells, the cloth mask and the grey op,
  first cuts of the four cells, and `--check` holding them.
- **build-2, easel (the map):** `units.js`'s figures, `atlas.js`'s multiply tint and outlines, the probe, the views
  and options in `shots.mjs`, and `play/units.mjs` in CI.
- **build-3, canvas:** how the units look and read, `FIGURE`'s numbers, the cuts, the banner and cloth, the captures
  on main and the branch, and the evidence.

## Settled here, by running

- **The inventory** above (`proto/units.mjs` on the seven retro maps, `proto/units.txt`).
- **The views' setups** (`proto/scenes.mjs`, `proto/scenes.txt`): each view below set up on main, its camera read
  back unclamped, with its units and digest. `lead(page, { civ: 'Gold' })` finds the third card.
  `raiseArmy(page, { warriors: 3 })` raises Red's camp on medium 9 in one turn (turn 468) and Gold's in eight (475).
- **The lineup** (`proto/lineup.mjs`): a built map where each civ's one army and one scout stand on plains and in a
  wood, and Gold's stand stacked. It stages on main with no error, digest `62bef9c95b4d1f08`. A stage gives each civ
  one army (a second `armies` entry for a civ moves the first), so the lineup has six units. A retro stage with
  `civs` founds new civs beside the pick's own, so the lineup is a built map, and the real views play the real game.
- **The source** (`actors.png`, 1671×941, RGBA): ground alpha is 0, a figure's interior 250 to 254, its rim
  partial. Boxes at alpha ≥ 128: walker (193, 107) 111×149, round-shield spearman (583, 72) 129×187, kite-shield
  spearman (973, 72) 122×188, flag (1360, 100) 104×157. Each stands alone in its box: the keep-polygon can be the
  box with a margin. The lower rows' buildings have a red halo; the top row has none.
- **Callers.** Only `layers/units.js` reads `unit.*`. `atlas.tinted` is used by `units.js` and `capitals.js`.
  Selection is by tile (`map/input.js`), so the figures' drawing can't move it. `play/army.mjs` reads
  `stats.labels`, which stays.

## Decisions I made

- **Two figures, one per kind.** The camp is the round-shield spearman, the scout the walker. The live model has no
  other unit, so the kite-shield spearman is not used. An army with archers keeps its "6+2" number and bow, not a
  third figure.
- **The civ's colour is cloth: a banner on every unit and the camp's shield face.** `ui.png` gives the scout a
  banner, and a flat cloth reads at 15 px where a recoloured tunic doesn't. The study's ochre tunic stays ochre, so
  Gold's camp is not just the art's own colour. The canvas may add the tunic if the banner alone doesn't read.
- **The tint is multiply, so the art's shading shows through the colour.** A flat fill would read as a sticker.
- **Your units and a rival's differ only by colour, as on main.** The design gives no other mark, and the top bar
  already marks your civ's colour.
- **The selected and hungry edges follow the figure's outline,** not a shield's path: "edged" means the same as on
  main, on a new shape.
- **Every number on a tile draws after every figure.** Main lets the second figure in a stack cover the first's
  number, which hides a stack's size, the signal `page.md` names.
- **Figures stand on their feet,** anchored at the cell's foot as the trees are, at today's place on the tile.
- **The figure is drawn from a canvas pre-scaled to the device pixel.** A 96 px pixel-art cell drawn straight down
  to 26 px aliases.

## Unsettled, and which step holds it

- **How the units look and read** (build-3, the canvas). This covers:
  - the cuts (scale, keep-polygons, anchors) and the banner (where on the staff and spear, its size);
  - the cloth: which pixels the mask keeps, and the grey's range;
  - `FIGURE`: `h`, `s`, `minPx`, `feet`, `gap`, the outline widths, the numbers' place;
  - whether the tunic joins the cloth.

  Proof: the captures of the views below, main beside the branch, read by eye; `shots.mjs --units` on every unit
  view (every unit nearest its own civ, and the tinted area at k 18 and phone k 6 above the bar build-3 sets); and
  `play/units.mjs`. All in `relay/specs/443/views.md`.

## Ideas taken

None were listed, and 442 left no `notes.md`.

## The views

`www/shots.mjs` keeps its views and adds those below. New options:

- `lead: 'Gold'` presses that civ's card (`Lead Gold`) instead of the first. A view with `lead`, `raise` or `turns`
  leads before anything else.
- `raise: N` runs `play.mjs`'s `raiseArmy(page, { warriors: N, turns: 10 })` after leading, then any `turns`.
- `stage: 'lineup'` stages the lineup (the spec in `proto/lineup.mjs`, copied into `shots.mjs`) instead of opening
  a map; the page opens bare (`open('')`).
- `on: 'at'` puts the camera on tile `at`, or on the mean of the tiles in `between`.

Each view waits for the map's animation to finish before setting its camera, as `proto/scenes.mjs` does (raising
selects the civ and pans to it). Each records `game.digest()`. Laptop is 1280×800 and phone 390×844.

| View | Map | Window | k | Camera | What it shows |
|---|---|---|---|---|---|
| `medium9-units-play` | medium 9, raise 3 | laptop | 18 | between 547, 780 | your camp and scout stacked on plains; Gold's scout in the wood |
| `medium9-stack-close` | medium 9, raise 3 | laptop | 35 | at 547 | your stack on plains, close |
| `medium9-wood-close` | medium 9 | laptop | 35 | at 780 | Gold's scout (a rival) in the wood |
| `medium9-gold-play` | medium 9, lead Gold, raise 3 | laptop | 18 | at 780 | your camp and scout stacked in the wood |
| `medium9-gold-close` | medium 9, lead Gold, raise 3 | laptop | 35 | at 780 | the same, close |
| `medium8-units-play` | medium 8 | laptop | 18 | between 1145, 639 | your scout on plains; Blue's and Gold's scouts in woods |
| `small2-camps-play` | small 2, 8 turns | laptop | 18 | at 527 | Gold's hungry camp on plains, Blue's camp on the coast |
| `small2-camps-close` | small 2, 8 turns | laptop | 35 | at 527 | the hungry camp, close |
| `medium9-units-edge` | medium 9, raise 3 | phone | 6 | between 547, 780 | strategic zoom |
| `medium9-units-flat` | medium 9, raise 3 | phone | 5 | between 547, 780 | the flat zoom |
| `lineup-close` | the lineup | laptop | 35 | between 125, 170 | each civ's army and scout on plains and in a wood; Gold's stack |
| `lineup-play` | the lineup | laptop | 18 | between 125, 170 | the same at play zoom |
| `lineup-edge` | the lineup | phone | 6 | between 125, 170 | the same at strategic zoom |

## For the review

- `git diff main --stat -- engine/core engine/cli engine/web` is empty, and `SAVE_VERSION` is 27. On both trees,
  every unit view's `digest` in `views.json` is the same.
- Run `node www/assets/draw-atlas.mjs` twice: the sha256 is the same both times and as committed, and `--check`
  passes. Every key of main's `atlas.json` is still there, and every one but the four unit cells keeps main's pixels.
- `node www/play/units.mjs` (in CI) on the lineup:
  - six figures drawn, each kind's cell;
  - every unit's tinted pixels nearest its own civ's colour;
  - a click on Gold's tile cycles its two units, and the white outline follows the selection;
  - both numbers on Gold's tile are in `stats.labels`, and their boxes don't overlap each other. No figure covers a
    number because every number draws after every figure: read the order in `units.js`. (A number's anchor can't
    sit outside the next figure's cell rectangle: at the starting `gap` 1.7 the cells, 2.4 s wide, overlap, and the
    first number at +0.95 s falls inside the second cell from +0.5 s.)
- Make a main worktree with the branch's `shots.mjs`, shoot every unit view on both and `--diff`: cameras and
  digests match. Run `--units` on the branch. Then read the pairs:
  - camp and scout are different figures at k 18 and phone k 6;
  - in `lineup-*`, red's units and blue's are told apart on plains and in the wood, and so are gold's;
  - in `medium9-units-play` and `medium9-gold-play`, your units read on plains and in the wood beside the capital's
    house and ring;
  - `small2-camps-*` shows the hungry camp's red edge;
  - every number in a stack reads;
  - trees, rivers, borders, icons and clans sit where main has them.
- On the branch page, select a unit, cycle a stack, give a Move to…, pan and zoom across k 6 to 35. They behave as
  on main.
- The PR carries main's and the branch's captures of `lineup-close`, `lineup-play`, `lineup-edge`,
  `medium9-units-play`, `medium9-gold-close` and `small2-camps-close`, from the files build-3 commits under
  `relay/specs/443/shots/`.
