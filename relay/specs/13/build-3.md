# build-3: Trails, hearths, markets and the end

Piece #13 is a proof: does a sparse world of clans, walking their rounds and paying real
labour to reach their food, spread across the land and form centres with markets around
fertile areas on its own, with no rule placing them? The design is
`docs/design/clans.md`; the whole spec is `relay/specs/13/spec.md`. build-1 and build-2 made
the land, clans that harvest with labour and eat, rounds that change a tile at a time, and
clans that divide and move. This step adds trails worn by rounds, grain taming and hearths,
markets (forming, going, clearing, fading), and the end of the pre-game.

Where `docs/design/clans.md` now answers one of the spec's design questions differently from
this brief, follow the design file.

Read `engine/CLAUDE.md`, then `core/src/{data,land,round,clans,market,world}.rs` as they are.
All randomness goes through `rng::rand(&mut w.rng)`; never iterate a `HashMap` in a rule;
compare floats with `total_cmp` and break ties by tile index or id, so saves replay exactly.
No rule here may look at fertility or place anything by where it is good to be: markets and
centres must come only from where clans live and trade. That's the question.

## What's there

- `Tile { …, fertility, cap, stock, wild_grain, tame: u32, claim: Option<u32>, held: u32,
  unheld: u32, trail: bool }`. A tile is **held** when it's on some clan's round
  (`claim.is_some()`).
- `land::step_cost` already drops a trailed tile's cost to `TRAIL.cost` (1);
  `land::cost_map(w, from, max, allowed)` is a bounded Dijkstra.
- `Clan { …, round, pop, store, harvest, basket, fed, market: Option<u32>, trip: f64 }`:
  `harvest` subtracts `trip` from the clan's labour already; `market` and `trip` are always
  `None` and 0 so far.
- `market::clear(baskets, start_prices, &Params::default()) -> Clearing { prices, after,
  volume, cleared, .. }` and `market::demand(basket, prices, goods)`: barter clearing with no
  money. It conserves every good. Use it as it is.
- `clans::nutrition(basket, need)`: food counting each food in full up to 0.4 × need.
- `TURN`: `regrow`, `move`, `harvest`, `eat`.

## What to build

### Numbers (`data.rs`)

```rust
pub const TRAIL = TrailRules { after: 10, fade: 30, cost: 1.0 };        // exists
pub const TENDING = TendingRules { after: 15, tame_turns: 120, domestic: 2.5 };
pub const MARKET = MarketRules { form_after: 20, spacing: 5, reach: 3.0, min_clans: 4,
    fade: 0.8, min_volume: 2.0, gather_bonus: 2.0, surplus_at: 0.1 };
pub const PREGAME = PregameRules { players: 3, centre_age: 30, centre_clans: 4,
    centre_spacing: 10, cover: 0.8, cover_steps: 3, max_turns: 500 };
```

### Trails and taming: the `hold` step (`land.rs`)

For every tile, after eating:

