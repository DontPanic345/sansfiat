# Supply Line Mechanic

Prototype: `D:\lab\sandbox\supply-lines` (hex 4X, `npm start`).

## Idea

Armies run on food, and food comes from home. The further a unit is from a city, the more effort it spends just staying fed, so reach is limited by logistics rather than by a hard rule.

## Labour

- Each unit has a **labour** pool that refreshes every turn.
- Everything a unit does draws on it: moving, fighting, resupplying, foraging and pillaging.
- Rough terrain costs more labour to cross. Plains are cheap, forest costs more, hills cost the most.

## Food store

- Units carry a small **store of food**. They eat from it every turn and use extra when they fight.
- Food in the store means a unit can fight without first spending labour on resupply.
- With an empty store a unit can't attack, and it suffers **attrition**, losing strength each turn until it is fed or dies.
- Fed units slowly recover.

## Supply line

- A unit refills its store from the nearest city that has spare supply, along a **supply line**.
- The line runs through friendly or neutral land. It can reach a unit standing in enemy land but can't pass through it.
- The line is measured by how hard the ground is to cross, so hills and forest shorten an army's reach.
- The longer the line, the more labour a resupply takes. Near home it's nearly free. At the edge of range it uses the unit's whole turn, and beyond that the unit is cut off.
- Supply a city hands out comes from its production, so a large army in the field slows building at home.
- When a unit is selected, its supply line is drawn on the map, and it shows whether the unit can draw on it this turn.

## Living off the land

- **Forage**: in friendly or neutral land, a unit takes food from the tile it stands on.
- **Pillage**: in enemy land, a unit takes more, and more cheaply.
- Either way the tile is stripped for a while and yields nothing to anyone. Foraging near your own cities hurts your own output, while pillaging hurts the enemy's.

## Automation (not in the prototype)

Two checkboxes per unit to cut the micromanagement:

- **Auto-supply**: the unit tops up its store along its supply line automatically, spending the labour that takes.
- **Auto-forage/pillage**: when it can't resupply, the unit forages or pillages its tile automatically.

## Open questions

- Should enemy units be able to cut a supply line, not just enemy territory?
- Should there be ways to extend lines, such as roads, depots or supply wagons?
- How do auto-supply and auto-forage interact with labour the player wants to keep for moving or fighting? For example, should there be a reserve threshold?
