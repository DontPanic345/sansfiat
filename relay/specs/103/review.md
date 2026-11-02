# #103 review: what was played and measured

## Checks
`engine/check.sh` passes end to end (4 m 54 s), with `play/all.mjs` at 14 of 14 scripts covering the 26 touch points:
art 3.0 s, bar 11.1, camp 15.8, civ 3.7, end 2.8, feed 4.1, game 8.9, layout 25.3, pick 4.6, save 4.8, scout 3.7, select 3.6, start 5.2, views 3.5.

## Health
No file under `engine/core/` changes, and neither does `cli/tests/save.rs`. `SAVE_VERSION` is still 3, so `stats::digest` and `sweep --judge` are main's by construction.
`cross-build.sh` agrees on every turn, native against wasm. The engine gains only `standing`, `standing_clans` and `standing_land` in `Sim.view()`, `save_version` in `rules()`, and `digest --game`.

## The review's game: small seed 4 (`node www/play/game.mjs --seed 4`)
Led Red at turn 214 ("You lead Red: Standing 15.7%, 1st of 3") and played to 244 with buttons, keys and clicks only:
Gift +1 (2 → 3), Raise… a guard, P and a click to claim (28 → 29), and Move to… sent the camp to tile 656 ("Marching to 9, -13: 3 turns").
At turn 220 the alert "Red's camp at (10, -14) is cut off from its capital" came. Following it selected the camp, whose panel reads "Cut off: its line can't feed it". Then P painted a 3-tile zone.
The page was reloaded at turn 229, and Continue read "Red, turn 229, small seed 4" with the digest unchanged (b85abb059200c9b1). The game was saved to a file at turn 244.
Page `244 90329f6b3bb76660`; `sansfiat digest --game` gives `244 90329f6b3bb76660`. They match. `sansfiat judge` reads the save: Red 17.3% (rank 1) against 16.9% for the AI control.

From the start screen (laptop): New game, small seed 4, Start, Skip to the pick, Lead, ten turns ended with space, then Game → Standing so far. The bar read "224 TURN Red 14.8% YOU ▲ Blue 6.4% Gold 6.2% End turn Play …", there were no page errors, and the end chart drew 11 turns.

Does the page say what happened and why? Mostly. The camp's cut-off is named in its doing line and raised as an alert. Seed 2's game (canvas.md) found a hungry scout with no reason row, and Move to… shows no warning before a target past the line's reach. Both are ideas for later, not regressions.

## Fixed here
- The end chart's axis labelled its quarter lines 6.3%, 13% and 19% on a 25% scale. It now steps in whole percentages: 2% steps on a 10% scale, 5% on 25%, 10% on 50% and 20% on 100%.
