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

## build-1 (the suite, the pass, the page)

- Suite green. `SAVE_VERSION` 24 and the pins in `cli/tests/save.rs` bumped; `UnitFlat` has 33 fields (`homing`, `back` for `wander`).
- `core/tests/cases.rs`: `refill` fills the scout to `cap_of` and the civ's store to twice that. `settle` returns
  `None` for an errand that waits `Doing::TooFar` at the end (medium 9 to the rival: further than a full pack
  there and back); the two invariant tests skip those errands. The search-order test still runs them all.
- Orders to a scout no longer note "past its line's reach" (`reach_note` is gone); `units::past_reach` stays for armies.
- `core/tests/pass.rs`: the pass on a staged wall (q = 0, one crossing). The scout builds it, an army marches over it, its
  line crosses it, and a clan divides to a signpost across. It passes on the rule as built; no fix was needed.
  The army test asserts the march (`units::route`) crosses the pass tile: an army strides several tiles a turn, so
  "stood on it" is not observable turn by turn.
- `scout_errand.rs` reports turns off the errand (`homing`/`back`), routes and kills; its 4-lost bound is unchanged and holds.
- Page: the scout's panel shows `u.doing`, Pack and Store (`store_walk`), no switches or feeding buttons, no Hunger row; the
  selected scout draws its walk to its store; the tile tooltip's reach warning is gone for scouts; `scoutKilled` is in
  feed.js, sounds.json (no .ogg exists for either) and check.mjs KINDS. `buttons.mjs` cases for Forage/Resupply/Auto-* keep
  their scout branches, which no longer run (buttons are enumerated from the panels).

## build-2 (the retro maps)

- `SCOUT` stays pack 36, spare 3, out 0.25 (`errand.md`, with the grid). `SAVE_VERSION` is 25 and the pins moved: the fix below.
- Bug found by tracing a starved medium 8 scout: a pass the scout built wore away behind it (unused trails go), `store_walk`
  was `None`, and it starved "searching". `scout::keep_way` marks the built passes between a scout and its nearest store used
  each turn; `core/tests/pass.rs::a_scout_past_its_pass_keeps_the_way_home` holds it. The first sweep's numbers for spare 1
  and 2 losing scouts were this bug.
- `core/tests/scout_routes.rs` is the report (`--ignored --nocapture`, `FIGHT=on|off`; a minute for both halves). `errand-main.txt`
  was made by the same test minus the new columns, on `main` in a worktree.
- The led game's lines files replay: `sansfiat play --game G --seed 61288 --size small --civ 1 [--no-fight] < lines`, where the
  lines are the `line` fields of `game/*.lines.jsonl`. After the fix `branch-fight-army` builds its route at +78 and the
  scout isn't killed (the lines are unchanged).
- Medium 9's scouts spend most of an errand walking to a store and back (the rival is more than 14 turns' walk): the store
  isn't what stops them, except civ 846's, which gains about a food a turn.
