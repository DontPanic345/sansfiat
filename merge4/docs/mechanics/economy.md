# Mechanics: people, markets and civs

Exact rules of the proof of concept as built. Constant names refer to `src/data.js` (core) and
`src/modules/civ/data.js` (`CIV`). [DESIGN.md](../DESIGN.md) holds the vision and the status summary;
this file says how each mechanic works now. Phases are those of the turn pipeline (DESIGN.md, "Turn order").

## Map

- Pointy-top axial hexes, `MAP.width` × `MAP.height` = 44 × 28.
- Generation (`src/mapgen.js`): smoothed height noise, `MAP.inlets` 5 inlets, `MAP.inlandSeas` 2 inland seas,
  `MAP.ridges` 5 mountain ridges of 10–24 tiles with hilly flanks (`flankHills` 0.4) and passes (`passChance` 0.06),
  deserts on about `desertShare` 20% of land, `MAP.rivers` 24 rivers, coast and shallows (ocean next to land).
- Food caps per tile: `TERRAIN[t].food` plus `FEATURES` (river: fish 5, grain 1; coast: shellfish 1;
  shallows: fish 4, shellfish 4), all scaled by fertility drawn in patches from `MAP.yieldRange` [0.4, 2].
- Move cost (`TERRAIN.moveCost`): plains, desert 1; forest, hills 2; mountain and ocean impassable.
- Hidden trail costs (`TRAILS`), rolled per tile at generation: forest trails to 1 or 2 (even odds); hills to 1 or 2
  (2 twice as likely); mountains to 3 with a bridge span of 1 (1 in 10), 2 (1 in 10) or never (8 in 10).
  Stored as `tile.trailCost` and `tile.span`; see [military.md](military.md#scouts-trails-and-fog) for scouting.
- Regrowth (`regrow` phase): each food regrows `FOOD[f].regen` of its cap a turn; game breeds logistically
  (regen × stock × (1 − stock/cap)) plus `restock` 2% of cap wandering in. A `fallow` tile regrows nothing until its
  count runs out. Regrowth is scaled by the `yieldFactor` hook (good and bad years, main game only).
- Terrain `defence` (forest 0.25, hills 0.5) and `CITY.defence` 0.5 are used in combat: see
  [military.md](military.md#armies).

## Clans

All clans, in the pre-game and the main game, run the hex-economy rules in `src/clans.js` (`CLAN`).

- **Seeds.** One clan of `CLAN.startPop` 6 people on each landmass of at least `MAP.minRegion` 30 tiles, topped up to
  `CLAN.seeds` 3.
- **Land.** A clan holds connected tiles (`tile.claim`) and forages them plus every unclaimed neighbour. Its land's total
  move cost is held to `CLAN.roam` 6. A clan that is not fully fed claims the best free tile next to its land, or swaps
  its worst tile for a better one if that gains more than `landMargin`. Home recentres one tile a turn.
- **Harvest** (`forage` phase): labour is `pop × harvestPerHead` 1.5 × (1 − market trip) × (1 − drafted/pop), taking at
  most `takeRate` 50% of each tile's stock and aiming for `surplusTarget` 1.2 × pop including store. Each food is filled
  to its diet share first.
- **Diet** (`settle` phase): need = pop. One food counts in full up to `dietShare` 40% of need and at `excessValue` 30%
  beyond. Leftovers are stored and lose `spoilage` 30% a turn. Only foods are eaten.
- **Growth.** Fully fed: +3% a turn, rising to +7% over `settleTurns` 30 settled turns. Fed below `stableAt` 80%:
  starving, shrinks by `starve` × (1 − fed/0.8). Below 1 person the clan dies out.
- **Dividing.** At `splitAt` 12 people a clan with 2+ tiles halves its land; the daughter takes the far half and half the
  people and store. Without land to halve, or starving at `hungrySplit` 8, it splits: a daughter of half (at most 6)
  walks to a site within `scoutRange` 6 steps that would feed her to `settleFed` 90%; none, no split. The civ module's
  `daughterSite` hook can choose that site (Expansion, below). Daughters inherit `owner` and `leaning`.
- **Moving on.** After `patience` 3 starving turns, a clan walks to a site within `moveRange` 4 steps that scores at least
  `moveMargin` better (site score = fed share + `MARKET.accessBonus` 0.2 if in a market's reach). A blocked walker gives
  up after 2 turns.
- **Tending** (`occupy` phase): grain on land held `TENDING.tendAfter` 15 turns in a row gains `rate` 1/120 tame a turn;
  unheld it loses the same. Grain cap = wild × (1 + 1.5 × tame); at tame 1 the tile is domesticated (2.5×), a **hearth**.

## Markets

- **Clearing** (`src/market.js`, `clear`). Each trader values a basket at Σ√amount. Prices (exchange ratios, geometric
  mean 1) are found by up to 300 iterations of tâtonnement, then whichever side of each good is long is rationed so goods
  are conserved exactly. `cleared` is false when fewer than 2 traders or 2 goods were posted; the market then keeps its
  last real ratios. Clans want only foods. A civ that posts writes `m.wants[civId]`: every good (`GOODS`: 5 foods, wood,
  stone, clay, pottery) it is not posting itself (all foods if it posts everything), so it never buys back its own
  surplus. `m.posts` and `m.wants` are cleared after clearing.
- **Going to market** (`forage` phase). A clan trades at the nearest market within `MARKET.reach` 3 path cost if its
  basket, less the trip (`tripCost` 5% of labour per unit of path cost), is worth more at last turn's ratios. Trails
  lower path cost, so they widen a market's catchment.
- **Formation** (`markets` phase). A market forms on a settled tile held `formAfter` 20 turns, at least `spacing` 5 from
  every market, with `gathering.minClans` 4 clans in reach: in the pre-game the most populous such site (a temporary
  gathering), in the main game the longest-held one, and not within `spacing` of a market that fell quiet the same
  turn. One a turn at most.
- **Fading.** Every market keeps `activity` = 0.8 × activity + 0.2 × volume (`gathering.fade`). A market no civ owns
  (a gathering, or a neutral market in the main game) disperses when activity falls below `gathering.minVolume` 2.
  A civ's founding market never fades.

## Pre-game and founding

- **End rule** (`pregameOver`). From `PREGAME.minTurns` 100 the pre-game ends as soon as `foundingSites` finds one site per
  player; at `PREGAME.maxTurns` 500 it ends regardless, with fallback sites. Seeds 1–3 end at turns 172, 186 and 197.
- **Founding sites** (`foundingSites`). A site is a thriving market (no owner, at least `FOUNDING.minTraders` 4 traders,
  activity at least `FOUNDING.minActivity` 4) with a hearth in its reach. Markets are `FOUNDING.spacing` 8 apart and
  hearths `TENDING.hearthSpacing` 8 apart, ranked by activity + traders, and **all on one landmass** (`landComponents`:
  land plus tiles a trail or bridge could open), so every civ can reach every other. Fallback (turn limit): the best
  landmass is topped up with the best-tended held tiles, then any fertile tile, on a new market if needed.
- **Decision:** the design asked for "settled land and one thriving market per player with grain in reach"; the code
  adds the shared-landmass rule because rivals on separate islands could never fight or trade, and requires a fully
  domesticated hearth so every civ starts with agriculture.
- **Founding** (`foundCivs`). Each site becomes a civ (`CIVS` names and colours) owning that market. Other gatherings become
  permanent markets, and settled sites far from any market with 4+ clans in reach get one. Phase becomes `choose`.
- **Choosing** (`chooseCiv`). The player picks a civ, or `civ: null` to watch all AI. The founding panel shows each
  candidate's clans, people, domesticated tiles, terrain mix and distance to the nearest rival.
- **Civ start** (civ `onStart`). A capital city rises on each founding market. Every clan within market reach of a founding
  market joins the nearest one; a civ that gets none takes the nearest unaligned clan. `startPop` records each civ's people.
  Each civ's store starts with `CIV.endowment` (grain 30): its clans' stores are empty at founding, and without it the
  first gifts, porters and army provisions are unaffordable. Each civ also gets one scout (scout `onStart`). `PLAYERS` allows 2–4 civs (browser default 3).

## Civs

### Store

- **Decision: one pooled `civ.store` per civ** (not per city). Holds every good.
- **Cap** (`capOf`): `CIV.baseCap` 50 per city + `storehouseCap` 80 per storehouse, all goods together. Past the cap
  materials and crafts are lost first, then food (`settle` phase, after the levy).
- **Spoilage.** Store food loses `CLAN.spoilage` 30% × `granarySpoil` 0.3 (any granary) × `potterySpoil` 0.7 (pottery
  known). Member clans' stores get × 0.3 within `granaryReach` 3 of a granary city, and × 0.7 once pottery is known.
- **Levy.** Fed member clans hand `CIV.levy` 50% of their leftover store to the civ each turn.

### Drafted labour

**Decision:** labour is people taken from the civ's own clans for the turn, not a separate pool.

- Before foraging (`draft`), each city states its needs in workers (`CIV.workers`): `project` 3 while building;
  `craft` 1 when pottery is known, clay ≥ 1 and pottery under `matTarget` 12; `hunt` 1 when a work tile has game;
  `farm` 2 while store food is under `foodTarget` 50% of the cap; `material` 1 for each of wood, stone, clay under
  `matTarget` 12 that its work tiles yield (forest, hills, river).
- Non-starving civ clans give up to `draftMax` 25% of their people, scaled down to what is wanted (`clan.drafted`).
  Drafted people still eat but do not forage. Starving clans are never drafted.
- Each wanted worker yields `labourPerWorker` × drafted/wanted labour (`work`, after foraging):
  projects gain labour; crafters turn up to `craft` 2 clay into pottery each; hunters bring `huntPerWorker` 1.5 game;
  farmers take `farmPerWorker` 1.5 food from the tiles within `CITY.workRadius` 1; gatherers bring `matPerWorker` 1.5 of a
  material, at most `tileMax` 1.5 per matching tile, scaled by the year.
- **Work tiles**: within `CITY.workRadius` 1 of the city, plus the land of the civ's clans within `CITY.territoryRadius` 2.

### Projects

`setProject` takes the goods from the store at once; drafted labour then completes it. One project per city.

| Project | Labour | Goods | Effect |
| --- | --- | --- | --- |
| granary | 10 | wood 4, clay 2 | Food spoilage × 0.3 in the store and in member clans within 3 |
| storehouse | 12 | wood 6, stone 2 | Store cap +80 |
| wall | 15 | stone 6, wood 2 | Defenders in the city × `ARMY.wallMult` 1.5; garrison + `ARMY.wallGarrison` 2 |

AI civs start the first project they can afford.

### Trading at market

- **Posting** (`trade` phase, before clearing). A civ posts everything above its reserve, split evenly across its trade
  markets: its own, plus one other within `tradeRange` 12 (the one whose trading clans hold the most land yielding the
  materials it is short of, else the nearest). That market may be a rival's.
- **Default reserve** (`keepOf`): food and crafts keep `CIV.keep` 8% and 5% of the cap per good; a material keeps what the
  dearest unbuilt project needs, and at least `keepMat` (clay 4, for pottery practice). `setPosting {good, keep}` sets the player's own reserve (`keep: null` restores default).
- **Material supply. Decision:** clans trading where a civ posts bring `clanMat` 0.3 of wood / stone / clay (× year) for
  each forest / hills / river tile they hold, and sell it, since clans want only food. That is how a civ without hills
  gets stone. Materials a clan fails to sell are dropped after trade.
- Goods received are added to the store after clearing (`collect`).

### Expansion

- **Daughters** go where the `daughterSite` hook says, in order: the player's chosen daughter site (`setDaughterSite`,
  a passable tile within 6 of a civ clan, not in rival territory or on a rival clan's land; lapses after `siteTurns` 20);
  else the option nearest the chosen city site; else the best-scoring option at least `CITY.minSpacing` 3 from every
  city. Only sites that would feed her count. Land halving (dividing) is not steered.
- **Cities.** With no city site chosen, a settled civ clan of `newCityPop` 6+ people, settled `newCitySince` 5+ turns,
  3+ from every city and next to grain tamed at least `cityTame` 30%, founds a city where it stands. With a city site
  chosen (`setCitySite`, same checks plus spacing and tamed grain; lapses after `citySiteTurns` 40 or when it becomes
  invalid), daughters are steered there and the city rises once a settled civ clan is within `cityNear` 1; no other city
  is founded meanwhile. At most `maxCities` 12.
- **Decision:** a city has no population of its own. Civ people are its clans; a city is a work centre, a store cap,
  a garrison and territory.

### Good and bad years

Per region of `CIV.region` 8 × 8 tiles, each main-game turn: a `yearChance` 2% chance of a drought (× `drought` 0.5) or a
bumper year (× `bumper` 1.5), even odds, lasting 3–5 turns (`yearTurns`). Scales regrowth, gathered materials and clans'
material bring. Droughts in a human civ's regions raise an alert.

### Leaning and joining

- **Trade:** after each clearing, an unaligned clan at a market where a civ posted gains
  `tradeLean` 0.15 × min(1, goods it traded / its pop) × that civ's share of the market's volume.
- **Gifts** (`giftFood`, unaligned clans only): at most `giftMax` 10 food per clan per civ per turn. Leaning gained is
  `giftLean` 0.25 × hunger × w/(1+w), w = food given this turn per head; hunger is 2 starving, 1 hungry, 0.4 fed.
  **Decision:** the cap and the diminishing return make winning a clan take food over several turns, and favour hard years.
- Leaning decays `leanDecay` 3% a turn. A clan whose highest leaning reaches `joinAt` 1 joins that civ.
- AI civs gift `aiGift` 5 food a turn to the unaligned clan within `aiGiftRange` 6 of their cities that is starving or
  leans most toward them, while their food exceeds their reserve by 5.
- The civ panel lists clans leaning toward the player with the strongest rival; the map does not draw leanings.

### Discoveries

- Agriculture: every civ starts with domesticated grain (founding needs a hearth).
- **Pottery**: `practice.pottery` 20. A turn in which the civ's workers dug clay (a river among a city's work tiles) adds
  1; otherwise a turn holding `potteryClay` 1+ clay from any source (trade too) adds `heldPractice` 0.5. Effect: store
  and clan spoilage × 0.7, and crafting pottery (a trade good) from clay.
- **Herding**: `practice.herding` 25. A turn in which a city's hunter worked adds 1; otherwise a turn in which member
  clans harvested `herdGame` 2+ game adds 0.5. Effect: hunters bring × `herdBonus` 1.5 game and deplete herds half as
  fast.
- **Spread by contact:** each turn, +`spread` 0.5 practice per civ in trading contact that knows it (both post at, or have
  clans trading at, the same market).

### Victory

- **Conquest:** one civ owns every founding city (capitals).
- **Prosperity:** after `CIV.turnLimit` 150 main-game turns, the civ still holding a city with the highest people now /
  people at founding wins (ties: more people, then lower id). **Decision:** the design said "most people"; the richest
  hearth gets the most people at founding, so a head count rewards the start, not the play.
- A civ with no cities is "fallen" (`s.mods.civ.lost`); its clans become unaligned (in the `markets` phase) and can be
  won by anyone through leaning.
- **Decision:** capturing a city transfers the city, not its clans or its civ's founding market. The city takes the new
  owner's naming scheme (`watchCities`: "Red town 3" taken by Blue becomes "Blue town N", the first free number) and
  notes its old name.
