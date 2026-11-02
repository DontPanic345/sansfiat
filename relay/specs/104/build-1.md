# #104 build-1 (clay): stacks, kinds, many armies and the fight rule

You shape the centre of the piece: armies as stacks of kinds, as many as a civ raises, and
the one rule by which people of two civs on a tile fight. Every later step builds on what you
leave: build-2 adds militia, the capital, siege, surrender and the fallen capital as parts
and outcomes of your rule; build-3 counts and provokes fights; build-4 draws them; build-5
tunes your numbers. Read `spec.md` items 1–11 first, then `docs/design/war.md`, then
`engine/CLAUDE.md`.

## Read before you change anything

- `core/src/units/mod.rs` whole: `Unit`, `UnitFlat`, `join`, `raise`, `disband`, `step`.
- `core/src/units/camp.rs` whole: `camp_march`, `hold`, `camp_turn`.
- `core/src/claim.rs`: `divide`, `tile_civ`, `step`, `take`.
- `core/src/orders.rs`: `Raise`, `Disband`, `check_raise`, `apply`.
- `core/src/script.rs` (`Lead`), and `core/tests/units.rs` and `core/tests/claim.rs` for the
  built maps (`tests/common`).

## What to build, and where

1. **Kinds** (`data.rs`). `pub enum Fighter { Warriors, Archers }` (serde camelCase, Default
   warriors) and `pub const FIGHTERS: [FighterRules; 2]` with `name`, `attack` and `defence`,
   each with a note. Start warriors at 1.0/1.0 and archers at 0.8/1.4. Add
   `pub const FIGHT: FightRules` with `rate` 0.1, `entrench` 0.1, `entrench_cap` 0.5,
   `retreat` 0.5, `hungry` 0.5, `floor` 0.01, and `militia` 0.3 and `surrender` 3 for build-2.
   Each field gets a doc comment saying what it is, and the const a note saying these are
   starting numbers for the glaze.

2. **The stack** (`units/camp.rs`, `units/mod.rs`). `CampState` gains `stack: [f64; 2]` and
   `still: u32`, and `bows: bool` for build-2. `UnitFlat` gains `stack` (a scout and a guard write
   zeros), `still` and `bows`, each `#[serde(default)]`. Every place that changes an army's
   people changes its stack in proportion: `join`, `disband`, hunger in `camp_turn`, and your
   fight. Put that in one helper on `Unit` (for example `take_people(k) -> [f64; 2]`, and
   `add_people(fighter, k)`), so build-2's surrender and militia use it too. `still` counts
   turns without a step: 0 when `camp_march` takes one or a `Move` is applied, else +1 each turn.

3. **Many armies** (`units/mod.rs`, `orders.rs`). `Order::Raise` gains `fighter: Fighter` and
   `into: Option<u32>` (`#[serde(default)]` both); `Order::Disband` gains
   `fighter: Option<Fighter>`. Item 4 of the spec says what each does and what each refusal
   says. `join` no longer finds "the civ's camp": a new army is made unless `into` names one.
   `units::camp(w, civ)` stays as the first army by id. `Lead` raises `into` its army. The
   AI's `camp(w, civ).is_none()` raise stays, a new army at the capital. Check `cli/src/view.rs`
   and `cli/src/play.rs`'s legal orders still print, and that `web/src/lib.rs` compiles. Build-2
   rewrites the view's army lines. Archers are refused for now ("archers come in build-2" is
   fine as the message: build-2 replaces it).

