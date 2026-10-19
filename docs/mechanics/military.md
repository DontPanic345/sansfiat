# Mechanics: exploration, armies and supply

Exact rules of the proof of concept as built. Constants: `SCOUT` (`src/modules/scout/data.js`), `ARMY`
(`src/modules/army/data.js`), `SUPPLY` (`src/modules/supply/data.js`), `TRAILS` and `CITY` (`src/data.js`).
Every unit has `labour` refilled to `labourMax` (4 for all unit types) at the end of each turn. A move into a tile costs
its `stepCost`; a unit may enter a tile dearer than its labour only with full labour.

## Scouts, trails and fog

- **Scouts.** Each civ gets one at the start; `buildScout` raises another in a city, at most `SCOUT.maxPerCiv` 3.
  **Gap, decision:** scouts cost nothing to raise in the proof of concept.
- **Routes** (`assignRoute {unit, from, to, walk?}`). The scout plans the best path believing unrevealed tiles at their
  best case and the best at their worst case (`routePlan`). While the optimistic one is cheaper, it reveals unknown tiles
  on it (`revealLabour` 1 each): impassable unknowns first, then the largest uncertainty, then the nearest. Then it builds
  trails on the best known route (`TRAILS.work`: forest 2, hills 3, mountain 6 labour, spread over turns in
  `tile.trailWork`). Then it walks to `to`, unless `walk: false`. **Decision:** reveal, then trail, then walk.
  `cancelRoute` stops it. A route that cannot be reached ends with an alert.
- **Bridges.** A mountain opens only if its span ≤ `SCOUT.bridgeSpan` 1; wider spans stay closed.
- **Trails are shared.** `tile.trail` lowers `stepCost` for every civ, clan, army and porter, and widens market reach.
- **AI roads. Decision:** an AI civ keeps 1 scout (2 with two or more cities). Its first scout trails a road from its capital
  to each rival city, nearest first, then to its own cities (`walk: false`); finished or hopeless pairs go in
  `s.mods.scout.roads`. Other scouts explore the cheapest unseen passable tile, remembering the last 40 tried.
- **Fog.** `civ.seen` grows each turn around units (`sight.unit` 2), cities (3), clans (1) and all land the civ's clans hold.
  **Decision: opaque fog.** Unseen tiles draw near-black with a faint terrain hint; seen but not currently visible tiles
  are dimmed. Panels and debug views show only what the human civ has seen (`fogOf`, `viewValues`). Watch mode and the
  pre-game show everything.
- **Map memory.** `civ.revealed` (hidden trail costs known) is separate from `seen`. Each turn, for each market and each
  ordered pair of civs with clans trading there, there is a `shareChance` 5% chance that one tile the first has revealed
  passes to the second.

## Armies

- **Raising** (`raiseArmy {city}`): at most `ARMY.maxCaptains` 3 captains; the civ's clans must hold `size` + 5 = 15 people.
  `size` 10 people leave the largest clans (each keeps at least 1). **Decision:** the army is provisioned only from the
  store, up to `provision` 4 food per troop; nothing is free.
- **Captain** (`u.army = {troops, zone}`). `moveCaptain` one tile at stepCost labour. Undeployed (empty zone), all troops
  ride with him and fight on his tile. A hungry army cannot step into a fight.
- **Zone.** `paint` adds a tile next to the zone (the first paint must be next to the captain and adds his tile too, and
  deploys the army); `unpaint` removes one if the zone stays connected; removing the last tile undeploys. A deployed
  captain who steps out of the zone paints the new tile. `undeploy` gathers everything on him.
- **Spread** (`armies` phase). Target per tile = troops / zone size. A detachment on a zone tile above ⌈target⌉ splits the
  excess off toward the nearest under-filled tile; one at or below it stays and entrenches +1 (max `maxEntr` 3, +25% power
  each). Others step toward the nearest under-filled tile linked to the captain (moving resets entrenchment). Detachments
  on one tile merge.
- **Cut off.** A detachment not linked to its captain through the zone walks toward him; after `cutTurns` 8 it disbands into
  the nearest clan of its civ. The player may `promote` a cut-off detachment to captain while under the cap; it takes its
  connected part of the zone, any linked detachments there, and a troop share of the old army's food. An orphan (captain
  gone) walks to the nearest own city and disbands there, or after 8 turns.
