# M3 retro: architecture

The engine on `main` at `ec1e47b` (Armies, #142): `engine/core`, `cli`, `web`, `www`. Measured:
line counts on main and at each M3 merge (`git show <c>:<file> | wc -l`), `git show --stat` of
the five merges, `sansfiat bench` built from each merge, and a scratch profiler (outside the
repo) that times each `TURN` step on the seven retro maps to the pick and 200 turns past it,
and counts the supply ring. Everything else is read from the code and says so where it is
inferred. Scratch files: `C:\Users\fallo\AppData\Local\Temp\claude\f69971be46a41c2f\e693b977-062f-4052-9ba9-f195e179e9e6\scratchpad\architecture\`
(`prof\src\main.rs`, `prof2.txt`, `bench-hist2.txt`).

**Verdict.** The shape H set held through L and A. Units live in `units/` with a state per
kind; every order from the player, the AI, `Lead`, the stories, `play` and the page goes
through `World::order` or `act`; spoil, eat and stride sit on the actor; and L and A each grew
a module their brief named (`claim.rs`, `units/fight.rs`), reusing `actors::divide` for the
guard and the army rather than writing their own. What slipped is small: `units/mod.rs` took
A's raise, join, disband and surrender and is at 781 lines; `cli/src/judge.rs` passed 800
(833) without a split; the guard eats by a third road. M4 needs three things first, and the
order matters. **One: the owner's call on the yardstick.** A tile a day's walk with a turn a
year is not a renaming of `data.rs`: it makes walking 28 times cheaper against work, so an
army crosses the large map inside one turn, and every distance rule re-tunes. **Two: a tile
many clans can share.** `Tile.claim` is one clan; on every retro map no two clans share a
home, and a town can't exist until that changes. **Three: labour across jobs**, which splits
cleanly into a digest-safe clay (a split with one job is `divide` exactly) and the farming
rule on it. Scale is past target: a large map runs 50 ms a turn past the pick, and 61% of it
is the festival choice (`markets::go`), a per-clan search that a town would repeat for every
clan on the same tile. `bench` doesn't see that: it times 200 turns from turn 0, which ends
before the pick on every retro map. The integration harness the owner asks for is small to
build (a `Stage` in core over the `map` command's own text format); its first two cases are
below, and the disband-all one shows it needs a page arm, because the bug is in the page's
order form, not the engine.

## 1. The code's shape against `architecture.md`

### The modules

`core/src`, 9,720 lines (M2's end: 6,736). Files near or past the 800-line rule:

| File | M2 end | H | G | L | P | A (main) |
|---|---|---|---|---|---|---|
| `map.rs` | 1,209 | | | | | 1,236 |
| `units/mod.rs` | (in `units.rs` 1,559) | 572 | 572 | 596 | 596 | **781** |
| `data.rs` | 576 | 576 | 589 | 664 | 664 | 758 |
| `units/scout.rs` | | 458 | 458 | 467 | 467 | 630 |
| `civs.rs` | 423 | 507 | 507 | 515 | 515 | 610 |
| `cli/src/judge.rs` | | | 528 | 781 | 781 | **833** |
| `actors.rs` | 308 | 349 | 349 | 359 | 359 | 359 |
| `orders.rs` | | 245 | 264 | 310 | 310 | 398 |

The rest of core is under 550 (`clans.rs` 542, `judge.rs` 492, `fight.rs` 466, `camp.rs`
419). `cli/src/main.rs` is 79 lines with a module per command; `www` has no file past 764
(`screens/play.js`).

- `map.rs` is deep (one `generate` hides it) and M3 barely touched it (G +16). Leave it.
- `data.rs` is a table, but its notes are growing into paragraphs (`FIGHT`'s note runs to
  four lines, `CLAIM`'s two cite `canvas.md`): +182 lines in M3. CLAUDE.md's rule is one
  `note` beside a number.
- `units/mod.rs` took A's +185: `join`, `raise`, `disband_kind`, `surrender`, `give_up`,
  `take_people`, `strip`, `supply`, `UnitFlat`'s serde and `Counters`. Raising and
  disbanding are a module of their own (`units/muster.rs`), and `strip`/`supply` belong in
  `line.rs` beside the line they use. A named `fight.rs` for its new rule (spec 104 line 95)
  but no home for growing the army itself, so it went to `mod.rs`.
- `cli/src/judge.rs` crossed 800 in A (+52) while A touched it, which the rule says splits it:
  `sweep --judge` and `judge FILE` are two commands in one file.

### Per row of the engine table

| System | Target | On main | Held? |
|---|---|---|---|
| Actors | One `Actor`; spoil, eat, walk once | `Actor::spoil` (clans, civ, camp, scout), `eat_by_diet` (clans), `eat_by_share` (camp, scout), `stride` (camp, scout, `line::turns`), `divide` (clans, camp `hold`, guard `claim::guard_shares`). The guard eats with `market::take` on the civ's store and then its clans' (`units/guard.rs:52-64`), a third eating road | Yes, but for the guard's meal |
| Units | `units/{mod,camp,scout,line,ai}.rs`, a state per kind, many per civ, kinds with strengths | Those plus `guard.rs` and `fight.rs`; `UnitState::{Camp, Scout, Guard}`; `Fighter::{Warriors, Archers}` in a stack; many armies | Yes; `mod.rs` swelled |
| Orders | `World::order(civ, Order)` for everyone | `orders.rs:143`; the AI (`units/ai.rs:59-138`), `Lead` (`script.rs:44`), stories (`story.rs:57`), `play` (`play.rs:177`), the retro sweep (`retro.rs:83`) and the page (`web/src/lib.rs:31`) all go through it. Founding's raises and a fallen civ's guard (`civs.rs:385`) call `units` directly, which is right: they are rules, not orders | Yes |
| Land | A claim the guard divides | `claim.rs`: claim, `guard_shares` through `actors::divide`, `held`, `tile_civ`, paper taken by standing on it | Yes |
| Fighting | Losses, entrenchment, retreat, surrender, siege | `fight.rs`: Lanchester per tile, `entrenched`, retreat, militia, surrender, besieged and fallen capitals. Presence reads the guard's and the army's divide shares, so a thin line is thin in a fight | Yes |
| Goals | Standing; a game that ends | `stats::standing`, `judge::Watch`; no end (R unbuilt; `www/screens/end.js` leaves `over` to R) | Half |
| AI | A planner for Standing | Stand-in claim, `guard_order`, a post and a zone | No (R) |
| Events | One constructor | `Event::new(kind, tile).civ().unit()…` in `events.rs`; `scout_event` (`scout.rs:290`) is a small wrapper | Yes |
| Save | Versioned | `SAVE_VERSION` 7 | Yes |
| Scale | Large under 10 ms a turn | 50 ms a turn past the pick on large 8 (section 5) | No |

The front ends: the CLI is a module per command; `sweep --judge` and `judge FILE` exist with
`--json` (the bands are the players angle's); `play` and `view.rs` are the player; the page
has screens, `model.js`, a file per layer, one panel shape (`panels.js`), the atlas and the
sound bus; play scripts share `play.mjs`. Every front-end row's target holds except the
judge's "CI fails on a band", which the owner set aside (owner.md §5).

### Did the actor model hold as a module through L and A?

`actors.rs` was touched by H (the upkeep) and L (`add_traffic`, +10). A touched nothing in it
and didn't need to:

- **L's guard** divides `GUARD.guarding` a person over the claim with `Walks` and `divide`
  (`claim.rs:129-130`), as the camp divides over its zone. The shares live on `Civ.guard`,
  not on the guard unit, whose territory is set to the claim (`claim.rs:137-156`). That is
  one actor's division stored on another; harmless now, but a second guard per civ would
  need it moved onto the unit.
- **A's fight** reads the division: `fight::presence` spreads an army's people over its held
  tiles by its guard shares and a guard's over the claim by `guard_shares`
  (`fight.rs:45-115`). That is the clay paying: `divide` became where people stand.
- **Walking** for every unit is `stride`; the camp marches at one person's pace
  (`line.rs:72`).
- **Eating** is three ways for three reasons (diet, pack share, the guard's store-then-kin).
  The guard's way is the one that isn't on the actor; `Actor::eat_by_share` on the civ's
  actor, then on each kin clan's, would say the same thing.

### Did one door hold?

Yes, and it paid: the stories, `play`, the judge's replays and the retro sweep were all
built on it in M3 with no second path. The AI's orders are re-derived on replay
(`world.rs:224`), and `check_order` is what `view.rs:252` lists as legal orders.

### Briefs naming the module to grow

M2's rule held where a piece had a new rule: L's spec named `claim.rs` and `actors.rs`
(102 build-1 line 31); A's named `units/fight.rs` (104 spec line 95, build-1 line 47). It
didn't hold for growth of what already existed: A's build-1 says "`units/mod.rs` whole:
`Unit`, `UnitFlat`, `join`, `raise`, `disband`, `step`" as the thing to read, and that is
where raising by kind and surrender went (+185 net). The rule wants a second clause: a brief
that adds to a file over 600 lines names the file to split it into.

### Draft rows for `architecture.md`

Not edited; for the retro to take.

| System | Target | On main | Built by |
|---|---|---|---|
| Actors | (as now) | Spoil, eat and walk on `Actor`; `divide` is the clans', the army's and the guard's. The guard eats from the civ's store outside it | Done in M3 H; the guard's meal with M4's clay |
| Units | (as now) | `units/{mod,camp,scout,line,ai,guard,fight}.rs`; `mod.rs` 781 lines holds raising and disbanding | Split `muster.rs` when next touched |
| Labour across jobs | An actor splits its days between jobs before each job divides over its tiles | One job per clan (`clans::reap`); camp and scout spend `left` down a ladder written in each kind's turn | M4 clay (digest-safe), then the farmer |
| Tiles shared | Many clans on one tile, each with its share of the tile's ground | `Tile.claim: Option<u32>`, one clan a tile | M4, before towns |
| The day | One unit of labour, walking and carrying (`labour.md`) | `CLAN.work` 3.5 a person a turn; a plains step costs 1; a tile is two-sevenths of a person's year | The owner's call, then a rule change of every distance |
| Supply ring | Per civ, the tiles an army's line feeds it from | Computed nowhere; `line::cut_off` is its edge, one army at a time | M4, read-only |
| Stage | A known map, units placed, orders given, outcomes asserted | Fixtures in `tests/common`, `scout.rs`'s local painters, stories on real maps | M4, first |
| Scale | Large under 10 ms a turn, measured past the pick | 50 ms a turn past the pick on large 8; `bench` times only the pre-game | M4, before towns |

## 2. What M4 needs first

### How a clan's turn spends labour today

- `clans::harvest` (`clans.rs:209`) calls `reap` (`:152`) for each clan, which calls
  `actors::divide(people × CLAN.work, routes, yearly)` (`:153`, `actors.rs:178`): the
  festival route is paid first, the rest is split evenly over the territory, and each tile's
  share pays its walk (`walk_cost` at `CLAN.inside` 0.25) before it works. A tile's work takes
  `labour / harvest_labour` food (1 labour a food) up to half its stock, and the whole take is
  scaled down to `surplus_target` (2 × people) less the store. Labour above the target goes
  unspent and unrecorded.
- The same `reap` runs inside the land rule: `value` → `worth` → `reap` for every candidate
  in `best_add` and `best_drop` (`actors.rs:212-262`), so whatever the split costs, `move`
  pays it once per candidate territory per clan.
- Walks are charged per actor, not per person: `divide` subtracts a route's cost once from the
  clan's labour, whatever its size. A clan of 2 and a clan of 20 pay the same for the same
  festival. That favours big clans, and towns will be big.

The units already split labour between jobs, as a ladder in each kind's turn: the camp
marches (`camp.rs:104`), guards (`hold`, `UNIT.guarding` a warrior when deployed), runs its
one-shot orders, forages, supplies (`camp.rs:290-325`); the scout walks, forages, supplies,
searches and refills (`scout.rs:546-605`). Each step takes from `left` and passes the rest
down. The guard has one job; the civ has no labour.

### What "split labour between jobs before each job divides over its tiles" is as a change

A **job** is what a share of labour works on: its tiles, their yearly walks and what a unit
of labour yields on each. The split comes before the division:

```rust
// actors.rs
pub struct Job<'a> { pub walks: &'a [f64] }                 // one entry per tile
pub fn split(labour: f64, routes: &[Route], shares: &[f64], jobs: &[Job]) -> Vec<Vec<f64>>
// routes paid once; what's left × shares[j] goes to job j; then divide(share_j, &[], jobs[j].walks)
```

- **The clay, digest-safe.** `split` with one job and `shares = [1.0]` must call `divide`
  with the same arithmetic in the same order; `reap` calls it. The camp and scout ladders
  become a `Ladder { left }` with `take(want) -> got`, the same subtractions in the same order.
  The guard eats through the actor. `stats::digest` unchanged for every seed proves it, as H
  was proved.
- **The rule, not digest-safe.** The farmer is a second job on the clan: fields on its home
  tile (and river-side ground), a yield per labour that rises with `tame`, no wear. The split
  between jobs is state on the clan (`shares`), moved one notch a turn when the value with the
  notch moved beats the value without, the way `actors::change` moves one tile a turn. That
  keeps the cost of `value` flat: an optimal split solved inside `value` would multiply the
  `move` step's cost by the solver's iterations. `SAVE_VERSION` bumps.
- **Callers.** `clans::reap` (and so `harvest`, `at_target`, `worth`, `value`, `site_value`);
  `camp::hold` and `camp_turn`; `scout_turn`; `claim::guard_shares`. The civ has none. The
  page's clan panel and `view.rs` read the shares to show "gathering, farming".
- **Fields need ground smaller than a tile.** `farming.md` and `labour.md` (drafts in the
  working tree) say a field is days, not a tile, and a tile is far bigger than anyone works.
  So the field job's "tiles" are a share of one tile's ground: `Tile` gains how much of its
  workable ground is in fields and whose, and that is the same change towns need.

### Towns

`Tile.claim: Option<u32>` (`map.rs:44`) is one clan a tile; `actors::edge` takes only
unclaimed tiles; `tests/common`'s `check_territories` asserts no tile in two territories. On
all seven retro maps at the pick + 200, no two clans share a home (measured). A town is many
homes and many fields on one tile, so before towns:

- A home is not a claim. Clans' gathering territories stay exclusive; a home tile can hold
  many homes.
- A tile's field ground is shared by area: a list of (clan, field days) bounded by the
  ground the tile allows.
- Per-clan work that depends only on the home tile (the festival search, below) is done
  once per tile.

This is a rule change and the first piece of M4, or the clay of the farmer piece. It is not
digest-safe if a home stops claiming; it can be staged so that the first step (homes in a
list beside `claim`) is.

## 3. The owner's yardstick (owner.md §2, §3)

### The engine's units today

| Quantity | In `data.rs` | Means |
|---|---|---|
| A person's labour a turn | `CLAN.work` 3.5, `UNIT.labour` 3.5 | A year's work |
| Food a person eats a turn | 1 (`UNIT.food`; a clan's need is its people) | A year's eating |
| Labour to harvest a food | `CLAN.harvest_labour` 1 | A person gathers 3.5 years' food a year, at most |
| A plains step | `TERRAIN` move cost 1; forest and hills 2, marsh 3 | 1 labour |
| A walk there and back once a turn | `walk_cost = 2 × path × weight` (`actors.rs:95`) | Charged once per actor |
| A line | carries `labour × UNIT.carry / line cost`, carry 3, cost `walk_cost(path, 1)` (`line.rs:49`) | 1 labour moves 3 food over a line cost of 1 |
| A pack | `UNIT.pack` 5 turns of food a warrior (`units/mod.rs:401`) | 5 years' food |
| A camp's march | `UNIT.labour` 3.5 a step's cost a turn (`line.rs:72`) | 3.5 plains steps a year |

In days (100 a person a turn, as `labour.md` drafts it): 1 labour is 28.6 days, so a plains
tile costs 28.6 days to cross. The owner's yardstick says 1. **The ratio of walking to working
is 28.6 times what the design wants.** A pack of 5 turns is 500 days of food; the note
`docs/notes/A Day's Walk.md` puts a full load at 30-50 days.

