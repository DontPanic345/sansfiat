# #339 build-2: evidence

## What was built

- `engine/www/screens/tests.js` (the Tests screen, 180 lines, 5.3 KB): the list from `Sim.tests()`, a test built with
  `Sim.stage_test(json)`, its map as a `MapView` fed the test's `build()` model, Play/Pause (a turn per `pace` ms,
  500 by default), Step, Again, space and Escape. The page shows `test_state()`; it never decides pass or fail.
- `engine/www/tests-view.js` (56 lines): the hold marks, the result line and the list rows, as words.
- `engine/www/index.html`: `#screen-tests` and a Tests button on Start, beside Load a file.
- `engine/www/main.js`: `tests` in `SCREENS`; `?tests` opens the list, `?tests=NAME` a test, `pace=MS` the pace.
  `window.game` gains `tests()`, `test(nameOrJson)`, `testState()` and `testView()` (the open test's `Sim.view()`).
- `engine/www/play/tests.mjs`: the proof, run by `all.mjs`. `engine/CLAUDE.md`'s `www/` row names `screens/tests.js`.
- `engine/www/style.css`: a tests block, 23 lines.
- `play.js` is untouched; the Tests screen builds its model with `model.js`'s `build`, which already takes any `Sim`.

## Proof

`node www/play/tests.mjs` (22.4 s): 19 checks, all ok. The list has the four tests in order, the first `scout-errand` with its
claim. Choosing it draws the map (more than 20% of sampled pixels differ from the background), reads `0 of 40`, and every
hold is pending. Step moves the scout from tile 119 to tile 62 at turn 1. Again, then Play, runs it to the end: `Passed`,
all 8 holds held, `game.testState().passed === true`. A copy with `"by": 2` on holds[3] (the built-trails hold) fails at
turn 2 and the result reads `Failed: by turn 2: Red's scout has built its trails at turn 2`. The page logs no errors.

Screenshots (read at 1280x800): `relay/specs/339/tests/tests-list.png`, `tests-errand-passed.png`, `tests-errand-failed.png`.
The claim, the map with the scout, and the holds' words and marks are legible.

## Checks

`engine/check.sh` exit 0 on the branch: fmt, clippy, every test file, build-web, cross-build, `check.mjs`, `all.mjs`.
`engine/CLAUDE.md` is 8128 bytes (budget 8192): the `www/` row names `screens/tests.js`; the `check-stage` row lost a clause.

`all.mjs` seconds (every script passes): total 194.6 s, against main's 174.7 s (`allmjs-main.txt`). The +20 s is
`tests.mjs` (22.4 s, a new script that opens Chrome); the other 22 scripts total 172.2 s, as on main.

## Not covered

- A refused order (`test_step` throws): the page shows the engine's reason under the result and stops Play. No script
  drives a refused order; build-1's `check.mjs` covers the door's own errors.
- Escape and space were not driven by a script; both are read in `screens/tests.js` `keys()`.
