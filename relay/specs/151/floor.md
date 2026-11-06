# #151 build-6: the noise floor, `judge --compare`, `bench --after`

Run on the branch, after build-6's code (debug build, which is opt-level 3 by the workspace's `dev` profile).

## The three runs

- `a.json`: `sansfiat sweep --judge --json`, the base run (no floor).
- `b.json`: `sansfiat sweep --judge --json --floor`, with `--nulls` at its default of 1. Its metrics are
  the base run's, and it carries `floor`: per path, the largest change one null made to it.
- `c.json`: a sweep at a different draw. The base run took 3 extra rolls at the pick instead of 0 (a
  temporary local patch to `judge`, built, run and reverted; it is not in the commit). So c is one
  perturbation of the same kind as b's null, at a draw that b's null did not use.

## Results

**Same run, with floors** (`judge --compare a.json b.json`): no change, as it must be. The total
metrics: 58 listed, 0 changed, 0 flagged.

**One null's floor against a different draw** (`judge --compare b.json c.json`; b is the one with
floors, since a has none and the floor must come from a report that carries it):

| | totals (58 metrics) | all metrics (1814 rows, `--all`) |
|---|---|---|
| changed (B ≠ A) | 37 | 891 |
| past their floor (flagged) | 14 | 401 |
| floor is 0 | 20 | 905 of 1811 floors |

The totals flagged: `standing.on_land_200`, `doubt.lead_changes`, `doubt.swings`,
`decisions.by_kind.move`, `decisions.by_kind.raise`, `decisions.levers_repeated`, `motion.land_changed`,
`land.festival_changes_per_100`, `land.flips_per_100`, `land.markets_200`, `economy.store_max`,
`armies.camps_lost`, `armies.people_vs_control`, `armies.warriors_starved`.

The three rows in `--all` that are only in B (`maps.small 3.decisions.by_kind.raise`, and the same under
`no_claims` and `no_fight`) are a kind that occurred in c and not in b. A `by_kind` entry exists only when
its kind occurred, so the flatten reads a kind absent from one run as "only in", never flagged, and with
no floor for it. That is a gap in the compare, not a fix made here: a kind that appears in one run is
never past its floor.

## Reading

- **The floor is not 0 on most metrics**, so the brief's rule for raising `--nulls`' default to 3 does not
  apply: 20 of 58 totals (34%) and 905 of 1811 floors (50%) are 0, which is not most. Among the zero
  totals are the exactly-stable counts (`motion.claim_taken`, `motion.clans_left`, `health.panics`,
  `health.digest_mismatches`, `health.pregame_by_centres`) and the fight group's counts (`fight.fallen`,
  `fight.taken`, `fight.surrendered` and their controls, which are 0 when fighting is off by construction).
- **One null under-covers.** 14 of the 37 totals that changed between b and c are past b's floor. A floor
  from one null is one sample of the noise, and it is a small one: a metric whose null happened to move
  little gets a small floor. This is the reason the spec said to check, and the check says one null gives a
  usable scale but a loose threshold: a flag is a lead to read, not a verdict. `--nulls 3` takes the
  maximum over three samples, so its floors are never smaller and it flags no more than this did, by
  construction; how many fewer was not measured (the brief says to run each sweep once).
- The default stays at 1, as the owner said ("the sweep once more"). The decision is in `result.json` for the
  review to overrule.

## Timing

- `sweep --judge --json`: 41.6 s wall (the base run).
- `sweep --judge --json --floor`: 80 s wall (the base run and one null).
- `sweep --judge` without `--floor` against the pre-change commit (ead129a, built in a worktree): the JSON is
  byte for byte the same but for `seconds`, and so is the text report but for its `seconds` line.

## Bench

`bench --size large --after 50 8` on the release build (`cargo build --release -p sansfiat`): 2 runs,
world 110.9 and 111.6 ms, pre-game 4403.8 and 4402.8 ms, 50 turns past the pick 1141.7 and 1142.3 ms, that is
22.8 ms a turn in both. (The debug build gave 27.6 ms a turn on the same map.)
