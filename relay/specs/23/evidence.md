# Evidence: #23, Same maths for native and web

## The cause

Seed 1, radius 16 ended on turn **363** natively and **314** in the wasm build (re-run on `main`:
`sweep 1 --radius 16` says 363; the wasm build under Node says 314; the spec session also got 314
in headless Chrome). The native build called the platform's libm (glibc) for `exp`, `ln` and `powf`;
Rust's wasm build carries its own port of musl's libm. The two differ in the last bit, the markets'
and clans' choices feed that back turn after turn, and the worlds part. After routing `exp`, `ln`
and `pow` through the `libm` crate (the musl port, the same code on every target), seed 1 ends on
**314 on both**, and the digest of the whole world is identical after every turn.

## The sweep, before (#16, native, glibc) and after (this PR, native = wasm)

Before (#16):

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

After (`cargo run --release -p sansfiat -- sweep --radius 16` and `--radius 20`):

| seed | r16 end | r16 ratio | r20 end | r20 ratio |
|---|---|---|---|---|
| 1 | 314 centres | 1.21 | 311 centres | 1.15 |
| 2 | 499 limit | - | 330 centres | 1.33 |
| 3 | 359 centres | 1.17 | 264 centres | 1.20 |
| 4 | 499 limit | - | 320 centres | 1.06 |
| 5 | 499 limit | 1.18 | 342 centres | 1.05 |
| 6 | 499 limit | 1.14 | 499 limit | - |
| 7 | 306 centres | 0.96 | 328 centres | 1.24 |
| 8 | 485 centres | 1.23 | 499 limit | - |
| 9 | 499 limit | - | 499 limit | - |
| 10 | 499 limit | - | 499 limit | 1.39 |
| | **4 of 10**, mean 1.15 | | **6 of 10**, mean 1.20 | |

(Ratios on a limit end are the centres the world had at the limit, as in #16; `-` is none.)

What moved: seeds shift because both builds now use the same, different libm, not because any rule
changed. The number of seeds ending by centres went from 3 to 4 at radius 16 and from 5 to 6 at
radius 20; the mean ratio held at 1.15 (r16) and rose from 1.10 to 1.20 (r20). #16's answer about
thin trade and fertility is not reopened here. The end turns match the spec session's pre-check
table exactly.

## The browser check

Seed 1, radius 16, headless Chrome (`google-chrome --headless=new --no-sandbox --disable-gpu
--virtual-time-budget=100000 --dump-dom`) on a throwaway page under `/tmp` importing the built
`www/pkg/web.js`, ending turns until `summary().end` is set: **`end.turn` = 314, by centres**. The
native sweep says 314. (`end.turn` is 314 while the world's turn counter reads 315 at that point; the
page's `sim.digest()` there, `35886197d4dedb7b`, is the native `sansfiat digest --seed 1 --radius 16`
line for turn 315.)

## The cross-build check

Passing, `./build-web.sh && ./cross-build.sh` (about 30 s):

```
seed 1 radius 16: 401 digests agree
seed 7 radius 16: 401 digests agree
seed 4 radius 20: 401 digests agree
```

Failing, to show it can: `f64::exp` put back in `market::clear`'s price step (under
`#[allow(clippy::disallowed_methods)]`, not committed), both targets rebuilt:

```
seed 1 radius 16: the builds part at turn 39; ...
seed 7 radius 16: the builds part at turn 53; ...
seed 4 radius 20: the builds part at turn 73; ...
```

All three seeds part, the script exits 1, and it ran all three before failing. The brief named
`pick_site`'s `exp`; I tried that first and the three seeds still agreed for 400 turns, so glibc and
musl give the same bits for every input that call site sees in these runs. That call site stays
guarded by the lint; the check only catches a difference that shows up in these three runs.

## The lint

```
error: use of a disallowed method `f64::exp`
   --> core/src/clans.rs:304:87
   = note: use libm::exp so native and wasm agree
```
