# Actors

Everything that acts in the world is an **actor**, and every actor works the same way. A clan,
the civ, an army's camp and a scout differ only in what they spend their labour on and who
chooses it. The other design files describe each actor; this file is the machine they share.

## An actor

- **People.** An actor is people, and its people give it labour each turn.
- **Home.** An actor has a home tile: a clan's home, the civ's capital, the camp's tile, where
  a scout stands.
- **Store.** An actor holds its goods in a store, or a pack if it moves. It eats from it, and
  the store spoils.
- **Territory.** An actor works a territory: the tiles it spreads its labour over.
- **Routes.** An actor walks routes beyond its territory: to its market, along its supply line,
  along a route it's scouting.

## Labour

An actor divides its labour evenly over its territory, and spends what its routes need.

- A hunter-gatherer clan divides its gathering over its territory.
- An army divides its guarding over its zone.
- A steward, later, divides his taxing over the land he taxes.

Each tile's share pays the walk to it first, then the work. So a tile far from home, or a
poor one, takes labour from better ones: holding more land isn't always better. An actor adds a
tile only if it does better with it, and drops the one it misses least.

## The cost of a route

Every walk is costed the same way, whoever walks it: there and back, year on year, priced in
the map's movement cost. A scout looking for a way through, a clan carrying its goods to
market, and a camp drawing its supplies all pay this one cost. Trails lower it for everyone.

## Markets

Every actor trades through the same market algorithm, with no money. It posts what its store
holds beyond what it keeps, and takes what is allocated. A clan's market is a festival
(`clans.md`); units trading at markets comes later.

## Who chooses

The rules choose for clans: which tiles to hold, which festival to walk to. The player chooses
for the civ's units, through orders and switches (`supply.md`). An order is a route or a
territory the player has fixed; the actor divides its labour around it as any actor does.

## Later actors

- **The farmer.** A farmer's territory is its fields, worked from home and traded from home.
  It may be one tile or many: the same rule for adding a tile decides how many, and
  intensive farming makes the territory small. Farmed land doesn't deplete as gathered land
  does.
- **The steward**, whose territory is the land he taxes, and a guard over the same land.

## Decided

- A clan takes a tile when it would feed its people better, and drops one when it has more than it needs. (#58, #61)
- A camp marches at one person's pace. (#69)
- Guarding held ground costs the camp labour, only while it's deployed. (#91)
