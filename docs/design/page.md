# The page

The page is the game, not a window on the engine. It teaches the few touch points the
player has, shows what changed and why, and stays out of the map's way.

## Screens

- **Start.** New game (size, seed, a random seed), continue the last game, load a file. The
  vision in a line.
- **The pick.** As now: three cards; pick one, or watch.
- **Play.** Below.
- **The end.** Won or lost and how, with Standing over the game as a line per civ. Play on,
  or a new game.

## The play screen

- **The map** fills the window: pan, zoom, fit. Tiles, each civ's land as a border, trails,
  routes on demand, units as icons with a stack's size, the ground armies hold.
- **The top bar:** the turn, every civ's Standing side by side with yours marked, End turn,
  Play and speed, and the menus.
- **The bottom panel** shows what's selected, in one shape for everything: a name and a line
  of what it's doing, its numbers, its latest reasons, and **one button row**. A unit's row:
  Move to…, Paint, Unpaint, Forage, Pillage, Resupply, the switches, Disband. The civ's row:
  Raise…, Gift, Keep, Claim. A clan's or a festival's row holds what it can be told, which
  may be nothing.
- **The feed and alerts** on the right, newest first, collapsible; clicking one goes there.
  Alerts are the led civ's trouble, and stay until seen.
- **Keys:** space ends the turn; P and U paint; H shows hidden country; Escape clears a
  selection or an armed click.

## What stays out of the way

- Debug views (fertility, hidden costs, worn land) under a Debug menu, off by default.
- No ring, radius or circle that isn't a rule.
- Markets, festivals and camps are called by their names. A unit is the camp, an army or
  the scout.

## Learning

- First-time hints at the pick, the first selection and the first alert: one line each,
  dismissed by doing the thing.
- Every number has a tooltip saying what it is.

## Saving

- Autosave to the browser every turn; Continue on the start screen. Save and load as a file.
- An old save fails with its version.

## Art and sound

- One atlas: tiles, units, icons. Placeholders first, drawn to the cells the paid set will
  fill.
- Sound hooks on feed events, silent until there's sound.

## Built as

Screens as modules; a view model built from `Sim.view()` once a turn; a draw layer per
mechanic; one panel component fed a selection; one play driver for the scripts.

## Decided

- The start screen offers New game (size, seed, a random seed), Continue with its line ("Red, turn 312, small seed 2") and Load a file; an old save's version error shows there. A bare link opens it. (#103)
- The pre-game plays on the map from Start, with Skip to the pick; the pick comes up over the map. (#103)
- The top bar shows a Standing chip per civ: yours marked "you", the leader with ▲, and clicking one selects that civ. (#103)
- The panel's rows per kind are settled: camp, scout, guard, civ, clan, festival and tile, each with its name and doing, numbers, reasons and one button row. A clan and a festival take no orders. (#103)
- A button that can't act now is greyed, and its tooltip says why. Raise… and Disband… open a count with Go. (#103)
- Escape steps back one thing at a time: a menu, then an armed click, then painting, then the selection. (#103)
- Alerts stay in the alerts box until clicked or their subject selected, and the Feed button counts them. (#103)
- The feed starts open only on windows 1400 px wide or more. Narrower, it is closed, and on a phone it is a drawer over the map. (#103)
- At laptop width and wider, the panel sits under the map in two columns: what it is, and why. On a phone it is a bottom sheet. (#103)
- Every clan's territory tint is off by default, under View → Territories. A selected clan's territory is still drawn. (#103)
- The atlas has three variants of each terrain at 128×148, with units and icons at 64×64 and masks tinted per civ; `atlas.json` is the artist's brief. `sounds.json` names a file per feed kind, and a missing file stays silent. (#103)
- Until a game can end, Game → Standing so far opens the end screen, which reads "The game goes on" over each civ's Standing line since the pick. (#103)
- The end screen comes up once, on the turn the game ends, and stops Play: "You won", "You lost", or "Blue won" for a watched game, with a line saying how and on which turn, then Standing over the game and Play on or New game. Before the end, Game → Standing so far reads "Turn N of M" and who leads; "The game goes on" is gone. (#152)
- After Play on, End turn still works and every button that gives an order is greyed with "the game is over". A game saved after its end continues as ended. (#152)
- Each pick card says "The game ends after N turns, at turn T"; the top bar shows the turn of the limit, and "Game over" once it ends. (#152)
