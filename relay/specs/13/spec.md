# Spec: #13, Proof: clans and the pre-game

## The question

Does a sparse world of clans, walking their rounds and paying real labour to reach their food,
spread across the land and form centres with markets around fertile areas on its own, with no
rule placing them?

## What the piece delivers

The pre-game of `docs/design/clans.md`, in the Rust engine, on the placeholder map:

- **The land** (design: *The land*): five foods, terrain food caps, fertility in patches
  (×0.4 to ×2) and regrowth with breeding game.
- **Clans** (*Clans*): seeds, the diet with its diversity bonus, growth, starving, dividing,
  moving on and a store that spoils.
- **The round** (*The round*, *Changing the round*): territory as a loop from home; labour
  for walking it, for harvesting on and off it, and for going to market; changing it one tile
  at a time, with cheapest insertion, starving swaps, home following the middle and the
  2-step limit.
- **Trails** (*Trails*), **grain and hearths** (*Grain and hearths*).
- **Markets** (*Markets*): forming, going (trade gain plus the desire to gather against the
  walk), clearing with the existing `market::clear`, and fading.
- **The pre-game end** (*The pre-game ends*): 3 centres at least `centreSpacing` apart, land
  cover of 80%, a turn limit as backstop. The world records that the pre-game ended, and
  why; it doesn't found anything.
- **The evidence**: CLI stats, a seed sweep, and the web map drawing clans, rounds, trails,
  markets, hearths and centres.

The rules carry no term that places clans, markets or centres: fertility enters only through
food caps. The review checks that, because it's what the question turns on.

It's a proof: its PR is never merged. Shortcuts it takes, outside the question:

- Centres are picked greedily (busiest market first), not by searching every triple.
- A leg between two round tiles that aren't neighbours passes through tiles that don't count
  as on the round (they wear no trail).
- The placeholder map keeps its Voronoi terrain; coast tiles stand in for shore and shallows.
- Tuning is one pass over the starting values, recorded, not a search.

## Scope map

| Step | In-scope line it serves |
|---|---|
| build-1: Food on the land, and clans that eat | Food on the placeholder map …; Clans: seeds, diet with the diversity bonus, growth, starving, … a small store that spoils; The round: territory as a loop from home, labour for walking it, harvesting on and off it |
| build-2: Rounds that change, clans that divide and move | The round: … changing it one tile at a time (cheapest insertion, starving swaps, home follows the middle, 2-step limit); Clans: … dividing, moving on |
| build-3: Trails, hearths, markets and the end | Trails worn by rounds kept for `trailAfter` turns …; Grain taming on held land, going wild on unheld land, and hearths; Markets: forming, going, clearing, fading; The pre-game end: 3 centres … a turn limit as a backstop |
| build-4: Show it: stats, sweep, web map, tuning | What a proof needs to show the answer: CLI stats and the web map drawing clans, rounds, trails, markets and hearths |
| review | Every line: checks the piece against the question and writes the findings |

## Design questions

Alan answers these in `docs/design/clans.md` before building starts. The briefs carry the
recommendations; where the design file answers differently, the build follows the design
file.

### 1. Map size

The placeholder map has radius 8: land within 7 of the middle. Three centres at least 10
apart only fit at its far corners, so the pre-game would almost never end.

- **a.** Raise `Config::default().radius` to 16 (about 800 tiles, roughly 600 land), and
  the web page's default with it. Tests that need speed pass a small radius.
- **b.** Keep 8 as the default and run the proof with `--radius 16`.
- **c.** Scale `centreSpacing` with the map (say radius × 0.6).

**Recommend a.** The design's spacing is a number of steps, and every run of the pre-game
should be able to end.

### 2. Food on the placeholder map

The design gives the five foods, fertility ×0.4 to ×2 and breeding game, but no caps, and
the placeholder map has no rivers or shallows (out of scope), only a `coast` terrain: land
next to sea.

