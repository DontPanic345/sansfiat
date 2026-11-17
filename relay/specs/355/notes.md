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

## Review (#355)

- A clan at its hold (`SOURCE.hold` a person) of a good still gives `SOURCE.share` of its days to the
  source: `fields::jobs` splits the days before `clans::reap` caps what is dug, so the days are lost, not
  refunded. A clan that holds obsidian nobody at its festival wants reaches the cap and then wastes a
  twentieth of its labour every turn. The design says a clan gathers "only up to what it cares to hold": the
  fix is to pass the cap into `jobs` so a capped clan has no source job. It moves digests and the sweep, so
  the review leaves it. It may be part of the starving cost (1.23 times the no-sources control).
- `actors::spoil`'s `used` can round to 1 plus an epsilon when a store has far more food than its salt keeps,
  leaving a salt of about -1e-17; the text view clamps it. Clamp `used` at 1 in the engine with the next
  change that moves digests.
- `scores.md`'s first draft had three numbers that `compare.txt` does not (lead changes, town clans,
  settlements founded); corrected there. The judge's figures are `compare.txt`'s.
- The sweep json holds the no-sources control's people, starving and festivals only, not its Standing,
  doubt or towns: what the sources did apart from the wider baskets, the cap and the levy is not
  separable from the report. Exporting the control's scores is the way to see it.
