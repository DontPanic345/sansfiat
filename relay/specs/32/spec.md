# Spec: #32, Founding

## What the piece delivers

The pre-game ends with three civs, the player leads one (or watches all three), and every
order is given through the page. Designs: `docs/design/founding.md` (founding, the store,
units) and `clans.md` (the clans that keep living as before). merge4's founding
(`merge4/src/modules/civ/rules.js` `onStart`, `upkeep`; `merge4/src/clans.js` `foundCivs`;
the pick in `merge4/src/main.js` `candidateHtml`) is the starting point; **merge4 stays
frozen**: read it, don't edit it.

1. **Founding.** When the pre-game ends, each of its three centres becomes a civ (name,
   colour, its market, a capital on the market tile). Clans in reach of a founding market
   join the nearest; a civ with none takes the nearest unaligned clan. Each civ's store gets
   30 grain, and each civ gets one warrior. The game waits for the pick.
2. **The pick.** The player leads one civ or watches all three on the AI. The pick shows each
   civ's clans, people, hearths, terrain and distance to the nearest rival.
3. **The main game.** Turns run on: every clan rule as in the pre-game, markets forming and
   fading (a civ's market never fades). A civ clan's daughters stay in its civ.
4. **The store.** One per civ, every good; spoils like a clan's store; a fed civ clan hands it
   half its leftover store each turn; capped, the excess lost.
5. **Units.** One kind, the warrior: a stand-in that moves, eats and disbands. Made at the
   capital from a person of a civ clan near it; fed from the store; hungry for three turns,
   it disbands; disbanding returns its person to the nearest civ clan. Selected and
   right-clicked, it paths to a tile over the cheapest route it knows, a turn's labour at a
   time.
6. **Events**: a civ is founded; a unit is made, goes hungry, disbands (by order, or from
   hunger).
7. **The page**: the pick, a top bar with the player's civ, people, store and units, a
   selection panel (what's selected and its actions as buttons), right-click to move, the
   tooltip naming the tile's civ, capitals, civ land, units and routes drawn. No order goes
   through a debug tool: `window.game.act` goes.

Out of scope (the piece and `founding.md`): scouting, the boon year (the endowment stands in),
minimap, graphs, start screen, saving (the Save/Load buttons stay as they are), clans joining
or leaving after founding, winning and losing, cities beyond the capital, projects,
materials, crafts, posting to markets, drafted labour, good and bad years, discoveries, fog,
and anything the AI civs do beyond existing.

This piece asks no question; the review checks it against the design and plays it.

## Steps

| Step | What | Model |
|---|---|---|
| build-1 | Civs, phases and the pick in the engine: founding, joining, the store and its levy, `ChooseCiv`, daughters' civ, the tile's civ, pick cards, `CivFounded`, CLI | sonnet high |
| build-2 | Units in the engine: the warrior, making, feeding and hunger, disbanding, paths and moving, the starting warrior, unit events, replay and cross-build with orders | sonnet high |
| build-3 | The page: wasm API, the pick, the top bar, the selection panel, right-click moves, drawing civs and units, feed and tooltip, `check.mjs`, headless screenshots | sonnet high |
| review | The whole piece against the design; plays founding and a unit's life through the page only | sonnet high |

Each step leaves `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`./build-web.sh`, `./cross-build.sh` and `node www/check.mjs` green (in `engine/`).

## The shape

New state on `World` (all serialised, so the digest covers it):

```rust
pub phase: Phase,                 // Pregame -> Choose (founding, at the end step) -> Main (ChooseCiv)
pub player: Option<u32>,          // the civ the player leads; None before the pick and when watching
pub civs: BTreeMap<u32, Civ>,
pub units: BTreeMap<u32, Unit>,   // build-2
```

```rust
#[serde(rename_all = "camelCase")] pub enum Phase { Pregame, Choose, Main }

pub struct Civ {            // core/src/civs.rs
    pub id: u32,
    pub name: String,       // CIVS[k].0
    pub color: String,      // CIVS[k].1
    pub market: u32,        // the founding market; never fades
    pub capital: usize,     // the market's tile: the capital city
    pub founded: u32,       // the turn
    pub store: Basket,      // every good (only foods exist)
    pub lost: f64,          // goods lost to the cap last turn, for the page
}

pub struct Unit {           // core/src/units.rs (build-2)
    pub id: u32,
    pub civ: u32,
    pub kind: UnitKind,     // enum UnitKind { Warrior }, serialised "warrior"
    pub tile: usize,
    pub dest: Option<usize>,
    pub banked: f64,        // labour put towards the next step and not yet enough to take it
    pub hungry: u32,        // hungry turns in a row
}
```

`Clan` gains `civ: Option<u32>`. Ids for civs and units come from `w.next_id`, like clans and
markets. Founding draws no random numbers.

New rules in `data.rs`:

```rust
pub const CIVS: [(&str, &str); 3] = [("Red", "#d64545"), ("Blue", "#3b6fd6"), ("Gold", "#d6a63b")];
pub const CIV: CivRules = CivRules { territory: 2, endowment: [0.0, 30.0, 0.0, 0.0, 0.0], levy: 0.5, store_cap: 50.0 };
pub const UNIT: UnitRules = UnitRules { labour: 3.5, food: 1.0, hungry_turns: 3, min_clan: 2.0 };
```

Actions (JSON as the page sends them):

| Action | JSON | Valid when |
|---|---|---|
| `EndTurn` | `{"type":"endTurn"}` | not `Choose` |
| `ChooseCiv { civ: Option<u32> }` | `{"type":"chooseCiv","civ":12}` / `"civ":null` | `Choose`, and `civ` is a civ or null |
| `MakeUnit` | `{"type":"makeUnit"}` | `Main`, a player civ, a civ clan near its capital can spare a person |
| `MoveUnit { unit, to }` | `{"type":"moveUnit","unit":40,"to":812}` | `Main`, the unit is the player's, `to` is reachable land |
| `DisbandUnit { unit }` | `{"type":"disbandUnit","unit":40}` | `Main`, the unit is the player's |

Player orders act for `w.player`'s civ, so they carry no civ. A refused order says why in
words the page can show.

The turn gains two steps after `eat`: `civs` (spoil the store, take the levy, feed the units,
disband the starving, cap the store) and `units` (move). Founding runs inside the `end` step,
once, right after the pre-game's end is set.

```rust
pub const TURN: &[Step] = &[regrow, move, go, harvest, trade, eat, civs, units, hold, markets, end];
```

New events (`EventKind`): `CivFounded`, `UnitMade`, `UnitHungry`, `UnitDisbanded`,
`UnitStarved`. `Event` gains `civ: Option<u32>` and `unit: Option<u32>`.

## Decisions the piece left open

1. **Founding happens in the `end` step, before the pick**, as merge4 does: the civs exist
   (with stores, clans and warriors) while the player chooses, so the pick can show them.
   `World.phase` is `Choose` until `ChooseCiv`; `EndTurn` is refused then ("choose a civ to
   lead, or watch"). Nothing before the end changes: the same seed gives the same pre-game.
2. **`World::run(turns)` watches at the pick**: when it meets `Choose`, it applies
   `ChooseCiv { civ: None }` through `act` (logged, so it replays) and carries on. Tests,
   `digest` and `bench` that run past the end keep working, and now run the main game. The
   CLI's `run` and `sweep` still stop at the end. `cross-build.mjs` does the same pick by hand.
3. **Civ names and colours** are merge4's first three, in `End.centres` order: Red, Blue,
   Gold. The page draws civ marks with a dark casing so Gold reads apart from hearths and
   centre rings.
4. **Joining.** A clan is in reach of a founding market when a tile of its round is in the
   market's `markets::reach` (path cost ≤ `MARKET.reach`), as `go` reads reach. It joins the
   civ whose market it reaches cheapest, ties to the lower civ id. A civ with no clan then
   takes the unaligned clan whose home is fewest hex steps from its capital (ties to the
   lower clan id), civs in id order.
5. **Clans' stores aren't emptied at founding.** `founding.md`'s "the civ's clans' stores are
   empty at founding" says why there's an endowment (in the boon-year story they'd be empty),
   not a rule to apply; emptying them now would only starve the first turns.
6. **The capital's territory is a label**: the land within `CIV.territory` (2) hex steps of the
   capital is the civ's. It doesn't stop clans (of any civ or none) holding rounds there.
   **The tile's civ** is the civ whose capital territory holds the tile, else the civ of the
   clan whose round holds it (`claim`), else none. A civ's **land** is the tiles whose civ it is.
7. **"Works the tiles next to it"**: the capital's work tiles are its tile and its six
   neighbours (`civs::work_tiles`), shown on the page. Nothing harvests them in this piece:
   the only labour a city could work them with is drafted labour, which `founding.md` puts out
   of scope, and the store's income is the levy. The scout and supply pieces, or a city piece,
   give them work. (Recorded as an idea for the owner.)
8. **The store**: one `Basket` over `GOODS`. Each turn, in the `civs` step: spoil by
   `CLAN.spoilage`; add `CIV.levy` × each fed civ clan's store (taken from the clan); feed the
   units; then cut the whole store proportionally to `CIV.store_cap` (50, merge4's `baseCap`
   for one city), recording `lost`. The endowment goes in at founding, uncapped by that turn.
9. **Near the capital**: a civ clan with a round tile within `CIV.territory` steps of the
   capital. Making takes one person from the near clan with the most people (ties to the lower
   id) that has at least `UNIT.min_clan` (2) people, so a clan is never emptied. Making costs no
   food; keeping does.
10. **The starting warrior** is made by the same rule; if no clan near the capital can spare a
    person, from the civ's most populous clan with 2+ people; if none (only on hand-built
    worlds), it's made without taking anyone.
11. **Feeding**: each unit eats `UNIT.food` (1, one person's need) a turn from its civ's store,
    taken from each good in proportion to what the store holds; civs and units in id order. A
    unit that gets less is hungry that turn: `UnitHungry` when it goes from fed to hungry; at
    `UNIT.hungry_turns` (3) hungry turns in a row it disbands (`UnitStarved`). Fed, the count
    resets.
12. **Disbanding** returns the person (pop + 1) to the civ clan whose home is fewest hex steps
    from the unit's tile, ties to the lower id; with no civ clan the person is lost and the
    event says so. A disband order is applied at once.
13. **Moving.** The cheapest route it knows is the cheapest path by `land::step_cost` (worn
    trails and river crossings as they stand; hidden trail costs are not read) over passable
    land. `MoveUnit` checks a route exists and sets `dest`; to the unit's own tile it clears
    `dest`. The `units` step moves every unit with a `dest`: it recomputes the route from where
    it stands (trails change), then takes steps while `banked + UNIT.labour` covers them; what's
    left is banked towards the next step, so a step dearer than a turn's labour (marsh across a
    river: 5) takes two turns. Arriving clears `dest` and `banked`; a new order clears `banked`.
    Orders don't move a unit at once: it moves when the turn ends, and the page shows the route
    straight away. A unit's labour is one person's work, `CLAN.work` (3.5).
14. **A warrior** does nothing else: it doesn't hold land, harvest, block or fight, and any
    number share a tile.
15. **AI civs** exist, keep their store and feed their warrior, and give no orders. Watching,
    all three are AI.
16. **The pick's numbers** (`civs::cards`): per civ, its clans and their people, the hearths and
    the three commonest terrains (with shares) on its land, the hex steps to the nearest rival
    capital, and whether any rival can be reached over land.
17. **Market fading**: a civ's market never fades (`fade` keeps every civ's market as well as
    `End.centres`, which are the same markets).
18. **The page's orders**: buttons in the selection panel and right-click on the map, nothing
    else. `window.game` keeps read-only access (view, layout, save, the map view) and loses
    `act`, so a headless script can find where to click but can't order through it.
19. **The digest's scripted game.** `sansfiat digest --lead` and `node cross-build.mjs … lead`
    lead the first civ and give the same orders from the same rule, so native and wasm are
    compared through orders as well as turns (build-2 writes the rule).

## Folded in from #31

- `node www/check.mjs` is still not in CI (the runner can't push workflow files). Build-3
  extends it to cover founding and orders; the review names the missing CI step again in the
  PR description.
- Centres were hidden under clans on busy maps until drawn last with a casing: capitals,
  civ land outlines, units and routes are drawn after clans, with a casing.
- The legend overlays the map's bottom-left: it gains founding's marks and folds itself shut
  when the pick appears (it stays toggleable), so it doesn't cover the pick or a capital.
- The greedy centre set flickers, so the feed may show more "becomes a centre" lines than
  three. `CivFounded` lines name the three that count.
- #31's review drove headless Chrome over the DevTools protocol and read screenshots; this
  piece's review does the same, with right-clicks through `Input.dispatchMouseEvent`.

## Tests the piece must have

- Founding: three civs on the centres with names, colours, capital on the market tile and
  30 grain; joining by cheapest reach with ties to the lower id; a civ with none takes the
  nearest unaligned clan; the rest stay unaligned; `CivFounded` per civ; phase `Choose`;
  `EndTurn` refused until `ChooseCiv`; `ChooseCiv` refused outside `Choose` or for a non-civ.
- Daughters (divide and split) keep their mother's civ; unaligned daughters stay unaligned.
- Store: spoils; levy from fed civ clans only; cap and `lost`; a civ's market never fades.
- Units: making (who gives the person, refusals); feeding and `UnitHungry` once; starving out
  after three turns; disbanding to the nearest civ clan, or lost; moving by the cheapest route
  (a constructed detour), a turn's labour at a time, banking for a dear step, river crossing
  costs, no reading of hidden costs; refusals (another civ's unit, sea, unreachable, watching).
- A save with `ChooseCiv`, `MakeUnit`, `MoveUnit` and `DisbandUnit` replays to the same world
  and the same events.
- Real maps: a small seed run to the end, led, and run 100 more turns: stores within the cap,
  every civ clan's civ exists, no unit off land.
- Native and wasm digests agree through the pick (watching) and through orders (`lead`).
- The page: `check.mjs` covers the pick and orders from the wasm build; build-3's headless
  screenshots show the pick, the top bar, a selected unit with its route, and the tooltip.

## What the later steps need to know

- `cargo test` runs at `opt-level = 3`; whole pre-games take a few seconds each. Tests that
  need a real founding use small seeds that end early (small seed 2 ends at turn 276, 6 at
  308, 9 at 299; medium 8 at 244, large 8 at 201, from #31's sweep) and few of them.
- Hand-built worlds: `core/tests/common/mod.rs` (`flat()`, `big()` radius 12, `put`, `open`,
  `hearth`). A founding test can set `w.end` by hand and call `civs::found(&mut w)`.
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs`; anything else
  through `libm` (clippy enforces it). No `HashMap`; deterministic orders everywhere.
- Pre-game behaviour must not change: no new random draws, no change to existing steps'
  arithmetic. The existing tests (and their seeds) must pass untouched except where a test
  runs past the end (which now watches) or builds `Event`s or `Clan`s by struct literal.
