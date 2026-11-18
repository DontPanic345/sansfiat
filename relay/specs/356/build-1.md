# #356 build-1 (clay): lots, a unit in the clearing, and how a good passes between markets

You shape the core of Exchange. Read `spec.md` and `probe.md` here first, then `docs/design/goods.md`,
`actors.md` (Markets) and `clans.md` (Festivals). The code: `core/src/market.rs` (`clear`),
`core/src/markets.rs` (`trade`, `go`, `walkers`), `core/src/civs.rs` (`levy`),
`core/src/actors.rs` (`spoil`, `keeps_back`, `civ_holds`, `civ_worths`), `core/src/clans.rs`
(the harvest builds `basket` from store, food and `reaped.dug`), `core/src/settle.rs`
(stores), `core/src/units/` (a unit's `store` is its pack), `core/src/judge_goods.rs` (how far
goods move).

## What you deliver

1. **`Config.exchange`** (`data/game.rs`, on by default, old saves load with it on). Off is
   the control: no unit trades, no good passes, no lot is written. **Done:** with it off, all 21
   digests of `relay/specs/340/digests.sh` (run from `engine/`, after setting the default off
   locally or by a flag you add) equal `relay/specs/340/digests-main.txt`. Write how you ran it
   in `pass.md`.

2. **`core/src/lots.rs`**: `Lot`, `Lots` and their methods as `spec.md` (1) gives them, the
   shape is yours to improve but keep: shares a good, a chain of source then markets, `mix` on
   arrival only, and two caps in `data/goods.rs` (`LOTS.most`, start 4; `LOTS.chain`, start 6).
   Attach `lots` to `Clan` (its store and basket are one place across the turn: the harvest
   builds the basket from the store, the eat step writes it back), `Civ`, `Settlement` and
   `Unit`, each `#[serde(default, skip_serializing_if = "Lots::is_empty")]`. Call `mix` at every
   arrival of a ware: the dig, the clearing, the levy, a merge (`markets::merge`), a daughter
   (`dividing.rs`, which takes a share and so copies the lots), a settlement taken, a disband.
   **Invariant**, tested in `core/tests/lots.rs`: for every store and every ware with more than
   1e-9 held, its lots' shares sum to 1 within 1e-9, and every chain starts at a tile whose
   source is that good. A test runs a small real world 60 turns past the pick and checks it on
   every store every turn (seconds, not minutes).

3. **A unit as a clearing row** (`core/src/exchange.rs`, new; `markets::trade` calls
   `exchange::unit_rows` and hands the results back). Rules in `spec.md` (2): a unit on a
   market's tile that isn't a rival's joins its clearing; brings its pack's wares (only the salt
   that buys its pack's room at last prices), worth 0 to it; wants food at 1; its lots go
   through the pool like a clan's. A unit's row gives no leaning. Note each unit's trade
   (`w.note(unit, ...)`) and keep, on the unit, last turn's food bought and the market, for the
   judge (`#[serde(default)]`). Test on a staged map (`core/tests/exchange.rs`): a camp with salt
   in its pack on a festival's tile with clans that want salt ends the trade step with more food
   and less salt; on a rival's capital market it trades nothing.

4. **Carry** (`exchange.rs`), as `spec.md` (3) gives it. Spec-review settled Carry against
   Lines by probe (`probe.md`): build Carry only. The second route reads the same
   `festival_costs` search its home already did this turn; its walk is `walk_cost` on the
   wares' weight (`carriers`), paid from its days as a route; it trades at the second festival
   in the same `trade` step, after the first festival's clearing (a second pass over markets for
   these clans only is fine). What you find by running: when the walk is worth a clan's days
   (the wares' worth at that festival's last prices against the walk, as `worths_from` weighs a
   festival, is a fair start) so that starving clan-turns and people stay within
   `judge --compare`'s floor of main. Obsidian sells only at a capital with room (`probe.md`),
   so don't expect its steps to move much; a probe-sized flow of a few hundred units a map is
   right. Run `cargo run -p sansfiat --release -- sweep --judge --json` (about 3 minutes) and
   write in `pass.md`: obsidian's and salt's `mean_steps_200` and `max_steps_200` beside main's
   (`sweep-main.json`: obsidian 5.35 / 18, salt 3.52 / 11), the wares carried a map, festivals
   where a ware trades, and the health line (people over control, starving). Carry's numbers
   go in `data/goods.rs` with a `note`.

## What the kiln relies on

`Lots` and its invariant; `exchange::unit_rows` taking any unit with wares in its pack;
`Config.exchange`; the unit's last trade on the unit; lots on every civ store and pack. Keep
names plain; write the kiln's interface in `notes.md`.

## Done

The four above, `engine/check.sh` whole once at the end, `SAVE_VERSION` bumped with the pins in
`cli/tests/save.rs` (exchange on moves the digest), and `engine/CLAUDE.md`'s layout naming
`lots.rs` and `exchange.rs` (it stays under 8 KB). `markets.rs` must not pass 600 lines.
Decisions you make go in `result.json`.
