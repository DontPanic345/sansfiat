# #104 build-3 (kiln): the scout as a unit, the judge's fight counts and the three stories

You build the piece's proofs on the clay (build-1) and the war rules (build-2). The scout
becomes a unit like the others, and foraging takes only what it takes. The judge counts
fighting against a control with none. `sansfiat story` plays the three stories on the retro
maps. Keep the clay's shape: you read its counters and events, and you don't change the fight
rule. Where it doesn't hold, make the smallest fix that works and record it as a decision. Read
`spec.md` items 19–22, the earlier steps' summaries, `docs/design/scouting.md` and
`docs/design/supply.md`.

## 1. The scout (`core/src/units/scout.rs`, `units/mod.rs`, `orders.rs`; spec item 19)

On main the scout starves on its first errand. On small seed 2, Red's scout routed from its
capital to Blue's (tile 301 to 39) at the pick goes hungry on the second turn and is gone by +30.
It forages only on a route and never on rival land, supplies only the turn's shortfall, and
can't be raised again.

- `ScoutState` gains `auto_forage` and `auto_supply` (default on), written to the flat fields
  `UnitFlat` already carries for a scout. `Switches`, `Forage`, `Pillage` and `Resupply`
  accept a scout (`unit_among`).
- It forages, or pillages on a rival's land, its own tile and the tiles within `SCOUT.reach`
  steps (add it to `data.rs`, starting at 1), richest first, paying each walk there and back
  (`walk_cost`) from its labour. It does so with or without a route, before its search.
- `strip` (`units/mod.rs`) takes only the food it takes: each stock drops in proportion to
  what was taken, and the tile rests `ceil(UNIT.fallow × taken / held)` turns. That holds for
  every unit (`supply.md`, Decided, M2 retro).
- Auto-supply refills its pack to full (`room`), as the camp's does.
- Once its trails are built, the route stays set until `ClearRoute`. While it is set, the known
  path's tiles are `used` each turn. It stops announcing the route after the first time
  (`heard`).
- `Raise { kind: scout, n }` makes a scout of `n` people at the capital, its pack filled from
  the store. It is refused if the store can't fill it, and emits `ScoutRaised`. Any number of
  scouts may exist. `Disband` accepts a scout.

**Proof.** On the seven retro maps, lead the first civ and route its scout at the pick to the
nearest rival capital, and separately to `Lead`'s target (a forest or hills tile seven steps
out). Over 50 turns, count each scout's hungry turns, people starved and whether it is alive,
on main (`git stash`, or a worktree of `main`) and now. Put the table in
`relay/specs/104/scout.md`. Done is no scout lost on its first errand on any map. If one still
is, say why and what `SCOUT.reach` would need. Build-5 may tune it.

## 2. The judge (`core/src/judge.rs`, `cli/src/judge.rs`, `cli/src/play.rs`; spec item 20)

- `Scores` gains a `fight` group, read from the civs' `Counters` at the end of the watch:
  people fallen, tiles taken by fighting and armies surrendered, per civ and in total.
- `sweep --judge` adds one control per map with `Config.fight` false, and prints the fight
  group with the control's beside it, in text and `--json`. Keep the run time in mind: one
  more run per map (main's report takes 26 s).
- `judge FILE` prints the fight group for the played game.
- `play --no-fight` starts a new game with fighting off.

## 3. The stories (`cli/src/story.rs`, new; `main.rs`'s dispatch and `USAGE`; spec items 21–22)

`sansfiat story skirmish|stack|surrender [--seed N --size NAME --civ K] [--no-fight]
[--turns N] --save FILE`. It runs to the pick, leads civ K, and each turn gives that civ's
orders through `World::act`, reading the world for its units, tiles and the nearest rival by
path. Then it ends the turn. The AI plays the rest. The save it writes is a normal save that
`judge FILE` scores and `play --game FILE` resumes. Each story is one function, written the way
`core/src/script.rs`'s `Lead` is: what it does on which turn, in its doc comment.

The economy every story lives in (`spec-review.md`): every store is 0 at the pick, the levy
brings about 3–4 food a turn, and a pack is 5 food a warrior. A camp at the capital resupplies
from the store every turn and the guard eats from it; the two together outeat the levy, and the
led civ's guard starves within ten turns of the pick. So every story sets gift 0, keeps its
guard by the AI's own rule (make `ai::land`'s guard raise and disband a function the story can
call for the led civ), and doesn't leave its camp at the capital beside the guard.

- **skirmish**: post the camp on its own land off the capital. Claim a corridor along the path
  to the nearest rival's capital, out until it overlaps the rival's held claim by a tile or two,
  and keep it. Optionally deploy the army's zone on the corridor's far end to thicken it.
  Default 80 turns.
- **stack**: bank the store first: send the starting camp to forage on its own land away from
  the capital, or disband it (your call, in the doc comment). With the camp away medium 9 Gold
  banks 46 by +35. Then raise every warrior the store fills at once into one army (gather the
  starting camp into it), and march it at the nearest rival's capital, both switches on. A
  march straight there starves on the way (medium 9 Gold's camp is gone eight turns out, two
  steps short of Blue's claim: the line is cut by length past about cost 28 and the pack is five
  turns), so stage it: march to the last tile of the path whose line isn't cut, wait there until
  the pack is full, then push. Default 80 turns.
- **surrender**: march an army into the rival's held land, past its guard, and halt it two or
  more steps inside. The rival's stand-in claim takes back the tiles behind it (build-1), which
  cuts its line. Default 60 turns.

The maps. Medium seed 9's Gold (10 steps from Blue, the closest pair) and small seed 2's Red
(13 steps from Blue) are the likeliest. Try each story on at least two retro maps, pick the one
where it plays clearest, and say why. A story that can't be made on any retro map is a finding:
say what stopped it.

Run each story with and without `--no-fight`, `judge` both, and write
`relay/specs/104/stories.md`. Per story it holds the map, the civ, the rival, the measures of
spec item 22 for the fight run and the control, and both civs' Standing at the end and the
difference. Under that, a few lines on what happened, from the events and notes.

Tests: `cli/tests/story.rs` runs each story for a few turns on small seed 2 and checks the save
replays to the same digest.

## Done

- `./check.sh` passes. `scout.md` and `stories.md` are written, and so is
  `relay/specs/104/build-3-judge.txt` (`sweep --judge`, with the no-fight control).
- Your summary gives the stories' headline numbers, the map each runs on, and the exact
  commands. Build-5 re-runs them as it tunes, and the review re-runs one.
- Bump `SAVE_VERSION` and the pins if the digest moves (foraging will move it). `cross-build.sh`
  passes.
