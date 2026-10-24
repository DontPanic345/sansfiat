# Founding, the HUD, then the scout

The scout needs a civ, and so do the army and supply lines: all three of M1's remaining
rebuilds are things a player does. The engine has no player yet; the pre-game ends at three
centres and stops. So founding comes first, and M1 becomes founding plus the three rebuilds.

## Order

1. **The map and the page** (#31): merge4's generator on the engine, on bigger maps
   with closed-off regions, and a full-screen page to watch the pre-game on, from
   `docs/design/map.md`. Founding sites and the pre-game's tuning only mean something on a
   real map, so it comes first.
2. **Founding** (#32): the pre-game hands over to three civs, the player leads one,
   and units can be made, moved and disbanded through a selection panel. From
   `docs/design/founding.md`.
3. **Scout on the engine** (#28), from `docs/design/scouting.md`.
4. Army and supply-lines interviews and rebuilds (#27, #29), on the same founding.
5. Retro (#30).

#26 closes into piece 1.

## Piece 1: The map and the page (#31)

**Goal:** The pre-game runs on a real world: coasts, ridges with passes, closed valleys,
rivers running to the sea, deserts, lakes, floodplains and deltas, on a map size the player
picks. It always ends with three centres, and you can watch it on a full-screen page that
tells you what's happening without clicking every tile.

**Spec:** sonnet high. The generator exists in merge4 and the work is porting it, so the
judgment is in the new parts (rivers that run to the sea, closed country, rain shadow,
floodplains, deltas) and in checking the pre-game still ends on every size. The page is
plain JS. The spec can put opus on any build step that needs it.

**Design:** docs/design/map.md, docs/design/clans.md

**Decided:**
- merge4's generator ported to the engine, with the changes `map.md` makes.
- Rivers cost movement to cross, not to follow; a trail bridges them.
- Hidden trail costs are rolled here; nothing reads them until the scout.
- Map size picked on a new-map control, with the seed.
- The pre-game always ends with three centres, topping up at the turn limit.
- A sweep over seeds and sizes shows how often the pre-game ends on its own and how often it
  needs the top-up.
- The page as `map.md` says: full screen, pan and zoom, top bar, event feed, tooltip.
- Debug views stay. The review starts a new map and watches the pre-game through the page
  only.

**In scope:** the generator, closed country, river crossing, the top-up, the events and CLI
output, the page, tests that events fire and replay identically, and the sweep.

**Out of scope:** founding, civs and units; a minimap, graphs, start screen and saving;
climate beyond deserts. merge4 stays frozen.

It runs after #23, so the generator's maths is the same native and in the browser.

## Founding

`docs/design/founding.md` is merge4's founding, cut down to what units need. Each centre
becomes a civ with a capital city, the player picks one, and clans in reach join it. Civ
clans' daughters stay in the civ. The store starts with 30 grain, takes half of each fed
clan's leftover store, spoils and has a cap. A unit is a person from a civ clan, fed from the
store, and every unit can move and disband. Founding adds the selection panel and the civ's
numbers to the page piece 1 builds; the scout adds its controls to them.

## Piece 2: Founding (#32)

**Goal:** The pre-game ends with three civs, the player leads one, and every order is given
through the page: no debug tools.

**Spec:** opus high. Founding sets the shape every player mechanic builds on: what a civ is,
what it owns, and how a unit is made, moved and disbanded. The HUD is plain JS.

**Design:** docs/design/clans.md, docs/design/founding.md

**Decided:**
- Founding, the store and units as `founding.md` says.
- Every unit moves and disbands. Select it, right-click a tile, and it paths there over the
  cheapest route it knows; disband is a button in the selection panel.
- New events: a civ is founded, a unit is made, goes hungry or disbands.
- The top bar adds the player's civ, its people, its store and its units; the tooltip adds
  the tile's civ.
- A selection panel: what's selected, and the actions it can take as buttons.
- Debug views stay. The review plays founding, and makes, moves and disbands a unit, through
  the interface only.

**In scope:** founding, the capital, the store, making, moving and disbanding units, the
selection panel, the new events, and tests.

**Out of scope:** scouting; the boon year; minimap, graphs, start screen, saving.

Runs after #31. Read its PR description and fold in any of its findings that matter here.

## Piece 3: Scout on the engine (#28)

**Goal:** A civ's scout finds and improves routes over hard country, given by the player
through the interface, as `scouting.md` says; `scout/` is archived.

**Spec:** opus high. Hidden costs touch the map, trails, movement and the clans' worn
trails; the search needs care.

**Design:** docs/design/scouting.md, docs/design/founding.md

**Decided:**
- Hidden trail costs fixed at map generation; about 1 in 5 mountains can be crossed.
- One trail per tile, worn or built, at the tile's hidden cost; any unused trail wears away.
- A revealed mountain takes the scout's whole turn.
- The search works outward from the scout instead of ping-ponging along a range.
- Selecting a scout gives "start here" and "route to here" in the selection panel, beside
  move and disband. The page shows known and possible routes with their costs; the feed
  carries route found, no route and trail built.
- The review plays a route through the interface only.

**In scope:** hidden costs, routes, built trails, decay, the page's route view, events,
tests, archiving `scout/`.

**Out of scope:** fog, bridge-span technology, alliances, several scouts on one route.

Runs after #32. Read its PR description and fold in any of its findings that matter here.
