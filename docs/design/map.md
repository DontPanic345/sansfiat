# The map

The world the clans spread over: coasts and seas, ridges with passes, rivers running to the
sea, deserts, and fertile land along the rivers. It replaces the engine's placeholder map.

merge4's generator is the starting point (`merge4/src/mapgen.js`): height noise, inlets and
inland seas, ridges with hilly flanks and passes, deserts, rivers, coast and fertility. Its
rules are kept unless this file changes them. The numbers are starting values to tune.

## Size

The player picks the map size when starting a new map: small (today's size), medium and
large, each about twice the area of the one before. The number of features (ridges, rivers,
inlets, seas) scales with the area.

### Always three centres

The pre-game ends with three centres on every size and seed. If it hits its turn limit with
fewer, it tops up with the best sites it can find: the best markets first, then the most
tended land, then any fertile land, raising a market where there isn't one.

## Rivers

- A river starts on high ground and runs to the sea, or into an inland sea or lake. Rivers
  can join.
- It heads for the sea fairly directly, with some wiggle; it never doubles back on itself.
  merge4's rivers follow the steepest drop on noisy height and wander round its pits.
- A river tile is more fertile than the land around it.
- **Crossing a river costs movement.** Entering a river tile from anywhere but the river's
  own course costs extra labour; travelling along a river doesn't.
- A trail on a river tile bridges it: the crossing costs nothing extra. Worn trails and
  built trails both bridge.

## Deserts and seas

- Deserts are dry land, kept to a share of the land as in merge4, and they sit in the rain
  shadow of ridges: on the side away from the sea.
- Inland seas and lakes break up the interior. A lake is a small inland sea that a river can
  feed or drain.

## Closed country

The map isn't all open. Ridges and water close off some regions, so getting in is a choice:

- **Valleys behind a pass.** A region ringed by ridges with one or two narrow passes in. Whoever
  holds the pass holds the valley.
- **Sealed valleys.** A region ringed by ridges and water with no pass at all. Clans can't
  walk in, so it stays empty through the pre-game, and only a scout's bridge opens it: at
  least one mountain on its wall can be bridged.
- **Peninsulas and necks.** Land joined to the rest by a strip a tile or two wide.

A map has a few of each, more on bigger maps. They're never where a founding centre has to
be: the open land still has to grow three centres.

## Also

- **Floodplains.** The last stretch of a big river, near the sea, is the most fertile land on
  the map.
- **Deltas.** Where a big river meets the sea it splits into marsh: very fertile, slow to
  cross.

## Hidden trail costs

The map rolls each forest, hills and mountain tile's hidden trail cost and each mountain's
bridge span (`scouting.md`), so the scout only reads them. Until the scout exists, worn
trails bring forest and hills down as they do now.

## On the page

The page fills the window, and the map takes the space. It pans and zooms, since a large map
won't fit on screen.

- A new map control with the size and the seed.
- Rivers, deserts, floodplains, deltas and closed country are drawn so you can tell them
  apart at a glance.
- A top bar: turn, phase, people, clans, markets, hearths, centres, with end turn and speed.
- An event feed beside the map, newest first: a clan divides, starves or moves on, a market
  forms or fades, a hearth is tamed, a centre forms, the pre-game ends. Clicking an event
  centres the map on it and selects what it's about. The events are derived from the turn,
  not saved, so a replay makes the same ones, and the CLI prints them.
- A tooltip on hover naming the tile's terrain and the clan holding it.

## Out of scope

Climate beyond deserts and rain shadow, seasons, and rivers changing course.
