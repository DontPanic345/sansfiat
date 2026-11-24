# #444 review

## Merged

`origin/main` moved three pieces (#429, #409, M5 fixes; 1c3f69b) and merged clean. Main's `engine/CLAUDE.md` grew to 8,136
bytes, so the merge put it at 8,551, over `check.sh`'s 8 KB budget. The piece's lines are trimmed: `--units` and `--clans`
share one line, `play/clans.mjs` is covered by `all.mjs`'s line, and the clan cells join the unit cells' sentence (8,183).

## Fixed

- **`CLAN.from` is in css px, not device px.** Build-3 chose 28 because a 10 to 14 *css* px tent crowds and reads as a speck;
  every capture and play script runs at one device px per css px, so the device-px test only held there. On a 2× laptop
  the cells began at k 6.8, on a 3× phone at k 4.5, the crowd build-3 turned down.
  `clansFlat` now divides the terrain cell by `dpr`: k 13.6 on every screen, the captures unchanged.
- A typo in `draw-atlas.mjs`'s comment.

## Departures from the spec, judged against the design

- **Phone k 6 keeps main's circles** (`CLAN.from` 28; spec: tent and hearth still read at k 6). Taken: the design asks the map
  to tell the live states apart readably (`map-art.md` 6), and the captures show 10 px cells touching. Settled shows at k 6
  by main's dark square, as before. Recorded in `page.md`'s ledger with the piece's look.
- **Walkers 1.75× a tent of the same people.** Size still tells people within a kind; a walker clan looks bigger than a tent of
  the same people. Acceptable: arrival is a one-turn signal and must read as people.
- **A dark edge on every cell.** Main's circle had a stroke; taken.
- **The lean views show no leaning clan** on this runner; the lineup's two leaning clans carry the claim. A gap in the real-map
  proof of the leaning disc, not in the drawing (the disc is main's code, moved).

## Read cold

- `clans.js`: kind rule `since ≤ 1 → walk`, `settled → hearth`, else tent, as the spec's inventory; leaning discs under all
  cells; one back-to-front order across homes; the flat path is main's code. Selection is by tile, untouched.
- At k 18 on medium 9 Gold's ochre-yellow cloth is near the pale hide's grey-white in small tents: told apart, but less than
  red or blue. Left to piece 8 (the whole-map pass).
- No rule, save or digest moved (`judge.md`).
