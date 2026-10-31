# Founding

The pre-game ends with three centres (`clans.md`). Founding turns each into a civ, the player
picks one to lead, and the main game starts. This file covers what a civ needs to keep its
units; the rest of a civ comes in later pieces.

## Founding

- At each centre, its clan becomes the civ, with a name and colour. Its home tile is the
  capital. The capital gets a market of its own there, unless one is already on that tile; the
  festival the clan went to stays where it is. The capital's market never fades.
- At founding, every clan whose path to a capital's market costs no more than its path to its
  festival switches to the capital's market. After that, clans choose festivals as ever.
- The player picks a civ to lead, or watches all three on the AI. The pick shows each civ's
  clans, people, tamed grain, terrain and distance to the nearest rival.
- Every clan that comes to a founding festival joins its civ. A civ that gets none takes the
  nearest unaligned clan. Other clans stay unaligned.
- Clans that join keep living as they did; the clan rules run for every clan in the main
  game, and the other festivals keep forming and fading as in the pre-game.
- A civ clan's daughters belong to its civ, so a civ grows as its clans do.

The civs don't have to be able to reach each other.

## The store

- Each civ has one store, at the capital, holding every good.
- It starts with the boon year's surplus: the civ's clans' stores are empty at founding.
- **The levy.** A civ clan pays a share of its surplus into the store when it walks to the
  capital's market. A civ clan that goes to another festival pays nothing. The levy slows the
  clans' growth, since a clan grows only on its surplus.
- Food in the store spoils, as it does in a clan's store.
- The store has a cap. Past it, the excess is lost.

## Units

A unit is people, and its people belong to the civ, not to a clan. The civ and each unit are
actors (`actors.md`), and a unit carries a pack, eats from it, and fills it as `supply.md`
says.

- **The army** is a camp of warriors. The player moves the camp, not the warriors, and the
  army divides its guarding over its zone. A civ has
  one camp, and starts with a small one, its pack stocked from the boon year.
- **Raising** warriors takes people from the civ's clans, whose people drop by one each.
- **Disbanding** sends warriors back to the civ's own clan.
- **The scout** is a small party that walks the country (`scouting.md`). Each civ starts with
  one.
- **Moving.** Select a unit, right-click a tile, and it paths there over the cheapest route
  it knows, a turn's labour at a time.

## Out of scope

- The boon year itself; the surplus stands in for it.
- Clans joining or leaving after founding.
- Winning and losing, so civs needn't reach each other.
- Cities beyond the capital, projects, materials and crafts, posting to markets, good and bad
  years, discoveries and fog.
- Anything the AI civs do beyond keeping their units.

## Decided

- A unit is a camp or a scout; warriors are the camp's people. (M2 retro, owner note 7)
- Scouts are raised and disbanded like the camp. (M2 retro, owner note 7)
- If the civ's own clan is gone, its biggest clan takes over the capital. (#67, #69)
- The levy is a share of the food civ clans bring to the capital's festival, until the store is full. (#72)
- Only unaligned clans lean toward a civ. Gifts and trade at its festival draw them, the leaning fades, and enough of it makes them join. (#81)
- A civ gives its spare food at its festival to win clans; how much it gives is the player's choice. (#81)
- A civ can lose clans and people to its rivals. (#58, #63, M3 answer 2)
