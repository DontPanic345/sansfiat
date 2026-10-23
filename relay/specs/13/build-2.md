# build-2: Rounds that change, clans that divide and move

Piece #13 is a proof: does a sparse world of clans, walking their rounds and paying real
labour to reach their food, spread across the land and form centres with markets around
fertile areas on its own, with no rule placing them? The design is
`docs/design/clans.md`; the whole spec is `relay/specs/13/spec.md`. build-1 put food on the
map and clans that harvest it with labour and eat it, each with a fixed round of just its
home. This step makes rounds change one tile at a time, and makes clans divide, send
daughters off and move on, so the population spreads. Trails, taming and markets are
build-3.

Where `docs/design/clans.md` now answers one of the spec's design questions differently from
this brief, follow the design file.

Read `engine/CLAUDE.md`, then `core/src/{data,land,round,clans}.rs` as build-1 left them.
All randomness goes through `rng::rand(&mut w.rng)`; never iterate a `HashMap` in a rule;
compare floats with `total_cmp` and break ties by tile index or id, so saves replay exactly.

## What build-1 gives you

- `Clan { id, home, round, pop, store, harvest, basket, fed_ratio, fed, starving, hungry,
  since, walk, market, trip }`; `round[0] == home`; a tile's `claim` is the clan whose round
  it's on, and a tile is on at most one round.
- `round::leg(w, home, a, b)`: cheapest path over passable tiles within `territory_radius`
  (2) of home; `round::cost(w, round)`: the sum of legs around the loop;
  `round::territory`: round tiles plus their land neighbours; `round::options(w, id, round)`:
  harvestable tiles with setup labour (0 on the round, 2 × path cost off it, out to
  `fetch_range` 3).
- `clans::plan_harvest(w, pop, store, labour, options) -> Plan`; labour is `pop × work`
  (2.5) less the round's cost less `trip`.
- `CLAN` in `data.rs` holds every number below.

## What to build

### The value of a round

`pub fn value(w, clan: &Clan, round: &[usize]) -> f64` in `clans.rs`: how well that round
would feed the clan this turn, from the current stock:
`nutrition(store + plan.food, pop) / pop`, where the plan uses labour
`max(0, pop × work − round::cost(round) − trip)` and `round::options(w, clan.id, round)`.
It counts walking against food, so "worth the extra walking" needs no other rule. Planning
stops at `surplus_target` × need, so a well-fed clan gains nothing from more land.

### Changing the round (`round.rs`, `clans.rs`)

- `pub fn insertion(w, round: &[usize], t: usize) -> Option<(usize, f64)>`: where adding
  `t` lengthens the loop least. For each consecutive pair `a, b` (including the last tile
  back to home; a one-tile round has the single pair home → home), the extra cost is
  `leg(a, t) + leg(t, b) − leg(a, b)`; returns the position to insert at (after `a`) and the
  extra cost, the first position on ties, `None` if no leg reaches `t`.
- `pub fn free_edge(w, clan: &Clan) -> Vec<usize>`: tiles that could be added: land,
  passable, next to a round tile, on no round, within `territory_radius` of home. Sorted.
- `pub fn middle(w, tiles: &[usize], prefer: usize) -> usize`: the tile among `tiles` with the
  least total hex distance to the others; ties to `prefer`, then the lowest index.
