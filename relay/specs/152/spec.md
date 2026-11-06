# #152 A game that ends: spec

## What it delivers

A game ends, and says how it went.

1. **The start.** A civ starts with its scout and nothing else: `UNIT.start` and `GUARD.start`
   are 0, and the store keeps the boon year's food that the army's pack took. The AI raises its
   guard as now and no army (until R). The stories, `Lead` and the camp sweep's lever raise the
   army they script.
2. **The end.** `Phase::Over(Over { winner, why, turn })`. A game ends when one capital stands,
   when the led civ's capital falls, or at the turn limit; the winner is the last capital, else
   the highest Standing. After it the player's orders are refused and the world runs on.
3. **Standing over time.** `World.standing`: every civ's Standing at the pick and after each
   turn since, rebuilt by a replay.
4. **The judge reads the end** on every run: games ended before the limit, turns to the end,
   capitals fallen, the led civ's rank at the end, beside the no-fight control in the sweep and
   beside the AI and idle controls in `judge FILE`.
5. **The page.** The end screen (won, lost or watched; how; Standing over the game as a line
   per civ; Play on, New game) comes up when the game ends; the pick's cards and the top bar
   show the limit; "The game goes on" goes.
6. **The limit per size and the stories**, set and measured on the retro maps by the canvas.
7. **`vision.md` Winning**: the first way to win is written into the prose (the piece decides
   this edit).

## The question and how it is measured

**The question:** doubt (`docs/process.md`, Scores): some games end before the limit on the
stories' maps, and every game ends; and the judge reads the end on every later piece.

| Score | Measure | Control |
|---|---|---|
| Doubt | **Every game ends.** `sweep --judge` on the seven retro maps: each AI run and each no-fight run reaches `Over`, by the limit or sooner; the report's end group says how and when. | None needed: a run that doesn't end is a failure. |
| Doubt | **Some games end before the limit on the stories' maps.** The stories (the canvas's, `story NAME` on each retro map and civ, played to the end) and `judge FILE` on each save: games ended before the limit, capitals fallen, turns to the end, the led civ's rank at the end. | The same story with `--no-fight`: no capital can fall, so every game ends at the limit. |
| Doubt | **The limit ends a game in time to matter**: on the retro maps' AI runs, the leader at the limit is the leader the game settles on (the canvas defines settled and measures it; see build-5). | The same runs played on long past the limit. |
| Standing | The led civ's rank at the end in each story's save, beside the AI control's and the idle control's (`judge FILE`). | The idle led civ. |
| Health | `check.sh` green; `stats::digest` moves (the start and the end are rule changes): `SAVE_VERSION` bumps once and the pins with it; a save played past its end replays to the same digest. | |
| Decisions | Every new thing the player meets is pressed in `www/play/end.mjs` on staged games: the end screen by conquest, by the led civ's fall and by the limit; Play on (End turn still works, orders are refused with "the game is over"); New game; the pick's card shows the limit. | |

Before the piece (spec probe, `probe.md`): with the decided start, no story takes a capital on
any small or medium retro map in 200 turns, with any civ led, and a stack banked to eight packs
reaches a rival capital only starving with one to three warriors left. So the honest answer to
"some games end before the limit" may be "not yet, on this economy", and F (the store worth
taking) and R (a rival that plays for the end) are the pieces that move it. The canvas searches
for a story that does it and records how near it gets; it does not change the fight, supply or
economy numbers to get there.

## Shape: Easel and Canvas

Taken as the piece suggests. The rules of the end are settled here (below); what only running
settles is the limit per size and whether a story can end a game early, and both are judged on
the retro maps at the end, on everything the easels built. So:

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | easel | The start: `data.rs`, founding, the AI's army, the fixtures, `Stage::army`, the stories, `Lead`, the camp sweep | Every later run is on the new start; the test fallout is its own area |
| build-2 | easel | The end: `core/src/over.rs`, `Phase::Over`, the limit, Standing over time, refusals, the CLI's text, the stage's `limit`, `vision.md` | Needs the start, so a staged conquest is only what it places |
| build-3 | easel | The judge: the end group in `core/src/judge.rs`, the sweep to the end, `judge FILE`'s end | Needs the phase and the history |
| build-4 | easel | The page: `Sim.view`'s end, the end screen, the cards, the bar, `end.mjs` | Needs the phase, the limit and the history |
| build-5 | canvas | The limit per size, the stories, the measures in `limit.md` and `stories.md` | Needs all of it |

