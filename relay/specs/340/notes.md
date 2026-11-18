# #340 notes for the next builder

- `digests.sh` runs in 52 s after a release build; `bench --size large 8 --after 50` in about 4 s.
- On this runner main's large 8 turn is 24.8 to 26.4 ms (`bench-main.txt`), not the 22.9 of M4's
  runner: compare in one session.
- `market::utility` has no caller in a rule, only `cli/src/clear.rs` and a test.
- Spec-review's runner gave main 41.3 ms a turn on large 8 and the digests took 76 s: the
  runners differ nearly 2×. `digests.sh` runs from `engine/`.
- `orders.rs` calls `path_between` when the planner's orders are checked (`w.order`), about 3 a
  turn on large 8, half of them repeats; it isn't the planner's cache, and a later piece could
  pass the turn's paths in.