- `pub fn adjust(w: &mut World)`, the `move` step, for each clan in a shuffled order
  (Fisher–Yates with `rand`). At most one change of tiles a turn:
  1. **Moving on.** If `hungry ≥ patience`, look for a site (below) within `move_range`
     steps. If the best is worth more than `value` of the current round by `move_margin`,
     pick among those that are (`pick_site`), free the round, and settle there: home and
     round `[site]`, `since` 0, `hungry` 0. That's the turn's change.
  2. **Not fully fed** (`!fed`, from last turn's eating): for each tile in `free_edge`,
     insert it at its `insertion` and take the `value`. **Adding a tile**: if the best beats
     the current value by more than `land_margin` (0.02), add it. **Starving swap**:
     otherwise, with that best tile added, drop the round tile (never home, never the tile
     just added) whose loss leaves the most value; do the swap if that beats the current
     value by more than `land_margin`.
  3. **Fully fed: dropping.** Find the round tile (not home) whose loss leaves the most
     value; drop it if that value is at least the current value (the clan doesn't hold
     land it can't use). Dropping removes the tile from the list, so its neighbours join
     up by a leg: the round reconnects around the gap.
  4. **Home follows the middle.** `middle` of the territory (`round::territory`), preferring
     home. If it isn't home, home steps to a neighbouring round tile that's nearer to it in
     hex steps (the lowest index if several); if none, home stays. The round is rotated to
     start at the new home.
  5. **Limit.** Any round tile more than `territory_radius` steps from home drops.
  Keep `claim` in step with every add and drop.

### Dividing, daughters and sites (`clans.rs`)

- `pub fn site_value(w, clan: &Clan, site: usize, pop: f64, store: &Basket) -> f64`: the
  value of round `[site]` for `pop` people with `store` (as `value`, with `trip` 0).
- `fn sites(w, clan, steps) -> Vec<usize>`: tiles within `steps` steps of home over
  passable land (breadth-first), on no round, in tile order.
- `pub fn pick_site(w: &mut World, scored: &[(usize, f64)], min: f64) -> Option<usize>`:
  among sites scoring above `min`, a weighted draw with weights
  `exp((score − best) / choice_spread)`, one `rand` call; `None` if none.
- **Divide** (in `eat`, after eating, for clans still alive, in id order): a clan with
  `pop ≥ split_at` and 2+ round tiles splits its round's tiles in two halves grown from its
  two farthest tiles (the poc's `halve`: each half takes in turn the unassigned tile next to
  it that's nearest its seed; leftovers go to the half whose seed is nearer). The clan keeps
  the half with home, in its round order. The daughter takes the other half, half the
  people and half the store; her home is `middle` of her half; her round is built from her
  home by adding her other tiles in order of distance from it (ties by index), each at its
  `insertion`; tiles more than `territory_radius` from her home or that no leg reaches are
  freed. Her `since` is her mother's, `fed` true. People and store are conserved.
- **Split**: a clan with `pop ≥ split_at` and one round tile, or a starving clan with
  `pop ≥ hungry_split`, sends a daughter of `min(pop, split_at) / 2` people with that
  share of the store to a site within `scout_range` steps whose `site_value` for her is at
  least `settle_fed` (`pick_site` with `min = settle_fed`, scoring each site by its value).
  No site, no daughter. Her round is `[site]`, `since` 0. The mother's `hungry` resets.
- Arrivals are the same turn: a turn is a year or more and these walks are a few days.
- New clans take ids from `next_id`.

`TURN` becomes `regrow`, `move` (`adjust`), `harvest`, `eat` (with divide and split).

The CLI's `report` line gains the mean round length (tiles) and mean round cost.

## Tests

In `core/tests/rounds.rs` and `core/tests/clans.rs`, on hand-built worlds (set terrain,
stock and rounds directly) unless said:

- `insertion` on a round of four tiles in a ring around home: a tile next to two consecutive
  round tiles goes between them, and the extra cost is its move cost; a one-tile round's
  extra is there and back.
- Worth working: a hungry clan next to a tile with no food doesn't add it; next to a tile
  with food it lacks, it adds it.
- Dropping: a fed clan with a forest tile that adds nothing it needs drops it, and its
  round's cost falls.
- Starving swap: a hungry clan whose round already reaches the limit swaps its worst tile
  for a better one, one tile a turn.
- Home follows the middle: home moves one tile a turn towards the middle and the round still
  starts there.
- Moving on: a clan starving `patience` turns next to a much better free site moves there and
  frees its old round.
- Divide: a clan at `split_at` with several tiles becomes two clans with disjoint rounds,
  the same people and the same store in total.
- Split: a starving clan of `hungry_split` with a good free site in reach sends a daughter
  there; with none in reach, it doesn't.
- Over 300 turns at radius 16 on seeds 1 to 3, checked every turn: rounds have distinct tiles
  and are pairwise disjoint; a tile's `claim` is set exactly when it's on that clan's round;
  `round[0] == home`; every round tile is within 2 steps of home; each clan's round differs
  from last turn's by at most one added and one dropped tile, apart from moving on and
  dividing.
- Spread: at 300 turns on seed 1, radius 16, there are more than 10 clans.
- `same_seed_same_world` and `replaying_a_save_rebuilds_the_world` still pass.

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` in
`engine/`, and `./build-web.sh`. Check speed with
`cargo run --release -p sansfiat -- bench 1 2 3 --turns 300`; report the ms a turn in your
summary.

## Out of scope for this step

- Trails, taming, hearths, markets (including any pull towards markets in site choice), the
  end of the pre-game (build-3).
- The web map, CLI stats and sweep (build-4).
- From the piece: good and bad years, materials and crafts, civs, a new map generator,
  hidden trail costs, traders.
