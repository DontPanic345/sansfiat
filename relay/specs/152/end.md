# #152 build-2 (easel): the end, evidence

What the build made, and what it was run on. The rules are in `core/src/over.rs`; the phase is
`Phase::Over` in `core/src/world.rs`; the limit is `END` and `Config.limit` in `core/src/data.rs`.

## The end, as a play

`sansfiat play --game g.json --seed 2 --size small --civ 1`, lines `end 250`, `view`, a raise, `end`:

```
turn 237, 0 of 200 turns since the pick at 237, phase main, leading civ 472
...
turn 487, the game is over: Gold won, the highest Standing at the turn limit, turn +200, leading civ 472
...
refused: the game is over
turn 488, the game is over: Gold won, the highest Standing at the turn limit, turn +200, leading civ 472
```

The limit falls on turn 437 (237 + 200). `end 250` plays 250 turns: the game is `Over` from 437 and the
world runs on to 487, as the `end` loop plays on in `Over`. The refused raise and the one more `end`
(to 488) show the refusal and that the turns go on. `judge g.json` reads the played game with no error.

## The story to the end

`sansfiat story surrender --seed 2 --size small --civ 1 --save s.json --turns 0`:

```
surrender on small seed 2: led civ Red against Blue, 200 turns past the pick (3 civs alive), saved to s.json; game over at +200: Gold won by the limit
```

`sansfiat story stack ... --turns 30` (before the end): `... 30 turns past the pick (3 civs alive), saved to s2.json; the game goes on`.

## The tests

`engine/core/tests/over.rs` (12 tests, on the stage and on a played world): conquest by the led civ
(`Why::Conquest`, `GameOver`, the refusal, `EndTurn` still works); the led civ's fall to a rival in
three civs (`Fell { by }`, the highest Standing among those left) and in two (`Fell`, the rival wins);
watching, one fall in three civs ends nothing; one civ never ends by conquest; the limit ends at the
third turn (`Over.turn == 3`, `Why::Limit`); a limit tie goes to the lower id; `standing` has
turns-since-pick + 1 entries in Main and after the end; the limit per size and the largest past it;
a played game past its end saves and replays to the same `stats::digest` and `standing`; after the
end the world runs on (AI civ orders checked, clans move).

## Digest pins

`SAVE_VERSION` stays 8 (build-1 bumped it; this piece bumps once). The pins in `cli/tests/save.rs`
move with the `Config.limit` field and the new `Phase` (both are in the world's JSON):

| pin | was | now |
|---|---|---|
| seed 1, 334 turns, watched | a8598b82eb51bbeb | d323e530c596705f |
| seed 2, 256 turns, watched | ff4816b3dc048e55 | 8599f80a5c890173 |
| seed 3, 264 turns, watched | 3105f025620ed7c0 | 856010169b2cdbf8 |
| seed 2, 256 turns, led | ccd11adcfe0474c8 | d8a0953463e71f72 |
| baseline, led, `--state` | 1be7ec6e2e7fe358 | 2f47089bf8ac6440 |

## The play scripts (build-1's fallout, fixed here)

- `www/play.mjs`'s `raiseArmy` raised each warrior from the civ's panel, which makes a new army at the
  capital each time: three armies of one warrior, and a zone of one tile. It now raises into the one
  army (selected on its tile). `game.mjs` passes again; `camp.mjs`'s painting passes.
- `www/play/buttons.mjs`'s scout Forage: the scout's pack starts full at the pick (the store fills one
  pack at a scout's cap), so Forage has no room to fill and the turn's eating is all that shows (10 → 8).
  The check now asserts the order is accepted and the pack falls only by what the scout eats.
- `www/play/game.mjs` and `www/play/feed.mjs`: with the army in one camp, the camp's tile holds the scout
  and the guard too, and the page's click cycle needs a frame between clicks to reach the camp (game.mjs
  now waits a frame, as camp.mjs does). A click on an alert also sees every alert about its subject
  (`feed.js` `see`), so the box loses three alerts (14 → 11) where feed.mjs expected one; it now asserts
  the alert clicked is gone and the box is shorter.
- `www/play/camp.mjs`'s hold is skipped: it needs a rival camp at a post, and the AI raises none until R.

## Known, not fixed here

- `sansfiat story surrender` is refused at +0: the raise of `UNIT.first` warriors "the store can't fill
  their packs" (build-1's start). The canvas (build-5) owns the stories.
- A lost led civ's feed does not show the end: `GameOver` is about the winner, so the feed (events about
  the led civ) is silent; the view's first line says it.
