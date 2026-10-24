# Clans and the pre-game

The pre-game spreads a light population over the world before play starts, so wherever a
player could get to, people already live. As they settle, small centres grow up around
fertile land on their own, with markets where people gather. Once enough of those centres
have formed, the game can start.

merge4's clans are the starting point. Its rules for land, harvest, diet, growth and
dividing are kept unless this file changes them. The numbers below are starting values to
tune, not targets.

## Time

A turn has no fixed length. It's however long it takes the seasons to average out: a year,
or longer. Nothing in the rules counts days or seasons.

Every year is the same for now. Good and bad years are out of scope (see the end).

## Scale

Early in the game, the world is sparsely populated, but people live everywhere they can
reach. Only an isolated pocket, such as an island or a valley cut off by mountains, might
stay empty. By the end game nearly every tile has people on it, often several trading
groups to a tile. The engine is built for that end state: per-clan work must not grow with
the number of clans in the world, and markets exist to keep the number of trading pairs
manageable.

## The land

The clans need a map with food on it. This piece adds that to the placeholder map, and
nothing more.

- Each terrain carries a cap of each food it can grow. The foods are game, grain, fruit, fish
  and shellfish.
- Fertility varies in patches. A tile's caps are scaled by its fertility, roughly ×0.4 to ×2,
  so some places are clearly better than others. Fertile areas are where centres should form.
- Food that's taken regrows each turn towards its cap. Game breeds: a hunted-out tile
  recovers slowly.
- Foods are spread unevenly. A clan's land seldom carries every food, so clans start out
  specialised by where they live.

## Clans

A clan is a group of people living off a territory. As in merge4:

- **Seeds.** The world starts with a few small clans, and the population spreads from them.
- **Territory.** A clan holds a connected patch of tiles around a home tile. It grows its
  territory when it isn't fully fed, and swaps a poor tile for a better one. See *Territory
  shape* below for how big it gets and what shape it takes.
- **Diet.** A clan needs one food per person per turn. Variety counts: any one food counts in
  full only up to a share of what the clan needs, and is worth less beyond that. This is the
  diversity bonus, and it's the only reason a clan wants foods it doesn't have.
- **Growth.** Fully fed, a clan grows, and grows faster the longer it has stayed put. Short of
  food, it holds steady; well short, it starves and shrinks.
- **Dividing.** A big clan splits its land and people in two. A starving clan sends a daughter
  off to find land within a few days' walk.
- **Moving on.** A clan that starves for several turns walks to a better site.
- **Storage.** A clan keeps a small store of what it didn't eat. The store spoils: it loses a
  share every turn.

### The round

A territory stands in for a semi-nomadic life. Over a year, a clan works its way around its
land, visiting its favourite spots in turn, without walking home between each one. The
**round** is that walk: a closed loop of tiles that starts and ends at home.

- **The round defines the territory.** The territory is the tiles on the round plus the
  tiles next to it.
- **Walking the round** costs its total move cost, once a turn. This is the clan's
  everyday walking.
- **Harvesting a tile on the round** costs only the harvesting.
- **Harvesting a tile off the round** costs the walk from the nearest tile on the round, there
  and back (2 × path cost), plus the harvesting.
- **Going to market** costs the walk from the nearest tile on the round to the market, there
  and back. A round that passes near a market makes going cheap.

A clan's labour each turn is its people times the work each person can do. Walking the
round comes out of it first, then each harvest and any market trip. A clan picks what to
harvest by what each tile gives for the labour it costs: food first by diet share, then by
how much it can take. Distant food is expensive, so a clan that lacks a food either does
without it, walks a long way for it, or trades for it.

### Changing the round

A clan changes its round one tile at a time.

- **Adding a tile.** The clan adds the tile where it lengthens the loop least, between the two
  tiles on the round it fits best. Adding the cheapest way keeps the loop compact, so the
  territory grows as a blob around home rather than a line chasing the best tiles.
- **Worth working.** A clan adds a tile only if the food it opens up is worth the extra
  walking. It doesn't hold land it can't use.
- **Dropping a tile.** The clan drops the tile whose loss costs it least, and reconnects the
  round around the gap.
- **Starving.** A clan that isn't fully fed swaps its least valuable tile for a better one,
  a tile a turn, so a hungry clan drifts across the land towards better ground.
- **Home follows the middle.** Home shifts one tile a turn towards the middle of the
  territory, and the round still starts and ends there.
- **Limit.** No tile on the round is more than `territoryRadius` (2) steps from home, so the
  territory can't stretch across the map however big the clan gets.

### Grain and hearths

Clans tame the grain they live with, as in merge4. This may change later, but it's in the
right direction.

- Grain on a tile a clan has held for a long time slowly becomes tame. Grain on land nobody
  holds slowly goes wild again.
- Tamer grain grows more: a tile's grain cap rises with how tame it is.
- A fully tamed tile is domesticated. It's a **hearth**, with about 2.5 times its wild grain.

Hearths are what make a fertile area a centre that can feed a civ.

### Trails

A round walked year after year wears a trail. A tile that has been on a clan's round for
`trailAfter` turns in a row gets a trail, and its move cost drops (forest and hills to 1).
Trails belong to the tile, not to the clan, so everyone uses them. A trail that's on nobody's
round for a long time wears away again.

So a clan's round gets cheaper the longer it keeps it, and settled clans pay less to live
than ones that keep moving.

## Markets

A market is where clans meet to trade, and in the pre-game it's also a festival.

- **Forming.** A market forms on a spot that has been lived on for a long time, with enough
  clans in reach. It's at most one new market a turn, kept a distance from other markets.
- **Going.** Each turn, a clan goes to the nearest market in reach if going is worth the walk.
  The walk is from the nearest tile on the clan's round, there and back. It's worth going when
  what it would gain from trading at last turn's prices, plus its desire to gather, is more
  than the labour the walk costs.
- **Desire to gather.** Clans want to go to festivals: to see others, and to share
  their surplus. This is the one place in this piece where something like happiness comes
  in, and it's kept as simple as possible. A clan holding a surplus (a store above what it
  needs) gets a fixed bonus towards going. Sharing surplus food is the reason, and the bonus
  stands for it.
- **Clearing.** The market finds exchange ratios with no money (the engine's existing
  market library) and swaps the goods.
- **Fading.** A market nobody trades at fades away.

## The pre-game ends

The pre-game ends once enough centres have formed by themselves. A centre is:

- a market that has lasted at least a set number of turns,
- with at least a set number of clans trading there,
- with a hearth in its reach,
- and far from the other centres: at least `centreSpacing` (about 10) steps from any other.

There are always **3 players** for now, so the pre-game waits for 3 centres. It ends when
there are 3 and the people have spread over the land they can reach: at least 80% of the land
connected to the centres is within 3 steps of some clan's territory. The civs that start
from these centres begin far apart. A turn limit is the backstop. What happens next, the boon
year and founding a civ, is a separate piece.

## The proof's question

Does a sparse world of clans, walking their rounds and paying real labour to reach their
food, spread across the land and form centres with markets around fertile areas on its own,
with no rule placing them?

## Out of scope

- Good and bad years, and any year-to-year variation.
- Buildings, materials (wood, stone, clay) and crafts. Only food exists.
- Civs, founding, the boon year, and anything the player does.
- A new map generator (rivers, ridges, coasts). The placeholder map gains food and
  fertility, and nothing more.
- Hidden trail costs and scouting.
- Traders. Traders work between markets, and they come in a later piece.
- More or fewer than 3 players.
