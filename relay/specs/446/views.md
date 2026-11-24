# #446 build-3 (canvas): what the branch changed and what it shows

Raw outputs: `after-*.txt` (this step's last runs; `baseline-*.txt` is build-1's). Captures: `shots/<view>-{preart,main,branch}.png`.
`after-overlays.txt`: preart | main | branch.

## What changed (all in `engine/www`, no Rust)

- `map/view.js`: `onScreen(i, reach = 0)` culls at max(80 px, reach world units × k + 4 px); `onCanvas(i, reach)` is the same test
  that ignores `noCull`. The trails layer (strokes every tile in one path, so off-canvas pieces change the AA of on-canvas ones: 18 px
  per close view) culls by `onCanvas`. That is what made `--cull` exit 0 again.
- `layers/units.js`: a stack's figures stand `FIGURE.step` (2.6 s, at most `span` 1.6 world in all) apart, centred on the tile, so every
  figure's middle is in its hex; a lone unit's slot is at most `slotMax` 0.6 world right of the centre (the flat zoom's s = 1.2 put it
  at 0.96, outside the hex). A stack's numbers stand over the figures, each on the first row where it clears the numbers already
  there (`rise`, `gapX`): two 3-character numbers stand side by side, a wide "3.5+3.5" goes a row up. A lone unit's number is where it
  was. Culls with `reach` 4 s. A dark edge (`FIGURE.edge` 1.3 css px, as a clan's cell has) under every figure; the probe draws
  without it. `gap` and `pad` are gone.
- `layers/clans.js`: a yard row spreads as wide as keeps each clan's middle inside the hex at its height (`Y.margin`), so the lower
  row of a town of 8 is narrower. Culls with `CLAN.reach` 3.5.
- `layers/places.js`: a dark edge (`PLACE.edge` 1.3 css px) round every place cell; culls with `PLACE.reach` 2.5.
- Legibility (the second attempt): every figure, and every settlement and capital, has an edge in its civ's colour (`FIGURE.civ`,
  `PLACE.civ`, 2 and 1.6 css px) inside its dark edge, both drawn from the cell's silhouette (`atlas.outline`), under the figure. A
  tan figure on grass, or Gold's figure on Gold's land, reads as its civ by the rim at once, as the pre-art shield did. The probe
  draws without either edge, so its tint counts are the cloth's alone. The flat zoom's least figure is `minPx` 7 (was 6): 17 px tall at k 5.
  A town (no civ) has only the dark edge.
- No atlas change (`atlas.md` not needed): the cloth's grey and share are unchanged, because the rim solved the read the cloth could
  not (Gold's cloth is #d6a63b over tan hide: the same hue as the body, whatever its share). No easel change.

## Checks, before (baseline.md) and after

| Probe | before (branch = main) | after |
|---|---|---|
| `--hits` | exit 1, 42 misses on 21+ views | exit 0, 0 misses (`after-hits2.txt`) |
| `--pop` | exit 1, 5 placements differ | exit 0, all 56 placements 0 px (`after-pop2.txt`) |
| `--cull` | exit 1, 10 views 18-20 px | exit 0 (`after-cull.txt`) |
| `--units` / `--clans` / `--places` | exit 0 | exit 0 (`after-units.txt`: `medium9-units-flat` figures 76-93 tinted px against the bar 40 at k 5, `TINT_BAR[5]` held) |
| `--pan` medians, ms | main 3.8-27 | 1.4-5.7, every view under its bar (bar ≥ 8; `after-pan.txt`; pre-art medians in `baseline-pan-preart.txt`) |
| `--time verify` frame | main's | 4.5-29.6, main's within noise (`after-time.txt`) |
| `play/all.mjs` | pass | pass |

`--pan`: slowest frames are still ~35-60 ms on the 15 views at k ≥ 11 (build-2's open item, unchanged).
`--diff` branch against main on all 21 verify views: 0.1-2.3% of pixels differ (`after-diff-main.txt`): the figures', places' and
stack's drawing only; against pre-art (`after-diff-preart.txt`) 16-97% (the art). Cameras and digests match on both, exit 0.

## Per view (pre-art | main | branch)

- **lineup-close (k 35):** pre-art: flat shields, numbers beside. Main: sprites, the Gold stack's second figure in the next hex.
  Branch: Gold's two figures stand in one hex, numbers side by side over them, each figure rimmed in its civ's colour (red, blue, gold
  read apart at a glance). Gold's numbers stand close under Red's "6.0" in the hex above (the lineup puts neighbours close): readable.
- **medium9-play (k 18):** pre-art: Gold's hall with a shield. Main: scout and hall tan. Branch: the scout and the hall each rimmed
  gold, reading as Gold's; the army's gold rim on the gold-rimmed hall is still a close mass (hall, figure, ring), but the figure's
  dark edge separates it.
- **medium9-places-play (k 18):** pre-art: Gold shields, a clear hall. Main: Gold's scout and army tan smudges, the hall a brown
  blob. Branch: the scout is a clear gold-rimmed figure; settlements are gold-rimmed palisades; the capital's hall reads as a rimmed
  hall in its ring, with the army (red hungry edge under it) on it: busier than the pre-art house and shield, no longer a smudge.
- **large8-places-close (k 35):** pre-art: a blue house in a gold ring. Main: hall, tents, a scout over the hall. Branch: hall and
  scout blue-rimmed (the capital is Blue's); the town of 8's every clan's middle is on the town's tile (`--hits`).
- **small2-edge (k 6):** pre-art: a shield beside each house. Main: a brown speck with a faint tint. Branch: each figure is a 14 px
  figure in a red, blue or gold rim: its civ reads at once, as the shield did, though the shape is a rimmed figure and not a shield.
- **medium9-units-flat (k 5):** the same. Figures are 17 px tall (`minPx` 7): Blue's scout in its rim, Gold's scout and army by
  Gold's hall read as Gold's. Tint counts 76-93 against the bar 40 (the bar held). A banner of 14 to 17 px can say its civ by its
  rim; it does not read as a shield, only as a person in a civ's colour.
- **Other verify views** (`--out` ran on all 21 on all three trees, cameras and digests match): the figures' rims show
  at close (small2-close, medium9-clans-close and -places-close, large8-play, medium9-units-play and -edge, places-edge): each as
  above; the woods, clans and terrain are as on main.

## The View overlays (`after-overlays.txt`: % of pixels an overlay moves by > 48; pre-art | main | branch)

main and the branch draw the same (their layers don't touch an overlay). Against pre-art: fertility moves 20% against 46% at
medium9-play, 27% against 55% at medium9-places-close, 3.9% against 8.3% at small2-edge. I read the fertility captures: the
overlay's fill still shows on land at every zoom, and the art's sea and mountains simply don't take it (they are not drawn in the
overlay's colour as the old flat hexes were), so fertility is not buried. Territories move more on the art (13-14% against 6-8%),
hidden costs, country, worn and festival routes the same as pre-art. No overlay needed a fix.

## Not done

- Pan drags by hand on the page (`play.mjs`) for seams: `--pan` ends on the view camera or within ulps, and `play/pan.mjs` passes;
  the review drags at three zooms.
- The capital's hall under an army is still the busiest spot on the map (hall, ring, figure, hungry edge): see medium9-places-play.
