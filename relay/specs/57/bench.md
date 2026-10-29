# #57 bench: the engine before and after the reasons

`cargo run --release -p sansfiat -- bench 1 2 3 --size small` (200 turns, watching), three runs
each, the best ms a turn per seed taken. The runner is noisy; the bar is no more than 10% slower.

## Digests (the save version's pins)

Taken from the code before any change, `digest` on the small map, watching unless `--lead`:

| run | turn | digest |
| --- | --- | --- |
| seed 1 | 336 (pick 316, +20) | `be1197ab970402b9` |
| seed 2 | 303 (pick 283, +20) | `8b866f497fb07b8d` |
| seed 3 | 239 (pick 219, +20) | `85468f7177213aab` |
| seed 2, `--lead` | 303 | `ed8f01e25fd6433b` |

## Before (commit 87cfd6c)

| seed | run 1 | run 2 | run 3 | best ms a turn |
| --- | --- | --- | --- | --- |
| 1 | 1.537 | 1.533 | 1.533 | 1.533 |
| 2 | 3.416 | 3.419 | 3.419 | 3.416 |
| 3 | 1.697 | 1.684 | 1.685 | 1.684 |

## After

The same three runs after the reasons, alerts and `PeopleLeft` (build-1's final code):

| seed | run 1 | run 2 | run 3 | best ms a turn | vs before |
| --- | --- | --- | --- | --- | --- |
| 1 | 1.555 | 1.545 | 1.588 | 1.545 | +0.8% |
| 2 | 3.502 | 3.484 | 3.472 | 3.472 | +1.6% |
| 3 | 1.706 | 1.732 | 1.710 | 1.706 | +1.3% |

The three bests sum to 6.723 ms a turn against 6.633 before: +1.4%, under the 10% bar. The digests
are unchanged (checked after the last change by `the_save_version_pins_the_rules` and `./check.sh`).
