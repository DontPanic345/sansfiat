# Spec: #23, Same maths for native and web

## What the piece delivers

The same seed gives the same world natively and in the browser, so a save replays on either
build. PR #16 found they come apart: seed 1 at radius 16 ends on turn 363 natively and 314 in
the browser. The piece changes no rule and has no design file.

- **The `libm` swap.** Every transcendental float function the simulation calls goes through
  the `libm` crate, on both builds. `sqrt`, `+`, `-`, `*`, `/`, `min`, `max`, `abs`, `floor`
  and `round` are exact under IEEE and stay as they are.
- **A lint** so `f64::exp`, `ln`, `powf` and their kin can't come back: clippy's
  `disallowed_methods`, in `engine/clippy.toml`, caught by the `cargo clippy` CI already runs.
- **A cross-build check in `ci.yml`**: a few seeds run for 400 turns on the native build and
  on the wasm build (under Node), and the digest of the whole world after every turn must be
  identical.
- **The evidence**: `sweep 1..10` at radius 16 and 20 rerun after the swap, the table beside
  #16's, and which seed was checked in a browser and the turn it ended on.

Out of scope: retuning or changing any rule, even if the sweep shifts (the thin trade #16
found is a later piece); the scale loops #16 noted (`trade`, `fade`, `form`).

## The cause, confirmed before the builds

The spec session ran the experiment, so the builds don't have to wonder:

1. On `main` (no change), a headless run of the wasm build under Node gave seed 1, radius 16:
   ends turn **314**; native `sweep 1 --radius 16`: ends **363**. #16 reproduced. A real
   headless Chrome (`google-chrome --headless=new --dump-dom` on a page that imports
   `pkg/web.js`, runs seed 1 until `summary().end` is set) also ends on turn **314**.
2. Swapping only the four call sites below to `libm::exp`, `libm::log` and `libm::pow` made
   native end on **314**, and the SHA-256 of the whole world's JSON at the end was identical
   between native (`run --seed 1 --radius 16 --json`) and wasm: `2c4f7511d90a6b8e…`. The wasm
   hash did not move at all from its value before the swap: Rust's wasm build already uses a
   port of musl's libm, which is what the `libm` crate is. So the native side was the odd one
   out, and the `libm` crate pins the answer on every platform (including MSVC and macOS).
   Spot checks of seeds 2, 5 and 9 at radius 16 end on the same turns in wasm as native
   (all three on the limit, turn 499).

So libm is the whole cause for seed 1, at least up to the end turn and the full end state. The
other sources the piece listed were audited and aren't present in `engine/core`: no `HashMap`
or `HashSet` (`BTreeMap` throughout), one rng (`mulberry32` on `u32`), no `f32`, no `mul_add`,
no threads or clocks; float sums are sequential iterator sums in a fixed order. `usize` is 32
bits in wasm and 64 natively; nothing the engine hashes or overflows depends on it, and the
experiment above agrees. If the cross-build check (build-2) ever shows a divergence after the
swap, find the cause and fix it; that is in scope.

The call sites (all of `engine/core`; `grep -rnE '\.(exp|ln|log|powf|powi|sin|cos)\('` finds
them):

| File | Code | Becomes |
|---|---|---|
| `core/src/clans.rs:304` (`pick_site`) | `((v - best) / CLAN.choice_spread).exp()` | `libm::exp(...)` |
| `core/src/market.rs:78` (`clear`) | `(p.step * excess.clamp(-1.0, 1.0)).exp()` | `libm::exp(...)` |
| `core/src/market.rs:80` (`clear`) | `(Σ prices[g].ln() / n).exp()` | `libm::exp(Σ libm::log(prices[g]) / n)` |
| `core/src/map.rs:97` (fertility curve) | `t.powf(1.5)` | `libm::pow(t, 1.5)` |
| `core/tests/market.rs:53` | `c.prices[g].ln()` | `libm::log(...)`, the same function the code uses |

## Decisions the piece left open

1. **`libm` is a dependency of `engine` (core) only**, `libm = "0.2"` in `core/Cargo.toml`
   (the experiment used 0.2.16). Cli and web get it through core. Cargo.lock pins the version,
   which is what keeps the two builds on the same code.