## Settled: the start (build-1)

1. `UNIT.start` and `GUARD.start` are 0, with a `note`. Founding raises the scout alone; its
   pack still comes from the store, and what the army's pack took stays there (the endowment's
   20 grain less the scout's pack). `units::raise_at_founding` and
   `units::guard::raise_at_founding` go with their calls.
2. A new `UNIT.first: 3`: the warriors a first army has where a script or a tool needs a number:
   the camp sweep's lever, `Lead`, the stories, and the economy group's pack's worth
   (`turns_at_pack` reads `pack_cap(UNIT.first)`; it read `UNIT.start`, also 3, so it is
   unchanged).
3. The AI's camp raise in `units/ai.rs` goes, with its doc line saying the AI raises no army
   until R. Its claim and its guard stay as they are.
4. `retro::kept_camp` is "no camp lost" (it read warriors ≥ `UNIT.start`): with no army at the
   start, a civ that never raised one kept every camp it had.
5. **The fixtures.** Tests that rely on founding's camp or guard raise them where they set up
   (a `common` helper that raises `UNIT.first` warriors and three guards as founding did, so
   their assertions hold); tests of founding itself say the new start. `Stage::army` makes a
   camp when the civ has none (as `Order::Raise` would, then placed), so staged armies need no
   camp from founding.
6. **The stories raise what they script.** `surrender` raises `UNIT.first` warriors at +0 with
   their packs, then marches as now; `skirmish` and `stack` drop the +0 disband. `Lead` raises a
   camp of `UNIT.first` at +0 when it has none. The page's `lead_orders` is `Lead`.
7. `SAVE_VERSION` bumps and the pins in `cli/tests/save.rs` move.

## Settled: the end (build-2)

8. **The phase.** `Phase` gains `Over(Over)`, `Over { winner: u32, why: Why, turn: u32 }`,
   `Why::{Conquest, Limit, Fell { by: u32 }}`, all `Copy`. Serialised as now (`"main"`), the
   new arm as `{"over": {...}}`. `Phase::playing()` is true in `Main` and `Over`; every rule
   that runs only in `Main` (`leaning`, `markets`' capital market, `units::step`, `units::ai`,
   `script.rs`) runs while `playing()`, so the world runs on after the end.
