# #157 A rival that plays: spec

## What it delivers

The AI civs play for Standing with the player's levers and no others. Once a turn, for each civ the player
doesn't lead, a **planner** reads five needs in order and gives the orders that serve the first one unmet:

1. **Feed the armies.** A hungry army withdraws toward home; a cut army answers the cut by breaking out where
   it outweighs the cutters, else marching home; an army that can't be fed sheds people; a hungry guard sheds
   a person (today's rule).
2. **Hold the land.** A connected patch round its clans' homes, raised guard while some of it is paper and it
   can feed one, shrunk from the far end when the guard can't hold it; a rival army on its land is met by an
   army that outweighs it.
3. **Grow Standing.** The gift at its festival while a clan there leans to a rival, none otherwise; the patch
   widened toward unaligned homes while the guard holds; the store kept for the armies it means to raise.
4. **Hurt the nearest rival.** Armies raised while the store and the ring feed them; a rival's clans pillaged
   where it can reach their homes; a zone painted across a rival army's line and kept while it feeds.
5. **Take a capital it outweighs.** An army marches on a rival capital whose defence it outweighs and that its
   pack and line can reach, and withdraws when hungry (need 1).

The stand-in claim goes. Leaving is tuned until Standing is losable. The judge reads every game as three led
games, one per civ, beside an idle control for each, and says which are in doubt.

1. **The judge's led games** (build-1). From each watched run, a **led game** per civ: the game as it would
   go if the player led that civ and played it as the AI does. It is the same world, since the AI plays a
   led civ's orders the same way (decision 2); only its end differs. Its end is the first of: that civ's
   capital falls, at most one capital stands, the limit. For each: its rank at the pick and at its end, its
   **rank changes**, how it ended and when, its Standing at its end, and whether it is **in doubt**. Beside it
   the **idle control**: the same map with that civ led and given no orders (as `judge FILE`'s). The sweep runs
   three idle runs a map. `judge FILE` gains the same reading for the game, its AI control and its idle
   control.
2. **The fight's panic** (build-1): `wiped` reads a civ with no people on the tile as wiped (`probe.md`).
3. **The planner and the land** (build-2). `core/src/units/plan.rs`: the need order, and needs 2 and 3. The
   patch claim replaces `claim::ai_claim` everywhere, founding included. The gift and the keep are set through
   `World::set_gift` and `World::set_keep`, which `act` also uses, so the AI's count as orders as the player's do.
   `PLAN` in `data.rs` with starting numbers.
4. **The armies** (build-3). `core/src/units/reads.rs`: what the planner reads of war (whether it outweighs a
   tile, whether a march's pack and line reach it, a rival army's line, a cut). Needs 1, 4 and 5 in `plan.rs`.
   The planner raises its first army.
5. **The tuning** (build-4, the canvas): the needs' thresholds and order, `PLAN`'s numbers and leaving, until
   led games are in doubt and some end early, against the idle control and the stand-in's numbers.
6. **The design edits** (build-4, as decisions for the review to ledger): the stand-in lines in `war.md` and
   `founding.md` go.

## Shape: Easel and Canvas

The piece's `Shape:` line suggests Easel and Canvas and I take it, with the easel cut in three by area. The
probe (`probe.md`) settled what could be settled by running: a crude planner that only marches at the nearest
capital ends a game early and takes four capitals on the towns economy, so no economy or fight number needs
to move; and it throws its people away (1055 warriors starved, 85 camps lost) and barely moves the lead, so
the hard part is the judgement: when to march, when to pull back, what to feed, what to keep. Every lever's
*rule* is in the piece's Decided and settled below; the numbers and the balance between the needs only work
together, judged on the maps by the judge, the idle control and the player. That is a canvas, and under it
three settled easels:

- **build-1** (easel): the judge's led games, in-doubt and the idle control in the sweep and in `judge FILE`;
  the fight's panic. Measured on the stand-in: the base the canvas reads against.
- **build-2** (easel): the planner's frame, the need order, the land and the gift and keep, the stand-in gone.
- **build-3** (easel): the war reads and the three army needs, each lever pressed on a stage.
- **build-4** (canvas, twinned): the tuning until the bounds below hold.

Why not one easel for the planner: land and war are two areas with their own reads, and the war half is
judgement-heavy enough that a session writing both and every stage would have no room to check either.
Why not Clay and Kiln: the shape at the centre (a need list, each need a check and orders) is small and
settled here; what's unsettled sits on top.

## The question and how it is measured

**The question:** doubt (`docs/process.md`, Scores). Is a led game ever in doubt?

**A led game** is one civ's view of a watched run (item 1). **A rank change** is the led civ's rank in Standing
changing, with the lead's margin: the led civ counts as above another civ until that civ is ahead of it by
more than `JUDGE.lead_margin`, and the other way round, so two civs neck and neck don't swap on noise. Its
rank is one more than the civs above it. **A led game is in doubt** when the led civ's rank changes at least
once after the pick, or its game ends before the limit. **A map is in doubt** when at least two of its three
led games are (the designer settles "in doubt on one map"). **Beats idle** is the led civ's Standing at its
game's end less the idle control's at the same turn.

**The controls.** The idle control (each civ led and given no orders, the same map to the same pick) for
beats-idle and, beside it, the idle civ's rank changes. The **stand-in** for doubt, ends, leaving and
decisions: the same sweep on the base, which build-1 runs before any planner and writes into `judge.md`
(the probe measured its watched numbers: `probe.md`). A move counts when it is past `sweep --judge --floor`'s
floor.

| Score | Measure | Control | Base (`probe.md`, build-1) |
|---|---|---|---|
| Doubt | **Led games in doubt**: at least 11 of the 21, and maps in doubt at least 5 of 7. | Stand-in | build-1 measures |
| Doubt | **Lead changes** (watched): more than the stand-in's total, on at least 5 of 7 maps. The spread at +100 and +200 and the swings, reported. | Stand-in | 4, on 4 maps; swings 4 |
| Doubt | **Some games end before the limit**: at least 3 of the 21 led games end before the limit, on at least 2 maps, and at most 14 (the limit still decides some). At most one ends before +50. Watched games ended early and capitals fallen, reported. | Stand-in | 0 of 7 watched; 0 fallen |
| Standing | **Beats idle**: the mean over the 21 led games is above the floor, and it is positive in at least 15. | Idle control | build-1 measures the stand-in's |
| Standing | **Losable**: clans left at least 1 on at least 4 of 7 maps, and more than the stand-in's in total. Aligned Standing at +200 reported per civ, not a goal. | Stand-in | 2, on 2 maps; aligned 0.328 |
| Standing | **The player loses some** (the play step, three seeds): on at least one seed the player's civ is not first in Standing at +100, or its capital falls; and `judge FILE` has it beating idle by more than the floor on at least two. | Idle control | none: #152's player led throughout |
| Decisions | Every lever the AI has is pulled on the retro maps: claim, raise (army and guard), disband, move, zone, switches, pillage, setGift, setKeep, each counted per 100 turns. Levers repeated more than the stand-in's. | Stand-in | claim 9.0, raise 3.5, disband 0.1; levers repeated 1.71 |
| Motion | People, land and clans changed no lower than the stand-in's, less the floor. | Stand-in | probe: 219 joined; see judge.md |
| Health | **No army starves by mistake**: canvas.md names why armies starve and surrender, and none does because the planner misjudges (marches past what its pack and line carry, stays hungry when it could withdraw, raises what the store can't feed). Warriors starved and camps lost reported, not bound. | Stand-in | probe planner: 1055, 85 |
| Health | **The planner's orders pass**: no `w.order` the planner gives is refused, over the seven maps (it checks with `check_order` first). | | |
| Health | Pre-games by centres 7 of 7. Panics 0. Digest mismatches 0. Every game ends. `sansfiat bench --size large 8 --after 50` at most 1.25 times the base's, same session, two runs each. | Base | 7, 0, 0 |
| | `engine/check.sh` green at every step. | | |

## What I settled

- **The need order and "first unmet"** are the piece's. A need is **met** when it has nothing to do that the
  civ can afford: a need with no affordable order passes to the next, so a civ with nothing to fix at home
  goes on to hurt its rival. Orders already standing (a march, a zone, a claim) carry on; the planner gives an
  order only to change something. Decision 3.
- **The patch.** Its seeds are the capital and the homes of the civ's own clans; it is the tiles of the
  shortest land paths from the capital to each seed within `PLAN.reach` steps, plus the land tiles within
  `PLAN.ring` of each seed, kept connected, never a rival's capital, never a tile a rival army holds. It
  widens by one tile a turn (the nearest unaligned home's ring first) while no tile of it is paper and the
  guard can be raised, and drops its farthest paper tile when the guard can't be fed. Decision 4.
- **The gift**: `PLAN.gift` while an unaligned clan at its festival has a leaning to a rival, else 0.
  Decision 5. **The keep**: the store's cap while it means to raise (need 4 or 5 has a target), else the
  default. Decision 6.
- **How it decides it outweighs a capital** (the designer settles). Its army's attack (each kind's people
  times its attack, halved if hungry) against the capital's defence: the militia (the people of the rival's
  clans living on the capital tile times `FIGHT.militia`), the whole guard entrenched to the cap (it is drawn
  back when an army stands there), and the rival's armies on the capital or within `PLAN.near` steps, at
  their defence and entrenchment. It outweighs at `PLAN.outweigh` times the defence. And the march reaches:
  the capital is in its ring, or the turns of the march (`line::turns`) leave `PLAN.spare` turns of pack.
  The same read serves any tile (a cut, a rival army on its land). Decision 7.
- **What "in doubt" measures on one map** (the designer settles): above.
- **The led game is read from the watched run**, not played again. Decision 2.
- **Leaving's lever is the canvas's.** The margin alone moved nothing under the stand-in (`probe.md`): a
  rival's leaning never reaches a civ's clan there. So build-2 adds `LEANING.leave_margin` at 1 (today's rule,
  digest-equal) and the canvas moves it, or the run, once the planner plays.
- **Where the code goes**: `units/plan.rs` (the needs), `units/reads.rs` (the war reads), `core/src/judge_led.rs`
  (the led games), `cli/src/sweep_led.rs` (the idle runs and the totals). `ai.rs` keeps the `ai` step and calls
  the planner. `sweep_judge.rs` (376 lines) grows only by the call.

## Unsettled, and which step holds it

All in build-4, the canvas:

- **The thresholds**: every `PLAN` number (`outweigh`, `spare`, `near`, `reach`, `ring`, `gift`, the packs it
  banks before raising, the ring share it raises within, when a hungry army turns home, when an idle army
  disbands). Proof: the doubt rows, beats-idle and the no-starving-by-mistake row.
- **The order's fine print**: whether a need's affordable test is right, and whether "hurt" or "take" must
  come before "grow" on some maps. The order itself is the piece's Decided and stays.
- **Leaving**: `LEANING.leave_margin` or `leave_run` (or `leave_at`) moves until clans leave on at least 4 of
  7 maps. Proof: the losable row.
- **The player**: whether the AI can make a player lose. Proof: the play step's three seeds, and the canvas's
  own `sansfiat play` game on one retro map, written into canvas.md.

## What the review runs

- `engine/check.sh`.
- `sansfiat sweep --judge --json --floor` on the branch and on main (a worktree), then
  `sansfiat judge --compare main.json branch.json`. Every bound in the table goes in the PR, pass or miss, with
  the old and new number.
- `cargo test --release -p engine --test plan -- --nocapture`: every lever on its stage.
- `sansfiat bench --size large 8 --after 50` on the branch and on main, two runs each, same session.
- The play step's three games, and `sansfiat judge FILE` on each: the led rank, rank changes, in doubt and
  beats-idle.
- One game of its own, `sansfiat play --game g.json` on small seed 2, leading the civ the AI leads first at the
  pick: 60 turns doing nothing but the gift and the guard, to see whether a rival comes for it.

No new button or order: the planner gives the player's orders. The gift and keep the AI sets go through the
same setters as the player's, so `buttons.mjs` still presses the player's.

## Decisions I made that the piece left open

1. **Design says "its orders are the player's orders; it has no others."** The gift and the keep are the
   player's `Action`s, not `Order`s. The planner sets them through the same two setters `act` uses
   (`World::set_gift`, `World::set_keep`), counted as the player's are. Making them `Order`s would change the
   wire form of the player's actions for no gain.
2. **A led game is the watched run read from one civ.** The AI gives a led civ's orders the same way it gives
   any civ's, and the world runs on after the end with every rule, so a game led by the AI is the watched game
   with another end. Reading it costs no runs; only the idle control does.
3. **A need is met when it has nothing affordable to do**, so the planner moves down the list every turn it
   can't act on a need. Read literally, "the first unmet need" would let one unaffordable want (a clan out of
   reach) stop every other need forever.
4. **The patch** is paths from the capital to its clans' homes plus a ring round each, widened a tile a turn
   while held. The stand-in's rival homes go: a civ's claim is round its own clans, and it reaches unaligned
   homes only by widening.
5. **Design says "gifts at festivals where a rival also gifts."** A civ gives only at its own festival (#102),
   so the planner gives there while a clan at it leans to a rival, and gives nothing otherwise, banking the
   food for armies.
6. **The keep is the store's cap while it means to raise an army.** #152's stories found that by default a
   store posts past a pack and never banks enough to raise (`relay/specs/152/stories.md`).
7. **Outweighing is attack against defence at a margin, with reach.** Militia, the drawn-back guard and
   nearby armies make a capital's defence, as `war.md` says it is the hardest tile to take.
8. **Warriors march; archers stay.** The planner raises archers only into an army that stays on its land to
   meet a rival there, where they defend at 1.4, and only where it holds forest; armies that march are
   warriors.
9. **The AI's scout keeps its own plan** (`scout.rs`); the planner doesn't move it.
10. **`units::post` stays** for the text view's hint and the camp sweep; the planner doesn't use it.
11. **The piece carries a `Whole:` line, and this session was asked for a spec**: the steps below follow the
    spec prompt. If the crew runs the piece whole, the spec and briefs are its plan.
