# Build 1: terrain, sizes and the base generator

First of five builds for #31, "The map and the page". Read `spec.md` here first (decisions 1–6,
9, 10 are yours), then `docs/design/map.md` and `merge4/src/mapgen.js` + `merge4/src/data.js`
(`MAP`, `FEATURES`, `TRAILS`). merge4 is frozen: read only. Work in `engine/`; read
`engine/CLAUDE.md`. Checks, all in `engine/`: `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `./build-web.sh`.

This step builds everything in the generator **except rivers, floodplains, deltas and closed
country** (builds 2 and 3). After it the pre-game runs on the new map.

## Before you change the generator

- `core/tests/common/mod.rs`'s `flat()` builds `World::new` (radius 6, seed 1), then overwrites
  terrain and food. Print `w.rng` and `w.next_id` from `flat()` now, and once the new generator
  is in, pin both in `flat()` to those values, so every test on a hand-built world keeps its
  random draws and clan ids. `flat()` also resets every new tile field (`river`, `flow`, `lake`,
  `flood`, `zone`, `pass`, `trail_cost`, `span`) to its zero, so no river or hidden cost leaks
  into a hand-built world. `island()` and `flat()` otherwise stay as they are.
- `cargo test` runs the engine unoptimised and is slow (a 300-turn small run is ~26 s; release
  ~2 s). Time `cargo test` first, then set `[profile.dev] opt-level = 3` in `engine/Cargo.toml`
  (debug assertions stay on) and time it again; note both in your result. Later builds run
  whole pre-games on medium and large maps in tests and rely on this.

## What to build

1. **`data.rs`**
   - Terrains `MOUNTAIN = 5` (name "mountain", `land: false`, `move_cost: None`, no food),
     `DESERT = 6` (land, cost 1, no food) and `MARSH = 7` (land, cost 3.0, food roughly
     game 2, grain 2, fish 4, shellfish 3: wet and rich; build-2 uses it). Don't move the
     existing indices. Pick colours for the page (build-5 uses them). Remove `REGION_TERRAIN`
     and `REGIONS` (placeholder only) once nothing needs them.
   - `SIZES: [(&str, i32); 3] = [("small", 16), ("medium", 23), ("large", 33)]` and
     `pub fn size_radius(name) -> Option<i32>`. `SMALL_TILES = 817`.
   - A `MAP` rules struct (starting values are per 817 tiles, merge4's ×817/1232): `ridges`
     3 (floor 1), `ridge_length` (10, 24), `pass_chance` 0.06, `flank_hills` 0.4,
     `inlets` 3, `inland_seas` 1, `lakes` 3 (a lake is 1–3 tiles, an inland sea 5–10 walked
     blobs as merge4's), `desert_share` 0.2, `shadow_reach` 5, and the hidden-cost tables.
     `fn scaled(per_small: usize, tiles: usize) -> usize` rounds `per_small × tiles / 817`.
     Later builds append their own rules here.
2. **`Tile` fields** (decision 6): add all of `river: u32`, `flow: Option<usize>`,
   `lake: bool`, `flood: bool`, `zone: Zone`, `pass: bool`, `trail_cost: Option<f64>`,
   `span: Option<u8>`, with `Zone { Open, Valley, Sealed, Neck }` (derive the serde traits,
   default `Open`). Only `lake`, `trail_cost` and `span` get real values in this step; the rest
   are initialised to their zero (`0`, `None`, `false`, `Open`) and filled by builds 2–3.
   Fix every place that builds a `Tile`.
3. **`map::generate(radius, rng) -> Vec<Tile>`**, rewritten, and it must work at any radius
   from 2 up (the tests build radius 6 worlds): features that need room (`edge ≥ 5`, inland seas,
   lakes) are simply skipped when there is none, never a panic. In this order. Use the hex
   helpers (`Hex::within`, `neighbors`, `distance`, `DIRS`) and `map::index`:
   1. Per tile `h`, `m`, `y` from the main rng (draw them in that order per tile), three
      smoothing passes (`(2·self + Σ neighbours) / (2 + n)`), normalise each to 0..1.
      `edge = radius − distance`.
   2. Terrain from `h`/`m` as decision 3; the rim is sea.
   3. Water: inlets cut in from the coast (merge4's `carveWater`: random-walk paths of 6–15
      tiles from a coastal sea tile with `edge ≥ 2`), inland seas (blobs at `edge ≥ 5`), then
      lakes: 1–3 tile blobs on land at `edge ≥ 5`. Counts scale with area.
   4. Ridges: merge4's `raiseRidges` (random-walk chains of mountain, hills on the flanks
      with `flank_hills`, hills instead of mountain at a pass with `pass_chance`; ridges
      don't overwrite sea).
   5. Rain shadow and deserts (decision 9): draw one wind direction `d` (an index into `DIRS`)
      from the rng; the wind comes from that side. For each plains tile count mountain tiles
      at `tile + k·DIRS[d]` for `k = 1..=shadow_reach` (the tiles upwind of it; cap the count
      at 3). Score = `m − 0.25 × shadow` (tune
      the weight so deserts visibly sit behind ridges); the lowest-scoring plains become
      desert until `desert_share` of the land (not sea) is desert. Deserts only come from
      plains, as merge4.
   6. Coast: a plains, forest or desert land tile with a `SEA` neighbour becomes `COAST`.
   7. `lake`: flood-fill the `SEA` tiles from the rim; sea tiles not reached are lakes/inland
      seas (`lake = true`).
   8. Fertility `y` as the placeholder does it (patchy, `libm::pow(t, 1.5)`, rounded to 1
      decimal, range `FERTILITY`) and caps = terrain food × fertility rounded to 1 decimal;
      `stock = cap`, `wild_grain = cap[GRAIN]`. (Build-2 adds the river, floodplain and delta
      adjustments; leave a clear single place for them: a function `finish_caps(&mut tiles)`
      that computes caps from terrain + flags + fertility, which build 2 extends.)
   9. Hidden trail costs (decision 10) from the sub-stream.
4. **Seeding and landmasses** (`clans.rs` `seed`, `map::landmasses`): no logic change should
   be needed because mountains aren't land (build-3 adds the sealed-zone exception to both of
   `seed`'s loops). Check `seed` still gives each landmass of
   `min_landmass`+ tiles a clan, and that `landmasses` is still deterministic.
5. **`sansfiat map [--seed N] [--size small|medium|large | --radius N]`**: prints the map as
   ASCII, one char per tile, rows offset by half a tile per row (pointy-top), a legend, and
   counts (tiles, land, water, lakes, mountains, hills, deserts, forests, and the number of
   landmasses). Characters: `~` sea, `=` lake, `,` coast, `.` plains, `f` forest, `h` hills,
   `^` mountain, `:` desert, `%` marsh. Later builds add `r` river, `z` flood etc.; leave the
   function easy to extend. This is how the later builds (and you) look at a map.
6. Keep `CLI --radius` working, and add `--size` to `run`, `sweep`, `bench` and `digest`
   (a size name sets the radius).

## Tests (in `core/tests/map.rs`)

- Every size × seeds 1..=20: the rim is all sea; land is between 35% and 75% of tiles (say what
  you found and pick the bounds from it, but not wider than 25%..85%); tile count is
  `1 + 3r(r+1)`; the three sizes' areas are each 1.9×–2.1× the last.
- Every size × 10 seeds: ridges exist (mountain tiles ≥ `scaled(…)` × something sane), hills,
  forest, plains, desert and coast are all present; inlets/seas/lakes counts: at least one lake
  tile on 10 seeds of each size and `lake` is set exactly on sea tiles not connected to the rim.
- Deserts: share of land ≤ `desert_share` (+ rounding); over 20 seeds the mean shadow count of
  desert tiles is clearly above that of non-desert plains (state the numbers in your result).
- Hidden costs: every forest tile has `trail_cost` 1 or 2; hills 1 or 2; mountains `Some(3)`
  with a span or `None`/`None`; others `None`. Over a size-large map the fraction of crossable
  mountains is within 0.1–0.35 and hills at 1 are within 0.2–0.5 (use pooled seeds if one map
  has too few). A test builds the same map with the sub-stream's seed irrelevant: terrain and
  fertility are identical whatever the hidden rolls give (set a different xor constant in a
  test-only path, or assert by generating twice and comparing tiles with the hidden fields
  blanked).
- `step_cost` doesn't read `trail_cost` or `span` (set them to odd values on a forest tile
  and the cost doesn't change; worn trails still bring forest and hills to `TRAIL.cost`).
- Determinism: same seed, same tiles; `World::replay` of a save still equals the world.
- Existing tests: fix those the new map breaks. Tests on hand-built worlds (`flat`, `island`)
  shouldn't move. Tests naming seeds on generated maps may: see spec.md "What the later steps
  need to know" for what's allowed, and list every changed test in your result.

## Also check and report

- The pre-game on the new map: `cargo run --release -p sansfiat -- sweep --size small` for
  seeds 1..=10. It may end later or by the limit more often now; that is information, not a
  failure of this step (build-4 tunes and tops up). Put the table in your result. If clans
  starve everywhere or no market forms on most seeds, look at why (a map with too little
  grain land? mountain walls?) and fix the generator, not the clan rules.
- `cargo run --release -p sansfiat -- bench --size large 1` build time of the world
  (should be tens of ms).

Commit when checks pass: "#31 build-1: new terrain, map sizes and the base generator".