- **a.** The poc's caps for the land terrains, and coast tiles carry the sea foods (sea
  tiles carry nothing and stay impassable).
- **b.** Sea tiles next to land carry fish and shellfish, harvested off the round from the
  coast.

**Recommend a.** It keeps food on tiles a clan can stand on. Starting values (×
fertility; move cost is labour to enter the tile):

| Terrain | game | grain | fruit | fish | shellfish | move cost |
|---|---|---|---|---|---|---|
| sea | | | | | | impassable |
| coast | 1 | | | 4 | 3 | 1 |
| plains | 3 | 4 | | | | 1 |
| forest | 3 | | 4 | | | 2 |
| hills | 2 | | 1 | | | 2 |

Fertility as in the poc: each tile draws a uniform number, smoothed 3 times (a tile weighs
2, each neighbour 1), stretched to 0..1, then fertility = 0.4 + 1.6 × y^1.5, rounded to 0.1.
It's drawn after the terrain, so a seed's terrain doesn't change. Regrowth as in the poc,
each turn towards the cap: grain 0.25 of cap, fruit 0.2, fish 0.3, shellfish 0.2; game
breeds, `cap × 0.02 + 0.35 × stock × (1 − stock / cap)`. A clan takes at most half a tile's
stock a turn (`takeRate` 0.5).

### 3. The unit of labour

The design says walking the round costs its total move cost and an off-round harvest costs
2 × path cost plus the harvesting, but not how move cost compares with harvesting, or
whether walking is per person.

- **a.** One unit: a point of move cost is one labour. Walking and fetching are paid once
  per clan (the design's literal words). Each person gives `work` 2.5 labour a turn, and a
  unit of food takes `harvestLabour` 1 to harvest.
- **b.** Walking is per person: a round costs move cost × people × 0.1.
- **c.** Walking is a share of labour, as the poc's `tripCost` (0.05 of labour per point of
  path cost).

**Recommend a.** It's the design as written, and it gives bigger, settled clans an edge in
walking, which is what trails and centres need. A seed clan of 6 has 15 labour.

### 4. What a clan holds and where it harvests

The design says the territory is the round plus the tiles next to it, and that a clan can
walk a long way for food, but not whether territories may overlap or how far a clan fetches.

- **a.** A tile is on at most one clan's round (it's *held*, and only held tiles count for
  taming, trails and markets forming). A clan may harvest any tile within `fetchRange` 3
  path cost of its round that isn't on another clan's round, so tiles next to two rounds are
  shared, first come first served in a shuffled order.
- **b.** Whole territories are exclusive: no clan harvests in another's territory.
- **c.** As a., but harvesting only within the territory.

**Recommend a.** It's the poc's rule (land exclusive, the ring around it shared) with the
design's off-round cost, and the 3-cost reach keeps each clan's work bounded.

### 5. The round's shape

"A closed loop of tiles" leaves open whether consecutive tiles on the round must be
neighbours.

- **a.** The round is a list of distinct tiles starting at home. The walk between
  consecutive tiles (a *leg*) is the cheapest path between them over land within
  `territoryRadius` of home; the round's cost is the sum of its legs, including back to home.
  A round of only home costs 0.
- **b.** Consecutive tiles must be neighbours, so every tile walked is on the round.

**Recommend a.** Cheapest insertion and "reconnects the round around the gap" both work as
written. Tiles passed through on a leg aren't on the round (a shortcut).

### 6. When the round changes

The design says a clan grows its territory when it isn't fully fed, swaps when it isn't, and
holds no land it can't use, but not the order, or what "worth" is measured in.

- **a.** One change a turn. The value of a round is how well it would feed the clan this
  turn: the planned harvest with that round, plus the store, as a share of need (planning
  stops at 1.2 × need). Not fully fed last turn: add the free tile next to the round whose
  cheapest insertion raises the value most, if by more than `landMargin` 0.02; failing
  that, swap: add that tile and drop the round tile whose loss leaves the most value, if
  the swap gains more than 0.02. Fully fed: drop the tile whose loss leaves the most value,
  if the value doesn't fall. Then home steps one tile towards the middle, and round tiles
  more than 2 from home drop.
