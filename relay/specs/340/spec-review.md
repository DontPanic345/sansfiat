# #340 spec-review: what running settled

## The planner's path searches, large 8, 50 turns past the pick

`path_between` timed by caller over the 50 turns. A "repeat" is a (from, to) already searched the
same turn. The times are inflated by the timing itself; read them as shares.

| Caller | Calls | ms | Repeats in the turn | ms in repeats |
|---|---:|---:|---:|---:|
| `muster`'s `near` (store → rival capital) | 394 | 309.3 | 0 | 0 |
| `path_cached` (`front`, `post_by`, `direct`) | 138 | 109.8 | 56 | 42.2 |
| orders the planner gives (`orders.rs`, inside `w.order`) | 149 | 32.8 | 89 | 31.3 |
| other steps (`fight.rs`, …) | 139 | 35.9 | 91 | 32.2 |
| `raid` | 0 | 0 | 0 | 0 |
| `across` | 1 | 0.0 | 0 | 0 |

So a `Paths` that lasts the turn saves only the `path_cached` repeats: about a tenth of the
planner's path time. `near` is two-thirds of it and never repeats: each civ asks from its own
stores. `raid`'s search never runs on large 8, so its flood fill doesn't show in the bench.

## `near`, bounded

`turns` walks at most `UNIT.labour` of step cost a turn (`stride`, to 1e-9), so a path costing more
than `PLAN.march × UNIT.labour` takes more than `PLAN.march` turns. A `cost_map` from each store,
bounded there (×1.001 + 1e-6), drops every capital past it before `path_between` runs. The capitals
inside still get `path_between` and `turns`, so the answer is the same. The prototype is
`near-bound.diff`.

- Digests: all 21 match `digests-main.txt`.
- Bench, alternating in one session on this runner (ms a turn):

| Run | main | bounded `near` |
|---|---:|---:|
| 1 | 41.547 | 35.732 |
| 2 | 41.406 | 35.716 |
| 3 | 41.253 | 35.683 |
| 4 | 41.099 | 35.557 |
| 5 | 41.333 | 35.548 |
| median | **41.333** | **35.683** (−13.7%) |

This runner gave main 41.3 ms, against the spec session's 25.2: the bench moves by runner,
nearly 2×, so only main in the same session is a control. Its spread in one session was 0.45 ms.
