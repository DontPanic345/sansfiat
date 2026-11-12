# #336 Land without a guard: progress

## Plan

1. Rules (core): the claim is the land (`claim.rs`), only armies fight (`units/fight.rs`), a capital
   falls to a rival army with no army of its own on it, the guard and militia are gone from `data.rs`,
   `orders.rs`, `units/`, `civs.rs`, the judge. **Done** in the first attempt (commit 5010754): core and
   the CLI build.
2. The planner (`units/ai.rs`): holds land with armies, takes a capital no army stands on. **Done**,
   tuning open (see Scores).
3. The web crate and the page: guard row, guard figures, `check.mjs`, `play/*.mjs`. In progress.
4. Tests: `core/tests`, `cli/tests` compile against the new rules, one test per Decided line. In progress.
5. Scores: `sweep --judge` on the branch beside `main-sweep.json`. First run: `branch-sweep-1.json`.
6. Played games (`sansfiat play`, a Sonnet player), ledger lines, PR.

## Scores so far (branch-sweep-1.json against main-sweep.json)

| | main | branch |
|---|---|---|
| games ending early | 0 of 7 | 0 of 7 |
| capitals fallen | 0 | 2 (small 3, small 6) |
| claim tiles taken (motion.claim_taken) | 0 | 8 |
| fight.taken | 14 | 4 |
| largest army (mean) | 14.1 | 4.0 |
