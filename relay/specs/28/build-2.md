# #28 build-2: the scout and the search (ford)

Read `relay/specs/28/spec.md`, Settled 5–9 and 12, and build-1's result. Port
`routePlan`/`jobs`/`scoutWork` from `scout/src/rules.js` (also at tag `prototype/scout`) onto
the engine, reusing the camp's code in `units.rs` (forage, line, eat, hunger) rather than
copying it.

Deliver:
- `UnitKind::Scout`, `SCOUT` rules in `data.rs` (people 2, pack, reveal labour), `Unit.route`
  (`serde(default)`); one scout per civ made at founding (Settled 5), after the camp, its pack
  from what the store has left; also with `--no-army`.
- `known`/`possible` pricing per civ and the plan (Settled 7), as a pub fn the web view can
  call: `units::route_plan(w, civ, from, to) -> Plan { known: Option<(Vec<usize>, f64)>,
  possible: …, candidates: Vec<usize> }`.
- The scout's turn in the `units` step: spoil; feed first (Settled 5: forage only while on a
  route, then the line for the rest of this turn's need, never more); the search (Settled 8)
  on the labour left, with the range-widened outward order and the fallback to building a
  reachable trail on the possible path when no reveal can be reached; eat. Tiles it steps
  into are marked `walked`.
- `Action::SetRoute { from, to }`, `Action::ClearRoute`, validated with reasons, logged,
  replayed. Fix camp code that assumes the civ's unit is its camp: `units::camp` (filter on
  `Camp`), `disband` and the starving path (by id), `Raise`, `MoveCamp`, the switches, the
  `ai` step, `Counters` and the sweep's per-civ camp line (a scout is not a camp).
- Events (Settled 9).
- `lead_orders` in `cli/src/main.rs` and `cross-build.mjs` give a route; `digest` counts
  trails and revealed tiles.
- Tests in `core/tests/scout.rs`: unrevealed pricing; on a built map with a straight range
  between `from` and `to`, the scout beside it and only a far mountain bridgeable, it reveals
  the range's mountains nearest-first from where it stands and never steps back past an
  unrevealed nearer one; a target in a valley sealed by mountains gets its pass built and a
  route found; no route can exist when no wall mountain can be bridged; trails it builds cut
  a clan's festival path cost and the camp's line cost; an idle scout at the capital neither
  forages nor starves; replay holds; the numbers proof below.

Numbers: tune `SCOUT.*` and `TRAIL.work`'s scale so a scout settles and trails a 6-tile
forest/hills route within 20 turns and a mountain pass within 40, without starving, and every
trail it built is still standing when route found comes (`TRAIL.fade` is 30). Record them, and
the turns each test took, in your summary.

Done: `engine/check.sh` and `./cross-build.sh` pass. build-3 relies on `route_plan`, the
scout's `route`, `Civ.revealed`, `Tile.trail_work`, the event texts, and the seed, size and
target tile of `lead_orders`' route (name them in your summary: `play-scout.mjs` plays it).
