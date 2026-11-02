# #103 build-2: the screens, the bar, the menus, keys, autosave, end screen, sound bus

**Touch points, after build-2** (script in `engine/www/play/`, all in `play/all.mjs`):

| # | Touch point | Script | Result |
|---|---|---|---|
| 1 | Start: new game with size, seed, a random seed | `start` | pass |
| 2 | Start: Continue the autosaved game | `save`, `start` | pass |
| 3 | Start: Load a file; an old save fails with its version | `start` | pass |
| 4 | The pre-game plays on the map, Skip to the pick | `start` | pass |
| 5 | The pick: three cards, Show, Lead, Watch | `pick` | pass |
| 6 | Map: pan, zoom, fit | `views` | pass |
| 7 | Top bar: every civ's Standing, yours marked, a tooltip on each number | `bar` | pass |
| 8 | End turn by button and by space | `bar` | pass |
| 9 | Play, pause and speed | `bar` | pass |
| 10 | Game menu: new game, save to file, load a file, Standing so far | `save`, `end` | pass |
| 11 | View menu: territories, festival routes, hidden country (H), legend | `views` | pass |
| 12 | Debug menu: fertility, hidden costs, worn land; off by default | `views` | pass |
| 13 | Autosave each turn, guarded when storage is refused | `save` | pass |
| 14 | End screen: Standing over the game, play on, new game | `end` | pass |
| 25 (sound half) | Sounds mapped by event kind, silent | `art` | pass |

Rows 15–24 and 26 are build-3's and build-4's. The old `play-*.mjs` scripts: `camp`, `lean`, `read`,
`scout` and `claim` pass; `play-hold` fails (its raise is 5 of 10 warriors; it already fails on main).

**Phone bar, measured** (`bar.mjs`, 390 px, small seed 2, led): the bar is one line, 4 px padding;
the turn 29 px, the three chips 195 px, End turn 70, ☰ 26, Feed 45, ending at 386 px of 390.

**What reads badly in `screens/build-2-*.png`** (laptop, half scale):
- With a capital selected, the bottom panel takes about 40% of the window's height, and the map
  shrinks to about 55%. This is the panel's `max-height: 45%` of the map column; the canvas decides.
- The start card's "Start" button wraps onto its own line below the seed row.
- The end chart is correct, but five turns make the lines flat, and the last-value labels crowd
  when two civs end at the same Standing.
- The bar at laptop is dense but reads: turn, three chips, then the controls.
- At phone width the "Show feed" button sits over the map's corner, against the bottom panel.

**Decisions** are in `result.json`. The `window.game.digest()` hook and the driver's `tile()` change
are in the diff.
