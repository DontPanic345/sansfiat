# #439 build-1 (easel): the atlas build, its check, and the views tool

Read `relay/specs/439/spec.md` first: it holds the source's measurements, the decisions and
the views table. Build-2, the canvas, judges how the plains tile looks and may change what you
build; your job is tooling that is exact. No Rust changes. Nothing under `engine/core`,
`engine/cli` or `engine/web` is touched.

Read before you edit: `www/assets/draw-atlas.mjs`, `www/assets/placeholders.js` (`layout()`,
`HEX_R`, how a terrain cell's hex is drawn), `www/atlas.js`, `www/map/layers/terrain.js`,
`www/play.mjs` (`open`, `SIZES`, `shot`, `eval`), `www/play/art.mjs`, `check.sh`. And the
prototype: `relay/specs/439/proto/png.cjs` (a working PNG decoder and encoder) and
`proto/proto.cjs` (the plains cut that works), with its output `proto/plains.png`.

## 1. `www/assets/png.mjs`

An ES module, `node:zlib` and `node:fs` only:

- `decode(buffer) → {width, height, data}`: `data` RGBA8, unpremultiplied. Colour types 2
  (RGB) and 6 (RGBA), bit depth 8, non-interlaced, every filter (0–4), several IDAT chunks.
  Anything else throws naming what it met.
- `encode({width, height, data}) → Buffer`: RGBA8, filter 0 on every row, one IDAT,
  `zlib.deflateSync(raw, { level: 9 })`, CRCs right.
- `readPng(path)`, `writePng(path, image)` beside them.

Port `proto/png.cjs`; keep it short and commented as the code is.

## 2. `www/assets/sources.js`

The named regions and the cells cut from them, as data with a note on each. Start it from the
prototype:

```js
/** Source studies, by name: repo paths, outside the site. */
export const FILES = { terrain: 'docs/proposals/map-art/terrain.png' };

/** A region: an explicit hex in a source file, in source px, and how it is cleaned. */
export const REGIONS = {
  'plains.grass': {
    file: 'terrain',
    hex: { cx: 551.5, cy: 854, rx: 96.5, ry: 109 },   // half-width, circumradius
    inset: 3,                                          // drops the drawn outline
    patches: [{ from: [555, 790], to: [452, 828], w: 72, h: 70, feather: 8,
                note: "hex 2's hill runs into the lower left: grass and a tuft over it" }],
    note: 'bottom-left row, third hex: grass, tufts and rocks',
  },
};

/** The atlas cells cut from regions, by atlas key: each replaces that placeholder in place. */
export const CELLS = {
  'terrain.plains.0': { region: 'plains.grass', ops: [], anchor: 'centre' },
  'terrain.plains.1': { region: 'plains.grass', ops: ['mirror'], anchor: 'centre' },
  'terrain.plains.2': { region: 'plains.grass', ops: [], anchor: 'centre' },
};
```

An anchor is `'centre'` or `[ax, ay]` in cell px. Paths resolve from the repo root (found
from the module's own URL), never from the working directory.

## 3. `www/assets/cut.mjs`

`cut(source, region, cell, {w, h}) → {data, ax, ay, flat}`, pure (no I/O), deterministic:

1. Patches are applied to a copy of the region's bounding box, in order: each destination
   pixel blends toward the source pixel with weight `min(1, d / feather)`, `d` its distance in
   px to the nearest of the destination rectangle's edges (`proto.cjs` leaves the left edge
   unfeathered; feather all four, and the canvas tunes the numbers).
2. The source's alpha is ignored. The cell's alpha is its hex's coverage: the placeholders'
   pointy-top hex, circumradius `HEX_R` (74) centred in the `w × h` (128 × 148) cell, sampled
   4 × 4 per pixel. Read `placeholders.js` to match its hex exactly, so a cut cell seams with
   its neighbours as a placeholder does.
3. Each covered sample maps linearly from the cell's hex to the region's hex shrunk by
   `inset` (x by `rx - inset`, y by `ry - inset`) and reads that source pixel; the pixel is the
   mean of its covered samples. Fully transparent pixels are `0,0,0,0`.
4. Ops in order: `mirror` flips the cell left to right (the anchor's x with it).
5. `flat`: the mean colour of the pixels with alpha 255, as `#rrggbb`, rounded.

Every sum in a fixed order and every result `Math.round`ed, so the output is the same on any
machine.

## 4. `draw-atlas.mjs`

- `node www/assets/draw-atlas.mjs`: the Chrome stage as now (it reproduces today's
  `atlas.png` byte for byte), but take its PNG back into Node, `decode` it, write every
  `CELLS` entry's cut over its key's rectangle (all four channels), `encode`, and write
  `atlas.png`. Then `atlas.json` as now plus, on every cell, `ax`, `ay` (the centre,
  `w / 2` and `h / 2`, unless the cell's anchor says otherwise) and, on a cut cell, `src` (the
  region's name) and `flat`. Keys, rectangles and notes stay exactly as `layout()` makes
  them; a cut cell's note says it is cut from its region (the region's note).
- `--check`, with no Chrome (import `play.mjs` only when drawing): read `atlas.png` and
  `atlas.json`; recut every `CELLS` entry and compare it pixel for pixel with its rectangle
  in `atlas.png`; compare `atlas.json`'s keys and rectangles with `layout()`'s, and each cut
  cell's `ax`, `ay`, `src` and `flat` with the recut. Print one line on success; on failure,
  each bad cell and the first differing pixel, and exit 1. It runs in a couple of seconds.
- A `CELLS` key that `layout()` doesn't make is an error in both modes.

Add `node www/assets/draw-atlas.mjs --check` to `check.sh` after `node www/check.mjs`.

Done for 1–4:
- run the build twice: `sha256sum www/assets/atlas.png www/assets/atlas.json` the same both
  times;
- with `CELLS` emptied (in a scratch edit you revert), the build writes main's `atlas.png`
  pixels (decode both and compare; the bytes differ, since Node encodes now) and main's
  `atlas.json` plus `ax`/`ay`;
- every placeholder cell's pixels are main's; the three plains cells are the cut, and
  `terrain.plains.0` matches `proto/plains.png` within a few levels a channel (the patch
  feathering on four edges may differ);
- `--check` passes, and fails when one byte of a plains rectangle in a scratch copy of
  `atlas.png` changes (an `--atlas DIR` flag that reads another folder is the easy way; or edit, run and revert).

## 5. `www/shots.mjs`

`node www/shots.mjs --out DIR [--only NAME,…]` and `node www/shots.mjs --diff A B`. A
script of its own, in `www/` (not `www/play/`, where `play/all.mjs` would run it in CI; and not
`views.mjs`, which `www/play/` already has). `--out` imports only `./play.mjs`, so the file
copies alone onto a main worktree and shoots there; `--diff` runs on the branch and imports
`./assets/png.mjs` dynamically, inside the `--diff` path, so main never needs it.

- The views are the table in `spec.md` ("The views"), in a `VIEWS` list at the top of the
  file: name, map size, seed, window (`laptop` or `phone`), k.
- Each view (the spec-review ran this sequence on main, and it is byte-stable):
  `open('?size=S&seed=N&run=1', { size: window })`;
  `page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000)`;
  find the plains tile nearest world (0, 0) over `game.map.px`/`py` (lowest index on a tie;
  plains is `game.map.l.terrain[i] === game.map.byName.plains`; it is tile 440 on small 2);
  set `game.map.cx`, `cy` to its centre and `k` to the view's, call `viewChanged()`,
  `page.until('!game.map.anim && !game.map.frame', 5000)`, read `cx`, `cy`, `k` back, and
  fail the view if `k` isn't the one asked (clamped by `minK`).
- The shot is the map canvas, not the page: `game.map.drawNow(); return
  game.map.canvas.toDataURL('image/png')`, written to `DIR/NAME.png`. A page screenshot would
  show the pick panel, which covers nearly all of a phone's map. One Chrome per view is fine
  (a few seconds each).
- `DIR/views.json`: per view its url, window, tile index, `cx`, `cy`, `k` and
  `{ ...game.map.stats }`.
- `--diff A B`: for each view in both, the cameras (`tile`, `cx`, `cy`, `k`) must match
  (exit 1 if not), and it prints the view, the share of pixels that differ and the PNG sizes.

Done: two runs of `--out` on the same checkout give identical PNGs (`--diff` prints 0 for
every view). Copy `shots.mjs` onto a main worktree (`git worktree add /tmp/main main`, then
`./build-web.sh` there), shoot `small2-play` on both, and `--diff` them: the cameras match.
Put that output and the two sha256 lines from the repeat build in `relay/specs/439/easel.md`.

## 6. `engine/CLAUDE.md`

Add the two commands to the command block (`node www/assets/draw-atlas.mjs [--check]` the
atlas from placeholders and sources; `node www/shots.mjs --out DIR` fixed map views) and, in
the `www/` row, `assets/` (the atlas: `placeholders.js`, `sources.js`, `cut.mjs`,
`draw-atlas.mjs`). Stay under 8 KB.

Run `engine/check.sh` once at the end. Commit as `#439 build-1: the atlas build and views`,
with `atlas.png` and `atlas.json` as the build wrote them.
