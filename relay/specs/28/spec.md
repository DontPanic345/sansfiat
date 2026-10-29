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
numbers (reveal and build labour, the scout's size) checked by tests, not by feel. It splits on
three seams:

| Step | Part | What | Model |
|---|---|---|---|
| build-1 | ford | The map's hidden costs: trails bring a tile to its `trail_cost`, mountains with a trail are passable, one trail per tile however made, decay for all, the civ's revealed set; tests; the pre-game still settles | haiku medium |
| build-2 | ford | The scout and the search: `UnitKind::Scout`, its turn (walk, reveal, build, forage), known/possible routes, outward order, actions, events, tests, CLI orders and cross-build | sonnet medium (off the ladder) |
| build-3 | ford | The page (panel buttons, both routes, revealed tiles, trails, feed), `check.mjs`, `play-scout.mjs`, docs, archiving `scout/` | haiku medium |
| review | | Against the design; plays a scout's route through the page only | sonnet high (coin) |

## Settled

1. **Trail cost.** `step_cost` on a tile with a trail is its `trail_cost` when it has one
   (`min` with the bare cost), else the bare cost: flat land gains nothing. `TRAIL.cost` goes.
   River crossing keeps its rule (a trail bridges it).
2. **Mountains.** A mountain is entered only with a trail on it, at its `trail_cost`
   (`MAP.mountain_cost`). A trail can be built only where `span <= CIV.bridge_span` (new, 1):
   every civ has the shortest span. `passable(w, i)` becomes "bare terrain passable, or a
   trail"; every caller of `passable`/`step_cost`/`cost_map`/`path` then sees built mountain
   passes, clans and camp alike. Clans still never *wear* a trail onto a mountain (they can't
   reach it bare), and a clan never holds a mountain tile in its territory (its food is 0
   anyway; `land` stays false).
3. **One trail per tile.** `Tile.trail: bool` stays the only trail. Worn (`land::hold` as now)
   or built (by a scout), it is the same trail. Any trail unwalked `TRAIL.fade` turns in a row
   wears away, built ones included, as now. A scout walking a tile counts as walking it.
   Build progress is `Tile.trail_work: f64` (`serde(default)`), reset when finished; partial
   work does not decay (kept simple; it is small).
4. **Knowledge.** `Civ.revealed: BTreeSet<usize>` (`serde(default)`). Only trail costs are
   hidden; terrain, food and trails themselves are seen by all. Alliances don't exist, so it is
   the civ's alone. A tile with a trail counts as known at its trail cost for everyone.
5. **The scout** is a `Unit` with `kind: Scout` on the same `Actor`: `people` (`SCOUT.people`,
   start 2), `home` the tile it stands on, `store` its pack (`SCOUT.pack` turns of food per
   person), `routes` its line to the capital. Founding makes one per civ at the capital, its
   people taken as the camp's are (one from each of the biggest civ clans), its pack from the
   store. It eats, spoils, goes hungry and starves as the camp does. At the end of each turn,
   on labour left, it forages its tile (as the camp's auto-forage; rival land: no pillage, it
   forages nothing there), and resupplies on its line only for what eating needs and foraging
   didn't cover (no refilling beyond that). So it seldom draws on its line. It never blocks
   anyone. No raising or disbanding actions in this piece: `Disband` keeps meaning the camp.
   If it starves to nothing, it is gone; the civ gets no new one (noted as an idea).
6. **The order.** `Action::SetRoute { from: usize, to: usize }` for the led civ's scout
   (`from != to`, both on the map; mountains allowed as targets only if passable to it later —
   simply allowed), and `Action::ClearRoute`. The scout's `route: Option<(usize, usize)>`
   (`serde(default)`). Validated with reasons, logged, replayed. The AI gives its scout no
   orders: AI civs' scouts sit at the capital foraging.
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
   - **Outward:** order candidates by tier (would-be-impassable first, then others), then by
     path cost from where the scout stands (nearest first), then by saving (biggest first),
     then index. Recomputing the possible route after every reveal is what keeps it working
     outward along a range: the nearest mountain is checked, the route shifts to the next
     nearest, and the far end is tried only when nearby ones fail. (Design says "rather than
     back and forth between the ends as `scout/` does"; this replaces `scout/`'s
     saving-then-distance order.)
   - It walks to beside the first candidate (a move costs its step cost, as one person's pace,
     like the camp), then reveals it: forest or hills `SCOUT.reveal` labour, a mountain the
     whole turn's labour (it must start the turn beside it with all its labour; else it waits
     beside it to next turn).
   - No candidates: it builds trails along the known path, the tile nearest the scout first,
     walking beside it, putting labour into `trail_work` until `TRAIL.work[terrain]` (forest <
     hills < mountain), only where a trail would cut the cost. When none is left: event
     **route found** with its cost, the route cleared. Trails are built on the revealed
     route only after the route is settled (as `scout/`).
   - Unreachable next job (no walkable path to beside it): it waits; event "scout blocked"
     once.
9. **Events**: route found (cost), no route can exist, trail built (tile), pass found (a
   revealed mountain that can be bridged), scout hungry/starved as the camp's. Route found
   also once when known first equals possible (before building), so the player hears the
   cost; "trail built" once per tile.
10. **The view** (`web`): per selected scout, its route, known path and cost, possible path
    and cost, revealed tiles of the led civ, tiles with trail work. Every tile's `trail` in
    the layout/view as now.
11. **The page**: selecting a scout shows "start here" and "route to here" in the selection
    panel: "start here" arms a click to pick `from`, "route to here" a click for `to`, then
    sends `SetRoute`; and "clear route". The map draws revealed tiles (a mark), known route
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

- **The pre-game with real trail costs** (build-1): worn trails used to cap forest and hills at
  1.0; now a forest rolled at 2 gains nothing. Proof: `sansfiat sweep --size all` still levels
  off and the retro maps still found three civs. If they don't, build-1 may keep the old cap
  for worn trails only and record it (Design says one trail brings the tile to its hidden cost;
  that is the fallback, not the plan).
- **Starting numbers** (build-2): `SCOUT.people`, `SCOUT.pack`, `SCOUT.reveal`,
  `TRAIL.work`. Start from `scout/`'s (`revealLabor 1`, work forest 2, hills 3, mountain 6)
  scaled to `UNIT.labour`. Proof: a test where a scout settles and trails a 6-tile forest/hills
  route within 20 turns, and a mountain pass within 40, without starving.

## Decisions the piece left open

- Worn trails and built trails both bring a tile to its hidden cost (no special cap).
- `CIV.bridge_span = 1` stands in for the technology.
- The scout is made at founding from the capital; no raise or disband for scouts here.
- AI civs don't scout.
- Partial trail work never decays.
- Outward order: impassable tier, then nearest to the scout, then saving.
- A revealing mountain takes a whole turn begun beside it.
- Shown route cost is the summed step cost, not `walk_cost`'s yearly figure.
