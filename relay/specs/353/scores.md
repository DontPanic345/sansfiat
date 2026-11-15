# 353 Signposts and settlements: the scores

`sansfiat sweep --judge` on the seven retro maps, every civ on the AI, 200 turns past the pick.
`branch-sweep.json` is this branch, `main-sweep.json` is main after J (d66cf37), `compare.txt` is
`judge --compare` of the two. The sweep's own control is the same map with `Config.signposts` off
(the planner posts none): its Standing equals main's on every civ (last two columns), so the rules
with signposts off are main's rules.

## The question: settlements a civ has at +100 and +200, and its Standing against the same civ with no signposts

| map | civ | signposts posted | settlements +100 | +200 | clans living there +200 | their stores | Standing +100 | no signposts | Standing +200 | no signposts | main +200 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| small 2 | 424 | 2 | 1 | 2 | 2 | 54 | 29.7% | 31.6% | 29.1% | 34.5% | 34.5% |
| small 2 | 425 | 0 | 0 | 0 | 0 | 0 | 38.0% | 36.8% | 39.2% | 35.6% | 35.6% |
| small 2 | 426 | 1 | 0 | 0 | 0 | 0 | 9.7% | 7.7% | 9.6% | 7.7% | 7.7% |
| small 3 | 247 | 0 | 0 | 0 | 0 | 0 | 23.5% | 23.5% | 22.5% | 22.5% | 22.5% |
| small 3 | 248 | 0 | 0 | 0 | 0 | 0 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| small 3 | 249 | 0 | 0 | 0 | 0 | 0 | 15.3% | 15.3% | 15.5% | 15.5% | 15.5% |
| small 6 | 245 | 0 | 0 | 0 | 0 | 0 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| small 6 | 246 | 2 | 2 | 2 | 3 | 54 | 23.1% | 30.9% | 23.3% | 30.8% | 30.8% |
| small 6 | 247 | 0 | 0 | 0 | 0 | 0 | 22.0% | 21.8% | 22.1% | 22.0% | 22.0% |
| small 9 | 309 | 0 | 0 | 0 | 0 | 0 | 17.4% | 17.4% | 17.5% | 17.5% | 17.5% |
| small 9 | 310 | 0 | 0 | 0 | 0 | 0 | 34.7% | 34.7% | 37.2% | 37.2% | 37.2% |
| small 9 | 311 | 0 | 0 | 0 | 0 | 0 | 16.0% | 16.0% | 15.2% | 15.2% | 15.2% |
| medium 8 | 816 | 0 | 0 | 0 | 0 | 0 | 12.1% | 12.1% | 12.2% | 12.3% | 12.3% |
| medium 8 | 817 | 1 | 0 | 1 | 2 | 27 | 13.3% | 13.3% | 13.3% | 12.8% | 12.8% |
| medium 8 | 818 | 0 | 0 | 0 | 0 | 0 | 7.9% | 7.9% | 7.6% | 7.3% | 7.3% |
| medium 9 | 845 | 0 | 0 | 0 | 0 | 0 | 17.9% | 18.1% | 18.1% | 18.7% | 18.7% |
| medium 9 | 846 | 1 | 1 | 1 | 1 | 8 | 4.3% | 3.9% | 4.0% | 4.1% | 4.1% |
| medium 9 | 847 | 0 | 0 | 0 | 0 | 0 | 3.7% | 3.7% | 3.4% | 3.4% | 3.4% |
| large 8 | 1718 | 0 | 0 | 0 | 0 | 0 | 7.9% | 7.9% | 7.8% | 8.1% | 8.1% |
| large 8 | 1719 | 2 | 1 | 1 | 0 | 21 | 8.6% | 8.5% | 8.9% | 8.7% | 8.7% |
| large 8 | 1720 | 1 | 0 | 0 | 0 | 0 | 6.3% | 6.4% | 6.0% | 7.1% | 7.1% |

Totals: 10 signposts posted, 7 settlements founded, 0 taken; 5 stand at +100 and
7 at +200, on 5 of 7 maps and for 5 of 21 civs.

Standing at +200, mean over civs: all civs 14.9% (no signposts 15.3%); the
5 civs with a settlement 15.7% (no signposts 18.2%).
7 civs are up on their control, 7 down, 7 the same (the maps where nobody posted are identical
to main). The worlds' people are 0.990 of the control's.

## Reading

- The rules work on the real maps: daughters walk to signposts and found settlements (first at +8
  on small 2), the settlements' markets hold clans (13 to 14 of Blue's 29 clans go to its two on
  small 6), and their stores fill to the cap.
- A settlement does not raise its civ's Standing as built. Of the 5 civs that have one at +200, 2
  are up a little (medium 8 Blue +0.5, large 8 Blue +0.2) and 3 are down (small 2 Red -5.4, small 6
  Blue -7.5, medium 9 Blue -0.1 points).
- Why, on small 6 (Blue, probed turn by turn): its people are level with the control until +50;
  then the control's army grows from 3.4 to 4.0 people, the planner widens its claim from 51 to 60
  tiles and eleven clans join by land; with settlements the army stays at 3.4 and the claim at 51.
  Half of Blue's clans pay their levy at the settlements, into stores nothing draws on, so the
  capital's levy feeds one warrior fewer. The loss is the claim that was not widened, not people
  starving at the settlement. The gap on one map is one warrior's threshold, so part of it is the
  chaos of a different game; the sweep's noise floor (`--floor`) was not run.
- The planner posts little: 10 signposts over 21 civs. It posts only on a tile nobody holds that
  alone feeds a daughter (`SETTLE.fed` 0.9), within 8 steps of a seat; on small 3 and small 9 no
  such tile is free.
- Not tuned: `SETTLE.draw` 0.8 and `SETTLE.walk` 7 days are the first numbers tried. The Glaze the
  piece asks for (how far a signpost draws, on the maps) is one run deep.

## What would change the answer

The store is the lever: a settlement takes levy from the capital and gives nothing back. Either the
armies' lines draw on the nearest store, or a settlement posts its spare food at its own market as
the capital does. Both are the owner's call (see the idea in the result).
