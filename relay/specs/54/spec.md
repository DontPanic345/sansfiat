# Spec: #54, Actors, and clans that live off their land

## What the piece delivers

The actor model (`docs/design/actors.md`) as engine code every later M2 piece builds on, and
the pre-game's clans (`docs/design/clans.md`) moved onto it:

1. **Actors.** One `Actor` (people, home, store, territory, routes, market) and the functions
   every actor uses: the cost of a walk, dividing labour over a territory, and the rule for
   adding and dropping a tile. A clan is the first actor; the camp (B), the scout (C) and later
   the farmer and steward are new actors on it.
2. **The round goes.** A clan holds a connected patch of tiles around home and harvests only
   that. It divides its gathering evenly over it; each tile's share pays the walk there and
   back first. It adds a tile only if its harvest rises, and drops the one it misses least.
3. **Growth on surplus.** A clan grows on what it harvests beyond its need; one that just
   meets it holds.
4. **Depletion.** A food harvested past what it regrows wears its tile's cap down; it
   recovers once it rests.
5. **Trails** wear where routes are walked, not where land is held.
6. **Taming** keeps rising while grain is held, through the main game.
7. **Festivals.** Each clan walks one route, to one festival, and changes when another is
   worth more. A festival forms where a clan has none worth going to, and a clan can hold one
   alone. Food is too heavy to carry far, so little is traded but people: starving clans shed
   people to growing ones, and a clan too small to stand alone merges into another there.
8. **The pre-game still ends in three centres on its own**, and people level off.
9. **`sansfiat sweep`** reports per map when the pre-game ends, and the clans, people,
   depleted tiles and people moved at festivals over time.
10. **The page** draws territories and festival routes instead of rounds.
11. **`hex-economy/` and `hex4x/` are archived** (tagged `prototype/hex-economy` and
    `prototype/hex4x` already).

Out of scope: work besides harvesting (traders, crafters, farmers), more than one festival
per clan, anything civs do (founding, the levy and units run as they do, on territories
instead of rounds), good and bad years, hidden trail costs.

The piece's question is the design's: does a sparse world of clans, paying real labour to
reach their food, spread, level off, and form centres around fertile land on its own? The
canvas answers it with the sweep in `relay/specs/54/sweep.md`; the review writes the answer
and the evidence into the PR description.

## The shape: Clay and Kiln, with a canvas on top

The piece's suggestion, Clay and Kiln, holds for the code: the actor model is the shape at the
centre, and once clans live on it, depletion, trails, festivals and the page are settled work
on top. But one important thing can't be settled until every rule exists: the numbers that
make the whole pre-game level off and still end in three centres on its own. Every kiln moves
them. So the numbers go to one `canvas` step at the end, judged by the sweep, and every
earlier step starts from numbers chosen to run, not to be right.

| Step | Part | What | Model |
|---|---|---|---|
| build-1 | clay | The actor model; clans on it: territory, divided labour, walks, holding land, growth on surplus, the festival route; the round removed; the sweep's series | opus medium |
| build-2 | kiln | The land: depletion, trails from walked routes, open-ended taming; depleted tiles in the sweep | haiku high |
| build-3 | kiln | Festivals: forming alone, fading by attendance, carrying little, people moving, small clans merging; people moved in the sweep | sonnet medium |
| build-4 | kiln | The page's territories and routes, the tooltip and panel, `check.mjs`; archiving the prototypes | haiku high |
| build-5 | canvas | Tune every number so people level off and three centres form on their own; `sweep.md` | opus low |
| review | | The piece against the design, the sweep's answer, playing the page | sonnet high |

Each step leaves `engine/check.sh` green (fmt, clippy, test, `build-web.sh`, `cross-build.sh`,
`node www/check.mjs`).

## What the clay leaves the kiln (the invariants)

The clay's exact Rust is its own; the kiln and later pieces rely on this:

- **`Actor`** (`core/src/actors.rs`) holds `people`, `home`, `store`, `territory` (tile
  indices, home among them, connected over passable land), `routes` (each with its
  destination tile, the path's tiles and its yearly cost) and `market: Option<u32>`. `Clan`
  carries one; nothing about the actor names clans.
- **`actors::walk_cost(path_cost, weight) = 2 × path_cost × weight`**: there and back, once a
  year (a turn). Every walk in the engine, in a territory or on a route, is priced by it. A
  festival route has weight 1; a clan's walk to a tile of its own territory has weight
  `CLAN.inside` (< 1, "costs little").
- **Walks inside a territory** go by the cheapest path from home over the territory's own
  tiles (`land::step_cost`, so trails and rivers count). There is one way to get, for an actor,
  each territory tile's walk cost and path.
- **Dividing labour.** `actors::divide` takes an actor's labour less its routes' cost, splits
  it evenly over its territory, takes each tile's walk from its share, and returns each tile's
  labour left for the work (0 if the walk eats the share). It knows nothing of food: a clan
  turns it into harvest, a camp later into guarding.
- **Holding land.** One add/drop rule, generic over a value function: candidates are the
  territory's edge (passable land next to it, held by nobody, within `CLAN.territory_radius`
  hex steps of home); a tile is added only if the value rises by `CLAN.land_margin`; a drop
  never disconnects the territory or drops home. The rule is a function an actor's chooser
  calls, so an actor whose territory is fixed by an order (later) never calls it.
