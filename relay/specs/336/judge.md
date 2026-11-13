# #336 Land without a guard: the review's judge

`sansfiat sweep --judge --json --floor` on the branch (merged with main at f840e80; engine code as
of e462415) and on main (f840e80), then `sansfiat judge --compare main-floor.json branch-floor.json`
(`review-compare.txt`). The branch's sweep and main's without floors are `branch-sweep.json` and
`main-sweep.json`; the numbers match.

## Verdict: better

The question is doubt: with every civ on the AI, do games end before the limit and do capitals fall?

| Score | Main | Branch | Floor | |
|---|---|---|---|---|
| all-AI games ending early (of 7) | 0 | 0 | 0 | no move |
| capitals fallen, all-AI | 0 | 2 (small 3, small 6) | 0 | past |
| tiles taken by an army (`motion.claim_taken`) | 0 | 36 (small 2: 24, 9: 9, 3: 2, 6: 1) | 20 | past, on three maps |
| doubt: spread of Standing at 200 | 0.098 | 0.177 | 0.008 | past, on 5 of 7 maps |
| doubt: lead changes | 8 | 13 | 4 | past; per map +3, +5, -3, -1 |
| doubt: maps with a lead change | 5 | 6 | 0 | past |
| led games ending early (of 21) | 0 | 3 | 0 | the led civ's capital fell |
| Standing: top civ at 200 | 0.163 | 0.234 | 0.004 | past |
| decisions: AI orders per 100 | 60.4 | 77.4 | 13.4 | past (moves 30 to 52) |
| motion: people changing hands | 1440 | 2355 | 112 | past |
| led over idle, mean Standing | 0.0494 | 0.0429 | 0.0039 | still ahead, down past the floor |
| led ahead of idle (of 21) | 21 | 18 | 0 | down |
| health: panics, digest mismatches | 0, 0 | 0, 0 | | in band |
| health: starving share | 0.0299 | 0.0251 | 0.0024 | in band |

Games still don't end early with every civ on the AI: that takes two of three capitals, and the
medium and large maps' capitals are too far apart for an army to reach one (no tile is taken on
them). The mechanism is the one the piece claims and it fires: land is taken by armies standing
on it (36 tiles against 0), two capitals fall, and the spread opens on small maps. The lead
changes move unevenly by map (small 2 and 3 up, small 6 and 9 down), so doubt rests mostly on the
spread. The small 9 claim-taken figure (9) is inside its floor (25).

## What the owner should read

- A civ starts with no army, so a capital no army of its civ stands on falls to the first rival
  army to reach it. Three led civs (the planner's) and two idle led civs lose their capital. A
  player's first order has to be a raise. That is the rule as `war.md` words it, not a bug.
- A led player beats an idle civ by less than on main (0.043 against 0.049, past the floor) and in
  18 of 21 games against 21: the idle civ's land costs it nothing until an army comes.
- `PLAN.hold` 15 is a number the planner's widening hangs on (the AI claims `armed x 15` tiles):
  it sets how wide the AI claims and so how much land changes hands. `scores.md` tried 6 and 30.
  Treat it as a rule the owner may overrule.
- The play found no claim-order preview; the claim order's effect was in `why` only. The review
  put its last line in the view's land block.
