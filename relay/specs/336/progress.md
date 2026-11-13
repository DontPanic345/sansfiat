# #336 Land without a guard: progress

## Plan

1. Rules (core): the claim is the land (`claim.rs`), only armies fight (`units/fight.rs`), a capital
   falls to a rival army with no army of its own on it, the guard and militia are gone from `data.rs`,
   `orders.rs`, `units/`, `civs.rs`, the judge. **Done** in the first attempt (commit 5010754): core and
   the CLI build.
2. The planner (`units/ai.rs`): holds land with armies, takes a capital no army stands on. **Done**,
   tuning open (see Scores).
3. The web crate and the page: guard row, guard figures, `check.mjs`, `play/*.mjs`. **Done** (964c6b7).
4. Tests: every target compiles; about 30 tests in 15 targets still fail, each for a reason
   `build-1.md` names. Handed on to build-1.
5. Scores: `scores.md` (`branch-sweep.json` beside `main-sweep.json`). **Done.**
6. The skirmish story (`cli/src/story.rs`) takes the rival's tiles one at a time; `SAVE_VERSION` is 20.
7. With `Config.fight` off no land is taken (`claim::step`), so the switch is still the control.
8. Handed on in `steps.json`: build-1 (the tests), the crew's play step, and the review.

## For the review and the owner

A civ starts with no army, and a capital no army of its own stands on falls to the first rival
army to walk onto it. On small seed 2 with the land made plains, the led civ sitting idle loses
its capital five turns after the pick; on the map as generated it loses four tiles in 17 turns
and still stands at 40. With every civ on the AI two capitals fall in seven games. A player's
first order has to be a raise.

## Scores so far (branch-sweep-1.json against main-sweep.json)

| | main | branch |
|---|---|---|
| games ending early | 0 of 7 | 0 of 7 |
| capitals fallen | 0 | 2 (small 3, small 6) |
| claim tiles taken (motion.claim_taken) | 0 | 8 |
| fight.taken | 14 | 4 |
| largest army (mean) | 14.1 | 4.0 |

## Tests (build-1)

The tests are done against the guardless rules: `cargo test --release` and `engine/check.sh` pass whole. A three-civ fight test (#291) is in `fight.rs`. `cargo fmt` also reformatted a few lines in `core/src` (no rule changed).
