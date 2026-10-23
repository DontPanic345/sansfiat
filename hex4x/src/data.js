export const TERRAIN = {
  ocean:    { color: '#2f5f8a', passable: false, moveCost: 1, food: 1, prod: 0, defense: 0 },
  plains:   { color: '#a3c264', passable: true,  moveCost: 1, food: 2, prod: 1, defense: 0 },
  forest:   { color: '#3f7a3a', passable: true,  moveCost: 2, food: 1, prod: 2, defense: 0.25 },
  hills:    { color: '#a58c5c', passable: true,  moveCost: 2, food: 0, prod: 2, defense: 0.5 },
  mountain: { color: '#7d7d7d', passable: false, moveCost: 1, food: 0, prod: 1, defense: 0 },
};

export const UNITS = {
  settler: { glyph: 'S', moves: 2, strength: 0,  cost: 30, canFound: true },
  warrior: { glyph: 'W', moves: 3, strength: 10, cost: 15, upkeep: 1 },
};

export const PLAYERS = [
  { name: 'Red',  color: '#d64545' },
  { name: 'Blue', color: '#3b6fd6' },
  { name: 'Gold', color: '#d6a63b' },
  { name: 'Teal', color: '#2fb3a5' },
];

export const CITY = { workRadius: 1, minSpacing: 3, defenseBonus: 0.5, defaultBuild: 'warrior', territoryRadius: 2 };

// Units with `upkeep` draw it from a city each turn; each `tilesPerLabor` of supply line costs 1 labor.
export const SUPPLY = { range: 8, tilesPerLabor: 2, unsuppliedLabor: 1 };
