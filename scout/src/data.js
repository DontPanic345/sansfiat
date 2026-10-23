export const TERRAIN = {
  ocean:    { color: '#2f5f8a', passable: false, moveCost: 1, food: 1, prod: 0, defense: 0 },
  plains:   { color: '#a3c264', passable: true,  moveCost: 1, food: 2, prod: 1, defense: 0 },
  forest:   { color: '#3f7a3a', passable: true,  moveCost: 2, food: 1, prod: 2, defense: 0.25 },
  hills:    { color: '#a58c5c', passable: true,  moveCost: 2, food: 0, prod: 2, defense: 0.5 },
  mountain: { color: '#7d7d7d', passable: false, moveCost: 1, food: 0, prod: 1, defense: 0 },
};

export const UNITS = {
  settler: { glyph: 'S', moves: 2, strength: 0,  cost: 90, canFound: true },
  warrior: { glyph: 'W', moves: 3, strength: 10, cost: 15, upkeep: 1 },
  scout:   { glyph: 'R', moves: 3, strength: 0,  cost: 20, canScout: true },
};

export const PLAYERS = [
  { name: 'Red',  color: '#d64545' },
  { name: 'Blue', color: '#3b6fd6' },
  { name: 'Gold', color: '#d6a63b' },
  { name: 'Teal', color: '#2fb3a5' },
];

// Ridges are wandering mountain chains with hilly flanks; `gap` is the chance a ridge tile is hills.
export const MAP = { ridges: 3, ridgeLength: [6, 14], gap: 0.06, flankHills: 0.4 };

export const CITY = { workRadius: 1, minSpacing: 3, defenseBonus: 0.5, defaultBuild: 'warrior', territoryRadius: 2 };

// Units with `upkeep` draw it from a city each turn; each `tilesPerLabor` of supply line costs 1 labor.
export const SUPPLY = { range: 8, tilesPerLabor: 2, unsuppliedLabor: 1 };

// Hidden per tile, rolled at mapgen as [value, weight] pairs: the cost a Trail brings the tile to,
// and for mountains the bridge span needed (null: never passable). `work` is the labor to build it.
export const TRAILS = {
  forest:   { work: 2, cost: [[1, 1], [2, 1]] },
  hills:    { work: 3, cost: [[1, 1], [2, 2]] },
  mountain: { work: 6, cost: [[3, 1]], span: [[1, 1], [2, 1], [null, 8]] },
};

export const SCOUT = { revealLabor: 1, bridgeSpan: 1 };