- **b.** As a., but a clan may add and swap in the same turn.

**Recommend a.** One tile a turn is what the design says.

### 7. Going to market, and the desire to gather

The design gives the rule (trade gain at last turn's prices, plus a fixed bonus with a
surplus, against the walk's labour) but no measure of the gain, of "surplus" or of the bonus.

- **a.** The gain is in food: the nutrition (with the diversity bonus) of what the clan
  would hold after trading its store and last turn's harvest at the market's last prices,
  less what it holds now, at `harvestLabour` labour per food. A surplus is a store of at
  least `surplusAt` 0.1 × people after eating. The bonus is `gatherBonus` 2 labour.
- **b.** As a., with the gain measured by the market library's utility (Σ √amount), as the
  poc did.

**Recommend a.** Nutrition is what clans value everywhere else.

### 8. Walking to a new site

The poc walks a moving clan or a daughter one tile a turn. A turn is a year or more here.

- **a.** A clan moving on, or a daughter, arrives the turn it leaves: it's at most 6 steps.
- **b.** It walks a tile a turn, holding no land and harvesting around where it stands.

**Recommend a.** A few days' walk is well under a turn, and it saves walking state.

### 9. Numbers the design names without values

Starting values, mostly the poc's:

| Name | Value | Meaning |
|---|---|---|
| seeds, startPop | 3, 6 | One seed per landmass of 30+ land tiles, then the rest anywhere, as the poc |
| dietShare, excessValue | 0.4, 0.3 | Poc |
| surplusTarget | 1.2 | A clan plans to harvest up to 1.2 × need, less its store (poc) |
| growth, settleTurns | 0.03 → 0.07, 30 | Poc |
| stableAt, starve | 0.8, 0.5 | Poc |
| spoilage | 0.3 | Poc |
| splitAt, hungrySplit, settleFed | 12, 8, 0.9 | Poc |
| patience, moveRange, moveMargin | 3, 4, 0.1 | Poc |
| scoutRange, choiceSpread | 6, 0.05 | Poc |
| territoryRadius | 2 | Design |
| trailAfter | 10 turns | A tile on a round 10 turns in a row gets a trail |
| trailFade | 30 turns | A trail on nobody's round for 30 turns in a row wears away |
| trail move cost | 1 | Design ("forest and hills to 1") |
| tendAfter, tameTurns, domestic | 15, 120, 2.5 | Poc (rate 1/120) |
| market formAfter, spacing, reach | 20, 5, 3 | Poc; reach is path cost |
| market minClans, fade, minVolume | 4, 0.8, 2 | Poc |
| centreAge | 30 turns | A centre's market has lasted 30 turns |
| centreClans | 4 | Clans trading there last turn (poc `minTraders`) |
| centreSpacing | 10 | Design; hex steps between centre markets |
| cover, coverSteps | 0.8, 3 | Design |
| maxTurns | 500 | Poc |

**Recommend** these as starting values; build-4 tunes them once and records what it changed.

## Design

### Turn order

`TURN` in `world.rs`, built up step by step:

| Step | Module | Built in | What |
|---|---|---|---|
| `regrow` | `land` | 1 | Every tile's stock regrows towards its cap |
| `move` | `clans` | 2 | In a shuffled order: moving on, else one round change; home towards the middle; the 2-step limit |
| `go` | `markets` | 3 | Each clan picks its market (or none) and sets `trip` |
| `harvest` | `clans` | 1 | In a shuffled order: labour, plan, take from stock; `basket = store + harvest` |
| `trade` | `markets` | 3 | Each market clears its clans' baskets |
| `eat` | `clans` | 1, 2 | Eat, grow or starve, die; then divide and split (2) |
| `hold` | `land` | 3 | Held counters, trails, taming |
| `markets` | `markets` | 3 | Fade, then form at most one |
| `end` | `pregame` | 3 | Sets `World.end` once 3 centres and the cover are there, or at `maxTurns` |

### Files

| File | Change |
|---|---|
| `core/src/data.rs` | `GOODS`/`FOODS` become the five foods; `Terrain` gains `move_cost` (`None` for sea) and `food`; `FOOD` regrowth; `CLAN`, `TRAIL`, `TENDING`, `MARKET`, `PREGAME` rule structs; `Config::default().radius` 16 |
| `core/src/map.rs` | `Tile` gains food and holding fields; fertility patches; caps; `index(radius, hex)`; `landmasses` |
| `core/src/land.rs` | New: `neighbours`, `step_cost`, `cost_map`, `regrow` (1); `hold` (3) |
| `core/src/round.rs` | New: `leg`, `cost`, `territory`, `options` (1); `insertion`, `free_edge`, `middle` (2) |
| `core/src/clans.rs` | New: `Clan`, `seed`, `nutrition`, `plan_harvest`, `harvest`, `eat` (1); `adjust`, `divide`, `split`, `move_on`, `site_value`, `pick_site` (2) |
| `core/src/markets.rs` | New (3): `Market`, `reach`, `go`, `trade`, `form_and_fade` |
| `core/src/pregame.rs` | New (3): `End`, `centres`, `land_cover`, `check_end` |
| `core/src/stats.rs` | New (4): `Summary`, `summary`, `centre_report` |
| `core/src/world.rs` | `World` gains `clans`, `markets`, `next_id`, `end`; `World::new` seeds clans; `TURN` |
| `core/tests/market.rs` | Baskets become 5 goods long |
| `core/tests/{land,clans,rounds,markets,pregame}.rs` | New tests (below) |
| `cli/src/main.rs` | `report` grows a line per step; build-4: `run --every`, stop at the end, `sweep` |
| `web/src/lib.rs`, `www/*` | build-4: `summary()` export, drawing and the panel |
| `relay/specs/13/tuning.md` | build-4: the sweep before and after tuning |

### Types

```rust
// data.rs
pub const GOODS: [&str; 5] = ["game", "grain", "fruit", "fish", "shellfish"];
pub const N: usize = GOODS.len();
pub const GAME: usize = 0; pub const GRAIN: usize = 1; pub const FRUIT: usize = 2;
pub const FISH: usize = 3; pub const SHELLFISH: usize = 4;
pub struct Terrain { pub name, pub color, pub land: bool, pub move_cost: Option<f64>, pub food: [f64; N] }
pub struct Food { pub regen: f64, pub breeds: bool, pub restock: f64 }
pub const FOOD: [Food; N];
pub struct ClanRules { seeds, start_pop, min_landmass, work, harvest_labour, take_rate, surplus_target,
    diet_share, excess_value, growth: (f64, f64), settle_turns, stable_at, starve, spoilage, split_at,
    hungry_split, settle_fed, patience, move_range, move_margin, scout_range, choice_spread,
    land_margin, territory_radius: i32, fetch_range: f64 }
pub struct TrailRules { after: u32, fade: u32, cost: f64 }
pub struct TendingRules { after: u32, tame_turns: u32, domestic: f64 }
pub struct MarketRules { form_after: u32, spacing: i32, reach: f64, min_clans: usize, fade: f64,
    min_volume: f64, gather_bonus: f64, surplus_at: f64 }
pub struct PregameRules { players: usize, centre_age: u32, centre_clans: usize, centre_spacing: i32,
    cover: f64, cover_steps: i32, max_turns: u32 }

// map.rs
pub struct Tile { q, r, terrain,
    pub fertility: f64, pub cap: Basket, pub stock: Basket,
    pub wild_grain: f64,        // grain cap before taming
    pub tame: u32,              // 0..=TENDING.tame_turns; a hearth at tame_turns
    pub claim: Option<u32>,     // the clan whose round it's on
    pub held: u32,              // turns in a row on some round
    pub unheld: u32,            // turns in a row on no round
    pub trail: bool }

// clans.rs
pub struct Clan { pub id: u32, pub home: usize, pub round: Vec<usize>, // round[0] == home
    pub pop: f64, pub store: Basket, pub harvest: Basket, pub basket: Basket,
    pub fed_ratio: f64, pub fed: bool, pub starving: bool, pub hungry: u32, pub since: u32,
    pub walk: f64,                 // the round's cost this turn
    pub market: Option<u32>, pub trip: f64 }   // set in build-3; 0 / None until then
pub struct Plan { pub take: Vec<(usize, Basket)>, pub food: Basket, pub labour: f64 }

// markets.rs
pub struct Market { pub id: u32, pub tile: usize, pub founded: u32, pub prices: Basket,
    pub activity: f64, pub volume: f64, pub traders: usize }

// pregame.rs
pub struct End { pub turn: u32, pub centres: Vec<u32>, pub cover: f64, pub by_limit: bool }

// world.rs: World gains
pub clans: BTreeMap<u32, Clan>, pub markets: BTreeMap<u32, Market>, pub next_id: u32, pub end: Option<End>
```

No `HashMap` is iterated anywhere a rule reads it, floats are compared with `total_cmp` and
ties broken by tile index or id, and all randomness goes through `rng::rand(&mut w.rng)`, so
a save still replays exactly.

### The rules, as built

**Labour.** A clan's labour is `pop × work`. The round's cost (`round::cost`) comes off
first, then the market trip (`trip`, 2 × path cost from the nearest round tile), and what's
left goes to harvesting.

**Harvest options.** Every tile on the round, with no setup; and every tile within
`fetchRange` path cost of the round (multi-source `cost_map` from the round tiles), not on
another clan's round, with setup 2 × its path cost.

**Planning (`plan_harvest`).** Food budget `max(0, surplusTarget × pop − Σ store)`. Each
tile is opened once; opening an off-round tile pays its setup. Phase 1 fills each food up to
`dietShare × pop` (counting the store), phase 2 takes anything up to the budget. In each
phase it repeatedly takes the tile whose take gives the most food per labour,
`amount / (setup + harvestLabour × amount)`, scaled down when labour or budget is short,
where a tile offers `takeRate × stock` of each food (in phase 1, capped per food at what's
still missing from its share). Ties go to the lower tile index.

**Eating** as the poc's `eat`: each food counts in full up to `dietShare × need`, the rest at
`excessValue`; leftovers spoil by `spoilage` into the store; fed (≥ 0.999) grows by a growth
rate from 0.03 to 0.07 over `settleTurns` turns since moving; below `stableAt` it starves,
`pop × (1 − starve × (1 − fed / stableAt))`; under 1 person the clan dies and its round is
freed.

**Round changes** as Design question 6, with cheapest insertion: the position between
consecutive round tiles `a, b` (including the last back to home) that minimises
`leg(a, t) + leg(t, b) − leg(a, b)`. Home steps to a neighbouring round tile nearer the
territory's middle (the territory tile with the least total hex distance to the others;
ties to home, then lowest index), and the round is rotated to start there.

