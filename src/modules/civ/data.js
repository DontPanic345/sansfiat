// Tuning for the civ module. Import only from src/data.js here.
export const CIV = {
  labourShare: 0.2,      // share of a civ clan's people working for the civ each turn
  workRadius: 2,         // tiles around a city worked for materials
  tileMax: 1.5,          // materials per matching tile per turn
  matTarget: 20,         // civ labour stops gathering a material at this stock
  reserve: { food: 10, material: 8, craft: 4 }, // kept back; the surplus is posted
  baseCap: 40,           // store cap per good per city; a storehouse adds storehouseCap
  storehouseCap: 40,
  granarySpoil: 0.3,     // spoilage multiplier with a granary
  projects: {
    granary:    { labour: 10, goods: { wood: 4, clay: 2 } },
    storehouse: { labour: 12, goods: { wood: 6, stone: 2 } },
    wall:       { labour: 15, goods: { stone: 6, wood: 2 } },
  },
  newCityPop: 6, newCitySince: 5, maxCities: 12,
  region: 8, yearChance: 0.02, yearTurns: [3, 5], drought: 0.5, bumper: 1.5,
  tradeLean: 0.03, giftLean: 0.04, joinAt: 1,
  practice: { pottery: 15, herding: 20 }, spread: 0.5, spreadRange: 12,
  hunt: 2, herdBonus: 1.5, craft: 2,
  turnLimit: 150,        // main-game turns until prosperity decides
};
