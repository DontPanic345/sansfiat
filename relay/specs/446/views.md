# #446 build-3 (canvas): what the branch changed and what it shows

Raw outputs: `after-*.txt` (this step's last runs; `baseline-*.txt` is build-1's). Captures: `shots/<view>-{preart,main,branch}.png`.
Session ran short of its usage window: **legibility is partly done** (see "Not done").

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
- No atlas change (`atlas.md` not needed). No easel change.

## Checks, before (baseline.md) and after

| Probe | before (branch = main) | after |
|---|---|---|
| `--hits` | exit 1, 42 misses on 21+ views | exit 0, 0 misses (`after-hits2.txt`) |
| `--pop` | exit 1, 5 placements differ | exit 0, all 56 placements 0 px (`after-pop2.txt`) |
| `--cull` | exit 1, 10 views 18-20 px | exit 0 (`after-cull.txt`) |
| `--units` / `--clans` / `--places` | exit 0 | exit 0 (`medium9-units-flat` scout 715 included, `TINT_BAR[5]` 40) |
| `--pan` medians, ms | main 3.8-27 | 1.8-7.3, every view under its bar (bar ≥ 8; `after-pan.txt`; pre-art medians in `baseline-pan-preart.txt`) |
| `--time verify` frame | main's | 4.5-29.6, main's within noise (`after-time.txt`) |
| `play/all.mjs` | pass | pass |

`--pan`: slowest frames are still ~35-60 ms on the 15 views at k ≥ 11 (build-2's open item, unchanged).
`--diff` branch against main on the six shot views: 0.26-1.9% of pixels differ (`after-diff-main.txt`): the figures' edges, the stack's
spacing, the places' edges. Cameras and digests match.

## Per view (pre-art | main | branch)

- **lineup-close (k 35):** pre-art: flat shields, numbers beside. Main: sprites, the Gold stack's second figure in the next hex.
  Branch: Gold's two figures stand in one hex, numbers side by side over them; they sit near the number of the Red unit in the hex
  above-left (the staggered lineup puts neighbours close), readable but crowded.
- **medium9-play (k 18):** all three read; branch adds the dark edge, figures separate from the ground a little more.
- **medium9-places-play (k 18):** pre-art: Gold shields and a clear hall. Main: Gold's scout and army are tan smudges, the capital's hall
  a brown blob in the ring. Branch: the dark edge separates the figures from the grass and outlines the hall, a little better;
  Gold on Gold's land is **still muddy** (tan tunic under gold cloth), and the capital's hall plus its clans are still a brown mass.
- **large8-places-close (k 35):** the town of 8: every clan's middle is now on the town's tile (`--hits`); the lower row is narrower.
- **small2-edge (k 6), medium9-units-flat (k 5):** a 14 px figure is a brown speck with a faint civ tint; the pre-art shield reads
  as its civ at once. `--units` passes by its tint count (≥ 40 tinted px at k 5), but by eye the banner does **not** read as its
  civ as well as the pre-art shield does.

## Not done (for the review or a next build)

- Legibility by eye of Gold's units and the capital's hall, and the flat-zoom figure: the cloth needs a brighter or larger share
  on a tan tunic (atlas tool recolour with `--check`), or the flat zoom (k < 8.3, s = minPx) needs a larger figure or a
  shield-like mark. Tried: only the dark edge on figures and places.
- `--overlays` after the art (fertility, territories, ...) were not re-read: nothing in this step touches them; `--overlays`
  shows main's = branch's in `baseline-overlays-*.txt`, with fertility reading weaker than pre-art (20% against 46% of pixels).
- `--out` on all 21 verify views on all three trees was not run; only the six shot views (cameras and digests match).
- Pan/Shift seam drags on the page (`play.mjs`) not done by hand.
