# #338 review: the judge

## Sweep (`sansfiat sweep --judge --json`, branch 74b9bed against origin/main)

`judge --compare main.json branch.json --all`: all 19,448 metrics change by 0.0000. No rule moved, which is
what the piece claims: everything it adds is read off the world (`outlook.rs`, `civs::gift_reach`, `cut_by`)
or is a refusal's wording (`claim::check`, the same accept/refuse set). No panics.

## The question: decisions in played games

| Game | Map, civ | Orders | Per 100 turns | Turns with none | Longest idle run |
|---|---|---|---|---|---|
| Control: M4 check on main | small 2, civ 1 | 11 in 103 | 10.7 | 95 of 103 | — |
| build-2, before its fixes | small 61288, civ 202 | 58 in 101 | 57.4 | 92 of 101 | 39 |
| play, all fixes in | small 2, civ 1 | 103 in 102 | 101.0 | 58 of 102 | 4 |

The play game: 86 of 103 orders are claims that the Reach line named. Its idle turns drop from 95 to 58,
and its longest run is 4.

## The other scores, played games (game / AI control / idle led civ)

| Game | Standing at the end | Doubt: lead changes | Motion: clans changed |
|---|---|---|---|
| small 2, Red | 47.4% r1 / 45.9% r1 / 30.1% r2 | 0 / 2 | 81 / 71 |
| small 61288, Gold | 16.5% r3 / 29.7% r1 / 0.0% (falls) | 3 / 3 | 73 / 56 |

## Checked by the review

- Camera: on a live small 2 page, clicking (-2, 4) twice gave byte-identical screenshots the second time;
  the first click moved the view.
- Gift line: the panel reads `2.0 a turn · reaches no clan · gave 0.0 · left 2.0`, and its tooltip gives the
  reason. The text view on the played game reads `reaches no clan at your market: gave 0.0, left 0.0`.
- The play note's finding 1 (Move to… does nothing) is not this piece's doing. In the lines file (lines
  455–468), the third press of Move to… toggled the armed click off (the hint went back to "Right-click"),
  so the click selected the clan. Earlier, an armed click on a mountain (-3, -2) disarmed without a march
  or a refusal. That is main's behaviour. On a fresh game, Move to… then a click marched the army.