9. **The step.** A last step in `TURN`, `over`, in `core/src/over.rs` (a module of its own):
   in `Main`, after the pick, it pushes the turn's Standing (item 11) and then ends the game on
   the first of:
   - **Conquest**: at least two civs were founded and at most one is standing
     (`civs::alive`). The winner is the one left (if none is left, the highest Standing, a tie
     to the lower id).
   - **Fell**: the player leads a civ and its capital has fallen. The winner is the standing
     civ with the highest Standing (a tie to the lower id); `by` is the civ that took the
     capital. In a watched game one capital falling ends nothing.
   - **Limit**: the turns since the pick reach the limit (item 10). The winner is the standing
     civ with the highest Standing, a tie to the lower id (the older civ, as `war.md`'s
     decided ties go).
   It emits a `GameOver` event (`civ` the winner, `other` the fallen civ's taker for Fell) and
   notes the winner. In `Over` the step only pushes Standing: the result never changes.
10. **The limit** counts turns after the pick. `END.limit` in `data.rs` is a turn count per
    size, by radius (`[(16, _), (23, _), (33, _)]`, as `SIZES`); a radius between sizes takes
    the next size up's, and one past the largest the largest's. `Config.limit: Option<u32>`
    (`#[serde(default)]`, `None` = the size's) lets a stage, a test or a script set a short one;
    `World::limit()` gives the one in force. Starting numbers, for the canvas to set: small
    200, medium 250, large 300.
11. **Standing over time.** `World.standing: Vec<BTreeMap<u32, f64>>`: index 0 is
    `stats::standing` at the pick (pushed by `ChooseCiv`), index `k` after `k` turns, every
    civ founded (a fallen civ's runs on at what the rule gives it). Derived and not saved
    (`#[serde(skip)]`), rebuilt by a replay as `orders` and `picked` are.
12. **Refusals.** In `Over`, `validate` refuses `Order`, `SetKeep` and `SetGift` with "the
    game is over"; `EndTurn` goes on. `World::order` (the AI's door) is not refused: the AI
    plays on.
13. **The CLI.** `play`'s view says the end in its first line ("the game is over: Blue won,
    the highest Standing at the turn limit, turn 512"), `end` keeps ending turns, and orders
    print the refusal. `story` stops at the end of the game or its `--turns`, whichever is
    first, and its summary line says the end. `--turns 0` plays to the end.
14. **The stage.** `StageSpec` gains `limit`; `Stage::over()` reads the phase's `Over`.
15. **`vision.md` Winning**: the "Not decided yet" paragraph becomes the first way to win,
    in the file's voice, with the other options kept as later ways: a civ loses when its capital
    falls; it wins when its capital is the last standing, or with the most Standing when the
    turn limit comes. No number in the prose.

## Settled: the judge (build-3)

16. **`EndScores`** in `core/src/judge.rs`, `Scores.end`, read from the world at the end of
    the watch: `over` (`Some { why: "conquest" | "limit" | "fell", winner, turn }`, the turn
    counted from the pick), `early` (over by conquest or fell, before the limit), `capitals_fallen`
    (`CapitalFell` events after the pick), `ranks` (each civ's rank in Standing at the end of
    the game, at `Over` if it ended, else at the watch's last turn), and `led_rank` (the led
    civ's, `None` when watching).
17. **The sweep plays to the end.** Each run of `judge_map` plays until `Over` and at least to
    +200, so every reading at +200 stays where it was; the end group reads at `Over`. The text
    prints one end line per map ("end: limit +200, winner Blue; no fight: limit +200; capitals
    fallen 0 (0)") and a total line ("total end: games ended before the limit 0 of 7 (no fight
    0 of 7), turns to the end 216.4 (216.4), capitals fallen 0 (0)"). The JSON carries the group
    for the run and for the no-fight control, so `judge --compare` reads it.
18. **`judge FILE`** reads the game's end group beside the AI control's and the idle
    control's, each played to the game's turns as now, and prints "end (game / control / idle)"
    with how each ended and the led civ's rank at the end.
19. `sweep_judge.rs` is 664 lines: its text output (`print_text` and the line helpers) moves
    to `cli/src/sweep_text.rs` first, byte for byte, then the end lines are added there.

## Settled: the page (build-4)

20. **`Sim.view()`** gains `over` (`null` or `{winner, why, by, turn}`), `limit` (`{turns,
    at}`: the limit after the pick and the turn it falls on; `null` before the pick) and
    `standing` (`[[turn, {civ: share}]]` from `World.standing`); `phase` reads `"over"`.
    `model.js`'s own Standing history goes: the engine's is the one, so a loaded game has its
    whole line.
21. **The pick's cards** (`civs::cards`) carry `limit`, and each card says "The game ends
    after N turns, at turn T".
22. **The top bar** shows the turn as "turn 312 of 512" in the main game, and "Game over" in
    the phase chip once it ends.
23. **The end screen** (`www/screens/end.js`): the header is the result and the line under
    it how.
    - Won: "You won" / "Yours is the last capital standing, on turn N" or "The highest Standing
      when the turn limit came, on turn N".
    - Lost: "You lost" / "Your capital fell to Blue on turn N" or "Blue had the highest
      Standing when the turn limit came, on turn N".
    - Watched: "Blue won" / the same lines in the third person.
    - Before the end, Game → Standing so far: "Turn 312 of 512" / "Blue leads".
    Below it the chart as now, from `view.standing`, then Play on and New game.
24. **When the game ends** the page stops Play, shows the end screen once, and Play on
    returns to the map with End turn working. Buttons that give orders grey with "the game is
    over" (the panel's `check` gives it). An autosave of an ended game Continues as ended.
25. **`www/play/end.mjs`** checks it on staged games (`game.stage`), each pressing the page's
    own buttons: a two-civ stage with the led civ's army on the rival's capital and no guard
    there (ends by conquest: "You won"); a three-civ stage where a rival army takes the led
    capital ("You lost", "fell to"); a stage with `limit: 3` ("turn limit"); then Play on, End
    turn advances the turn, a Raise is refused with "the game is over", and New game reaches
    start. `pick.mjs` checks a card's limit line. `check-stage.mjs` checks `view.over`,
    `view.limit` and `view.standing`'s length.

## Unsettled, and which step holds it

All in build-5, the canvas:

- **The limit per size.** What proves it: on the seven retro maps all on the AI, played past
  the limit, the leader at the limit is the leader the game settles on (define settled, for
  example the leader at the last lead change), and the Standing spread at the limit is no
  longer moving by more than the lead margin a sample; and the limit is longer than the turns
  any story takes to end a game on that size. A large map's turns cost most: the sweep's wall
  time is recorded.
- **Whether a story ends a game before the limit.** A `conquest` story (or `stack`
  rewritten): what a determined player could script to take the nearest rival's capital and
  then the other, with the levers the player has (bank, raise, resupply, the switches,
  march, pillage). Played on each retro map with each civ led, to the end, with `--no-fight`
  as the control. If no game ends early, the canvas says how near it got (capitals fallen,
  the farthest army, warriors at the capital) and why, and the piece's answer is "not yet".

## What the review runs

- `engine/check.sh`.
- `sansfiat sweep --judge --json` on main (a worktree) and on the branch, then
  `sansfiat judge --compare main.json branch.json`: the moves are the start's (no army, the
  AI's orders) and the end group is "only in B". Every run ends.
- The canvas's stories on the retro maps, `judge FILE` on a save of each: the end and the led
  rank beside the controls; one with `--no-fight`.
- `sansfiat play --game g.json` with a short limit (a staged or edited config is fine) played
  to the end: the view says it, an order is refused, `end` keeps going, and `judge g.json`
  reads the end.
- `node www/play/end.mjs` and `node www/play/pick.mjs`, and a look at the end screen's three
  screenshots.

## Decisions I made that the piece left open

1. **The limit counts turns after the pick**, not from the world's turn 0: picks fall between
   turns 237 and 431 on the retro maps, so a fixed turn would give games of very different
   lengths. The card shows both the count and the turn it lands on.
2. **The led civ's fall ends the game** (Why::Fell): "lost when your capital falls". The
   winner is then the highest Standing among the civs left, as the decided rule says when no
   last capital stands; the taker is named in the why.
3. **A fallen civ's player may keep watching** (the designer settles): Play on runs the world
   on as a watched game; the led civ's orders are refused as after any end.
4. **A watched game's end screen** (the designer settles) names the winner and how, in the
   third person, with the same chart and buttons.
5. **After the end the world runs on with every rule**; the result is fixed at `Over` and a
   later fall changes nothing. The AI keeps playing its civs; the player's civ gets no orders.
6. **Ties go to the lower id**, the older civ, as `war.md`'s decided tie for land does.
7. **The end by conquest needs two civs founded**, so a one-civ world (a stage) never ends by
   it.
8. **`Config.limit`** overrides the size's limit, so a stage, a test or a page script can end
   a game in a few turns. It is saved with the config.
9. **`UNIT.first`** (3) names a first army where a script or the judge needs a number, since
   `UNIT.start` is now 0.
10. **The scout's pack still comes from the store**; "the boon year's food stays in the store"
    is read as the army's pack not taking it.
11. **The sweep plays each run to its end and at least to +200**, so the existing +200
    readings stay comparable with main's reports.
12. **The canvas changes no fight, supply or economy number** to make a story conquer: the
    question asks whether the game as built ends early, and F and R are the pieces that move
    it.