4. **The fight** (`core/src/units/fight.rs`, new). The rule in spec items 6–8, with `hold`'s
   contest folded in. Shape it so build-2 adds parts and outcomes without reshaping it:
   - `presence(w) -> BTreeMap<usize, Vec<Part>>`: per tile, the parts there. A `Part` is
     `{ civ, who: Who, people, attack, defence }` with `Who::Army(id) | Who::Guard(id) |
     Who::Militia(clan)`. Build-2 adds militia parts here and the capital's drawn-back guard.
     Armies split their people over their held tiles by their guard shares. A guard splits over
     its civ's held claim by its own shares: split `claim::divide` so the guard's own division
     is a function you can call.
   - `contested(&presence)`: the tiles with two or more civs over `FIGHT.floor`.
   - `step(w)`: per contested tile in tile order, the holder at the start (`holder_of`: an army
     holding it, else `claim::tile_civ`), each side's strength, the losses (split over parts by
     people, an army's over its stack), the retreats, who holds after, and the counts. A side
     whose people there all fall loses the tile as a retreat does. With `Config.fight` off: no
     losses and no retreats.
   - `hold` (`camp.rs`) keeps the spread and returns, as now, each camp's wanted tiles and
     shares. Its `beaten` test between civs goes. The fight decides between civs, and `hold`'s
     rule stands between one civ's own armies. Keep `hold`'s result in `CampState.held` and
     `guard` as now, so `claim::divide`, `civs::holder`, the line and the page read it unchanged.
   - Only armies retreat. A retreating army moves at once to the nearest tile, by path cost,
     that its civ holds (`civs::holder == civ`) and isn't contested, and keeps its zone only if
     it is still connected. A guard never retreats: it loses a tile only when all its people
     there fall (losses are capped at presence), and then the tile goes through `claim::take`,
     made `pub(crate)`. Note it on both civs, and emit `TileTaken` rather than `ClaimTaken` when
     the tile was held. Spec item 8 says why (`spec-review.md`: a guard's retreat would end
     every skirmish on its first turn).
   - Notes: one per army and per civ whose guard fought, on each contested tile, with both
     strengths, entrenchment, losses each way and the outcome (spec item 11 has an example).
     Events: `Skirmish` (the first turn a tile is contested, keyed so a continuing fight doesn't
     repeat it; keep last turn's contested tiles on `World`, `#[serde(default)]`), `Retreated`
     and `TileTaken`, with wording in `events.rs`.
   - `Counters` gains `fallen: f64`, `taken: u32` and `surrendered: u32` (`#[serde(default)]`).

5. **The AI's stand-in claim** (`claim::ai_claim`) skips tiles a rival army holds (its
   `held`) or stands on. The AI re-paints its claim whenever it differs from the stand-in, so
   without this a tile a stack took comes back under it the next turn and is fought for and
   "taken" again every turn. A test: a stack on a tile it took from an AI civ isn't fought
   again the turn after, and `taken` counts it once.

6. **The march stops** (`camp.rs`, `camp_march`). It ends on the first tile of its path where
   a rival has presence (the presence before this turn's march is fine, as long as it is the
   same for every army in id order). Paper doesn't stop it. Only armies take paper in
   `claim::step`, not scouts or guards.

7. **`Config.fight`** (`data.rs`), `#[serde(default = "yes")]`, with the doc comment saying
   off is the control (no losses, no retreats, and from build-2 no militia, surrender or fall).

8. **The order of the step** (`units::step`): fallow, marches, `hold`, `fight::step`, each
   unit's turn. Hunger's take comes off the stack.

9. **Saves.** The rules change, so bump `SAVE_VERSION` and the pins in `cli/tests/save.rs`
   together. `./cross-build.sh` must pass: `digest --lead` and `Sim::lead_orders` agree.

## Tests (`core/tests/fight.rs`, new)

On built maps, with civs and units placed by hand:
- The stack sums to people after raise, join, disband by kind and in proportion, hunger and a
  fight.
- Two armies of two civs on one tile: each loses `rate ×` the other's strength, the weaker
  retreats when under `retreat ×` the other's, and the stronger holds.
- Entrenchment: the same fight with the defender still 5 turns loses less.
- Archers defend better than warriors and attack worse.
- A stack of ten entering a guarded claim tile: the guard's presence there is wiped, the tile
  leaves the claim (`TileTaken`), and the army loses under 0.05 people.
- Two guards on an overlapped claim tile, one with twice the other's share there: both lose
  people, neither retreats, and neither tile leaves its claim.
- A march stops on a tile a rival holds, and not on paper.
- Conservation: the people of the world before the fight step equal those after plus
  `fallen`, summed over civs.
- `Config.fight` off: the same fights lose nobody.
- A scout on a rival's held tile neither fights nor takes it.
- Raise `into` an army whose line is cut is refused with its reason, and raise with `into: None`
  makes a second army.

## Done

- `./check.sh` passes. The new tests pass. `sweep --judge` runs without a panic. Save its
  report as `relay/specs/104/build-1-judge.txt` and say in your summary what moved beside
  `main-judge.txt` and why. Nothing should fight on the AI maps, so the moves should come only
  from scouts and guards no longer taking paper, and from the march.
- Your summary names the fight module's interface (`Part`, `Who`, `presence`, `step`, the
  helpers on `Unit`) and the invariants the kiln must keep: the stack sums to people, every
  loss goes through the one helper and is counted in `fallen`, and holding between civs is
  decided only in `fight.rs`.
- Where the spec's rule doesn't fit the code, make the smallest change that works and record
  it as a decision.
