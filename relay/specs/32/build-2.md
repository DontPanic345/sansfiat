# Build 2: units in the engine

Piece #32, Founding. Build-1 made civs, the phases (`Pregame → Choose → Main`), the pick
(`ChooseCiv`), the store and its `civs` step. You add units: the warrior, made, fed, moved
and disbanded, with their events and orders. The page comes in build-3. Work in `engine/`;
read `engine/CLAUDE.md` and build-1's result first. `relay/specs/32/spec.md` has the plan.

## The design you're building (from `docs/design/founding.md`)

> A unit is people, so it costs people and food.
>
> - **Making one** takes a person from a civ clan near the capital, whose people drop by one.
>   It's made at the capital.
> - **Keeping one** costs its food each turn, taken from the store. A unit the store can't
>   feed goes hungry; hungry for a few turns, it disbands.
> - **Disbanding** sends its person back to the nearest civ clan.
> - **Moving.** Select a unit, right-click a tile, and it paths there over the cheapest route
>   it knows, a turn's labour at a time.
>
> The one kind of unit for now is a **warrior**: a generic unit that moves, eats and disbands,
> and nothing else. It's a stand-in so founding has a unit to work with; the scout, army and
> supply pieces replace or change it. Each civ starts with one.

The piece adds: every unit moves and disbands; new events: a unit is made, goes hungry or
disbands. A warrior doesn't hold land, harvest, block or fight, and any number share a tile.
AI civs give no orders: their warrior sits and eats.

## What to build

### State and rules

- `data.rs`: `UNIT: UnitRules { labour: 3.5, food: 1.0, hungry_turns: 3, min_clan: 2.0 }`,
  documented: `labour` is one person's work a turn (`CLAN.work`), `food` one person's need,
  `hungry_turns` the hungry turns in a row before it disbands, `min_clan` the fewest people a
  clan must have to give one.
- New module `core/src/units.rs`:

  ```rust
  #[serde(rename_all = "camelCase")] pub enum UnitKind { Warrior }
  pub struct Unit { pub id: u32, pub civ: u32, pub kind: UnitKind, pub tile: usize,
                    pub dest: Option<usize>, pub banked: f64, pub hungry: u32 }
  ```

  `World` gains `units: BTreeMap<u32, Unit>` (serialised). Ids from `w.next_id`.

### Making

`units::make(w, civ) -> Result<u32, String>` (also used at founding) and
`Action::MakeUnit` (`{"type":"makeUnit"}`), which makes one for `w.player`'s civ.

- **Near the capital**: a clan of the civ with a round tile within `CIV.territory` (2) hex
  steps of the capital. The giver is the near clan with the most people (ties to the lower
  id) among those with at least `UNIT.min_clan` people. Its `pop` drops by 1.
- The unit is a `Warrior` on the capital's tile, `dest` none, `banked` 0, `hungry` 0.
- Refusals (as `Err` text the page shows): not the main game; "you're watching: you lead no
  civ"; "no clan near the capital has a person to spare".
- Making costs no food.
- Push `UnitMade` (tile: the capital; `civ`, `unit`, and `clan`: the giver).

**The starting warrior.** In `civs::found`, after joining, each civ (id order) makes one by
the same rule; if no near clan can spare a person, from the civ's most populous clan with
`UNIT.min_clan`+ people (ties to the lower id); if none, it's made without taking anyone
(`clan: None` on the event). It never fails.

### Feeding and hunger

In the `civs` step, at build-1's slot "feed units" (after the levy, before the cap): each
civ's units in id order eat `UNIT.food` from its store, taken from each good in proportion to
what it holds. A unit that gets less than `UNIT.food` (allow 1e-9) is hungry this turn:
`hungry += 1`, and if it was 0 before, push `UnitHungry`. Fed, `hungry = 0`. A unit at
`UNIT.hungry_turns` hungry turns disbands (below) with `UnitStarved` instead of
`UnitDisbanded`.

### Disbanding

`units::disband(w, unit, starved: bool)` and `Action::DisbandUnit { unit }`
(`{"type":"disbandUnit","unit":40}`), applied at once. The person rejoins the civ clan whose
home is fewest hex steps from the unit's tile (ties to the lower id): its `pop` + 1. With no
civ clan the person is lost. The unit is removed. Event (`UnitDisbanded` or `UnitStarved`):
tile the unit's, `civ`, `unit`, `clan` the one it rejoined (or `None`).
Refusals: not the main game; watching; no such unit; "that unit isn't yours".

### Moving

- `land::path(w, from, to, allowed) -> Option<(Vec<usize>, f64)>`: the cheapest path by
  `step_cost` (the tiles after `from`, ending at `to`, and its cost), Dijkstra with the same
  tie-breaks as `cost_map` (cheapest, then lowest tile) and deterministic predecessors (on an
  equal cost, keep the first found). `from == to` gives `([], 0)`.
- A unit's route uses `allowed = land && passable`. "The cheapest route it knows" is
  `step_cost` as it stands: worn trails and river crossings count; hidden trail costs
  (`trail_cost`, `span`) are never read (a test holds it).