### Renaming or rule change?

- **Renaming, digest-safe:** label every number in `data.rs` with its unit in days or food-
  turns (1 labour = 100/3.5 days; 1 food = a person's year of eating), and let `view.rs` and
  the page show days. No arithmetic changes. Worth doing first: it makes the gap visible in
  every panel.
- **Rule change:** making a tile a day's walk is dividing every move cost by 28.6 against
  work, or multiplying labour by it. Either way every distance rule re-tunes:
  `CLAN.territory_radius` 2 and `inside` 0.25, `MARKET.reach` 3 and `reach_walk` 14, `spacing`
  6, `UNIT.carry`, `UNIT.pack`, the scout's reach, the hidden trail costs. Charging walks per
  person (above) is a further rule change the yardstick implies.
- **The consequence the owner must decide.** At a tile a day and a turn a year, a person walks
  about 100 tiles a turn. The large map is 67 tiles across (radius 33). An army would march
  from any capital to any other inside one turn, and outrun its ring (below) in days. The
  per-turn fight, cut and surrender rules assume a march of 3.5 tiles a turn. So either war
  runs in days inside a turn (a `units` step that loops over, say, ten ten-day sub-steps:
  march, hold, fight, supply, each sub-step), or tiles stay larger than a day for armies, or
  the turn is shorter than a year. The farmer's year and the soldier's day don't both fit one
  clock as the engine stands. This is a design call before it is code, and it decides how big
  M4's yardstick piece is.

### The supply ring as a computation

The line's own rule already defines it. For a warrior, a line of cost `c` (there and back)
brings `UNIT.labour × UNIT.carry / c` food a turn for all his labour, and he eats
`UNIT.food`. The share of his labour the line takes to feed him is
`s = c × UNIT.food / (UNIT.labour × UNIT.carry) = c / 10.5`; `line::cut_off` (`line.rs:55-63`)
is `s > 1`. So per civ:

- **Per tile:** Dijkstra from the capital (later, from every tile a line may start at: the
  civ's stores) over the tiles `line_by` allows (walkable, not a rival's land or held ground),
  path cost `p`; `s(t) = 2p / 10.5`. Inside the ring `s < 1` and `1 − s` of the army's labour is
  free to guard and fight; at the edge it is 0; outside the army lives on pack and forage.
- **Measured** (on main, +200 past the pick, rivals not excluded, so an upper bound): the
  ring is 13 to 50 tiles per civ on the retro maps, about the size of the claims (22 to 58).
  In all but one ring, under half the tiles leave an army half its labour: e.g. large 8, the three civs' rings
  are 22, 15 and 50 tiles, of which 8, 5 and 11 are at `s ≤ 0.5`.
- **Where it lives:** in core, beside the line (`units::line::ring(w, civ) -> Vec<(usize, f64)>`),
  because the rule is Rust's and the page must not copy it (M2's `lead_orders` lesson). The
  page gets it from `Sim.ring(civ)` and draws it as a layer (`map/layers/ring.js`) when an army
  or the civ is selected; `labour.md` asks for that. The judge reads it as a measure: tiles in
  the ring, held tiles in it, and army-turns spent outside it. Read-only, so digest-safe, one
  Dijkstra per civ per call. Small.

