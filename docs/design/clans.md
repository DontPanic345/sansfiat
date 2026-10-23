# Clans and the pre-game

The pre-game spreads a light population over the world before play starts, so wherever a
player could get to, people already live. As they settle, small centres grow up around
fertile land on their own, with markets where people gather. Once enough of those centres
have formed, the game can start.

The poc's clans are the starting point. Its rules for land, harvest, diet, growth and
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

A clan is a group of people living off a territory. As in the poc:

- **Seeds.** The world starts with a few small clans, and the population spreads from them.
- **Territory.** A clan holds a connected patch of tiles around a home tile. It grows its
  territory when it isn't fully fed, and swaps a poor tile for a better one.
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

### Labour and movement

A clan's labour each turn is its people times the work each person can do. Labour pays for
walking to food, harvesting it, and walking back.

The point of a territory is to stand in for a semi-nomadic life. Over a year, a clan works
its way around its own land, visiting its favourite spots in turn. It doesn't walk home
between each one. So walking costs less inside the territory than outside it:

- **Inside its territory**, harvesting a tile costs the tile's move cost × `inside` (about
  0.25), paid once per visit, with no walk back. The clan is assumed to be somewhere nearby
  already.
- **Outside its territory**, harvesting a tile costs the full path cost from the nearest tile
  of the territory, there and back: 2 × path cost.
- **Harvesting** costs the same labour wherever the tile is.

A clan picks the tiles to harvest by what each one gives for the labour it costs: food first
by diet share, then by how much it can take. Distant food is expensive, so a clan that
lacks a food either does without it, walks a long way for it, or trades for it.

### Trails

Walking wears trails into the land. Every time a clan enters a tile, it adds that move cost
to the tile's wear. Once wear passes a threshold, the tile has a trail, and its move cost
drops (forest and hills to 1). Trails belong to the tile, not to the clan, so everyone uses
them. A trail nobody walks wears away again.

Because walking inside a territory still costs something, clans wear trails into their own
land. Their favourite routes get cheaper over time, and the routes to market most of all.

## Markets

A market is where clans meet to trade, and in the pre-game it's also a festival.

- **Forming.** A market forms on a spot that has been lived on for a long time, with enough
  clans in reach. It's at most one new market a turn, kept a distance from other markets.
- **Going.** Each turn, a clan goes to the nearest market in reach if going is worth the walk.
  The walk is costed like an outside trip (there and back, full price). It's worth going when
  what it would gain from trading at last turn's prices, plus its desire to gather, is more
  than the labour the walk costs.
- **Desire to gather.** Clans want to go to festivals: to see others, and to share a good
  year's surplus. This is the one place in this piece where something like happiness comes
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
- in a fertile area,
- and set apart from the other centres.

It ends when there are enough centres for the players and the population has spread over
most of the land it can reach. There's also a turn limit as a backstop. What happens next,
the boon year and founding a civ, is a separate piece.

## Abstractions in this piece

Each one, and why it's here:

| Abstraction | Stands for | Why we need it |
|---|---|---|
| A turn with no fixed length | The seasons averaging out | Seasons would need a calendar, and nothing in this piece needs one |
| Territory | Semi-nomadic roaming over a year | Without it, every clan would walk every trip from a single home |
| The cheap walk inside territory | Being somewhere nearby already | Same reason; full price outside keeps distance real |
| The diversity bonus | Needing a varied diet | It gives clans a reason to want foods they don't have, so trade has a point |
| The desire to gather | Festivals, company, sharing surplus | Trade alone may not bring clans together early; this is the smallest stand-in |
| Spoilage | Food going off | Stops stores growing without limit, so a surplus has to be used |

## Open questions

1. **Grain tending.** The poc's clans tamed the grain on land they held for a long time, and
   the pre-game waited for domesticated "hearths". Is that in this piece, or a later one?
   (This draft leaves it out.)
2. **Traders.** When a clan divides, might some of the daughter's people become traders
   rather than a new clan? This draft leaves it out, and uses forming at long-lived spots.
3. **Territory size.** The poc capped a territory's total move cost. With cheap walking
   inside, should the cap be on labour instead, so a territory is as big as the clan can
   work?
4. **How many centres, how far apart, and how much land must be covered** before the game
   starts?

## The proof's question

Does a sparse world of clans, paying real labour to walk to their food, spread across the
land and form centres with markets around fertile areas on its own, with no rule placing
them?

## Out of scope

- Good and bad years, and any year-to-year variation.
- Buildings, materials (wood, stone, clay) and crafts. Only food exists.
- Civs, founding, the boon year, and anything the player does.
- A new map generator (rivers, ridges, coasts). The placeholder map gains food and
  fertility, and nothing more.
- Hidden trail costs and scouting.
