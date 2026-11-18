# #340 build-2: evidence

- `relay/specs/340/digests.sh` (from `engine/`, after `cargo build --release`) diffed against
  `digests-main.txt`: 21 of 21 lines identical.
- `grep -rn 'UNIT\.carry\|UNIT\.pack\|SCOUT\.pack\|CARRY\.load' engine/` (excluding `target/`): no matches.
- `engine/check.sh` exit 0, whole, on the branch after the last code edit (a later doc-only wrap in
  `units/line.rs` was checked by `cargo fmt --check` and `cargo build -p engine`).
- Touched tests pass: `actors`, `units`, `scout_routes`, `markets`, `market`.
