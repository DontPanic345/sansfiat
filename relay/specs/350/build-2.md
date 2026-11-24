# #350 build-2 (ford): the page, where an armed click stays armed

Read `relay/specs/350/spec.md`, "6, 7, 8". Three page findings, one cause: Move to…, the paint and claim buttons
and the P and U keys are toggles, so a second press turns the mode off and the next click selects instead. The
spec's probe reproduced this in headless Chrome. It failed on main and passed with the change below. Don't change
Rust, and don't touch `www/panels-land.js` (build-1 adds a signpost line there).

## Code

`www/screens/orders.js` (`arm`, line ~47, and its doc comment), `www/screens/play.js` (the keys at ~657-658, the
`paint` hook in `panelDraw` at ~531, Escape at ~630-634), `www/bar.js` (the chips' click at ~30, their title at
~151), `www/play/live.mjs` (its text header), `www/play/camp.mjs` (~132-138), `www/play/civ.mjs` (~47-50 and
~106), `www/play/buttons.mjs` (read it and run it).

## Build

1. **Arm, never disarm.** `arm(kind, h)` sets `routing.arm = kind`. Escape, a click off the map and a finished
   order still disarm, as now. The doc comment says a press arms and Escape disarms.
2. **P, U and their buttons arm.** P calls `setPainting('paint')` and U calls `setPainting('unpaint')`. The panel's
   `paint` hook calls `setPainting(mode)`. `setPainting` already maps paint and unpaint to claim and unclaim on the
   civ. Escape stops painting.
3. **The chip says it selects.** Your Standing chip's title adds that a click selects your civ, and a rival's chip
   says what a click on it does. `live.mjs`'s header lists the chips as buttons by civ name, as it lists the panel's
   buttons, so a script session can see the touch.
4. **Tests.**
   - `camp.mjs`: replace "a second press of Move to… disarms it". Press Move to… twice, and
     `document.body.dataset.armed` is `'move'`. Click a free neighbour of the camp, and
     `game.view().units[camp].dest` is that tile.
   - `civ.mjs`, after its claim: with Claim on, end the turn, press P and click a second free neighbour. The claim
     gains that tile and the selection is still the civ. Then select a clan on the map and click your chip. The
     selection is the civ again, and the chip's title names the click.
   - Revert your `orders.js` and `play.js` lines, and check that both scripts fail on that code. Put the failing
     lines in your summary.
   - `buttons.mjs`, `select.mjs`, `scout.mjs` and `all.mjs` pass. If a case pressed a button twice to turn it off,
     it presses Escape instead.

## Done

`camp.mjs` and `civ.mjs` fail on main and pass here, `node www/play/all.mjs` passes, and `engine/check.sh` passes.
About 5 files and 40 lines. Give the decision in your summary: an armed click or paint stays armed until Escape.

The spec's probe is `relay/specs/350/probe-page.mjs`. It was written for a `/tmp` copy of `www`, so read it for the
calls, not to run it as it stands.
