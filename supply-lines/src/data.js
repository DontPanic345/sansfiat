export const TERRAIN = {
  ocean:    { color: '#2f5f8a', passable: false, moveCost: 1, food: 1, prod: 0, defense: 0 },
  plains:   { color: '#a3c264', passable: true,  moveCost: 1, food: 2, prod: 1, defense: 0 },
  forest:   { color: '#3f7a3a', passable: true,  moveCost: 2, food: 1, prod: 2, defense: 0.25 },
  hills:    { color: '#a58c5c', passable: true,  moveCost: 3, food: 0, prod: 2, defense: 0.5 },
  mountain: { color: '#7d7d7d', passable: false, moveCost: 1, food: 0, prod: 1, defense: 0 },
};

// labor: pool refreshed each turn, spent on moving, fighting, resupplying, foraging, pillaging.
// food: store capacity for units with upkeep; they eat `upkeep` each turn and `SUPPLY.fightFood` per fight.
export const UNITS = {
  settler: { glyph: 'S', labor: 2, hp: 10, strength: 0,  cost: 30, canFound: true },
  warrior: { glyph: 'W', labor: 4, hp: 10, strength: 10, cost: 15, upkeep: 1, food: 4 },
};

export const PLAYERS = [
  { name: 'Red',  color: '#d64545' },
  { name: 'Blue', color: '#3b6fd6' },
  { name: 'Gold', color: '#d6a63b' },
  { name: 'Teal', color: '#2fb3a5' },
];

export const CITY = { workRadius: 1, minSpacing: 3, defenseBonus: 0.5, defaultBuild: 'warrior', territoryRadius: 2 };

// Supply-line length is summed terrain moveCost, capped at `range`. A resupply costs
// 1 labor plus 1 per `costPerLabor` of line. Foraged or pillaged tiles yield nothing for `regrowTurns`.
export const SUPPLY = {
  range: 10, costPerLabor: 3,
  fightLabor: 2, fightFood: 1,
  forageLabor: 2, pillageLabor: 1, regrowTurns: 5,
  attritionHp: 3, healHp: 1,
};
