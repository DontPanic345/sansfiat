// Tuning for the civ module. Import only from src/data.js here.
export const CIV = {
  // Labour. Each turn a civ drafts workers from its clans, as many as its cities' work needs
  // (a project, crafting, hunting, each material below matTarget), at most draftMax of a clan's
  // people. Starving clans are never drafted. One worker gives labourPerWorker labour.
  draftMax: 0.25,
  workers: { project: 3, craft: 1, hunt: 1, farm: 2, material: 1 },
  labourPerWorker: 1,
  matPerWorker: 1.5,     // materials one worker gathers in a turn
  tileMax: 1.5,          // ...but at most this per matching tile
  matTarget: 12,         // the civ stops gathering a material at this stock
  farmPerWorker: 1.5,    // food one farm worker takes from the city's tiles (while food < foodTarget x cap)
  foodTarget: 0.5,
  huntPerWorker: 1.5,    // game one hunter brings in (x herdBonus with herding)
  craft: 2,              // pottery one crafter makes from as much clay
  // Food income: fed member clans hand this share of their leftover store to the civ.
  levy: 0.5,
  // Store. The whole store (all goods) holds baseCap per city plus storehouseCap per storehouse;
  // past that, goods are lost. Default posting keeps keep[kind] x cap of each good and posts the rest.
  baseCap: 50,
  storehouseCap: 80,
  keep: { food: 0.08, craft: 0.05 }, // materials: default keep is what unbuilt projects need
  // Spoilage of food (CLAN.spoilage a turn) is multiplied by granarySpoil with a granary (the civ
  // store, and member clans living within granaryReach of a granary city) and by potterySpoil
  // once the civ knows pottery (its store and all its clans).
  granarySpoil: 0.3,
  granaryReach: 3,
  potterySpoil: 0.7,
  // Clans trading at a market where a civ posts bring a little of what their land yields:
  // clanMat of wood / stone / clay per forest / hills / river tile they hold.
  clanMat: 0.3,
  tradeRange: 12,        // a civ also posts at one other market this close to its own (see tradeMarkets)
  projects: {
    granary:    { labour: 10, goods: { wood: 4, clay: 2 } },
    storehouse: { labour: 12, goods: { wood: 6, stone: 2 } },
    wall:       { labour: 15, goods: { stone: 6, wood: 2 } },
  },
  // Cities: a settled civ clan of newCityPop, settled newCitySince turns, founds one where
  // grain within CITY.workRadius is tamed at least cityTame. A chosen daughter site lasts
  // siteTurns, a chosen city site citySiteTurns; a city site needs a civ clan within cityNear.
  newCityPop: 6, newCitySince: 5, maxCities: 12, cityTame: 0.3, cityNear: 1,
  siteTurns: 20, citySiteTurns: 40,
  // Good and bad years per region (region x region tiles).
  region: 8, yearChance: 0.02, yearTurns: [3, 5], drought: 0.5, bumper: 1.5,
  // Leaning. Trade: tradeLean x (goods the clan traded / its pop, at most 1) x the civ's share
  // of the market's trade. Gifts: giftLean x hunger x w / (1 + w), w = food given this turn per
  // head; at most giftMax food per clan per civ per turn. Leaning decays by leanDecay a turn.
  tradeLean: 0.15, leanDecay: 0.03, joinAt: 1, leanNoteStep: 0.25,
  giftLean: 0.25, giftMax: 10, giftHunger: { starving: 2, hungry: 1, fed: 0.4 },
  aiGiftRange: 6, aiGift: 5,
  // Discoveries: practice turns to learn; spread adds practice a turn per trading contact who knows it.
  // Digging clay (a river in the city's work tiles) or hunting near a city is a full turn of
  // practice; holding potteryClay clay from any source (trade too), or member clans harvesting
  // herdGame game a turn, is heldPractice. The civ keeps keepMat of a material back from market.
  practice: { pottery: 20, herding: 25 }, spread: 0.5,
  heldPractice: 0.5, potteryClay: 1, herdGame: 2, keepMat: { clay: 4 },
  herdBonus: 1.5,
  turnLimit: 150,        // main-game turns until prosperity decides
};
