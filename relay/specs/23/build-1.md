# Build 1: Route the maths through libm, lint it

You are doing the first of two builds for #23, "Same maths for native and web". Read
`spec.md` in this folder first for the why (it confirmed libm is the cause). This step changes
no rule and no number except through the maths library; its result is that native and wasm
now compute the same `exp`, `ln` and `pow`.

Work in `engine/`. Checks, all in `engine/`: `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `./build-web.sh`.

## What to build

1. Add `libm = "0.2"` to `core/Cargo.toml` `[dependencies]`; let `Cargo.lock` update (commit it).
2. Replace the transcendental calls in `engine/core`. Find them with
   `grep -rnE '\.(exp|ln|log|log2|log10|powf|powi|sin|cos|tan|atan|atan2|sinh|cosh|tanh|cbrt|hypot)\(' core cli web`
   and confirm the list is exactly:
   - `core/src/clans.rs` `pick_site`: `((v - best) / CLAN.choice_spread).exp()` → `libm::exp((v - best) / CLAN.choice_spread)`
   - `core/src/market.rs` `clear`: `(p.step * excess.clamp(-1.0, 1.0)).exp()` → `libm::exp(...)`;
     and `(goods.iter().map(|&g| prices[g].ln()).sum::<f64>() / goods.len() as f64).exp()` →
     `libm::exp(goods.iter().map(|&g| libm::log(prices[g])).sum::<f64>() / goods.len() as f64)`
   - `core/src/map.rs` the fertility curve: `t.powf(1.5)` → `libm::pow(t, 1.5)`
   - `core/tests/market.rs` (the mean-log-price check): `.ln()` → `libm::log(...)`
   Keep the arithmetic order exactly as it is; only the function changes. If the grep finds
   anything else, route it the same way and say so in your result.
3. Add `engine/clippy.toml` with `disallowed-methods`, one entry per path, each with a `reason`
   that names the replacement (`use libm::exp so native and wasm agree`, …): `f64::exp`,
   `f64::exp2`, `f64::exp_m1`, `f64::ln`, `f64::ln_1p`, `f64::log`, `f64::log2`, `f64::log10`,
   `f64::powf`, `f64::powi`, `f64::sin`, `f64::cos`, `f64::tan`, `f64::asin`, `f64::acos`,
   `f64::atan`, `f64::atan2`, `f64::sinh`, `f64::cosh`, `f64::tanh`, `f64::asinh`,
   `f64::acosh`, `f64::atanh`, `f64::cbrt`, `f64::hypot`, and the same paths on `f32`. Leave
   `sqrt`, `mul_add`, `floor`, `ceil`, `round`, `trunc`, `abs`, `min`, `max`, `clamp` alone.
   Add a comment at the top of the file saying why (one line: the platform's maths library
   differs from wasm's in the last bit, #23).
4. Add a line to `engine/CLAUDE.md` under **Invariants**: floating-point `exp`, `ln`, `powf`
   and the like go through `libm` (clippy's `disallowed_methods` enforces it), so a save
   replays identically native and in the browser.

## Tests and proof that it's done

- With the lint in place and the swap done, `cargo clippy --all-targets -- -D warnings` is clean.
- Prove the lint bites: temporarily put `.exp()` back on one call (in `pick_site`) and
  run clippy; it must fail with your `reason` text in the message. Do the same for one call
  through `f64::exp(x)` path syntax. Note in your result that you did, then undo it. Don't
  commit the violation.
- `cargo test` passes. No existing test should need a number changed; if one does (a test
  that pinned a value the swap moved), look at it: change only what the maths forces, say which
  and why in your result.
- Quick native-versus-wasm sanity (the real check is build-2): `./build-web.sh`, then
  `cargo run --release -p sansfiat -- sweep 1 --radius 16` should say `ended 314 by centres`
  (it said 363 before; wasm said 314 before and still does). Put this in your result.

Commit when the checks pass: "#23 build-1: route exp, ln and pow through libm, lint the rest".
Don't touch `ci.yml`, the CLI or the web crate: build-2 does.
