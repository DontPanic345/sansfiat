# build-1: Food on the land, and clans that eat

Piece #13 is a proof: does a sparse world of clans, walking their rounds and paying real
labour to reach their food, spread across the land and form centres with markets around
fertile areas on its own, with no rule placing them? The design is
`docs/design/clans.md`; the whole spec is `relay/specs/13/spec.md`. This step lays the
ground: food on the placeholder map, and clans that harvest it with labour and eat it. Their
rounds don't change yet (build-2), and there are no trails, taming or markets (build-3).

Where `docs/design/clans.md` now answers one of the spec's design questions differently from
this brief, follow the design file.

Read `engine/CLAUDE.md` first: the world changes only through `act`, a turn runs the `Step`s
in `TURN`, all randomness goes through `rng::rand(&mut w.rng)`, and a save must replay to an
identical world. Never iterate a `HashMap` in a rule; use `Vec` or `BTreeMap`, compare floats
with `total_cmp`, and break ties by tile index or id.

## What to build

### `core/src/data.rs`

- `GOODS` becomes the five foods: `["game", "grain", "fruit", "fish", "shellfish"]`, with
  `pub const GAME = 0, GRAIN = 1, FRUIT = 2, FISH = 3, SHELLFISH = 4` and `FOODS` as an alias
  of `GOODS`. `N` stays `GOODS.len()`. Update the comment ("The clans piece decides the real
  list") to say these are the foods, and only food exists for now.
- `Terrain` gains `move_cost: Option<f64>` (labour to enter the tile; `None` is impassable)
  and `food: [f64; N]` (its caps before fertility):

  | Terrain | game | grain | fruit | fish | shellfish | move cost |
  |---|---|---|---|---|---|---|
  | sea | | | | | | None |
  | coast | 1 | | | 4 | 3 | 1 |
  | plains | 3 | 4 | | | | 1 |
  | forest | 3 | | 4 | | | 2 |
  | hills | 2 | | 1 | | | 2 |

  Coast tiles stand for the shore and the shallows, so they carry the sea foods; sea carries
  nothing.
- `pub struct Food { regen: f64, breeds: bool, restock: f64 }` and `pub const FOOD: [Food; N]`:
  game `{0.35, true, 0.02}`, grain `{0.25}`, fruit `{0.2}`, fish `{0.3}`, shellfish `{0.2}`
  (`breeds` false, `restock` 0 for all but game).
- `pub const FERTILITY: (f64, f64) = (0.4, 2.0);`
- `pub struct ClanRules` and `pub const CLAN: ClanRules`, with these values (all fields
  `pub`; build-2 uses the ones marked 2):

  | Field | Value | Meaning |
  |---|---|---|
  | `seeds` | 3 | Seed clans |
  | `start_pop` | 6.0 | People in a seed clan |
  | `min_landmass` | 30 | Land tiles for a landmass to get its own seed |
  | `work` | 2.5 | Labour a person gives a turn |
  | `harvest_labour` | 1.0 | Labour to harvest one unit of food |
  | `take_rate` | 0.5 | Most of a tile's stock a clan takes in a turn |
  | `surplus_target` | 1.5 | A clan plans to harvest up to this × need, less its store (the poc's 1.2 can't fully feed a clan on two foods: spec question 10) |
  | `diet_share` | 0.4 | A food counts in full up to this share of need |
  | `excess_value` | 0.3 | … and at this beyond it |
  | `growth` | (0.03, 0.07) | Growth when fed: just moved → settled |
  | `settle_turns` | 30 | Turns to reach the settled growth rate |
  | `stable_at` | 0.8 | Fed below this share, a clan starves |
  | `starve` | 0.5 | How fast a starving clan shrinks |
  | `spoilage` | 0.3 | Share of the store lost each turn |
  | `split_at` | 12.0 | (2) A clan this big divides |
  | `hungry_split` | 8.0 | (2) A starving clan this big sends a daughter |
  | `settle_fed` | 0.9 | (2) A daughter's site must feed her this well |
  | `patience` | 3 | (2) Starving turns before a clan moves on |
  | `move_range` | 4 | (2) Steps a clan looks when moving on |
  | `move_margin` | 0.1 | (2) How much better a site must be to move |
  | `scout_range` | 6 | (2) Steps a daughter looks |
  | `choice_spread` | 0.05 | (2) Softness of site choice |
  | `land_margin` | 0.02 | (2) Gain a round change must make |
  | `territory_radius` | 2 | No round tile is further than this from home |
  | `fetch_range` | 3.0 | Path cost from the round a clan will fetch food |

- `Config::default()` becomes `{ seed: 1, radius: 16 }`: three centres 10 steps apart need
  the room. Tests that need speed pass a smaller radius.

### `core/src/map.rs`

- `Tile` gains (all `pub`):
  `fertility: f64, cap: Basket, stock: Basket, wild_grain: f64, tame: u32, claim: Option<u32>,
  held: u32, unheld: u32, trail: bool`. `cap` is the terrain's `food` × fertility, rounded to
  0.1; `stock` starts at `cap`; `wild_grain` is the grain cap; `tame`, `held`, `unheld` start
  at 0, `claim` at `None`, `trail` false. (`tame`, `held`, `unheld` and `trail` are used from
  build-3; `claim` from this step.)
- Fertility in patches, drawn **after** the terrain and coast so a seed's terrain is
  unchanged: every tile draws `y = rand`, then 3 smoothing passes (each tile becomes
  `(2 × own + Σ neighbours) / (2 + count of neighbours on the map)`), then stretch to 0..1 by
  min and max, then `fertility = 0.4 + 1.6 × y^1.5`, rounded to 0.1.
- `pub fn index(radius: i32, h: Hex) -> Option<usize>`: the position of `h` in
  `Hex::ORIGIN.within(radius)` (the order `generate` builds tiles in), worked out
  arithmetically, `None` off the map.
- `pub fn landmasses(tiles, radius) -> Vec<Vec<usize>>`: the connected groups of land tiles,
  each sorted, the groups in order of their lowest tile.

### `core/src/land.rs` (new)

- `pub fn neighbours(w: &World, i: usize) -> impl Iterator<Item = usize>`: on-map
  neighbours, in `DIRS` order.
- `pub fn step_cost(w: &World, i: usize) -> Option<f64>`: the terrain's `move_cost`; with a
  trail, `min(cost, TRAIL.cost)`, where `pub const TRAIL` lives in `data.rs` with
  `after: 10, fade: 30, cost: 1.0` (build-3 sets trails; add the struct now so `step_cost` is
  final).
- `pub fn cost_map(w: &World, from: &[usize], max: f64, allowed: impl Fn(usize) -> bool) -> BTreeMap<usize, f64>`:
  bounded Dijkstra. Sources cost 0; entering a tile costs its `step_cost`; only `allowed`
  tiles are entered; stops past `max`.
- `pub fn regrow(w: &mut World)`, the `regrow` step: every food on every tile moves towards
  its cap. Not breeding: `stock = min(cap, stock + cap × regen)`. Breeding (game):
  `stock = min(cap, stock + cap × restock + regen × stock × (1 − stock / cap))`.

### `core/src/round.rs` (new)

The round is the clan's yearly walk: `clan.round`, a list of distinct tiles, `round[0]` the
home. The walk between consecutive tiles (a **leg**), including from the last back to home,
is the cheapest path between them over passable tiles within `territory_radius` of home.

- `pub fn leg(w, home: usize, a: usize, b: usize) -> Option<f64>`: `cost_map` from `a`,
  allowed tiles within `territory_radius` hex steps of `home`, read at `b`. `a == b` is 0.
- `pub fn cost(w, round: &[usize]) -> f64`: the sum of the legs around the loop. One tile:
  0. Two tiles: there and back. A leg with no path counts as `f64::INFINITY` (build-2 drops
  such tiles at once, so it only shows in a half-built round).
- `pub fn territory(w, round: &[usize]) -> Vec<usize>`: the round's tiles and their land
  neighbours, sorted, no repeats.
- `pub fn options(w, clan_id: u32, round: &[usize]) -> Vec<(usize, f64)>`: what the clan can
  harvest and the setup labour for each: every round tile at 0; every other tile within
  `fetch_range` path cost of the round (`cost_map` from all the round tiles, over passable
  tiles), with some food in stock and not on another clan's round (`claim` none or this
  clan), at `2 × path cost` (there and back from the nearest round tile). Sorted by tile.

