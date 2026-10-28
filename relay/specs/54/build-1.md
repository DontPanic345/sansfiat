# Build 1 (clay): the actor model, and clans on it

Piece #54, "Actors, and clans that live off their land". You shape the core every later
piece of M2 stands on: the actor model, with clans as its first actors. Then the clans live
on it: the round goes, a clan holds a connected territory, divides its labour evenly over it,
pays for every walk, and grows only on surplus. Work in `engine/`; read `engine/CLAUDE.md`
first ("Adding a mechanic", "Invariants"). `relay/specs/54/spec.md` has the whole plan: its
"What the clay leaves the kiln" is your contract with the steps after you, and "What's
settled" has every rule (yours are marked build-1).

After you, build-2 adds depletion, trails from walking and open-ended taming; build-3 changes
festival forming and fading and adds people moving and small clans merging; build-4 redraws
the page; build-5 tunes every number. **Don't tune**: pick starting numbers that run (the
pre-game ends, by the turn limit and top-up if need be) and say in your summary how it runs.

## The design you're building

From `docs/design/actors.md`:

> Everything that acts in the world is an **actor**, and every actor works the same way. A
> clan, the civ, an army's camp and a scout differ only in what they spend their labour on
> and who chooses it.
>
> - **People.** An actor is people, and its people give it labour each turn.
> - **Home.** An actor has a home tile: a clan's home, the civ's capital, the camp's tile,
>   where a scout stands.
> - **Store.** An actor holds its goods in a store, or a pack if it moves. It eats from it,
>   and the store spoils.
> - **Territory.** An actor works a territory: the tiles it spreads its labour over.
> - **Routes.** An actor walks routes beyond its territory: to its market, along its supply
>   line, along a route it's scouting.
>
> An actor divides its labour evenly over its territory, and spends what its routes need.
> (A clan divides its gathering over its territory; an army divides its guarding over its
> zone; a steward, later, divides his taxing over the land he taxes.) Each tile's share pays
> the walk to it first, then the work. So a tile far from home, or a poor one, takes labour
> from better ones: holding more land isn't always better. An actor adds a tile only if it
> does better with it, and drops the one it misses least.
>
> Every walk is costed the same way, whoever walks it: there and back, year on year, priced
> in the map's movement cost. Trails lower it for everyone.
>
> Every actor trades through the same market algorithm, with no money. A clan's market is a
> festival.
>
> The rules choose for clans: which tiles to hold, which festival to walk to. The player
> chooses for the civ's units, through orders. An order is a route or a territory the player
> has fixed; the actor divides its labour around it as any actor does.
>
> Later: a farmer's territory is its fields (the same rule for adding a tile decides how
> many); a steward's is the land he taxes.

From `docs/design/clans.md`:

> - **Territory.** A clan holds a connected patch of tiles around its home tile.
> - **Labour.** A clan divides its gathering evenly over its territory. Walking inside its
>   own territory costs a clan little, but enough that trails still form.
> - **Holding land.** A poor tile takes labour from better ones, so it can lower the harvest.
>   A hungry clan swaps its worst tile for a better one, so it drifts towards better ground.
>   Home follows the middle of the territory.
> - **Diet.** One food per person per turn. Any one food counts in full only up to a share of
>   need, and is worth less beyond that.
> - **Growth.** A clan grows only on what it harvests beyond its need. One that just meets
>   its need holds; well short, it starves and shrinks.
> - **Dividing.** A big clan splits its land and people in two. A starving clan sends a
>   daughter off to find land within a few days' walk.
> - **Moving on.** A clan that starves for several turns walks to a better site.
> - **Storage.** A small store that spoils.
> - **Festivals, one route.** Each clan walks one route, to one festival, each turn. It
>   changes festival when another is worth more. Going costs the walk there and back. It's
>   worth going when what the clan gains there, plus its desire to gather, is more than the
>   walk costs. Clans want to go to festivals, to see others and to share their surplus; a
>   clan holding a surplus gets a bonus towards going.
> - **Scale.** Per-clan work must not grow with the number of clans in the world.

## What to build

### The actor (`core/src/actors.rs`, new)

Shape it as you judge best; the kiln and B, C and F rely on what spec.md's "What the clay
leaves the kiln" lists. In short:

- An `Actor` with `people`, `home`, `store`, `territory` (home among it, connected), `routes`
  (destination, path tiles, yearly cost) and `market: Option<u32>`. `Clan` carries one, and
  nothing in `actors.rs` names clans. Serialised (the digest covers it).
- `walk_cost(path_cost, weight) = 2 × path_cost × weight`, the only way any walk is priced.
  Festival routes weigh 1; a clan's walks to its own tiles weigh `CLAN.inside` (start well
  under 1, e.g. 0.25; build-5 tunes it).
- Walks inside a territory: the cheapest path from home over the territory's own tiles by
  `land::step_cost`; each tile's cost and path.
- `divide`: labour less the routes' cost, split evenly over the territory, each tile's walk
  paid from its share; returns each tile's work labour (0 where the walk eats the share).
