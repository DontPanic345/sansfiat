# Architecture

The shape the code is growing into. `engine/CLAUDE.md` is the map of what's there; this file
is what each system of a finished game needs from the code, what exists on main, and which
piece builds the rest.

## The engine

One deterministic world that changes only through orders, replays from a seed and a log, and
runs the same in native and wasm. That holds, and it stays the first rule.

| System | Target | On main | Built by |
|---|---|---|---|
| Actors | One `Actor`: people, home, store, territory, routes. Spoil, eat and walk live on it once, and every actor (clan, civ, unit, later farmer, herd, trader) is one | `actors.rs` holds the land half. Spoiling is written four times, eating three, walking twice, across `units.rs`, `clans.rs` and `civs.rs`; the civ's store is a bare basket | M3 H |
| Units | `units/{mod,camp,scout,line,ai}.rs`; a `Unit` with its kind's own state; many per civ; kinds with strengths | `units.rs`, 1,559 lines, one camp and one scout per civ in one flat struct | M3 H (the shape), M3 A (many, of kinds) |
| Orders | `World::order(civ, Order)` validates and applies for any civ. The player's go through `act` and are logged; the AI's and a script's go through the same door, re-derived on replay | Actions act on "the player's camp"; the AI bypasses `validate` | M3 H |
| Land | A civ's land is its claim: a territory it guards as every actor divides labour; clans on it lean to it; people on it count toward Standing | A 2-step ring plus its clans' territories, defined nowhere in the design | M3 L |
| Fighting | A contest rule per tile per turn: losses by strength, entrenchment, retreat, surrender; a siege is a cut line | `hold` picks a holder by guard share; nobody is lost | M3 A |
| Goals | Standing per civ each turn; win and lose rules; a game that ends | None | M3 G (the score), M3 R (the end) |
| AI | A planner per civ giving orders for Standing: claim, guard, raise what it can feed, gift, pillage, cut, answer a cut | A post 3 steps toward a rival and a zone over its own line | M3 R |
| Labour across jobs | An actor splits labour between jobs before each job divides over its tiles | One job per actor | M4 |
| Goods | A basket of goods, some of them food, each with weight, spoiling and a use; sources on tiles; crafts as labour | `Basket = [f64; FOODS]`; everything is eaten | M5 |
| Markets | A clearing that takes any participant's row: clans, the civ, units, traders | Clans, with the civ bolted on as an extra row | M5 |
| Knowledge | Knacks per civ: level, practice, luck, contact | None | M6 |
| Routes | A route staged through rests: campsites, caches, caravanserai; a line reaches as far as its rests | A line is one path from a unit to the capital | M7 |
| Events and reasons | One constructor in `events.rs`; a reason beside the rule that acts | Seven constructors in `world.rs`; reasons hold | M3 H |
| Save | Config plus the order log, versioned; an old save fails with its version | Holds | Keep |
| Scale | Per-clan work flat in the number of clans; a large map under 10 ms a turn | Large runs 29.6 ms a turn | M4, before towns |

## The front ends

| System | Target | On main | Built by |
|---|---|---|---|
| The CLI | `run`, `sweep`, `judge`, `play`, `digest`, `replay`, a module each | `main.rs`, 1,010 lines | M3 G |
| The judge | `sweep --judge`: invariants, health bands and the scores, against controls, on the retro maps; `--json`; thresholds in a file; CI fails on a band | Per-piece sweeps a session has to know to run | M3 G |
| The player | `sansfiat play`: a led civ as text, orders as JSON lines; strategy files; an LLM player and judge | `lead_orders`, hard-coded twice | M3 G |
| The page | An app: screens (start, pick, play, end), a view model from `Sim.view()`, a draw layer per mechanic, one panel shape for whatever is selected, keys, autosave, a Debug menu | Modules grown a piece at a time; `drawCivs` draws everything; a panel per kind on the right | M3 P |
| Art | Placeholder tiles and icons on one atlas that paid art fills; `www/assets/atlas.json` names every cell | Flat colours | M3 P (the atlas), paid art after M5 and M7 (`roadmap.md`) |
| Sound | A sound bus fed by feed events; `www/assets/sounds.json` maps event kinds to files | None | M3 P (the bus), paid sound after M7 |
| Play scripts | One driver, `play.mjs`, with clicks by tile, and a script per touch point | Six scripts repeating the setup | M3 P |

## Rules that hold

- A refactor is digest-safe: `stats::digest` unchanged for every seed. Otherwise it's a rule
  change, with a `SAVE_VERSION` bump.
- Shared actor behaviour goes in `actors.rs`. A new actor is a new state on it, not new
  machinery.
- A brief names the module that should grow, not only the code to reuse.
- A file a session can read whole: split anything past about 800 lines when a piece touches it.
- Floating point goes through `libm`; randomness through `rng::rand`.
