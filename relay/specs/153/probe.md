# #153 spec probe: the flat trail and the errand on main

Run by the spec step on `main` at `b377337` (#152 merged), in a scratch worktree.

## The flat trail

`map.rs`'s `roll_trails` gave plains and coast a trail cost of `bare × FLAT` (no random draw,
so the rest of the map rolls as before), read from an environment variable. With `FLAT=1.0` a
trailed plains tile costs what it does on main (`step_cost` takes the less of bare and trail),
and the scout ignores flat land (no `TRAIL.work` entry), so `1.0` is main. Trails wear by
traffic as now; the scout builds none on flat land. `sansfiat sweep --judge --json` on the
seven retro maps; `--floor` (one null perturbation at the pick) for `1.0` and `0.75`.

The floor is a perturbation after the pick. A flat trail changes the pre-game, so each map is a
different game and the floor understates the noise: read a move that holds in the same
direction across 0.85, 0.75 and 0.5 as real, and a move that flips as noise.

| Total | Main (1.0) | floor | 0.85 | **0.75** | 0.5 |
|---|---|---|---|---|---|
| Aligned share at +200 (Standing) | 0.209 | 0.008 | 0.247 | **0.236** | 0.276 |
| Top Standing at +200 | 0.106 | 0.005 | 0.114 | **0.108** | 0.125 |
| Trading share at the pick | 0.898 | 0.000 | 0.937 | **0.929** | 0.967 |
| Trading share at +200 | 0.897 | 0.001 | 0.932 | **0.925** | 0.971 |
| Festival changes per 100 clan-turns | 0.65 | 0.01 | 0.39 | **0.39** | 0.22 |
| Festivals at the pick | 218 | 0 | 203 | **211** | 208 |
| AI orders per 100 turns | 13.3 | 0.7 | 11.7 | **9.0** (−32%) | 7.0 (−48%) |
| Land tiles changing hands | 2448 | 392 | 1806 | **1612** (−34%) | 1522 |
| Clans joined | 123 | 25 | 129 | **90** | 76 |
| People changing hands | 570 | 95 | 641 | **461** | 394 |
| Lead changes (maps) | 10 (4) | 3 (1) | 10 (5) | **3 (1)** | 5 (4) |
| Spread at +200 | 0.071 | 0.008 | 0.065 | **0.058** | 0.071 |
| Held tiles a civ at +200 | 21.3 | 1.1 | 23.4 | **21.8** | 19.8 |
| Store mean | 17.1 | 0.6 | 18.3 | **16.1** | 21.0 |
| Pre-games ended by centres | 7 of 7 | | 7 of 7 | **7 of 7** | 7 of 7 |

Read:

- **Trade and Standing move, every value, the same way**: the trading share rises by 0.03 to
  0.07 against a floor of 0.001, and the aligned share by 0.03 to 0.07 against 0.008. A flat
  trail ties clans to their festival (changes fall by 40%) and pulls more of them into civs.
- **The two bounds the piece names hold at 0.75**: every pre-game ends by centres, and the AI's
  orders fall by a third, not by half (its claims, mostly: claims 11.7 → 6.9 a hundred turns).
  At 0.5 the AI's orders fall by 48%, at the bound.
- **Motion falls** by a third at 0.75 (land), a quarter at 0.85. Not stilled: past the floor,
  under half.
- **Doubt is noise here**: lead changes read 10, 10, 3 and 5 across main, 0.85, 0.75 and 0.5,
  with no trend. Reported, not a bound.

So the value is settled at **0.75**, the owner's starting value (M4 answer 4): it meets both
bounds and moves trade and Standing. The canvas re-measures it on the finished branch.

The pre-game sweep (`sansfiat sweep --size all`, seeds 1–10 on each size):

| Size | Main (1.0): ended on their own, top-up, median end | 0.75 |
|---|---|---|
| small | 10 of 10, 0, turn 256 | 10 of 10, 0, turn 262 |
| medium | 10 of 10, 0, turn 375 | 10 of 10, 0, turn 351 |
| large | 9 of 10 (one at the limit with three centres), 0, turn 369 | 9 of 10 (the same), 0, turn 383 |

The pre-game ends as on main on every size: no top-up anywhere, and the same one large seed
reaches its limit with three centres already.

## The errand on main

`cargo test --release -p engine --test scout_errand -- --ignored --nocapture`: the first civ
led, its scout routed at the pick to the nearest rival capital and to `Lead`'s target (the first
forest or hills tile seven steps out), 50 turns, no other order.

| map | to | 2 people (main): hungry turns / starved / alive | 1 person: hungry turns / starved / alive |
|---|---|---|---|
| small 2 | rival | 36 / 2 / no | 44 / 1 / no |
| small 2 | Lead's target | 0 / 0 / yes | 0 / 0 / yes |
| small 3 | rival | 42 / 2 / no | 46 / 1 / no |
| small 3 | Lead's target | 40 / 2 / no | 42 / 1 / no |
| small 6 | rival | 8 / 2 / no | 33 / 1 / no |
| small 6 | Lead's target | 42 / 2 / no | 40 / 1 / no |
| small 9 | rival | 37 / 2 / no | 35 / 1 / no |
| small 9 | Lead's target | 0 / 0 / yes | 10 / 1 / no |
| medium 8 | rival | 29 / 2 / no | 30 / 1 / no |
| medium 8 | Lead's target | 1 / 0 / yes | 0 / 0 / yes |
| medium 9 | rival | 45 / 2 / no | 44 / 1 / no |
| medium 9 | Lead's target | 39 / 2 / no | 43 / 1 / no |
| large 8 | rival | 44 / 2 / no | 45 / 1 / no |
| large 8 | Lead's target | 44 / 2 / no | 46 / 1 / no |
| **lost** | | **11 of 14** | **12 of 14** |

With #152's start (no army taking the store first) main loses 11 of 14, not 13. A one-person
scout loses 12: half the eating for half the labour leaves it no better off, because what kills
it is what it can get, not what it eats. It strips its ground and leaves it fallow, and on an
errand past the line's reach the line can't make up the difference. So the person count is not
the lever. The feeding rule is (spec items 14–18), and the canvas tunes its numbers.
