# hex4x

Minimal turn-based hex 4X used as a prototyping base. Any part may be replaced.
Plain ES modules, no build step, no dependencies.

```
npm start          # http://localhost:8000  (?seed=N&players=N&ai=1,2 — ai= for hotseat)
npm test           # node --test
npm run sim -- --seed 3 --players 3 --turns 100 [--json]
```

## Shape

- `src/data.js` — terrain, unit types, players, city constants. Tune here first.
- `src/rules.js` — all game logic. `dispatch(state, action) -> {state, error?}` is pure.
  - Add an action: an entry in `ACTIONS` with `validate`, `apply`, `candidates`.
  - Supply (`allocateSupply`, tuned by `SUPPLY` in data.js): units with `upkeep` draw from the nearest city with spare supply via a line through friendly/neutral territory; line length eats labor (`movesLeft`); a city's leftover supply feeds production.
  - Add per-turn behaviour: a function in `TURN_START`.
- `src/game.js` — `newGame(config)`, `replay(config, actions)`.
- `src/mapgen.js`, `src/hex.js` (axial coords, pointy-top), `src/rng.js`.
- `src/ai.js` — trivial policy; drives `sim` and the non-human players.
- `src/render.js` (canvas) and `src/main.js` (input, side panel) are the only DOM code.

## Invariants

- State is plain JSON. `apply` mutates only the clone `dispatch` hands it.
- All randomness goes through `rand(state)`, so a seed plus the action list replays exactly.
- UI never mutates state directly; it only dispatches actions.

In the browser console, `game.state`, `game.act({type: 'endTurn'})` and `game.history` are available.
