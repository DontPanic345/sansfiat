# #426 notes for the next builder

- `markets::gain` recomputes `keeps_back`, `worths` and the basket's nutrition on every call, and
  `load` calls it once a load. A clan far from its festival stops after one or two loads; a clan at
  the door returns without the loop. `homes::Prices::routes` calls `route` and `load_to` for each
  candidate home, so a farmer weighs two or three loads per candidate. Hoist the shared parts if a
  bench shows it.
- `Clan.walkers` is what `go` priced; `trade` reads the walk back as route cost ÷ walkers. A route
  set anywhere else (a test, a new rule) must set `walkers` too, or `trade` takes the whole cost as
  one walker's walk.
- `tests/walk.rs` (the #154 report) now bins clans by what their walkers can carry, not by what they
  brought.
- `tests/cases.rs` is main's again: with this piece's final picks every errand settles within 200
  turns, so the branch's `UNSETTLED` exception went.
- `market::clear` bounds each price within `PRICE_SPAN` of the goods' geometric mean. Without it a
  market where clans bring salt and next to no food (clans with no surplus still bring their salt)
  drove salt's price to 1e-126 and the food prices to 1e31 within 80 turns, then to NaN, and a NaN
  price broke the save's replay (a NaN never equals itself). Main never needed it.
- `Clan.nursing` is set in `eat` and read in `fields::jobs` next turn; a daughter starts at 0 and her
  mother keeps the whole debt. `Clan.born` and `Clan.crowded` are `serde(skip)` readings `eat` writes
  each turn, for the judge only.
- `eat` adds the home tile's people up once a turn (a map over the clans), before anyone grows.
- Main's sweep in `main-sweep.json` is main with `Clan.born` and the judge's settling counts patched in;
  `born` is read as the people after growth less before, so main's digests are unchanged.
