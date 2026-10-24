# Review: #23, Same maths for native and web

Review the whole piece against `spec.md` in this folder, then fix what's wrong, and write the PR
description. The piece changes no rule: anything that moves a rule, a number in `data.rs` or a
test's pinned value beyond what the maths swap forces is a finding.

Checks, in `engine/`: `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`./build-web.sh`, `./cross-build.sh`. Run them all.

## What to check

- **Every transcendental call goes through `libm`.** Grep `engine/` (core, cli, web, tests) for
  `.exp(`, `.ln(`, `.log`, `.powf(`, `.powi(`, `.sin(`, `.cos(`, `.tan(`, `.atan`, `.tanh(`,
  `.cbrt(`, `.hypot(` and for paths like `f64::exp`. None should remain outside `libm::`. The
  arithmetic order at each call site is unchanged from `main` (`git diff main -- engine/core`): a
  swapped call that also reordered a sum would move results for no reason.
- **The lint**: `engine/clippy.toml` lists the paths in the spec and clippy is clean. Prove it
  bites yourself, once: temporarily add a `.exp()` in `core/src` and run clippy; undo.
- **The check can fail and compares what it claims**: read `cross-build.sh` and
  `cross-build.mjs`. Both sides print the same lines (`<turn> <digest>`), the native side runs
  the full 400 turns past the pre-game's end, the wasm side uses the freshly built `www/pkg`, the
  script exits non-zero on a diff, and `ci.yml` runs it after `./build-web.sh` with Node set up.
  Look for ways it could pass vacuously: both outputs empty, a missing file ignored, a diff of
  the same file with itself, `set -e` swallowed by a pipe.
- **Other divergence sources**: build-2's result says whether the check found any. Re-grep
  `engine/core` for `HashMap`, `HashSet`, `f32`, `usize` arithmetic that could differ between 32
  and 64 bits, anything reading a clock or the platform. Fix any you find (in scope), without
  changing a rule.
- **The sweep**: rerun `sweep --radius 16` and `--radius 20` (release) and compare with
  `evidence.md` and the table in `spec.md`; they must match exactly.
- **The browser**: rerun the headless Chrome check for seed 1, radius 16 (recipe in `spec.md`)
  and confirm the turn matches the native sweep's.
- Scope: nothing retuned, nothing in `docs/design/` touched, nothing about `trade`/`fade`/`form`
  scale loops.

## The PR description

The piece asks a question in effect: do the native and wasm builds now give the same world, and
was libm the whole cause? Rewrite the PR description (`gh pr edit --body`, as your session prompt says), drawing on
`relay/specs/23/evidence.md` (build-2's notes). Start with the answer in one or two sentences,
then the evidence, and end with your verdict:

- the answer, plain: yes or no, and for what seeds and turns it was checked;
- the cause and the swap experiment (363 vs 314 → 314 and 314, same digests);
- the sweep tables beside #16's, saying what moved and that no rule changed;
- which seed was checked in the browser and the turn it ended on, and that CI checks the wasm
  build under Node, not a browser;
- the lint, in a line, and the cross-build check's failing run;
- anything left open (other platforms, a pinned golden digest) as a short list.

Say honestly what you did not verify. Commit fixes: "#23 review: …".