## 4. The integration harness (owner.md §1.5)

### What exists

| Where | What it does | What it lacks |
|---|---|---|
| `core/tests/common/mod.rs` | `flat`, `big`, `island`, `put` (a clan), `open` (a market), `centre`, `found`, `three`, `led`, `put_camp`, `hold_land` | Terrain is painted tile by tile; no map you can read |
| `core/tests/scout.rs` | `lead(seed)` (a real map to the pick), `plain`, `paint`, `open_ray`, `until_route_done` | Local to one file; the pre-game reruns per test |
| `core/tests/scout_errand.rs` | The retro maps, the scout routed, 50 turns, a table | A report (`#[ignore]`), no assertion |
| `core/tests/units.rs`, `claim.rs`, `fight.rs` | Built maps, orders through `act`, assertions | Each builds its own set-up |
| `cli/src/story.rs` | Scripted orders a turn on a real map, saved for `judge` and `play` | Asserts only that the save replays (`cli/tests/story.rs`) |
| `core/src/script.rs` | `Lead`: the scripted game, compared native against wasm | A digest, not an outcome |
| `play --game` | A game a line at a time, with a lines file | Driven by a person or an LLM, not asserted |
| `www/check.mjs`, `www/play/*.mjs` | The page's data checks and touch points | Real maps only; the panel's order form is tested by clicking |

