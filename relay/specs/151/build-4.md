# #151 build-4 (ford): `Sim.stage`, the page arm and the Disband all case

The page can be built from a staged game, so its panel shapes are tested under Node as data.
Read `spec.md` items 13–14, build-1's summary (`StageSpec`, `Stage::from_spec`),
`web/src/lib.rs` (`Sim::new`, `load`, `act`, `view`), `www/main.js`, `www/screens/play.js`
(`show`/`begin`), `www/check.mjs` (its last section builds panel shapes with `describe` and a
hand-made `panelCtx`) and `www/panels.js` (`disband`, `campShape`, `scoutShape`, `guardShape`,
`civShape`).

## Do

1. **`Sim.stage(spec: &str) -> Result<Sim, String>`** in `web/src/lib.rs`, a constructor like
   `load`: `serde_json` into `StageSpec`, `Stage::from_spec`, `into_world()`, a fresh `Lead`.
   Add `Sim.staged() -> bool` (true for a staged Sim). A staged world is not a save: in
   `www/screens/play.js` and `www/save.js` skip the autosave and the Save button for a staged
   Sim (read how they write it; keep the change to the one condition each).
   A spec with `retro` runs the world's pre-game on build, as the native stage does (no cache).
2. **`window.game.stage(spec)`** in `www/main.js`: `app.show('play', { sim: Sim.stage(JSON.stringify(spec)) })`
   (the screen already starts any `Sim` it is handed, as the start screen does for a loaded
   save). Check it with a staged game in a short Node-free test in `play.mjs`' style only if
   it is cheap; build-7's script is its real user.
3. **The arm in `www/check.mjs`**, after the existing panel section:
   - A helper `staged(spec)` -> `Sim`, and `view = JSON.parse(sim.view())`.
   - Stage a two-civ game on a small hand-written map text (a few tiles of plains, sea on the
     rim, food on a tile or two) led by the first civ, with a camp of **4.6** people at the
     capital. `panelCtx` takes the staged view and the staged Sim's layout/tile/notes.
   - Panel shapes: build the camp, scout, guard and civ shapes from the staged view and the
     clan and a tile; assert every button has a `label` and exactly one of `action`, `arm`,
     `paint`, `stepper`; a stepper's options each have a `label`, a `cap` and a `make` that
     returns an order or action object with a `type`.
   - **Disband all**: take the camp's `Disband…` button, its option labelled `all`, make the
     order at its `cap` (the count the page would send), apply it with `sim.act(JSON.stringify(order))`
     and assert that the army is gone from the view and the civ's clan has 4.6 more people.
     This fails today (the order carries `n: 4`; 0.6 stays). Wrap it in a helper
     `expectBug(name, message, fn)` at the top of the arm: it runs `fn`, passes if it throws an
     `AssertionError` whose message includes `message`, and fails (naming the case) if `fn`
     passes or throws something else. The doc comment says: "Red on main: T fixes the order
     form, then drops the wrapper."
   - The scout's and the guard's `all` option the same way, inside the same `expectBug` (a
     fractional scout of 1.5 is staged beside).
4. A `StageSpec` round trip through `Sim.stage` under Node: the digest of `sim.digest()` for
   the same spec twice is equal.

## Done

`./build-web.sh` then `node www/check.mjs` pass (the Disband case by `expectBug`);
`./cross-build.sh` still agrees (no digest moves); `cargo clippy --all-targets -- -D warnings`
and `cargo fmt`. Append the Disband page-arm's failure message (exactly) to
`relay/specs/151/cases.md`, under the heading "Disband all, the page arm", with what T must
do (make `all` send no count).
