# Build 1 (ford): reasons, alerts and the save version in the engine

Piece #57, "Reading the world". Read `relay/specs/57/spec.md` first: its items 1–11 and 18 and
its "Tests the piece must have" are your exact done. The design is `docs/design/clans.md`,
`docs/design/supply.md` and merge4's "Making it readable" (at tag `prototype/merge4`,
`docs/DESIGN.md`; its `note` is in `src/core.js`, and `git grep "note(" prototype/merge4 -- src`
shows the kind of line it wrote). `engine/CLAUDE.md` has the commands and invariants.

**No rule changes.** Nothing you add may move the world: no new randomness, no change to what any
step does to clans, tiles, markets, civs or units. The proof is the digest test below.

## First, before any edit

1. `cargo run --release -p sansfiat -- digest` (or a short test) for seeds 1, 2, 3 on the small
   map, run to the pick and 20 turns on, watching (`World::run`): note the three digests. They go
   into `the_save_version_pins_the_rules`.
2. `cargo run --release -p sansfiat -- bench 1 2 3` (small): note the time. Write both into
   `relay/specs/57/bench.md`, and the after numbers when you're done. More than 10% slower: make
   notes cheaper (format only when a rule acts, skip work when nothing changed).

## Where things go

- `core/src/data.rs`: `NOTES { kept: 8, shown: 5 }` with doc comments.
- `core/src/world.rs`: `Note`, `World.notes` (`#[serde(skip)]`), a `World::note(id, text)` that
  keeps the rule in spec item 2, dropping gone subjects' notes at the end of `end_turn`;
  `Event.alert` and `Event.amount`; `EventKind::PeopleLeft` and its text; the alert marking in
  `end_turn` (spec items 7–8; keep last turn's candidates in a `#[serde(skip)]` field);
  `SAVE_VERSION`, `Save.version`, and `replay`'s refusal (item 10). Event helpers may grow one
  for `amount`.
- The rules (spec item 4): `clans.rs`, `markets.rs`, `civs.rs`, `leaning.rs`, `units.rs`,
  `pregame.rs`. Each note is one call where the rule already decides. Most rules work on a
  cloned clan or a `get_mut`; borrow so you call `w.note` after the mutable borrow ends.
- `markets.rs` `people` and `merge`: the per-civ sums for `PeopleLeft` (item 9).
- `web/src/lib.rs`: `Sim.notes(id) -> String`, `[{turn, text}]` newest first (empty for an
  unknown id); `rules()` gains `notes_shown`. Events already reach the page through `Event::json`,
  so `alert` and `amount` arrive on their own; say so in `view`'s and `events`' doc comments.
- `cli/src/main.rs`: alerts as `!` in `run`/`replay` text output, `run --why ID` (item 18); the
  USAGE text and `engine/CLAUDE.md`'s command list.
- `www/check.mjs`: `notes()` returns arrays of `{turn, text}` for a clan after some turns; events
  carry a boolean `alert`; `Sim.load` of a save JSON without `version` throws a message naming
  the version.
- `www/feed.js`: add `peopleLeft` to `KINDS` (so the page doesn't break on the new kind); the
  rest of the page is build-2's.
- Tests under `core/tests/`: a new `notes.rs` and `alerts.rs`, and the save tests in `world.rs`
  (or a new `save.rs`), covering every item in the spec's test list but `play-read.mjs`. For the
  camp going hungry, `www/play-camp.mjs` and `core/tests/units.rs` show how a led camp is moved
  out on its land.
- `engine/CLAUDE.md`: the layout table (notes in `world.rs`, alerts) and the invariant that a
  change moving `the_save_version_pins_the_rules` bumps `SAVE_VERSION` with it.

## Done

`./check.sh` green (fmt, clippy, tests, web build, cross-build, check.mjs), the three digests
unchanged, `bench.md` written, and `run --seed 2 --size small --turns 400 --why <a clan id>`
printing a believable story for a clan. Put two or three of those notes in your summary so
build-2 and the review see the voice.

## What build-2 relies on

`Sim.notes(id)`, `rules().notes_shown`, events with `alert`, the `peopleLeft` kind, and the load
error's text.
