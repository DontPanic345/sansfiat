# #28 Scouts open the country

## What it delivers

Every forest, hills and crossable mountain tile already carries a hidden `trail_cost` (and a
mountain its `span`), rolled in `map.rs` on its own stream and read by nothing. This piece makes
trails use them, gives each civ a **scout** (an actor like the camp) that finds the best route
between two tiles the player picks and builds trails along it, and shows it all on the page.
Because trails belong to the tile, a built trail cuts every route over it: clans' festival
routes and walks, the camp's line. `scout/` is archived to tag `prototype/scout`.

## Shape: Henry Ford (as the piece suggests)

It is a port of `scout/` (`routePlan`, `jobs`, `scoutWork` in `scout/src/rules.js`) onto
machinery that exists (`land::path`, the camp's `Unit`/`Actor`, forage and line in `units.rs`,
the selection panel and feed). Its rules are settled below; what is left is a few starting
numbers (reveal and build labour, the scout's size) checked by tests, not by feel. The one
measurement that could have changed the plan, whether the pre-game still settles with real
trail costs, was run at spec-review (`sweep.md`): it does. It splits on three seams:

| Step | Part | What | Model |
|---|---|---|---|
| build-1 | ford | The map's hidden costs: trails bring a tile to its `trail_cost`, mountains with a trail are passable, one trail per tile however made, decay for all, the civ's revealed set; tests; the pre-game still settles | haiku medium |
| build-2 | ford | The scout and the search: `UnitKind::Scout`, its turn (walk, reveal, build, forage), known/possible routes, outward order, actions, events, tests, CLI orders and cross-build | sonnet medium (off the ladder) |
| build-3 | ford | The page (panel buttons, both routes, revealed tiles, trails, feed), `check.mjs`, `play-scout.mjs`, docs, archiving `scout/` | haiku medium |
| review | | Against the design; plays a scout's route through the page only | sonnet high (coin) |

## Settled

1. **Trail cost.** `step_cost` on a tile with a trail is its `trail_cost` when it has one
   (`min` with the bare cost), else the bare cost: flat land gains nothing. `TRAIL.cost` goes.
   River crossing keeps its rule (a trail bridges it). Marsh has no hidden cost, so a trail on
   marsh no longer cuts its 3; it still bridges a river there. Measured (`sweep.md`): the
   pre-game still settles on all sizes and the retro maps found 21 of 21 civs, so there is no
   fallback cap for worn trails.
2. **Mountains.** A mountain is entered only with a trail on it, at its `trail_cost`
   (`MAP.mountain_cost`). A trail can be built only where `span <= CIV.bridge_span` (new, 1):
   every civ has the shortest span. `passable(w, i)` becomes "bare terrain passable, or a
   trail"; every caller of `passable`/`step_cost`/`cost_map`/`path` then sees built mountain
   passes. Clans' festival routes use `passable` alone, so they cross passes at once. Units
   don't: `units::walkable` is `land && passable`, and mountain `land` is false, so it becomes
   `passable && (land || trail)`: the camp's moves and line and the scout's walking cross a
   built pass. A camp's `MoveCamp` target stays on land. Clans still never *wear* a trail onto
   a mountain (they can't reach it bare), and a clan never holds a mountain tile in its
   territory (`actors::edge`, `clans.rs` and `pregame.rs` keep `land && passable`).
3. **One trail per tile.** `Tile.trail: bool` stays the only trail. Worn (`land::hold` as now)
   or built (by a scout), it is the same trail. Any trail unwalked `TRAIL.fade` turns in a row
   wears away, built ones included, as now. A tile the scout steps into counts as walked that
   turn (the `units` step runs after `harvest` clears `walked` and before `hold`); the tile it
   stands still on doesn't, so an idle scout wears no trail. Build progress is
   `Tile.trail_work: f64` (`serde(default)`), reset when finished; partial work does not decay
   (kept simple; it is small). **Finishing a trail sets `rest_run` to 0**: an unwalked tile's
   `rest_run` is in the hundreds, and `hold` would wear a fresh trail away the turn it is
   built.
4. **Knowledge.** `Civ.revealed: BTreeSet<usize>` (`serde(default)`). Only trail costs are
   hidden; terrain, food and trails themselves are seen by all. Alliances don't exist, so it is
   the civ's alone. A tile with a trail counts as known at its trail cost for everyone.
5. **The scout** is a `Unit` with `kind: Scout` on the same `Actor`: `people` (`SCOUT.people`,
   start 2), `home` the tile it stands on, `store` its pack (`SCOUT.pack` turns of food per
   person), `routes` its line to the capital. Founding makes one per civ at the capital, its
   people taken as the camp's are (one from each of the biggest civ clans), its pack from the
   store after the camp's (the store starts at 20 and the camp takes 15, so the scout's pack
   starts short; it forages). It eats, spoils, goes hungry and starves as the camp does.
   **It feeds first**: before its search, it covers this turn's eating that its pack doesn't,
   by foraging its tile when it has a route (as the camp's auto-forage; on rival land it
   forages nothing, no pillage), else, or for what foraging didn't cover, on its line. It
   never refills beyond that, so it seldom draws on its line. Without a route it doesn't
   forage at all: an idle scout at the capital would strip the capital's clan's home every
   `UNIT.fallow` turns. It never blocks anyone. No raising or disbanding actions in this
   piece: `Disband`, `MoveCamp` and the switches keep meaning the camp, so `units::camp` finds
   the civ's `Camp`, and `disband` and the starving path work on a unit id, not "the civ's
   unit". If the scout starves to nothing, it is gone; the civ gets no new one (noted as an
   idea).
