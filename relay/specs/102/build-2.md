# #102 build-2 (easel): the claim, the guard, land, leaning and leaving

You build the land in core: a civ's claim, its guard, what land it holds, clans leaning to it by land
and leaving their civ, the levy and Standing by land, the AI's stand-in, and the `claims` control.
`spec.md` items 7–25 are the full rules, with the starting numbers; this brief says where they go and
what done is. Read `spec.md` and build-1's summary first. You don't tune: the numbers are starting
values that must run; the canvas (build-4) tunes them.

## What to build, and where

1. **`core/src/claim.rs`** (new, one `pub mod` line in `lib.rs`): the land. `Civ.claim` and `Civ.guard`
   live on the civ (`core/src/civs.rs`, `CivFlat`, `#[serde(default)]`), and this module holds what
   reads and changes them: checking a claim (connected, land, holds the capital), `connected_part`
   (the claim's tiles still joined to the capital), the guard's division (spec item 12, reusing
   `actors::divide` and `Walks` as `units/camp.rs::hold` does), `held(w, civ)`, the new `tile_civ` (spec
   item 13; `civs::tile_civ` stays as the name callers use, calling here, or callers move: your call),
   the `claim` step (item 14, in `TURN` after `units`), and the AI's stand-in claim (item 22).
   `CIV.territory`, `civs::territory` and `civs::work_tiles` go, with their tests; `web/src/lib.rs`'s
   `territory`/`work` fields become `claim`, `guard`, `land` (build-3 draws them; keep `www/check.mjs`
   passing).
2. **The guard** (items 10–11): `UnitState::Guard` in `core/src/units/mod.rs` (its flat JSON's `kind`
   is `"guard"`), raised and disbanded through `Order::Raise { n, kind }` / `Disband` in
   `core/src/orders.rs`; `kind` is `#[serde(default)]` to `camp`. Its turn (eat from the store, hunger,
   losing people to the own clan) goes in `units/mod.rs`'s `units` step or a small `units/guard.rs` if
   it passes ~80 lines. The orders a guard refuses are refused in `check_order` with a reason the page
   shows. `civs::keep` adds the guard's `GUARD.keep_turns` turns of food. At founding, `GUARD.start`
   guards when the civ may claim (`raise_at_founding`'s neighbour).
3. **`Order::Claim { tiles }`** (item 8), `kind()` `claim`, checked in `check_order`, applied by setting
   the claim (capital added, sorted) and dividing the guard at once so the page sees it this turn.
4. **Leaning and leaving** in `core/src/leaning.rs` (items 15–19): every clan's leanings fade; leaning
   by land; joining as now plus the start at `LEANING.join`; `Clan.drawn` and `Clan.parent`
   (`core/src/clans.rs`, saved, `#[serde(default)]`; `parent` set where a clan divides and sends a
   daughter); leaving with daughters; the civ's own clan never leaves. Founding's joins
   (`civs::found`) set the starting leaning. `ClanLeft` and `ClaimTaken` in `core/src/events.rs` with
   their wording, alerts for the player's civ (`World::flag_alerts`).
5. **Gifts and trade** (item 17) in `core/src/markets.rs::trade` and `leaning::give`.
6. **The levy** (item 20) in `civs::levy`. **Standing** (item 21) in `stats::standing`.
7. **The AI** (item 22) in `core/src/units/ai.rs`: the stand-in claim (from `claim.rs`) sent as
   `Order::Claim` when it differs, and the guard raised and disbanded by its rule. `post` reads land
   through `tile_civ` as before.
8. **`Config.claims`** (item 23) in `core/src/data.rs`, with the `retro` and `judge` CLI paths passing
   `all`; `sweep --after` gains `--no-claims`. The numbers of item 24 as `CLAIM`, `GUARD` and the new
   `LEANING` fields, each with a note.
9. **The save**: digests re-pinned (`SAVE_VERSION` stays 3, build-1's). `engine/CLAUDE.md`: a row for
   `claim.rs` (and `units/guard.rs` if made), the changed rows, and a line in Adding a mechanic if the
   guard shows a unit kind's pattern A will follow.

## Tests (`core/tests/claim.rs`, `core/tests/leaning.rs`; built maps from `tests/common`)

- A claim must be connected and hold the capital; water or a gap is refused; `claims: none` refuses it.
- One guard over a small claim holds every tile; the same guard over a wide claim leaves the far tiles
  paper; a camp holding a paper tile of its own claim makes it held.
- A rival scout ending its turn on a paper tile takes it and the tiles cut off behind it; on a held
  tile it takes nothing; never the capital.
- Two held claims on one tile: the thicker guard's land.
- An unaligned clan on a guarded home leans faster than one on a barely held home, and joins; a civ
  clan on a rival's land, drawn for `leave_run` turns, leaves with its daughter; the own clan doesn't.
- Raising a guard leaves Standing as it was; an unaligned clan on a civ's land counts toward it.
- An unaligned clan on the land pays the levy at the civ's market.
- The guard goes hungry with an empty store and loses people to the own clan after `hungry_turns`.
- Replaying a save with claims and guards rebuilds the same world.

## Done

- `./check.sh` passes.
- `cargo run --release -p sansfiat -- sweep --judge` on the retro maps saved as
  `relay/specs/102/build-2-judge.txt`: it runs with no panic, pre-games 7 of 7 by centres, and the
  claims show (land tiles changing, clans joining). Say in your summary what the starting numbers did:
  how many tiles each civ held at +200, its guard, clans that left. It is the canvas's starting point,
  not a verdict.
- `sweep --after 200 --maps retro --no-claims` runs.

Build-3 relies on: `Civ.claim`, `Civ.guard`, `claim::held`, `tile_civ` on land, `Order::Claim` and
`Raise { kind }`, `ClanLeft` and `ClaimTaken`, `Config.claims` with `only(k)`, and `Sim.view()`'s
`claim`, `guard`, `land` per civ.
