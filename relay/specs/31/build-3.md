# Build 3: closed country

Third of five builds for #31. Read `spec.md` (decisions 6, 11, 12), `docs/design/map.md`
("Closed country"), and the earlier steps' results. Work in `engine/`; checks: `cargo test`,
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `./build-web.sh`.

The map isn't all open. Ridges and water close off some regions so that getting in is a
choice. This step builds three kinds, tags them, and guarantees the open land can still grow
three centres.

## The kinds (`map.rs`, a function `close_country`)

It runs after the base land, ridges and deserts and **before** coast, `lake` marking and
rivers (so rivers and coasts see the finished terrain). Per-817-tile counts in `MAP`:
`valleys` 2 (floor 1), `sealed` 1 (floor 1), `necks` 2 (floor 1). "A few of each, more on
bigger maps."

1. **Valley behind a pass** (`Zone::Valley`). Choose a centre `c` on land at `edge ≥ ρ + 4` (the
   wall, at ρ+1, stays 3 steps off the rim) with at
   least 7 land tiles' worth of interior, radius ρ = 2..=3 (interior = tiles within ρ of c, wall
   = tiles at distance ρ+1). Interior tiles stay as they are (but sea/mountain in the interior
   becomes plains/forest by the underlying noise: simply keep land and turn sea tiles to
   plains). Wall tiles that are not sea become `MOUNTAIN`; wall tiles that are sea stay sea
   (water is a wall too). Open **one or two** wall tiles as passes: `HILLS` with `pass = true`
   (a pass tile is hills, so it costs 2 to cross). Passes must be on land (a wall tile whose
   outside neighbour is land and not sea, so the pass actually leads somewhere). Tag interior
   and wall (and passes) `zone = Valley`.
2. **Sealed valley** (`Zone::Sealed`). Same, but no pass: the wall is mountain and water all
   round. Roll hidden costs (build-1) as usual afterwards, then **force at least one wall
   mountain to `span = Some(1)`, `trail_cost = Some(3.0)`** (a scout can bridge it). Sealed
   interiors are ≥ 7 tiles, with food (plains/forest), so it's a real prize. Interior must be
   land; the hex distance between a sealed valley and the nearest valley or other sealed
   valley ≥ 3.
3. **Peninsula and neck** (`Zone::Neck`). A land blob attached to the rest by a strip 1–2
   tiles wide. Cut water (tiles → `SEA`) in a ring around a blob of 8–20 land tiles reachable
   from the coast, leaving a strip of 1–2 tiles. Tag the blob and the neck `Neck`. (Cutting the
   sea by hex distance from a centre on the coast, leaving a gap of one or two tiles in the
   direction of the mainland, is enough.) If cutting the ring would leave the mainland itself
   disconnected the attempt fails and is skipped.

All three kinds are tagged on the tiles; the wall tiles carry the zone too (so the page can
outline them) except that the sea stays sea but zone isn't set on tiles that were outside.
Zones don't overlap; keep ≥ 3 steps of clearance between features and don't build any within 3
of the rim. On small radii (< 16) build none (see `spec.md` decision 12). A small map has
room for few: the 2 valleys, 1 sealed and 2 necks are a target, and a feature that can't be
placed after a handful of random tries is skipped.

## Never where a centre has to be: `map::open_sites`

- `open_sites(tiles, radius) -> Vec<usize>`: greedily pick sites in **open** land (zone
  `Open`, passable i.e. `TERRAIN.land`, and in the same connected passable region as the
  largest such region), a site being a tile with ≥ 12 land tiles having grain or fruit within 3
  steps, mean fertility ≥ 0.8, whose distance to every earlier site is ≥ `PREGAME.centre_spacing`.
  Take the best sites first (most such tiles, then lowest index). Return as many as found up to
  3.
- Generation retries: if `open_sites` finds fewer than 3, re-roll the closed features (not the
  base land) up to 8 attempts, then drop all closed features (the map is then all open). If even
  then the base land can't hold three sites, re-roll the whole map from the continuing rng (up
  to 8 times; the world's rng is whatever it is afterwards, deterministic). `open_sites` needs
  fertility, so one attempt is: closed features, then coast, `lake`, rivers and `finish_caps`
  (builds 1 and 2) on a copy of the base land, then `open_sites`; keep a snapshot of the base
  land to start each attempt from. Report how often each fallback fired.
- At radius < 16 skip all of this (no closed country, no check).

## What closed country means for the rest of the engine

- **Seeding** (`clans.rs`): never seed a clan in a `Sealed` zone, in either of `seed`'s loops
  (the per-landmass one: a radius-3 sealed interior has 37 tiles, over `min_landmass`; and the
  random free-land one). Daughters and moves can't
  enter anyway (walls are mountain and sea); assert that in a test.
- **Pass tiles**: nothing special in movement; `pass` is for the page and tests.
- **Peninsulas** are ordinary land: they may be seeded, and may be settled by clans from the
  mainland through the neck.
- `pregame::land_cover` already counts land connected to the centres only, so sealed valleys
  don't hurt cover.

## Tests (`core/tests/closed.rs`)

- Over every size (radius 16 and up) × 15 seeds: pooled over the seeds, each size has at least
  `15 × scaled(valleys) × 0.7` valleys (one valley is one connected `Valley` zone), and sealed
  valleys and necks likewise; at least 12 of the 15 maps have each kind; larger sizes have more
  in total than smaller; zones don't overlap. (A map can lose its closed features to the retry
  fallback, so a per-map floor would be flaky: report how many maps lack each kind.)
- Valley: its wall has exactly one or two pass tiles, each `HILLS` + `pass`; every other wall
  tile is mountain or sea; from the open outside you can reach the interior through a pass over
  passable land and **can't** reach it without (BFS over passable tiles that excludes the pass
  tiles never enters the interior).
- Sealed valley: no pass tile; BFS over passable tiles from outside never enters the interior
  (the interior is a landmass of its own); at least one wall mountain has `span: Some(1)`;
  interior ≥ 7 land tiles with some food.
- Neck: the blob connects to the mainland through a strip whose narrowest cut is 1 or 2 tiles
  (check by removing the neck tiles: the blob disconnects).
- `open_sites` finds 3 sites for every size and seeds 1..=40 (large maps too), in open land
  only, ≥ `centre_spacing` apart. Report how often a re-roll was needed and if the "drop all
  closed features" fallback ever fired (it shouldn't on these seeds).
- A full pre-game run (≤ 500 turns) for 5 seeds on small: no clan ever holds a tile in a
  sealed zone (check every turn: round tiles and homes).
- Earlier tests green; list any you changed and why.

## Report

ASCII of a small and a large map showing each kind (add to the `map` command: `V` valley
interior, `S` sealed interior, `N` neck, `P` pass, wall tiles keep their terrain char), counts
per size over seeds 1..=10. Run `sweep --size small` for seeds 1..=10 and put the table in
your result.

Commit when checks pass: "#31 build-3: closed country and the open-land guarantee".