### The design: a `Stage`

A module in core, `core/src/stage.rs`, public like `script.rs`, so the core tests, the stories
and the wasm build (for a play script) share it. It reads the map as `sansfiat map` prints it
(`map::ascii`, `map.rs:1187`), so a test can paste a cut of a real map.

```rust
pub struct Stage { pub w: World }

impl Stage {
    // The map
    pub fn map(terrain: &str) -> Stage;            // the `map` command's glyphs, one hexagon; a parser
                                                   // `map::parse` that round-trips `ascii`
    pub fn retro(size: &str, seed: u32) -> Stage;  // a real map at the pick, cloned from a per-process cache
    pub fn hidden(self, layer: &str) -> Stage;     // same layout: digits are trail costs, `.` none; spans by a suffix
    pub fn marks(self, layer: &str) -> Stage;      // same layout: letters name places for `at('A')`
    pub fn food(self, mark: char, basket: Basket) -> Stage;
    // Who is there
    pub fn civ(self, at: char, people: f64) -> (Stage, u32);   // a centre, a market, founding
    pub fn clan(&mut self, tiles: &[usize], people: f64) -> u32;
    pub fn army(&mut self, civ: u32, at: usize, stack: [u32; 2]) -> u32;
    pub fn lead(self, civ: u32) -> Stage;
    pub fn fed(self, civ: u32) -> Stage;           // a control: the store topped each turn, outside the log
    // Orders and time
    pub fn order(&mut self, civ: u32, o: Order) -> Result<(), String>;  // `act` for the led civ, else `World::order`
    pub fn turn(&mut self);
    pub fn until(&mut self, max: u32, done: impl Fn(&World) -> bool) -> u32;  // turns taken, panics at max
    // Reads
    pub fn at(&self, mark: char) -> usize;
    pub fn tiles(&self, glyph: char) -> Vec<usize>;
    pub fn revealed(&self, civ: u32, t: usize) -> bool;
    pub fn trail(&self, t: usize) -> bool;
    pub fn route_cost(&self, unit: u32) -> Option<f64>;
    pub fn people(&self, id: u32) -> f64;           // a clan or unit; 0 when gone
    pub fn store(&self, id: u32) -> f64;
    pub fn holder(&self, t: usize) -> Option<u32>;  // `fight::holder_of` / `claim::tile_civ`
    pub fn events(&self, kind: EventKind) -> usize;
}
```

