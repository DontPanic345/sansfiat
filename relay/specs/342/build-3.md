# #342 build-3 (kiln): the page and the text view

Read `relay/specs/342/spec.md` (item 12) and build-1's and build-2's summaries. Keep the clay's
shape: read the herd through `herds::` and `Clan.herds`. Where the clay doesn't hold, make the
smallest fix that works and record it as a decision. No rule or number changes here.

Fields did each of these for its job (#155 build-3): follow it, piece by piece.

## What you build

1. **`web/src/lib.rs`**: the clan row gains `herds` (its share), `herd` (its animals,
   `store[ANIMALS]`) and `grazing` (`[[tile, animals the tile carries for it]]`: its herding tiles and
   each one's room, `min(graze, days × HERD.tend)`, from a `herds::` reader the clay's `carries` uses;
   add `herds::pastures(w, c)` if there is none, as `fields::plots`). The civ's `living` gains
   `herding` (Σ people × herds), and gathering becomes Σ people × (1 − fields − herds); the parts
   still add to its people. The layout's tile gains `graze` (`herds::graze`). The rules block gains
   `herd: { herder }`.
2. **The clan panel**: a **Herd** row for a clan with herds above 0: "24 animals, grazing 9 tiles
   that carry 40", its tip naming the tiles; the doing line for a herder: "Herds a third of its days;
   …", as a farmer's says "Farms …". `www/panels.js` is 592 lines: move the clan's shape into a new
   `www/panels-clan.js` (as `panels-land.js` holds the tile's) before you add to it.
3. **The civ panel's Living row**: "gathering 70 · farming 14 · herding 12 · in the army 6", with
   the tip saying herding.
4. **The tile tooltip**: "grazing 6 animals" on a tile with `graze` above 0, beside its fields.
5. **A herds layer** (`www/map/layers/herds.js`, after `fields.js`; `www/map/view.js` is 605 lines,
   so first move its layer imports and `LAYERS` into a new `www/map/layers/index.js` and import that): on each tile a herder grazes, a
   few small pale marks in the tile's upper half (fields' furrows sit in the lower half), more marks
   with more animals carried. On by default, under View → Herds (`bar.js`'s `LAYER_OF`, the atlas
   entry if layers are listed there).
6. **The feed**: `ClanHerding` reads "Clan 140 takes up herding at (3, −2)" and selects its clan
   on a click (build-2 gave it a kind and a sound).
7. **The text view** (`cli/src/view.rs`): a clan's line gains `, herd 24 animals on 9 tiles` for a
   clan with herds, as `fields_text` does for fields.
8. **`www/play/herds.mjs`** (listed in `all.mjs`), after `fields.mjs`: a staged plains map with no
   river ground, the led civ's clan; play until it herds (cap the turns at the turn the clay's staged
   game in `core/tests/herds.rs` names, plus margin); then the feed has a `clanHerding` item and a click selects its clan, the
   clan panel has a Herd row with animals above 0, the civ's Living row has herding above 0 and its
   parts add to its people and army, the herds layer draws on a grazed tile, and View → Herds turns
   it off. It writes `herds.png` (the herder selected) and `herds-civ.png`. `check-stage.mjs` and
   `check.mjs` gain what `fields` has there.

## Done

- `engine/check.sh` green (it runs `www/play/all.mjs`).
- `node www/play/herds.mjs --out relay/specs/342/` passes, and you look at both screenshots: the
  marks read at a glance on plains and hills and don't hide the furrows or the territories. Say what
  you saw in your summary.
- `engine/CLAUDE.md`'s `www/` row names `panels-clan.js` (stays under 8 KB).

Commit as `#342 build-3: herds on the page`.
