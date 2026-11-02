# #104 build-2 (kiln): archers, militia, the capital, siege, surrender and the fallen capital

You build the war rules on build-1's clay: the fight module `core/src/units/fight.rs`, the
stack on `CampState`, and the helpers that take and add an army's people. Keep the clay's
shape. Militia and the drawn-back guard are new **parts** in its `presence`. Surrender and the
fallen capital are **outcomes** after its step. Every loss goes through its helper and is
counted in `Counters.fallen`, and holding between civs is decided only in `fight.rs`. Where the
clay doesn't hold, make the smallest fix that works and record it as a decision. Read
`spec.md` items 12–18, build-1's summary and decisions, and `docs/design/war.md`.

All numbers are already in `data.rs` (`FIGHT.militia`, `FIGHT.surrender`). Use them. Don't tune
them: build-5 does.

## What to build, and where

1. **Archers** (`orders.rs`, `units/mod.rs`, spec item 17). `Raise { fighter: archers }` is
   allowed when the civ holds a forest tile (`claim::tile_civ == civ` on a `FOREST` tile),
   else refused with "you hold no forest to make bows on". The army they join or make sets
   `CampState.bows`, and `camp_march` takes no step that turn. Clear it at the end of the
   army's turn. Emit `ArchersRaised`.

2. **Militia** (`fight.rs`, spec item 12). In `presence`, on a tile where an army of civ Y
   stands (its own tile), each clan of a civ X ≠ Y whose home is that tile is a
   `Who::Militia(clan)` part of X, with people = the clan's people and strength per person
   `FIGHT.militia ×` warriors' defence. Its losses come off `clan.people`. It never retreats.
   Unaligned clans give no militia.

3. **The capital** (`fight.rs`, `claim.rs`, `civs.rs`, spec item 13). `Civ` gains
   `pressed: bool` (`#[serde(default)]`), set by the fight step when a rival army stands on
   its capital and cleared when none does. While pressed, its guard is one part on the capital
   with all its people, entrenched at the cap, and `claim::divide` gives the rest of the claim
   no guard share.

4. **Siege** (`fight.rs` or `civs.rs`, `clans.rs`, spec item 14). `Civ` gains
   `besieged: bool`: every walkable neighbour of the capital is held or stood on by a rival
   army. While it is set, the civ's clans whose home is the capital harvest nothing, and
   `levy` puts nothing in the store. Note it on the civ once, when it starts and when it ends.

5. **Surrender** (`camp.rs`'s hunger, `units/line.rs`, `units/mod.rs`, spec item 15). In
   `camp_turn`, after eating and before hunger's disband: an army hungry `FIGHT.surrender` turns
   in a row (3, the same turn hunger would first take a warrior) whose line is cut by a rival
   surrenders whole. Add `line::cut_by_rival(w, unit) -> Option<u32>`: `Some(rival)` when the
   line over walkable ground alone would carry the army's food and the line that respects
   rivals doesn't, the rival being the first whose land (`tile_civ`) or held ground lies on the
   walkable-only line. Don't use `line::cutter`: it falls back to the lowest rival. Its people
   (as people, with its pack in the clan's store) join that rival's nearest clan by steps (ties
   to the lower id). It counts in its own civ's `Counters.surrendered`, and the people are not
   `fallen`. Its own tile is never the rival's land (the army took it), which is why the rival is
   read off the line. With the line open or cut by length alone, hunger works as now. Emit
   `ArmySurrendered` (civ, other, unit, amount) and note both civs and the receiving clan.
   With `Config.fight` off there is no surrender: hunger works as on main.

6. **The fallen capital** (`fight.rs` for the test, a `fall(w, civ, by)` in `civs.rs`, spec
   item 16). After the fight step, a capital with a rival army on it and no part of its civ
   left there (no guard people, no army, under one person of militia) falls. `Civ.fallen:
   Option<(u32, u32)>` (`#[serde(default)]`) is set, and then:
   - its claim is cleared and its guard disbanded;
   - its clans go unaligned with their leanings cleared;
   - its armies and scouts disband to the nearest clan by steps;
   - its store fills the packs of the taking civ's armies on the tile, in id order, and the
     rest is lost;
   - its capital's market fades like any other festival;
   - `CapitalFell` is emitted, with a note on both civs.

   Then find every rule that reads a civ's `capital` or iterates `w.civs`, and skip fallen
   civs where a rule would treat them as alive. At least: `claim::tile_civ`, `claim::check`,
   `zone_ok`, the line (`line_by`), `units::ai` and `post`, `civs::levy` and `keep`,
   `leaning::lean` and `give`, `stats::standing` and `standing_parts`, `markets`'s capital
   market never fading, and `World::validate`/`check_order` ("your civ has fallen"). Add
   `Civ::is_out()` or a `civs::alive(w)` iterator, and use it.

7. **Alerts and wording** (`events.rs`, `world.rs`'s `alert_key`, spec item 18). The new
   kinds' text, and alerts for the led civ: a skirmish begun on its land or by its army, a
   tile it lost, its army retreating, a surrender it is party to, any fallen capital.

8. **The text view and legal orders** (`cli/src/view.rs`, `cli/src/play.rs`). Each army is
   "army 475 at …: 6.0 warriors, 2.0 archers, pack …, entrenched 3 turns", with "pressed" and
   "besieged" on the civ's line. The legal orders show raise with `fighter` and `into` for each
   army whose line isn't cut, a scout raise once build-3 adds it, and disband with a kind. Keep
   the view under 80 lines at the pick and +100 on small seed 2.

9. **Saves.** Bump `SAVE_VERSION` and the pins together if the digest moves. `cross-build.sh`
   passes.

## Tests (`core/tests/fight.rs`, `core/tests/civs.rs`)

- Militia: an army on a rival clan's home loses people to it, and the clan loses people. An
  unaligned clan's home gives no militia.
- The capital: a rival army on it draws the whole guard there, and the rest of the claim is
  paper that turn.
- Siege: all neighbours held by a rival, the capital's clans harvest nothing and the store gets
  no levy.
- Surrender: an army of three whose line a rival's land cuts, starved `FIGHT.surrender` turns,
  adds all three to the rival's nearest clan before hunger takes one, and both civs hear. An
  army cut off by length alone, on unclaimed land, starves as now. With fighting off it doesn't
  surrender.
- A capital falls only when no guard, army or militia of its civ is left on it. Then its clans
  are unaligned, its Standing is 0, its claim is empty, `tile_civ` of its capital is not it,
  its orders are refused, and a turn after runs without a panic.
- Archers: refused without held forest; with it the army doesn't march that turn.
- People are conserved through surrender and a fall.

## Done

`./check.sh` passes, the tests pass, and `sweep --judge` runs without a panic. Save its report
as `relay/specs/104/build-2-judge.txt`. The AI doesn't fight, so it should match build-1's
except where a rule reached the AI maps; say which. Your summary says what build-3's stories can
rely on: how to read a surrender, a fall and a siege from the world and the events.