A staged world is not a save (its set-up is outside the log), so a staged test asserts
outcomes, and a test that must replay starts from `retro` and gives only orders.

### The scout and the mountains (owner.md §1.3)

```rust
#[test]
fn a_scout_checks_every_mountain_that_could_open_its_route() {
    // A valley: the capital A, the target B beyond a range of four mountains.
    let s = Stage::map(r"
          ~ ~ ~ ~ ~ ~ ~
         ~ . . ^ . . . ~
        ~ . . . ^ . . . ~
       ~ . . . . ^ . . . ~
        ~ . . . ^ . . . ~
         ~ . . . . . . ~
          ~ ~ ~ ~ ~ ~ ~")
        .marks(r"
          . . . . . . .
         . . . 1 . . . .
        . . . . 2 . . . .
       . A . . . 3 . B . .
        . . . . 4 . . . .
         . . . . . . . .
          . . . . . . .")
        .hidden(/* 1 and 2 impassable, 3 impassable, 4 a pass at cost 3, span 1 */);
    let (mut s, red) = s.civ('A', 14.0);
    let mut s = s.lead(red).fed(red);
    let scout = units::scout(&s.w, red).unwrap().id;
    s.order(red, Order::Route { unit: scout, from: s.at('A'), to: s.at('B') }).unwrap();
    s.until(60, |w| w.units[&scout].scout().unwrap().built || w.units[&scout].scout().unwrap().route.is_none());
    // Every mountain that could have carried a cheaper route was looked at before it settled.
    for m in ['1', '2', '3', '4'] {
        assert!(s.revealed(red, s.at(m)), "mountain {m} never checked");
    }
    // And the route it found is the pass at 4.
    assert!(s.trail(s.at('4')));
    // The invariant, map-free: when a scout settles, nothing unrevealed could beat its route.
    assert!(units::route_plan(&s.w, red, s.at('A'), s.at('B')).candidates.is_empty());
}
```

