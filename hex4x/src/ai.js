import { legalActions } from './rules.js';
import { pick } from './rng.js';

// Picks one action for the current player. Uses its own RNG object so AI choices
// don't disturb the game's RNG stream.
export function randomPolicy(s, rng = { rng: s.turn * 7919 + s.current }) {
  const actions = legalActions(s);
  const of = (type) => actions.filter((a) => a.type === type);
  if (of('foundCity').length) return of('foundCity')[0];
  const moves = of('move');
  if (moves.length) return pick(rng, moves);
  return { type: 'endTurn' };
}

// Plays the current player's whole turn, returning the state after endTurn.
export function playTurn(s, dispatch, policy = randomPolicy) {
  const rng = { rng: s.turn * 7919 + s.current };
  const player = s.current;
  while (s.current === player && s.winner == null) {
    const { state, error } = dispatch(s, policy(s, rng));
    if (error) throw new Error(`AI chose illegal action: ${error}`);
    s = state;
  }
  return s;
}
