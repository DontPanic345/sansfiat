# Sans Fiat — pre-game

Economy prototype for a 4X with no treasury: goods are worth what clans will trade for them.
This covers the pre-game only. Hunter-gatherer clans spread, deplete land, split and trade at markets.
The busiest markets become the players' civs. Plain ES modules, no build step, no dependencies.
Design notes: `C:\Users\fallo\OldTown\🚧Projects\🌏SansFiat`.

```
npm start          # http://localhost:8000  (?seed=N&players=N)
npm test           # node --test
npm run sim -- --seed 3 --players 3 [--turns 400] [--every 25] [--json]
```

## Shape

- `src/data.js` — foods, terrain caps, `CLAN`, `MARKET`, `PREGAME` tuning. Tune here first.
- `src/rules.js` — all game logic. `dispatch(state, action) -> {state, error?}` is pure.
  - A turn runs the steps in `TURN` in order: regrow → forage (move, decide to trade, harvest) → trade → settle (eat, grow, split, die) → occupy → form markets → stats → game-start check.
  - Add an action: an entry in `ACTIONS` with `validate`, `apply`, `candidates`. Only `endTurn` exists so far.
  - Clans walk one tile a turn along a `route` and harvest as they go. A clan sets out only after `CLAN.patience` hungry turns, and a hungry clan big enough splits instead. A daughter splits off only if a site within `CLAN.scoutRange` would feed her; she starts next to her parent and walks there.
- `src/market.js` — `clear(baskets)`: searches for exchange ratios where supply meets demand, then rations. Each clan values a basket at Σ sqrt(amount), so trades are value-neutral and there is no numeraire.
- `src/game.js` — `newGame(config)`, `replay(config, actions)`, `runPregame(s, n)`.
- `src/mapgen.js` (terrain, inlets and inland seas, mountain ridges, deserts, rivers, coast/shallows, patchy fertility scaling food caps), `src/hex.js` (axial, pointy-top), `src/rng.js`.
- `src/render.js` (canvas) and `src/main.js` (controls, side panel) are the only DOM code. `src/views.js` holds the debug map views (keys 1–9).

## Invariants

- State is plain JSON. `apply` mutates only the clone `dispatch` hands it.
- All randomness goes through `rand(state)`, so a seed plus the action list replays exactly.
- UI never mutates state directly; it only dispatches actions.

In the browser console, `game.state`, `game.act({type: 'endTurn'})` and `game.history` are available.
