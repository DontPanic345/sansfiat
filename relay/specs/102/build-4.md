# #102 build-4 (canvas): does claiming pay, and does the lead move?

The easel is built: festivals and trails tuned (build-1), the claim, the guard, land, leaning and
leaving on starting numbers (build-2), and the judge, view and page reading them (build-3). You find
the numbers, and where the runs show a rule wrong, the rule, so that on the retro maps a civ that
claims and guards gains Standing over one that doesn't and the lead changes hands after the pick,
with health held. If it can't be done within the design, you show that with numbers: that answers the
question no, and it is a result.

Read `spec.md` (the whole of it, and "Left unsettled" closely), `probe.md`, `main-judge.txt`, and the
summaries and judge files of builds 1–3.

## What you tune

`CLAIM.floor`, `GUARD.guarding`, `GUARD.start`, `GUARD.keep_turns`, `LEANING.land`, `LEANING.guarded`,
`LEANING.leave_run`, `CLAIM.ai_ring`, `CLAIM.ai_reach`, and the AI's guard rule (spec items 11, 12, 16,
19, 22, 24). Not the festival and trail numbers (build-1's targets must still hold), nor the judge's.
You may change an easel rule the runs show wrong (the stand-in's shape, how the guard feeds, how a
civ clan's own leaning is kept up, the leaving test), saying so as a decision with the evidence.
Every number stays in `data.rs` with a one-line note of why it is what it is.

## What proves it (spec, Left unsettled)

From `cargo run --release -p sansfiat -- sweep --judge --solo`:

- `solo_gain` above +0.005 for at least 14 of 21 civs, the mean positive;
- lead changes on at least 4 of 7 maps in the claims run, and more than in the no-claims control;
- pre-games by centres 7 of 7, panics 0, digest mismatches 0, people against `--no-army` at least 0.95,
  no civ with no clans at +200, no civ losing half its clans by +200 on a map, and build-1's festival
  and trail targets still met.

And by play, through `sansfiat play` on small seed 2 (or another retro map, saying why): two games of
100 turns from the pick, one leading a civ that claims everything within 6 steps with the starting
guard, one that keeps the stand-in claim and raises its guard as its store allows. The wide one should
hold less land at +100: a wide claim is thin. `sansfiat judge` on each. Then
`node www/play-claim.mjs relay/specs/102/screens` again on the final numbers, and look at the shots.

## Record

- `relay/specs/102/canvas.md`: what you tried (a table: the numbers, solo gain mean and count, lead
  changes and maps, health), what you chose and why, the two played games (their notes and the judge's
  score lines), and what the page shots show.
- `relay/specs/102/canvas-judge.txt` and `.json`: the final `sweep --judge --solo`.
- The PR gets old and new numbers from these: put the answer to the question in your summary in one
  sentence with the numbers.

`./check.sh` passes at the end. Budget the sweeps: one `--solo` run is a few minutes; a variant that
only needs the claims run can skip `--solo` until it looks right.
