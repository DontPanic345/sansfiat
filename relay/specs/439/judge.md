# #439 review: the judge

No sweep run: `git diff origin/main --stat -- engine/core engine/cli engine/web` is empty, so the engine,
every digest and `SAVE_VERSION` (27) are main's, and a sweep of the branch is a sweep of main.

| Score | main | branch | floor |
|---|---|---|---|
| Standing | main's | main's (same engine) | — |
| Doubt | main's | main's | — |
| Decisions | main's | main's | — |
| Motion | main's | main's | — |
| Health | main's | main's | in band |

Verdict: **same**. The piece asks no score question; its question is the art and the build.

## The review's own checks

- `node www/assets/draw-atlas.mjs` twice (Chrome): `atlas.png` 6a0b0558…5365 and `atlas.json` b36d382c…8f68
  both times, the committed bytes. `--check` passes.
- A copy of `atlas.png` with one channel of one pixel in `terrain.plains.1` flipped: `--check` fails, naming the
  cell and the pixel.
- `atlas.json` has all 41 of main's keys with main's `x`, `y`, `w`, `h`; each note starts with main's note.
- `small2-flat` (phone, k 5, the flat fill): main paints 1736 px of plains `#9cbf5a`, the branch the same
  1736 px `#7b882d` (the cell's `flat`); every other colour count is identical.
- Nothing in the browser imports `assets/sources.js` or names `docs/proposals/map-art/`.
- The captures read against `docs/proposals/map-art/ui.png`: the olive grass with occasional rocks is the
  target's look; floodplain tint, rivers, borders, fields, hearths and units sit where main has them.
