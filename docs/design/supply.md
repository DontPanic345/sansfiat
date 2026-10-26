# Supply lines

A unit's reach comes from the food it can get, not from a rule about distance. It carries a
pack, refills it along a supply line from its capital, and lives off the land past the
line's end. The further from home, the more of its labour goes on staying fed.

merge4 built this once (`merge4/docs/notes/Supply Line Mechanic.md`). The numbers are starting
values to tune.

## The pack

- A unit carries its food in a pack and eats from it each turn, wherever it stands. It no
  longer eats straight from the store.
- A full pack holds **8** turns of food. A warrior eats **1** a turn.
- A unit with an empty pack goes hungry; after **3** hungry turns it disbands, as `founding.md`
  says.
- A unit is made with a full pack, taken from the store.

## The line

- A unit's supply line runs from its tile to its capital along the cheapest path, priced in
  the same movement cost the map already uses.
- A **resupply** fills the pack from the store and costs **1** labour, plus **1** for every **3**
  of line cost. A unit that can't pay it from a turn's labour is cut off.
- The line can't pass through a rival civ's land.
- Trails lower the line's cost like any path, so the scout's trails stretch a civ's reach.

## Past the line

- **Forage**, on the civ's own or unclaimed land: **2** labour for up to **4** food from the tile.
- **Pillage**, on a rival's land: **1** labour for up to **4** food.
- Either way the tile yields nothing to anyone for **5** turns. Clans harvest those tiles too,
  so foraging near home takes food from the civ's own clans, and pillaging starves a rival's.

## Switches

Every unit has two:

- **Auto-supply**, on by default: resupplies along the line each turn when it can.
- **Auto-forage**, off by default: when the line can't reach, forages or pillages the tile it
  stands on.

Both run at the end of the turn on the labour left over. A move the player ordered always comes
first, so there's no reserve to set.

## On the page

- Selecting a unit draws its line home, coloured by what a resupply would cost.
- The selection panel shows the pack and any hunger, with buttons to resupply, forage or
  pillage, and the two switches.
- The feed reports resupplied, cut off, went hungry, foraged and pillaged.

## Out of scope

- Rival units cutting a line; only rival land blocks one. The army piece decides what units
  do to lines.
- Porters, caches, depots, roads and pack animals.
- Rivers carrying supply.
- Goods other than food on a line.
- Fighting.
