# Build 2: The cross-build check in CI, the sweep, the evidence

Second and last build of #23, "Same maths for native and web". Read `spec.md` in this folder
first: it has the why, the decisions (the digest, the seeds, Node for the wasm side) and the
evidence the PR needs. Build-1 already routed `exp`, `ln` and `pow` through `libm` and added
the lint; read its result. This step adds the check that proves the two builds agree, wires
it into CI, reruns the sweep and records the evidence. It changes no rule.

Work in `engine/`. Checks, all in `engine/`: `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `./build-web.sh`, and the new `./cross-build.sh`.

## What to build

1. **`stats::digest(w: &World) -> String`** in `core/src/stats.rs`: FNV-1a 64 over the bytes of
   `serde_json::to_string(w).unwrap()`, returned as 16 lowercase hex digits (`{:016x}`).
   Offset basis `0xcbf29ce484222325`, prime `0x100000001b3`, wrapping multiply. Move
   `serde_json` from `[dev-dependencies]` to `[dependencies]` in `core/Cargo.toml`
   (`serde_json.workspace = true`). Say in the doc comment what it's for: one value that
   changes if any bit of the world does, so two builds can be compared. Nothing else in core changes.
2. **`Sim.digest()`** in `web/src/lib.rs`, returning `stats::digest(&self.world)`.
3. **`sansfiat digest [--seed N] [--radius N] [--turns N]`** in `cli/src/main.rs` (add to
   `USAGE`; parse flags the way `run` does; defaults `Config::default()` and `--turns 400`).
   It builds the world and prints `<turn> <digest>` for turn 0, then after each of `--turns`
   turns, one per line, nothing else. It runs the full number of turns whether or not the
   pre-game has ended (`World::run(1)` keeps working after the end).
4. **`engine/cross-build.mjs`**: Node (24, ESM, no dependencies). Arguments `seed radius
   turns`; it imports `./www/pkg/web.js` (resolve the path from `import.meta.url`, use
   `pathToFileURL` and `initSync({ module: readFileSync('www/pkg/web_bg.wasm') })`; the
   `--target web` build has no auto-loading of the wasm under Node; spec-review checked that a
   dynamic `await import(pathToFileURL(...).href)` of `www/pkg/web.js` works under Node 24 with
   no `package.json`), makes
   `new Sim(seed, radius)`, and prints the same `<turn> <digest>` lines as the CLI (turn 0
   first, then `sim.act('{"type":"endTurn"}')` and `sim.digest()` after each turn).
5. **`engine/cross-build.sh`** (`set -euo pipefail`, `cd "$(dirname "$0")"`, executable): for
   each of `1 16`, `7 16`, `4 20` (seed, radius): 400 turns; build the CLI once with
   `cargo build --release -p sansfiat`, run `target/release/sansfiat digest` and
   `node cross-build.mjs`, `diff` the two outputs. On a difference print which seed and radius,
   and the first differing line of each side (`diff … | head`), and exit non-zero at the end
   (run all three before failing, so one run shows every seed that parts). On success print one
   line per seed: `seed 1 radius 16: 401 digests agree`. It expects `www/pkg` to be built
   (`./build-web.sh` first); say so in a comment and fail with a clear message if
   `www/pkg/web_bg.wasm` is missing. Put it in the temp dir for outputs (`mktemp -d`), not the repo.
6. **`ci.yml`**: in the `engine` job, after `./build-web.sh`, add `- run: ./cross-build.sh`.
   Node 24 is on `ubuntu-latest`'s runner image by default but the `engine` job doesn't set it
   up: add `actions/setup-node@v4` with `node-version: 24` before it, like the other jobs.
7. **`engine/CLAUDE.md`**: add `digest` to the command list and to the layout (`stats.rs` row,
   `web/` row for `digest`), plus `./build-web.sh && ./cross-build.sh` as the command that
   compares the builds, and one sentence under Invariants that the two builds must give the
   same digest for the same seed.

## Tests

- A core test in `core/tests/stats.rs`: `digest` is the same for two worlds built and run alike,
  is 16 hex digits, and differs after one more turn. (The cross-build check itself is the test
  that the builds agree.)
- Run `./build-web.sh && ./cross-build.sh`: it must pass (all three seeds agree on 401 digests).
- **Show the check can fail.** Temporarily put one call site (`pick_site`'s `exp`) back to
  `f64::exp` under `#[allow(clippy::disallowed_methods)]`, rebuild both targets and run
  `./cross-build.sh`: it must fail and name the first turn at which a seed's builds part. Record
  that turn and seed in the evidence, then undo it completely (`git diff` clean of it) and run
  it green again. Don't commit the violation.
- If the passing run ever shows a divergence, the swap was not the whole story: find the source
  (float summation order, a `HashMap`, platform-specific code, a `usize` width) and fix it,
  and say what it was in the evidence and your result. That is in scope. Don't change a rule.

## The sweep and the evidence

After the check passes, run, release build:

```
cargo run --release -p sansfiat -- sweep --radius 16
cargo run --release -p sansfiat -- sweep --radius 20
```

Compare against the pre-check table in `spec.md` (end turn); if they differ, find out why
before going on. Then write `relay/specs/23/evidence.md`, the raw material for the PR
description, with:

- the cause, in a few lines: native 363 vs wasm 314 before; after the swap both 314 with
  identical digests (numbers from your own runs);
- both sweep tables in the shape of #16's (columns: seed, r16 end and how it ended, r16
  fertility ratio, r20 end and how, r20 ratio, plus the totals row) **beside** #16's table, which
  is (end turn / ratio, `-` for none):

  | seed | r16 end | r16 ratio | r20 end | r20 ratio |
  |---|---|---|---|---|
  | 1 | 363 centres | 1.19 | 380 centres | 1.10 |
  | 2 | 499 limit | - | 499 limit | - |
  | 3 | 373 centres | 1.06 | 352 centres | 1.04 |
  | 4 | 499 limit | 1.40 | 269 centres | 1.00 |
  | 5 | 499 limit | 1.18 | 499 limit | 1.24 |
  | 6 | 499 limit | 1.05 | 271 centres | 1.08 |
  | 7 | 348 centres | 0.97 | 305 centres | 1.15 |
  | 8 | 499 limit | 1.19 | 499 limit | - |
  | 9 | 499 limit | - | 499 limit | - |
  | 10 | 499 limit | - | 499 limit | - |
  | | **3 of 10**, mean 1.15 | | **5 of 10**, mean 1.10 | |

  and one sentence on what moved (seeds shift because both builds now use the same, different
  libm, not because any rule changed; #16's answer about thin trade and fertility is not
  reopened here, but say if the mean ratio or the number of seeds ending by centres changed);
- **the browser check**: seed 1, radius 16, in headless Chrome against the built `www/pkg` (the
  recipe is in `spec.md`; write the throwaway page and server under `/tmp`, not in the repo),
  and the turn `end.turn` reports; it should be 314, the native sweep's number;
- the cross-build check's failing run (which seed, which turn the builds part) and its passing
  run (the output lines);
- the lint's failing output from build-1's result, one line.

Commit when the checks pass: "#23 build-2: cross-build check in CI, sweep and evidence".
Say in your result what the check compared and anything surprising.
