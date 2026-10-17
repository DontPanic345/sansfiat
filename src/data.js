export const FOODS = ['game', 'grain', 'fruit', 'fish', 'shellfish'];

// regen: share of a tile's cap that regrows each turn. Game breeds instead: regen scales
// with the herd left, so a hunted-out tile recovers slowly, plus `restock` wandering in.
export const FOOD = {
  game:      { color: '#c0703a', regen: 0.35, breeds: true, restock: 0.02 },
  grain:     { color: '#e3c95a', regen: 0.25 },
  fruit:     { color: '#c4487a', regen: 0.2 },
  fish:      { color: '#4aa3df', regen: 0.3 },
  shellfish: { color: '#9be0d0', regen: 0.2 },
};

// food: the cap of each food on a tile of this terrain.
export const TERRAIN = {
  ocean:    { color: '#2f5f8a', passable: false, moveCost: 1, food: {} },
  plains:   { color: '#a3c264', passable: true,  moveCost: 1, food: { game: 3, grain: 4 } },
  forest:   { color: '#3f7a3a', passable: true,  moveCost: 2, food: { game: 3, fruit: 4 } },
  hills:    { color: '#a58c5c', passable: true,  moveCost: 2, food: { game: 2, fruit: 1 } },
  desert:   { color: '#d9c58f', passable: true,  moveCost: 1, food: {} },
  mountain: { color: '#7d7d7d', passable: false, moveCost: 1, food: {} },
};

// Added on top of the terrain's food.
// shallows: ocean next to land.
export const FEATURES = {
  river: { fish: 5, grain: 1 },
  coast: { shellfish: 1 },
  shallows: { fish: 4, shellfish: 4 },
};

// yieldRange: every food cap on a tile is scaled by its fertility, drawn from this range in patches.
export const MAP = { width: 44, height: 28, rivers: 24, ridges: 5, passChance: 0.06, inlets: 5, inlandSeas: 2, desertShare: 0.2, minRegion: 30, yieldRange: [0.4, 2] };

// need = pop food per turn. A fully fed clan grows: from the first growth value for a clan
// that just moved to the second once it has stayed settleTurns. Fed below stableAt it shrinks;
// in between it holds steady. A clan forages its land plus the unclaimed tiles around it, and
// walks all of it each turn, so its land's total move cost is at most `roam`.
// A food counts in full up to dietShare of need, and at excessValue beyond that, so living on
// one food takes 2.4x the harvest.
export const CLAN = {
  seeds: 3,
  startPop: 6,
  harvestPerHead: 1.5,
  takeRate: 0.5,
  surplusTarget: 1.2,
  dietShare: 0.4,
  excessValue: 0.3,
  growth: [0.03, 0.07],
  settleTurns: 30,
  stableAt: 0.8,
  starve: 0.5,
  spoilage: 0.3,
  splitAt: 12,
  hungrySplit: 8,
  settleFed: 0.9,
  patience: 3,
  moveRange: 4,
  scoutRange: 6,
  roam: 6,
  landMargin: 0.02,
  choiceSpread: 0.05,
  moveMargin: 0.1,
  trail: 8,
};

export const NAME_SYLLABLES = ['ka', 'ru', 'mo', 'ta', 'ne', 'shi', 'ba', 'lo', 'vi', 'den', 'ur', 'ash', 'el', 'tor', 'mi', 'ga', 'son', 'ya', 'hu', 'ze', 'ro', 'an', 'ki', 'pa'];

// reach is in path cost (terrain moveCost). tripCost is the share of labour lost per unit of path cost.
// Pre-game markets are gatherings: one forms where minClans live within reach, and disperses
// when its traded volume, averaged with weight `fade` on the past, falls below minVolume.
export const MARKET = {
  formAfter: 20, spacing: 5, reach: 3, tripCost: 0.05, accessBonus: 0.2, history: 200,
  gathering: { minClans: 4, fade: 0.8, minVolume: 2 },
};

// Grain on land held tendAfter turns in a row gains `tame` each turn, and loses it when left.
// At tame 1 the tile is domesticated: grain cap x domestic. The game starts once domesticated
// hearths exist hearthSpacing apart, one per player.
export const TENDING = { tendAfter: 15, rate: 1 / 120, domestic: 2.5, hearthSpacing: 8 };

export const PREGAME = { minTurns: 100, maxTurns: 500 };

export const PLAYERS = [
  { name: 'Red',  color: '#d64545' },
  { name: 'Blue', color: '#3b6fd6' },
  { name: 'Gold', color: '#d6a63b' },
  { name: 'Teal', color: '#2fb3a5' },
];
