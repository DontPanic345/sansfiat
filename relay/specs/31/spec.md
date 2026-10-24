# Spec: #31, The map and the page

## What the piece delivers

The pre-game runs on a real world instead of the placeholder Voronoi hex, and you can watch it
on a full-screen page. Designs: `docs/design/map.md` (the map, the page) and `clans.md` (what
the clans do on it). merge4's generator (`merge4/src/mapgen.js`) is ported to the engine and
changed where `map.md` says. **merge4 stays frozen**: read it, don't edit it.

1. **A generator** in `engine/core/src/map.rs`: coasts, inlets, inland seas and lakes, ridges
   with hilly flanks and passes, deserts in rain shadow, rivers that run to the sea, closed
   country (valleys behind a pass, sealed valleys, peninsulas and necks), floodplains, deltas,
   fertility in patches, and the hidden trail costs.
2. **Three map sizes**, picked by the player: small (today's radius 16), medium (23) and
   large (33), each about twice the area of the one before (817, 1657, 3367 tiles). Feature
   counts scale with area.
3. **Rivers cost movement to cross**, not to follow; a trail on a river tile bridges it.
4. **The pre-game always ends with three centres.** At the turn limit with fewer, it tops up
   with the best sites it can find.
5. **Events**, derived from the turn and not saved, printed by the CLI and shown on the page.
6. **A sweep** over seeds and sizes: how often the pre-game ends on its own, how often it
   needs the top-up.
7. **The page**: full screen, pan and zoom, new-map control (size and seed), top bar, event
   feed, tooltip; rivers, deserts, floodplains, deltas and closed country drawn so they read
   at a glance. Debug views stay. Plain JS, no framework, no build step beyond `./build-web.sh`.

Out of scope: founding, civs, units, scouting itself; a minimap, graphs, start screen, saving
(the page keeps its existing Save/Load buttons as they are); climate beyond deserts and rain
shadow; seasons; rivers changing course. The hidden trail costs are only rolled and stored:
nothing reads them until the scout.

**The question** this piece asks (and the review answers in the PR description): *does the
pre-game still end, with three centres, on every size and seed, and how often does it do it
on its own versus needing the top-up?* The sweep is the evidence.

## Steps

| Step | What | Model |
|---|---|---|
| build-1 | Terrain, tile fields, sizes, the base generator (noise, water, ridges, rain-shadow deserts, coast, fertility, hidden trail rolls), `map` ASCII command | sonnet high |
| build-2 | Rivers to the sea and to lakes, crossing cost and bridging, river fertility, floodplains, deltas | opus high |
| build-3 | Closed country: valleys behind a pass, sealed valleys, peninsulas and necks; open land always holds three sites | opus high |
| build-4 | The top-up, events, CLI output, the sweep, tuning, cross-build sizes | sonnet high |
| build-5 | The page | sonnet high |
| review | The whole piece; watches the pre-game through the page only; writes the answer | sonnet high |

Each step leaves `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
and `./build-web.sh` green (build-4 and build-5 also run `./cross-build.sh`).

## Decisions the piece left open

1. **Map shape stays the engine's hex of radius R.** Everything (`map::index`, `Config`,
   the page) is built on it; merge4's offset rectangle isn't ported. "Edge" is `R − distance
   from origin`. The rim (`edge == 0`) is always sea.
2. **A size is a radius**: `data::SIZES = [("small",16),("medium",23),("large",33)]`.
   `Config { seed, radius }` and the save format are unchanged; the page and the sweep map a
   size name to a radius. `--radius N` still works on the CLI for any N. Feature counts are
   per-817-tiles values × `tiles / 817`, rounded, never below the floor each feature names.
3. **The generator doesn't reproduce merge4's maps** (different shape and rng use). It
   keeps merge4's rules and its recipe: random `h`, `m`, `y` per tile, 3 smoothing passes,
   normalise, thresholds `ocean < 0.35` (with the 0.25 penalty for `edge < 2`), `mountain >
   0.88`, `hills > 0.72`, `forest` where `m > 0.6`.
4. **New terrains**, appended so existing indices (`SEA 0, COAST 1, PLAINS 2, FOREST 3,
   HILLS 4`) don't move: `MOUNTAIN 5` (`land: false`, `move_cost: None`, no food), `DESERT 6`
   (land, cost 1, no food), `MARSH 7` (land, cost 3, wet foods). "Land" keeps its meaning in
   the code, "a tile clans can stand on": a mountain is not land, so seeding, territory,
   cover and landmasses don't need to special-case it. `map::landmasses` groups land tiles by
   adjacency as now, so a ridge splits landmasses. Coast stays a *terrain* (as in the
   placeholder; it carries the sea foods): a plains, forest or desert tile beside water
   becomes `COAST`; hills and mountains don't.
5. **Lakes are sea tiles**. `SEA` is used for the ocean, inland seas and lakes alike; a tile
   flag `lake: bool` (water not connected to the rim sea) lets the page colour them apart and
   lets rivers tell "the sea" from "a lake" for deltas.
6. **Tile fields added** (all serialised, so the digest covers them): `river: u32` (0 none;
   else the number of river tiles upstream including this one), `flow: Option<usize>`
   (the tile index downstream; for a mouth, the water tile it empties into), `lake: bool`,
   `flood: bool` (floodplain), `zone: Zone` (`Open | Valley | Sealed | Neck`, see build-3),
   `pass: bool`, `trail_cost: Option<f64>` and `span: Option<u8>` (hidden; see below).
7. **Rivers**: BFS distance from water over river-passable tiles (land, not mountain,
   desert allowed); a river steps to an unvisited neighbour whose distance is not greater,
   preferring lower, with a sidestep chance for wiggle. Distance strictly falls on every
   non-sidestep step, no tile is revisited, so it can't double back and always ends in water
   or in another river. Sources on high ground (hills, beside mountains, or noisy height
   `≥ MAP.upland` 0.6) at least `river_min` (6) steps from water, lowered to the map's deepest
   land less one where the land isn't that deep (build-2: small maps rarely have land more
   than 4 steps from water). Crossing rule: `step_cost(w, from, to)` takes the tile left as well as the one
   entered; entering a river tile costs `RIVER.cross` (starting value 2.0) extra unless
   `from.flow == Some(to)` or `to.flow == Some(from)` (along the course) or the tile has a
   trail. A leg's cost therefore differs by direction only when an endpoint is a river tile;
   accepted.
8. **Floodplain and delta**: a floodplain is a river tile with `river ≥ BIG_RIVER` (8) within
   `FLOOD_REACH` (6) path steps of its mouth, plus its non-mountain land neighbours there;
   fertility is `FLOODPLAIN_FERTILITY` (2.2), above the normal 2.0 cap, so it is the richest
   land on the map; the terrain becomes PLAINS if it was coast/desert. A delta is the mouth
   tile of a river with `river ≥ DELTA_RIVER` (10; 12 in the plan, lowered in build-2) that empties into an *ocean* (not a lake)
   and the mouth's land neighbours that touch the sea: terrain `MARSH`, fertility 2.2. Marsh is
   slow (cost 3).
9. **Rain shadow**: one wind direction per map (from the rng). A desert candidate's score is
   its moisture `m` minus a shadow term (ridge tiles upwind within 5 steps). Deserts are the
   lowest-scoring plains up to `desert_share` (0.2) of the land.
10. **Hidden trail costs** are rolled at the end from a sub-stream seeded `rng ^ 0x5bd1e995`
    so they don't shift the rest of the map: forest 1 or 2 (even); hills 1 (1 in 3) or 2;
    mountain cost 3 with span 1 (1 in 10), 2 (1 in 10) or none (8 in 10), as merge4's
    `TRAILS`. `trail_cost: None` and `span: None` elsewhere or when a mountain can't be crossed.
    Nothing reads them in this piece; a test holds `step_cost` to that.
11. **Closed country is built, not found**: after the base land and ridges, the generator
    raises rings of mountain (and, for passes, hills) around chosen interiors, or cuts water
    to leave a neck. Zones are tagged on the tiles. A sealed valley's wall always has at least
    one mountain with `span: Some(1)`. Clans are never seeded in a sealed zone and can't walk
    in, so it stays empty. (Build-3: features are placed in turns, sealed, neck, valley, from
    centres that fit; necks are radius 2 only and their cut may run into a lake or inland sea
    as long as the map keeps a lake; a feature that leaves the open land likely short of three
    sites, judged on the terrain before coast and rivers, is taken down again.)
12. **Open land guarantee**: after generation, `map::open_sites` finds three centre-sized sites
    at least `PREGAME.centre_spacing` apart in the open (zone `Open`, connected, passable) land;
    if not, the closed features are re-rolled (up to 8 attempts, then the closed features are
    dropped, then, if the base land alone can't hold three sites, the whole map is re-rolled from
    the continuing rng, up to 8 times; build-3 skips the layouts on base land that likely can't
    hold three sites even all open, and goes straight to the drop). The guarantee applies only at radius 16 and up: tiny
    hand-test radii (the tests' `flat()` is radius 6) get no closed country, no retry and no
    `open_sites` check, and must still generate without panicking. A test holds it over many
    seeds and all sizes.
13. **The top-up** (`pregame::check_end`, at `w.turn + 1 >= max_turns` with fewer than
    `players` centres): keep the centres already chosen, then add sites until there are three
    in this order of preference, each at least `centre_spacing` steps from every taken site and
    on passable land in an `Open`, `Valley` or `Neck` zone (never `Sealed`) and in the same passable region as the first centre: (a) existing markets, best first (traders, then older, then lower
    id), whatever their age or hearths; (b) the most tended land: tiles by `tame` (highest
    first), a market raised on it; (c) any fertile land: tiles with grain by fertility, a
    market raised on it. A raised market is an ordinary `Market` (founded this turn, no
    traders). `End` gains `topped_up: Vec<u32>` (the market ids added, a subset of `centres`).
    Markets in `End.centres` never fade after the end.
14. **Events** are `world::Event { turn, kind, tile, clan, market, other }` (`other` is the daughter on a division) pushed to
    `World.events` by the steps where things happen; `#[serde(skip)]` so they're not saved or
    in the digest but a replay rebuilds them identically (the replay test compares them).
    `Event::text()` is the one place a line is worded; the CLI and the page use it. Kinds:
    `ClanDivided` (clan, the daughter), `ClanStarving` (a clan starts starving), `ClanMoved`
    (moved on), `MarketFormed`, `MarketFaded`, `HearthTamed` (a tile reaches full tame),
    `CentreFormed` (a market becomes one of the chosen centres), `PregameEnded`.
15. **The wasm `Sim` gains** `events(from: usize) -> String` (JSON of events from that index;
    the page keeps a cursor), `sizes() -> String` and enough tile data in `state()` (it is the
    whole world JSON already). `Sim::new(seed, radius)` is unchanged.
16. **Page state lives in the URL for testing**: `?size=medium&seed=7&play=1&speed=fast`
    start a game without clicking, so the review can drive headless Chrome and take
    screenshots.
17. **More clan seeds on bigger maps only if the sweep says it's needed** (build-4): the
    default is `CLAN.seeds` unchanged. If medium/large seeds mostly fail to end on their own
    because the people can't spread in time, build-4 may scale the extra seeds with area
    (`seeds × tiles / 817`) and records it, with the sweep before and after. It changes no
    other rule.

## Tests the piece must have

- Map: every size and many seeds produce rims of sea, a connected open core, features present
  in the right numbers; rivers end in water or in another river, never revisit, fall in
  distance; river tiles are richer than neighbours; floodplains are the richest land; deltas
  are marsh at ocean mouths; deserts sit in shadow more than not; hidden costs rolled to the
  distribution and nothing reads them; sealed valleys have a bridgeable wall mountain and no
  pass; valleys have one or two passes; necks are 1–2 tiles wide.
- Movement: entering a river tile costs extra off-course, nothing along the course, nothing
  with a trail (worn or otherwise).
- Pre-game: ends with exactly 3 centres on every size for a set of seeds; the top-up fires
  in a constructed case, in each of its three tiers, and respects spacing; sealed valleys
  stay empty through a full run.
- Events: each kind fires in a constructed or real run; a replay of a save gives the same
  events as the original; the CLI prints them.
- Native and wasm digests agree on a medium map (cross-build).

## What the later steps need to know

- The pre-game's tuning was done on the placeholder map; expect existing end-to-end tests that
  name seeds to move. Re-pick seeds only for tests about a property ("some seed ends by
  centres"); never loosen what a test asserts to make it pass, and say in the result which
  tests changed and why.
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs` in the generator;
  anything transcendental goes through `libm` (clippy enforces it, #23).
- All randomness through `rng::rand` on the main stream except the hidden-cost sub-stream.
  Iteration orders must be deterministic (no `HashMap`; sort before using).
- A map generates in well under a second at radius 33; keep it so (bench).
- **Test speed.** `cargo test` builds the engine unoptimised: a 300-turn small run takes about
  26 s there (88 ms a turn; release is about 7 ms), and a large map has several times the clans.
  Build-1 sets `[profile.dev] opt-level = 3` in `engine/Cargo.toml` (debug assertions and
  overflow checks stay on) and records `cargo test` before and after. Tests that run whole
  pre-games on medium and large maps use few seeds (say 2 medium, 1 large) unless the measured
  time allows more; the whole `cargo test` should stay under about 10 minutes in CI.
- **Hand-built worlds.** `core/tests/common/mod.rs`'s `flat()` calls `World::new` at radius 6 and
  then overwrites terrain, caps and claims. Build-1 makes it also reset every new tile field
  (`river`, `flow`, `lake`, `flood`, `zone`, `pass`, `trail_cost`, `span`) and pin `w.rng` and
  `w.next_id` to the values the old generator left (print them before changing the generator),
  so tests on hand-built worlds don't move.