- `Action::MoveUnit { unit, to }` (`{"type":"moveUnit","unit":40,"to":812}`): refused when
  not the main game, watching, no such unit, not yours, `to` not a tile, not land, or "no
  route there". To its own tile it clears `dest`. Otherwise it sets `dest = Some(to)` and
  `banked = 0`. It doesn't move the unit: it moves when the turn ends.
- A `units` step right after `civs` in `TURN` (`units::march`): each unit with a `dest`, in
  id order, recomputes its route from where it stands; with none (can't happen with static
  terrain, but be safe) it clears `dest`. It has `banked + UNIT.labour` to spend: while the
  next step's cost is no more than what's left, it takes the step and pays. On arriving,
  `dest = None`, `banked = 0`. Otherwise `banked` is what's left, which goes towards the
  next step, so a step dearer than a turn's labour (marsh across a river: 3 + 2) is taken the
  turn after.
- `units::route(w, unit) -> Option<(Vec<usize>, f64)>`: its route from where it stands to
  `dest`, for the page.

### Validation and actions

All checks live in `World::validate` (with helpers in `units.rs`); `act` applies. Player
orders act for `w.player`'s civ. Every refusal leaves the world unchanged.

### Events

`EventKind::UnitMade`, `UnitHungry`, `UnitDisbanded`, `UnitStarved`, worded in
`Event::text` (the civ by name from `w.civs`, which never go away; the unit by id, since it may
be gone):

- `"Red makes warrior 812 from clan 40"` (no clan: `"Red makes warrior 812"`)
- `"Red's warrior 812 goes hungry"`
- `"Red's warrior 812 disbands; its person rejoins clan 40"` (no clan: `"…; its person is lost"`)
- `"Red's warrior 812 disbands from hunger; its person rejoins clan 40"` (same for lost)

### CLI, digest and cross-build

- `run`'s founding report adds each civ's units.
- `digest --lead`: a scripted game so native and wasm compare through orders. Rule, applied
  between turns, before each `EndTurn`:
  1. At `Choose`: `ChooseCiv` the lowest civ id.
  2. In `Main`, every 20 turns since the pick (counting from the pick's turn): `MakeUnit`
     (ignore a refusal).
  3. Each of the player's units with no `dest`, in id order: `MoveUnit` to the first that's
     accepted of: the other civs' capitals in civ id order, then its own capital.
  4. 60 turns after the pick: `DisbandUnit` the player's lowest unit id, if any.
  Write the same rule into `cross-build.mjs` as an optional fourth argument `lead`, and add a
  line `"2 16 400 lead"` to `cross-build.sh` (small seed 2 ends at 276). Without `lead` both
  watch at the pick (build-1).
- `bench` runs through the pick (watching), as `digest` does.

### Docs

Update `engine/CLAUDE.md`: `units.rs`, `land::path`, the `units` step, the actions,
`digest --lead` and the cross-build line.

## Tests (`core/tests/units.rs`)

Hand-built worlds (`common::big()`, `put`, `open`), founded by setting `w.end` and calling
`civs::found`, then `ChooseCiv`.

- Founding gives each civ one warrior on its capital; it took a person from the right clan
  (near, most people, 2+), or from the civ's biggest clan, or from nobody; `UnitMade` each.
- `MakeUnit`: takes one person from the near clan with the most people; skips a clan of 1.5;
  refused while watching, before the pick, and with no near clan that can spare one (world
  unchanged on every refusal).
- Feeding: units eat 1 a turn in proportion from the store; with an empty store a unit is
  hungry; `UnitHungry` fires once, not every hungry turn; on the third hungry turn it
  disbands with `UnitStarved` and its person rejoins the nearest civ clan; fed again, the
  count resets.
- `DisbandUnit`: person to the nearest civ clan by home hex steps, ties to the lower id; with
  no civ clan, lost; refused for another civ's unit and while watching.
- Moving: on a plains world with a forest wall and a gap, the unit takes the cheaper detour,
  not the straight line; it moves a turn's labour a turn (3 plains steps on turn one with 0.5
  banked; it arrives when the costs say); a step dearer than 3.5 (marsh across a river) is
  taken the next turn; a river crossed off its course costs `RIVER.cross` more and a trail on
  it doesn't; setting `trail_cost` and `span` on every tile changes no route; a new order
  resets `banked`; an order to its own tile clears `dest`.
- `MoveUnit` refusals: another civ's unit, a sea tile, a mountain, a tile with no route (a
  sea-split island), watching, before the pick.
- `land::path` agrees with `cost_map`'s cost on random pairs of a real map.
- Replay: a world led, with `MakeUnit`, `MoveUnit` and `DisbandUnit` among `EndTurn`s, saves
  and replays to an equal world and equal events.
- Real map: small seed 2 to the end, led, 100 turns with a unit made and sent towards a
  rival capital: no unit ever off passable land, stores within the cap, every unit's civ
  exists.

## Constraints

- The pre-game must not change: no new random draws, no change to existing steps' arithmetic.
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs`; anything else
  through `libm`. No `HashMap`.
- Keep `cargo test` near its time after build-1 (report it): real-map tests use one small seed
  that ends early.

## Done when

`cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`./build-web.sh`, `./cross-build.sh` (with the `lead` line passing) and `node www/check.mjs`
pass.
