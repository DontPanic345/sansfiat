# #440 judge

No sweep: `git diff origin/main...HEAD --stat -- engine/core engine/cli engine/web` is empty, so the simulation,
every digest and `SAVE_VERSION` (27) are main's, and `sweep --judge` would print main's numbers on both sides.
The piece moves no score by construction; its question is the map's look, judged on the views.

| score | main | branch | floor |
|---|---|---|---|
| Standing | main's | unchanged | — |
| doubt | main's | unchanged | — |
| decisions | main's | unchanged | — |
| motion | main's | unchanged | — |
| health | main's | unchanged | — |

Verdict: **same** (no rule moved; art only).

## The review's own checks

- Read the diff cold: `map/ridges.js` (pure builder), `map/layers/ridges.js` (static layer after `terrain`, cached
  per `view.nb`, culled by drawn rectangle), `terrain.js` (`cellHeight`, `isFlat`, mountain flat colour), `view.js`
  (import, `LAYERS` entry, `noCull`, nothing else). They do what the spec says.
- Shot `large8-fork-play` and `small9-fork-edge` again on the branch (build-2 only skimmed the large views): the
  large map's ranges read as long chains joined at the forks; the phone at k 6 shows them as grey chains.
- Read the five committed pairs: on main, grey hexes with a triangle; on the branch, peaks on grass joined by low
  ridges, southern over northern, with borders, rivers, clans and icons drawn over them.
