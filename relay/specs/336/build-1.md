# #336 build-1: the tests against the guardless rules

The rules, the planner, the page, the web crate and the scores are built and on the branch
(`progress.md`). Every test target compiles. `cargo test --release --no-fail-fast` in `engine/`
leaves about 30 tests failing in 15 targets, all because the rules moved under them. Make each
pass as below, changing tests only, then run `engine/check.sh` whole.

Change no rule and no number in `engine/core/src` or `engine/cli/src`. If a test can't pass
without one, stop and say which in `result.json` as `blocked`.

## The rules the tests meet

- A civ's claim is its land (`claim::held`, `claim::tile_civ`): no guard, no paper, no floor.
- `claim::step` runs after `units`: a rival army (never a scout) that ends its turn on a tile of
  a claim, not the capital, where no army of the owner holds, takes it with whatever that cuts
  off (`ClaimTaken`, `amount` the tiles lost). Nobody fights for it. With `Config.fight` off no
  land is taken.
- Only armies are parts of a fight (`Who::Army`). A clan never defends.
- A capital falls when a rival army stands on it and no army of its civ stands on it or holds
  it (`fight::falls`). A civ starts with no army, so on a retro map an idle led civ's capital
  falls within a few turns of the pick: its units go and its market fades.
- The planner marches on any capital it outweighs, and a capital no army stands on is always
  outweighed (`units::outweighs`). Its claim widens while `armed × PLAN.hold` covers one tile
  more, and never shrinks except under a rival army.

## A test whose subject isn't war

Where a test's led civ sits idle past the pick and the test is about scouts, hunger, routes or
the pre-game, turn the fight off on its world or stage as it is made
(`w.config.fight = false`, or `stage.world_mut().config.fight = false`). Say so in one comment
line. This covers:

- `core/tests/alerts.rs`, `a_led_camp_going_hungry_alerts_once_and_its_starving_once_per_loss`
  (drop the sentence about a guard from its comment).
- `core/tests/cases.rs`, the errand's stage (line 40).
- `core/tests/scout.rs` `lead`, `core/tests/scout_orders.rs` `lead` and its stage at line 194,
  `core/tests/scout_errand.rs` (lines 55 and 193), and `core/tests/scout_feed.rs` if it fails.
- `core/tests/pregame.rs`, `pregame_ends_with_three`: run a turn at a time and stop when
  `w.end` is set, instead of `w.run(PREGAME.max_turns)`, so the centres are read at the pick.

## Test by test

- `core/tests/civs.rs`
  - `a_civ_serialises_...`: `KEYS` has no `"guard"`.
  - `a_card_counts_...`: Red's people are 12.5 (no guard raised); fix the two comments.
- `core/tests/claim.rs`, `a_rival_army_ending_its_turn_on_land_...`: the claim order's tiles
  aren't connected. Use `at_big(-3, 0)` for `at_big(-3, 1)` throughout the test.
- `core/tests/fight.rs`
  - `a_march_stops_on_the_first_tile_of_a_rivals_land`: a march no longer stops on land nobody
    stands on. Rename it `a_march_crosses_a_rivals_land_nobody_stands_on` and assert the army
    ends on its destination, `at_big(3, 0)`.
  - `a_tile_taken_from_an_ai_civ_is_not_fought_for_again`: the AI drops a tile a rival army
    stands on from its claim before `claim::step`, so `ClaimTaken` there is 0 or 1: assert
    `<= 1`, keep the other two asserts, and fix the comment.
- `core/tests/leaning.rs`: `(-5, 1)` is on the civ's land now, so its clan is levied and leans
  by `LEANING.land`. In `nothing_is_posted_or_given_below_keep`,
  `a_civ_posts_what_it_holds_beyond_keep_...` and `leanings_fade_and_those_under_the_floor_go`
  put that clan on `at_big(-5, 2)` (two steps out, off the claim). If a number still moves,
  work out why from `civs::levy` and `leaning.rs` before changing the expected value.
  `a_home_on_a_civs_land_leans_to_it_by_the_land_gain_a_turn` is new and weak: make it assert
  that the clan on the land leans to the civ by exactly `LEANING.land` more than the one off it.
- `core/tests/planner.rs`
  - Add a helper `garrison(w, civ, people)`: a second army for `civ` on its capital, made as
    `fight.rs`'s `new_army` makes one, so the capital is no prize.
  - `the_claim_widens_toward_a_clan_...`: rename it
    `the_claim_widens_toward_a_clan_while_its_armies_can_hold_it`. Drop the `poor` lines that
    expect it to stop widening (the levy no longer gates it) and the guard wording; the last
    block (its army removed, the claim stays and widens no further) is the new ending. `cap` is
    unused: remove it.
  - `an_army_marches_onto_a_rivals_land_it_outweighs_and_pillages_it`: Blue's and Gold's
    capitals must not be prizes. Stand each one's army on its capital with 12 people before
    planning (`stand(&mut w, blue, cap, 12.0)`).
  - `it_marches_on_a_capital_it_outweighs_and_pulls_back_from_one_it_does_not`: Red marches on
    Gold's capital instead. Stand Gold's army on its capital with 12 people at the start.
  - `it_stands_across_the_line_of_a_rival_army_abroad_...`: with Blue's army abroad its capital
    is empty and Red marches on it. `garrison(&mut w, blue, 12.0)`, and stand Gold's army on its
    capital with 12.
- `core/tests/ring.rs`, `the_walk_out_goes_hungry_past_the_ring`: `k` 9 and 10 are Blue's land,
  where no line runs. Loop `k` over `1..=8` and say why in the comment.
- `core/tests/save.rs`: the version in the three expected strings is 20.
- `cli/tests/save.rs`: `VERSION` is 20; set the five digests to what `sansfiat digest` prints
  (the failing asserts print them). Check `play` still gives the picks the comment names for
  seeds 1 to 3 and fix the comment if not.
- `cli/tests/play.rs`
  - `the_view_at_the_pick_lists_the_claim_and_its_new_legal_orders`: the view has no `claim:`
    line with `held` and `paper`, no guard raise and no paper on the map. Assert what it prints
    now: the `land: N tiles, your claim:` line, the `nobody stands on it` line, the claim order
    and the map's legend for land (run `sansfiat play` and `map` to read them).
  - `judge_prints_the_claim_lines_for_the_led_civ`: drop the guard raise from the script; assert
    the `land (game / control):` line and the motion line as `sansfiat judge` prints them.
  - `the_view_says_whether_a_gift_reaches_each_clan_in_reach`: no clan in reach trades elsewhere
    on small seed 6 at turn 25 any more. Find the first of seeds 1 to 24 that shows one, as the
    comment says, and use it; if none does within a minute of looking, raise an army in the
    script first (the led civ may have fallen) and look again.

## Done

- `cargo test --release` passes in `engine/`, and `engine/check.sh` passes whole, including
  `www/check.mjs` and `www/play/all.mjs`. `www/check.mjs` expects save version 20.
- Each Decided line of the piece has a test that names it. They exist in `claim.rs`, `fight.rs`
  and `civs.rs`; check the three-civ fight (#291) has one in `fight.rs` and add it if not: three
  civs' armies on one tile, each loses by the others' strength, one holds.
- `grep -rn "guard\|militia\|paper" engine/core/tests engine/cli/tests` shows no stale wording.
  An army's `guard` shares in `CampState` are not the guard: leave them.
- `relay/specs/336/progress.md` says the tests are done.