- Held: `held += 1`, `unheld = 0`. Unheld: `held = 0`, `unheld += 1`.
- **Trails.** `held ≥ TRAIL.after` (10 turns in a row on some clan's round, any clan) sets
  `trail`. A trail on a tile with `unheld ≥ TRAIL.fade` (30) wears away. Trails belong to the
  tile: every clan's legs and paths already use `step_cost`, so everyone gets them.
- **Taming.** Only tiles with `wild_grain > 0`. Held with `held ≥ TENDING.after`: `tame`
  rises by 1, up to `tame_turns`. Unheld: `tame` falls by 1, down to 0. Then
  `cap[GRAIN] = wild_grain × (1 + (domestic − 1) × tame / tame_turns)`, rounded to 0.1.
  A tile with `tame == tame_turns` is a **hearth** (`pub fn is_hearth(t: &Tile) -> bool`):
  2.5 times its wild grain.

### Markets (`markets.rs`, new)

```rust
pub struct Market { pub id: u32, pub tile: usize, pub founded: u32, pub prices: Basket,
    pub activity: f64, pub volume: f64, pub traders: usize }
```

`World` gains `pub markets: BTreeMap<u32, Market>` (ids from `next_id`).

- `pub fn reach(w, tile) -> BTreeMap<usize, f64>`: `cost_map` from the market's tile over
  passable tiles out to `MARKET.reach` (3).
- **Going**, the `go` step, before `harvest`. Build each market's reach once. For each clan:
  the market nearest to any of its round tiles within reach (least cost, ties to the lower
  id), at cost `c`. No market: `market = None`, `trip = 0`. Otherwise:
  - the basket it expects to bring, `b = store + harvest` (last turn's harvest);
  - its gain from trading at last turn's prices: `goods` the foods with `b > 0`,
    `d = demand(&b, &m.prices, &goods)`, `gain = max(0, nutrition(d, pop) − nutrition(b, pop))`;
  - its desire to gather: `gather_bonus` (2 labour) if it holds a surplus,
    `Σ store ≥ surplus_at × pop`, else 0;
  - the walk: `trip = 2 × c`, there and back from the nearest round tile.
  It goes if `gain × harvest_labour + bonus > trip`: `market = Some(id)`, `trip` set (the
  harvest then has that much less labour). Otherwise `None` and 0.
- **Clearing**, the `trade` step, after `harvest`. For each market in id order: the
  baskets of its clans (in id order); `clear(&baskets, &m.prices, &Params::default())`; each
  clan's basket becomes its `after`; if it cleared, `prices = c.prices`.
  `volume = Σ c.volume`, `traders` = clans that came,
  `activity = fade × activity + (1 − fade) × volume`.
- **Fading and forming**, the `markets` step, last but `end`:
  - Fading: a market with `activity < min_volume` goes; clans pointing at it get `None`.
  - Forming, at most one a turn: candidate tiles are clan homes with
    `held ≥ form_after` (lived on a long time), at least `spacing` hex steps from every
    market. For each, the clans in reach are those with any round tile in the candidate's
    `reach`. Candidates with at least `min_clans` clans in reach qualify; the one with the
    most people in reach (ties to the lower tile) gets a market: `founded = turn`, prices all
    1, `activity = 2 × min_volume`, `volume` 0, `traders` 0.

### The end (`pregame.rs`, new)

```rust
pub struct End { pub turn: u32, pub centres: Vec<u32>, pub cover: f64, pub by_limit: bool }
```

`World` gains `pub end: Option<End>`.

- `pub fn is_centre(w, m: &Market) -> bool`: `turn − founded ≥ centre_age` (30),
  `traders ≥ centre_clans` (4, last turn), and a hearth within the market's `reach`.
- `pub fn centres(w) -> Vec<u32>`: markets that are centres, busiest first (`traders`, then
  older, then lower id), each taken if at least `centre_spacing` (10) hex steps from every
  one taken, up to `players` (3). Greedy, not a search over triples: a shortcut.
- `pub fn land_cover(w, centres: &[u32]) -> f64`: the land tiles connected over passable land
  to any centre's market tile (all land if `centres` is empty); the share of them within
  `cover_steps` (3) hex steps of some clan's territory tile (`round::territory`), counted by
  a breadth-first search over the map from all territory tiles.
- `pub fn check_end(w)`, the `end` step: if `end` is `None`: with `centres.len() ==
  players` and `land_cover ≥ cover`, set `end` (`by_limit` false); else if
  `turn + 1 ≥ max_turns`, set it with whatever centres there are and `by_limit` true.
  Turns may run on after it; nothing else reads it yet.

`TURN` becomes: `regrow`, `move`, `go`, `harvest`, `trade`, `eat`, `hold`, `markets`, `end`.

The CLI's `report` adds a line: markets, clans trading, hearths, trails, cover, and the end
if set (turn, centres' tiles, by limit or not).

## Tests

In `core/tests/markets.rs` and `core/tests/pregame.rs` (hand-built worlds: set rounds,
stocks, markets and counters directly), plus every existing test:

- Trails: a forest tile on a round 10 turns in a row gets a trail and costs 1; the clan's
  round cost falls with it; 30 turns on no round and the trail is gone.
- Taming: grain held 15 turns starts gaining a step a turn and the cap rises with it; held
  long enough, the tile is a hearth with 2.5 × its wild grain; grain on unheld land loses
  a step a turn.
- Forming: a home held 20 turns with 4 clans in reach gets a market; with 3 it doesn't; a
  site under 5 steps from a market doesn't; two qualifying sites in one turn get one market.
- Going: a clan with only grain next to a market whose prices make fish cheap goes; the same
  clan with the market at path cost 3 and no surplus doesn't when the gain is under 6
  labour; a surplus adds `gather_bonus`; a clan that goes harvests with `trip` less labour.
- Clearing: food summed over a market's clans is the same before and after `trade`, and each
  clan's nutrition doesn't fall much (the library's utility can't fall).
- Fading: a market nobody goes to is gone once `activity` drops under 2.
- `centres`: three qualifying markets 10+ apart are all taken; one 6 steps from a busier one
  isn't; a market without a hearth in reach isn't a centre.
- `land_cover` on a small hand-built map gives the share by hand.
- The end: set up three centres and full cover and `end` is set by centres; set
  `turn = max_turns − 1` and run a turn, and it's set `by_limit`.
- `same_seed_same_world` and `replaying_a_save_rebuilds_the_world` still pass with markets.

Then run seed 1 to 3 at radius 16 for 500 turns with `cargo run --release -p sansfiat --
run --seed N --turns 500` and put in your summary whether each ended, when, and how many
markets and hearths there were. Not ending isn't a failure of this step: build-4 tunes.

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` in
`engine/`, and `./build-web.sh`.

## Out of scope for this step

- Founding civs or doing anything with the end beyond recording it.
- Traders between markets; hidden trail costs and scouting; trails anywhere but on rounds.
- Any pull towards fertile land or markets in where clans settle (site choice stays as
  build-2 made it).
- The web map, CLI stats and sweep, tuning (build-4).
- From the piece: good and bad years, materials and crafts, civs, a new map generator.
