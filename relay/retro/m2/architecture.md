# M2 retro: architecture

The engine on `main` at `70454be` (after #94): `engine/core`, `cli`, `web`, `www`. Measured: line
counts, `git show --stat` of each piece's merge commit, `git log` per file, one `cargo test` run.
Everything else is read from the code and is said to be inferred where it is.

**Verdict.** The actor model held as a *shape* but not as a *module*. Every new actor (camp,
scout) carries an `Actor` and reuses its two good tools (`Walks`/`divide`, `walk_cost`), and F's
zone really is the camp's territory divided as every actor divides its labour. But `actors.rs`
was never touched after #59 (`git log -- engine/core/src/actors.rs` is that one commit): what
the camp and scout share beyond the land (spoil, eat, hunger, pack, march, line) was written
into `units.rs` instead, twice in places. `units.rs` took B, C and F (693 + 555 + 294 lines
added) and is now 1,559 lines, 23% of `core/src`, with one flat `Unit` struct that is a union
of camp and scout fields. The rest of core is in fair shape. M3 should open with one small,
digest-preserving refactor piece (shape and steps below), and fold the bigger seams (goods,
labour across jobs, action addressing) into the feature pieces that first need them.

## 1. The modules

`core/src`, 6,736 lines. "pub" is the count of public items, a rough width of the interface.

| Module | Lines | pub | Owns | Depth |
|---|---|---|---|---|
| `units.rs` | 1559 | 37 | camp (raise, disband, march, pack, line, cut, zone, hold, forage, pillage, AI post and zone), scout (route plan, search, reveal, trails), `Counters`, every `check_*` for orders | Shallow and wide: two actors and the AI in one file; 12 fns called from `web`/`cli` |
| `map.rs` | 1209 | 17 | generation: terrain, closed country, rivers, floods, hidden trail costs, `ascii` | Deep: `generate(radius, rng)` hides ~1,100 lines. Untouched by M2 except C's hidden costs. |
| `world.rs` | 656 | 26 | `World`, `Action`, `validate`/`act`, `TURN`, save/replay, `Event` and its wording, notes, alerts | Deep core (`act`, `replay`) wrapped in a shallow event layer: 7 near-identical `*_event` constructors (world.rs:228–350), one added per piece |
| `data.rs` | 576 | 56 | every number, `Config` | A table, as intended |
| `clans.rs` | 532 | 15 | seeding, `reap`/`value`, move, harvest, eat, dividing | Deep; the one real client of `actors::change` |
| `civs.rs` | 423 | 13 | founding, levy, keep, store upkeep, territory, cards | Middling. A `Civ` is *not* an `Actor`: its own `store`, spoiled in its own line (civs.rs:345) |
| `markets.rs` | 401 | 14 | reach, `go`, `trade`, `people`, form/fade | `trade` (markets.rs:142–228) inlines the levy, the civ's gift, posting and trade leaning: deep for callers, poor locality |
| `pregame.rs` | 343 | 7 | the pre-game's end | Deep |
| `actors.rs` | 308 | 20 | `Actor`, `Route`, `walk_cost`, `Walks`, `divide`, `change`/`best_add`/`best_drop`, `recentre`, `trim` | Deep for the land, empty for everything else: no store, eating or moving behaviour |
| `land.rs` | 301 | 14 | neighbours, `step_cost`, `cost_map`, `path`, regrow, wear, trails/taming (`hold`) | Deep |
| `stats.rs` | 152 | 8 | summary, reports, `digest` | Fine |
| `market.rs` | 116 | 6 | `clear` | Deep: one call, conserved goods, tested |
| `leaning.rs` | 84 | 3 | gift, gift leaning, the `lean` step | Small and clean (D's whole engine side) |

The front ends: `cli/src/main.rs` 1,010 lines (each piece added a sweep mode and orders to
`lead_orders`); `web/src/lib.rs` 346 (`view()` is 110 lines of hand-built JSON documented in a
30-line comment); `www/map-view.js` 1,196 (`drawCivs` alone is map-view.js:741–946),
`main.js` 840 (40 top-level functions), `select.js` 336, `feed.js` 217, `check.mjs` 406, six
`play-*.mjs` 721.

### Where M2's pieces put their code (merge commits)

| Piece | Biggest engine additions |
|---|---|
| A #59 | `actors.rs` +308 (new), `clans.rs` rewritten, `markets.rs` +251, `land.rs` +77 |
| B #68 | `units.rs` +693, `civs.rs` +108, `world.rs` +101, `cli` +162 |
| D #82 | `leaning.rs` +77 (new), `markets.rs` +51, `civs.rs` +37, `cli` +136 |
| C #75 | `units.rs` +555, `pregame.rs` +154, `civs.rs` +86, `land.rs` +74 |
| F #89 | `units.rs` +294, `cli` +84, `main.js` +74 |
| E #94 | `world.rs` +169, notes spread over six rule files, `markets.rs` +60 |

### Did the clay become the one model?

- **Reused, as designed:** the camp and the scout are `Unit { actor: Actor, .. }` with
  `Deref` to it; the camp's line is an actor `Route` priced by `walk_cost` (units.rs:425); F's
  `hold` builds `Walks` over the zone and calls `actors::divide` (units.rs:603–604), the way
  `actors.md` says an army divides its guarding. Clans, markets and civs price routes with
  the same `walk_cost`. That part of the clay paid.
- **Parallel machinery, measured:**
  - Spoiling is written four times: clans.rs:246, civs.rs:345, units.rs:534 (camp),
    units.rs:1195 (scout).
  - Eating and hunger: `clans::eat`, the camp's tail of `camp_turn` (units.rs ~770–790) and
    `scout_turn` (units.rs ~1235–1255) each take from the store, count `hungry` and starve.
    The camp and scout copies differ only in what starving does.
  - Walking: the camp's `walk` (units.rs:446) and the scout's `march` (units.rs:1014) both
    spend banked labour on steps along a path.
  - The line: `camp_turn` and `scout_turn` both rebuild `actor.routes` from `line()` by hand.
  - The civ's store is a bare `Basket` on `Civ`, not an actor's store; D made the civ a
    trader at its festival by appending it as an extra row of the clearing
    (`c.after[ids.len()]`, markets.rs:205), not through anything an actor has.
  - Basket helpers live in `units.rs` (`take`, `total`, `add`), and `markets.rs:17` and
    `leaning.rs:8` import `units::take`: the dependency points the wrong way.
  - `fn steps(w, a, b)` (hex distance between tiles) is defined in five rule files plus
    `map.rs`.
- **The `Unit` struct is a union:** 23 fields, of which `route`, `heard`, `blocked` are only the
  scout's and `held`, `guard`, `full`, `cut_by`, `auto_*`, `resupply`, `forage`, `pillage`,
  `foraging`, `supplying` only the camp's (units.rs:67–113). `UnitKind` is matched by hand in
  `step` and filtered with `kind == Camp` in `hold`, `ai`, `camp()`.

Why (inferred from the specs): #29's spec said "the camp is a `Unit` … built on `Actor`" and
#28's said the scout should reuse "the camp's code in `units.rs` (forage, line, eat, hunger)".
Both were read as "same file, same struct", and nobody's brief owned `actors.rs`. A Clay and
Kiln piece leaves the clay without an owner once it merges.

## 2. Deepening opportunities, ranked

Ranked by what each saves the next cold session, against what it costs. "Digest-safe" means
the change must leave `stats::digest` unmoved for every seed, so `cli/tests/save.rs`' pinned
digests and `cross-build.sh` prove it with no new tests.

1. **Split `units.rs`; give units a kind-specific state.** `units/mod.rs` (the `Unit`, raise,
   disband, `leader`/`own`, `step` dispatch), `units/camp.rs` (march, `camp_turn`, zone,
   `hold`), `units/line.rs` (`line`, `line_by`, `cut_off`, `cut_at`, `cutter`), `units/scout.rs`
   (`Plan`, `route_plan`, the search), `units/ai.rs` (`post`, `ai`, `ai_zone`). The interface:
   `Unit { id, civ, actor, banked, hungry, state: UnitState }` with `enum UnitState { Camp(Camp),
   Scout(Scout) }`. Saves are config plus the action log, so the struct's shape doesn't touch
   saves; the view's JSON is kept by `#[serde(flatten)]` on the state. A cold session working on
   the scout reads ~500 lines instead of 1,559. Cost: one session, digest-safe. **Take.**
2. **Put the shared actor upkeep on the actor.** `Actor::spoil()`, `Actor::eat(need) -> f64`
   (share covered), and one `walk_along(path, banked, labour)` used by camp, scout and later any
   carrier; basket helpers (`take`, `total`, `add`, `sum`) move to `market.rs` beside `Basket`
   (or a small `goods.rs`); one `steps`/`dist` on `World` or in `land.rs`. Hunger stays with each
   kind (what starving does differs), but the count does not. Deletes four spoils, three eats,
   two walks; the next actor (porter, herd, trader) gets them free. Cost: part of the same
   session as 1; digest-safe if the arithmetic order is kept (floating-point: say so in the
   brief). **Take.**
3. **One event constructor.** Replace the seven `*_event` helpers with
   `w.emit(Event::at(kind, tile).civ(c).unit(u).clan(k).other(o))` and move `Event`,
   `EventKind` and `text` to `events.rs` (world.rs drops to ~350 lines). Every piece so far
   added a helper or a field. Cost: small, digest-safe. **Take.**
4. **`lead_orders` into core.** It is written twice, in Rust (cli/src/main.rs:263) and JS
   (`cross-build.mjs` `leadOrders`), and B, D, C and F each edited both (their briefs say
   "mirror it"). Move it to `core::script::lead(w, &mut pick)`, expose it as `Sim.lead()`, and
   let `cross-build.mjs` call it. That still compares native and wasm (the script runs inside
   each build). It is also where the players angle's strategy files would plug in. Cost: small,
   digest-safe. **Take.**
5. **Actions address a civ and a unit.** `Action::MoveCamp { to }`, `Zone`, `Raise`,
   `SetRoute` … act on "the player's camp" or "the player's scout" (`units::camp(w,
   w.player)`), and the AI bypasses `validate` entirely (`units::ai` calls `raise`,
   `order_move`, `set_zone` directly). Draw the seam at `World::order(civ, Order)`, which
   validates for that civ and applies; `act` logs player orders through it, and the AI's
   orders go through it unlogged (they are re-derived on replay). Then a second camp, a pack
   train or an automatic player are just another caller. Cost: medium; changes the save's
   action JSON (bump `SAVE_VERSION`) and the page's order calls. **Fold into the first piece
   that needs it** (automatic players, or more than one camp).
6. **Market participants.** `markets::trade` builds the clearing's rows from clans, then
   bolts on the civ (levy before, gift and posting inside, trade leaning after). Units trading
   at markets, traders between markets and crafters will each want a row. The interface: a
   participant brings a basket and takes its `after`; the market neither knows nor cares what
   it is. Cost: medium; digest-safe if the row order is kept. **Fold into the trade piece.**
7. **Goods are not all food.** `Basket = [f64; N]` with `GOODS = FOODS` (data.rs:4–5), and
   about 30 places sum a basket as food (`total(&store)`, `iter().sum()`): eating, the pack's
   room, the levy, keep, posting. `utility` (market.rs:19) and `nutrition` (clans.rs:119) take
   every good as food. Obsidian, salt or beads would be eaten. The seam: a `FOOD` mask or range
   and `food(&basket)`, used wherever food is meant. Cost: medium, touches most rule files.
   **Fold into the first materials piece, as its clay.**
8. **Labour across jobs.** `actors::divide` splits one activity over tiles. Farming, crafts,
   herding and carrying need an actor to split labour between jobs before each job divides its
   share over its own tiles or routes. That is the actor model's next real extension, and the
   one place a design decision is needed (owner, `actors.md`). **Fold into the farming piece,
   as its clay, with `actors.rs` named as that piece's to own.**
9. **Names that collide.** `land::hold` (trails and taming, the `hold` step) vs `units::hold`
   (held ground); `Tile.held`/`unheld` (a clan holding grain) vs `Unit.held` (an army holding
   ground); `map::Zone` (closed country) vs the camp's zone; `actors::divide` (labour) vs
   `clans::divide` (a clan splitting); `units::walk` vs `map::walk`. Each makes grep lie to a
   cold session. Cost: small, digest-safe. **Take, with 1–4.**
10. **`www` layers.** Each piece added 50–85 lines to each of `main.js`, `select.js` and
    `map-view.js`; `drawCivs` draws territory, zones, held tiles, camps, lines and scouts in
    one 200-line method. A layer per mechanic (`drawZones`, `drawLines`, `drawScouts`), each
    fed its slice of `view()`, would let a page step read one. Cost: small to medium, checked by
    `play-*.mjs` screenshots, not a digest. **Later**, unless M3's pieces are page-heavy.

## 3. What makes the code hard for a cold build session

- **Files too big to read whole.** `units.rs` is 61 kB (~15k tokens), `map.rs` 47 kB,
  `map-view.js` 45 kB, `cli/src/main.rs` 42 kB. All of `engine/` that a session might read is
  about 500 kB, which is the 100–180k tokens M2's proposal says each session reloads. Splitting
  `units.rs` helps the pieces most likely to come next (anything about the army or scouting).
- **One change, many places.** A new army mechanic touches `units.rs`, `world.rs` (Action,
  validate, act, EventKind, text, an event helper), `data.rs`, `civs.rs` (`Counters` live on
  `Civ`), `web/src/lib.rs` (view and its doc comment), `www/check.mjs`, `main.js`, `select.js`,
  `map-view.js`, `feed.js` (KINDS), `cli/src/main.rs` (sweep, `lead_orders`) and
  `cross-build.mjs`. F's stat touched 22 engine files. Opportunities 3 and 4 remove two of
  these; the rest is the honest cost of a vertical slice.
- **Tests at seams.** Coverage is broad (229 tests, measured) and tests go through `World` and
  `end_turn`, which is the right test surface. Gaps: no test of the actor upkeep as such
  (there is no such module), and the scout's and camp's hunger are tested separately because
  they are written separately. Fixtures are copied: `markets3`/`found`/`three` in
  `tests/alerts.rs`, `civs.rs`, `units.rs` and `pregame.rs`; `total` in four test files;
  `tests/common/mod.rs` has neither. Test run: 21 test binaries, slowest `closed.rs` 17 s and
  `pregame.rs` 15 s (measured, one run, warm build); the wall time is mostly building and
  linking 21 binaries.
- **`engine/CLAUDE.md`.** Accurate on the spot checks I made (`levy` called by `trade`,
  `check_*` from `validate`, `lead_orders` mirrored, the sweep flags); not checked line by line. But M2's own rule, "Layout rows cut to one clause each",
  didn't hold: the `units.rs` row is ~130 words and the `www/` row ~200, because each piece
  appended to its row. After the split those rows should become one per file. It doesn't say
  that `actors.rs` is where shared actor behaviour goes, which is the sentence that would have
  stopped the parallel machinery; "Adding a mechanic" should gain it.

## 4. What the roadmap's next systems need first

| System | Needs | Which opportunity |
|---|---|---|
| Farming | an actor splitting labour between gathering and farming; fields as territory that doesn't wear | 8 (labour across jobs); `actors::change` already decides how many fields |
| Crafts and materials (obsidian, salt, beads, weaving) | goods that aren't food: deposits on tiles, a non-food utility, carrying weight; crafters as actors | 7 first, then 2 (store and eat on the actor) and 8 |
| Pack animals and herding | a carrier actor that walks a line and has its own pack; herds as a store that grows and eats | 2 (shared walk/eat/spoil), 1 (a new `UnitState`), 5 if they are separate units with orders |
| Trade between markets | traders as actors with two-market routes; markets taking any participant | 6, then 2 |
| Fighting and sieges | several camps per civ, orders addressed to a unit, camps meeting on tiles; `hold`'s contest rule (units.rs:584) is already the pairwise place a fight would decide | 5 first, then 1 (camp state grows); `Counters` should move from `Civ` to `units` |

The common thread: every one of these is a new actor or a new job for an actor. If the shared
behaviour isn't in `actors.rs` by then, each will write its own eat, spoil and walk, as C did.

## 5. Recommendation

Open M3 with one refactor piece, before the feature pieces, and keep it digest-safe so its
proof is mechanical. Fold 5–8 into the feature pieces that first need them, each naming
`actors.rs` (or the new goods code) as its clay.

Shape: **Henry Ford.** Nothing to measure or tune, every change is settled here, and it splits
by area. Two build steps and a review; I'd put both builds on **Sonnet medium** (each reads
`units.rs` whole and must keep floating-point order; Haiku risks a moved digest it can't
explain) and the review on **Haiku high** (the checks decide; it reads the diff for anything
that changes a rule). Spec on the lowest the replacement for the coin allows: there is little
to design.

Draft piece:

> **Goal:** The engine's shared actor behaviour lives on the actor, and the army's code reads
> as camp, scout, line and AI, so the next actors (farmer, porter, trader, herd) are built on
> them rather than beside them. No rule changes: every seed's digest is what it was.
>
> **Shape:** Henry Ford: a refactor with settled parts, split by area.
>
> **Design:** docs/design/actors.md
>
> **Decided:**
> - `stats::digest` is unchanged for every seed and size; `SAVE_VERSION` and the pinned
>   digests in `cli/tests/save.rs` don't move; `./check.sh` passes, cross-build included.
> - `units.rs` becomes `units/` with `camp.rs`, `scout.rs`, `line.rs` and `ai.rs`; `Unit` keeps
>   what every unit has and holds the camp's or scout's own fields in a `UnitState`. The page's
>   view JSON keeps its shape.
> - Spoiling, eating from a store and walking along a path are one each, on `Actor` in
>   `actors.rs`; clans, the civ's store, the camp and the scout use them. Each keeps its own
>   rule for what hunger does.
> - Basket helpers live beside `Basket`; nothing imports them from `units`. One tile-distance
>   helper.
> - One event constructor; `Event`, `EventKind` and their wording move to `events.rs`.
> - `lead_orders` moves into core and the wasm build exposes it; `cross-build.mjs` calls it
>   instead of mirroring it.
> - Colliding names are renamed: `land::hold`, `Tile.held`/`unheld`, `map::Zone`,
>   `clans::divide`.
> - Shared test fixtures move into `tests/common`.
> - `engine/CLAUDE.md`'s Layout has one row per file, one clause each, and "Adding a
>   mechanic" says shared actor behaviour goes in `actors.rs`.
>
> **In scope:** core, web's view, `cross-build.mjs`, tests, `engine/CLAUDE.md`.
>
> **Out of scope:** any rule or number; actions addressed to a unit; goods that aren't food;
> labour across jobs; the page's drawing.

Steps, if the designer agrees: build-1 the `units/` split, `UnitState`, the actor upkeep and
basket helpers (the big read); build-2 events, `lead_orders`, renames, fixtures, CLAUDE.md;
review.

## Ideas

| Idea | Value | Cost | Rec |
|---|---|---|---|
| A refactor piece opening M3, digest-safe, as drafted above | high | medium | take |
| Split `units.rs` into `units/{camp,scout,line,ai}.rs` with a `UnitState` per kind | high | medium | take |
| Spoil, eat and walk on `Actor`; one each, used by clans, civ store, camp, scout | high | small | take |
| Basket helpers beside `Basket`; one tile-distance helper | medium | small | take |
| One event constructor; `events.rs` | medium | small | take |
| `lead_orders` in core, exposed to wasm; `cross-build.mjs` stops mirroring it | medium | small | take |
| Rename colliding names (`hold`, `held`, `Zone`, `divide`, `walk`) | medium | small | take |
| Shared test fixtures in `tests/common` | low | small | take |
| `engine/CLAUDE.md`: one row per file; "shared actor behaviour goes in `actors.rs`" under Adding a mechanic | medium | small | take |
| A Clay and Kiln piece names who owns the clay after it merges (later pieces' briefs list `actors.rs` as theirs to extend) | high | small | take |
| `World::order(civ, Order)`: actions addressed to a civ and unit; the AI through the same validate, unlogged | high | medium | try, with automatic players or a second camp |
| Market participants: the clearing takes any row; civ, traders, units plug in | medium | medium | later, with the trade piece |
| A food mask and `food(&basket)`; goods that aren't eaten | high | medium | later, as the first materials piece's clay |
| Labour split across jobs on the actor | high | large | later, as the farming piece's clay; owner's design call |
| Move `Counters` from `Civ` to the units module | low | small | later, with fighting |
| `www` map layers per mechanic instead of one `drawCivs` | medium | medium | later |
| One test binary (`tests/all/main.rs`) to cut link time | low | small | try |

## For other angles

- **briefs:** the M2 specs told the scout to "reuse the camp's code in `units.rs`" (#28 spec
  line 15, build-2 brief); reuse became same-file copies. Briefs could name the module that
  should grow, not just the code to reuse.
- **players:** opportunity 4 (`lead_orders` in core) and 5 (`World::order(civ, …)`, the AI
  through `validate`) are the engine seams `players.md`'s session interface and strategy files
  need.
- **cost:** each step reloads ~500 kB of engine source; splitting `units.rs` is the one
  architecture change likely to show up in context use for army and scout steps.
