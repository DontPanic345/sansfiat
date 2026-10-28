# Build 2 (easel): the camp on the page, the sweep, archiving

Piece #29. Read `relay/specs/29/spec.md` (Settled 13 and 15) and build-1's summary: the camp,
its actions and `units::line` exist in the engine. Design: `docs/design/supply.md` "On the page".

Delivers:

- `web/src/lib.rs`: the view carries each camp (warriors, pack, turns of food left, hunger,
  switches, line path, cost and carry, cut off) and the civ's store; `act` takes the new actions.
- `www/`: selecting a camp draws its line home, coloured from green (carries a turn's food with
  little labour) through amber to red (cut off); the panel shows warriors, pack and hunger, with
  Raise, Disband, Resupply, Forage, Pillage buttons and the two switches; right-click moves the
  camp. The feed reports the new events. `check.mjs` checks the view's new fields; a `play.mjs`
  script (keep it in `www/`) selects a camp, moves it, and screenshots the line into
  `relay/specs/29/screens/`.
- `cli/src/main.rs` sweep: `--after N` runs past the pick with all on the AI and reports per
  civ what Settled 15 lists, one line per civ and a total "civs with a camp at the end: k of n";
  `--no-army` is the control; `--maps retro` is the seven maps (small 2, 3, 6, 9; medium 8, 9;
  large 8). Write a first run to `relay/specs/29/sweep.md` (army and control).
- `supply-lines/` archived as `hex4x/` and `hex-economy/` were (a README pointing at the
  commit); `engine/CLAUDE.md` updated for the camp, the actions and the sweep.
- `./engine/check.sh` passes.

Not yours: the numbers; the colour thresholds may be retuned by build-3.
