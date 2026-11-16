# #337 notes for the next builder

- `cargo test --release` stops at the first failing binary. After the whole step it is
  `core/tests/cases.rs`: `a_settled_route_has_no_cheaper_trail`,
  `a_scouts_search_on_small_9_to_the_forest_is_in_order`,
  `settled_scouts_leave_nothing_unrevealed_that_could_beat_them`. They drive scouts on retro maps
  under the old feeding; the binaries after it (`cargo test --release --no-fail-fast`) were not read.
  `core/tests/scout_feed.rs` passes; `web` and `cli` compile; the page's scripts were not run.
- `SAVE_VERSION` and the pins in `cli/tests/save.rs` are not bumped yet.
- `scout_turn` takes a full pack (48 food at `SCOUT.pack` 24) from a store whose cap is 60 and which
  holds about 10 at the pick: watch for a scout that waits at `Filling`, and for a store the scout
  empties before an army is raised. An idle scout at a store holds two turns of food.
- A scout made at founding still takes an army's pack (`pack_cap`) from the store, not a full one.
- `Doing::TooFar` reads `back`, the tile the scout turned round on; a fresh route never says it
  before the first walk out.
- Waiting on a long command: `pgrep -f` in a wait loop matches its own shell. Run the command in
  the foreground with a long timeout instead.
