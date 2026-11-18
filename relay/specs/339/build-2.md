# #339 build-2 (ford): the Tests screen

Read `relay/specs/339/spec.md` (What the piece delivers, 5 and 6) and build-1's summary, which
gives the wasm door's JSON: `tests()`, `Sim.stage_test(json)`, `test_step()`, `test_state()`.
The verdict is core's: the page shows `test_state()` and never decides pass or fail itself.

## The screen: `www/screens/tests.js`

A screen module like `screens/start.js` (`init`, `show(args)`, `hide`), a `<section
id="screen-tests">` in `index.html`, and `tests` in `main.js`'s `SCREENS`.

- **Reached** from Start by a Tests button beside New game / Continue / Load, and by URL:
  `?tests` opens the list, `?tests=NAME` opens that test chosen. A Back button returns to Start.
- **The list**: one row per test from `tests()`: its name and its claim. Choosing a row builds
  its `Sim` through `Sim.stage_test` with the test's `json` (the file's text, which `tests()` carries) and shows it.
- **The chosen test**: the claim, the kept stores in a line when it has any ("the store is kept
  at 50 each turn": the stage's hand, said plainly), the turn (`0 of 40`), the map at fit, and
  the holds, each with its words and a mark: pending, held (and the turn it settled), failed
  (the turn and the engine's why).
- **The map** is its own `MapView` (`www/map/view.js`), as `screens/play.js` builds one, fed
  the test's `Sim`; it draws what the play screen's map draws (terrain, borders, units and
  their marches, trails). Don't add to `screens/play.js` (759 lines); if you need a helper both
  share (building the view model from a `Sim`), move it out of `play.js` into a module both
  import.
- **Play** plays one turn every half second or so (a pace you can watch a scout cross the map
  at), through `test_step()`, redrawing the map and the holds after each; Play becomes Pause
  while it runs. **Step** plays one turn. **Again** rebuilds the test from its start. At the
  last turn the result line reads **Passed**, or **Failed: <the hold's words> at turn N**.
- Keys: space is Play/Pause; Escape goes back to the list.
- `window.game` gains `tests()` (the list), `test(nameOrJson)` (opens the screen on a test by
  name, or on a test given as an object, for scripts), and `testState()`.

`screens/tests.js` should stay under 300 lines; put drawing helpers it needs in `www/map/` or a
small `www/tests-view.js`.

## Proof: `www/play/tests.mjs`, in `all.mjs`

- Opens `?tests`; the list shows four rows, the first named `scout-errand` with its claim.
- Clicks `scout-errand`: the map is drawn (the canvas isn't blank: pixels differ from the
  background), the turn reads 0, every hold is pending.
- Presses Play (speed it with a query or by pressing Step: the script must run in seconds, so
  press Step to the end, or pass a pace) until done: the result reads Passed, every hold is
  held, and `game.testState()` agrees with what core says: the same test run under Node in
  `check.mjs` passes, and the page's `passed` is `true`.
- Opens a copy of `scout-errand` with `"by": 2` on the trails hold through `game.test({...})`:
  the result reads Failed, names that hold's words and turn 2.
- Watches it move: after Step, the scout's icon is on a different tile than at turn 0
  (`game.testState().turn` 1 and the view's scout tile changed).
- Screenshots `tests-list.png`, `tests-errand-passed.png` and `tests-errand-failed.png` to
  `--out`; copy those three into `relay/specs/339/tests/` for the review.
- Start's Tests button opens the list (one click from the bare URL).

## Done

`node www/play/tests.mjs` and `node www/play/all.mjs` pass; `engine/check.sh` passes; the
three screenshots are in `relay/specs/339/tests/`. `engine/CLAUDE.md`'s `www/` row names
`screens/tests.js` (keep the file under 8 KB). Read the screenshots yourself: the claim, the
map and the holds are legible at laptop size.

About 6 files, 450 lines: `screens/tests.js` (~250), `index.html` (~20), `main.js` (~15),
`screens/start.js` (~5), `tests.mjs` (~120), `all.mjs` and `CLAUDE.md` (2).