The last assertion is a property every scout test can end with, on any map.

### Disband all (owner.md §1.2)

The engine side passes today: `Order::Disband { n: None }` disbands every person
(`units/mod.rs:637`, and `www/check.mjs:487` checks it through `Sim`). The bug is the page's
order form: `panels.js:269` makes the "all" option `{type:'disband', n}` with a count, and the
stepper opens at 1 (`panel.js:131`). So the case has two arms:

```rust
#[test]
fn disband_all_sends_every_warrior_home_at_once() {
    let (mut s, red) = Stage::map(PLAINS_7).civ('A', 20.0);
    let mut s = s.lead(red);
    let army = s.army(red, s.at('A'), [4, 2]);
    let clan = s.w.civs[&red].clan;
    let before = s.people(clan);
    s.order(red, Order::Disband { unit: army, n: None, fighter: None }).unwrap();
    assert_eq!(s.people(army), 0.0);
    assert_eq!(s.people(clan), before + 6.0);
}
```

```js
// www/check.mjs, the page arm: the panel's shape on a staged Sim, not a click.
const sim = Sim.stage(PLAINS_7, { civ: 'A', people: 20, army: [4, 2] });
const shape = panels.shapeOf(model.build(sim, ...), { unit: armyId });
const all = shape.buttons.find(b => b.label === 'Disband…').stepper.options.find(o => o.label === 'all');
sim.act(JSON.stringify(all.make(/* the count the stepper opens at */)));
assert.equal(JSON.parse(sim.view()).units[armyId], undefined, 'Disband all leaves no army');
```

The page arm needs `panels.js`'s shapes importable under Node (they are data; `panel.js`
draws them) and a `Sim.stage(text, …)` that builds a `Stage` in wasm.

### What it costs, and what it relates to

- One build step, digest-safe (tests and a pub module, no rule): `map::parse` with a round-trip
  test against `ascii` on the retro maps; `Stage`; `common/mod.rs`'s `flat`, `put`, `found`,
  `three`, `led` and `scout.rs`'s `plain`, `paint`, `open_ray` rewritten on it; the two cases
  above; `Sim.stage` and the `check.mjs` arm. Inferred size: 400 to 600 lines, about H's
  build-2.