- The add/drop rule, generic over a value function: edge candidates (passable land next to
  the territory, nobody's, within `CLAN.territory_radius` hex steps of home); add only on a
  rise of `CLAN.land_margin`; drops never disconnect the territory or drop home. A function the
  chooser calls, so a later actor with a fixed territory just doesn't call it.
- After the harvest, which tiles every walk crossed this turn (inside paths and route paths,
  not home), readable per tile: build-2 makes trails from it. Recording it is yours; the trail
  rule isn't.

### Clans on it (`core/src/clans.rs`, `core/src/markets.rs`)

The rules are in spec.md "What's settled", The actor and Clans. In short:

- `round.rs` goes, with `Clan.round`, `fetch_range`, `round::options` and `plan_harvest`.
  `middle` and anything still needed move to where they belong. `Tile.claim` is the clan whose
  territory holds the tile.
- **Harvest**: each tile's work harvests its foods in proportion to stock, `harvest_labour` a
  unit, up to `take_rate` of stock, all tiles' work scaled by one factor so the clan stops at
  `surplus_target × need − store`. Shuffled order, first come first served.
- **Value**: nutrition per person of store plus that harvest from current stock.
- **`move` step**: hungry adds the best edge tile on a rise of `land_margin`, else swaps it
  for its worst; fed drops the tile it misses least if value doesn't fall; one change a turn;
  home steps towards the middle; tiles beyond `territory_radius` or cut off from home drop.
  Moving on as now.
- **Growth on surplus** in `eat`: `growth(since) × min(1, surplus / (CLAN.grow_full × need))`,
  `surplus = max(0, nutrition(harvest, pop) − pop)` (start `grow_full` around 0.3). Starving
  as now.
- **Dividing** halves the territory; each half keeps only what's connected to its home.
  Splitting as now (the daughter's territory is her site).
- **The `go` step**: score festivals with route cost from home under `MARKET.reach_walk`
  (`worth = gain × harvest_labour + desire − route cost`, `desire = MARKET.desire +
  MARKET.company × min(traders last turn, MARKET.company_cap)` + `gather_bonus` with a
  surplus); go to the best worth > 0; keep the current one unless another beats it by
  `MARKET.switch_margin`. The route is the actor's one route; `trip` is its cost.
- `form` keeps its rule but reads "in reach" as route cost from a clan's home within
  `MARKET.reach_walk`; `fade` and `trade` stay. Build-3 replaces forming and fading.

New numbers go in `data.rs` with doc comments like the others. Retire the ones nothing reads.

### Every reader of the round

`civs::found` (joining by reach reads the territory's tiles), `units` (a clan near the
capital), `pregame::land_cover` (cover from territories), `stats::summary` (replace
`round_len`/`round_cost` with territory size and walk cost), the CLI, `web/src/lib.rs`'s
`view` (clans' JSON), and `www/` where it reads `c.round`: make the smallest change that keeps
the page working and `check.mjs` passing (build-4 redraws it). Founding, the levy and units
run as before.

### The sweep's series

`sansfiat sweep` gains `--every N` (default 50): after each seed's line, a line of its turn,
clans and people every N turns to the end, and per size a line saying whether people levelled
off (the change in people over the last 100 turns before the end, as a share). Leave room for
build-2's depleted tiles and build-3's people moved as columns of the same series.

### Docs

`engine/CLAUDE.md`'s Layout (one clause a row: `actors.rs` in, `round.rs` out) and usage
line for the sweep.

## Tests

Rewrite `core/tests/rounds.rs` as `core/tests/actors.rs`, and fix `clans.rs`, `markets.rs`,
`pregame.rs`, `civs.rs`, `units.rs`, `events.rs` and `stats.rs` where they read rounds or
assert numbers the new rules change. Tests on real maps assert what must hold, not end turns
or counts: build-5 retunes everything. At least:

- `walk_cost`; `divide` on a hand-built territory: equal shares, walks paid first, a far
  tile's work 0, routes paid before the division.
- The add/drop rule: never disconnects, never drops home, adds only on a rise; a poor far tile
  lowers the value and isn't added; a hungry clan adds, a fed one drops.
- No harvest from tiles off the territory.
- Growth: a clan that harvests just its need holds; one that harvests beyond grows; a store
  alone doesn't make it grow.
- Dividing leaves two connected territories, neither sharing a tile.
- The festival route: picks the best by worth, keeps its festival within the margin, goes
  nowhere when nothing beats the walk; `trip` is the route's cost.
- Every turn on a few real seeds: every territory connected, contains home, within the
  radius, and every tile's claim matches exactly one territory.
- A save replays to the same world; the pre-game on a small seed ends with three centres
  (by whatever path, top-up included).

## Done

- `engine/check.sh` passes (cross-build's native and wasm digests agree).
- `cargo build --release -p sansfiat && ./target/release/sansfiat sweep --size all` runs;
  put its output, and `bench` against today's 3–5 ms a turn on small, in
  `relay/specs/54/clay-sweep.md`, and say in your summary how the pre-game runs now.
- Your summary names the actor's types and functions as they are, for the kiln.
