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
- `tests/cases.rs` names "medium 9 to Lead's target" in `UNSETTLED`: with this piece's picks the target
  is 7 steps out but 83 to 90 days by the known route, and the scout searches 7 to 22 candidates for
  200 turns, reaching none open on turn 200. A scout trapped in a rival's zone now counts as not the
  search's to settle, as a scout too far from its store does (medium 9 to the rival). The piece moves
  every pick, so these cases met new errands; none of the search's rules changed.
- `market::clear` bounds each price within `PRICE_SPAN` of the goods' geometric mean. Without it a
  market where clans bring salt and next to no food (clans with no surplus still bring their salt)
  drove salt's price to 1e-126 and the food prices to 1e31 within 80 turns, then to NaN, and a NaN
  price broke the save's replay (a NaN never equals itself). Main never needed it.