- **Disband** (`disband`): within `CITY.territoryRadius` 2 of an own city, troops rejoin the nearest civ clan.
- **Skirmish.** On each tile with fighting units of two or more civs (detachments, undeployed captains with troops), plus a
  city's or fort's garrison when its owner has no troops there. Power = troops × (1 + 0.25 × entrenchment) ×
  `hungryPower` 0.5 if hungry × `cityDefence` 1.5 in an own city (× `wallMult` 1.5 with a wall). Garrisons fight as fully
  entrenched (× 1.75). Each side loses max(1, round(`dmg` 0.5 × enemy power)) troops, spread over its units; a garrison
  loses round(0.5 × enemy power / 1.75). A side outnumbered `retreatRatio` 3 to 1 has a `retreatChance` 60% chance to slip
  to a free neighbour toward its captain and lose nothing.
- **Holding.** `tile.held` = the civ with the most fighting troops on the tile. `territoryOf` puts held first, then forts,
  then the nearest city within 2.
- **Cities.** Each has `city.garrison` (`ARMY.garrison` 3), which regrows 1 a turn while nobody holds the tile. A city falls
  when an enemy holds its tile and its garrison is 0: `transferCity`, garrison reset to 3. Its clans keep their civ.
- **Forts** (`buildFort`): 2 labour, 2 wood, 2 stone, own troops on the tile. Garrison 3; own detachments on it are fully
  entrenched; taken when its garrison falls (new owner, garrison 3).
- **Fight cost.** Fighting records `foughtTurn`/`foughtTroops` on the captain; supply charges `fightEat` for it.
- **AI.** Raises when its clans hold `aiRaisePop` 30 people, `aiRaiseGap` 20 turns after its last raising, with 10 food in
  store or 20 wild food near the city, and a rival city within `aiReach` 40 path cost. It always marches undeployed;
  defends a city with enemy troops within 2; else marches on the cheapest rival city it beats by `aiOdds` 1.2 (counting
  friends within 3), spending at most `aiMoveLabour` 2 labour a turn on walking; a starving army goes home and disbands.
  It promotes only orphans with 3+ troops.

## Supply

- **Army food** (`u.supply = {food, foodMax, auto}` on the captain; `foodMax` = troops × `carryPerTroop` 4). After the
  fights each army eats `eatPerTroop` 0.25 per troop + `fightEat` 0.25 per troop that fought. Short: food goes to 0, the
  army is hungry (cannot attack, defends at half) and loses `attrition` 10% of its whole troops (at least 1), spread over
  its detachments. **Decision:** attrition is on the army total, not per detachment, so a spread-out army does not starve
  faster tile by tile.
- **Supply line.** Source: the cheapest own city (while the store holds food) or own cache within `range` 10 path cost,
  through tiles that are not enemy territory (troop-held tiles count). Ties go to the city. `resupply` costs
  1 + ⌊cost / `costPerLabour` 3⌋ labour and fills the captain. `autoSupply` does it each turn in the `armies` phase.
  The army panel draws the line.
- **Forage and pillage. Decision:** decided by `landOwner` (a fort, else the nearest city within 2, else the civ of the clan
  claiming the tile), not by who stands on it. `forage` (2 labour) on own or neutral land, `pillage` (1 labour) on enemy
  land, take up to `gather` 4 food; the tile then yields nothing for `regrowTurns` 5 turns, to anyone.
- **Porters** (`buildPorter`, 1 food): carry `porterCarry` 12, eat `porterEat` 1 a turn while loaded, do not fight or hold
  ground. `loadPorter` in an own city; `unloadPorter` feeds own captains on the tile first, then drops a cache (not on
  enemy land or another civ's cache). `setPorterRoute {city, to, target}` is a standing order: load at the city while the
  cache is under `target` (default 24) or a captain waits there, walk, unload, walk back. `clearPorterRoute` ends it.
  **Decision:** porters refuse routes into enemy land, and turn back (food returned to the store as grain) when the target
  becomes enemy land without an own captain on it, an unload is refused, or the home city is lost.
- **Caches** (`tile.cache = {owner, food}`): spoil `cacheSpoil` 30% a turn; vanish under 0.1 food unless a porter stands
  there; change owner when the tile's territory does.
- **AI.** Every captain on auto-supply; forages or pillages below `aiForageBelow` 60% food; when no source is within
  `farFromHome` 8 and the store holds 12 food, routes a porter (building one per captain at most) to a cache site: the
  army's tile, or the last non-enemy tile on the way to it. Porters of vanished armies are released.
- **Not built:** auto-forage for the player and a labour reserve for automation; goods (not food) moving along lines;
  attacks on porters or caches other than by taking the tile; pack animals; healing (`SUPPLY.heal` reads a `troopsMax`
  nothing sets).
