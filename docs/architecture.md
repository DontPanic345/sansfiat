# Architecture

The shape the code is growing into. `engine/CLAUDE.md` is the map of what's there; this file
is what each system of a finished game needs from the code, what exists on main, and which
piece builds the rest.

## The engine

One deterministic world that changes only through orders, replays from a seed and a log, and
runs the same in native and wasm. That holds, and it stays the first rule.

| System | Target | On main | Built by |
|---|---|---|---|
| Actors | One `Actor`: people, home, store, territory, routes. Spoil, eat and walk live on it once, and every actor (clan, civ, unit, farmer, herd, trader) is one | Spoil, eat and stride on `Actor`; `divide` is the clans', the army's and the guard's, and the fight reads where people stand from it. The guard eats from the civ's store outside the actor | Done in M3 H; the guard's meal with M4 D |
| Labour across jobs | An actor splits its days between jobs before each job divides over its tiles | One job per clan (`clans::reap`); the camp and scout spend `left` down a ladder written in each kind's turn | M4 D (digest-safe clay), then F (the farmer's job) |
| The day | One unit of labour, walking and carrying (`design/labour.md`); the supply ring per civ | `CLAN.work` 3.5 a person a turn; a plains step costs 1, so a tile is two-sevenths of a person's year; walks charged once per actor; the ring computed nowhere | M4 D: labels, walks per person, `line::ring`. The rescale waits on the owner's clock call |
| Units | `units/{mod,camp,scout,line,ai,guard,fight,muster}.rs`; a `Unit` with its kind's own state; many per civ; kinds with strengths | All but `muster.rs`; `mod.rs` at 781 lines holds raising and disbanding | M3 H and A; the split with M4 D |
| Orders | `World::order(civ, Order)` validates and applies for any civ. The player's go through `act` and are logged; the AI's and a script's go through the same door, re-derived on replay | Holds: the AI, `Lead`, the stories, `play`, the retro sweep and the page all go through it | Done in M3 H |
| Land | A civ's land is its claim: a territory it guards as every actor divides labour; clans on it lean to it; people on it count toward Standing | Holds (`claim.rs`); a clan on held land joins within about eleven turns, so people on land add nothing yet | Done in M3 L |
| Fighting | A contest rule per tile per turn: losses by strength, entrenchment, retreat, surrender; a siege is a cut line | Holds (`units/fight.rs`); fires only in the stories, since no AI marches | Done in M3 A; exercised by M4 R |
| Goals | Standing per civ each turn; win and lose rules; a game that ends | Standing and `judge::Watch`; no end | M3 G (the score), M4 E (the end) |
| AI | A planner per civ giving orders for Standing: claim, guard, raise what it can feed, gift, pillage, cut, answer a cut | A stand-in claim, a guard rule, a post and a zone; worth about as much as no orders | M4 R |
| Tiles shared | Many clans on one tile, each with its share of the tile's field ground | A home is not a claim: many clans may live on one tile, each gathering its own claim and sharing the tile's field ground (`homes.rs`) | Done in M4 W |
| Goods | A basket of goods, some of them food, each with weight, spoiling and a use; sources on tiles; crafts as labour | `Basket = [f64; FOODS]`; everything is eaten | M5 |
| Markets | A clearing that takes any participant's row: clans, the civ, units, traders | Clans, with the civ bolted on as an extra row | M5 |
| Knowledge | Knacks per civ: level, practice, luck, contact | None | M6 |
| Routes | A route staged through rests: campsites, caches, caravanserai; a line reaches as far as its rests | A line is one path from a unit to the capital | M7 |
| Events and reasons | One constructor in `events.rs`; a reason beside the rule that acts | Holds | Done in M3 H |
| Save | Config plus the order log, versioned; an old save fails with its version | Holds (`SAVE_VERSION` 7) | Keep |
| Stage | A known map, units placed, orders given, outcomes asserted, with the real engine | Holds: `Stage` and `StageSpec`, `map::parse`, the retro cache; `tests/common` is built on it | Done in M4 S |
| Scale | Per-clan work flat in the number of clans; a large map under 10 ms a turn past the pick, measured by `bench --after` | Large 8 past the pick (`bench --size large 8 --after 50`) is 0.40 of the turn before #156 in the same session: 25.5 and 25.8 ms against 65.3 and 65.2 on the Relay runner, about 2.4 times slower than the machine of the old 23 ms, so about 10 ms there. The festival search runs once per home tile on flat arrays, clans are indexed by market, the AI's paths are cached per turn and the scouts' search is bounded by their labour. The bound is at most 0.45 of the turn before #156, measured in one session | Holds, with towns (#156) |

## The front ends

| System | Target | On main | Built by |
|---|---|---|---|
| The CLI | `run`, `sweep`, `judge`, `play`, `story`, `digest`, `replay`, `bench`, a module each | Holds; `judge` is `sweep_judge.rs` and `game_judge.rs` | Done in M4 S |
| The judge | `sweep --judge`: the scores against controls on the retro maps, `--json`; `judge --compare` with a measured noise floor; `judge FILE` beside the AI and an idle led civ. A report the review reads with a rubric; CI runs no bands | `sweep --judge` (`--json`, `--floor`); `judge --compare` against the noise floor; `judge FILE` beside the AI and an idle led civ; the economy group (stores, levy, largest army, guard) | Done in M4 S |
| The player | `sansfiat play`: a led civ as text, orders as JSON lines; an LLM player and judge on every piece | `play` and the text view; the player and judge prompts in PR 124 | M4 (the Relay change and the play step) |
| The page | An app: screens, a view model from `Sim.view()`, a draw layer per mechanic, one panel shape, keys, autosave, a Debug menu | Holds | Done in M3 P; grown by each piece |
| Art | Placeholder tiles and icons on one atlas that paid art fills; `www/assets/atlas.json` names every cell | Holds | Done in M3 P; paid art after M5 and M7 (`roadmap.md`) |
| Sound | A sound bus fed by feed events; `www/assets/sounds.json` maps event kinds to files | Holds, silent | Done in M3 P; paid sound after M7 |
| Play scripts | One driver, `play.mjs`, a script per touch point, and one that presses every button and checks the world | The driver, the touch points and `buttons.mjs` | Done in M4 S |

## Rules that hold

- A refactor is digest-safe: `stats::digest` unchanged for every seed. Otherwise it's a rule
  change, with a `SAVE_VERSION` bump.
- Shared actor behaviour goes in `actors.rs`. A new actor is a new state on it, not new
  machinery.
- A brief names the module that should grow, not only the code to reuse.
- A file a session can read whole: split anything past about 800 lines when a piece touches it,
  and a brief that adds to a file over 600 lines names the file to split it into.
- Floating point goes through `libm`; randomness through `rng::rand`.
