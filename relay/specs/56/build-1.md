# Build 1 (magnum): armies hold ground

Read `engine/CLAUDE.md`, `docs/design/actors.md`, `docs/design/supply.md`, and this piece's
`spec.md`, whose **Settled** list is your rules. Build all of it: the zone and `Action::Zone`, the
spread, guarding as divided labour, held land and `civs::holder`, the line test, contested tiles,
events and counters, the AI's zone, the page, `www/play-hold.mjs`, tests, and archiving
`paint-army/`. `paint-army/index.html` (before you archive it) shows the painting and prune to port.

## What you tune

`UNIT.guard`, guarding's labour, the zone cap and the AI's zone length, by sweep and by play.
Add held and cut-by-army columns and a `--no-zone` control to `sweep --after`. AIs never paint to
cut, so the sweep's cut-by-army stays 0: it measures what guarding costs, and the scripted cut
shows what it does. Write the tables
and what you chose to `relay/specs/56/sweep.md`. Where a Settled rule can't be made to play, make
the smallest change that does and record it as a decision.

## Proof

- Unit tests: a zone must be connected, within its cap and clear of capitals; the camp's own tile
  is held; the spread fills one ring a turn; moving clears it; a tile held by a rival blocks the
  line and one's own held tile on rival land doesn't, nor do the line's ends; the larger guard
  share takes a contested tile and a tie stays with the holder; a camp on rival land still
  pillages; a cut camp eats its pack, then forages.
- Replay and `cross-build.sh` stay green; add a zone order to `lead_orders` and mirror it in
  `cross-build.mjs`. `check.mjs` covers the zone and held fields the page reads.
- `node www/play-hold.mjs` cuts a rival camp's line through the page and ends turns until the feed
  says it is cut and hungry; keep its screenshots in `relay/specs/56/screens/`. It may raise
  warriors first and pick the seed and the rival: the AI guards its line, so the cut needs a
  thicker guard than the AI's on a ring of tiles the line can't get round.
- AI camps with zones don't starve more than the `--no-zone` control in the sweep.
- `./check.sh` passes.

The 90 minutes are tight for one session: get the engine, its tests and `play-hold.mjs` green
before tuning, and keep sweeps few (`sweep --after 200 --maps retro` takes a few minutes).

The review relies on `play-hold.mjs`, `sweep.md`, and the counters.
