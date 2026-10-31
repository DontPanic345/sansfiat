# Baselines, from main before the piece

Taken by the spec step with the release build of `main` at `abfa2cd`. The piece's builds compare against these; none of it is regenerated.

| File | What | How |
|---|---|---|
| `seed1-r16-t400.txt`, `seed7-r16-t400.txt`, `seed4-r20-t400.txt`, `seed3-r23-t150.txt` | `<turn> <digest>` after every turn, watching | `sansfiat digest --seed S --radius R --turns T` |
| `seed2-r16-t400-lead.txt` | The same for the scripted led game. It hashes the log, so it **moves** when the log's shape does (see `spec.md`, "The digests") | `… --lead` |
| `*.state.txt` | `<turn> <state digest>`: the same worlds hashed without their `log` field (the JSON as a `serde_json::Value` with `log` removed, FNV-1a 64 as `stats::digest`). The led game's proof | `… --state` (the flag build-1 adds) |
| `plays/<name>.txt`, `plays/exit.txt` | stdout and exit code of `engine/www/play-{camp,scout,hold,lean,read}.mjs` on main (taken by the spec-review; deterministic: two runs agree). `play-camp` and `play-lean` exit 1 on main | `plays.sh` runs and diffs them |
| `retro-sweep.txt` | `sansfiat sweep --after 200 --maps retro`, byte for byte | deterministic apart from nothing: no timings in it |

Bench (`sansfiat bench`, small, 200 turns, noisy runner): seed 1 1.54 ms a turn, seed 2 3.43, seed 3 1.70. Build-1 takes its own best-of-three on the same runner before it edits, and the review compares against that; more than 10% slower fails.

`compare.sh` builds the release binary and diffs the current build against the four watching `.txt` files, the led run's `.state.txt` and `retro-sweep.txt`: six lines, each `ok` or `DIFFERS`. After build-4 the led run's plain `.txt` is not compared (its digest moves with the log's shape); the state digest is.

`plays/read.txt` was updated in build-6: its refusal line names the build's save version, which build-4 made 2 (`SAVE_VERSION` moves on purpose, spec item 12). The play itself is unchanged and still checks only that #error names the version.
