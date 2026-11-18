# #356 build-3 (kiln, page): a store's goods, the flow layer, Fill salt

Read `spec.md` here (7), `docs/design/page.md`, and the earlier builds' summaries. Keep the
clay's shape; where it doesn't hold, make the smallest fix and record it.

## What you deliver

1. **The view model** (`web/src/lib.rs`'s view, `www/model.js`): each store (capital,
   settlement) and pack carries its wares' lots: good, source tile, market tiles in order,
   share. Units carry whether Fill can act now and why not (as other buttons' reasons do).
2. **The panel** (`www/panels.js`, 592 lines: put the new code in `www/panels-goods.js` and
   import it): a store's every good stays listed (`heldGoods`); a ware's line is clickable and
   selects that good, shown marked; clicking it again or Escape clears it. A unit's button row
   gains **Fill salt**, greyed with its tooltip when it can't act (page.md, Decided #103).
3. **The flow layer** (`www/map/layers/flow.js`, new, like `lines.js`): with a store and a ware
   selected, each lot's chain drawn from its source through each market's tile to the store,
   width by share, the source marked. Nothing drawn otherwise: no debug view, no circle.
4. **The play script** (`www/play/exchange.mjs`, listed in `all.mjs`, staged through
   `../play-stage.mjs`): presses Fill on an army at its capital and checks the pack's salt rose
   and the store's fell in `Sim.view()`; selects a store's obsidian (or salt, whichever the
   stage holds) and checks the flow layer drew a chain whose first point is a source tile.
   Add Fill to `www/play/buttons.mjs` as each unit button is.

## Done

`node www/play/exchange.mjs` passes; a screenshot of the flow layer on a real small map
(`node www/play.mjs ... --shot`), saved as `relay/specs/356/flow.png`, shows a chain from a
source; `engine/check.sh` whole once at the end.
