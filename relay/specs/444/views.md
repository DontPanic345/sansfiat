# #444 build-3: the clans on the map, judged beside main

Main is `f5ea782` (the branch's base) in a worktree (`/tmp/b3/main`, the branch's `shots.mjs` and `play-stage.mjs` copied in,
`www/pkg` copied from the branch: no Rust changed). Both trees shot the 12 clan views (`--out`, `--only`). Captures are in
`shots/<view>-main.png` and `<view>-branch.png` (`proto/crops.mjs`: the same rectangle in both, 2× or 3×):
`clans-lineup-close`, `clans-lineup-play`, `clans-lineup-edge`, `medium9-clans-play`, `medium9-clans-close`,
`small2-clans-close`, `medium8-clans-play`. `git diff main --stat -- engine/core engine/cli engine/web` is empty.
`atlas.png` and `atlas.json` are unchanged from build-1 (the sha256s in `atlas.md` hold): the cuts and cloth are as cut.

## What I changed, and why

Build-2's first captures (every view shot, read beside main) showed two things that did not read, and a third at k 18.

1. **Every cell has a dark edge** (`CLAN.edge`, 1.3 css px, `rgba(24,14,4,0.9)`, `atlas.outline`, under the cell). Main's circle has
   a dark stroke; the cell had none. At k 18 a clan of no civ (the pale hide, `#d8d2c4`) was nearly invisible on grass, where
   main's pale circle is the clearest dot on the map. With the edge the pale tent reads as a pale tent at k 18 and k 35, and a
   red or blue tent keeps its colour. The starving edge draws under it (red 3.5 css px, then the dark line, then the cell).
2. **`CLAN.from` 12 → 28** (a terrain cell 28 device px high: **k 13.6** at one device px per css px, 6.8 on a 2× phone).
   Under it the clans are main's circles. At phone k 6 the cells were a crowd: neighbours touch (10 css px tall, 12 wide, a hex is
   10.4), a pale tent was lost against the terrain, and the starving edge (3.5 px round a 10 px cell) was a red blob over the
   tent. Tried 16 (k 8) and 20 (k 9.7): at 12 to 14 px tall the dark edge and the art's shading leave the pale tent a brown
   speck, easy to take for a rock or a hill on the terrain beside it (medium 9 at k 8, 10 and 12, same camera as
   `medium9-clans-edge`). At k 14 (15 px) it reads as a tent. So the zoom the cells begin at is **k 13.6**.
   Phone k 6 and 5 now both draw main's circles: `medium9-clans-edge` and `clans-lineup-edge` are pixel for pixel main's
   (0% differ), as `medium9-clans-flat` is. `CLAN.minPx` (10 css px) no longer binds: above k 13.6 `max(minPx / k, …)` is under
   0.74 world units, below a 4-person clan's 0.84.
