# #153 build-4: the page

What the scout's panel, the map and the tooltip show after build-4, and the four screenshots `www/play/scout.mjs`
keeps. Run: `node www/play/scout.mjs --out relay/specs/153/scout-page` (after `./build-web.sh`, from `engine/`).
Every button the piece adds is pressed in `www/play/buttons.mjs` (the scout's Move to…, Route from…, Route to…,
Stop, Clear route, Disband… people, Disband all on the camp, the scout and the guard) and in `scout.mjs`.

## Screenshots

- `scout-page/march.png`: small seed 2, the scout sent to tile 58 with Move to…. Its panel says "Going to -13, 1:
  3 turns", its march path (a line from the scout to 58, flagged at the end) is drawn on the map, and Disband all
  sits in the row.
- `scout-page/start-ring.png`: small seed 2, the scout at tile 58 after it stands there. Route from… has set the
  start on the capital, and the hint reads "Route from (-9, -5): now Route to…". The start ring is drawn on the
  capital in the civ's colour (a black casing round it); the capital's own red icon sits inside it, so the ring is
  faint in a still. The script checks `body.dataset.routeStart` (the capital's tile); the ring's pixels were sampled
  by hand on the canvas, not by a script.
- `scout-page/unrevealed.png`: small seed 12, the scout's route from its capital to the rival's. Nine tiles still
  to check and one mountain left: "Left unrevealed (2, -13): 20.75 through it, can't beat the known 20". The
  Unrevealed row reads "9 to check; 1 left". The scout is still selected after a click on the row.
  (Small seed 9 is the usual choice for this errand, but its scout is lost by turn 8 at the starting feeding numbers
  (build-3's losses), so the list never shows there. Build-5 tunes that.)
- `scout-page/hungry.png`: small seed 2, the scout sent to a tile ten steps out with both switches off. After a few
  turns it is hungry: the line leads "Hungry 1 turn: loses a person at 3", the Hunger row reads "foraging 0, line 0
  of 2 it eats", and the Why list gives the engine's note on the turn it went hungry.

## What the scripts check (`scout.mjs`, `buttons.mjs`)

- Move to… sets `dest` and no route, draws the march, and the scout stands there within 15 turns with the line
  saying "At". A right-click sends the selected scout to a tile.
- Route from… rings the start; Escape clears it and keeps the scout selected; Route to… sends `[start, target]`
  and clears the start mark; the known path is drawn; Clear route clears the route.
- A tile ten steps out with Move to… armed shows "Past the line's reach: its line there carries … of the 2 it eats"
  in the tooltip.
- Stop clears the scout's dest; Disband… people takes one of two people and the clan gets it back; Disband all
  sends the camp, the scout and the guard home whole, their people to the clan.
