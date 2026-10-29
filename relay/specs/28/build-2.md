# #28 build-2: the scout and the search (ford)

Read `relay/specs/28/spec.md`, Settled 5–9 and 12, and build-1's result. Port
`routePlan`/`jobs`/`scoutWork` from `scout/src/rules.js` (also at tag `prototype/scout`) onto
the engine, reusing the camp's code in `units.rs` (forage, line, eat, hunger) rather than
copying it.

Deliver:
- `UnitKind::Scout`, `SCOUT` rules in `data.rs` (people 2, pack, reveal labour), `Unit.route`
  (`serde(default)`); one scout per civ made at founding (Settled 5), also with `--no-army`.
- `known`/`possible` pricing per civ and the plan (Settled 7), as a pub fn the web view can
  call: `units::route_plan(w, civ, from, to) -> Plan { known: Option<(Vec<usize>, f64)>,
  possible: …, candidates: Vec<usize> }`.
- The scout's turn in the `units` step (Settled 8) with the outward order; then forage and
  minimal resupply; then eat.
- `Action::SetRoute { from, to }`, `Action::ClearRoute`, validated with reasons, logged,
  replayed. Check camp code that assumes every unit is a camp (`Raise`, `MoveCamp`, AI).
- Events (Settled 9).
- `lead_orders` in `cli/src/main.rs` and `cross-build.mjs` give a route; `digest` counts
  trails and revealed tiles.
- Tests in `core/tests/scout.rs`: unrevealed pricing; it reveals the nearer mountain of a range
  before the far end (a built map); no route can exist on a sealed target; it builds and trails
  cut a clan's or the camp's path cost; replay holds; the numbers proof below.

Numbers: tune `SCOUT.*` and `TRAIL.work`'s scale so a scout settles and trails a 6-tile
forest/hills route within 20 turns and a mountain pass within 40, without starving. Record them.

Done: `engine/check.sh` and `./cross-build.sh` pass. build-3 relies on `route_plan`, the
scout's `route`, `Civ.revealed`, `Tile.trail_work`, and the event texts.
