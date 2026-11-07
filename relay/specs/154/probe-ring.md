# #154 probe: the ring on main

Run by the spec-review on main (270d0d3), throwaway; nothing here is on the branch.

## The judge's runs have no armies

`sansfiat sweep --judge` with a counter in `Watch::turn` (per unit a turn: inside the ring when its
line from where it stands carries what its people eat, `line_from`): **0 army-turns** on the seven
maps over every run. The AI raises no army before R (`units/ai.rs`), and `economy.largest_army` is
0. Scouts over all the sweep's runs: 12,355 turns inside the ring (3 hungry), 11,457 outside (15
hungry). So the judge can't show the ring is where armies live; the proof stages them.

## Staged armies on the retro maps

`ring-probe.rs` (run as `engine/core/tests/zz_ringprobe.rs`, `--ignored --nocapture`, 100 s): on
each retro pick, each civ led in turn, a 3-person army (`Stage::army`, full pack, switches on)
stood on the first land tile within 8 steps of the capital in each band of the ring's share
(`3 × UNIT.food / carry` from `line_from`), 20 turns. Turns whose civ's store ended under 3 food
are dry and not counted; a run stops when the army is gone (not counted after).

| share | armies | counted turns | hungry | rate | gone |
|---|---|---|---|---|---|
| under 0.5 | 21 | 278 | 0 | 0.000 | 3 |
| 0.5–0.75 | 21 | 291 | 0 | 0.000 | 2 |
| 0.75–0.9 | 20 | 270 | 0 | 0.000 | 2 |
| 0.9–1 | 20 | 279 | 0 | 0.000 | 3 |
| 1–1.1 | 20 | 302 | 1 | 0.003 | 4 |
| 1.1–1.25 | 21 | 322 | 52 | 0.161 | 9 |
| 1.25–1.5 | 20 | 248 | 60 | 0.242 | 17 |
| 1.5–2 | 21 | 222 | 72 | 0.324 | 21 |
| 2 and up | 19 | 194 | 74 | 0.381 | 18 |

- Inside the ring no army goes hungry while its civ's store holds. Every army gone inside the
  ring went on a turn its civ's store was empty: small 3 civ 192, small 9 civ 375 and medium 8 civ
  683 have stores of about 10 food at the pick, and a 3-person army drains them in 9–14 turns.
  Counting those turns, inside the ring's rate is 0.08–0.11, all of it the store.
- Past share 1.1 hunger climbs with the share; 1–1.1 is soft, since a full pack spoiling a tenth a
  turn outlasts 20 turns when the line brings nine tenths.
- So the ring as the formula draws it is where staged armies live on the real maps. The bound the
  spec sets (under 1: under 0.05; 1.25 and up: at least 0.2 and four times inside) passes on main.

## The staged walk-out's edge

On a bare plains stage a step costs 1 (1.33 days after build-2), a line there and back 2 a step,
so the share is `2k / 10.5`: 0.95 at five steps, 1.14 at six. At six the line brings 2.63 of the 3
food the army eats; a full pack (15) spoiling 10% a turn falls as `(15.75) 0.9ⁿ − 0.75`, empty near
turn 29. The walk-out runs 40 turns, not `UNIT.pack + 2`.