- A per-process cache of `retro` worlds (a `OnceLock` keyed by size and seed) also cuts test
  time: `scout.rs`'s `lead(2)` reruns the pre-game per test today (not measured).
- **Stories** are scripts on real maps for the judge and play to read; a stage is a known map
  with an assertion. They share the "orders a turn" shape: `script.rs` can hold a `Script`
  trait that `Lead` and the stories implement, and `Stage::run(script, n)` runs either.
- **The judge** scores whole games beside a control; a stage asserts one rule's outcome in a
  few turns. A piece's question goes to the judge; a play note's "this didn't work" goes to a
  stage, and stays as a test.

## 5. Scale

### Measured

`bench` on each M3 merge, built from `git archive` (this machine, one pass; a second pass
within 10%). Best of nothing; ms a turn for seeds 1, 2, 3, 200 turns from turn 0:

| Size | H `a8c1d49` | G `5496695` | L `8eb7081` | P `a8d8d3c` | A `ec1e47b` |
|---|---|---|---|---|---|
| small | 3.4 / 7.3 / 3.7 | 3.3 / 7.3 / 3.7 | 2.7 / 4.0 / 2.3 | 2.7 / 4.0 / 2.2 | 2.7 / 4.1 / 2.4 |
| medium | 11.0 / 5.9 / 5.9 | 10.9 / 5.7 / 5.7 | 6.9 / 4.0 / 3.1 | 6.8 / 4.0 / 3.4 | 7.0 / 3.9 / 3.1 |
| large | 20.2 / 12.7 / 23.0 | 20.6 / 13.3 / 23.4 | 13.1 / 8.9 / 14.8 | 12.5 / 8.1 / 15.1 | 12.3 / 8.5 / 15.3 |

L made the pre-game a third faster (festivals closer, `reach_walk` 20 → 14, `data.rs:295`), and
nothing since moved it. H's own bench (`relay/specs/100/bench.md`) gave small 1.56 / 3.47 /
1.71 on its runner; this machine is about 2.2 times slower, so compare columns, not machines.
An earlier run during another angle's cargo build gave 4 to 6 times these; bench numbers in a
PR need the machine quiet.

**`bench` never reaches the main game.** It runs 200 turns from turn 0; the retro maps pick at
turn 237 to 431. The profiler times each step to the pick and then 200 turns past it:

| Map | Clans | Pre-game ms a turn | Main ms a turn | `go` | `ai` | `move` |
|---|---|---|---|---|---|---|
| small 2 | 148 | 4.6 | 8.5 | 5.0 | 1.3 | 1.0 |
| small 3 | 88 | 2.5 | 4.9 | 2.6 | 1.0 | 0.7 |
| small 6 | 116 | 4.2 | 6.9 | 4.1 | 1.2 | 0.9 |
| small 9 | 107 | 3.6 | 7.2 | 3.5 | 1.7 | 0.9 |
| medium 8 | 272 | 9.4 | 15.5 | 10.2 | 1.9 | 1.8 |
| medium 9 | 185 | 5.3 | 11.7 | 5.2 | 2.7 | 2.3 |
| large 8 | 636 | 26.1 | 50.4 | 30.7 | 9.5 | 4.7 |

A second run gave the same to within 4%. The main game is about twice the pre-game: the AI
step arrives, and `go` grows by half. On large 8, `go` is 61% of a turn and `ai` 19%; harvest,
eat, trade and units together are under 5 ms.

### Which loops are per clan × per tile

- **`markets::go`** (`markets.rs:109`): each clan runs `worths` (`:65`), a `cost_map` out to
  path cost 7 kept in a `BTreeMap` (`:72`, `land.rs:96`), prices every festival in reach with
  `gain` (a demand calculation per festival), then a full `path` to its chosen festival
  (`:122`). About 48 µs a clan on large. Everything but `gain` depends only on the home tile,
  so a town of k clans repeats it k times. Done per home tile per turn, with a `Vec` in place of
  the `BTreeMap`, it stays flat in clans on a tile.
- **`clans::adjust`** (`move`): `actors::change` evaluates `value` for every edge tile and
  every droppable tile, and each `value` builds `Walks` (an O(n²) scan, `actors.rs:115`) and
  runs `reap`. Per clan × edge × territory². Small territories (mean 2.6 to 4.0 tiles) keep it
  cheap; a farmer whose job split is searched inside `value` would multiply it.
- **Per market × every clan**: `trade` (`markets.rs:161`), `people` (`:252`) and
  `civs.rs:224` each filter all clans per market. M × C; towns raise C on the same M. An index
  of clans by market built once a turn makes it C.
