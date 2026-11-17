# #355 notes for the next builder

- Baskets are `[f64; N]` with N = 7: the five foods, then `SALT`, `OBSIDIAN`. `market::total` and
  `market::take` are food only, so a pack never takes salt. Anything that sums a basket with
  `.iter().sum()` counts salt as food: use `total`. The whole step fixed every site it found in
  `core`, `cli/src/view.rs` and `report.rs`; `cli/src/retro.rs:130`, `view.rs` pack sums and
  `web/src/lib.rs:202` sum unit packs, which hold food only, and are right as they are.
- Tile baskets (`cap`, `stock`, `take`) have the two extra slots, always 0: a source is
  `Tile.source`, not a stock.
- A clan's salt is used by its own store at `eat` (`Actor::spoil`), so `Clan.store[SALT]` at the turn's
  end is what was left over; `Clan.basket` shows what it had at the harvest.
- Civ stores read 0 salt at +200 in the sweep because the store uses it the turn it arrives:
  `civ_most` is the number that shows salt reaching them.
- `Market.wares` and `Market.ware` are last turn's volume of salt and obsidian, for the judge.
- The planner does nothing new: the AI's civ gets both goods by levy and market, and `raise` arms
  from the store. A planner that claims toward obsidian is an idea, not built.
- `data.rs` (about 1100 lines) and `map.rs` (1250) are past the 800-line rule; both were before
  this piece. `sources.rs` holds the placing so `map.rs` grew by four lines.
- `sweep --judge` runs nine configs a map now and takes about 4 minutes.
- None of the milestone's open ideas was taken.
