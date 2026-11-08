# #155 build-4 (canvas): fields that pay

Tune the field rules on the retro maps until a civ that farms gains Standing over one that doesn't, its store
holds an army's packs, settled farmers stop moving, and nobody starves who shouldn't. Read `spec.md` in full:
its table holds every bound, and its "Unsettled" says what you may change. `probe.md` is the spec step's run of
a sketch of these rules. `judge.md` (build-2) is the branch's starting point, in the judge's own counts.

What the easels give you:
- build-1's field job: `core/src/fields.rs`, and `FIELD` in `core/src/data.rs`: `flood`, `river`, `grain`,
  `notch`, `margin`, `farmer` and `tame`;
- build-2's `fields` group and the no-fields control: `sansfiat sweep --judge --json --floor` prints every
  bound, pass or miss, and `total.fields_vs_control` in the JSON;
- build-3's page, which reads the rules through the view and needs no change unless a rule's shape does.

You may change the easels where a bound needs it. Examples: the home rule's target (the field days' middle
rather than the best tile), the field ground's terrain weighting, or the notch's weighing. Keep the names later
steps and the page read. Record each change to a rule as a decision. The piece's Decided holds whatever you
tune: nobody orders a clan to farm; the split moves one notch a turn toward what feeds the clan better; a
starving farmer falls back a notch; the yield is net of seed and grows with fertility and tame; a field
doesn't wear; a farmer's home follows its fields; the levy, the store, the gifts and the guard work as now.

## The bounds

Each is read from one `sweep --judge --json --floor` against the no-fields control in the same sweep. The
spec's table gives the measure and the probe's number:

1. Farming civs (share ≥ 0.1 at +200) gain at least +0.01 Standing at +200 on average against the control. At
   least three quarters of them gain, and their mean beats the non-farming civs' mean by at least 0.01.
   Aligned Standing at +200 rises past the floor.
2. A field day at a hearth of fertility 1 yields at least 1.5 times a gathering day: `1.5 × FIELD.grain ≥
   1.125`.
3. On at least 5 of 7 maps, the civ that farms most holds a pack on at least half its turns. At most 2 of 21
   civs never hold one.
4. Settled farmers' home moves per 100 clan-turns, over the seven maps, are under half the gatherers'.
   Settled farmers move on no more than gatherers.
5. People, land and clans changed hands: no fewer than the control, less the floor.
6. Starving clan-turns over the seven maps at most 1.25 times the control's share, and on no map over 1.6
   times. Settled farmers starve at a lower rate than gatherers.
7. Pre-games by centres 7 of 7. Panics 0. AI orders a hundred turns at least half the control's.
8. `sansfiat bench --size large 8 --after 50` within 1.25 times the branch's base.
9. `engine/check.sh` green, `www/play/fields.mjs` passing.

The probe at y8, the starting numbers, met 1, 2, 3, 5 and 7, missed 6 overall (1.29 times), and didn't
measure 4. Bound 6 is the work. It is where the probe's starving fell: gatherers squeezed beside farmers, most
on large 8 and medium 9. Bound 3 passed at y8 with nothing to spare. On the two civs that never hold a pack,
the guard eats the levy (`probe.md`, the store flow).

## How

Each sweep takes about three minutes with the sixth run. Run settings in a worktree a time, or change a number
and rerun. Start from y8 and move what the bound that misses points at. A higher `margin` or a smaller `notch`
slows the rush to fields. A higher `farmer` line keeps a clan's home on its land until it farms a real share.
Less `flood` ground leaves more of a river's food to its gatherers. Keep a table of every setting you run, with
each bound's number.

If no setting meets every bound, send the nearest with the bound it misses in an `ask`. If bound 3 misses
because the guard eats the levy, name each failing civ's store flow: levy in, gifts, posting, the guard's
meals, spoilage, the cap.

## Proof and record

Write `relay/specs/155/canvas.md`. It holds the table of settings with each bound's number, the chosen
setting's `fields` block and per-civ table, and `judge --compare` of the base (`main` at the piece's base, a
worktree) against the branch. Each number in `data.rs` you set carries a `note` naming `canvas.md`. If this
step is twinned, each version writes the same file on its own branch, from the same seven maps and the same
control, so the pick reads them side by side.

## What later steps rely on

The tuned `FIELD`, and every bound's number in `canvas.md`, for the review's PR.

## Done

Every bound met, or the ask sent with the nearest; `canvas.md` written; `check.sh` green.
