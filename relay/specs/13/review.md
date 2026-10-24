# review: Review the piece

Piece #13 is a proof, so its PR is never merged. The review answers its question and writes
the findings into the PR description:

> Does a sparse world of clans, walking their rounds and paying real labour to reach their
> food, spread across the land and form centres with markets around fertile areas on its
> own, with no rule placing them?

Read `relay/specs/13/spec.md` (its design questions, as Alan answered them in
`docs/design/clans.md`), `relay/specs/13/tuning.md`, and the diff
(`git diff origin/main...HEAD -- engine`).

## Look hardest at

1. **No rule places anything.** Search the rules for every read of `fertility`, `cap` and
   `stock`. Fertility may enter only through the caps (`map.rs`). Markets may form only from
   where clans live (held homes and clans in reach), centres only from markets, hearths only
   from held grain. A rule that scores sites by fertility, pulls clans towards markets, or
   picks centres by anything but the market rules answers the question falsely: remove it and
   say so.
2. **Labour is real.** Walking the round comes off first, then the market trip, then each
   harvest; off-round tiles pay 2 × path cost once; a clan never uses more labour than
   `pop × work`. Check `plan_harvest`, `harvest` and `go` against the spec, with a test for
   anything wrong.
3. **The round's rules.** Cheapest insertion, one change a turn, starving swaps, home a tile
   a turn towards the middle, the 2-step limit, `claim` matching rounds. The invariant test
   over 300 turns should hold; check it checks what it says.
4. **Determinism.** No `HashMap` iterated in a rule, all randomness through `rand`, ties
   broken; `replaying_a_save_rebuilds_the_world` covers clans and markets.
5. **Scale.** Per-clan work must not grow with the number of clans: per-clan searches are
   bounded by `territory_radius`, `fetch_range`, `scout_range` and market `reach`, never a
   loop over all clans or tiles per clan. Compare ms a turn at radius 12 and 20 against the
   number of clans.

## The evidence

- `cargo run --release -p sansfiat -- sweep 1 2 3 4 5 6 7 8 9 10 --radius 16`, and a second
  sweep at another radius (say 20) to see it isn't one map's luck.
- For one or two seeds, `run --every 50`: how the clans spread, when markets and hearths
  first appear, and the centre report (fertility near each centre against the land's mean).
- Screenshots of the web map at the end of a run (fertility view on, and off), if the runner
  has a headless browser: commit them under `relay/specs/13/` and link them from the PR body
  by their raw URL on this branch. Without one, say so.

## The findings

Write the PR description with `gh pr edit --body`, keeping its first line,
"Proof for #13". Then:

- **The answer**: yes, no, or partly, in a sentence or two.
- **The evidence**: the sweep table, the centre fertility ratios, the turns things happen,
  screenshots.
- **What it took**: the numbers tuning changed (from `tuning.md`) and the shortcuts the spec
  lists.
- **What a merge would need**: what to keep, what to rework, and the open questions for
  Alan (for example the starting values that mattered most, or a rule that didn't pull its
  weight).

Then the usual review: fix what's substantive with a test, remove anything outside the scope
map (list it in `ideas`), wait for CI to go green.
