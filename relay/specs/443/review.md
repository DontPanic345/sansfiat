# #443 spec-review

Read against main at 40656df (the branch's base; main has not moved). The spec's inventory, callers and setups
hold: only `map/layers/units.js` reads `unit.*`; `atlas.tinted` is used by `units.js` and `capitals.js` (through
`layers/icon.js`); `atlas.scaled`, `view.dpr`, `view.selectedUnit`, `play.mjs`'s `lead` and `raiseArmy` exist as the
briefs use them; on main the mask draws under a hollow figure, and the spec turns it to a full figure with its mask
multiplied over, which no other caller depends on. `actors.png`'s top row is the walker, the round-shield spearman,
the kite-shield spearman and the flag, as the spec names them.

## Fixed

- **The stack check could not pass.** build-2's `play/units.mjs` asked each number's anchor to lie outside the
  other unit's figure rectangle. With the starting numbers a cell is 2.4 s wide and the gap 1.7 s, so slot 1's
  cell spans +0.5 s to +2.9 s from slot 0, and slot 0's number at +0.95 s is inside it whatever the art. The check
  is now that the two numbers' text boxes (`stats.labels[].box`, new) don't overlap; that no figure covers a
  number holds by the drawing order, which the review reads.
- **"Nearest civ colour" in plain RGB fails gold.** Multiply shades the civ's colour by the cloth's grey. Gold
  (214,166,59) at grey 0.6 is (128,100,35): nearer red (97.6) than gold (111) in RGB. By chromaticity (each colour
  over its r+g+b) a shaded colour stays its own at any grey. The probe's `nearest` now uses chromaticity, so the
  CI check fails only on a colour that is really wrong, not on the shading build-3 picks.
- **build-3's Fit** names that the starting `feet` put a figure's feet 1.2 world units below the tile's centre,
  outside its pointy-top hex (as main's shield is), so the canvas knows to move them; and **its Numbers** names
  wide stacked numbers running together, which main hid by covering them.

## Kept

- The shape (two easels by area, one canvas) and the scores: build-1 5 haiku high, build-2 4 haiku high, build-3 9
  sonnet high. Merging the easels would save a session but cross two areas 442 kept apart; each is under 500 lines.
- The review at Opus medium: the goal is judged by eye.
- Nothing was run beyond arithmetic on the spec's numbers: the setups and the lineup were run on main by the spec
  step (`proto/scenes.txt`, `proto/lineup.mjs`), and nothing in the plan depends on a number a sweep would settle.
