// Tuning for the supply module.
export const SUPPLY = {
  range: 10,
  costPerLabour: 3,
  carryPerTroop: 4, // captain's foodMax = troops * this
  eatPerTroop: 0.25, // food eaten per troop per turn
  fightEat: 0.25, // extra food per troop that fought this turn
  attrition: 0.1, // share of the army's troops lost per hungry turn (min 1), spread over its detachments
  heal: 1, // troops regained per fed turn, up to troopsMax if known
  forageLabour: 2,
  pillageLabour: 1,
  gather: 4, // food taken from a tile
  regrowTurns: 5,
  cacheSpoil: 0.3,
  porterCarry: 12,
  porterEat: 1,
  porterCost: 1, // food from the store to raise a porter
  farFromHome: 8, // AI wants a cache when an army is farther (line cost) than this from every city
  aiForageBelow: 0.6, // AI forages/pillages when its food is below this share of foodMax
};
