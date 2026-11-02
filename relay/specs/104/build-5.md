# #104 build-5 (glaze): tune the fight until the story plays out on real maps

You tune the numbers that only work together, until the Red and Blue story of `war.md` comes
out of the three scripts on the retro maps. Everything is built: the fight rule
(`core/src/units/fight.rs`), militia, the capital, siege, surrender and the fallen capital
(build-2), the scout and `sansfiat story` with its controls (build-3), and the page (build-4).
Read `docs/design/war.md` first (its story is the target), then `spec.md` (items 8, 12–16, 22
and "Left unsettled"), then build-3's `stories.md` and every earlier summary.

## What you tune

In `data.rs`:
- `FIGHT.rate`, `entrench`, `entrench_cap`, `retreat`, `hungry`, `militia`, `surrender` and
  `floor` (`surrender` above `UNIT.hungry_turns` lets hunger take a warrior a turn first, and
  past the turns that takes to empty the story's army it never fires);
- the kinds' attack and defence;
- `SCOUT.reach`.

Each keeps one `note` saying why it is what it is, with the run that showed it.

You may change the kiln: a rule's form (the loss curve, how retreat picks its tile, when the
march stops), a story's script, a threshold. Each change is a decision with the run that
showed it. You don't change the economy's numbers (`UNIT.pack`, `UNIT.food`, the levy, the
store's cap, `GUARD`, `CLAIM`) to make a story come out. If the economy can't feed a stack worth
the name on any retro map, say so with the numbers: it is part of the piece's answer.

## What proves it

Run each story on its map with and without `--no-fight` (the commands are in build-3's
summary), and `judge` each. The target, from `war.md`'s story:

- **skirmish**: both sides lose people, a few a turn in all, decisive for neither. Fallen per
  contested tile-turn is small: you settle the band and say why. At most a third of the
  contested tiles change hands over the run.
- **stack**: it takes the thinly held tiles on its way, a tile a turn, and is stopped at the
  guarded capital: by the militia and the drawn-back guard, by its own hunger, or both. It
  retreats or surrenders, and the capital doesn't fall.
- **a capital can fall**: on one map, the stack against a capital whose guard is gone or thin.
  No order disbands a rival's guard, so pick the map, or the turn, where a rival's guard has
  starved. Small seed 2's Blue is down to a guard of 1 by +120. Show a fall, or say what stops
  one.
- **surrender**: cut off, the army goes hungry and surrenders to the rival within about ten
  turns of its pack running out. Its people join the rival's clan, and both civs' Standing move
  against the control.
- **Standing**: in at least two stories, some civ's Standing at the end differs from the
  control's by more than `JUDGE.lead_margin`. Report each, fight against control.
- **Health**: `sweep --judge` beside `main-judge.txt` and `build-3-judge.txt`: no panic, and the
  AI's Standing, armies and guards at +200 close to build-3's. The AI doesn't fight, so a big
  move means the fight reached the AI maps where it shouldn't. The no-fight control matches the
  fight run wherever nothing fought.
- **The page**: play one fight through `play.mjs` (build-4's `army.mjs` does) with your final
  numbers and read the panel's "Why". Does it say why the fight went as it did?

Write `relay/specs/104/glaze.md`: each number from and to, with the run that moved it; the
stories' final table (fight and control, both civs' Standing); `sweep --judge`'s totals beside
main's; and a few lines of each story as it now plays, from its events. Save the final judge
as `relay/specs/104/glaze-judge.txt`, and update `stories.md` with the final runs.

## Done

`./check.sh` passes, `glaze.md` is written, and `SAVE_VERSION` and the pins are bumped if the
digest moved. Your summary answers the piece's question with the numbers: does the story come
out, and does fighting move Standing? Where it doesn't, it says what stops it.
