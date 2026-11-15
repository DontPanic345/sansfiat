# build-1: signposts and settlements on the page

The engine half of #353 is built and tested (`engine/core/src/settle.rs`, `core/tests/settle.rs`;
`relay/specs/353/progress.md` says what stands). The page does not show any of it yet. This step
makes the page do what the piece's Decided line says:

> The page draws signposts and settlements, and the settlement's panel shows its store, its clans
> and its land.

and gives the player the order: **Signpost** and **Remove signpost** (`docs/design/page.md`: the
civ's button row, one panel shape for everything, a greyed button says why).

## What the engine gives you

- Orders (`core/src/orders.rs`): `{"type":"signpost","tile":T}` and `{"type":"unpost","tile":T}`,
  sent as every order is (`Action::Order`). `Sim.check` / `World::check_order` gives the refusal
  text: use it for the greyed button's tooltip and for a refused click.
- `Sim.view()` (`web/src/lib.rs`) has two new arrays:
  - `signposts`: `{ civ, tile, turn, worth }` (worth: the share of a daughter's need the tile
    feeds, 1 = in full).
  - `settlements`: `settle::Panel` rows: `{ id, civ, tile, market, founded, store[5], cap,
    levied, clans[], traders[], land[] }`. `clans` live on the tile, `traders` come to its market,
    `land` is the part of the civ's claim nearer this settlement than the capital or another.
- Events `settlementFounded` and `settlementTaken` carry their wording (`Event::text`); the feed
  shows them as it shows any event. `settlementTaken` is an alert for the led civ.
- Notes: a settlement has reasons under its own id (`w.notes[settlement.id]`), as a clan has.

## Build

1. **A map layer** `www/map/layers/settle.js`, in the pattern of the other layers there: a signpost
   is a small post icon in its civ's colour on its tile; a settlement is a mark that reads as a
   seat smaller than the capital, in its civ's colour. Draw to the atlas cells if the atlas has
   room (`atlas.js`, placeholders first), else plain canvas shapes. No ring or radius.
2. **The settlement's panel**: selecting a settlement's tile (as a festival or a tile is selected;
   see `panels-land.js` and `panels.js`) shows one panel in the common shape: its name line
   ("Red's settlement at (q, r)"), its store against the cap with the levy this turn, the clans
   living there and their people, the clans that come to its market, its land (count; the tiles
   highlighted on the map while selected, if the tile panel does that for a claim), and its
   latest reasons. Every number has a tooltip.
3. **The buttons**: on the civ's row, `Signpost…` arms a click: the next tile clicked sends the
   order (Escape clears it, as for Move to… and Claim). It is greyed with the engine's reason when
   the civ keeps `SETTLE.signposts` (3) already. Selecting a tile with one of the led civ's
   signposts shows the signpost (posted N turns ago, worth) and a `Remove signpost` button.
4. **Checks**: `www/check.mjs` (and `check-stage.mjs` for the panel shape) cover the two view
   arrays and the panel as data; `www/play/buttons.mjs` presses Signpost… and Remove signpost once
   each on a staged game and checks the world changed (`engine/CLAUDE.md`: a button's proof is
   `buttons.mjs`); a new `www/play/settle.mjs`, added to `all.mjs`, posts a signpost on a staged or
   retro game, runs turns until a settlement is founded (or stages one: a clan of the civ living on
   the signposted tile founds it at the turn's `settle` step), selects it and reads its panel, and
   saves one screenshot to `relay/specs/353/screens/settle.png`.
   If a staged `Sim` cannot place a signpost or a clan where you need it, extend `StageSpec`
   (`core/src/stage.rs`) minimally.
5. `engine/CLAUDE.md`'s layout table names the new files (it must stay under 8 KB and has about 100 bytes of room: shorten a row to fit), and
   `relay/specs/353/progress.md` item 6 is ticked.

## Done

`engine/check.sh` passes whole. The screenshot shows a signpost and a settlement that read as
what they are. Nothing in `engine/core/src` changes but `stage.rs` if needed: the rules are
settled, and the sweep numbers in `relay/specs/353/scores.md` must not move (`stats::digest` on a
run is unchanged by this step).
