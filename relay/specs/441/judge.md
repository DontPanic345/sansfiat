# #441 review: the judge

No sweep was run. `git diff origin/main...HEAD --stat -- engine/core engine/cli engine/web` is empty, so the
engine, every digest and `SAVE_VERSION` (27) are main's, and `sweep --judge` would return main's report byte for byte.
The piece moves art only: the verdict on the four scores is **same** by construction.

| Score | Main | Branch | Floor |
|---|---|---|---|
| Standing | main's | identical (no engine change) | n/a |
| Doubt | main's | identical | n/a |
| Decisions | main's | identical | n/a |
| Motion | main's | identical | n/a |
| Health | main's | identical; static draw +1–3 ms on five play views, +6 ms on large8-play (`views.md`) | — |

# The review's own checks

- Read the diff cold: `map/edges.js` (the piece rules), `layers/edges.js` (draw, cull, cache per map by `view.nb`),
  `view.js` (import, `LAYERS`, comment only), `terrain.js` (the stipple dropped), `placeholders.js` (keys after the
  mountain pieces), `check-map.mjs` (counts the pieces again with its own vertex walk: every corner of every tile,
  deduped by position, so it doesn't share `edges.js`'s corner-1/corner-2 enumeration).
- Fixed: `FRINGE` was two copies, `placeholders.js` (which cells are cut) and `map/edges.js` (which keys the map asks
  for). `edges.js` now re-exports the atlas's. `check-map.mjs` and `play/edges.mjs` pass.
- Opened the six committed pairs, and shot four more views on the branch that build-3 didn't commit
  (`large8-delta-play`, `medium8-strait-close`, `small9-pond-close`, `large8-pond-play`) and the marsh at k 35 on
  small 2 and large 8. The bank follows every shore and turns at the corners with no gap; the strait stays open water;
  necks read as land between two banks; the mouth on small 2 (tile 142's delta) opens through the bank; lakes are
  lighter than the sea; desert, forest and hills read apart; the flat zoom tells the nine kinds apart.
- **Marsh is the weakest ground.** At k 35 it is darker olive grass with pools 2–3 px long, so you can tell it from
  coast only up close. At k 18 the pools are about a pixel. Main's flat teal marsh hex was louder. `map.md` asks that
  deltas be told apart at a glance. Piece 4 (woodland and marsh) is where marsh gets its reeds and water, so this is
  left to it, not fixed here.
- Static draw: large8-play's 18.3 ms against main's 12.0 is two draws a tile at that camera. It is a static layer,
  redrawn when the view moves, so a pan on a large map at play zoom costs about 6 ms more a frame. A per-tile composite
  would close it; left as a note.
