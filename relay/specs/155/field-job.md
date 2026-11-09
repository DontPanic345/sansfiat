# #155 build-1: the field job, measured

Built in `engine/core/src/fields.rs` on the branch (`relay/155`), base `85868ed`. Runs are the release
`sansfiat` binary: `sweep --judge --json` (fields off), `sweep --judge --json --floor` (fields on, the defaults),
both on the branch and on a detached worktree of the base.

## Fields off is main

`Config::default().fields` set to false locally (not committed), `sweep --judge --json` on the branch, against the
base's. Every map's `standing`, `motion`, `economy` and `land` blocks are equal, and so are the totals of each.

```
maps 7 7 mismatches []
total standing equal / motion equal / economy equal / land equal
```

## Fields on: the floor compare

`sweep --judge --json --floor` on the branch (defaults) and on the base, then `judge --compare base.json branch.json`.
Changes past the floor, from the totals:

| Metric | Base | Fields | Change | Floor |
|---|---|---|---|---|
| Standing, aligned at +200 | 0.2525 | 0.3221 | +0.0696 * | 0.0053 |
| Standing, top at +200 | 0.1069 | 0.1594 | +0.0525 * | 0.0018 |
| Spread at +200 | 0.0497 | 0.1012 | +0.0515 * | 0.0071 |
| Lead changes | 9 | 4 | -5 | 6 (not past) |
| Clans joined | 125 | 195 | +70 * | 16 |
| People changed hands | 629 | 980 | +350 * | 49 |
| Land changed | 1022 | 1195 | +173 * | 118 |
| Store mean | 25.9 | 29.4 | +3.5 * | 1.3 |
| Levy per turn | 9.2 | 13.8 | +4.6 * | 0.5 |
| Turns at a pack (total) | 426 | 508 | +82 * | 42 |
| AI orders per 100 turns | 6.9 | 8.4 | +1.5 * | 1.2 |

The full compare is in `field-job-compare.txt`.
Aligned Standing rises past the floor, as the spec expects. The probe's y8 gave 0.253 to 0.312; the built job gives
0.322 (the built rules differ from the probe's: field grain counts at the budget's scale, `Tile.field` is the days
worked, and the farmer's home follows the best field tile by days, not by ground times yield).

## Per civ: Standing at +200, fields on against fields off

The fields-off run is `sweep --judge --json` on the branch, the control. This is the per-civ Standing gain over
21 civs: mean +0.023, 14 of 21 gaining. The farming subset (a civ whose clans put at least a tenth of their people's
days to fields at +200) needs each clan's farming share, which build-2's judge group reports; this run does not
split the civs by it.

| Map | Civ | Off | On | Change |
|---|---|---|---|---|
| small 2 | 424 | 0.2162 | 0.2725 | +0.0563 |
| small 2 | 425 | 0.2141 | 0.3152 | +0.1011 |
| small 2 | 426 | 0.0649 | 0.0509 | -0.0140 |
| small 3 | 247 | 0.0811 | 0.0721 | -0.0090 |
| small 3 | 248 | 0.1149 | 0.1523 | +0.0374 |
| small 3 | 249 | 0.0805 | 0.0698 | -0.0107 |
| small 6 | 245 | 0.1245 | 0.1734 | +0.0489 |
| small 6 | 246 | 0.0931 | 0.1117 | +0.0186 |
| small 6 | 247 | 0.0916 | 0.0824 | -0.0092 |
| small 9 | 309 | 0.0657 | 0.1230 | +0.0574 |
| small 9 | 310 | 0.1111 | 0.2099 | +0.0988 |
| small 9 | 311 | 0.0965 | 0.1079 | +0.0113 |
| medium 8 | 816 | 0.0826 | 0.0950 | +0.0124 |
| medium 8 | 817 | 0.0495 | 0.0779 | +0.0284 |
| medium 8 | 818 | 0.0568 | 0.0492 | -0.0076 |
| medium 9 | 845 | 0.0586 | 0.1282 | +0.0696 |
| medium 9 | 846 | 0.0483 | 0.0337 | -0.0146 |
| medium 9 | 847 | 0.0229 | 0.0160 | -0.0069 |
| large 8 | 1718 | 0.0288 | 0.0401 | +0.0112 |
| large 8 | 1719 | 0.0256 | 0.0318 | +0.0062 |
| large 8 | 1720 | 0.0404 | 0.0421 | +0.0017 |

## Clans with fields at +200

Scratch counts (not committed), the pick then 200 turns on the AI, on each retro map:

| Map | Clans | With fields | Farmers (share at least 0.1) | Farming share of people |
|---|---|---|---|---|
| small 2 | 157 | 50 | 50 | 0.078 |
| small 3 | 96 | 15 | 15 | 0.047 |
| small 6 | 123 | 36 | 36 | 0.088 |
| small 9 | 136 | 57 | 57 | 0.109 |
| medium 8 | 308 | 71 | 71 | 0.067 |
| medium 9 | 198 | 48 | 47 | 0.084 |
| large 8 | 713 | 185 | 185 | 0.073 |

Every map has clans with fields at +200.

## Not measured here

- The store bound (a pack on half the turns at the farming civ) and the starving bound (`clan-turns` starving against
  the no-fields control, read past the floor): these need the no-fields control as a sweep run, which is build-2's.
  The compare shows turns at a pack rising from 426 to 508 in total and the store mean from 25.9 to 29.4.
- The motion bound for settled farmers: needs the farmer's home counts from build-2's judge group.

## Scout errand: the test runs on the control

`every_scouts_first_errand_is_survived` (`engine/core/tests/scout_errand.rs`) bounds the led civ's scout on its first
errand: at most 4 of 28 lost by 50 turns with no refill, all on medium 9. With fields on, 6 are lost: small 3 (route to
the rival, idle), small 6 (move to Lead's target) and medium 9 (route to the rival, move to Lead's target, idle). The
owner's reply to the blocker was "fix or remove the test". The test now runs its errands with `Config.fields` false,
the control, where it passes as on main: the bound is the scout's rules on the ground they were measured on.

Why that and not a new bound with fields on: the bound is a snapshot of one float path. The test's own comment says
main with `CLAN.work` nudged by one part in 10^12 loses 5 of the 28 on other maps. A probe of small 3's idle scout
shows the paths match until turn 13 past the pick, when fields on sends the scout to other ground (the clans' takes
around it differ), and from there it starves at turn 47 with the civ's store at 49 food. Fields change every clan's
take from the pick, so a bound with fields on would move with every tuning of `FIELD` (build-4's).

The ignored report beside it, `the_ais_scouts_live_to_200` (every civ on the AI to +200, asserting no AI scout lost),
already fails on the base: with fields off 2 of 21 scouts are gone and 9 people starved; with fields on, 4 gone and 10
starved. By map, fields off / on, scouts gone: small 3 1/1, small 9 0/1, medium 9 1/2, the rest 0/0.

## Gift-reach test: seed changed

`the_view_says_whether_a_gift_reaches_each_clan_in_reach` (`engine/cli/tests/play.rs`) names the first of seeds 1 to 24
that shows a clan in reach trading elsewhere at turn 25. With fields, seed 19 no longer shows it; the first that does
is seed 6 (seeds 6, 8 and 24 match; 1 to 5 and 7 do not). The test now plays seed 6. The comment's rule still holds.
