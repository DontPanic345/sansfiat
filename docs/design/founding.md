# Founding

The pre-game ends with three centres (`clans.md`). Founding turns each into a civ, the player
picks one to lead, and the main game starts. This file covers what a civ needs to grow and to
make, move and disband units; the rest of a civ comes in later pieces.

merge4's founding is the starting point. Its rules are kept unless this file changes them.

## Founding

- Each centre becomes a civ, with a name and colour, owning its market. A capital city rises
  on the market tile, and the market never fades.
- The capital holds the land within 2 tiles as the civ's territory and works the tiles next
  to it.
- The player picks a civ to lead, or watches all three on the AI. The pick shows each civ's
  clans, people, hearths, terrain and distance to the nearest rival.
- Every clan within reach of a founding market joins the nearest one. A civ that gets none
  takes the nearest unaligned clan. Other clans stay unaligned.
- Clans that join keep living as they did; the clan rules run for every clan in the main
  game, and the other markets keep forming and fading as in the pre-game.
- A civ clan's daughters belong to its civ, so a civ grows as its clans do.

The pre-game always ends with three centres (`map.md`), so there are always three civs.
They don't have to be able to reach each other.

## The store

- Each civ has one store, holding every good.
- It starts with an endowment, 30 grain, standing in for the boon year: the civ's clans'
  stores are empty at founding.
- Each turn, a fed civ clan hands half its leftover store to the civ.
- Food in the store spoils, as it does in a clan's store.
- The store has a cap. Past it, the excess is lost.

## Units

A unit is people, so it costs people and food.

- **Making one** takes a person from a civ clan near the capital, whose people drop by one.
  It's made at the capital, with a full pack of food from the store (`supply.md`).
- **Keeping one** costs its food each turn, eaten from its pack. A unit with an empty pack
  goes hungry; hungry for a few turns, it disbands.
- **Disbanding** sends its person back to the nearest clan of its civ.
- **Moving.** Select a unit, right-click a tile, and it paths there over the cheapest route
  it knows, a turn's labour at a time.

The one kind of unit for now is a **warrior**: a generic unit that moves, eats and disbands,
and nothing else. It's a stand-in so founding has a unit to work with; the scout, army and
supply pieces replace or change it. Each civ starts with one.

## Out of scope

- The boon year itself; the endowment stands in for it.
- Clans joining or leaving after founding.
- Winning and losing, so civs needn't reach each other.
- Cities beyond the capital, projects, materials and crafts, posting to markets, drafted
  labour, good and bad years, discoveries and fog.
- Anything the AI civs do beyond existing.
