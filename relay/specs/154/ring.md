# #154 build-3: the ring

Run on the branch (`relay/154`, build-3): `cargo test --release -p engine --test ring -- --ignored
--nocapture` for the table, and `sansfiat sweep --judge --json` for the `supply` group. The table
beside `probe-ring.md`'s main (the spec-review's probe, run on main).

## The retro maps by band of the ring's share

Each civ led in turn, on all seven retro picks: a 3-person army staged on the first land tile within
8 steps of the capital in each band of `units::ring`'s share, 20 turns. A turn counts unless its
civ's store ends under the army's turn of food (dry). When the army is gone, a dry store ends the
count, and otherwise the turns left count as starving (the brief's rule).

| share | armies | counted turns | starving | rate | dry turns | gone |
|---|---|---|---|---|---|---|
| under 0.5 | 21 | 321 | 0 | 0.000 | 89 | 1 |
| 0.5–0.75 | 21 | 323 | 0 | 0.000 | 86 | 1 |
| 0.75–0.9 | 19 | 310 | 0 | 0.000 | 59 | 1 |
| 0.9–1 | 18 | 278 | 0 | 0.000 | 71 | 1 |
| 1–1.1 | 19 | 331 | 6 | 0.018 | 34 | 2 |
| 1.1–1.25 | 19 | 312 | 56 | 0.179 | 49 | 5 |
| 1.25–1.5 | 21 | 350 | 132 | 0.377 | 25 | 19 |
| 1.5–2 | 21 | 377 | 212 | 0.562 | 11 | 21 |
| 2 and up | 19 | 348 | 227 | 0.652 | 7 | 18 |

Under share 1: 6 starving of 1,232 counted turns, rate 0.005 (bound: under 0.05). From 1.25 up: 566
of 1,075, rate 0.53 (bound: at least 0.2, and at least four times the under-1 rate). Both bounds
hold; the test asserts them.

**Against main's probe (`probe-ring.md`).** The probe's rates from 1.25 are 0.24–0.38, and build-3's
are 0.38–0.65. The difference is the count, not the armies: the probe stopped counting when an army
was gone, and build-3 counts the turns left as starving, as the brief asks. The under-1 bands match
(0.000 with one or two 1–1.1 starving turns). Armies inside the ring still starve only where their
civ's store has run dry (dry turns are set apart and the gone armies are counted on live stores only).

## The staged walk-out

A straight line of plains from the led civ's capital, a 3-person army `k` steps out with a full
pack and both switches on, 40 turns. The civ's store is topped up to its cap (`CIV.store_cap`,
60 food) before each turn: the line drains the store to keep the pack full, and the capital's clan
eats from it too, so without the top-up the store, not the line, runs the army out. The share at one step is
0.19. Every step under share 1 stays fed for 40 turns; the first step at share 1 or more is the sixth,
which goes hungry inside the 40. The test asserts both for steps 1 to 10.

## The sweep's supply group

`sansfiat sweep --judge --json` on the branch, the main run of each map (the sweep's `supply` group per
map; the totals are its sums and the means over maps). Held counts are in brackets. Armies: no army is
raised before R on the sweep, so the army-turns are 0 inside and outside. Scout-turns are the main run's.

| map | ring tiles at the pick (held) | ring tiles at +200 (held) | scout-turns inside | hungry | scout-turns outside | hungry | clan territory, pick | clan territory, +200 |
|---|---|---|---|---|---|---|---|---|
| small 2 | 36 (3) | 46 (43) | 305 | 0 | 295 | 0 | 2.67 | 2.58 |
| small 3 | 104 (3) | 104 (85) | 2 | 0 | 598 | 3 | 3.58 | 3.48 |
| small 6 | 148 (3) | 148 (94) | 206 | 0 | 394 | 1 | 3.35 | 3.46 |
| small 9 | 82 (3) | 85 (40) | 400 | 0 | 200 | 0 | 3.25 | 3.16 |
| medium 8 | 126 (3) | 130 (101) | 709 | 0 | 41 | 1 | 2.78 | 2.73 |
| medium 9 | 79 (3) | 85 (56) | 448 | 0 | 302 | 0 | 4.15 | 4.10 |
| large 8 | 69 (3) | 69 (62) | 371 | 0 | 529 | 0 | 2.69 | 2.63 |
| **total** | 644 (21) | 667 (481) | 2441 | 0 | 2359 | 5 | 3.21 (mean over maps) | 3.16 |

Scouts inside the ring: 0 hungry of 2441 turns, rate 0.000 (bound: under 0.05, met). Outside: 5 of 2359, rate 0.0021 (reported).

Over all three runs each map carries (main, no-claims, no-fight; the JSON has no other), the scout-turns
are 7,746 inside with 0 hungry and 6,654 outside with 17 hungry. Main's figures in `probe-ring.md`
(3 of 12,355 inside, 15 of 11,457 outside) were counted over the sweep's five runs a map; this JSON
carries three, so the two denominators are not the same runs and are not reconciled here. The rates
are close: main 0.0002 inside and 0.0013 outside; here 0.0000 inside and 0.0026 outside.

A ring tile is one whose share is under one; a held ring tile is one of those the civ holds at that
moment (`claim::held`). A unit at its capital is inside (share 0).