### `core/src/clans.rs` (new)

`Clan` and `Plan` derive what `World` needs (`Clone, Debug, PartialEq, Serialize, Deserialize`),
and so do every struct added to `World` or `Tile` in this piece.

```rust
pub struct Clan {
    pub id: u32, pub home: usize, pub round: Vec<usize>, // round[0] == home
    pub pop: f64, pub store: Basket, pub harvest: Basket, pub basket: Basket,
    pub fed_ratio: f64, pub fed: bool, pub starving: bool, pub hungry: u32, pub since: u32,
    pub walk: f64,                // the round's cost this turn
    pub market: Option<u32>,      // build-3; None until then
    pub trip: f64,                // build-3: labour for the walk to market; 0 until then
}
pub struct Plan { pub take: Vec<(usize, Basket)>, pub food: Basket, pub labour: f64 }
```

- `World` gains `pub clans: BTreeMap<u32, Clan>`, `pub next_id: u32` (ids for clans and,
  later, markets, from 1). Build-3 adds `markets` and `end`; leave them out.
- `pub fn seed(w: &mut World)`, called at the end of `World::new`: one clan on a random land
  tile with some food in each landmass of at least `min_landmass` tiles (in `landmasses`
  order), then more on random land tiles with food until there are at least `seeds` (skip a tile
  already held; with more big landmasses than `seeds`, every one still gets its seed). Each has `start_pop` people, round `[home]`, `claim` set on home, fed, an
  empty store.
