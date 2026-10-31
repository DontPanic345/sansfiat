# Build 1 (ford): the rest, part 1

Piece #100, "The engine's shape for many units and players". Read `relay/specs/100/spec.md`
first, above all "The digests": the world's JSON must not change by a byte, and the piece is proved
by digests. This step's items are spec items 15, 16, 17 and the `--state` flag of item 2, plus the
Basket helpers of item 9. `engine/CLAUDE.md` has the commands. **No rule changes.**

## First, before any edit

1. `cd engine && cargo build --release -p sansfiat`, then `target/release/sansfiat bench 1 2 3`
   three times; write the best ms-a-turn of each seed into `relay/specs/100/bench.md` ("before").
   The review holds the piece to within 10% of it.
2. Add the `--state` flag to `digest` in `cli/src/main.rs` (on the code as it is): it prints the same
   `<turn> <hash>` lines, hashing `serde_json::to_value(&world)` with its `log` key removed
   (`as_object_mut().remove("log")`), then `to_string`, FNV-1a 64 exactly as `stats::digest` does.
   Put the hashing in `stats.rs` as `pub fn state_digest(w: &World) -> String` and have `stats::digest`
   share the FNV loop. Add it to USAGE and the command list in `engine/CLAUDE.md`.
3. Prove it: `sansfiat digest --seed 2 --radius 16 --turns 400 --lead --state` equals
   `relay/specs/100/baseline/seed2-r16-t400-lead.state.txt` (use `diff`), and `digest` without the
   flag equals `seed2-r16-t400-lead.txt`. Add a test beside the pinned digests in `cli/tests/save.rs`
   only for the state digest of turn 303 (`46e67d48ffcbe520`) — the version bump belongs to build-4.

## The work

- **`core/src/events.rs`** (spec item 15): move `Event`, `EventKind`, `Event::text`, `Event::json`
  and `civ_name` out of `world.rs`. Add `Event::new(kind, tile)` and the chainable setters, and
  `World::emit(&mut self, e: Event)` (stamps `turn`, pushes). Replace the seven `World` event
  constructors (`event`, `civ_amount_event`, `civ_event`, `civ_clan_event`, `unit_event`,
  `unit_event_other`, and `scout_event`'s follow-up poke at `events.last_mut()`) at all 28-odd call
  sites in `core/src`, `web`, `cli` and `core/tests`. `scout_event` (in `units.rs` today) may
  write the cost with `.cost(c)` before emitting. `lib.rs` re-exports `Event`/`EventKind` from
  `events`; `World`'s `alert_key`/`flag_alerts` stay in `world.rs`. The JSON of an event, field order
  included, does not change (`core/tests/events.rs` and `alerts.rs` hold it).
- **Basket helpers** (item 9): `take`, `add`, `total` and `clans::sum` become free functions in
  `core/src/market.rs`, once, with the same bodies; delete the private copies in `units.rs`
  (`take`, `add`, `total`) and `clans.rs` (`sum`), and fix their callers (`units.rs`, `markets.rs`,
  `leaning.rs`, `clans.rs`). Leave `units.rs` otherwise as it is: build-2 splits it.
- **One tile-distance helper** (item 16): `land::steps(w, a, b)`; delete the private copies and
  the inline `hex().distance` stand-ins.
- **Colliding names** (item 17), JSON unchanged: `land::hold` → `land::tend` (and the `TURN` step's
  name `"hold"` → `"tend"`); `Tile.held`/`unheld` → `claim_run`/`free_run` with
  `#[serde(rename = "held")]`/`"unheld"`; `map::Zone` → `map::Country` and `Tile.zone` → `country`
  with `#[serde(rename = "zone")]` (the variants, and `layout()`'s `zone` column, stay); `clans::divide`
  → `divide_land`. `units::walk` is build-3's. Update tests, `cli`, `web` and the doc comments that
  name them.

## Done

`./check.sh` green; `bash relay/specs/100/baseline/compare.sh` prints six `ok` lines (the four watching runs,
the led run's `--state`, and the retro sweep) — say in your summary that it did; `bench.md` holds the
"before" numbers.

## What build-2 and later rely on

`World::emit(Event::new(kind, tile)….)` is how every event is made. `market::{take, add, total, sum}`
are the basket helpers. `land::steps` is the distance. `sansfiat digest --state` and `stats::state_digest`
exist.