3. **Walkers are drawn taller** (`CLAN.walkers` 1.75 for a lone clan, `walkersShared` 1.3 on a shared home). At a tent's
   height (15 px at k 18) the man, the woman and the child merged into two gold humps beside a tent: "one more tent". At 1.45 they
   still did; at 2.0 the three read as people (the man with his pack, the woman's headscarf, the child). 1.75 is 26 px at k 18
   for 4 to 5 people. A walker clan is thereby larger than a tent of the same people; "people read by size" still holds
   among the walkers, and the walkers read as a group moving, which a tent doesn't.
4. **The bar** (`CLAN_BAR` in `shots.mjs`, `--clans` exits 1 under it): **100 tinted pixels at k 35 and 40 at k 18.** The fewest
   in the views are 113 (a hearth at k 35, small 2) and 43 (a shared home's hearth at k 18, 14 px tall, its canvas 30% of the
   cell). At 40 the colour shows on a 14 px cell (the hearth's red canvas in `clans-lineup-play`); a tent at k 18 has 57 to 282.
   A dot is main's circle whole and has no bar; no cell draws at phone k 6 now.
5. **`stats.clansFlat`** (new, set by `clans.js`'s `draw`): `--clans` and `play/clans.mjs` read it to say a clan is a dot, as it is
   decided in one place. `play/clans.mjs`: k 5 and k 6 are dots (k 6 with the terrain's cells on), k 13 dots, k 14 cells.

## Per view: main and the branch

| View | Main | Branch |
|---|---|---|
| `clans-lineup-close` (k 35) | circles: red, blue, gold (starving: red ring), pale; a ring of four on the shared home with its dark square; a pale clan on Gold's units' tile | tents in red, blue, yellow (starving: red edge round it) and pale on plains and in the wood; the shared home shows a red hearth (tent, seated man, fire, smoke) behind a blue tent, a red tent and three small red walkers; the pale clan on tile 126 stands under Gold's scout and camp: their figures and "2.0", "4.0" are clear, and the tent shows only as its white cloth left of the scout's pack (main's pale circle was half under the shields too) |
| `clans-lineup-play` (k 18) | the same at 18 px | tents 14 to 16 px: red, blue, gold and pale read in a row; the hearth reads as a tent with a fire and a figure; the starving gold tent's red edge is as strong as main's ring |
| `clans-lineup-edge` (phone k 6) | dots | dots, identical to main (0% differ) |
| `medium9-clans-play` (k 18) | circles, Gold's four round the town at 734, the capital's house | tents in Gold's colour on the town's tiles, two hearths among them, walkers in the wood at 874 and 919: man, woman, child in a file; pale tents at the coast; starving tents edged red. The capital's house, the ring and the town mark sit where main has them |
| `medium9-clans-close` (k 35) | the same close | the same close: tents with their baskets, hearths with fire and smoke, walkers; Gold's tents read yellow-tan against the green (the pale hide is grey-white, not Gold), at k 35 clearly Gold's |
| `medium9-clans-edge` (phone k 6) | dots | dots (identical) |
| `small2-clans-close` (k 35) | red circles; the settled ones with a dark square; the mixed home 503 a ring | red tents on Red's land, settled ones as hearths (fire, smoke, the man), starving tents with a red edge outside the red border, readable against it; the mixed home's hearth and tent both show; the trails, icons and yellow tamed-grain squares are where they were, and a clan hides a square no more than main's circle did |
| `medium8-clans-play` (k 18) | pale circles in the desert, circles in Blue's colour | pale tents on the desert read as tents (grey-white on tan); Blue's tents, hearths and the starving blue walkers (red edge) read; two walkers in a file at 725 and 771 |

The lean views show no leaning clan on this runner (build-2 says so); the lineup's two leaning clans (plains, to blue at 0.6; wood, to gold
at 0.2) are the only ones, and read as pale tents on a tinted disc: not as a civ's tent. (Small: the disc is under the tent, at most
1.4 world units, and the edge of the tent shows on it.)

## Judged

- **Shape.** Tent, hearth and walkers are three things at k 35 and k 18 (the hearth has a fire, a man and smoke; the walkers three upright figures).
  Phone k 6 is circles: the tent and the hearth are not told apart there, as on main (the dark square is main's settled mark and stays).
- **Colour.** In `clans-lineup-*` red, blue, gold and none are told apart on plains and in the wood at k 35 and k 18. Gold's cloth is yellow, not the
  study's ochre (the grey floor 190, #443's fix, holds). Not tested at phone k 6, where the circles are main's.
- **Signals.** The red edge on a starving tent reads as clearly as main's ring at k 18 and k 35, on Red's own land too (small 2: a red tent inside a
  red border, edged red outside the dark line).
- **Co-location.** The town at 734 and the mixed home at 503 show each clan; the lineup's four are four things (hearth, two tents, walkers).
  The pale clan under Gold's units (tile 126) leaves their figures and numbers readable, but is mostly hidden by the scout: only its cloth shows. A unit over a clan is the layer order (units over clans); it is not changed here. The cramped part: at k 35 a shared home's four are small (0.8
  world units) and overlap.
- **Fit.** A lone clan stands inside its hex at k 18 and 35; trees, rivers, borders, icons, the capital's house, towns and units are where main has them.

## `--diff` (`diff.txt`)

Cameras and digests match in every view (exit 0). `medium9-clans-flat`, `medium9-clans-edge` and `clans-lineup-edge`: 0%.

| View | Share that differ |
|---|---|
| medium9-clans-play | 3.87% |
| medium9-clans-close | 5.43% |
| small2-clans-play | 5.56% |
| small2-clans-close | 8.41% |
| medium8-clans-play | 5.09% |
| medium8-lean-play | 4.08% |
| small9-lean-close | 5.23% |
| clans-lineup-close | 1.27% |
| clans-lineup-play | 0.40% |

## `--clans` (`clans.txt`)

Exit 0: every clan nearest its own colour and of its state's kind, and over the bar. Each kind's least tinted pixels and the
share of its opaque pixels (a cell's opaque pixels are the cell and its cloth, so the share is the cloth's):

| View | tent | hearth | walk | dots |
|---|---|---|---|---|
| medium9-clans-play (k 18) | 100: 64+, 54–74% | 3: 44+, 31% | 3: 215+, 43–44% | - |
| medium9-clans-close (k 35) | 44: 200+, 48–60% | 3: 129+, 28–29% | 3: 684+, 41–43% | - |
| medium9-clans-edge / -flat (k 6 / 5) | - | - | - | 185: 16+ / 10+ |
| small2-clans-play (k 18) | 129: 57+, 52–71% | 10: 43+, 25–32% | - | - |
| small2-clans-close (k 35) | 69: 178+, 47–62% | 8: 113+, 25–28% | - | - |
| medium8-clans-play (k 18) | 131: 66+, 54–73% | 6: 45+, 31–33% | 4: 178+, 43–50% | - |
| medium8-lean-play (k 18) | 108: 64+, 55–70% | 7: 43+, 28–35% | 2: 141+, 43–48% | - |
| small9-lean-close (k 35) | 51: 224+, 50–56% | 1: 210 | 1: 622 | - |
| clans-lineup-close (k 35) | 11: 263+, 49–55% | 4: 126+, 26–28% | 1: 331 | - |
| clans-lineup-play (k 18) | 11: 100+, 58–66% | 4: 44+, 29–31% | 1: 124 | - |
| clans-lineup-edge (k 6) | - | - | - | 16: 16+ |

Bars: 100 at k 35, 40 at k 18 (above). The least at k 18 is 43, at k 35 113.

## `--time`

`--time medium9-clans-edge` (`time.txt`; terrain time, five redraws): branch median 8.7 ms, main 8.8 ms (the clans there are dots, main's).
The terrain time doesn't include the clans' layer, so the whole `drawNow` was timed too, medium 9 +16 on the phone window, seven dirty redraws, median:

| k | branch | main |
|---|---|---|
| 6 | 11.5 ms | 11.5 ms |
| 18 | 4.3 ms | 3.7 ms |
| 35 | 2.4 ms | 2.2 ms |

Under twice main's at every zoom; the atlas caches (`scaled`, `shaded`, `outline`: 400 entries) hold the clans' canvases between redraws.

## `play/clans.mjs`

Passes: 16 of 16 clans drawn once, each of its state's kind; every cloth nearest its own civ; the starving clan has red pixels round it (79) and none
with the edge off; the shared home's four stand apart, each anchored inside its hex; a click on it reaches each of its four; phone k 5 and k 6 are dots,
k 13 dots, k 14 cells.

## Changes to the easel

- `layers/clans.js`: `CLAN.edge`, `CLAN.walkers`, `CLAN.walkersShared`, `CLAN.from` 28; the dark edge under every cell; `stats.clansFlat`.
- `shots.mjs`: `CLAN_BAR` and its check in `--clans`; the least tinted pixels and share per kind; `expect` reads `stats.clansFlat`.
- `play/clans.mjs`: the flat-zoom checks (k 5, 6, 13, 14); `expect` reads `stats.clansFlat`.
- `proto/crops.mjs`: the captures' crops.
