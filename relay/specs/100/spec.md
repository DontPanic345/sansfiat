# #100 The engine's shape for many units and players: spec

## What it delivers

The engine takes the shape the next milestones need, and the game the player sees does not
change by a digit. `units.rs` (1,559 lines) becomes `units/{mod,camp,scout,line,ai}.rs`; a `Unit`
holds what every unit has and a `UnitState` holds its kind's own. Spoiling, eating from a store and
walking a path are each written once, on `Actor` in `actors.rs`, and the clans, the civ, the camp
and the scout use them. Every order names its civ and unit and goes through one door,
`World::order(civ, Order)`, whoever gives it: the player (through `act`, logged), the AI (directly,
re-derived on replay) and the scripted game (which moves into core, so the wasm build runs the same
code). Events get one constructor and a file. The CLI and the test fixtures stop repeating
themselves.

Nothing about a rule or a number changes. The proof is that the engine's output does not move.

## Shape: Henry Ford

Taken as the piece suggests. Everything is settled below; the digests are the proof, and the units
module, the orders and the rest are the seams. Seven builds, in this order because each is smaller
and safer on the one before:

| Step | Area | Why here |
|---|---|---|
| build-1 | The rest, part 1: the `--state` digest flag, `events.rs`, Basket helpers, one tile-distance helper, the colliding names | Small and mechanical; makes the later steps' call sites cleaner |
| build-2 | The units module: the split into five files and `Unit`/`UnitState` with its serde shim | A pure move; nothing else touches the files while it happens |
| build-3 | Shared actor behaviour: spoil, eat, walk once on `Actor`; the civ an actor | Every turn runs it, so it comes before the orders add more code to the same files |
| build-4 | The orders: `Order`, `World::order`, `Action::Order`, the AI through it, `SAVE_VERSION` 2, tests | The one place a digest legitimately moves (the log's shape) |
| build-5 | The scripted game into core, run by the CLI and the wasm build | Needs the orders |
| build-6 | The page's order calls and `check.mjs` | Needs the orders; the headless-Chrome plays, diffed with main's by `plays.sh`, are its proof |
| build-7 | The CLI split, `tests/common`, `engine/CLAUDE.md` | Last, so nothing else edits the files it reshapes |

## The digests (read before any step)

`stats::digest` is FNV-1a 64 over `serde_json::to_string(world)`, in struct field order, **and the
world's JSON holds `log`**. So:

1. **The world's JSON stays byte for byte what it is.** Every type that is restructured keeps its
   serialised form: field names, order, flatness. A renamed Rust field keeps its JSON name with
   `#[serde(rename = "…")]`. A type whose Rust shape differs from its JSON gets a *flat mirror*:
   a private struct holding the old field list in the old order, with
   `#[serde(from = "Mirror", into = "Mirror")]` on the real type. `Unit` (kind-specific state) and
   `Civ` (the actor) use it. `Event`s are `#[serde(skip)]` on `World`, so moving them changes
   nothing.
2. **The led game's digest moves, once, and only because of the log.** The scripted game's orders
   are in `log`, and the log changes shape (item 12). The design says "the world digests don't
   move": the three watching digests in `cli/tests/save.rs`, and every watching digest of every
   seed and size, do not move. The led game's `stats::digest` does, so the piece proves the rest of
   its world by the **state digest**: the same FNV over the world's JSON as a `serde_json::Value`
   with `log` removed (keys sorted, as `Value` keeps them). `sansfiat digest [--lead] --state` prints
   it (build-1 adds the flag). `relay/specs/100/baseline/` holds both digests of every turn of five
   runs, taken from main before the piece; `README.md` there says how.
   - The led-game constant in `the_save_version_pins_the_rules` is repinned with the version bump
     (build-4), and a fifth assertion pins the led game's **state** digest to its baseline value
     (`46e67d48ffcbe520` at turn 303, seed 2 small), so the test proves the piece changed no rule.
3. **Cross-build** compares native with wasm for the same code, so it passes through the piece; it
   is not a proof against main. The baseline files are.

## Settled (decisions the owner can overrule)

### The units module (build-2)

4. **Files.**
   - `units/mod.rs`: `Unit`, `UnitState`, `UnitKind`, `Counters`, the queries (`camp`, `scout`,
     `pack_cap`, `leader`), raising and disbanding (`givers`, `home_clan`, `join`, `raise`,
     `raise_at_founding`, `raise_scout_at_founding`, `disband`), what the camp and the scout both
     use (`strip`, `supply`, `room`, `rival_of`, `name_of`), and the `units` step.
   - `units/line.rs`: `walkable`, `path_between`, `route`, `line`, `line_by`, `cut_off`, `cut_by`,
     `cutter`, `cut_at`, `turns`.
   - `units/camp.rs`: the camp's turn: `camp_march`, `guarding`, `hold`, `camp_turn`, the zone (`zone_of`, `zone_cap`, `zone_ok`, `set_zone`).
   - `units/scout.rs`: the scout: `Plan`, `route_plan`, the trail pricing, `search`, `reveal`,
     `scout_turn`, `scout_event`.
   - `units/ai.rs`: `post`, `ai`, `ai_zone`.
   The orders' `check_*`/`order_*` functions move with their subject (camp or scout) in build-2 and
   to `orders.rs` in build-4. No file is left over about 600 lines.
5. **`Unit` and `UnitState`.** `Unit` keeps `id`, `civ`, `actor` (with `Deref`/`DerefMut` to it, as
   now), `banked`, `hungry` and `state: UnitState`. `UnitState` is `Camp(CampState)` or
   `Scout(ScoutState)`. `CampState` holds `dest`, `auto_forage`, `auto_supply`, `resupply`,
   `forage`, `pillage`, `cut`, `foraging`, `supplying`, `held`, `guard`, `full`, `cut_by`.
   `ScoutState` holds `route`, `heard`, `blocked`. `Unit::kind()` reads the state;
   `Unit::camp()/camp_mut()/scout()/scout_mut()` give the state or `None`; `Unit::held()` gives the
   held tiles (empty for a scout), for `civs::holder`. Every field a scout has that is a camp's
   (`dest`, `held` …) is read only through the camp's state: a scout does not carry dead fields in
   Rust.
6. **The JSON stays flat** (item 1): a unit still serialises as `id, civ, kind, people, home, store,
   territory, routes, market, dest, banked, hungry, auto_forage, auto_supply, resupply, forage,
   pillage, cut, foraging, supplying, held, guard, full, cut_by, route, heard, blocked`, a camp with
   the scout's fields at their defaults (`route: null`, `heard`, `blocked` false) and a scout with
   the camp's at the values `raise_scout_at_founding` gives them, which nothing changes after:
   `dest: null`, **`auto_forage: true`, `auto_supply: true`**, the five other flags `false`, `held: []`,
   `guard: []`, `full: false`, `cut_by: null`. (Not `Default`: a scout written with `auto_forage: false`
   moves every digest from the founding on. Checked on main's JSON by the spec-review.) `Sim.view()`'s `units` row is that JSON plus the extras it adds; the page reads it
   unchanged.

### Shared actor behaviour (build-3)

7. **Written once each, on `Actor` (`actors.rs`):**
   - `Actor::spoil(&mut self)`: `store = store × (1 − CLAN.spoilage)`, used by the clan (after it
     eats), the civ (before its cap), the camp and the scout (before they march).
   - Eating from the store, in the two ways the game has: `Actor::eat_by_share(need) -> Basket`
     takes `need` in proportion to what the store holds (the camp and the scout), and
     `Actor::eat_by_diet(need) -> f64` eats each food up to the diet share, then the excess at
     `CLAN.excess_value`, leaves the rest in the store and returns what the person got (the clan).
     Each caller keeps its own rule for what hunger does: the clan's `fed_ratio`, growth and
     starvation, the camp's `hungry` count and disbanding, the scout's lost person.
   - `actors::stride(w, from, path, labour, slack) -> (taken, left)`: walks `path` from `from`,
     spending `labour` on steps while each costs no more than `labour + slack`. The camp calls it
     with `banked + UNIT.labour` and slack `0.0`; the scout with its pool and slack `EPS`; `turns`
     with the camp's. The scout marks `walked` on the tiles taken, as it does now. `units::walk`
     and `units::march`' loop go.
   Each must reproduce its caller's float operations in the same order: the digests are exact.
8. **The civ is an actor.** `Civ` holds an `Actor` (`Actor::new(capital, 0.0)` with the civ's store:
   home the capital, `people` 0, territory the capital, no routes, `market: None`), `Deref`s to it as
   `Unit` does, and `civs::upkeep` spoils through `Actor::spoil`
   before the cap. Its JSON keeps `store` where it was and gains nothing (item 1, the flat mirror).
   The civ keeps its own `capital` and `market` (a `u32`) fields, which its call sites read; a
   field of `Civ` shadows the `Actor` field of the same name through `Deref`, so `civ.market` is
   still the founding market. The mirror keeps the `#[serde(default)]`s the fields have now. The civ
   gets no new behaviour: it does not eat, walk or trade differently.
9. **Basket helpers** (`take`, `add`, `total`, `sum`) move to `market.rs` beside `Basket` as free
   functions, once; `units::take`/`add`/`total` and `clans::sum` go.

### The orders (build-4)

10. **`Order`** is in `core/src/orders.rs`, with `World::order(civ, Order) -> Result<(), String>`
    and `World::check_order(civ, &Order)`:
    ```
    Raise { n }            Disband { unit, n: Option<u32> }   Move { unit, to }
    Zone { unit, tiles }   Switches { unit, forage, supply }  Resupply { unit }
    Forage { unit }        Pillage { unit }
    Route { unit, from, to }   ClearRoute { unit }
    ```
    (serde `tag = "type"`, camelCase.) The *design* says "raise names the civ"; the civ is the
    first argument of `order`, so `Raise` carries no `civ` field (a decision). Every other order
    carries its `unit`. Checks as today, taken from the civ and unit given instead of the player's
    camp, in this order and with these refusals:
    1. the phase is `Main`: "the game has not begun" (as `leader` says it now; `check.mjs` matches
       /not begun/ for an order before the pick);
    2. (`act`/`validate` only) the player leads `civ`: "you're watching: you lead no civ" when the
       player leads none, "that civ is not yours" when it leads another;
    3. the unit exists: "{id} is not a unit"; it is `civ`'s: "that unit is not yours"; it is the
       order's kind: "that unit is not a camp" / "that unit is not a scout";
    4. the order's own check (`check_move` and the rest move here, their text unchanged; "you have
       no camp"/"you have no scout" stay only where nothing else says it, e.g. `Raise` has no unit). `SetKeep` and `SetGift`
    stay `Action`s: they set the civ, not a unit, and the AI never gives them.
11. **`Action::Order { civ, order }`** replaces `Raise`, `Disband`, `MoveCamp`, `Zone`,
    `SetSwitches`, `Resupply`, `Forage`, `Pillage`, `SetRoute` and `ClearRoute`. Its wire form is
    `{"type":"order","civ":1,"order":{"type":"move","unit":12,"to":5}}`. `validate` refuses it
    unless the game is in `Main`, `civ` is the civ the player leads and the order checks; `act`
    applies it with `World::order` and logs it. `World::order` itself does not require the player to
    lead `civ` and does not log: the AI calls it. `ChooseCiv`, `EndTurn`, `SetKeep`, `SetGift` are as
    now.
12. **`SAVE_VERSION` becomes 2.** The log's shape changed, so a version-1 save is refused with its
    version by the existing path. `core/tests/save.rs`'s messages and `cli/tests/save.rs`'s
    `VERSION` follow.
13. **The AI gives its orders through `order`** (`units/ai.rs`): `Raise { n: UNIT.start }` when it
    has no camp (an `Err` ignored, as now), `Move` to its post, `Zone` over its line. Not logged:
    a replay re-derives them from the world as it has always done. Validation does not change what
    the AI does: the spec-review instrumented main's `ai` and ran the five baseline runs and the retro
    sweep, and no AI move failed `check_move`'s conditions (land, passable, a path from where the camp
    stands); `ai_zone` already gates on `zone_ok`, and `raise` refuses what `check_raise` refuses (the
    AI runs only with the army on). So the AI's choices are unchanged, and a digest that moves in
    build-4 is a bug in the generalised checks, not a case to handle. The note "posted
    at …" is still written before the move.

### The scripted game (build-5)

14. **`lead_orders` becomes `core/src/script.rs`**: `pub struct Lead { pick: Option<u32> }` with
    `Lead::orders(&mut self, w: &mut World)` — the whole of the CLI's `lead_orders`, using
    `Action::Order`. The CLI's `digest --lead` and the wasm build both call it. `Sim` holds a `Lead`
    and gains `lead_orders()`; `cross-build.mjs` calls `sim.lead_orders()` and drops its mirror
    (the doc comment listing the rules moves with the code). The script's behaviour is unchanged,
    rule for rule.

### The rest (build-1, build-7)

15. **Events.** `Event`, `EventKind` and their wording (`Event::text`, `json`) move to
    `core/src/events.rs` (the alert marking, `alert_key` and `flag_alerts`, stays in `world.rs`), re-exported from `lib.rs` as they are
    (`engine::{Event, EventKind}`). One constructor: `Event::new(kind, tile)` with chainable
    `.clan(id)`, `.market(id)`, `.civ(id)`, `.unit(id)`, `.other(n)`, `.cost(c)`, `.amount(a)`, and
    one `World::emit(event)` that stamps the turn and pushes. The seven constructors on `World`
    go; every call site becomes one `emit`. The JSON of an event is unchanged.
16. **One tile-distance helper**, `land::steps(w, a, b) -> i32`; the private `steps(w, a, b)` copies in `actors`, `civs`, `clans`, `pregame`
    and `units` go, and so do inline `w.tiles[a].hex().distance(w.tiles[b].hex())` where they stand for
    it (`map::Geo::steps` stays: it has no `World`).
17. **Colliding names**, with the JSON kept by `#[serde(rename)]`:
    - `land::hold` (the tending step) → `land::tend`; the step in `TURN` is named `tend`. `units::hold`
      (the army's holding) keeps its name.
    - `Tile.held`/`Tile.unheld` → `Tile.claim_run`/`Tile.free_run` (JSON `held`, `unheld`).
    - `map::Zone` → `map::Country`; `Tile.zone` → `Tile.country` (JSON `zone`; `layout()` keeps its
      `zone` column). Variants unchanged.
    - `clans::divide` → `clans::divide_land`.
    - `units::walk` → gone, replaced by `actors::stride` (item 7).
18. **Test fixtures** in `core/tests/common/mod.rs` (a `three()` world with three civs founded,
    `led()`, `markets3`, `fill_stores`, `found`, `put_camp`, `end_turn`, `total`, `kinds` …), each
    defined once; the duplicated copies in `units.rs`, `civs.rs`, `scout.rs`, `alerts.rs`, `notes.rs`
    and the rest are removed. No test is deleted or weakened.
19. **The CLI** splits into a module per command under `cli/src/`: `main.rs` (dispatch, USAGE,
    `size`), `run.rs`, `map.rs`, `digest.rs`, `sweep.rs`, `retro.rs`, `replay.rs`, `bench.rs`,
    `clear.rs`, and `report.rs` for the shared printers. `sweep.rs` (`Run`, `Point`, `sweep_run`,
    `sweep`) and `retro.rs` (`RETRO`, `retro_maps`, `CivArmy`, `MapRun`, `after_run`, `sweep_after`) are
    two files: together they are about 470 lines today. Output is byte-identical.
20. **`engine/CLAUDE.md`**: one row per file in the layout table, one clause each, including the new
    files; under Adding a mechanic, "shared actor behaviour goes in `actors.rs`"; its item 3 says
    anything the player or an AI does to a unit is an `Order`; the invariant "the world changes only
    through `act`" says "(and the AI's orders, which a replay re-derives)".

## Left unsettled

Nothing. What a build may choose is internal naming where this spec gives none (the mirror
structs, helper names inside a file, where inside a file a function sits) and the order of work
inside a step.

## What proves the piece

The piece's question is *does the game play the same*, so the scores are held to the control, which
is **main before the piece** (`baseline/`). The five scores of `process.md`:

| Score | How it is measured | Passes when |
|---|---|---|
| Standing, Doubt, Decisions, Motion | `sansfiat sweep --after 200 --maps retro` — people by civ, camps, held tiles, clans joined, gifts and the totals — diffed against `baseline/retro-sweep.txt` | Byte-identical |
| Health: the digests | `sansfiat digest` for the four watching runs of `baseline/` against their `.txt`; the led run's `--state` digest against its `.state.txt`, every turn | Identical on every line |
| Health: the save | `the_save_version_pins_the_rules` (three watching digests unchanged, the led digest repinned, the led state digest at its baseline); a save made by this build replays to an equal world | Passes |
| Health: speed | `sansfiat bench 1 2 3` best of three, against the number build-1 takes before it edits | No more than 10% slower |
| Health: the builds | `./check.sh`: fmt, clippy, tests, web build, cross-build (native and wasm agree on five runs, one led), `check.mjs` | Passes |

"Decisions" also means the AI's orders: they are the same orders from the same rules, which the
watching digests (the AI plays every turn of them) and the sweep prove.

The page's proof is the plays: `www/play-camp.mjs`, `play-scout.mjs`, `play-hold.mjs`,
`play-lean.mjs`, `play-read.mjs` each print what they printed on main and exit as they did there.
`bash relay/specs/100/baseline/plays.sh` runs the five (in parallel, about 30 seconds) and diffs each
one's stdout and exit code with `baseline/plays/`, taken from main by the spec-review: five `ok` lines.
Two of them **fail on main** and are held to failing the same way: `play-camp` loses its camp to hunger
on the way back and throws at its line 86 (exit 1), and `play-lean` finds no clan leaning towards the
civ in 30 turns (exit 1). They were written against earlier rules; fixing them is not this piece (an
idea for the owner). Their stdout up to the failure is still the comparison. The plays drive the page
by its buttons and clicks, so they need no edits; the orders the page sends are the wire form of item 11.

### What the review must run

1. `./check.sh`.
2. `bash relay/specs/100/baseline/compare.sh`: the digest comparison for all five baseline runs and the
   sweep diff, six `ok` lines.
3. `bench` against build-1's number.
4. `bash relay/specs/100/baseline/plays.sh /tmp/plays`: five `ok` lines; open the screenshots.
5. Read: that no source file this piece touched is over about 800 lines (`map.rs` and `data.rs`
   aside, which it only renames in; tests aside), that `engine/CLAUDE.md` has a row for every file in `core/src`, `core/src/units`,
   `cli/src` and `web/src`, and that nothing under `relay/runs/` is committed.
6. The decisions below go into the design's Decided ledger: `actors.md` takes items 7, 8 and 10–13.
7. The PR description answers the question: the retro sweep and every digest are unchanged, the
   speed, and the file sizes before and after.

## Tests the piece must have

- build-1: the `--state` flag reproduces `baseline/seed2-r16-t400-lead.state.txt` on the unchanged
  code.
- build-2: a camp's and a scout's JSON keys, in order, equal the list in item 6 (`core/tests/units.rs`).
- build-3: `Actor::spoil`, `eat_by_share`, `eat_by_diet` and `stride` each have a test in
  `core/tests/actors.rs`; a civ's JSON keeps its `store` key where it was.
- build-4: each `Order` is refused for a unit of another civ, a missing unit, and the wrong phase; the
  player's order through `act` is logged and an order for a civ the player doesn't lead is refused; the
  AI's orders are not in the log; a led game's save replays to an equal world; the pinned test as in item
  2.
- build-5: `Lead` run twice on equal worlds gives equal logs; cross-build passes.
- build-6: `check.mjs` sends and refuses the new wire form; `plays.sh` prints five `ok` lines.

## Not in this piece

Any rule or number. More than one camp per civ (the orders name a unit and the code looks a civ's camp up
by `units::camp`, as before). Unit kinds. Goods other than food. `docs/architecture.md`'s table (the
review notes in an idea which rows are now built).

## What the spec-review ran

On main at `25be2c6`, before any build:

- `./check.sh`: green, about 4 minutes.
- `compare.sh` with build-1's `--state` definition patched in for the run (then reverted): six `ok`
  lines. The definition in build-1's brief reproduces `seed2-r16-t400-lead.state.txt` and
  `seed1-r16-t400.state.txt`, and gives `46e67d48ffcbe520` at turn 303 of seed 2 small, led.
- A unit's and a civ's JSON keys from `Sim.state()`: the 27 of item 6 for both kinds; a scout's
  `auto_forage`/`auto_supply` are `true` (item 6 now says so).
- `ai` instrumented on the five baseline runs and the retro sweep: no AI move fails `check_move`'s
  conditions (item 13).
- The five plays, twice: deterministic stdout; `play-camp` and `play-lean` exit 1 on main.
  `baseline/plays/` and `plays.sh` hold them.
