// Tuning for the army module (ported from the paint-army prototype).
export const ARMY = {
  maxEntr: 3, entrBonus: 0.25, dmg: 0.5, retreatRatio: 3, retreatChance: 0.6,
  size: 10, garrison: 4, wallMult: 1.5, maxCaptains: 3,
  // Defenders in their own city fight at x(1 + CITY.defence) (x wallMult with a wall). Troops dug
  // in (entrenched at least one turn) and garrisons also get x(1 + TERRAIN[terrain].defence).
  hungryPower: 0.5,     // a hungry army defends at this share of its power (and cannot attack)
  provision: 4,         // food per troop a newly raised army draws from the civ store (if there)
  cutTurns: 8,          // a cut-off detachment that cannot promote disbands after this many turns
  fort: { labour: 2, wood: 2, stone: 2 },
  // City garrison: garrison, plus one per garrisonPer people of the owner's clans within the
  // city's territory (at most garrisonPop more), plus wallGarrison with a wall. It regrows one a
  // turn while no enemy holds the tile. Each garrison troop fights at garrisonPower (x wallMult
  // with a wall) and a siege kills at most siegeLoss of it a turn, so a city takes a few turns.
  warnRange: 3,         // the human is alerted when enemy troops come this close to a city
  garrisonPer: 12, garrisonPop: 3, wallGarrison: 2, garrisonPower: 1.1, siegeLoss: 2,
  // AI forts: a captain builds one on a tile near home (within aiFortHome of an own city) that
  // faces a rival city (within aiFortRival), no closer than aiFortGap to another fort, at most
  // aiForts per civ, when the store holds the cost plus aiFortSpare of each material.
  aiFortHome: 4, aiFortRival: 10, aiFortGap: 4, aiForts: 2, aiFortSpare: 2,
  // AI
  aiRaisePop: 30,       // civ clans must hold this many people before the AI raises an army
  aiRaiseGap: 20,       // turns between AI raisings
  aiMoveLabour: 2,      // labour an AI captain spends walking per turn (the rest feeds it)
  aiReach: 40,          // path cost limit for AI targets
  aiOdds: 1.2,          // attack a city only with this much more power than its defence
};
