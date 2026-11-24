# #444 build-1 (easel): the clan cells and their masks in the atlas

Read `relay/specs/444/spec.md` first: the inventory, "The clan cells" under what the piece delivers, "How the clans
are drawn" (the cells' size and anchor), and the decisions. Build-2 draws your cells on the map, and build-3, the
canvas, judges how they look and may change any recipe. Your job is cells that are exact in their keys, sizes and
anchors, and first cuts that read: a hide tent, a tent by a fire with its smoke, and a family walking, each with a
mask that holds only its cloth. Don't change Rust: nothing under `engine/core`, `engine/cli` or `engine/web`. Don't
change map code: `www/map/`, `www/atlas.js` and `www/legend.js` are build-2's.

Read these before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js,png.mjs}`,
`www/play/art.mjs`, `relay/specs/443/atlas.md`, and `relay/specs/443/proto/{compare-main.mjs,cells.mjs}` (how 443 cut
the unit figures, compared with main and showed its cells). The actor regions in `sources.js` (`actor.camp`,
`actor.scout`, `actor.flag`) and the unit cells in `CELLS` (with `over`, `cloth` and `{ op: 'grey', lo: 190 }`) are
your pattern. Look at `docs/proposals/map-art/actors.png`'s second row; `relay/specs/444/proto/crop.mjs` writes a
crop of it on grey at any scale, which is how you read a polygon off it.

## 1. A floor in `www/assets/cut.mjs`

The second row has a soft fringe the top row lacks: 17,000 to 25,000 pixels of alpha 1 to 127 round each group, some
of them red (`relay/specs/444/proto/boxes.mjs`). Cut as it is, the fringe smears into the rim at game scale.

- A sprite region may name `floor` (0 to 255): a source sample whose alpha is under it counts as clear (alpha 0).
  Without `floor` a region cuts as today, so every existing cut stays identical to the pixel. Start the clan regions
  at `floor: 128`; build-3 tunes it.
- An assert in `draw-atlas.mjs`'s `asserts()`: `floor: 0` is the plain cut.
- Report in `atlas.md`, per clan cell, its rim: the opaque pixels within 1 px of a clear one, and how many of those
  are red-dominant (r > 2g and r > 2b). Do the same for the cut without the floor, so build-3 sees what the floor did.

## 2. The cells

In `placeholders.js`'s `layout()`, after the unit row, a row of six cells, each 112×96 with anchor (56, 90):
`clan.tent`, `clan.tent.mask`, `clan.hearth`, `clan.hearth.mask`, `clan.walk`, `clan.walk.mask`. Export the size as
`CLAN = { w: 112, h: 96, ax: 56, ay: 90 }` beside `UNIT`. The notes say what each is: a clan's group standing on the
ground, the ground's middle at the anchor (a tent: a clan that gathers; a hearth: a settled clan; walkers: a clan
that came to its home this turn); a mask: its cloth in grey, tinted by multiply and drawn over the cell. Their
placeholder drawings must still draw something, as `play/art.mjs` requires every cell to be non-blank. Every other key
keeps its size and its pixels; its rectangle may move down with the rows.

In `sources.js`, under the actors, three regions with key `actor`, each its box with a 3 px margin as the
keep-polygon unless a neighbour falls inside it, its ground point the middle of the ground its group stands on, and
`floor: 128`:

- `clan.tent`: the hide tent, about x 79 to 330 and y 352 to 540. A basket at its foot falls inside a plain box; keep
  it or cut round it, as reads better. Leave out the drying rack and the barrel.
- `clan.hearth`: the left tent, the seated man, the fire with its pot, and the smoke: the blob at (1172, 339) 279×214.
  Leave out the right tent and the woman (the blob at (1408, 400)).
- `clan.walk`: the man, the woman and the child: (617, 386) to (866, 537). Leave out the donkey (from x 874).

Scale each to fit about 108×88 in its cell. `clan.<kind>.mask` is the same spec with `cloth` and
`{ op: 'grey', lo: 190 }`. First cloth polygons, read off a 6× crop:

- the tent: its hide, the brown and the cream (not the poles, the opening's dark, or the basket);
- the hearth: the tent's canvas (not the poles, the man, the fire or the smoke);
- the walkers: the man's tunic, the woman's dress and shawl, the child's tunic (not skin, hair, packs, staff or boots).

## 3. Checks and evidence

- `node www/assets/draw-atlas.mjs` twice gives the same bytes; `--check` passes, with the new assert.
- Every key of main's `atlas.json` is still there with main's pixels: copy `relay/specs/443/proto/compare-main.mjs`
  into `relay/specs/444/proto/` and point it at this piece. `node www/play/art.mjs` passes.
- `relay/specs/444/cells.png`: the six cells at 3×, on mid grey and on a forest-green patch, each mask beside its cell,
  and each cell with its mask multiplied in red, blue, gold and the pale (`#d64545`, `#3b6fd6`, `#d6a63b`,
  `#d8d2c4`) drawn over it. Its proto under `relay/specs/444/proto/`.
- `relay/specs/444/atlas.md`: the sha256 of `atlas.png` and `atlas.json`, the `--check` line, the compare line, each
  cell's region, scale, anchor, floor and cloth, the share of each cell's opaque pixels its mask covers, and the rim
  table from 1.
- `engine/CLAUDE.md`'s `assets/` row names `floor` and the clan cells' keys, size and anchor, within its 8 KB. It is
  7,760 bytes and build-2 adds a line after you: spend under 150 bytes, tightening the row's own wording if needed.

Run `engine/check.sh` whole once before you finish.

Build-2 relies on: the six keys at 112×96 with anchor (56, 90); `CLAN` exported from `placeholders.js`; each mask
overlaying its cell; a mask transparent outside the cloth and grey inside it, so multiply gives the colour shaded.
