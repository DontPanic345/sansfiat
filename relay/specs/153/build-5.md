# #153 build-5 (canvas): a scout that lives, and the flat trail on the finished branch

The easels built the piece. Flat trails are at `TRAIL.flat` 0.75 (build-1, `flat.md`). The
search orders mountains by best case and the scout has `Move` (build-2, `search.md`). The
feeding forages only a tile's spare and never fallows; a scout running low wanders to rich
ground, and an idle one waits there; rich ground is judged by what a turn's foraging there brings after the walks; the numbers are
`SCOUT.wander`, `low`, `fed`, `rich`, `reach` and `people` (build-3, `feeding.md`). The page shows all of it (build-4, `page.md`). Read
`spec.md` (all of it: the question, the unsettled items, the decisions), `probe.md` (the spec
step's runs), `review-probe.md` (spec-review's rough build of the feeding on the retro maps:
your starting point), the four build records, `core/src/units/scout.rs`, `core/src/units/forage.rs`
(`graze_at`, `supply`), `core/src/land.rs` (`spare`, `yield_of`), `core/src/data.rs`
(`ScoutRules`, `UNIT.pack`), `core/tests/scout_errand.rs`.

This step holds the piece's unsettled work. You may change the easels where the painting
needs it (say so in your summary), but not the feeding rule's shape that the piece decided:
the scout takes only a tile's spare and never fallows it, a hungry scout wanders and comes
back, and an idle scout waits on rich ground. Armies' numbers don't move.

## 1. The scout lives through its errand

Tune the feeding until `scout_errand` passes with no `#[ignore]`: all 28 scouts alive at 50
turns on the seven retro maps (each routed to the nearest rival capital, routed to `Lead`'s
target, moved to `Lead`'s target, and idle), with no refill. The levers, in the order to try
them: `SCOUT.wander` (how far it looks for rich ground), `SCOUT.rich` (how much more than its
eating rich ground must bring), `SCOUT.low` and `SCOUT.fed` (when it leaves and when it counts
as fed), whether the errand's own foraging fills the pack or only covers the turn (the probe:
fewer turns wandering, no more work done), `SCOUT.reach` (the round it forages), then `SCOUT.people`
(1 or 2) or a scout's own pack size (a `SCOUT.pack` in turns, read by the scout's `pack_cap`
in place of `UNIT.pack`; the army's pack doesn't move). `probe.md` found one person alone
loses 12 of 14 under today's rule; under the new rule it may differ, so measure it rather than
assume.

For each setting you try, record a line in `relay/specs/153/errand.md`: the numbers, scouts
alive of 28, people lost, hungry scout-turns, scout-turns wandering on the routed errands, and
tiles revealed and trails built on them. **What proves it:** all 28 alive. Among the settings
that pass, take the one whose routed errands reveal the most tiles and build the most trails:
those are what a scout is for. Give that setting's work beside the refilled control's (the
`#[ignore]`d report build-3 wrote: the same errands with the pack topped up each turn). The
probe's starting point did a fifth of the control's work (32 tiles against 152). Living is the
piece's bound; the work is reported. If the setting you take does under a quarter of the
control's, add an `ask` idea saying so, with the numbers: past the line's reach the scout's
labour goes on its food, and D's supply ring or a cheaper scout forage is the owner's call.

**The AI's scouts.** Count the AI's scouts lost by +200 on the seven retro maps, all on the
AI (a `#[ignore]` report in `scout_errand.rs`, run with `--ignored`, is the place). It must be
none. Put the count in `errand.md`.

If no setting keeps all 28 alive within an hour of the session, keep the best, say which
errands die and why in a paragraph (where it stood, what its ground spared, what its line
carried), and leave the test ignored with that reason. That is the piece's answer, and D's
supply ring is the next lever.

## 2. The flat trail on the finished branch

Run `sansfiat sweep --judge --json --floor` on main (a worktree at the branch's base) and on
the branch, then `judge --compare`. **What proves it:** the trading share at the pick and at
+200, and the aligned share at +200, rise past the floor; `pregame_by_centres` is 7 of 7; the
AI's orders per 100 turns and land changing hands are above half of main's. Run the pre-game
sweep (`sansfiat sweep --size all`) too: every size ends as on main. Keep 0.75 if all hold.
If one fails, try 0.85 and 0.65 and take the value nearest 0.75 that holds them all. Record
every run's totals (standing, doubt, decisions, motion, land, health) in
`relay/specs/153/trail.md` beside `probe.md`'s table. Report lead changes and spread; they are
not a bound.

## Record

`errand.md` (the settings, the 28-errand table for the one taken, the search's progress, the
AI's scouts) and `trail.md` (the sweeps and the value kept). If a number moved, `SAVE_VERSION`
bumps and the pins move. `engine/check.sh` green.

## Done

`scout_errand` runs in `cargo test` and passes, or stays ignored with the reason written.
`trail.md` gives the old and new totals the review copies into the PR. Each tuned number has
its reason as its `note` in `data.rs`.
