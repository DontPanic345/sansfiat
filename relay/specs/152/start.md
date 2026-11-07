# #152 build-1 (easel): the start

A civ starts with its scout and nothing else. This file is the record of that change: what moved, every
assertion changed and why, the judge's totals before and after, the three stories, and what the page
scripts still need.

## What changed

- `data.rs`: `UNIT.start` and `GUARD.start` are 0 (with notes). `UNIT.first` is 3 (the warriors of a first
  army where a script or the judge needs a number). One addition the brief did not name: `GUARD.first` is 3,
  the guards a staged civ has (`StageSpec`, below), since a staged civ used to get the guard founding gave it.
- Founding (`civs::found`) raises the scout alone. `units::raise_at_founding` and
  `units::guard::raise_at_founding` are gone. The scout's pack still comes from the store.
- `units/ai.rs::ai` raises no army; its doc says so (until R). Its claim, guard, post and zone stay.
- `judge.rs`'s `turns_at_pack` reads `UNIT.first` (same number, no score moves). `civs::keep`'s fallback
  for a civ with no camp reads `UNIT.first` (a new camp's pack, as before). `cli/src/retro.rs`'s lever raises
  `UNIT.first`; `kept_camp` is "no camp lost" (`army.lost == 0`).
  `grep -rn "UNIT.start\|GUARD.start" core cli web www` finds only `data.rs`.
- `Stage::army` makes the civ a camp when it has none (`units::make_camp`: an empty camp at the capital, no
  people taken from clans, no event), then places it as before.
- `Stage::from_spec` raises `GUARD.first` guards for each civ it founds.
- The stories: `surrender` raises `UNIT.first` warriors at +0 (a new army at the capital) before its
  switches and move. `skirmish` and `stack` lose the +0 disband in `bank_and_raise` and its doc lines.
  The module doc's store claim is measured (below) and rewritten.
- `Lead` (`script.rs`): at +0, if the led civ has no camp, `Order::Raise { n: UNIT.first, kind: Camp }`.
- `SAVE_VERSION` 7 → 8; the pins in `cli/tests/save.rs` move to the new digests.

## Measured: the store at the pick

On small 2, every civ's store is 10 grain at the pick (the endowment's 20 less the scout's pack of 10). Before
the start moved it was 0: the army's pack of 15 took the rest. So at the pick a civ can fill one warrior's pack,
not three: `Raise` of three warriors is refused with "the store can't fill their packs" (`surrender`'s +0
raise is refused on small 2 for this reason, see the stories below).

## Assertions changed, and why

Rule assertions that changed are the start's own: the founding test asserts the new start (below); the AI
test asserts the AI raises no army. Everything else changed follows a fixture (a helper that raises the army
and guard as founding did, or a number that is the fixture's).

| File | Assertion | Change | Why |
| --- | --- | --- | --- |
| `core/tests/units.rs` `founding_with_no_centre_clan…` | camp at the capital with `UNIT.start` people; goods = store + scout + camp = endowment; camp count 1; `red.army.raised` 1 | Now: `raised` 0, no camp, no guard, scout with `SCOUT.people`; the clans lose `SCOUT.people` in all; store = endowment less the scout's pack; camp count 0 | Founding's subject is the start. This is the start's own assertion (a rule change) |
| `core/tests/units.rs` `a_real_map_replays_with_the_camp_under_orders` | 3 camps at the pick, each `people == UNIT.start`, `army.raised == 1` | 0 camps at the pick; the player raises `n: 1` at +0 (the store fills one pack); the rest unchanged | The start (no camp at the pick). `n: 1` because a store of 10 can't fill three (the unlogged `fill_stores` broke the replay) |
| `core/tests/units.rs` `the_ai_raises_a_lost_camp…` | "the AI raised its camp", `raised == 2` | "the AI raises no army until R" (no camp); then `armed` gives its camp and the post is checked; `raised == 2` | The AI's raise is the rule that changed. The final `raised` count is unchanged (three()'s camp and this one) |
| `core/tests/units.rs` `on_a_real_map_the_ai_camps…` | AI camps survive the pick and are fed | `armed` raises each civ's first army before the 100 turns; the same assertions | The AI raises none; the test's subject is the AI's camps |
| `core/tests/units.rs` corridor, corridor_off | (no assertion changed) | `armed_all` after `found` | Fixture: the camps the corridor tests put their own on |
| `core/tests/leaning.rs` `the_default_keep_is_the_camps_full_pack` | `keep` with no camp = `pack_cap(UNIT.start)` | `pack_cap(UNIT.first)` (the same 3) | The reader moved, not the rule |
| `core/tests/judge.rs`, `src/judge.rs` | `pack_cap(UNIT.start)` | `UNIT.first` | Same number |
| `core/tests/civs.rs` `a_card_counts_clans…` | `(clans, people, hearths) == (2, 6.5, 1)` | `(2, 9.5, 1)` | Fixture: the start takes no camp or guard from the clans; `hold_land` raises the guard (3). The card's rule is unchanged |
| `core/tests/cases.rs` `disband_all_sends_every_person_home` | camp, scout, guard from the stage | the stage places an army (`stage.army`) and raises a guard | Fixture: a civ starts with no army or guard |
| `core/tests/scout.rs` `the_scout_is_made…` | "the scout is not the camp" (a camp exists) | `camp(me).is_none()`: "a civ starts with its scout and no army" | The start's own assertion |
| `core/tests/events.rs` `every_kind_fires_in_real_runs` | `CampRaised` in the list of kinds the three runs must fire | removed from the list (note says why; `units.rs` and the stories see it) | The AI raises no army, so these runs never raise a camp |
| `core/tests/orders.rs` (`led_game`, the watcher, not-begun, AI log, another-civ, wrong-kind) | camps exist at the pick | `armed_all` after the pick | Fixture: the orders are about units that exist |
| `core/tests/notes.rs` `a_watched_game…` | camps exist in a watched game | `armed_all` then 10 turns | Fixture |
| `core/tests/stage.rs` `a_spec_round_trips…` | hand-built world with no guards | raises `GUARD.first` guards per civ, as `from_spec` does | Fixture: the builder must match `from_spec` |
| `core/tests/save.rs` | version 7 literal | 8 | `SAVE_VERSION` bumped |
| `core/tests/common/mod.rs` `three()` | camps from founding | `three()` calls `armed` (a helper: `UNIT.first` warriors banked and raised one at a time, then `GUARDS` = 3 guards) | Fixture, as the brief says |
| `cli/tests/play.rs` | games at the pick have a camp; `results.len() == 5`; `lines.len() == 3`; judge's `player orders: 1 (move 1)` | the first call raises a warrior at the pick (`RAISE`), so counts and indices shift by one; orders read `2 (move 1, raise 1)` | Fixture |
| `cli/tests/play.rs` gift test | seed 9, 5 turns after the pick, a clan in reach trades elsewhere | seed 1, with `RAISE` (seed 9 no longer has such a clan; `RAISE` on seed 1 gives one) | The start changes the clans; the test's subject is the gift's reach |
| `cli/tests/save.rs` pins | the four digests and the state digest | new values from `sansfiat digest` (see the code) | `stats::digest` moved with the start (`SAVE_VERSION` 8) |
| `www/check.mjs` | 3 camps at the pick; a raise adds to an existing camp; save version 7 | 0 camps at the pick; the first raise is `''` (allowed) and adds people; version 8; the panel arm reads the post-raise view | The start |

## The judge: before and after

Before (`main`, `sansfiat sweep --judge`):

```
total standing: top at +200 0.096, aligned at +200 0.200, on land at +200 0.001
total doubt: lead changes 10 on 6 maps, spread +100 0.061 +200 0.062, swings 20; no claims: lead changes 5 on 3 maps
total decisions: AI orders per 100 turns 21.86 (claim 15.98, disband 1.12, move 3.10, raise 1.05, zone 0.62), levers repeated 3.10
total motion: clans changed 127, people 594.5, land tiles 2425; clans left 0, joined 130; claimed tiles taken 0
total land: markets 218 at the pick, 211 at +200; trading 0.90/0.89; homes trailed 0.85/0.83; worked trailed 0.31/0.30; route trailed 0.97/0.99 (pick/+200); festival changes 0.87 and flips 0.09 per 100 clan-turns; per civ at +200 claimed 32.95, held 16.33, guard 2.43
total economy: store mean 8.14, store max 47.14, at pack 106.7 civ-turns a map, levy per turn 8.02, largest army 3.0, guard mean 2.14
total armies: camps kept 13 of 21, warriors starved 26, camps lost 6, people vs control 0.993
total fight (control, fighting off): people fallen 0.00 (0.00), tiles taken 0 (0), armies surrendered 0 (0)
total health: pre-games by centres 7 of 7, panics 0, digest mismatches 0
seconds 108.1
```

After (this branch):

```
total standing: top at +200 0.106, aligned at +200 0.209, on land at +200 0.000
total doubt: lead changes 10 on 4 maps, spread +100 0.069 +200 0.070, swings 5; no claims: lead changes 4 on 3 maps
total decisions: AI orders per 100 turns 13.24 (claim 11.43, disband 0.21, raise 1.60), levers repeated 1.62
total motion: clans changed 102, people 498.5, land tiles 1961; clans left 0, joined 103; claimed tiles taken 0
total land: markets 218 at the pick, 211 at +200; trading 0.90/0.90; homes trailed 0.85/0.85; worked trailed 0.31/0.30; route trailed 0.97/0.98 (pick/+200); festival changes 0.66 and flips 0.10 per 100 clan-turns; per civ at +200 claimed 32.43, held 21.33, guard 2.76
total economy: store mean 16.90, store max 48.09, at pack 228.9 civ-turns a map, levy per turn 7.77, largest army 0.0, guard mean 2.61
total armies: camps kept 21 of 21, warriors starved 0, camps lost 0, people vs control 0.992
total fight (control, fighting off): people fallen 0.00 (0.00), tiles taken 0 (0), armies surrendered 0 (0)
total health: pre-games by centres 7 of 7, panics 0, digest mismatches 0
```

What moved, as expected: no AI army (`total armies: camps kept 13 of 21, camps lost 6, warriors starved 26`
before; `camps kept 21 of 21, camps lost 0, warriors starved 0` after; `largest army 0.0`), so the AI's
orders fall (`AI orders per 100 turns 21.86 → 13.24`; raise 1.05 → 1.60). Stores
rise (`store mean 8.14 → 16.90`, `at pack 106.7 → 228.9 civ-turns a map`). The `total standing` top at +200
rises 0.096 → 0.106 and `lead changes` moves from 10 on 6 maps to 10 on 4, since the AI's world now starts
with no army. `total fight` is unchanged (no army, nothing fought). Per map, `small 2` moves from
`orders/100 6.83` to `3.83`, and `medium 9` from `30.83` to `11.17`.

## The stories on small 2 (default turns)

```
== skirmish
  refused at +50: {"type":"zone","unit":503,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +51: {"type":"zone","unit":503,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +52: {"type":"zone","unit":503,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +69: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +70: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +71: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +72: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +73: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +74: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +75: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +76: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
  refused at +77: {"type":"zone","unit":515,"tiles":[98,77]} (1 warriors can hold at most 1 tiles)
== stack
stack on small seed 2: led civ Red against Blue, 80 turns past the pick (3 civs alive), saved to /tmp/story-stack.json
== surrender
surrender on small seed 2: led civ Red against Blue, 60 turns past the pick (3 civs alive), saved to /tmp/story-surrender.json
  refused at +0: {"type":"raise","n":3,"kind":"camp"} (the store can't fill their packs)
```

- `skirmish` (80 turns, 3 civs alive): the army is one warrior, raised when the store fills; 18 zone
  refusals ("1 warriors can hold at most 1 tiles"). No capital falls.
- `stack` (80 turns, 3 civs alive): no refusals; its summary line only (the save has the army).
- `surrender` (60 turns, 3 civs alive): the +0 raise of 3 warriors is refused ("the store can't fill their
  packs"), so it has no army to march: it does nothing, as the probe found.

No story ends a game here (three civs alive in each).

## The page

`node www/check.mjs` passes. `./cross-build.sh` (native and wasm digests agree), `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` pass; `cargo test` passes (335).

`node www/play/all.mjs`: 13 of 16 scripts pass (`army`, `civ`, `feed`, `layout`, `select`, `start`, `pick`,
`bar`, `art`, `save`, `scout`, `views`, `end` pass; `camp`, `game` and `buttons` still fail). The page
scripts now raise their army through the page (`raiseArmy` in `www/play.mjs`: Raise… warriors, then a
guard, ending turns until the store fills a pack). The three that fail:

- `camp.mjs` at the hold (seed 4): the thicker guard must march onto a rival camp's post. The AI raises no
  army until R, so there is no rival camp to post against ("no rival camp at a post"). This is blocked
  until R, not a fixture problem. Before the hold, the same script raises 7 warriors in 150 turns (the
  store is 10 grain at the pick and the guard eats from it); its `WARRIORS` target went from 10 to 7 for that
  reason.
- `game.mjs` at the zone: "P and clicks paint the camp's zone (1 tiles)": the camp walks to a tile six or
  more steps from the capital and the ring round it is not the civ's held land, so no tile joins the zone.
  Not diagnosed further.
- `buttons.mjs` at the scout's Forage: the scout's pack falls 10 → 8 in a turn with Forage pressed. It was
  failing the same way before `raiseArmy` existed, so it is not the page script's raise. Not diagnosed.

So `engine/check.sh` is not green: `camp.mjs`, `game.mjs` and `buttons.mjs` fail as above. Those are for the
owner to decide (the hold waits on R; the other two need a look at the economy the start leaves).

## Decisions

1. `GUARD.first` (3), a new number in `data.rs`: a staged civ gets the guard founding used to give it.
2. `Stage::army` makes its camp with `units::make_camp` (no clans, no event); `Stage::from_spec` gives
   each civ `GUARD.first` guards.
3. Test fixtures raise one warrior at a time (`units::raise` takes at most one person per clan). `armed`
   banks the pack with an unlogged store top-up, which is for tests only; a test that replays its log raises
   through `Order::Raise` at one warrior, since the store at the pick fills one pack.
4. `camp.mjs`'s hold raises 7 warriors (was 10) and waits up to 150 turns, since the store feeds one pack
   at a time.
