# #340 judge

Verdict: **same** (a health piece: no rule or number moves).

- Digests: all 21 (seven retro maps at turn 400, AI, `--lead`, `--state`) equal `digests-main.txt`; `SAVE_VERSION` 26. Re-run in the review: identical.
- `sansfiat judge --compare` of the branch's and main's `sweep --judge --json` (build-3): 231 metrics, 0 changed, 0 flagged.
- `sweep --judge` prints no `pass` or `MISS` (review run: 0 matches).
- Large 8 turn past the pick, five alternating runs in one session (`bench.md`): branch median 35.79 ms, main 41.60 ms (main's spread 0.19 ms): -14.0%.
- Greps: no `UNIT.carry`, `UNIT.pack`, `SCOUT.pack`, `CARRY.load` left; `data.rs` gone, largest `data/` file 257 lines; `engine/CLAUDE.md` 5,418 bytes.
