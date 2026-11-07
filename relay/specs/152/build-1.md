# #152 build-1 (easel): the start

A civ starts with its scout and nothing else. Read `spec.md` items 1–7 and `probe.md`. The
code: `core/src/data.rs` (`UnitRules`, `GuardRules`), `core/src/civs.rs` (`found`, the end of
it), `core/src/units/mod.rs` (`raise_at_founding`, `take_people`, `make_scout`),
`core/src/units/guard.rs` (`raise_at_founding`), `core/src/units/ai.rs` (`ai`),
`core/src/script.rs` (`Lead`), `core/src/stage.rs` (`army`, `civs`), `core/tests/common/mod.rs`,
`cli/src/story.rs`, `cli/src/retro.rs` (`after_run`, `kept_camp`), `core/src/judge.rs`
(`turns_at_pack`), `cli/tests/save.rs` (the pins).

## What to build

1. **`data.rs`.** `UNIT.start` and `GUARD.start` are 0, each with a note: a civ starts with its
   scout and nothing else, and raises its army and guard itself (`founding.md`, The start). Add
   `UNIT.first: u32 = 3`, noted as the warriors of a first army where a script or the judge
   needs a number. Keep the fields (the piece names them) rather than deleting them.
2. **Founding** raises the scout alone. Remove `units::raise_at_founding` and
   `units::guard::raise_at_founding` and their calls in `civs::found` (with `start` 0 they would
   do nothing; dead code goes). The scout's pack still comes from the store; the rest of the
   endowment stays there. Fix `found`'s comment.
3. **The AI** raises no army: remove the camp raise in `units/ai.rs::ai` and say in its doc that
   the AI raises no army until R. Its claim, its guard (`guard_order`), its post and zone code
   for a camp it has stay (a camp it never has makes them idle).
4. **Readers of `UNIT.start`.** `judge.rs`'s `turns_at_pack` and its doc read `UNIT.first`
   (same number, so no score moves); `cli/src/retro.rs`'s lever raises `UNIT.first`;
   `kept_camp` is "no camp lost" (`army.lost == 0`), its doc saying so. `grep -rn
   "UNIT.start\|GUARD.start"` over `core`, `cli`, `web` and `www` must find only `data.rs`.
5. **`Stage::army(civ, tile, people)`** makes the civ a camp when it has none (a new camp
   unit as `units::join` makes one, without taking people from clans: a stage places), then
   places it as now. `Stage::civs`' doc no longer says founding raises a camp and a guard.
6. **The fixtures.** Run `cargo test` and fix every failure (the spec step ran the suite with
   both starts at 0: dozens fail, most at `common::camp_id`'s "a camp"). Add a `common` helper
   (`armed(w, civ)` or similar) that raises `UNIT.first` warriors at the capital with their
   pack from the store and `3` guards (`units::guard::raise`), as founding did, and call it
   from `three()` so the tests built on it keep their meaning. Tests whose subject is founding
   itself (a card, the start's units, `units.rs` lines 26 and 557) assert the new start: a
   scout and nothing else, the store holding the endowment less the scout's pack.
   **An assertion of a rule may not change.** An assertion whose number is the fixture's (which
   clan gave a person, a store's exact amount after the camp's pack) may, with a comment saying
   it follows the fixture. List every assertion you changed, and why, in
   `relay/specs/152/start.md`.
7. **The stories.** `surrender` raises `UNIT.first` warriors at +0 (`Order::Raise`, kind camp,
   warriors, a new army) before its Switches and Move. `skirmish` and `stack` drop the +0
   disband in `bank_and_raise` (nothing to disband) and its doc lines. Update the module doc's
   claim about every store at 0 at the pick: measure it and say what it is now.
8. **`Lead`** (`script.rs`): at +0, if the led civ has no camp, `Order::Raise { n: UNIT.first,
   kind: Camp, .. }`; update its numbered doc. `core/tests/script.rs` must still pass.
9. **The save version.** `SAVE_VERSION` bumps by one and the pins in `cli/tests/save.rs` move
   to the new digests.

## Proof and record

- `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`,
  `./cross-build.sh`, `node www/check.mjs`, `node www/play/all.mjs` (the page's scripts that
  select the camp or the guard at the start will need a raise first: `www/play/camp.mjs`,
  `army.mjs`, `game.mjs`, `buttons.mjs` and others; make each press Raise… through the page
  where it needs an army, since that is what a player now does).
- `sansfiat sweep --judge` before (main's binary) and after: put both totals blocks in
  `start.md` with a line on what moved (expect: no AI army, so armies and fight read 0, the AI's
  orders fall, stores rise). Run the three stories on small 2 at their default turns and put
  their summary lines and refusals in `start.md`.

## What later steps rely on

Founding makes a scout and nothing else; `UNIT.first` exists; `Stage::army` makes a camp; the
stories and `Lead` raise their own army; every check is green.

## Done

`engine/check.sh` green; `start.md` written; `engine/CLAUDE.md` unchanged unless a line in it
is now wrong (the AI's line in `units/` says it raises a camp: fix it).
