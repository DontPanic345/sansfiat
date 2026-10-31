# Build 6 (ford): the page's orders and the play scripts

Piece #100. Read `relay/specs/100/spec.md` item 11 and build-4's wire form. The engine's actions for a
camp and a scout are now `{"type":"order","civ":C,"order":{…}}`; the page and its scripts still send the old
shapes. **The page looks and behaves as it did.**

## The work

1. **`www/main.js`**: `order(action)` and its callers (`zone` at ~230, `setRoute` at ~483, `moveCamp` at ~501,
   and whatever else sends one of the ten old actions) send the new form. Add one helper, `unitOrder(unit,
   order)`, that wraps `{ type: 'order', civ: state.player, order: { …, unit } }` so each call site is one
   line; `chooseCiv`, `endTurn`, `setGift` and `setKeep` stay as they are.
2. **`www/select.js`**: the unit panel's buttons (`raise` at ~173, `disband`, `resupply`, `forage`, `pillage`,
   the switches, `Stop` (a `move` to the unit's own tile), `Undeploy` (a `zone` with no tiles), `Clear route`) and the
   civ panel's `Raise warrior` build their `action` through the same helper with the selected unit's id (`u.id`) or,
   for raise, the civ's (`c.id`).
3. **`www/check.mjs`**: every `sim.check`/`sim.act` of an old action (about twenty) uses the new form, and it
   gains: a `move` for a unit that is not yours is refused with "that unit is not yours"; an order for a civ the
   player doesn't lead is refused; the old `moveCamp` JSON fails to parse.
4. **The play scripts** drive the page by its buttons and clicks and send no action JSON themselves (the
   spec-review grepped them), so they need no edit. Leave them as they are.
5. `web/src/lib.rs` doc comments name the new form where they name the old.

## Done

`./check.sh` green (build it first: `./build-web.sh`, which `check.sh` runs). `bash relay/specs/100/baseline/plays.sh
/tmp/plays` prints five `ok` lines: each play's stdout and exit code equal main's. Two plays fail on main and must
fail the same way (`play-camp` exits 1 at its line 86 after its camp starves, `play-lean` exits 1 finding no leaning
clan): do not fix them, that is not this piece. Open each screenshot under `/tmp/plays` and say in your summary what
you saw in one clause each. The Raise, Disband, Resupply, Forage,
Pillage, switches, Stop, Undeploy and route buttons all work from the panel (exercise the ones the plays don't).
