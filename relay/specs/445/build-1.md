# #445 build-1 (easel): the place cells and their masks in the atlas

Read `relay/specs/445/spec.md` first: the inventory, "The place cells" under what the piece delivers, "How the places
are drawn" (the cells' size and anchor), and the decisions. Build-2 draws your cells on the map, and build-3, the
canvas, judges how they look and may change any recipe. Your job is cells that are exact in their keys, sizes and
anchors, and first cuts that read: a hamlet of thatched huts, a palisade round its huts with a flag, and a hall on its
mound among houses, the two owned ones with a mask that holds only their cloth. Don't change Rust: nothing under
`engine/core`, `engine/cli` or `engine/web`. Don't change map code: `www/map/`, `www/atlas.js` and `www/legend.js` are
build-2's.

Read these before you edit: `www/assets/{cut.mjs,sources.js,draw-atlas.mjs,placeholders.js,png.mjs}`,
`www/play/art.mjs`, `relay/specs/444/atlas.md`, and `relay/specs/444/proto/{compare-main.mjs,cells.mjs,crop.mjs}`
(how 444 cut the clan groups, compared with main, showed its cells, and read polygons off a crop). The clan regions in
`sources.js` (`clan.tent`, `clan.hearth`, `clan.walk`, with `floor: 128`) and their cells in `CELLS` (with `cloth` and
`GREY`) are your pattern. Look at `docs/proposals/map-art/actors.png`'s third row.

## 1. The cells

In `placeholders.js`'s `layout()`, after the clan row, a row of five cells, each 176×128 with anchor (88, 118):
`place.town`, `place.settlement`, `place.settlement.mask`, `place.capital`, `place.capital.mask`. Export the size as
`PLACE = { w: 176, h: 128, ax: 88, ay: 118 }` beside `CLAN`. The notes say what each is: a place standing on the ground,
the ground's middle at the anchor (a hamlet: a town, a festival clans crowd; a palisade: a civ's settlement; the hall: a
civ's capital); a mask: its cloth in grey, tinted by multiply and drawn over the cell. Their placeholder drawings must
still draw something, as `play/art.mjs` requires every cell to be non-blank. Every other key keeps its size and its
pixels; its rectangle may move down with the rows (the atlas grows by one 128 px row).

In `sources.js`, after the clans, three regions with key `actor`, each its box with a 3 px margin as the keep-polygon,
its ground point the middle of the ground the group stands on, and `floor: 128`. Boxes at alpha ≥ 128
(`relay/specs/445/proto/boxes.txt`):

- `place.town`: the thatched hamlet, (113, 660) 299×200. Scale about 0.56.
- `place.settlement`: the palisade with its huts, smoke and flag, (609, 595) 379×296. Scale about 0.40.
- `place.capital`: the hall on its mound and the houses round it, (1148, 614) 450×276. Scale about 0.37.

Scale each to fit about 168×120 in its cell. Each mask is the same spec with `cloth` and `GREY`. First cloth polygons,
read off a 6× crop (copy 444's `crop.mjs` into `relay/specs/445/proto/` and point it at the third row):

- the settlement: the flag's cloth (not the pole, and not the chevron if it reads better left dark; your call);
- the capital: the two banners hanging on the hall's front (not the posts, the roof or the walls).

## 2. Checks and evidence

- `node www/assets/draw-atlas.mjs` twice gives the same bytes; `--check` passes.
- Every key of main's `atlas.json` is still there with main's pixels: copy `relay/specs/444/proto/compare-main.mjs` into
  `relay/specs/445/proto/` and point it at this piece. `node www/play/art.mjs` passes.
- `relay/specs/445/cells.png`: the five cells at 2×, on mid grey and on a forest-green patch, each mask beside its cell,
  and each owned cell with its mask multiplied in red, blue and gold (`#d64545`, `#3b6fd6`, `#d6a63b`) drawn over it. Its
  proto under `relay/specs/445/proto/` (444's `cells.mjs` is the pattern).
- `relay/specs/445/atlas.md`: the sha256 of `atlas.png` and `atlas.json`, the `--check` line, the compare line, each
  cell's region, scale, anchor, floor and cloth, the share of each owned cell's opaque pixels its mask covers, and each
  cell's rim (opaque pixels within 1 px of a clear one, and how many are red-dominant: r > 2g and r > 2b), as 444's table.
- `engine/CLAUDE.md`'s `assets/` row names the place cells' keys, size and anchor beside the clan cells. The file is 8,183
  bytes against `check.sh`'s 8,192 budget, and build-2 adds a line after you: tighten the row's own wording so the file
  ends under 8,100 bytes.

Run `engine/check.sh` whole once before you finish.

Build-2 relies on: the five keys at 176×128 with anchor (88, 118); `PLACE` exported from `placeholders.js`; each mask
overlaying its cell; a mask transparent outside the cloth and grey inside it, so multiply gives the colour shaded.
