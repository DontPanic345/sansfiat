# #340 build-3: evidence

- `relay/specs/340/digests.sh` (from `engine/`, after `cargo build --release`), diffed against
  `digests-main.txt`: 21 of 21 lines identical, on the code as committed (run twice: before and
  after the `within_march` move).
- `relay/specs/340/bench.md`: large 8, 50 turns past the pick, five alternating runs each in one
  session: branch median 35.79 ms, main 41.60 ms (main's spread 0.19 ms).
- `sansfiat sweep --judge --json` before and after the change: the new report has no `"pass"`;
  `sansfiat judge --compare` on the two new reports lists 231 metrics, 0 changed, 0 flagged.
- `grep -rn '"pass"\|MISS' engine/cli/src`: only `map.rs`'s `pass` tile glyph.
- `engine/check.sh` exit 0, whole, after the last code edit (`cargo fmt --check` clean).
- `engine/CLAUDE.md` 5,418 bytes; `core/src/units/ai.rs` 631 lines (634 on main), the new
  `paths.rs` 63 lines.
