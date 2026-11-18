# #338 spec probe: can a session play the page?

Run on `main` at 148528b, headless Chrome through `www/play.mjs`, laptop size.

- `lead(page, { seed: 61288 })` (the pre-game run, then the first card) took 1.4 s. The first
  card led civ 200; turn 258 at the pick.
- 20 presses of Space (End turn) took 1.3 s, turn 258 to 278.
- `JSON.stringify(game.save())` written to a file is read by `sansfiat judge FILE`: it replayed the
  20 turns beside the control and the idle control and printed
  `player orders: 0 (none), 0.00 per 100 turns; turns with none 20 of 20, longest run 20`.

So a page game is judged by the same judge as a text-view game, and a 100-turn game costs seconds
of engine time: a session's play time is its own reading, not the page's.

`play.mjs`'s `open` lives as long as its node process, so holding one page across a session's tool
calls needs a process that stays up and takes commands: build-1's driver.
