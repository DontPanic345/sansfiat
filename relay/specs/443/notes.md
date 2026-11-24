# Notes for the next builder

- A two-unit stack now reaches into the next hex: the second unit's slot is max(gap 2.9 s, label + pad) past the first,
  about 2.25 world units right of the tile's centre at k 35, past the neighbour's centre column (1.73). On main it was
  about 1.56. A click on the second figure selects the neighbour tile (selection is by tile). Piece 6 or 7 (towns,
  capitals) could stack the units tighter, or put the numbers above the figures, so a stack stays in its hex.
- A unit on its own civ's capital house reads by outline only (red on red, gold on gold), as main's shield did.
- `LINEUP` lives in both `play-stage.mjs` and `shots.mjs`; keep them in step by hand.
