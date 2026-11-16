# build-1: the suite, the pass and the page

Read `relay/specs/337/progress.md` first: it says the rule the whole step built in
`engine/core/src/units/scout.rs` (`scout_turn`, `shut`, `store_walk`, `kill`, `Doing`,
`doing_text`), `search.rs` (walks take `shut`), `orders.rs` (`army_feeds`), `data.rs` (`SCOUT`),
`cli/src/view.rs` (the scout's line) and `web/src/lib.rs` (a scout's row carries `doing`,
`pack_cap`, `back`, `store_walk`; `wander` and `hunger` are gone). Don't change the rule; a rule
that looks wrong goes in your result as a decision or a note.

## 1. The suite green

- `cargo test --release` in `engine/`: fix every test the rule broke. A test of the old foraging
  (rich ground, auto-forage on a scout, the `Hunger` reason, a scout drawing on its line) is
  rewritten to the new rule or removed; a test of anything else keeps its meaning.
  `core/tests/scout_errand.rs` is a report over the retro maps: make it compile and report turns
  off the errand (`homing` or `back`), routes built and scouts killed; build-2 runs it.
- The digest moved: bump `SAVE_VERSION` and the pins in `cli/tests/save.rs` (`engine/CLAUDE.md`,
  Invariants). `UnitFlat` drops `wander` and adds `homing` and `back`.
- `engine/CLAUDE.md`'s layout line for `units/` and `forage.rs`'s wording stay true; the file stays
  under 8 KB.

## 2. The pass: a stage test

Decided: a mountain a scout has bridged is walked by every actor at its trail cost (a clan, an
army, a line, the scout), and a clan weighs a signpost by the cheapest path it can walk, a pass
included. `land::passable`, `line::walkable` and `settle::draw` read a trailed mountain as open
already; this test proves it and you fix what it shows.

`core/tests/pass.rs`, on a built map (`Stage::flat` or `Stage::parse`): a mountain range that cuts
the map in two with one crossing (`paint(i, MOUNTAIN, Some(cost), Some(1))`; the rest
`paint(i, MOUNTAIN, None, None)`), two civs, the led one with a stocked store.

- The led scout, given a route across, reveals the crossing and builds its trail by a named turn.
- Before it: `units::path_between` across is `None`, an army's `Move` across is refused, and a
  signpost on the far side draws nobody (`settle::draw` is `None`).
- After it: the led army ordered across arrives by a named turn, having stood on the pass tile;
  its line (`units::line`) from the far side runs over the pass; and with a signpost on the far
  side and clans dividing, a clan of the led civ lives on the far side by a named turn (set the
  food so clans divide, or call `settle::draw` over enough rolls and assert it names the signpost,
  then run turns until a home is there).
- The pass costs each walker `step_cost` = its hidden trail cost.

## 3. The page

A scout's panel (`www/panels.js`, about lines 255-330) shows what the engine now has:

- A status line from `u.doing` (the same words as the text view): searching, walking to a store,
  walking back, waiting at a store, trapped, its route is built, idle.
- Pack: food of `u.pack_cap`, in turns; its store and the walk to it in turns (`u.store_walk`).
- No Auto-forage or Auto-supply switch, and no Resupply, Forage or Pillage button, on a scout: the
  engine refuses them. An army's panel is unchanged.
- The Hunger row with the foraging reason goes; hungry turns stay.
- Selecting a scout draws its walk to its store (`store_walk.path`) where an army draws its line.
- `www/check.mjs`, `www/check-stage.mjs`, `www/play/scout.mjs` and `www/play/buttons.mjs` follow:
  every button a scout's panel still has is pressed once, and the scripts no longer look for the
  switches on a scout. `ScoutKilled` gets its entry wherever `ScoutStarved` has one (feed wording,
  `www/assets/sounds.json`).

## Done when

`engine/check.sh` passes whole. Notes for the next session go in `relay/specs/337/notes.md`.
