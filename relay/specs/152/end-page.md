# #152 build-4 (easel): the page shows the end, evidence

What the build made, and what it was run on. The engine's view is `web/src/lib.rs` (`view`: `over`,
`limit`, `standing`; `phase`: `"over"`); the card's limit is `civs::cards` (`Card.limit`); the page is
`www/screens/end.js`, `www/screens/play.js` (`checkEnd`, `halt`, `standing()`), `www/bar.js`, `www/screens/pick.js`.

## The end, as the page shows it

Screenshots (half size, `page.shot(file, 0.5)`), from `node www/play/end.mjs --out relay/specs/152/end`:

- `end/end-won.png`: conquest (A's army of five on B's capital, B's militia and guard beaten).
- `end/end-lost.png`: the led civ's fall, "Your capital fell to Blue on turn 1", the chart a line per civ.
- `end/end-limit.png`: three civs, `limit: 3`, the header "Blue won" or "You won" and the line "...turn limit...".

## What end.mjs covers

Each game is staged (`game.stage`), then played by the page's own End turn, Play on, Standing so far, New game and
the civ chip (the panel's Raise…):

- conquest: "You won", "last capital", a polyline per civ;
- fell: "You lost", "fell to <taker>";
- limit: "turn limit"; then Play on (the turn advances, the phase stays over), Raise… disabled with title "the game is over",
  Standing so far shows the screen again, New game reaches start;
- watched: the same limit with the pick's Watch: the header names the winner ("X won"), not "You".

`pick.mjs` checks each card's "The game ends after N turns, at turn T" (T = the turn now + N, N = 200 on small) and that
the view's `limit` after Lead is the same. `bar.mjs` checks the turn reads "312 of 437" (`#turn` and `#turnlabel`).
`check-stage.mjs` checks a staged game with `limit: 2`: phase `over`, why `limit`, `over.turn` = `limit.at`, standing
length 3, an order refused with "the game is over", and an EndTurn still going through.
`save.mjs`: Continue has the same Standing length as before the save (the history is the engine's now).

## What had to change to stage an end

- The staged civs' clans were 0.5 people and died on the first turn (a clan under one person dies, `clans.rs`), so the
  stages use 1.5 people and grain on each capital (a clan with no food starves to under one).
- `Stage::army` now holds the camp on its tile (`dest` = the tile). A camp with no `dest` posts itself toward a
  rival or its own capital (its auto switches), so a rival's army placed on a led capital walked off it on turn 1.
  The over.rs test already set `dest` by hand (`capital_open_to`); the stage does it now. Stage-only: a played game's
  camps are not changed.