2. **The lint lives in `engine/clippy.toml`** (workspace-wide; cli and web don't use these
   anyway) and disallows, with a reason naming the `libm` function: `f64::exp`, `exp2`,
   `exp_m1`, `ln`, `ln_1p`, `log`, `log2`, `log10`, `powf`, `powi`, `sin`, `cos`, `tan`, `asin`,
   `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `cbrt`, `hypot`
   and the same list on `f32`. I confirmed `disallowed-methods` with these paths flags method
   calls on `f64` including in `core/tests` under `--all-targets`. `powi` is on the list
   because it lowers to a compiler-rt call that isn't promised to match across targets.
   `sqrt`, `mul_add`, `floor`, `ceil`, `round`, `trunc` are exact and stay allowed.
3. **The check compares a digest of the whole world after every turn**, not just the summary:
   the summary is lossy (counts and means), and a one-bit difference in a price could hide in
   it for many turns. The digest is FNV-1a 64 over `serde_json::to_string(&world)`, printed as
   16 hex digits. `serde_json` prints floats with shortest-round-trip (`ryu`), the same on both
   targets, so equal worlds give equal text. A per-turn line `turn digest` also tells a failing
   run the first turn at which the builds part, which is where to look.
4. **The digest is code in the engine, shown by both front ends**: `stats::digest(&World) ->
   String` in core (so `serde_json` moves from core's `[dev-dependencies]` to `[dependencies]`),
   `Sim.digest()` in the web crate, and a CLI `sansfiat digest [--seed N] [--radius N]
   [--turns N]` that prints a line `<turn> <digest>` after every turn (turn 0 first). Not a
   test inside `cargo test`, because the point is to run the *wasm* build.
5. **The wasm side runs under Node, in CI.** `engine/cross-build.mjs` loads
   `www/pkg/web_bg.wasm` with `initSync` and prints the same lines as the CLI. Node's V8 is the
   engine Chrome runs, and wasm float arithmetic is IEEE-exact apart from NaN payloads, so Node
   exercises the same code a browser does, without driving a browser in CI. A real browser
   check is done once, by hand, for the PR (see below), not in CI.
6. **Seeds and length**: `engine/cross-build.sh` runs seeds 1 and 7 at radius 16 and seed 4 at
   radius 20, 400 turns each, always the full 400 (the world keeps stepping after the
   pre-game ends, so seed 1's end at turn 314 and the turns after it are both covered). Those
   three end by centres at turns 314/306/320 after the swap, so end detection (which reads
   fertility, markets and rounds) is covered. It exits non-zero and prints the first differing
   line of each seed that disagrees. About 1200 digests over ~1 MB of JSON each, a few seconds
   to a minute; if a run turns out slower than that, cut the seeds, not the turns.
7. **The check must be able to fail.** Build-2 shows it: with one call site temporarily put
   back to `f64::exp` (under `#[allow(clippy::disallowed_methods)]`, not committed) the script
   fails at some turn, and it is recorded in the evidence. Without that the green check
   proves nothing.
8. **No change to the pinned save format** or any rule. A save made before this piece may not
   replay to the same world after it (the native numbers moved); that's fine, nothing ships yet.

## Evidence the PR needs

Build-2 writes `relay/specs/23/evidence.md`; the review turns it into the PR description.

- The cause: the 363 vs 314 reproduction and the swap-only experiment (the numbers above, redone
  on the real change), in a line or two.
- **The sweep tables** `sweep 1..10 --radius 16` and `--radius 20`, in the shape of #16's table
  (seed, r16 end turn and how it ended, r16 fertility ratio, same for r20, the "N of 10" row),
  with #16's table beside it. #16's numbers are the *native* ones from before the swap, so seeds
  shift; that's expected and not a bug. Pre-checked in the spec session (end turn only; the build
  reruns them for the real table):

  | seed | r16 end | r20 end |
  |---|---|---|
  | 1 | 314 centres | 311 centres |
  | 2 | 499 limit | 330 centres |
  | 3 | 359 centres | 264 centres |
  | 4 | 499 limit | 320 centres |
  | 5 | 499 limit | 342 centres |
  | 6 | 499 limit | 499 limit |
  | 7 | 306 centres | 328 centres |
  | 8 | 485 centres | 499 limit |
  | 9 | 499 limit | 499 limit |
  | 10 | 499 limit | 499 limit |

  (4 of 10 by centres at r16, 6 of 10 at r20; #16 had 3 and 5.) If the build's numbers differ
  from these, something other than the swap changed; find out why before writing the table.
- **The browser check**: seed 1, radius 16, in headless Chrome against the built `www/pkg`,
  and the turn it ended on (314 after the swap), next to the native 314. The recipe: serve a
  small page that imports `./pkg/web.js`, loops `sim.act('{"type":"endTurn"}')` until
  `JSON.parse(sim.summary()).end` is set, and writes `end` into the DOM; run
  `google-chrome --headless=new --no-sandbox --disable-gpu --virtual-time-budget=100000
  --dump-dom http://localhost:PORT/`. Chrome is installed on the runner and in `ubuntu-latest`.
  The page and the server are throwaway, outside the repo or deleted.
- The cross-build check's failing run (turn it parted at) and its passing run.
- The lint's failing output on a deliberate violation.

## Scope map

| Step | In-scope line it serves |
|---|---|
| build-1: Route the maths through libm, lint it | The `libm` swap; the lint |
| build-2: Cross-build check in CI, sweep, evidence | The cross-build check in `ci.yml`; rerunning the sweep; the browser check; any other source of divergence it finds |
| review | Every line; writes the answer into the PR |

## Open questions

None for the owner. Things a later piece might want, in `ideas` rather than here: a pinned
golden digest for a seed (it would also catch a rule change by accident, so it isn't wanted
now); running the cross-build check on Windows and macOS runners; a real-browser check in CI.
