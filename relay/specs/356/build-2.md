# #356 build-2 (kiln, engine): the Fill order, the planner trading, the trade group, the view

Read `spec.md` here (4, 5, 6, 7) and build-1's summary, `pass.md` and `notes.md`. Keep the
clay's shape (`Lots`, `exchange::unit_rows`, `Config.exchange`); where it doesn't hold, make the
smallest fix that works and record it as a decision.

## What you deliver

1. **`Order::Fill { unit }`** (`core/src/orders.rs`, validated where the other unit orders
   are). The unit stands on a store of its civ (`settle::store`); it takes salt, with its lots
   (`Lots::mix` into the pack), up to `UNIT.salt` a person less what it holds (`data/units.rs`,
   start 1.0). Refused with a reason off a store, with no salt there, or with a full salt room.
   `actors::civ_holds`' salt becomes `cap / KEEPING.keeps + KEEPING.packs` (`data/goods.rs`,
   start 10): this is the levy's and the market's want too, so the economy moves; that's
   expected. Test in `core/tests/exchange.rs`: Fill on a capital moves salt from store to pack;
   off a store it is refused. A disband gives the pack's wares and lots to the clan it joins
   (build-1 may have done it; check).

2. **The planner** (`core/src/units/trade.rs`, new: `ai.rs` is past 600 lines, so the new
   rules go here and `ai.rs` only calls them). With `exchange` on: an army the planner orders
   past its ring (a march toward a rival) is first given Fill where it stands, if it stands on
   a store with salt. An army with salt in its pack, whose line can't feed it this turn, and
   whose pack holds under `PLAN.trade_at` turns of its food (start 2.0), is ordered to the
   nearest market that isn't a rival's within its pack's turns, and holds there while it buys.
   These are orders through the log, as every planner order is. Starting numbers that run; the
   glaze tunes them, so don't tune.

3. **The judge's trade group** (`core/src/judge_trade.rs`, new, read by `judge.rs` as
   `judge_goods.rs` is): exactly as `spec.md` (6) defines it. Whatever the clearing must keep for
   it (each row's net, the prices) is kept on the `Market` as last turn's, `#[serde(default)]`,
   as `ware` is. **Army-turns fed at a market not the civ's** count turns where an army bought
   more than 1e-6 food at a market neither its civ's capital market nor its settlements'.

4. **The sweep** (`cli/src/sweep_trade.rs`, new; `sweep.rs` adds the config): the
   `exchange`-off control beside each map, a line a map and the totals: army-turns fed at a
   foreign market, at its own; each good's share by count and by worth at +200 and over the
   watch, the largest of each; the control's. In the JSON for `judge --compare`. `judge FILE`
   shows the group (`cli/src/game_judge.rs`).

5. **The text view** (`cli/src/view.rs`, 565 lines: put the new lines in `cli/src/view_goods.rs`
   if it would pass 600): a unit's pack wares and the Fill order in its help; a store's wares
   as chains (`obsidian 4.2: 60% from (3, 9) via the festival at (5, 7), the capital's market`),
   the largest lots first.

## Done

Tests above pass; one sweep (`sweep --judge --json` > `sweep-kiln.json` here, about 4–5
minutes) shows the trade group's rows and the control's, with the first number's value
whatever it is, written in `kiln.md` beside the control. `engine/check.sh` whole once at the
end; `SAVE_VERSION` and the pins bumped if the digest moved; `engine/CLAUDE.md` names the new
files.
