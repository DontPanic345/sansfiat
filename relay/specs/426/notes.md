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
- `tests/cases.rs` names "large 8 to the rival" in `UNSETTLED`. With this piece the pre-game on large 8
  picks capitals 39 steps apart (main: 15), and the scout searches the 66-day route for 200 turns
  without settling: the known cost moves between 66 and 68.3 days as trails wear and fade, and 1 to 13
  candidates reopen every few turns. Main's errand there builds its route on turn 2.