- `pub fn nutrition(b: &Basket, need: f64) -> f64`: with `k = diet_share × need`,
  `Σ min(b_f, k) + excess_value × Σ max(0, b_f − k)`.
- `pub fn plan_harvest(w: &World, pop: f64, store: &Basket, labour: f64, options: &[(usize, f64)]) -> Plan`:
  - Food budget `max(0, surplus_target × pop − Σ store)`. Labour left starts at `labour`.
  - A tile offers `take_rate × stock` of each food. Each tile is opened at most once;
    opening a tile pays its setup labour.
  - **Phase 1, diet shares first.** A food is wanted up to `diet_share × pop`, counting the
    store and what's planned. Repeatedly pick the tile (opened or not) that still offers
    something wanted, whose take (each food capped at what's still wanted) gives the most
    food per labour, `amount / (setup if not yet opened + harvest_labour × amount)`; scale
    the take down so it fits the labour left and the budget; take it. Stop when nothing is
    wanted, nothing wanted is on offer, or no labour or budget is left.
  - **Phase 2, then how much it can take.** The same, with every food wanted up to the
    budget (what each tile has left to offer).
  - A tile whose setup is more than the labour left can't be opened. Ties go to the lower
    tile index. `labour` in the plan is what it used; `food` the total.
- `pub fn harvest(w: &mut World)`, the `harvest` step: for each clan in a shuffled order
  (Fisher–Yates with `rand`), `walk = round::cost`, labour
  `max(0, pop × work − walk − trip)`, plan with `round::options`, take the plan from the
  tiles' stock, set `harvest` to the plan's food, `basket = store + harvest`,
  `store = 0`. Clans harvest one after another, so a tile two clans can reach is first come,
  first served.
- `pub fn eat(w: &mut World)`, the `eat` step, in id order, as the poc's `eat`:
  `need = pop`, `k = diet_share × need`; `got = Σ min(basket_f, k)`; the excess
  `left_f = max(0, basket_f − k)`, of which `eaten = min(Σ left, max(0, need − got) / excess_value)`
  is eaten (`got += eaten × excess_value`); what's left of each food, scaled by
  `1 − eaten / Σ left`, times `1 − spoilage`, is the store. `fed_ratio = got / need`,
  `fed = fed_ratio ≥ 0.999`, `starving = fed_ratio < stable_at`; `hungry` counts starving
  turns in a row (0 when not starving); `since += 1`. Fed: `pop × (1 + g)` with
  `g = growth.0 + (growth.1 − growth.0) × min(1, since / settle_turns)`. Starving:
  `pop × (1 − starve × (1 − fed_ratio / stable_at))`. Under 1 person the clan dies: its
  round tiles' `claim` goes back to `None` and it's removed.
- `TURN` becomes `regrow`, `harvest`, `eat` (build-2 puts `move` before `harvest`, build-3
  more).

### `cli/src/main.rs`

`report` (text) adds a line: clans, people, mean fed ratio, tiles held.

### `core/tests/market.rs`

The library tests hard-code 4 goods (`[0.0; 4]`, 4-long baskets). Make them 5 long (or use
`N`) with the same meaning; the expected numbers don't change.

## Tests

In `core/tests/land.rs` and `core/tests/clans.rs`, plus the existing tests passing:

- `GOODS` is the five foods in order.
- Every tile's `cap` is its terrain's `food` × fertility (to 0.1); sea tiles have none;
  `stock == cap` at the start; every land tile's fertility is in 0.4..=2.0.
- Fertility is patchy: on seed 1 at radius 16, the mean difference between neighbours is
  well under the mean difference between random pairs, and fertility spans at least
  0.7..1.5.
- `index` matches `within` for every hex at radii 0, 1, 5 and 16, and is `None` off the map.
- `step_cost`: sea `None`, plains 1, forest 2, forest with a trail 1.
- Regrowth: grain taken to 0 regrows `0.25 × cap` a turn; game at `0.1 × cap` regrows
  exactly by the breeding formula, and takes more turns to get back to cap than grain does.
- Seeds: `max(seeds, big landmasses)` clans of `start_pop` on distinct land tiles, round `[home]`, `claim` set; every landmass of `min_landmass`+ tiles has one.
- `nutrition`: need 10, one food 10 → 4 + 0.3 × 6 = 5.8; four foods of 2.5 each → 10.
- `plan_harvest` on a hand-built world (set tiles' stock directly):
  - with the same food on a round tile and a neighbouring off-round tile, it takes the round
    tile first;
  - an off-round tile's setup is paid once however much it takes;
  - it never uses more than its labour, nor plans more than the budget;
  - holding plenty of grain in store, it takes a missing food before more grain.
- Walking comes off first: a clan whose round costs more than its labour harvests nothing
  (build the round by hand).
- Eating: a fed clan grows by the growth rate for its `since`; a clan at 0.9 fed holds
  steady; at 0.4 fed it shrinks by `starve × (1 − 0.4 / 0.8)`; the store is what's left
  less `spoilage`; a clan under 1 person is removed and its tiles' `claim` cleared.
- Over 50 turns at radius 16 on seeds 1 to 3, clans live, people stay above 0, no tile's
  stock is negative or above cap.
- `same_seed_same_world` and `replaying_a_save_rebuilds_the_world` still pass.

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` in
`engine/`, and `./build-web.sh` (the web crate must still build).

## Out of scope for this step

- Changing rounds, dividing, splitting and moving on (build-2).
- Trails being set, taming, hearths, markets, the end of the pre-game (build-3).
- The web map and the CLI's stats and sweep (build-4).
- From the piece: good and bad years, materials and crafts, civs, a new map generator,
  hidden trail costs, traders.
