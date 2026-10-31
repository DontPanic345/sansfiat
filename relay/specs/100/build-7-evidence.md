# Build-7 evidence: the CLI split, shared test fixtures, engine/CLAUDE.md

## Tests

`cargo test --workspace`: 237 passed, 0 failed, 0 ignored, before and after. No `#[ignore]` anywhere.
No warnings from `cargo test --workspace --no-run`.

## CLI output, before and after

Captured with the pre-split binary (built from HEAD) and the split binary, into the same set of files:
`run` (seeds 2 and 3, small, 120 turns), `run --why 1`, `run --save` then `replay` (text and
`--events-json`), `map` (seeds 2 and 3), `sweep` (seeds 2 and 3, small, 120 turns), `bench` (seeds 2
and 3, small, 60 turns; the `ms` timings are masked before comparing), `sweep --after 200 --maps
retro`, and `digest --lead --state`. 18 output files, every one identical.

Also `bash relay/specs/100/baseline/compare.sh`: six `ok` lines (four watching digests, the led
state digest, the retro sweep).

## Sizes

`wc -l engine/cli/src/*.rs`: main.rs 65, run.rs 105, map.rs 61, digest.rs 46, sweep.rs 273,
retro.rs 211, replay.rs 22, bench.rs 36, clear.rs 47, report.rs 121. No file is over 275 lines.

## Checks

`engine/check.sh`: exit 0 (fmt, clippy with -D warnings, cargo test, build-web, cross-build, www/check.mjs).

`bash relay/specs/100/baseline/plays.sh`: five `ok` lines.

## Shared fixtures

`core/tests/common/mod.rs` holds once each: `markets3`, `found`, `fill_stores`, `three`, `led`,
`camp_id`, `put_camp`, `camp_mut`, `end_turn`, `total`, `kinds`, `made`, `nbrs` and `centre`.
Copies removed from units.rs, civs.rs, alerts.rs, events.rs (its `centre`, and its `kinds` renamed
`all_kinds`), pregame.rs, clans.rs, leaning.rs, map.rs, rivers.rs and closed.rs.

Kept apart because the bodies differ: `pregame::three_centres` (takes the clans' people, on a flat
map), `orders::led_game` (returns the world only), `closed::generated` (the closed map's
`Generated`, not a tile Vec), `market::total` (sums a list of baskets, and that file does not import
common). `land` (clans, rivers) and `walk` (rivers, wear) take different arguments and stay local.
