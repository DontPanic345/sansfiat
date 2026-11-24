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

- A button that can't act now is greyed, and its tooltip says why. Raise… and Disband… open a count with Go. (#103)
- The map fills a clan's circle in its civ's colour, pale for a clan of no civ. The feed says what brought a clan to a civ: land, gifts or trade. (#352)
- A left click on a tile centres the view on it, with the zoom unchanged, so clicking again to cycle what's there leaves the map still. (#338)
- The led civ's panel always carries a Reach line, which names the nearest claim, gift or army that brings a clan or a rival's land, with its order, or says nothing does and what would. It also carries a Capital line: who holds the capital and the nearest rival army. A claim always reaches a clan on the real maps, so a line shown only when nothing is in reach would never be seen. (#338)
- A Tests screen sits beside the four, reached from Start: each stage test is listed by name and claim, watched on its small map and marked held or failed by core's verdict, never the page's, so a rule's proof can be seen. (#339)
- Close in, a clan is drawn as it lives: a tent while it gathers, a hearth once settled, walkers the turn it comes to its home, its cloth in its civ's colour (pale for a clan of no civ) and an unaligned clan's leaning a disc on the ground under it; zoomed out to where the tents would crowd, it keeps its circle. The model's states are what the map tells apart, and no more. (#444)
