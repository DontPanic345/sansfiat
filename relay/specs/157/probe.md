# #157 spec: what the probes measured

Run in the spec step on `main` (06ee26b, Towns merged), release build, `sansfiat sweep --judge --json`, about
70 s a sweep on this runner. Every number is the sweep's total over the seven retro maps unless a map is named.

## The base: the stand-in AI

The AI claims the stand-in patch, raises its guard and nothing else. No army is raised, so nothing fights.

| | base |
|---|---|
| lead changes (watched) | 4, on 4 of 7 maps |
| swings | 4 |
| games ended before the limit | 0 of 7; capitals fallen 0 |
| fallen, tiles taken, surrendered | 0, 0, 0 |
| clans left | 2 (small 2 and small 3, one each) |
| clans joined | 219 |
| aligned Standing at +200 | 0.328 |
| AI orders per 100 turns | 12.7 (claim 9.0, raise 3.5, disband 0.1) |
| store mean | 7.5 (small 9) to 41.5 (small 6) |

## A crude planner: every AI civ plays the conquest story

`probe.diff` (about 30 lines in `units/ai.rs`, behind `PROBE=<packs>`): every AI civ keeps its whole store
(keep at `CIV.store_cap`), raises every warrior its store fills once it fills four packs, turns both
switches on and marches straight at the nearest standing rival's capital. No withdrawal, no outweighing,
no gift or claim change. It is the strongest plan #152's stories found, given to all three civs at once.

| | base | probe (4 packs) |
|---|---|---|
| maps that finished | 7 | 5 (small 2 and small 6 panicked, below) |
| games ended before the limit | 0 of 7 | 1 of 5 (small 3, conquest at +165) |
| capitals fallen | 0 | 4 (small 3: 2; small 9: 1; medium 8: 1) |
| lead changes | 4 on 4 maps | 6 on 3 maps (small 3: 4) |
| fallen people / tiles taken / surrendered | 0 / 0 / 0 | 285 / 162 / 39 |
| warriors starved / camps lost | 0 / 0 | 1055 / 85 |
| aligned Standing at +200 | 0.328 | 0.146 |
| clans left | 2 | 0 |

What it says:

- **The economy can feed an army that takes a capital.** On the towns economy a store of four packs fills
  on every map, and three civs lost their capital to an army that marched straight in. No fight, supply or
  economy number needs to move for a game to end early.
- **A planner that only marches throws its people away.** 1055 warriors starved and 85 camps were lost:
  armies marched past their ring, starved and surrendered, and the next one went the same way. The
  Decided's "withdraws when hungry" and "marches on a capital whose militia it outweighs" are what keep
  the people. Aligned Standing halved, partly because a fallen civ's clans go unaligned and partly from
  the armies thrown away.
- **Doubt follows fighting but not much.** Lead changes moved only on small 3, where two capitals fell.
- **Clans don't leave.** Fighting took land (162 tiles) but no clan left: taken tiles were won back or the
  clans on them leaned home harder than to the taker.

## A three-civ fight panics

On small 2 and small 6 the probe panicked at `core/src/units/fight.rs:240` (`lost[&c]`: "no entry found
for key"). `wiped` indexes `lost`, which has an entry only for the civs with people on the tile above
`FIGHT.floor`, and it is called on `holder` (line 312, `holder.is_some_and(|h| … wiped(h) …)`). When the
tile's holder (by claim, `holder_of`) has no people there and two other civs fight over it, `lost[&h]`
panics. A civ with no people on the tile has none left: `wiped` reads a missing civ as wiped. Build-1 fixes
it with a stage that repeats it.

## Leaving's margin, under the stand-in

`margin.diff` (behind `MARGIN`): a civ's clan is drawn when its strongest rival leaning is at least
`leave_at` and more than `MARGIN` times its leaning to its own civ (today 1).

| margin | clans left |
|---|---|
| 1 (today) | 2 |
| 0.5 | 3 |
| 0.25 | 2 |

The margin barely moves leaving: under the stand-in a civ's clan almost never has a rival leaning of 0.1.
Its own land gives it 0.1 or more a turn and nothing of a rival's reaches it, since a rival gives only at
its own festival, to clans not its own, and a rival's land doesn't reach its home. So leaving is moved by
what the planner does (taking land under a clan's home, pillaging, drawing a clan to its festival) more
than by the margin, and it can only be tuned once the planner plays. It is the canvas's (`spec.md`,
Unsettled).

With the probe planner and margin 0.5 together the sweep is identical to the probe's at margin 1: still no
clan left on the five maps that finished.
