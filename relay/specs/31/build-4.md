# Build 4: the top-up, events, CLI output and the sweep

Fourth of five builds for #31. Read `spec.md` (decisions 13–15, 17), `docs/design/map.md`
("Always three centres", "On the page" for the events), `docs/design/clans.md` ("The
pre-game ends"), and the earlier steps' results. Work in `engine/`; checks: `cargo test`,
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `./build-web.sh`,
`./cross-build.sh`.

## 1. The top-up (`pregame.rs`)

In `check_end`, when `w.turn + 1 >= PREGAME.max_turns` and fewer than `players` centres
exist, top up to exactly `players` as decision 13 says:

- Start from the centres the greedy `centres()` already took.
- Tier (a): the other markets, best first (traders desc, founded asc, id asc), regardless of
  age, traders or hearths.
- Tier (b): the most tended land: land tiles with `tame > 0` and grain, `tame` desc then
  fertility desc then index asc; raise a market on the tile.
- Tier (c): any land tile with grain, fertility desc then index asc; raise a market there.
- A candidate is skipped unless it is at least `centre_spacing` steps from every site already
  taken, is passable land, is in zone `Open` or `Valley` or `Neck` (never `Sealed`), and is in
  the same passable region as the first centre (or, with no centres, as the most populated
  clan's home).
- Raising a market: a normal `Market` (next id, `tile`, `founded = w.turn`, zero traders and
  volume, prices all 1.0, `goods` empty, activity 0 — copy how `markets::form` builds one; reuse
  that code through a helper, don't copy it) and the tile isn't already a market's.
- `End` gains `topped_up: Vec<u32>` (ids added, in order, a subset of `centres`) and still
  `by_limit`. `by_limit` stays true for any end by the limit. Update users of `End` (stats,
  web, CLI).
- Markets whose id is in `w.end.centres` never fade (`markets::form_and_fade`).
- If the map somehow offers fewer than `players` sites even in tier (c) with spacing, relax
  nothing: end with what exists and keep `centres.len() < players`; a test (below) shows this
  doesn't happen, and the sweep reports it if it does.
- Cover isn't needed at the limit.

Tests (`core/tests/pregame.rs`): on `flat()` worlds: (1) zero centres at the limit → three
sites from the best three markets that are ≥ spacing apart (give five markets, two close
together); (2) two centres and no more markets → tier (b) picks the most tended tile and
raises a market; (3) no tame land → tier (c) picks the most fertile grain tiles; (4) the
mixed order: one tier-(a) market, then (b), then (c); (5) candidate in a sealed zone or
closer than spacing is skipped; (6) one centre by itself and 2 topped up, `topped_up` has
exactly the two new ids; (7) topped-up markets survive turns after the end. And: for sizes
small/medium/large on seeds 1..=5 each, `w.run` to the end gives `end.centres.len() == 3`
with the sites ≥ `centre_spacing` apart.

## 2. Events (`world.rs`, the steps)

Decision 14. `Event { turn: u32, kind: EventKind, tile: usize, clan: Option<u32>, market:
Option<u32>, other: Option<u32> }` (`other` is the daughter on a division), serialisable;
`World.events: Vec<Event>` is `#[serde(skip)]` (so the state JSON and the digest don't carry
it) and starts empty in `World::new`. Push in the step where it happens:

| Event | Where | Tile |
|---|---|---|
| `ClanDivided` | `clans.rs` where a clan divides or sends a daughter | the daughter's home |
| `ClanStarving` | `eat`, when `starving` flips from false to true | the clan's home |
| `ClanMoved` | the move-on in `adjust` | its new home |
| `MarketFormed` / `MarketFaded` | `markets::form_and_fade` | the market's tile |
| `HearthTamed` | `land::hold`, when a tile's `tame` first reaches `tame_turns` | the tile |
| `CentreFormed` | `pregame::check_end`: when a market newly appears among the current `centres()` (keep the previous turn's set in a `#[serde(skip)]` field `centres_seen`) and again for each top-up | the market's tile |
| `PregameEnded` | `check_end` when it sets `end` | the first centre's tile |

`Event::text(&self, w: &World) -> String`: the one wording, e.g. "Clan 4 divides; clan 9
settles at (3, -2)", "Clan 4 is starving", "Clan 4 moves on to (2, 1)", "A market forms at
(3, -2)", "Market 2 fades", "A hearth is tamed at (1, 0)", "Market 2 becomes a centre", "The
pre-game ends: 3 centres, 2 topped up". (Faded markets are gone from `w`: keep what `text` needs
in the event or phrase without it.) The events are cleared never (the whole game's list, in
order); per step order is the TURN order, deterministic.

Tests (`core/tests/events.rs`): each kind fires in a real run (find a seed/size where it does
in ≤ 500 turns on small; several seeds pooled is fine) and in a constructed case where easy
(`hold` hearth; `check_end` top-up gives `CentreFormed` ×N then `PregameEnded`); events are
in turn order; **a replay of a save gives identical events** to the original world's (compare
the vectors); the digest of a world doesn't change when events are cleared (skip is real);
event `tile`s are valid indices.

## 3. CLI

- `run` prints each event as it happens as `turn  text` (`    42  Clan 4 is starving`) between
  the `--every` summary lines, unless `--no-events`. With `--json` events aren't printed (the
  JSON is the world); add `--events-json` to print `{"events":[…]}` instead for scripts.
  `replay` prints the events the same way.
- The end report says whether the top-up ran: `ended turn 500 by the turn limit, 1 centre
  topped up (market 17)`.
- `map`, `sweep`, `bench`, `digest` take `--size`.
- Document the new flags in `USAGE` and in `engine/CLAUDE.md` (Layout table: map.rs, pregame,
  the events; commands list).

## 4. The sweep

`sansfiat sweep [seed ...] [--size small|medium|large|all] [--turns N]` (default size small,
seeds 1..=10; `all` runs each size). Per run print: size, seed, end turn, how (`centres`,
`limit`, `limit+topup`), centres, topped up count, cover, clans, people, markets, hearths,
closed-country counts (valleys / sealed / necks), ms a turn. After each size print:
`N of M ended on their own, K needed the top-up, J ended on the limit with three centres
already (cover short), median end turn`. Use `std::thread` to run seeds in parallel if it
makes `all` tolerable, keeping output in seed order (the engine is deterministic, so this is
safe; wasm doesn't use the CLI). Large maps are slower: measure first (`bench`).

Run it in the background with output to a file; the session is stopped at 90 minutes. A first
run: seeds 1..=10, all sizes. **Write the table to `relay/specs/31/evidence.md`** (commit it)
with the date, commit hash, and these answers in prose: how often the pre-game ends on its
own per size; how often it needs the top-up; what the top-up picks (which tier, how often);
and anything odd (a seed where cover is short, a map where fewer than three sites exist).

## 5. Tuning, only if the evidence asks for it

If on medium or large most seeds need the top-up because people can't spread in 500 turns,
try (decision 17) scaling the clan seeds with area, `seeds × tiles / 817`, rounded, and rerun
the sweep. Keep it only if it clearly helps and say before/after in `evidence.md`. Do not
change other rules or move the pre-game's turn limit. If small maps mostly need the top-up,
find why (map land? unreachable grain? spacing?) and report; adjust the generator's numbers
(`MAP`) if it is the map, and say what you changed.

## 6. Cross-build

`cross-build.sh`: add a medium map pair (`"3 23"`) with a shorter run (say 150 turns) so the
check stays fast; larger digests of a new map across native and wasm catch float drift in the
generator. Run `./build-web.sh && ./cross-build.sh` and put the result in yours.

## 7. Web crate

`Sim` gains `events(from: usize) -> String` (JSON array of events from index `from`, each with
a `text` field from `Event::text`, so the page doesn't re-word) and a free function
`sizes() -> String` (`[{"name":"small","radius":16},…]`). `state()` stays the whole world.
`terrain()` already lists terrains; add `water`/`passable` flags only if the page needs
them (build-5 may ask for them: keep the JSON additive).

Commit when checks pass: "#31 build-4: the top-up, events, CLI output and the sweep".