- **Tile walked this turn.** After the harvest, the tiles every walk crossed this turn (the
  paths inside territories and every route's path, not counting home) can be read per tile,
  for trails.
- `Tile.claim` is the clan whose territory holds the tile. Every reader of `round`
  (`civs::found`, `units`, `pregame::land_cover`, `stats`, the CLI, `web`) reads the
  territory instead.
- Per-clan work is bounded by `CLAN.territory_radius` and the reach of a festival route, never
  by the number of clans in the world.

Each kiln brief says: keep the clay's shape; where it doesn't hold, make the smallest fix that
works and record it as a decision.

## What's settled

### The actor (build-1)

- An actor's labour is `people × CLAN.work`. Its routes are paid first, each at its
  `walk_cost`. What's left is divided evenly over the territory: each tile's share is
  `labour / n`, its walk is paid from the share, and the rest is the tile's work.
- The festival route is the actor's only route for a clan (`routes` holds 0 or 1). It goes by
  the cheapest path from home to the festival's tile over passable land, weight 1.

### Clans (build-1)

- **Harvest.** On each tile, the work harvests every food there in proportion to its stock,
  each at `CLAN.harvest_labour` a unit, up to `CLAN.take_rate` of the stock. A clan works only
  as hard as it needs to reach `CLAN.surplus_target × need − store` (as now): every tile's work
  is scaled down by one factor, the walks still paid in full. There is no fetching from tiles
  off the territory (`fetch_range` goes) and no plan by diet (`plan_harvest` goes). Clans
  harvest in a shuffled order, first come first served, as now.
- **Value** of a territory to a clan: the nutrition per person of its store plus what dividing
  its labour over that territory would harvest this turn from the current stock (the harvest
  above, with the target). This is `clans::value` on the new model.
- **Holding land** (the `move` step): short of its harvest target (`clans::at_target`: its
  territory can't reap `surplus_target × need − store` this turn), a clan adds the edge tile
  that raises its value most if it rises by `land_margin`, else swaps its worst tile for it if
  that does; at its target, it drops the tile whose loss leaves the most value, if the value
  doesn't fall. (Build-1: read as decision 2 says, "a fed clan's harvest is at its target".
  Reading "fed" as `fed_ratio ≥ 1` locked clans that just meet their need: they never added
  land, so never had a surplus to grow on.) At most
  one change a turn. Home then steps towards the territory's middle (`round::middle` as now),
  and tiles beyond `territory_radius` of home, or cut off from home, drop.
- **Growth on surplus.** In `eat`, a fed clan grows by `growth(since) × min(1, surplus /
  (CLAN.grow_full × need))`, where `surplus = max(0, nutrition(harvest, pop) − pop)`: this
  turn's harvest, not its store, so a store doesn't feed growth twice. One that just meets its
  need holds; a starving one shrinks as now.
- **Dividing** halves the territory as `halve` does; each half keeps only the tiles connected
  to its home, the rest freed. **Splitting**, **moving on** and the store are kept.
- **The festival route** (the `go` step): a clan scores each festival whose route cost is
  under the most any festival could be worth to it: `worth = gain × harvest_labour + desire −
  route cost`, where `gain` is as `markets::gain` now, and `desire = MARKET.desire +
  MARKET.company × min(traders last turn, MARKET.company_cap)`, plus `MARKET.gather_bonus` if
  it holds a surplus (store ≥ `surplus_at × pop`, as now). It goes to the best with worth > 0;
  it keeps its current festival unless another is worth more by `MARKET.switch_margin`. Its
  route and `trip` follow. Forming and fading are as now until build-3 (forming reads "in
  reach" as a route cost from home within `MARKET.reach_walk`).

### The land (build-2)

- **Depletion.** Each food on each tile has a wear in `0..=DEPLETION.max`; the tile's cap is
  its unworn cap × (1 − wear). A turn whose take of a food exceeds that food's sustainable
  yield (`regen × cap` for plants; game's most, `(regen / 4 + restock) × cap`) raises its wear
  by `DEPLETION.rate × excess / cap`; a turn at or under it (or untouched) lowers it by
  `DEPLETION.recover`. The unworn cap is the generated cap, and for grain the tamed cap. A
  tile is **depleted** when some food's wear is at least `DEPLETION.depleted_at`.
- **Trails** come from walking: a tile walked `TRAIL.after` turns in a row gets a trail; one
  unwalked `TRAIL.fade` turns loses it. Holding a tile no longer makes a trail.
- **Taming** counts held turns with no ceiling. A hearth is still `tame ≥ tame_turns`; the
  grain cap rises to `TENDING.domestic` × wild at `tame_turns`, then on by `TENDING.beyond` ×
  wild each further `tame_turns`. Unheld, it goes wild a step a turn, as now.

### Festivals (build-3)

- **Forming.** In the `markets` step, each clan with no festival this turn and settled
  `MARKET.form_after` turns (`since`) raises one on its home if no festival is within
  `MARKET.spacing` hex steps; biggest clans first, then lower id, each checked against those
  raised before it. `min_clans` goes: a clan can hold one alone.
- **Fading** by attendance: `activity` smooths the clans that came, and a festival fades under
  `MARKET.min_attendance`. A festival one clan keeps coming to stays.
- **Carrying little.** A clan brings at most `MARKET.carry × pop` food to clear, its basket
  scaled down evenly; what it didn't bring stays in its basket.
- **People** (a `people` step after `eat`): at each festival, of the clans that came this
  turn, each starving clan offers `MARKET.shed × pop × (1 − fed_ratio)` people; each growing
  clan (this turn's surplus > 0) has room for its surplus in people (one food a person). The
  offer goes to the growing clans in proportion to their room, all of it scaled down if it's
  more than the room. People carry no food. The festival records the people moved.
- **Small clans.** After the people move, a clan under `CLAN.min_pop` at a festival with
  another clan merges into the clan there whose home is fewest hex steps from its own (ties to
  the lower id): people and store join it, its tiles are freed. `ClanMerged` event. A small
  clan alone keeps on, and dies under one person as now.

### The page (build-4)

- Territories filled from `claims` as now; rounds no longer drawn; each clan's festival route
  drawn home to festival along its path; trails as now. The tooltip names the clan's
  territory and the tile's wear; the selection panel shows a clan's territory size, festival
  and route cost. The legend's "round" becomes "festival route".

## What's left unsettled, and where

| Unsettled | Proof | Step |
|---|---|---|
| The actor's Rust: how `Clan` embeds `Actor`, the route type, how walks are computed and cached | the invariants above, its tests, the kiln builds on it without new machinery | build-1 |
| Every number: `CLAN.inside`, `grow_full`, `territory_radius`, `land_margin`, `min_pop`; `DEPLETION.*`; `TRAIL.*`, `TENDING.beyond`; `MARKET.desire`, `company`, `company_cap`, `switch_margin`, `reach_walk`, `spacing`, `form_after`, `min_attendance`, `carry`, `shed`; `PREGAME.*` if needed | the sweep (`relay/specs/54/sweep.md`): on small, medium and large, seeds 1–10, most end by centres on their own (as many as now: 8 of 10 small); people level off (change over the last 100 turns before the end under 10%); some tiles depleted, some people moved; ms a turn on each size within 2× today's (small 3–5, medium 5–9, large 8–14) | build-5 |
| Whether a rule's form (not just its number) must change for the pre-game to work | the same sweep; each change recorded as a decision | build-5 |

Builds 1–4 pick starting numbers that run (the pre-game ends, by the backstop if need be) and
say so; they don't tune.

Build-5 tuned them (`relay/specs/54/sweep.md`) and changed one rule's form: the pre-game ends by
centres only once people have also levelled off, changing by under `PREGAME.settled` (5%) over
the last `PREGAME.settle_turns` (100) turns (`pregame::settled`, counted each turn in
`World.people_seen`). The design's pre-game "settles on a light population the land carries";
before this, three centres and the cover came while people were still spreading.

Today's sweep (main, small, seeds 1–10): 8 of 10 end by centres on their own, 2 by the limit
and top-up, median end turn 382, 87–149 clans, 440–749 people, 3–5 ms a turn. Medium: 9 of 10, median
257, 5–9 ms. Large: 10 of 10, median 305, 8–14 ms.

## Decisions the piece left open

1. **Design says every walk is costed the same way, and clans.md says walking inside a
   territory costs little.** Both hold: one `walk_cost`, with a weight per kind of walk. A
   festival route (and later a supply line) has weight 1; a clan's walk to its own tiles has
   `CLAN.inside`, well under 1, but not 0, so the path's tiles are walked and trails form.
2. **"A clan adds a tile only if its total harvest rises"** is read as its value: the
   nutrition of what it would harvest, up to its target, per person. Diet is kept from merge4,
   so harvest is weighed as food the clan can use. A fed clan's harvest is at its target, so
   it adds nothing and drops what it doesn't need; a hungry one adds.
3. **Within a tile, work is spread over the foods in proportion to their stock.** The design
   divides labour evenly over tiles and says nothing within one; diet shapes which tiles a
   clan holds, not what it picks on each.
4. **Growth reads this turn's harvest**, not the store, so a store doesn't feed growth turn
   after turn. The settling ramp (`growth(since)`) is kept as the most a clan grows.
5. **The territory keeps a hard bound**, `CLAN.territory_radius` hex steps from home: walks
   bound it in practice, but the bound keeps per-clan work fixed (clans.md's Scale).
6. **Design says "reach"**: a festival is in a clan's reach when its route cost from home is
   under `MARKET.reach_walk`. A centre's "tamed grain in its reach" stays `markets::reach`
   around the festival's tile, as now.
7. **Desire to gather** is a base, plus company (more for each clan that came last turn, up
   to a cap: "to see others"), plus the surplus bonus. Company is what draws clans together,
   so festivals grow into centres.
8. **Changing festival** needs another worth more by a margin, so clans don't flicker between
   two equal ones.
9. **Design says food is too heavy to carry far**: a clan brings at most `MARKET.carry` food a
   person to a festival, so little is traded but people.
10. **Design says a festival nobody comes to fades**: fading reads attendance, not goods
    volume; with little food carried, volume would fade every festival.
11. **Design says a small clan "merges into another at its festival, or its people spread
    across the clans there"**: it merges into the nearest. One rule, and it keeps the merged
    clan's people together.
12. **People move between any clans at a festival**, whatever their civ: civs are out of
    scope, and D decides what joining a civ means.
13. **Depletion is per food**: a hunted-out tile's game wears without its fruit.
14. **Taming has no ceiling but slows**: a hearth is reached as now, then the grain keeps
    rising more slowly, so the main game's grain keeps growing without running away.
15. **Archiving under CI.** The runner can't push workflow files, and `ci.yml`'s `js` job
    runs `npm test` in `hex-economy` and `hex4x`. Build-4 deletes both folders but leaves each
    a `package.json` whose test passes and a `README.md` naming its tag, so CI stays green;
    the owner drops the two matrix entries and the stubs (an idea for the owner).
16. **Event kinds and wording stay** (`MarketFormed`, "A market forms"): the engine still
    calls a festival a market, as founding's civs do. Only `ClanMerged` is new.
17. **Tests on real maps assert what holds, not end turns or counts**, since build-5 retunes
    every number. Tests needing a real founding may set `w.end` by hand and call
    `civs::found`, or run to the end, whatever turn it is.

## Tests the piece must have

- Actors: `walk_cost`; divide over a hand-built territory (equal shares, walks paid first, a
  far tile's work at 0); the add/drop rule never disconnects, never drops home, adds only on a
  rise; routes paid before division.
- Clans: a poor far tile lowers the value, so it isn't added; a hungry clan adds, a fed one
  drops; growth on surplus (holds at need, grows on harvest beyond it, the store doesn't feed
  growth); dividing gives two connected territories; no harvest from off the territory.
- Festival route: the best by worth, kept within the margin, none when nothing is worth the
  walk; trip cost is the route's.
- Land: wear rises past the yield, recovers resting, never past `max`; trails from walking
  only; taming past `tame_turns` keeps raising grain.
- Festivals: one clan forms its own; spacing; fading by attendance; carry cap conserves goods;
  people move from starving to growing at one festival, conserved, scaled to room; a small
  clan merges into the nearest; `ClanMerged`.
- The whole: a save replays to the same world; native and wasm digests agree; a small seed
  runs to its end with three centres; `check.mjs` covers territories and routes.

## What the later steps need to know

- `cargo test` runs at `opt-level = 3`. `core/tests/common/mod.rs` builds flat worlds
  (`flat()`, `big()`, `put`, `open`, `hearth`); it resets tile fields by name, so a new
  `Tile` field must be reset there too.
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs`; anything else
  through `libm`. No `HashMap`; deterministic orders and tie-breaks everywhere. All randomness
  through `rng::rand` on `World.rng`.
- `World.events` and `centres_seen` are `#[serde(skip)]`; a counter derived from turns may be
  too (a replay reruns the turns), but anything a rule reads next turn must be serialised.
- `./target/release/sansfiat sweep` (after `cargo build --release -p sansfiat`) runs ten small
  seeds in about 5 seconds today; `--size all` runs every size.