6. **The order.** `Action::SetRoute { from: usize, to: usize }` for the led civ's scout
   (`from != to`, both on the map, neither water: land or mountain), and
   `Action::ClearRoute`. A new `SetRoute` replaces the old route. The scout's
   `route: Option<(usize, usize)>` (`serde(default)`). Validated with reasons, logged,
   replayed. The AI gives its scout no orders: AI civs' scouts sit at the capital, fed from
   the store at no line cost.
7. **Pricing.** For civ `c` and tile `i`, `known(i)` and `possible(i)`: with a trail, its step
   cost; revealed, `min(bare, trail_cost)` if a trail could go there (`trail_cost` set, mountain
   `span <= bridge_span`), else bare (a mountain that can't be bridged: impassable); unrevealed
   forest/hills: known = bare, possible = the lowest `MAP.*_cost` value; unrevealed mountain:
   known impassable, possible `MAP.mountain_cost` if any `MAP.mountain_span` value is `<=
   bridge_span`. River crossing added as `step_cost` does (a trail would remove it: possible
   leaves it out on any trailable tile, known keeps it). `known` and `possible` are Dijkstra
   paths from `from` to `to` with these costs (a generic `land::path_by(w, from, to, cost_fn)`).
   A route's cost shown to the player is the path's summed step cost.
8. **The search** (each turn, `units` step, after the camp, on the scout's labour
   `people × UNIT.labour`), looping while labour remains, as `scoutWork`:
   - possible has no path: event **no route can exist**, route cleared.
   - candidates: tiles on the possible path, not revealed, whose known and possible cost
     differ. If known is cheaper than or equal to possible, there are none.
   - **Outward along a range.** Recomputing the possible path alone doesn't work outward: the
     path crosses a range at one mountain, and when it fails the cheapest detour flips from
     one side of it to the other (k, k+1, k−1, k+2 …), so the scout walks back and forth
     across a widening stretch, which is `scout/`'s fault. So when a candidate is an
     unrevealed mountain, the mountain candidates are widened to its **range**: the
     unrevealed mountains joined to it through mountain tiles, whose cheapest possible route
     from `from` to `to` through them is cheaper than known. Order: tier (mountains first),
     then path cost from where the scout stands (nearest first), then saving (biggest
     first), then index. Recomputed after every reveal, it sweeps along the range from where
     it stands, one way and then the other, and tries the far end only when the near ones
     fail.
   - It walks to beside the first candidate it can reach (a move costs its step cost, as one
     person's pace, like the camp), then reveals it: forest or hills `SCOUT.reveal` labour, a
     mountain all the turn's search labour (it must start the turn beside it, fed, with none
     spent; else it waits beside it to next turn).
   - No candidates: it builds trails along the known path, the tile nearest the scout first,
     walking beside it, putting labour into `trail_work` until `TRAIL.work[terrain]` (forest <
     hills < mountain), only where a trail would cut the cost. When none is left: event
     **route found** with its cost, the route cleared.
   - **No reveal it can reach**: it builds the first trail it can reach on the possible path
     (revealed, trailable, cutting the cost), as `scout/`'s `jobs` does. This is how it gets
     into a sealed valley or past a range: the pass must be built before the tiles beyond
     can be reached and revealed.
   - Nothing it can reach (no walkable path to beside any reveal or trail): it waits; event
     "scout blocked" once.
9. **Events**: route found (cost), no route can exist, trail built (tile), pass found (a
   revealed mountain that can be bridged), scout hungry/starved as the camp's. Route found
   also once when known first equals possible (before building), so the player hears the
   cost; "trail built" once per tile.
10. **The view** (`web`): per selected scout, its route, known path and cost, possible path
    and cost, revealed tiles of the led civ, tiles with trail work. Every tile's `trail` in
    the layout/view as now.
11. **The page**: selecting a scout shows "start here" and "route to here" in the selection
    panel: "start here" arms a click to pick `from`, "route to here" a click for `to`, then
    sends `SetRoute` (with no start picked, `from` is the scout's tile); and "clear
    route". The scout and the camp start on the capital together; a click there cycles them
    as it does now. A right-click with the scout selected sends nothing (`MoveCamp` is the
    camp's). The map draws revealed tiles (a mark), known route
    solid, possible route dashed, with their costs in the panel, and trails (as now). The feed
    carries the events. Debug views stay; play needs none.
12. **CLI and cross-build**: `lead_orders` and `cross-build.mjs` add a `SetRoute` from the
    capital to a tile across rough country (chosen by a fixed rule build-2 writes), so digests
    compare the search natively and in wasm. `stats::digest` includes trail and revealed
    counts.
13. **Archiving**: `scout/` becomes a one-line `README.md` pointing at tag
    `prototype/scout` (it exists), and `scout` leaves the CI `js` matrix, as #29 did for
    `supply-lines/`.

## Unsettled, and who holds it

- **Starting numbers** (build-2): `SCOUT.people`, `SCOUT.pack`, `SCOUT.reveal`,
  `TRAIL.work`. Start from `scout/`'s (`revealLabor 1`, work forest 2, hills 3, mountain 6)
  scaled to `UNIT.labour`. Proof: a test where a scout settles and trails a 6-tile forest/hills
  route within 20 turns, and a mountain pass within 40, without starving, with every trail it
  built still standing when route found comes (a trail left unwalked `TRAIL.fade` = 30 turns
  wears away, so a slow build can lose its first trails).

## Decisions the piece left open

- Worn trails and built trails both bring a tile to its hidden cost (no special cap); a trail
  on marsh only bridges its river. Measured at spec-review (`sweep.md`).
- `CIV.bridge_span = 1` stands in for the technology.
- The scout is made at founding from the capital; no raise or disband for scouts here.
- AI civs don't scout.
- Partial trail work never decays.
- Outward order: impassable tier widened to the candidate mountain's range, then nearest to
  the scout, then saving. Design says "work outward from where it stands"; recomputing alone
  zig-zags.
- When no reveal can be reached, the scout builds the trail it can reach on the possible path
  (as `scout/`), so passes get built before the land beyond is revealed.
- The scout feeds before it searches, forages only while on a route, and takes its pack from
  what the camp leaves in the store.
- Units (camp and scout) walk over built mountain passes; a camp can't be sent onto one.
- Finishing a trail resets its `rest_run`; a tile the scout steps into counts as walked.
- A revealing mountain takes a whole turn begun beside it.
- Shown route cost is the summed step cost, not `walk_cost`'s yearly figure.
