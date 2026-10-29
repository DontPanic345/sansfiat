# Build 1 (easel): the camp in the engine

Piece #29. Read `relay/specs/29/spec.md` first: its "Settled" list is your spec, items 1–14
and 16, and from 15 the engine's `Config.army` switch for the control. Design: `docs/design/supply.md`, `founding.md`, `actors.md`. Code: `core/src/units.rs`
(rewrite it around the camp), `civs.rs` (founding, levy), `actors.rs`, `markets.rs` (the
capital's market, routes), `land.rs` (regrow and fallow), `clans.rs` (harvest skipping fallow
tiles; the civ's clan pinned), `world.rs` (actions, events, the AI step), `data.rs`,
`cli/src/main.rs` `lead_orders` and `engine/cross-build.mjs`, `web/src/lib.rs` (keep it
compiling; build-2 does the page).

Delivers:

- Everything in the spec's Settled 1–14, 16, and `Config.army` (off: no camp at founding, the
  AI raises none, no levy), with numbers in `data.rs` (`CIV`, `UNIT`). Start
  from: levy 0.3, `UNIT.start` 3, pack 5, carry 1.0, forage 0.5, pillage 1.0, fallow 5. These
  are build-3's to tune: make them easy to change, and don't tune.
- Functions build-2 and build-3 rely on, in `units.rs`: the camp's `line(w, unit)` ->
  `Option<(Vec<usize>, cost, carry_per_turn)>` (carry with all its labour), `cut_off(w, unit)`,
  and per-civ counters the sweep reads, kept on the civ so they outlive a camp (turns hungry,
  turns cut off, food foraged, pillaged, resupplied, warriors starved, camps lost, camps
  raised).
- Tests under `core/tests/` (replace `units.rs`'s, and fix `civs.rs` and `events.rs` tests that
  assume the old founding, levy or warriors): founding makes the biggest near clan the civ
  with the market on its home; the civ's clan stays on the capital through dividing; the levy
  only from clans walking to the capital, out of the harvest before growth (a levied clan
  grows less than the same clan unlevied); raising drops
  `n` clans by one each; disbanding returns to the civ's clan; a camp at the capital fills with
  no labour; a camp of 6 marches as far in a turn as a camp of 2; a far camp's line costs more and carries less; a line through rival land is
  refused; foraging strips a tile so a clan harvests nothing there for `fallow` turns; pillage
  yields more than forage; switches off means no foraging or supply; hunger loses warriors; `Config.army` off founds no camp and
  levies nothing;
  replay rebuilds the world.
- `./engine/check.sh` passes, including `cross-build.sh` with the new `lead_orders`.

Not yours: the page, the sweep, the numbers.
