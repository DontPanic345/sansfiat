# build-4: Show it: stats, sweep, web map, tuning

Piece #13 is a proof: does a sparse world of clans, walking their rounds and paying real
labour to reach their food, spread across the land and form centres with markets around
fertile areas on its own, with no rule placing them? The design is
`docs/design/clans.md`; the whole spec is `relay/specs/13/spec.md`. builds 1 to 3 built
the pre-game in the engine. This step makes it visible enough to answer the question: CLI
stats and a sweep over seeds, the web map, and one tuning pass over the starting numbers.

Where `docs/design/clans.md` now answers one of the spec's design questions differently from
this brief, follow the design file.

Read `engine/CLAUDE.md`, then skim `core/src/{data,world,clans,markets,pregame}.rs` for the
state: `World { config, turn, rng, tiles, log, clans, markets, next_id, end }`;
`Tile { q, r, terrain, fertility, cap, stock, wild_grain, tame, claim, held, unheld, trail }`;
`Clan { id, home, round, pop, store, harvest, basket, fed_ratio, fed, starving, hungry,
since, walk, market, trip }`; `Market { id, tile, founded, prices, activity, volume, traders }`;
`End { turn, centres, cover, by_limit }`; `round::territory`, `land::is_hearth`,
`markets::reach`, `pregame::{centres, land_cover}`.

## What to build

### `core/src/stats.rs` (new)

- `pub struct Summary { turn, clans, people, mean_fed, starving, held, territory, round_len,
  round_cost, trails, hearths, markets, trading, centres: Vec<u32>, cover, end: Option<End> }`
  (`Serialize`), and `pub fn summary(w: &World) -> Summary`: counts read off the world;
  `territory` is tiles in some territory; `round_len` and `round_cost` are means over clans;
  `trading` is clans with a market; `centres` is `pregame::centres(w)`; `cover` is
  `land_cover` over them.
- `pub struct CentreReport { market: u32, q, r, traders, age, hearths_in_reach,
  fertility_near: f64, fertility_land: f64 }` and `pub fn centre_report(w, ids) -> Vec<_>`:
  `fertility_near` is the mean fertility of land within 2 hex steps of the market;
  `fertility_land` the mean over all land. Plus the pairwise hex distances between them.
  This is the evidence for "around fertile areas".

### CLI (`cli/src/main.rs`)

- `run [--seed N] [--turns N] [--radius N] [--every N] [--save f] [--json]`: `--turns`
  defaults to `PREGAME.max_turns` and is a cap; the run stops at the turn `end` is set.
  `--every N` prints a summary line every N turns (turn, clans, people, mean fed, held,
  trails, hearths, markets, trading, centres, cover). At the end it prints the summary and
  the centre report (or that the turn limit was reached, with the centres it had).
- `sweep [seed ...] [--radius N] [--turns N]` (seeds default 1..=10): one line a seed
  (seed, ended at turn, by centres or limit, clans, people, markets, hearths, cover, the
  centres' mean `fertility_near / fertility_land`, ms a turn), then a line of totals: how
  many ended by centres, the median end turn, and the mean fertility ratio.
- Keep `replay`, `bench`, `clear`, `goods` working; update `USAGE` and `engine/CLAUDE.md`'s
  command list and layout table for the new commands and modules.

### Web (`web/src/lib.rs`, `www/main.js`, `www/index.html`)

- `Sim::summary() -> String`: the `Summary` as JSON. `terrain()` adds `move_cost`.
  Export `goods()` (the food names) too.
- The radius box defaults to 16. A **Run to end** button runs turns until `end` is set (in
  batches, redrawing between them, so the page stays responsive).
- The map draws, over the terrain:
  - a **fertility view** toggle that shades land by fertility;
  - each clan's **territory** as a faint fill in a colour from its id, and its **round** as a
    closed line through its tiles' centres in order, back to home;
  - **trails** as a mark on the tile (a short tan line or dots);
  - each **clan** as a circle at home, its area by people, outlined red when starving;
  - **hearths** as a small yellow mark;
  - **markets** as a ring with their reach outlined; **centres** picked out (a star or a
    thick gold ring); after the end, the end's centres.
- The side panel: turn, the summary line, and the end when set. Clicking a tile shows its
  terrain, fertility, each food's stock / cap, tame (as a share) and trail; the clan whose
  territory or round it's on (people, fed, store, round length and cost, its market and
  trip); and a market there (age, traders, activity, prices).
- No console errors, from load to the end of a run at radius 16.

### Tuning (`data.rs` and `relay/specs/13/tuning.md`)

Run `sweep 1 2 3 4 5 6 7 8 9 10 --radius 16` in release. If fewer than 6 of the 10 end by
centres, tune: change only the numbers in `data.rs` (the spec lists them under design
question 9 and the caps under 2), a few at a time, with a reason each, and at most four
rounds. Never add a rule or a term that looks at fertility, and don't change how rules work.
Write `relay/specs/13/tuning.md`: the sweep before, each change and why, the sweep after.
If the numbers can't make it end, say what's stopping it there (for example "no hearths by
turn 500", "markets fade before 30 turns") — that's a finding for the review, not a
failure of this step.

## Tests

- `core/tests/stats.rs`: on a world run 100 turns at radius 12, `summary` counts match the
  world (clans, people summed, markets, trails, hearths); `centre_report` gives
  `fertility_land` equal to the mean over land tiles.
- Every existing test passes; tuning mustn't break the hand-built tests (if a test pins a
  number you changed, it should read the constant).
- `./build-web.sh`, then serve `engine/www` (`python3 -m http.server -d www`) and load it in
  headless Chrome if the runner has one (`google-chrome --headless` or a Playwright
  install): no console errors after a run to the end. If no browser is available, say so in
  your summary.

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` in
`engine/`.

## Out of scope for this step

- New rules, or changes to how any rule works; only the numbers move.
- Anything after the end: founding, the boon year, civs, the player.
- A new map generator, traders, years, materials.
