# #151 build-6 (ford): `judge --compare`, the floor and `bench --after`

Read `spec.md` items 18–20, build-5's summary (the split: `cli/src/sweep_judge.rs` and
`cli/src/game_judge.rs`), `cli/src/sweep_judge.rs` (`Report`, `judge`, `played`),
`cli/src/main.rs` (USAGE and dispatch: `judge FILE` is `judge_game` today), `cli/src/bench.rs`
and `cli/src/retro.rs` (`to_pick`).

## 1. `cli/src/compare.rs` (new)

- `flatten(&serde_json::Value) -> BTreeMap<String, f64>`: every numeric leaf by its path,
  `a.b.c`; an array of objects is keyed by the object's `id` field, or `"<size> <seed>"` for
  a map object (it has `size` and `seed`), else by index. Booleans and strings are skipped.
  Skip the report's `seconds`, `floor` and `panics` fields.
- `compare(a, b) -> Comparison`: per path in either report: A, B, change, floor, flagged. A
  path only in one report is "only in A" or "only in B", never flagged. The floor for a path
  is the largest of A's and B's `floor` entry for it; with neither report carrying `floor`,
  nothing is flagged and the output's last line says "no floor in either report: nothing is
  flagged". Flagged is `|B − A| > floor`.
- `print`: grouped by the top-level group in this order: standing, doubt, decisions, motion,
  land, economy, armies, fight, health, then any other; a fixed-width table with the columns
  `metric A B change floor` and a `*` on a flagged row. By default only the `total.` paths;
  `--all` adds the per-map paths. End with counts: listed, changed, flagged, only in A/B.
- `main.rs`: `sansfiat judge --compare A.json B.json [--all]` (the existing `judge FILE`
  stays); USAGE gets the line. A missing file or bad JSON is an error naming the file.

## 2. `sweep --judge --json --floor [--nulls N]`

In `sweep_judge.rs`: with `--floor`, after the base sweep, run the sweep again `N` times
(`--nulls`, default 1) with the perturbation of `spec.md` item 19: after `ChooseCiv` at the
pick, take `N`'s k-th extra draw from `w.rng` (`engine::rng::rand(&mut w.rng)` k times) and
nothing else. `played` takes the number of draws as a parameter (0 for the base). The report
gains `floor: BTreeMap<String, f64>`: per flattened path, `max over the nulls of |null − base|`.
`--floor` without `--json` is refused with a message; `--nulls` without `--floor` too. Without
`--floor` the report is byte for byte as before. Wire the flags in `sweep.rs`'s parser
(and USAGE).

## 3. `bench --after N`

In `bench.rs`: `--after N` runs each seed to the pick untimed (use `retro::to_pick`), picks no
civ (`ChooseCiv { civ: None }`), and times `N` turns, printing
`seed S: world W ms, pre-game P ms, N turns past the pick T ms (X ms a turn)`. `--turns`
without `--after` is as before; both together is an error. `docs/architecture.md`'s Scale row:
its "On main" cell reads what `bench --after 50` gives on `--size large 8` now (run it and
write the number).

## Tests

- `cli/tests/compare.rs` on small JSON strings: flatten keys (arrays by id and by index, maps
  by size and seed), a change within the floor is not flagged and one past it is, a metric in
  one report only is listed and never flagged, no floor flags nothing, A against A has no
  changes.
- `bench --after 3` on a small map prints its line (an integration test, as `cli/tests` do).

## Measure and record: `relay/specs/151/floor.md`

On the branch: `sansfiat sweep --judge --json > a.json` (main's economy-free JSON is not
needed); `sansfiat sweep --judge --json --floor > b.json`; then `sansfiat judge --compare a.json
b.json` (the same run twice, with b's floors: changes 0) and `--compare` of `a.json` against
a third run `c.json` made with `--nulls 1` perturbation at a different draw (your choice,
say which): how many of the total metrics changed, how many are past their floor, and how
many floors are 0. If most floors are 0 (one null is too lucky, for lack of variance in the
metric), say so and set `--nulls`' default to 3, as a decision in `result.json` (and update
the USAGE). The compare takes a minute or two; run the sweeps once each.

## Done

`cargo test`, `clippy`, `fmt`; `sweep --judge` without `--floor` is byte-identical to
before but for `seconds`; `engine/CLAUDE.md`'s command list gets `judge --compare` and
`bench --after` (stay under 8 KB).
