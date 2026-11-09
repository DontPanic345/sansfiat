# #155 build-4 pick: fields that pay

Chose **both**: B's rules and canvas, with A's way of proving the farmer's doing line on the page.

Both versions ran the same seven retro maps against the same no-fields control in the same sweep, from the same
commit (b656f2d), and both read y8 to the same digits (`judge.md`'s numbers). Every number below is the
version's own, from its `relay/specs/155/canvas.md`; I ran no sweep. The landed engine is byte for byte B's
(`core/`, `cli/` and `data.rs` unchanged from `relay/155-build-4-b`), so B's sweep is the landed sweep. The
only change past B is `www/play/fields.mjs`, a page check, and `check.sh` on the result is my own run.

## The numbers

| Bound | Measure | A (A's run) | B (B's run) | Landed |
|---|---|---|---|---|
| | Setting | grain 0.9, notch 0.05, margin 0.02, farmer 0.1; no rule changed | grain 0.8, notch 0.1, margin 0.01, farmer 0.2; a field is sown only on a tile that yields | B |
| | Settings run | 11 | 19 | B |
| 1 | Farming civs (share ≥ 0.1 at +200) | 10 of 21 | 13 of 21 | B |
| 1 | Their mean Standing gain at +200 | +0.0461 | +0.0407 | B |
| 1 | Gaining | 10 of 10 | 12 of 13 | B |
| 1 | Others' gain | −0.0013 | −0.0086 | B |
| 1 | Farming over others | +0.0474 | +0.0493 | B |
| 1 | Aligned Standing at +200 (control 0.2525) | 0.3162, floor 0.004 | 0.3183, floor 0.004 | B |
| 2 | Field day at a hearth of fertility 1 (gathering 0.75; bound 1.125) | 1.35 | 1.20 | B |
| 3 | Maps where the top farmer holds a pack half its turns (≥ 5) | 5 of 7 | 5 of 7 | B |
| 3 | Civs never at a pack (≤ 2; control 4) | 2 | 1 | B |
| 4 | Home moves per 100 clan-turns, settled / gatherers | 0.042 / 0.114 (0.37×) | 0.031 / 0.122 (0.25×) | B |
| 4 | Moves on per 100, settled / gatherers | 0.004 / 0.037 | 0.000 / 0.051 | B |
| 5 | Clans changed hands (control 127) | 179 | 177 | B |
| 5 | People changed hands (control 629) | 939 | 953 | B |
| 5 | Land changed hands (control 1022) | 1050 | 1150 | B |
| 6 | Starving against the control, raw | 1.262 | 1.222 | B |
| 6 | Its floor | 0.156 | 0.187 | B |
| 6 | Read less the floor (≤ 1.25) | 1.106 | 1.036 | B |
| 6 | Worst map read less its floor (≤ 1.6) | 1.298 (large 8) | 1.211 | B |
| 6 | Settled farmers / gatherers starving share | 0.26% / 2.24% | 0.13% / 2.26% | B |
| 7 | Pre-games by centres | 7 of 7 | 7 of 7 | B |
| 7 | Panics | 0 | 0 | B |
| 7 | AI orders per 100 (control 6.91; ≥ half) | 7.31 | 8.26 | B |
| 8 | `bench --size large 8 --after 50`, branch / base | 81.7 / 118.9 ms (under check.sh's load) | 53.2 / 121.7 ms | B |
| 9 | `check.sh` | green (A's) | not rerun end to end after its last edit (B's) | green (my run) |
| 9 | `www/play/fields.mjs` proves the farmer's doing line | yes, seed 1 (A's) | no: on seed 9 the check is skipped (B's) | yes, seed 1, farmer after 3 turns (my run) |
| | Page checks with a changed seed | fields.mjs 9 → 1, layout.mjs and select.mjs 1 → 2 | none | fields.mjs 9 → 1 |
| | `SAVE_VERSION` | 16 | 16 | 16 |

## What the diffs showed

- **The rule.** B changes one rule: `fields::jobs` sows only tiles with ground and a yield above 0, so a clan
  never spends days on grain still wild, and a daughter that inherits a share onto a tile with no field gathers
  with all its days. It sits inside the piece's Decided (the yield grows with fertility and tame; nobody orders
  a clan to farm), and tiles still reach a yield through the tame a held tile gains. A changed no rule.
- **The question the brief asked.** The brief asked who the starving gatherers are. B answered it: the excess
  is a growth transient of the first 100 turns, daughters of a growing people starting on one tile, and by
  turns 175–200 the rate equals the control's. A tuned until the bound passed and did not say who starves.
- **Room on the bounds.** A's chosen setting is one combination among its eleven that passes, its raw
  starving (1.262) is over 1.25 and passes only less its floor, its worst map raw is 1.645, and it holds the
  most civs never at a pack the bound allows (2). Its own table shows grain 0.9 alone at 1.39 read: the pass is
  a narrow draw. B passes with room on every starving reading and on the store's never count.
- **More farmers.** B has 13 farming civs to A's 10: fields pay for more of the civs that live beside them.
  A's slower split (notch 0.05) leaves more civs under the 0.1 share at +200.
- **The page check.** B made the doing-line check conditional on a hardcoded 0.2 and on seed 9 it never runs,
  so the test stopped proving the line. A waits for a clan at `rules().field.farmer` and plays seed 1, which
  proves it. On B's rules seed 1 gives a farmer after 3 turns and every fields.mjs check passes, so that part
  of A landed. A's layout.mjs and select.mjs seed moves were for A's rules (Red's army starved on seed 1);
  on B's rules they pass on seed 1, so they didn't land.

## What landed

`relay/155-build-4-b` merged, then `www/play/fields.mjs` from A's approach: the loop waits for a clan of the
led civ at `rules().field.farmer`, the doing line is checked unconditionally, and the game is small seed 1.
`engine/check.sh` on the result: green, run in full (fmt, clippy, every test, the web build, cross-build, check.mjs, and all.mjs with fields.mjs, layout.mjs and select.mjs passing).
