# #443 build-3: the units on the map, beside main

Main is `40656df` in a worktree with the branch's `shots.mjs` copied in. Both trees shot the same 15 views
(`--out`): the 13 unit views and two that select a unit (below). `--diff` (`diff.txt`): every camera and every
digest matches (`exit 0`; digests are the ones in `easel.md`, the two new views carry `62bef9c95b4d1f08` and
`ff55c006fe645483`). `git diff main --stat -- engine/core engine/cli engine/web` is empty. `--units` on the
branch (`units.txt`): 68 unit rows, every one nearest its own civ and above the bar; exit 0. Build-2's finding
(medium9-units-flat, a scout read as another civ) is gone: its scouts have 65 to 66 tinted pixels.

Captures are in `shots/<view>-main.png` and `<view>-branch.png` (`proto/crops.mjs` crops both round the branch's units;
2× or 3× when small): `lineup-close`, `lineup-play`, `lineup-edge`, `medium9-units-play`, `medium9-gold-close`,
`small2-camps-close`, and `lineup-selected`, `small2-camps-selected` (the two edges).

## What changed, and why

The views showed the build-2 figures did not carry the colour. With the banner and the camp's shield face alone, the
tinted share of a figure was 5 to 8% for the scout (31 px at k 18) and 31% for the camp, so the scout read as a brown
walker with a speck of colour and Gold's units read as the study's own ochre. So:

1. **The tunic joins the cloth.** (The spec's allowed change.) Each mask also holds the figure's tunic: a polygon round
   the body read off a 6× crop (`CAMP_TUNIC`; the scout's `SCOUT_BODY` and `SCOUT_SHORTS`), less the outline's darkest
   pixels (luminance under 60). The skin, hair, boots, pack and gloves lie outside the polygons. A hue test was tried
   first and dropped: the tunic's shading ranges from (140, 100, 30) to (251, 209, 137), and a colour threshold
   left it blotchy. `cut.mjs`'s cloth entries may now be `{ poly, where }`.
2. **The grey runs from 190 to 255** (`{ op: 'grey', lo: 190 }`, was 90). At 90 the tunic's middle came out near 0.55
   of the civ's colour: blue and red read, but Gold's came out dark olive, the study's ochre again. At 190 the
   shading still shows (the strap, the fold of the shorts) and Gold's tunic is yellow, with a yellow banner (a bright
   (214, 166, 59) against the study's cream flag with a red mark).
3. **The cloth's rim is firmer.** `atlas.shaded` draws the tinted cell over itself, so a rim pixel of alpha a has
   2a − a². At a dozen px the rim is most of the cloth, and the brown under it greyed blue to (73, 71, 87), nearer
   gold than blue by chromaticity (lineup-edge, scout 14).
4. **The figure stands in its hex.** `feet` 0.9 → 0.5, `slotX` 0.9 → 0.8, `slotY` 1.1 → 0.55: the feet are at 0.63
   world units below the centre and 0.48 right, inside the hex (the lower right edge is at y 0.74 for that x).
   The head reaches 0.81 above the centre. The number's place follows: `labelY` −0.3 → −0.5 (the chest).
5. **A stack's units stand apart.** `gap` 1.7 → 2.9 and a `pad` (0.4): the next unit starts at least
   `max(gap, labelX + pad + the number's width)` after this one, so two numbers never meet, and a wide "4.0+2.0" pushes the next
   unit on. At 1.7 the numbers "2.0" and "4.0" touched ("2.04.0"), and on the capital "2.03.0".
6. **The bow** is over the figure's right shoulder (`bowX` 0.4, `bowY` −1.55): at the old place, the left of the spear, the
   second unit's bow fell on the first unit's number. Drawn black on a white halo, 1.8 over 3.4 css px.
7. **The selected edge** is 3 css px (was 2.4): beside a tree or the wood it was the thinnest signal.

## Per view: main and the branch

| View | Main | Branch |
|---|---|---|
| `lineup-close` (k 35) | six shields, each a flat civ colour. Gold's two overlap, the second's number alone shows (the first's is under it). | six figures: camps with spear and round shield, scouts with pack and staff, in red, blue and gold on plains and in the wood. Gold's two stand apart with "2.0" and "4.0", each readable. |
| `lineup-play` (k 18) | the same at 38 px. | figures 26 px tall; each unit's civ reads at a glance, Gold's tunic is yellow, not the study's ochre. The blue camp in the wood and the red scout in the wood read against the trees. |
| `lineup-edge` (phone k 6) | shields of 17 px. | figures 14 px: a camp (shield on the right, spear) and a scout (pack) differ in outline; red, blue and gold read on plains and in the wood. Gold is the dimmest on the olive grass (its mean is (122, 87, 41)); its banner is the clearest part. |
| `medium9-units-play` (k 18) | Red's two shields over its capital's house, the numbers "3.0" only; Gold's scout is a shield by the ring. | Red's scout stands on the house and its camp beside it, each with its number ("2.0", "3.0"), well apart. The red tunic over the red house is the weakest read (same colour under it); the outline, skin and pack show it. Gold's scout stands on its house at 780 with its "2.0" clear of the ring's other numbers. |
| `medium9-gold-play`, `medium9-gold-close` (lead Gold) | Gold's two shields over Gold's house, one number. | the camp and the scout stand in the wood at 780: the scout inside the house's hex, the camp at its right with the spear's head against the trees, each with its number. Gold's yellow over the gold house is the same weak read as Red's, and shows by shape. |
| `small2-camps-play`, `small2-camps-close` | Gold's hungry camp: a shield with a red edge on a mountain pass; Blue's camp on the coast. | the hungry camp has the figure's own red edge (3.5 css px), as clear as main's: it hugs the spear, the head and the shield. The yellow clan marker sits where it did. |
| `lineup-selected` (k 18) | the selected red camp: a white outline on the shield. | the selected camp edged white round its figure, as clearly as the shield. |
| `small2-camps-selected` (k 35) | the hungry camp selected: white over red on the shield. | white edge, red halo outside it: both read. |
| `medium8-units-play`, `medium9-wood-close`, `medium9-stack-close`, `medium9-units-edge`, `medium9-units-flat` | shields. | the same fit: a lone figure inside its hex, trees, rivers, borders, icons and clans where main has them (`--diff` shows 0.2 to 1.1% of pixels differ, the units and their numbers). At phone zoom a unit on a capital hides behind the capital's marker, as main's shield does. |

Fit, measured: a lone figure's feet are at (0.48, 0.63) world units from the tile's centre. At k 18 the hex is 36 px
tall and the figure 26 px; at k 35 the hex is 70 px and the figure 50 px. The figure's rectangle covers the house's
right side as main's shield did. The "hides the house or the clans more than main's shield" check is by eye: the house
shows on every capital view in `shots/medium9-units-play-branch.png`, the clans' dots and the hearth squares are drawn
where main draws them (those layers are under the units and not moved).

## `--diff` (cameras and digests match)

See `diff.txt`: the share of differing pixels per view.

## `--units`: the bar

The bar is on tinted pixels per figure, by the view's zoom (`TINT_BAR` in `shots.mjs`), and `--units` exits 1 under it:

| Zoom | Bar | The least in the views | Why |
|---|---|---|---|
| k 35 | 350 | 407 | the same patch at 3.7× the area of k 18 |
| k 18 | 100 | 124 | a 10 by 10 px patch of a 26 px figure: the tunic reads as a colour, not a speck (the build-2 scout had 31) |
| phone k 6 | 40 | 63 | a 6 by 6 patch of a 14 px figure |
| phone k 5 | 40 | 65 | the flat zoom, 14 px |

Each unit's tinted share and pixels are in `units.txt`. Per view, the tinted pixels (and share of the figure's opaque
pixels) of the scouts and of the camps:

| View | Units | Scouts | Camps |
|---|---|---|---|
| `medium9-units-play` | 6 | 145–151 (34–36%) | 153–155 (43–44%) |
| `medium9-stack-close` | 2 | 412 (26%) | 485 (40%) |
| `medium9-wood-close` | 1 | 412 (26%) | - |
| `medium9-gold-play` | 5 | 128 (30%) | 124–138 (19–39%) |
| `medium9-gold-close` | 2 | 412 (26%) | 485 (40%) |
| `medium8-units-play` | 3 | 127–128 (30%) | - |
| `small2-camps-play` | 5 | 128 (30%) | 138 (20–40%) |
| `small2-camps-close` | 4 | 407–413 (25–26%) | 483–487 (25–40%) |
| `medium9-units-edge` | 6 | 63–67 (43–47%) | 66 (55%) |
| `medium9-units-flat` | 6 | 65–66 (43%) | 66–72 (52–59%) |
| `lineup-close` | 6 | 414–416 (26%) | 483–493 (40%) |
| `lineup-play` | 6 | 128 (30%) | 137–138 (40–41%) |
| `lineup-edge` | 6 | 66 (45%) | 64–66 (50–53%) |
| `lineup-selected` | 6 | 128 (30%) | 137–138 (24–40%) |
| `small2-camps-selected` | 4 | 407–413 (25–26%) | 483–487 (25–40%) |

(A camp's share under 40% is a figure whose opaque pixels include a hungry or selected outline.) Build-2's numbers for
the same views: scouts 31 at k 18, 14 at phone k 6.

## `play/units.mjs`

Passes: six figures on their tiles; every unit's probe nearest its own civ (tinted 414, 416, 416, 493, 483, 491 at k 35);
a click on Gold's tile selects one unit and the next click the other, and the marked figure follows; the two numbers
"2.0" and "4.0" have boxes at least 4 px apart (it was "do not overlap", 2 px apart in build-2); and a new check: with
both units given a "3.5+3.5" stack on the drawn model, the two wide numbers are at least 4 px apart.

## Changes to the easel

- `layers/units.js`: `FIGURE` (`feet`, `slotX`, `slotY`, `gap`, `labelY`, `bowX`, `bowY`, `selected`; new `pad`); the
  next slot's offset is `max(gap, labelX + pad + the number's width)`; the bow's white halo.
- `atlas.js`'s `shaded`: the cloth's alpha doubled at the rim (above).
- `assets/cut.mjs`: a cloth entry may be `{ poly, where }`; `{ op: 'grey', lo }` sets the floor. `draw-atlas.mjs`'s assert: a
  `where` that accepts all is the plain cut.
- `shots.mjs`: `TINT_BAR` and its check in `--units`; a view's `select` (`{ kind, civ }` or `{ tile }`) presses a unit; the
  views `lineup-selected` and `small2-camps-selected`.
- `play/units.mjs`: the 4 px margin, and the wide-number check.
- `proto/crops.mjs`, `proto/shotcrop.mjs`, `proto/bow.mjs` (the bow on a stack of archers, set on the drawn model).
