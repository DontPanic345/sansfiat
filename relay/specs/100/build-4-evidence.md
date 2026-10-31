# Build-4 evidence: the orders (#100)

Run on `relay/100` after the build's last edit, from `engine/` and the repo root.

| Check | Command | Result |
|---|---|---|
| Everything CI runs | `./check.sh` | exit 0 (fmt, clippy -D warnings, cargo test, web, cross-build with five runs, `www/check.mjs`) |
| Watching digests, every turn | `bash relay/specs/100/baseline/compare.sh` | six `ok`: seed 1, seed 7, seed 4, seed 3 (plain digests), led state digest (seed 2, `--lead --state`), retro sweep |
| Speed | `sansfiat bench 1 2 3`, three runs, best | seed 1 1.546, seed 2 3.454, seed 3 1.702 ms a turn; limits 1.714, 3.818, 1.881 (10% over `bench.md`'s before numbers) |
| AI never refused | `sansfiat digest --turns 300` and `--lead`, seeds 4 to 12 (not 7), small and medium | no panic in 32 runs |

Digests pinned by `cli/tests/save.rs`:

- Watching, unchanged: seed 1 at turn 336 `be1197ab970402b9`, seed 2 at turn 303 `8b866f497fb07b8d`, seed 3 at turn 239 `85468f7177213aab`.
- Led, plain: `ea1c746b5be2e2b2` (moved, because the log's shape changed; was `ed8f01e25fd6433b`).
- Led, state: `46e67d48ffcbe520` (unchanged).

Not run here: `plays.sh` (build-6's proof). The page still sends the old action shapes until build-6.