- **`units::ai`**: per AI civ a turn, `post` runs `path_between` to every rival capital
  (`ai.rs:20-26`), full-map searches, plus the claim's checks. Flat in clans, heavy in tiles;
  inferred to be most of the 9.5 ms on large (not profiled below the step).
- **Per unit**: a camp's turn finds its line up to five times (`line`, `cut_off`, `cut_by`,
  and `supply` once per call: `camp.rs:233-325`, `units/mod.rs:719`). Nine units today; many
  armies will feel it.

### What towns will do

Towns put many clans on few tiles. Per-clan work that depends only on the tile (the festival
search, the walks of a home's fields) is the cost that grows; per-clan work on the clan's own
territory doesn't change per clan. With homes shared and `go` per tile, a large map's main turn
falls to roughly 20 ms (inferred: `go` at a third of its cost, the rest unchanged); under 10
also needs `ai`'s rival paths cached per turn. Measure it with a `bench --after N` that times
the turns past the pick, which the target row should name.

## For other angles

- **game:** the scout's mountain case (section 4) is the owner's §1.3 written as a test; the
  assertion that ends it (`route_plan(...).candidates.is_empty()` when the scout settles) is
  the rule the scout broke. Disband all is a page bug (`panels.js:269`), not an engine one.
- **game / design:** a tile a day's walk with a turn a year puts an army's whole campaign
  inside one turn. `labour.md` should say which clock war runs on before M4's yardstick piece
  is cut.
- **players:** the supply ring is a judge measure for free once `line::ring` exists (tiles in
  the ring, armies' turns outside it). The judge's `sweep` plays past the pick, where the
  engine is twice as slow as `bench` says.
- **briefs:** "a brief names the module to grow" held for new rules (L, A) and not for growth
  of a big file (A's raising went into `units/mod.rs`). Add: a brief that adds to a file over
  600 lines names the file to split it into. A bench in a PR needs a quiet machine and the
  turns past the pick.
- **cost:** H's split held the read size down for L and A; `units/mod.rs` at 781 and
  `cli/src/judge.rs` at 833 are the next big reads.

## Ideas

| Idea | Source | Value | Cost | Take, try, later or drop |
|---|---|---|---|---|
| The owner decides which clock war runs on under the day yardstick (sub-steps of days in a turn, larger tiles for war, or a shorter turn) before the yardstick piece | Section 3; `labour.md` | High | An owner's hour | Take |
| Label every `data.rs` number with its unit in days or food-turns; the view shows days | Section 3 | Medium | Small, digest-safe | Take |
| A `Stage` in core over `map::parse`, the two owner cases, `Sim.stage` and a `check.mjs` page arm | Section 4; owner.md §1.5 | High | One step, digest-safe | Take, first in M4 |
| Fix Disband all in the page's order form (the "all" option sends `n: null`) | `panels.js:269` | Medium | Tiny | Take |
| `actors::split` with jobs, and a `Ladder` for the camp and scout, digest-safe; the guard eats through the actor | Section 2 | High | One step | Take, as M4's clay |
| The farmer's job split as state moved a notch a turn, not solved inside `value` | Section 2 | High | Part of the farming piece | Take |
| Homes that don't claim; a tile's field ground shared by area | Section 2, Towns | High | Medium, a rule change | Take, before towns |
| `markets::go` per home tile per turn, with a `Vec` cost map | Section 5 | High | Small, digest-safe if the order of draws holds | Take |
| An index of clans by market built once a turn (`trade`, `people`, `civs.rs:224`) | Section 5 | Medium | Small, digest-safe | Take, with the above |
| Cache the AI's rival paths per turn | Section 5 | Medium | Small, digest-safe | Try |
| `bench --after N`: time the turns past the pick; the Scale row names it | Section 5 | Medium | Tiny | Take |
| `line::ring(w, civ)`; `Sim.ring`; a `ring.js` layer; a judge measure | Section 3; owner.md §3 | High | Small, read-only | Take |
| Walks charged per person walking, not per actor | Section 2; `actors.rs:178` | Medium | A rule change | Later, with the yardstick |
| Split `units/mod.rs` (`muster.rs`; `strip`/`supply` to `line.rs`) and `cli/src/judge.rs` (sweep and file) | Section 1 | Medium | Small, digest-safe | Take when next touched |
| Brief rule: adding to a file over 600 lines names the file to split it into | Section 1 | Medium | Tiny | Take |
| The guard's shares on the guard unit rather than `Civ.guard` | Section 1 | Low | Small | Later, with a second guard |
| A `Script` trait that `Lead`, the stories and `Stage::run` share | Section 4 | Low | Small | Try |
| `data.rs` notes back to one line each | Section 1 | Low | Tiny | Take |
