# #157 build-2 (easel): the planner's frame, the land and the gift

The AI civs are to play for Standing with the player's levers (`docs/design/war.md`, `vision.md`). Today
`core/src/units/ai.rs` claims a stand-in patch (`claim::ai_claim`), keeps a guard (`guard_order`) and, with no
army raised, does nothing else. Build the **planner** that replaces it: its frame and need order, and the two
needs about land and Standing. Build-3 adds the three army needs into the same frame; build-4 tunes all of
it. Read `relay/specs/157/spec.md` in full (its "What I settled" and decisions are the rules), `probe.md`, and
build-1's `judge.md` (the stand-in's numbers and the JSON paths).

## The frame (`core/src/units/plan.rs`)

`ai.rs` keeps the `ai` step: for each civ the player doesn't lead, it calls `plan::turn(w, civ)`. The planner
walks five needs in this order, and gives the orders of the first that has something affordable to do; a need
with nothing it can afford to do is met, and the next is read. Orders standing from earlier turns (a march,
a zone, a claim) carry on: the planner orders only to change something.

1. feed the armies (build-3; here, the guard's half: a guard that went hungry last turn sheds a person, as
   `guard_order` does today);
2. hold the land;
3. grow Standing;
4. hurt the nearest rival (build-3);
5. take a capital it outweighs (build-3).

Each need is a function that reads the world and returns the orders it would give (`Vec<Order>`, plus the
gift and keep it sets), so a test can ask a need what it wants without the turn. Every order is checked with
`w.check_order` before `w.order`: the planner never has an order refused. Each order carries one `w.note` on
the civ saying which need gave it and why ("claims (3, −2): clan 41's home is off its land"). Draw no random
numbers.

`PLAN` in `data.rs`: every threshold the planner reads, each with a `note`. Starting numbers that run; the
canvas tunes them.

## 2. Hold the land

- **The patch** (`plan::patch(w, civ) -> Vec<usize>`): seeds are the capital and the homes of the civ's own
  clans within `PLAN.reach` steps of the capital (start 4, the stand-in's reach). The patch is the tiles of
  the cheapest land path from the capital to each seed (`land::path` or `path_between`) and the land tiles
  within `PLAN.ring` (start 1) of each seed; never another civ's capital, never a tile a rival army holds or
  stands on (with `Config.fight`), cut to what's connected (`claim::connected_part`). The claim check passes
  on it.
- **Widen** by one tile a turn while no tile of the claim is paper and one more guard can be fed
  (`guard_order`'s test): the land tile nearest the claim within `PLAN.ring` of an unaligned clan's home within
  `PLAN.reach` (+1 steps for each tile already widened is fine, or keep a simple distance: say which).
- **Raise the guard** a person a turn while a claimed tile is paper and it can feed one: `guard_order`'s
  rule, moved here.
- **Shrink**: when a tile is paper and no guard can be fed, drop the paper tile farthest from the capital,
  and whatever no longer joins.
- The need has nothing to do when the claim is the patch (plus what it widened), nothing is paper, and the
  guard is fed.

## 3. Grow Standing

- **The gift**: `PLAN.gift` (start `LEANING.gift_default`) while an unaligned clan at the civ's festival has a
  leaning to a rival civ, else 0. A civ gives only at its own festival (#102), to clans not its own.
- **The keep**: the store's cap (`CIV.store_cap`) while the civ means to raise (build-3 sets the want; here
  read a `PLAN`-driven `wants_army(w, civ)` that returns false until build-3), else the default (`None`).
- **Widening toward unaligned homes** is item 2's widen; this need's job is the gift and keep. It has nothing
  to do when both are already what they should be.

Both go through two setters on `World` that `act`'s `SetGift` and `SetKeep` arms call too:
`World::set_gift(civ, food)` and `World::set_keep(civ, food)`, each counting the lever (`count(civ,
"setGift")`) and writing the note, the player's wording kept for the player and "the AI set …" for the AI.
No new `Action` or `Order`.

## The stand-in goes

- `claim::ai_claim` and `CLAIM.ai_ring`/`ai_reach` go. Founding (`civs.rs`, `found`) gives each civ the
  planner's patch (`plan::patch`) where it may claim, else its capital.
- `cli/src/story.rs` reads the rival's claim from the world (`w.civs[&rival].claim`) where it read
  `ai_claim`; its stories still run (`check.sh` runs them, or say how they're checked).
- `core/tests/claim.rs`'s stand-in tests become patch tests.
- `units::post` stays: the text view and the camp sweep use it.
- `docs/design` lines that name the stand-in are the review's: list them in your decisions ("Until the AI
  learns to claim …" in `war.md`'s ledger, "Until the AI learns to play for the end, it raises no army" in
  `founding.md`'s).

## Leaving

`LEANING.leave_margin` (start 1.0): a civ's clan is drawn while its strongest rival leaning is at least
`leave_at` and more than `leave_margin` times its leaning to its own civ (`leaning.rs`). At 1.0 it is today's
rule, digest-equal. The canvas moves it.

## Proof, and what later steps rely on

- `engine/check.sh` green. The digest moves (the claim changes), so `SAVE_VERSION` bumps and the pins move.
- `core/tests/plan.rs`, on stages (`core/src/stage.rs`), one test a lever:
  - the patch covers the capital, the paths to its clans' homes and their rings, and no rival capital;
  - it widens toward an unaligned home while the guard holds, and not while a tile is paper;
  - the guard is raised while a tile is paper and the store feeds one, and a hungry guard sheds a person;
  - it shrinks from the far paper tile when no guard can be fed;
  - the gift goes to `PLAN.gift` when an unaligned clan at its festival leans to a rival, and to 0 when none
    does; `setGift` is counted;
  - the need order: with a paper tile and a contested festival together, the land is served first;
  - no planner order is refused over 100 turns of a retro stage.
- `sansfiat sweep --judge --json` on the seven maps: no panic, every game ends, and in `relay/specs/157/plan.md`
  the led block, lead changes, clans left, aligned Standing and orders per 100 by kind beside `judge.md`'s
  stand-in. Not a bound: the canvas tunes. Say what the patch did to held land and clans joined.
- Build-3 relies on: `plan::turn`, the need functions' shape, `PLAN`, `wants_army`, the setters, and
  `core/tests/plan.rs`'s stage helpers.

`engine/CLAUDE.md`'s layout gains `units/plan.rs` (and its line on `ai.rs` changes).
