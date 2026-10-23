# Pre-game Economy

Prototype: `D:\lab\sandbox\hex-economy` (hex map, `npm start`, `?seed=N&players=N`).

## Idea

Before the game starts, hunter-gatherer clans spread over an empty map, hold land, deplete it, divide and trade. There is no money. The pre-game ends when grain is first domesticated, and the places where that happens become the players' civs.

A turn stands for all the seasons together, not a set number of years.

## The land

- Terrain sets which foods a tile has: game, grain, fruit, fish and shellfish.
  - Plains have game and grain.
  - Forest has game and fruit.
  - Hills have a little game and fruit.
  - Desert has nothing.
  - Mountains and sea can't be walked on.
- Rivers add fish and a little grain. Coasts add shellfish. Shallow sea next to land has fish and shellfish.
- **Fertility** varies in patches and scales every food on a tile, from 0.4× to 2×. Rich land shows up as river valleys and coasts in fertile patches.
- Food that is harvested grows back each turn:
  - Plants, fish and shellfish regrow a fixed share of the tile's full yield.
  - **Game breeds back** in proportion to the herd that's left, plus a trickle that wanders in. A hunted-out tile takes a long time to recover, so big game runs out first where clans have stayed longest.

## Diet

- A clan needs one unit of food per person each turn.
- Any one food counts in full only up to 40% of that need, and at 30% beyond it. Living on a single food takes 2.4× the harvest, so clans want variety. That is why they cluster where several foods meet.
- Food left over goes into a store, and 30% of it spoils each turn.

## Clans and their land

- A clan **holds land**: a connected patch of tiles with its home at the centre. No tile has two holders.
- It forages its own land plus the unclaimed tiles next to it. It can't forage a neighbour's land, so neighbouring clans compete for the edges between them and leave thin buffers.
- **Roaming limits land.** A clan walks all its land each turn, so the total movement cost of its tiles is capped. That's about six plains tiles or three forest tiles, so forest clans hold less.
- A clan that isn't growing takes the best free tile next to its land. Once it can roam no further, it swaps its worst tile for a better one. Its land creeps toward good ground and leaves poor ground behind, and its home follows one tile a turn.

## Fed, stable, starving

- **Growing:** fully fed. Growth is 3% a turn for a clan that just moved, rising to 7% once it has stayed put for 30 turns. Settled clans grow faster.
- **Stable:** fed between 80% and 100%. Numbers hold steady. This is a clan at the carrying capacity of its land.
- **Starving:** below 80%. The clan shrinks, and it dies out if it falls below one person.

## Moving and dividing

- **Dividing:** a clan of 12 or more divides like a cell. Its land splits in two connected halves, and the daughter takes the half away from home, with half the people and food store.
- **Leaving:** a starving clan of 8 or more sends a daughter off to find new land. So does a big clan holding a single tile. The daughter only goes if a site within six steps would feed her. She starts next to her parent and walks there.
- **Moving on:** a clan that starves for three turns in a row gives up its land and walks to better ground within four steps, if there is any.
- Walking clans move one tile a turn and forage wherever they stand.
- Movement choices favour better sites without always taking the best, so clans don't all rush to the same tile.

## Gatherings

There are no permanent markets in the pre-game, only **gatherings**, temporary meeting places like seasonal festivals.

- A gathering forms at a long-held home that has at least four clans within reach. The busiest such site is chosen, and it must be well apart from other gatherings.
- It lasts only while trade there stays strong. When the average traded volume falls too low, it disperses.
- Clans prefer to settle within reach of a gathering.

## Trade

- Each turn a clan within reach of a market or gathering decides whether to go. The trip costs labour, so it harvests less, more so the further away it is. It goes only if what it would get by trading, at last turn's exchange ratios, beats what the trip costs.
- **No money.** Each trader posts its whole basket and values a basket by the sum of the square roots of its amounts. The market searches for **exchange ratios** at which supply meets demand, then hands out the goods.
- Every trade is value-neutral at those ratios, and every trader comes away better off because its basket is more varied.
- Ratios are relative only. Nobody holds them or receives them.

## Tended grain

- Grain on land that has been held for 15 turns in a row starts to **tame**. After about 120 more turns of continuous holding it is **domesticated**, at 2.5× its wild yield.
- If the land is left, tamed grain goes feral again at the same pace.
- Long-settled clans on grain land become rich, grow faster and divide more.

## Game start

- The pre-game ends once there is a domesticated **hearth** for each player, at least eight tiles apart.
- Each player takes a hearth. Their market is a gathering already near it, or a new one on it.
- Gatherings still running become permanent markets. More markets then form across the map wherever settled clans are densest.
- After that, one new market forms each turn on the longest-held site far from the others.

On the seeds tried so far, grain is first domesticated around turn 136. That's the earliest it can happen, because some grain land is held unbroken from the first turns. The game then starts between turns 150 and 200, with about 1,000 people in 130–190 clans and about 20–30 markets.

## Debug views

The terrain view shows:
- clan land, shaded and outlined in each clan's colour;
- tamed grain as gold dots;
- gatherings as dashed diamonds;
- clans coloured by state: cream for growing, amber for stable, red for starving.

The other views are heatmaps:
- population per tile of land;
- fertility;
- the tile's own yield;
- site yield (everything a clan there could reach);
- diet variety;
- food left on the tile;
- turns held;
- turns settled;
- tended grain;
- an impassable map.

## Open questions

- About 20% of clans are starving at game start. Is that the right level of pressure?
- Should grain tame only while its holder is growing, so domestication happens later and less evenly?
- Should daughters trade on better terms with their parent clans, a kinship network, instead of or alongside open markets?
- Is a gathering the right seed for a player's market, or should hearths alone decide where civs start?
