# Clans and the pre-game

The pre-game spreads a light population over the world before play starts, so wherever a
player could get to, people already live. As they settle, small centres grow up around
fertile land on their own, with festivals where people gather. Once enough of those centres
have formed, the game can start.

A clan is an actor (`actors.md`): its labour, territory, routes and market work as every
actor's do. merge4's clans are the starting point. Its rules for diet, growth and dividing are
kept unless this file changes them.

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

- Each terrain carries a cap of each food it can grow. The foods are game, grain, fruit, fish
  and shellfish.
- Fertility varies in patches, scaling a tile's caps, so some places are clearly better than
  others. Fertile areas are where centres should form.
- Food that's taken regrows each turn towards its cap. Game breeds: a hunted-out tile
  recovers slowly.
- **Depletion.** A tile harvested past what it regrows wears down: its cap falls. It recovers
  once it rests.
- Foods are spread unevenly. A clan's land seldom carries every food, so clans start out
  specialised by where they live.

## Trails

A route walked year after year (`actors.md`) wears a trail. Trails belong to the tile, not to whoever wore
them, so everyone uses them. A trail nobody walks wears away again.

## Clans

A clan is a group of people living off a territory.

- **Seeds.** The world starts with a few small clans, and the population spreads from them.
- **Territory.** A clan holds a connected patch of tiles around its home tile.
- **Labour.** A clan divides its gathering evenly over its territory, as every actor divides
  its labour. Walking inside its own territory costs a clan little, but enough that trails
  still form.
- **Holding land.** A clan holds land as every actor does: a poor tile takes labour from
  better ones, so it can lower the harvest. A hungry clan swaps its worst tile for a better
  one, so it drifts towards better ground. Home follows the middle of the territory.
- **Diet.** A clan needs one food per person per turn. Variety counts: any one food counts in
  full only up to a share of what the clan needs, and is worth less beyond that.
- **Growth.** A clan grows only on what it harvests beyond its need. One that just meets its
  need holds; well short, it starves and shrinks. So the pre-game settles on a light
  population the land carries without strain.
- **Dividing.** A big clan splits its land and people in two. A starving clan sends a daughter
  off to find land within a few days' walk.
- **Moving on.** A clan that starves for several turns walks to a better site.
- **Storage.** A clan keeps a small store of what it didn't eat. The store spoils.

## Taming

Clans tame the grain they live with.

- Grain on a tile a clan holds tames a little each turn, and keeps taming through the main
  game; its grain grows with it. Grain on land nobody holds slowly goes wild again.
- Farming and irrigation tame it further, later.

Tamed grain is what makes a fertile area a centre that can feed a civ.

## Festivals

A clan's market (`actors.md`) is a **festival**. Clans have little to trade there, since food is too heavy to
carry far: they trade people, and later ideas and small goods such as obsidian.

- **One route.** Each clan walks one route, to one festival, each turn. It changes festival
  when another is worth more.
- **Forming.** A festival forms where clans in reach have none worth going to, kept a
  distance from other festivals. A clan can hold one on its own.
- **Going.** Going costs the walk there and back. It's worth going when what the clan gains
  there, plus its desire to gather, is more than the walk costs.
- **Desire to gather.** Clans want to go to festivals: to see others, and to share their
  surplus. A clan holding a surplus gets a bonus towards going.
- **Clearing.** The festival clears with the market algorithm and swaps the goods.
- **Fading.** A festival nobody comes to fades away.

### People at festivals

- Clans at one festival trade people. Starving clans shed people to growing ones, so a
  growing clan there grows faster, and a starving one shrinks faster, than it would alone.
- A clan too small to stand alone merges into another at its festival, or its people spread
  across the clans there.

## The pre-game ends

The pre-game ends once enough centres have formed by themselves. A centre is:

- a festival that has lasted a while,
- with enough clans coming to it,
- with tamed grain in its reach,
- and far from the other centres.

There are always three players for now, so the pre-game waits for three centres. It ends when
there are three and the people have spread over the land they can reach. A turn limit is the
backstop. Founding a civ is `founding.md`.

## The proof's question

Does a sparse world of clans, paying real labour to reach their food, spread across the land,
level off, and form centres with festivals around fertile areas on its own, with no rule
placing them?

## Out of scope

- Good and bad years, and any year-to-year variation.
- Buildings, materials and crafts. Only food exists.
- Work besides harvesting: traders, crafters, farmers.
- More than one festival per clan.
- Hidden trail costs and scouting (`scouting.md`).
