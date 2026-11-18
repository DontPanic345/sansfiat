# #340 notes for the next builder

- `digests.sh` runs in 52 s after a release build; `bench --size large 8 --after 50` in about 4 s.
- On this runner main's large 8 turn is 24.8 to 26.4 ms (`bench-main.txt`), not the 22.9 of M4's
  runner: compare in one session.
- `market::utility` has no caller in a rule, only `cli/src/clear.rs` and a test.
