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
