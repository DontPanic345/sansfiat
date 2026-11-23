# #443 judge

No sweep was run: `git diff origin/main...HEAD --stat -- engine/core engine/cli engine/web` is empty and
`SAVE_VERSION` is 27, so `sweep --judge` on the branch is main's to the digest. Every unit view's digest matches
main's (`diff.txt`, `easel.md`). Verdict: **same** in Standing, doubt, decisions and motion; health unchanged.

The piece's own questions, read on the views (`views.md`, `shots/`, `units.txt`):

| Question | Main | Branch |
|---|---|---|
| Camp and scout told apart by shape | same shield | spearman with round shield vs walker with pack and staff, at k 35, 18 and phone k 6 |
| Civ told by colour, plains and wood | flat shield fill | tunic, banner, shield face: 68 of 68 unit rows nearest their own civ; tinted px least 407 / 124 / 63 / 65 at k 35 / 18 / 6 / 5 |
| Selected and hungry edges | shield stroke | figure outline, white 3 px, red 3.5 px |
| Stack numbers | second shield covers the first's number | every number after every figure; boxes ≥ 4 px apart |

Review's own checks: `draw-atlas.mjs` rebuilt to the committed hashes (541a12f9…, 47be6d6d…); a case no step ran
(large seed 4, led, 40 turns on, phone k 6): 4 units drawn, each nearest its own civ, least tinted 64 px, no page
error, a redraw 10 ms.