**Dividing.** At `splitAt` people with 2+ round tiles, the round's tiles split in two halves
grown from its two farthest tiles (the poc's `halve`); the daughter takes the half without
home, half the people and half the store; her home is her half's middle and her round is
built by cheapest insertion from it. With 1 tile, or starving at `hungrySplit` people, a
daughter of `min(pop, splitAt) / 2` goes to a free site within `scoutRange` steps whose
value for her is at least `settleFed`, picked by `pick_site`. **Moving on**: after `patience`
starving turns, a clan moves to a free site within `moveRange` steps worth more than its own
round by `moveMargin`. Arrivals are the same turn; the new round is `[site]`, `since` 0.

**Trails and taming** (`hold`): a held tile's `held` counts up and `unheld` resets; an unheld
tile's `held` resets and `unheld` counts up. `held ≥ trailAfter` gives a trail;
`unheld ≥ trailFade` takes it away. A trail makes `step_cost` `min(cost, 1)`. Grain on a
tile with `held ≥ tendAfter` gains a step of `tame`, grain on an unheld tile loses one; the
grain cap is `wild_grain × (1 + (domestic − 1) × tame / tameTurns)`; at `tameTurns` the tile
is a hearth.

**Markets.** A market's reach is a `cost_map` from its tile out to `reach`. *Go*: each clan
looks for the market nearest any of its round tiles within reach, and goes if Design
question 7's test passes. *Trade*: each market clears its clans' baskets with
`clear(&baskets, &m.prices, &Params::default())`, takes the new prices if it cleared, and
smooths `activity = fade × activity + (1 − fade) × volume`. *Fade*: a market with
`activity < minVolume` goes (it starts at `2 × minVolume`). *Form*: among clan homes held
`formAfter` turns, at least `spacing` steps from every market, with at least `minClans`
clans holding a round tile within reach, the one with the most people in reach (ties to the
lowest tile) gets a market. One a turn.

**The end** (`check_end`, once `end` is `None`): a centre is a market at least `centreAge`
turns old with at least `centreClans` traders last turn and a hearth within its reach.
`centres` takes them busiest first (ties older, then lower id), each at least
`centreSpacing` steps from those taken, up to 3. Cover is the share of land tiles connected
over land to a centre's market that lie within `coverSteps` steps of a territory tile. With
3 centres and cover ≥ 0.8, `end` is set; at `turn + 1 ≥ maxTurns` it's set with
`by_limit`. Turns may still run after it; the CLI stops there.

### Tests

- `land.rs` (1): foods are the goods; caps are terrain × fertility and sea has none; stock
  starts at cap; fertility in 0.4..=2 and patchy (neighbours closer than random pairs);
  `index` matches `within`; `step_cost` (sea none, forest 2, trail 1); regrowth exact for
  grain and for breeding game, and a hunted-out game tile recovers slower than grain.
- `clans.rs` (1, 2): seeds; `nutrition`; `plan_harvest` (round before off-round, setup paid
  once, labour and budget limits, a missing food before excess); walking comes off first;
  eating, growth, starving, spoilage, dying; divide and split conserve people and store;
  moving on.
- `rounds.rs` (2): cheapest insertion on a hand-built round; worth working; dropping;
  starving swap; home moves at most a tile a turn; over 300 turns on seeds 1 to 3: rounds
  disjoint and matching `claim`, home first, every round tile within 2 of home, one change a
  turn per clan; clans spread past the seeds.
- `markets.rs` (3, plus the existing library tests at 5 goods): forming, going and not
  going, trip labour, clearing conserves food, fading; trails and taming in `hold`.
- `pregame.rs` (3): `centres` and `land_cover` on hand-built worlds; `end` by centres and by
  the limit.
- `world.rs` (existing): same seed same world and save replay, now with clans and markets.
- `stats.rs` (4): `summary` matches the world it reads.

### Evidence (build-4 and the review)

`sansfiat run --seed N [--turns N] [--every N]` prints a stats line every N turns and, at
the end, the centres: where, how far apart, clans trading, hearths in reach, and the mean
fertility of land within 2 steps of each against the land's mean. `sansfiat sweep 1 2 … 10`
prints a line a seed: end turn, by centres or limit, cover, clans, and the centres' fertility
ratio. The web page draws territories, rounds, trails, clans, markets with their reach,
hearths and centres, with a fertility view. The review runs the sweep and writes the answer.
