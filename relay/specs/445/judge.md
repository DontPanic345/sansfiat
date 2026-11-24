# #445 judge

No sweep ran: `git diff origin/main...HEAD --stat -- engine/core engine/cli engine/web` is empty, so every rule, digest,
`SAVE_VERSION` and save is main's. `diff.txt` shows the ten place views' cameras and digests matching main's (small 2 +40
`4ee21ce6517e1cc5`, medium 9 +40 `c38cfa6eb0163d07`, large 8 +40 `37104de33cd8e31b`, lineup `514f01f255b6d28f`).

| Score | main | branch | floor |
|---|---|---|---|
| Standing | main's | identical (same digests) | — |
| Doubt | main's | identical | — |
| Decisions | main's | identical | — |
| Motion | main's | identical | — |
| Health | main's | identical | — |

Verdict: **same**: the piece is art only, and asks none of the four scores.

## Review's own checks

- Medium 9 on the live page (no step ran it), the camera on a capital, k 5 → 35: `stats.places` is `mark` exactly where
  `stats.clansFlat` holds (k ≤ 13, places from 13.6), as the spec's switch says.
- A fallen civ keeps no settlements (`settle::fallen` removes them), so `placesOn` can't draw a settlement in a fallen civ's
  colour that the probe's living-civ candidates would miss.
- Nits left: `stats.places` keeps its last value while the places layer is off (the layer doesn't draw, so nothing
  resets it); the legend's capital and settlement rows still show main's houses, which the map draws only at the flat zoom
  (build-2's decision; the spec asked for the new marks).
