# #339 judge

The piece changes no rule: `stats::digest` pins and `SAVE_VERSION` are unchanged, so the four scores (Standing,
doubt, decisions, motion) cannot move and no sweep was run. The question is health, counted in `recount.md` and
recounted in the review with `count.md`'s rule on the branch:

| Row | Control (main, 63b941e) | Target | Branch |
|---|---|---|---|
| Page scripts reading a seeded pick | 19 of 22 | 5 of 23 | 6 of 23 (5 without bar's pre-game check) |
| Core test files calling `Stage::retro` | 10 of 45 | 8 of 45 | 8 of 45 |

`node www/play/all.mjs` on the branch (review run): 23 scripts, all pass, about 190 s against main's 174.7 s over 22
(`tests.mjs`, new, is 22.6 s of it; every staged script is faster than on main).
