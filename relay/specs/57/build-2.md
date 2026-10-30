# Build 2 (ford): reading the world on the page, and archiving merge4

Piece #57, "Reading the world". Read `relay/specs/57/spec.md` first: its items 12–17 and 19 are
your exact done. build-1 left in the engine: `Sim.notes(id)` (`[{turn, text}]`, newest first),
`rules().notes_shown`, events carrying `alert` (and `amount` on the new `peopleLeft` kind), and
`Sim.load` failing with a reason for a save of another version. Its summary quotes a few notes.
`engine/CLAUDE.md` has the commands; the page is `engine/www/` (`main.js`, `select.js`,
`map-view.js`, `feed.js`, `index.html`), plain ES modules, no build step besides
`./build-web.sh` for the wasm.

Much is already drawn: clan territories and festival routes (`map-view.js` `drawOverlay`), the
selected camp's line (`setLine`, `lineColour`), each camp's zone and held tiles (`drawCivs`),
the closed-country outlines (`drawTerrain`, "Closed country"). Fill the gaps; don't redraw.

## Where things go

- `select.js`: a "Why" list in each panel (item 12): clans in the tile panel, the market there,
  camp, scout, capital (the civ's id). One helper; `ctx.sim.notes(id)`. Take up to
  `rules.notes_shown`.
- `main.js` `fillTip` and `select.js` `tilePanel`: who holds the tile (item 13): a camp holding it
  (`v.units[*].held`), and home / territory / route for its clan.
- `map-view.js`: the selected clan's territory and route bright, others dim (item 14), from a
  `setFocus(clanId | null)` the page calls on selection; the player's camp's line always, thin
  and faint (item 15); the closed-country outlines behind `options.country` (item 16), redrawing
  the terrain layer when it toggles, as the other layer toggles do.
- `index.html` / `main.js`: View → "Hidden country" (`v-country`), key H, off by default; the
  legend's zone rows and `tileKind`'s zone name only when on.
- `feed.js` / `index.html`: the alerts box at the top of the feed panel, alert styling in the
  list, the unread count on the Feed button (item 17). Give `peopleLeft` a fitting icon.
- A new `www/play-read.mjs [out-dir]` (default `relay/specs/57/screens`), on `play.mjs`'s
  `open`, as `play-camp.mjs` and `play-lean.mjs` are: seed 2, small; run to the pick, lead the
  first civ; select a clan's home tile and shoot the panel with its reasons; select the camp and
  shoot it; toggle Hidden country on and shoot the map, then off; move the camp out until an
  alert shows and shoot the alerts box; load a save with `version` removed through the file input
  and check `#error` names the version. It exits non-zero if any of those is missing (no notes,
  no alert within its turn budget, no error). Add it to `engine/CLAUDE.md`'s command list and
  the `www/` row of the layout table.
- Archive `merge4/` (item 19): keep only `README.md` ("Archived: the code is at tag
  `prototype/merge4`; the owner may delete this folder once `ci.yml`'s `js` matrix drops it.")
  and `package.json` (`{"name": "merge4", "private": true, "scripts": {"test": "echo archived"}}`);
  drop its row from the repo `README.md`. Don't edit `.github/` or `docs/design/`.

## Done

`./check.sh` green, `node www/play-read.mjs` passing with its screenshots in
`relay/specs/57/screens/`, and `cd merge4 && npm test` printing `archived`. Look at your
screenshots: a panel whose reasons read as noise, or a map too busy with every route bright, is
yours to fix. Name the screenshots in your summary for the review.
