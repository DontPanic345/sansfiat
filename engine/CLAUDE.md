# engine

The Sans Fiat engine in Rust: a world that runs turn by turn and changes only through logged
actions. One engine, two front ends: a CLI and a wasm web page. The game's design is in
`docs/design/`; build what it says.

```
./check.sh                                   # every check CI runs, in order
cargo run -p sansfiat -- map [--seed N] [--size small|medium|large]   # the map as ASCII
cargo run -p sansfiat -- run --seed 3 [--size NAME] [--turns N] [--save game.json] [--why ID]   # to the pick unless --turns; --why prints a subject's reasons
cargo run -p sansfiat -- sweep [seed ...] [--size small|medium|large|all] [--turns N]   # the pre-game, seeds in parallel
cargo run -p sansfiat -- sweep --judge [--json] [--solo]   # the retro maps played (army twice, --no-army once, no claims once; --solo adds each civ claiming alone) and scored by the judge: a report, with no verdict
cargo run -p sansfiat -- sweep --after 200 --maps retro [--no-army] [--no-gifts] [--no-zone] [--lever G]   # the retro maps past the pick, one line per civ and the totals; the --no-* flags are controls
cargo run -p sansfiat -- play --game g.json [--seed N --size NAME [--civ K]]   # a game a line at a time: the file is a save, each call resumes it; stdin lines (orders, actions, end, why, view, map, note); g.lines.jsonl records them
cargo run -p sansfiat -- judge g.json [--json]   # a played game judged turn by turn, beside the same map with every civ on the AI
cargo run -p sansfiat -- replay game.json | bench | digest [--lead] [--state] | digest --game FILE   # replay a save; time turns; digests to compare builds (--state: the world without its log; --game: a saved game's `<turn> <digest>`, to match the page's `Sim.digest()`)
node www/play.mjs "?size=small&seed=2&run=1" --shot pick.png   # the page in headless Chrome; import { open } from it to click
node www/play-<area>.mjs [out-dir]           # leads a civ through one area and screenshots it; one per area in www/
node www/play/all.mjs                        # every script in www/play/ (art, views, ...) with a temp --out: a table of pass and seconds; check.sh runs it
node www/assets/draw-atlas.mjs               # redraws assets/atlas.png and atlas.json from assets/placeholders.js
```

`cargo test` runs optimised (`[profile.dev] opt-level = 3`): whole pre-games are too slow otherwise.

## Layout

