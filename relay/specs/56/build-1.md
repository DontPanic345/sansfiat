# Build 1 (magnum): armies hold ground

Read `engine/CLAUDE.md`, `docs/design/actors.md`, `docs/design/supply.md`, and this piece's
`spec.md`, whose **Settled** list is your rules. Build all of it: the zone and `Action::Zone`, the
spread, guarding as divided labour, held land and `civs::holder`, the line test, contested tiles,
events and counters, the AI's zone, the page, `www/play-hold.mjs`, tests, and archiving
`paint-army/`. `paint-army/index.html` (before you archive it) shows the painting and prune to port.

## What you tune

`UNIT.guard`, guarding's labour, the zone cap and the AI's zone length, by sweep and by play.
Add held and cut-by-army columns and a `--no-zone` control to `sweep --after`. Write the tables
and what you chose to `relay/specs/56/sweep.md`. Where a Settled rule can't be made to play, make
the smallest change that does and record it as a decision.

## Proof

- Unit tests: a zone must be connected and within its cap; the spread fills one ring a turn;
  moving clears it; a tile held by a rival blocks the line and one's own held tile on rival land
  doesn't; the contested-tile order; a cut camp eats its pack, then forages.
- Replay and `cross-build.sh` stay green; add a zone order to `lead_orders` and mirror it in
  `cross-build.mjs`. `check.mjs` covers the zone and held fields the page reads.
- `node www/play-hold.mjs` cuts a rival camp's line through the page and ends turns until the feed
  says it is cut and hungry; keep its screenshots in `relay/specs/56/screens/`.
- AI camps with zones don't starve more than the `--no-zone` control in the sweep.
- `./check.sh` passes.

The review relies on `play-hold.mjs`, `sweep.md`, and the counters.
