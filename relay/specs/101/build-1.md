# #101 build-1 (easel): Standing, the orders counter, the judge and its bands

You build the measuring half of the easel: Standing, a counter of orders, `core/src/judge.rs` that
scores a game turn by turn, and `sansfiat sweep --judge` with its bands in `engine/judge.toml`, run by
CI. `spec.md` in this folder items 1–12 are the full rules; this brief says where they go and what done
is. Read `spec.md` and `probe.md` first.

## Rules that hold everything

- **No digest moves.** Every field you add to `World` is `#[serde(skip)]`. `SAVE_VERSION` stays 2.
  `bash relay/specs/100/baseline/compare.sh` prints six `ok` lines before you start and after you
  finish; that is your proof the game didn't change.
- `sweep --after` prints exactly what it prints now (compare.sh diffs it). `report::line` is unchanged.
- Numbers go in `core/src/data.rs` with a `note`: a `JUDGE` const (`lead_margin: 0.005`, `sample: 10`),
  in the style of the others there. The bands are not numbers of the game: they go in `judge.toml`.

## What to build, and where

1. **`core/src/stats.rs`**: `pub fn standing(w: &World) -> BTreeMap<u32, f64>` (spec item 1: clans' and
   units' people of the civ over all clans' and units' people; every civ present; 0 when nobody), and
   `Summary.standing` filled from it.
2. **`core/src/world.rs` / `orders.rs`**: `World.orders` (spec item 2), built empty in `World::new`,
   `#[serde(skip)]`; `Order::kind(&self) -> &'static str` giving the wire name; `World::order` counts on
   `Ok`; `act` counts `setKeep` and `setGift`.
3. **`core/src/judge.rs`** (new; one `pub mod` line in `lib.rs`): `Watch` (spec items 3–6): `new(&World)`
   at the pick, `turn(&World)` after each turn; it keeps every civ's Standing per turn and exposes what
   the sweep and build-2's `sansfiat judge` read: Standing at a turn offset, the leader per turn and
   `lead_changes`, the spread at an offset and its min and max, `swings`, `clans_changed`,
   `people_changed`, `land_changed`. Give it a serialisable `Scores` (or similar) for one game, grouped
   as spec item 8 groups them, so build-2 can print the same for a played game. Keep it free of the
   CLI: no printing, no threads.
4. **`cli/src/judge.rs`** (new): `sweep --judge [--bands FILE] [--json]`, dispatched from
   `cli/src/sweep.rs`'s flag parsing as `--after` is (USAGE gains its line). The runs of spec item 7,
   reusing `retro::retro_maps` and the shape of `retro::after_run` (move the shared run-to-the-pick
   into one helper rather than copying it; keep `after_run`'s output identical). The report of item 8,
   text and `--json`. The bands of items 10–11: parse with the `toml` crate (add it to the workspace
   dependencies and `cli/Cargo.toml`), check every band, print, exit 1 on a fail. Panics caught per
   map (item 7); a panic's map and message are printed.
5. **`engine/judge.toml`** (new): the three health fail bands of item 11, each with its `note`, and a
   comment at the top saying what a band is and that a piece moving one says so in its PR.
6. **`engine/check.sh`**: `cargo run -p sansfiat -- sweep --judge` after `cargo test`.
7. **`engine/CLAUDE.md`**: rows for `core/src/judge.rs`, `cli/src/judge.rs` and `judge.toml`, and the
   command line beside `sweep --after`.

## Tests (`core/tests/judge.rs`, `core/tests/standing.rs` or one file; CLI tests as the existing ones)

- Standing: summed over civs it equals the civs' share; a civ with no people is 0; raising a camp
  (an `Order::Raise`) leaves the civ's Standing as it was to within float error.
- The counter: after a turn where the AI moves, its civ's `move` count is one more; a player's
  `Action::Order` counts under its kind; `serde_json::to_string(&world)` has no `orders` key.
- `Watch`: two near-equal civs swapping by less than `lead_margin` count no change, and by more count
  one; a clan joining a civ counts under `clans_changed` with its people; two runs of equal worlds give
  equal scores. Built worlds are fine here (`tests/common`): these test the rules.
- Bands: a value in, out of warn, out of fail; a band naming a metric that isn't there fails.

## Done

- `./check.sh` passes, the judge included; say in your summary how long the judge took in it.
- `compare.sh`: six `ok`.
- `cargo run --release -p sansfiat -- sweep --judge` and `--json` output saved to
  `relay/specs/101/build-1-judge.txt` and `build-1-judge.json`: the per-map lines, the totals, the
  bands. Your summary names them. Build-3 sets the bands from these numbers, so say in the summary
  anything in them that surprised you (a measure always 0, one that varies between maps by orders of
  magnitude).

Build-2 relies on: `stats::standing`, `Summary.standing`, `World.orders`, `Watch` and its scores type
usable on a played game (a led civ among AI civs), and the report printer in `cli/src/judge.rs` being
callable for one game.
