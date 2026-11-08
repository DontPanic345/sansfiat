# #155 build-3 (easel): the page

The player meets fields. The civ panel shows how its people live, the clan panel shows a farmer's fields,
fields are drawn on the map, and the tile tooltip says what ground a tile has for them. Read `spec.md` item 9,
the look under "What I settled", and decision 12. `docs/design/farming.md`, What the player meets, and
`docs/design/page.md` are the design. Build-1 left `fields::{ground, yield_a_day, farmer, plots}`,
`Clan.fields` and `Tile.field`. Build-2 put `clanFarming` in the feed.

The code: `web/src/lib.rs` (`layout`, `view`), `www/model.js`, `www/panels.js` (569 lines), `www/map/view.js`
(the layer list and the View toggles), `www/map/layers/floodplains.js` and `hearths.js` (static and turn layers
to follow), `www/bar.js` (the View menu), `cli/src/view.rs` (the text view, under 80 lines),
`www/check-stage.mjs` and `www/check-ctx.mjs` (panel shapes as data), `www/play/` and `www/play.mjs`.

## What to build

1. **The view.** `layout` gains `ground`, each tile's field days (`fields::ground`). It is static. `view`
   gains, for each clan, `fields` (its share) and `plots`, as `[[tile, days, grain]]` from `fields::plots`.
   For each civ it gains `living`: `{gathering, farming, army}`. Gathering is Σ people × (1 − fields) and
   farming is Σ people × fields over its clans. Army is the people of all its units: camps, the guard and its
   scouts. Document the keys in `view`'s doc as the others are.
2. **The civ panel.** A `Living` number after People: "gathering 82 · farming 14 · in the army 6", rounded to
   whole people. Its tooltip: "Its people by what they do: gathering over their land, working fields, or in its
   units".
3. **The clan panel.** For a clan with fields, a `Fields` number, "3 tiles · 9.4 days · 6.1 grain". Its tooltip
   lists each tile ("(3, −2): 4.0 days, 2.6 grain") and says "Days a turn on fields, and the grain they
   yielded". The doing line leads with how it lives: "Farms a fifth of its days" when it is a farmer, then
   what it says now. The fraction is the share in words (a tenth, a fifth, three tenths, …, all), from a
   helper in `panels.js`. A clan with share 0 shows no Fields row.
4. **The tile tooltip** (`tileKind` and the tile panel): a tile with ground says "field ground 20 days", and a
   tile with fields adds "fields 9.4 days (clan 140)".
5. **The fields layer**, `www/map/layers/fields.js`, a turn layer drawn over the terrain and under the
   territories. On every tile with `Tile.field > 0`, three short parallel furrows in the lower half of the hex,
   in a dark ochre (`#7a5a1e`). The furrows grow more opaque with the tile's field days, from 0.35 at one day
   to 0.9 at its whole ground. It is on by default, under View → Fields, and the atlas gets an
   `overlay.fields` cell in `atlas.json` with the same brief. Draw the strokes until the cell is drawn, as
   `floodplains.js` falls back.
6. **The text view** (`cli/src/view.rs`): a selected or listed clan with fields prints "fields 3 tiles, 9.4
   days, 6.1 grain". Stay under 80 lines.
7. **`www/play/fields.mjs`**, added to `all.mjs`. Lead a civ on small seed 9 with `play.mjs`'s `lead` (`{ seed: 9 }`, the
   first civ). Play turns until some clan of the led civ has `fields > 0`, at most 40. Then:
   - the feed holds a `clanFarming` item, and clicking it selects that clan;
   - that clan's panel has a Fields row, and its first tile is a tile with `ground > 0`;
   - the civ's panel has a Living row whose farming part is above 0 and whose three parts sum to its people,
     within rounding;
   - the fields layer is on and draws on that clan's field tile;
   - View → Fields turns it off.
   Write `fields.png`, the farmer selected with its fields drawn, to `--out`. The script exits non-zero when
   any check fails, as the others do.
8. **`check-stage.mjs`**: the clan panel's shape with and without fields, and the civ's Living row, as data.

`panels.js` is 569 lines. If these additions take it past 600, move the clan, festival and tile panels into
**`www/panels-land.js`**, re-exported from `panels.js` so no caller changes.

## Proof and record

- `engine/check.sh` green, `node www/play/fields.mjs` passing, and `node www/play/all.mjs` passing.
- Look at `fields.png`. A glance should find the fields among the floodplain wash and the territories. If it
  can't, change the colour or the weight and say what you changed. Commit `fields.png` and a screenshot of the
  civ panel (`civ-panel.png`) under `relay/specs/155/`.

## What later steps rely on

Nothing in core. Build-4 may change `FIELD`, and the page reads it through the view.

## Done

`check.sh` and `all.mjs` green, the screenshots committed.
