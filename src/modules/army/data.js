// Tuning for the army module (ported from the paint-army prototype).
export const ARMY = {
  maxEntr: 3, entrBonus: 0.25, dmg: 0.5, retreatRatio: 3, retreatChance: 0.6,
  size: 10, garrison: 3, wallMult: 1.5, maxCaptains: 3,
  cityDefence: 1.5,     // defenders in their own city fight at x1.5 (x wallMult with a wall)
  hungryPower: 0.5,     // a hungry army defends at this share of its power (and cannot attack)
  provision: 4,         // food per troop a newly raised army draws from the civ store (if there)
  cutTurns: 8,          // a cut-off detachment that cannot promote disbands after this many turns
  fort: { labour: 2, wood: 2, stone: 2 },
  // AI
  aiRaisePop: 30,       // civ clans must hold this many people before the AI raises an army
  aiRaiseGap: 20,       // turns between AI raisings
  aiMoveLabour: 2,      // labour an AI captain spends walking per turn (the rest feeds it)
  aiReach: 40,          // path cost limit for AI targets
  aiOdds: 1.2,          // attack a city only with this much more power than its defence
};
