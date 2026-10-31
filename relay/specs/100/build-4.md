# Build 4 (ford): the orders

Piece #100. Read `relay/specs/100/spec.md`: items 2, 10, 11, 12, 13 and "The digests". The design is
`docs/design/actors.md` ("Who chooses") and `docs/architecture.md` (Orders). **No rule changes.** This
is the one step where a digest legitimately moves: the led game's, and only through the log.

## The work

1. **`core/src/orders.rs`**: `Order` (spec item 10; serde `tag = "type"`, `rename_all = "camelCase"`),
   `World::order(civ, Order) -> Result<(), String>` (check, then apply; does not log; the AI calls it)
   and `World::check_order(civ, &Order)`. Move the `check_*` and `order_*` functions out of
   `units/{mod,camp,scout}.rs` into it and generalise them from "the player's camp" to the unit named:
   `own(w)` and `leader(w)` give way to `unit_of(w, civ, id)` (the unit exists, is a camp for a camp
   order and a scout for a route order, and belongs to `civ`), and `Raise`'s check (givers, store, `army`
   on) takes `civ`. The checks run in spec item 10's order with its refusals: phase first ("the game has
   not begun": `check.mjs` matches /not begun/ for a raise before the pick); in `validate` only, the
   player leads `civ` ("you're watching: you lead no civ" / "that civ is not yours"); then "{id} is not a
   unit", "that unit is not yours", "that unit is not a camp" / "that unit is not a scout"; then each
   order's own check with its text as it is. `with_camp` goes.
2. **`Action::Order { civ, order }`** (item 11) replaces the ten camp/scout variants. `validate`:
   `Main` phase, `civ` is `w.player`, then `check_order`. `act` applies with `World::order` and logs. Keep
   `SetKeep`/`SetGift` as they are, using `leader` (keep it, in `units/mod.rs`).
3. **The AI through `order`** (item 13), in `units/ai.rs`: `Raise`, `Move`, `Zone`. Write the
   "posted at …" note before the move as now. The spec-review checked on main that every AI move the
   five baseline runs and the retro sweep make passes `check_move`'s conditions, `ai_zone` already gates
   on `zone_ok`, and `raise` refuses what `check_raise` does: the AI's orders all pass. So if a watching
   digest moves, the generalised check refuses what it shouldn't (or applies differently): fix the check
   or the apply, never the AI's choice and never by relaxing a check.
4. **`SAVE_VERSION = 2`**; update `core/tests/save.rs`'s expected messages and `cli/tests/save.rs`'s
   `VERSION`. Repin the led-game constant in `the_save_version_pins_the_rules` to the new value
   (`sansfiat digest --seed 2 --size small --turns 303 --lead`), keep the three watching constants
   untouched, and keep the state-digest assertion build-1 added (`46e67d48ffcbe520`). The three watching
   constants and the state digest not moving are the proof this step's change is the log and only that.
5. **Everything that gave the old actions** now gives `Action::Order`: the CLI's `lead_orders`
   (`cli/src/main.rs`; look the camp and scout up with `units::camp`/`units::scout` and name them),
   `web/src/lib.rs`' doc comments and `Sim::check`, `www/check.mjs` and `cross-build.mjs` (their mirrors
   of the script: edit the action JSON only; build-5 deletes the mirror) so `./check.sh` is green. The
   page (`main.js`, `select.js`) and the play scripts are build-6's.
6. **Tests.** Rewrite the ~50 uses of the old actions in `core/tests/{units,scout,notes,alerts,civs}.rs`
   and anywhere else `cargo test` flags. New (item list in the spec): every `Order` is refused for a unit
   of another civ, an id that is no unit, and outside `Main`; the player's order through `act` is logged and
   `act` refuses an order for a civ the player doesn't lead; the AI's orders are not in the log; a led game's
   save replays to an equal world (`a_real_map_replays_with_the_camp_under_orders` already does for a
   camp: keep it on the new shape); `World::order` for an AI civ works while a player leads another.

## Done

`./check.sh` green. Watching digests of the four baseline runs identical to `baseline/*.txt`, the led
run's `--state` identical to its `.state.txt`, every turn (`diff` empty; say you ran all five), and the
retro sweep `diff` empty. `bench 1 2 3` within 10% of `bench.md`. The wire form is exactly
`{"type":"order","civ":1,"order":{"type":"move","unit":12,"to":5}}`; put one example per `Order` in the
doc comment of `Order`.

## What build-5 and build-6 rely on

The wire form above (camelCase `type` values: `raise`, `disband`, `move`, `zone`, `switches`, `resupply`,
`forage`, `pillage`, `route`, `clearRoute`; fields `n`, `unit`, `to`, `tiles`, `forage`, `supply`,
`from`); `World::order`; `Sim.check(action)` takes the same JSON.

The comparison with the baselines is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines.
