# #103 build-1: measurements

**Digests, small seeds 1–3, `digest --turns 400`, before and after** (unchanged):

| seed | digest |
|---|---|
| 1 | `400 0cf637438d9b93a3` |
| 2 | `400 1158b7f7a3a84817` |
| 3 | `400 84434f4a185255cd` |

**`stats.terrainMs`**, large seed 2 (3,367 tiles), fit, laptop 1280×800, median of ten forced redraws:
main 8.8 ms, now 4.75 ms (range 3.7–5.9). Terrain is drawn from the atlas cells pre-scaled to whole
device pixels (`atlas.scaled`), one `drawImage` per tile, with flat fills below a 12 px cell.

**Screens** (small seed 2, laptop, half scale): `screens/build-1-before-*.png` (main) and
`screens/build-1-after-*.png`, at the pick and five turns after leading Red with the camp selected.
