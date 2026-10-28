# Supply lines

A unit's reach comes from the food it can get, not from a rule about distance. It carries a
pack, refills it along a supply line from its capital, and lives off the land past the
line's end. The further from home, the more of its labour goes on staying fed.

Every unit is an actor (`actors.md`) and works this way: the army's camp, whose pack is the
camp's stock, and the scout.
The pack is also what a unit will post from and fill when units trade at markets, later.

The idea is `docs/notes/Supply Line Mechanic.md`, and merge4's numbers are a place to start
tuning from.

## The pack

- A unit carries its food in a pack and eats from it each turn, wherever it stands.
- A full pack holds several turns of food.
- A unit with an empty pack goes hungry; hungry for a few turns, it disbands.
- A unit is made with a full pack, taken from the store.

## The line

- A unit's supply line runs from its tile to the capital, where the store is, along the
  cheapest path. It's costed as every route is (`actors.md`).
- What a line carries each turn is the labour put on it over the line's cost. The unit pays
  the labour, so a long line takes most of a unit's turn, and past some length it's cut off.
- The line can't pass through a rival civ's land, or ground a rival army holds.
- Trails lower the line's cost like any path, so the scout's trails stretch a civ's reach.

## Past the line

- **Forage**, on the civ's own or unclaimed land: labour for food from the tile.
- **Pillage**, on a rival's land: more food for less labour than foraging.
- Either way the tile yields nothing to anyone for a while. Clans harvest those tiles too,
  so foraging near home takes food from the civ's own clans, and pillaging starves a rival's.

## Switches

Every unit has two:

- **Auto-forage**, on by default: forages or pillages the tile it stands on. A scout forages
  along its route, and seldom needs its line.
- **Auto-supply**, on by default: resupplies along the line each turn for what foraging
  didn't cover.

Both run at the end of the turn on the labour left over. A move the player ordered always comes
first, so there's no reserve to set.

## On the page

- Selecting a unit draws its line home, coloured by what it carries for its cost.
- The selection panel shows the pack and any hunger, with buttons to resupply, forage or
  pillage, and the two switches.
- The feed reports resupplied, cut off, went hungry, foraged and pillaged.

## Out of scope

- Porters, caches, depots, roads and pack animals.
- Rivers carrying supply.
- Goods other than food on a line.
- Units trading at markets.
- Fighting.
