# #350 build-1 (ford): the engine and the text view say what they do

Read `relay/specs/350/spec.md`, items 1 to 5. Every item is view text, a derived reading or a note. None moves the
world. If a change you make moves `stats::digest`, `SAVE_VERSION` or a pin in `cli/tests/save.rs`, you've changed a
rule: undo it. Don't touch `www/screens/`, `www/bar.js` or `www/play/camp.mjs` and `civ.mjs`, which are build-2's.

## Code

- `cli/src/view.rs`: `legal` (the claim arm, ~484), `claim_lines`, `settle_lines`, `unit_line` (~357) and
  `draws_on`.
- `cli/src/view_goods.rs`.
- `core/src/outlook.rs`: `reach`, `ClaimTo`.
- `core/src/claim.rs`: `check`, `change`.
- `core/src/settle.rs`: `worth`, `draw`, `roots`, `store_name`.
- `core/src/units/line.rs`: `line_at`, `line_among`, `stores`, `store_of`.
- `core/src/units/camp.rs` (~356, the supplying note).
- `web/src/lib.rs` (~330, `signposts`).
- `www/panels-land.js` (~81, the signpost rows).
- Tests: `cli/tests/play.rs`, `core/tests/settle.rs` (`staged`, `rich`, `clan`), `core/tests/stores.rs` (`stage`,
  `a_line_ends_at_the_nearest_store_that_holds_a_turn_of_food`), and `cli/src/game_judge.rs:366` for an in-crate
  test module.

## Build

1. **The claim order.** Do this as spec item 1 says. The legal line carries the current claim's ids, without the
   roots, ready to copy. It says the order sets the land to exactly the listed tiles, and that a tile left out is
   dropped, with the clans whose homes are on it. When `outlook::reach(w, me).claim` is some, a second line gives
   the whole order (the claim's ids plus the reach's tiles) and names what it adds and the clan it brings.
   - Test in `cli/tests/play.rs`, small 2 civ 1 at the pick. Every id in the `land:` line is in the claim line. Find
     the reach order, send it, and check it is accepted. The land then grows by exactly its tiles, the clan is the
     civ's, and Standing is not lower. The pick's view stays under 90 lines.
   - The spec sent that order by hand on main (43 ids plus 307): accepted, Standing 21.5% to 22.3%.
2. **The idle signpost.** A reading in `settle.rs` (spec item 2) is `None` while the tile feeds above
   `SETTLE.fed`. Otherwise it says why the signpost draws nobody, naming the clan that gathers the tile when one
   does. Show it in `settle_lines`, in a `signposts[].idle` field in `web/src/lib.rs`, and on the signposted tile's
   panel in `panels-land.js`. If `www/check.mjs` lists view keys, add the field there.
   - Test in `core/tests/settle.rs`: free and rich, then a clan's claim on the tile. The reading names the clan, and
     `settle::draw` for a clan of the civ returns `None`.
3. **Where the army stands.** In `unit_line`, the empty-line case names the store from `settle::store_name`, or
   the market at the line's end. `draws_on` adds the line's days.
   - Test: an army on a settlement in `stores.rs`'s world. The line names the settlement and doesn't say "at its
     store". Use an in-crate `#[cfg(test)]` module in `view.rs`, or whatever reaches `unit_line`.
4. **Why that store.** `line.rs` gains a reading of each store's line days (or `None`) and whether it holds a turn
   of the unit's food. It is built on `line_among`, so it can't disagree with `store_of`. In `camp.rs`, when
   supplying starts or the store drawn on changes, write the note naming the store and its days, and each other
   store's days or why it was passed over (under a turn of food, cut off). Never write it every turn.
   - Two tests in `stores.rs`, as the spec says. One makes the capital's tile dear to enter, so the settlement wins
     and the note names both with their days. In the other, the capital holds under a turn.
5. **Salt.** It is already met on main: the store line uses `max(0.0)`. Add a guard: a store with `-0.0` and
   `-1e-12` salt prints `salt 0.0`. If that needs it, move the formatting into a `stock(store)` fn in
   `view_goods.rs`. Say in your summary that this test passes on main.
6. **Check against main.** For items 1 to 4, revert each fix and check that its test fails. Put the failing line of
   each in your summary.
7. **The session log.** Write `relay/specs/350/view.md`:
   - small 2 led as civ 1, with the pick's claim lines and the reach order sent and its result
   - a signpost posted, and its line after `end 30`
   - an army raised, its line, and `why <army>`
   - plain excerpts, no commentary beyond a line each

## Done

- The tests for items 1 to 4 fail on main and pass here, and item 5's guard passes.
- `engine/check.sh` passes whole, with the save pins and `SAVE_VERSION` unchanged.
- `view.md` exists.
- `engine/CLAUDE.md` needs no change unless you add a module. Keep it under 8 KB.

About 9 files and 250 lines with tests.
