# #340 build-3: the bench, large 8, 50 turns past the pick

`sansfiat bench --size large 8 --after 50`, ms a turn past the pick, five runs of each binary,
alternating in one session (release builds: the branch, and main in `git worktree add /tmp/main main`).
The branch's final code (`within_march` in `paths.rs`, the planner's paths and reach cached a turn).

| Run | branch | main |
|---:|---:|---:|
| 1 | 35.696 | 41.580 |
| 2 | 35.950 | 41.604 |
| 3 | 35.806 | 41.596 |
| 4 | 35.790 | 41.646 |
| 5 | 35.667 | 41.453 |
| median | **35.790** (−14.0%) | **41.596** |

Main's spread in this session: 41.453 to 41.646 (0.19 ms). The branch's median is 5.7 ms under
main's lowest run, so the planner's step is faster by more than the noise.

An earlier session's first five pairs (before the bound moved into `within_march`, same code
otherwise) gave the branch 35.97 and main 41.64 ms.

This runner gives main 41.6 ms; the spec session's runner gave 25.2 ms and spec-review's 41.3 ms.
Only main in the same session is a control.