| Path | What |
| --- | --- |
| `core/src/world.rs` | `World`, `phase`, `Action`, the `TURN` steps, `act`/`run`/`save`/`replay`, reasons (`World::note`), alerts (`flag_alerts`), `SAVE_VERSION`. |
| `core/src/events.rs` | `Event` and `EventKind`, their wording (`text`), `Event::new` and its setters, and `World::emit`. |
| `core/src/data.rs` | Every rule's numbers, `Config` (with `Claims`: which civs may claim); tune here first. |
| `core/src/map.rs` | Map generation: terrain, water, closed country, rivers, foods, hidden trail costs. |
| `core/src/land.rs` | Neighbours, move costs, `path`, `steps` (the tile distance), and the `regrow`, `wear` and `tend` steps (`tend` wears a trail where a tile's `traffic` reaches `TRAIL.on` and takes it away under `TRAIL.off`). |
| `core/src/actors.rs` | `Actor` and what every actor does once: `spoil`, `eat_by_share`, `eat_by_diet`, `stride` (walking a path on labour), `walk_cost`, `Walks`, `add_traffic` (a clan's people onto the tiles of its walks, home included), `divide` and the add/drop rule `change`. |
| `core/src/clans.rs` | Clans on the actor: seeding, `reap`, `value`, and the `move`, `harvest` and `eat` steps. |
| `core/src/market.rs` | `Basket` and its helpers (`total`, `take`, `add`, `sum`); `clear`: barter exchange ratios by tâtonnement. |
| `core/src/markets.rs` | Festivals (markets in code): reach, the `go` step (`choose` keeps a festival `MARKET.keep_turns` turns and while it is worth over `−stay_margin`), trading, forming and fading, people moving. |
| `core/src/pregame.rs` | The pre-game's end: centres, land cover, `top_up`. |
| `core/src/claim.rs` | The land: a civ's claim (`check`, `connected_part`, `ai_claim`, `set`), its guard's division (`divide`: `Civ.guard`), `held` and `tile_civ` (the civ whose land a tile is), and the `claim` step, where a unit on another civ's paper takes it (`take`). |
| `core/src/civs.rs` | Civs, each an actor (its store is the civ's): `found` (the claims, the camp, the scout, the guard), the `levy` (clans at its market that are its own or on its land), `keep` (with the guard's food), the `civs` step and the pick's `cards`. |
| `core/src/leaning.rs` | Every clan's leanings: fading, leaning by land (`lean`), gifts (`give`), joining at `LEANING.join`, and leaving for a rival it is drawn to with its daughters (`leave`). |
| `core/src/units/line.rs` | What a camp may walk, `path_between`, `route`, the line to the capital (`line`, `cut_off`, `cut_at`) and `turns`. |
| `core/src/units/camp.rs` | `CampState`, the camp's turn (march, `hold`, forage, supply, eat) and its zone (`zone_ok`). |
| `core/src/units/guard.rs` | The guard: `raise` (people from the civ's clans, into one guard at the capital), its turn (eats from the civ's store, hungry, loses people to the own clan) and founding's raise. |
| `core/src/units/scout.rs` | `ScoutState`, the scout's route search (`route_plan`, trails, reveals) and `scout_turn`. |
| `core/src/script.rs` | `Lead`, the scripted game (`digest --lead`, `Sim::lead_orders`): the engine's second player. |
| `core/src/orders.rs` | `Order` (the wire form of a unit's order: `Raise` with its `kind`, `Claim`), `World::order` and `check_order`: every order's checks and its applying, the player's and the AI's. |
| `core/src/units/ai.rs` | The AI's `post`, `ai` (its stand-in claim and guard, `land`) and its zone (`ai_zone`). |
| `core/src/stats.rs` | `summary`, `standing_parts` and `standing` (each civ's share of the people in two parts: its clans and units, and the unaligned clans on its held land), the evidence reports, and `digest` and `state_digest` to compare builds. |
| `core/src/judge.rs` | `Watch`: a game scored turn by turn from its pick (Standing in its two parts, the leader and lead changes, the spread and swings, what changes hands: clans left and joined, claimed tiles taken), and its `Scores` in the five groups plus `land` (trail shares and festivals at the pick and +200, festival changes and flips per 100 clan-turns, each civ's claim, held land and guard at +200 via `civ_lands`; `stats::trails` reads a world). Reads the world, changes nothing. |
| `core/src/hex.rs` | Axial pointy-top hexes. |
| `core/src/rng.rs` | Mulberry32, matching the JS prototypes bit for bit. |
| `core/src/lib.rs` | The crate root: one `pub mod` line per module. |
| `cli/src/main.rs` | The `sansfiat` binary: `USAGE`, the size names, `main` and the dispatch to one module per command. |
| `cli/src/run.rs` | `run` and its `Output` (the report, the JSON or the events), with the events and reasons it prints. |
| `cli/src/map.rs` | `map`: the map as text, with a legend and counts. |
| `cli/src/digest.rs` | `digest`: a turn and its digest per line; `--lead` plays the scripted game, `--state` hashes the world without its log; `--game FILE` replays a save and prints its one line. |
| `cli/src/sweep.rs` | `sweep`: the pre-game over seeds in parallel, its series lines, and the `Run` and `Point` they keep. |
| `cli/src/retro.rs` | `sweep --after`: the retro maps past the pick, each civ's camp and clans, and the controls. `to_pick` and `kept_camp` are shared. |
| `cli/src/judge.rs` | `sweep --judge`: the retro maps judged (the army run twice, the `--no-army` and no-claims controls, and with `--solo` each civ's claim-alone run), the totals in groups (standing, doubt, decisions, motion, land, solo, armies, health), the text and `--json` report. `judge FILE`: a played game replayed under a `Watch` beside its control, its led civ's claim and land beside the control's, the text and `--json` report. |
| `cli/src/play.rs` | `play`: a game file resumed per call (`World::replay`), each stdin line applied through `act`, the save and the lines file (`FILE.lines.jsonl`) written after each. |
| `cli/src/view.rs` | `view::text`: the led civ's text view (standing, store, units, the claim with its held land, paper and guard, clans, reach, places, feed, legal orders), under 80 lines at the pick and +100. |
| `cli/src/replay.rs` | `replay`: a save's world, printed as `run` prints it. |
| `cli/src/bench.rs` | `bench`: the time per turn for each seed. |
| `cli/src/clear.rs` | `clear`: the barter clearing of a basket file (`basket_of`, `clear_file`). |
| `cli/src/report.rs` | The printers the commands share: the summary line, the centres, the report and the civs' text. |
| `web/src/lib.rs` | The wasm-bindgen `Sim` the page drives: `layout()`, `view()`, `act`/`check`, the order form. |
| `www/index.html` | The page's markup: one `<section>` per screen (start, play with its bar, map, feed and bottom panel, end). |
| `www/style.css` | The page's styles, with the phone breakpoint at 760 px. |
| `www/main.js` | The router: loads the wasm, the atlas and the sounds, then shows one screen at a time (`app.show`); `?size&seed` goes to play, a bare URL to start; sets `window.game` for the scripts. |
| `www/app.js` | What the screens share: the wasm data, the atlas, the sound bus, the router and the current screen. |
| `www/screens/start.js` | The start screen: New game (size, seed, 🎲, Start), Continue (the autosave's line), Load a file; an old save's error shows here. |
| `www/screens/play.js` | The play screen: the map, the bar, the panel and the feed; the game in progress (`sim`, model, selection), the turn loop, Skip to the pick, the keys and the tooltip. |
| `www/screens/pick.js` | The pick: the cards over the map, built once per game. |
| `www/screens/end.js` | The end screen: Standing over the game as an inline SVG, Play on, New game; `over` is R's to fill. |
| `www/bar.js` | The top bar: the pre-game's counts, a Standing chip per civ, End turn, Play and speed, the Game, View and Debug menus, the Feed toggle; every number's tooltip is its `title`. |
| `www/save.js` | Autosave (`localStorage['sansfiat.autosave']`): written after each turn and action, once a second while a run goes; refused storage turns it off. `continueLine`. |
| `www/sound.js` | The sound bus: each feed kind's file from `assets/sounds.json`, tried once; `heard` counts kinds; muted until the first click. |
| `www/legend.js` | The map's legend rows, with the closed-country rows when Hidden country is on. |
| `www/select.js` | The selection panel: one function from a selection to `{title, html, actions}`; `unitOrder` and `civOrder` send the order form. |
| `www/model.js` | The view model, `build(sim, layout, rules, prev)`: the parsed `Sim.view()` as `v` plus clan civs, unit stacks, holders, claim counts, Standing and rank, the leader, festival walkers, the player's units and the Standing history. Layers and panels read it. |
| `www/map/view.js` | `MapView`: pan, zoom, picking, the offscreen layer for the static layers, the selection's state, and `LAYERS` (the draw order) with `setLayer(name, on)`. |
| `www/map/geom.js` | Hex corners, a clan's walk, and the clan hue the view and layers share. |
| `www/map/layers/*.js` | One layer per file, `{ name, static, draw(ctx, model, view) }`: terrain (atlas cells), fertility, floodplains, rivers, country, passes, hidden-costs, territories, worn, trails, hearths, festival-routes, festivals, land, held, lines, clans, leanings, centres, capitals, units, routes, overlay. `icon.js` draws an atlas cell on the map. |
| `www/atlas.js` | Loads `assets/atlas.json` and `atlas.png`: `cell(name)`, `tinted(name, colour)` for a `.mask` cell, `scaled(name, h)` pre-scaled cells. |
| `www/assets/` | `atlas.json` (every cell's place, size and note: the artist's brief), `atlas.png` (placeholders), `placeholders.js` (draws each cell and lays out the sheet), `draw-atlas.mjs` (renders both through headless Chrome), `blank.html`, `sounds.json` (each feed kind's file name; no sound files ship). |
| `www/feed.js` | The event feed: newest first, a filter by kind; `KINDS` is every event kind with its icon and colour. |
| `www/check.mjs` | The page's data checks under Node, sending the wire forms through `Sim`; `check.sh` runs it. |
| `www/play.mjs` | Drives the page in headless Chrome: clicks by tile (on the map's canvas) and label, waits on state, screenshots; `SIZES`, `open(query, {size})`, `goto`, `reload`, `key`, `file`, `drag`, `tip`, `download(dir)` and `lead(page, {seed, civ})`; the play scripts import it. |
| `www/play/all.mjs` | Runs every other script in `www/play/` and prints pass and seconds; fails without Chrome under `CI`. |
| `www/play/start.mjs` | Touch points 1-4: a bare URL shows start; New game, the pre-game and Skip to the pick; 🎲; Load a file into play; another version's save shows its error; Continue. |
| `www/play/pick.mjs` | Touch point 5: three cards; Show highlights; Lead selects the capital; Watch leaves no civ led. |
| `www/play/bar.mjs` | Touch points 7-9: the counts and the chips with tooltips, End turn and space, Play and space, speed, each menu and Escape; at phone width the one-line bar and ☰. |
| `www/play/save.mjs` | Touch points 2, 10, 13: the autosave holds turn N; Continue after a reload has the same digest; Save to file and Load a file match; refused storage plays on. |
| `www/play/end.mjs` | Touch point 14: Standing so far (a line per civ), Play on, New game. |
| `www/play/art.mjs` | The atlas: every cell inside the PNG and not blank, the map's terrain drawn from it; the sound bus: every feed kind in `sounds.json`, and kinds heard after ten turns. |
| `www/play/views.mjs` | The map: pan, zoom and fit, each layer toggled by name, H for hidden country, a hover's tooltip; the View and Debug menus' items and their defaults. |
| `www/play-camp.mjs` | A camp's line home: moves it out on its own land and screenshots the walk. |
| `www/play-hold.mjs` | A rival camp's line cut: a thicker guard marches onto the nearest rival camp's tile. |
| `www/play-lean.mjs` | A civ's gift: raised above the default, and the clan that leans to it. |
| `www/play-read.mjs` | The reasons and alerts: a clan's and the camp's panels, with Hidden country on. |
| `www/play-scout.mjs` | A scout's route: the scout selected, and its start and end picked by clicking. |
| `www/play-claim.mjs` | A civ's claim: painted out from the capital, three guards raised, ten turns; the border (held and paper), the tints and a festival's walkers screenshotted. Exits non-zero if the border or the claim's counts are missing. |
| `www/pkg/` | The wasm build (`build-web.sh`), not committed. |

## Adding a mechanic

1. Its state is a field (or a struct of fields) on `World`, built in `World::new`.
2. Its per-turn work is one or more `Step`s in `TURN`, in the place the design puts them.
3. Anything the player or an AI does to a unit is an `Order` (`orders.rs`, through `World::order`; the player's go through `Action::Order` and are logged). Anything else the player does is a variant of `Action`, checked in `validate` and applied in `act`. `World::run` takes the player's pick itself (watching), so tests and the CLI that run past the end keep working; a front end that calls `act(EndTurn)` itself must send `ChooseCiv` when `phase` is `choose` (as `cross-build.mjs` does).
4. A module of its own under `core/src/`, tests under `core/tests/`.
5. Shared actor behaviour (spoil, eat, walk) goes in `actors.rs`; a new actor is a new state on it, not new machinery.
6. A unit kind is a `UnitKind` and a `UnitState` variant. A kind with no state of its own (the guard) takes no order but raising and disbanding, which `unit_among` refuses with the reason, and its turn is one `match` arm in `units::step`.

## Invariants

- Reasons and alerts are derived: `World.notes` and the alert candidates are `#[serde(skip)]`, so a
  digest and a save don't see them, and a replay rebuilds them. A rule writes a reason only where it
  acts on the subject (a note is one `w.note` call beside the decision), never a per-turn status.
- A change that moves `stats::digest` for a seed bumps `SAVE_VERSION` in `core/src/world.rs` and the
  pinned digests in `cli/tests/save.rs` (`the_save_version_pins_the_rules`) together: a save from the
  old rules would otherwise replay into a different world without a word.
- The world changes only through `act` (and the AI's orders, which a replay re-derives). A `Save` (config and action log) replays to an identical world;
  `replaying_a_save_rebuilds_the_world` holds it to that.
- All randomness goes through `rng::rand` on `World.rng`.
- Floating-point `exp`, `ln`, `powf` and the like go through `libm` (clippy's `disallowed_methods` enforces it), so a save
  replays identically native and in the browser.
- The native and wasm builds give the same `stats::digest` for the same seed after every turn; `./cross-build.sh` checks it in CI.
- `clear` conserves every good exactly; tests hold it to that and to nobody losing utility.
- `wasm-bindgen` in `web/Cargo.toml` is pinned to the installed `wasm-bindgen-cli` version.
