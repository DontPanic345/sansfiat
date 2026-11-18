# #342 build-4 (glaze): herds that pay

Read `relay/specs/342/spec.md` whole, `probe.md` (nine settings the spec step ran on a sketch),
and the summaries and files of builds 1–3 (`clay.md`, `judge.md`). The rules are built; their numbers only work together. Tune them on the retro maps
against the no-herds control until every bound in the spec's table holds, read from
`sansfiat sweep --judge --json --floor` (its floor; one civ swings by up to half a point,
`main.md`). You may change the clay's and the kilns' rules where a bound needs it: record each change
as a decision with the sweep that shows why, and keep `core/tests/herds.rs`, the stage test and
`www/play/herds.mjs` passing (change what a test pins only with the rule it pins).

## What you tune

Every number in `HERD` (`data/herds.rs`), animals' `weight` in `GOOD`, and `herds::offtake`. The bounds that matter
most, in order:

1. **Herding civs gain Standing**: at least 4 civs herd at +200; they gain at least +0.01 on average
   past the floor, three quarters of them gain, and they beat the others by 0.01.
2. **Fields still pay**: farming civs' gain against no-fields not below +0.03 (main +0.049).
3. **Wide, thin land**: herders' tiles a person at least 1.5 times gatherers'.
4. **Starving** at most 1.25 times the control's past its floor, no map over 1.6.
5. **Animals change hands** on at least 4 of 7 maps.
6. Clans, people and land changed hands no fewer than the control's, less the floor; the bench at
   most 1.25 times main's in one session (five runs each).

## What the spec expects you to meet

The probe (`probe.md`) starts you at p4: 9 civs herd and 8 gain, +0.033, starving 1.15 times, and
two failures it couldn't fix:

- **Narrow land.** In every setting herders hold fewer tiles a person than gatherers (0.36–0.64
  against 0.70–0.94). The likely cause: a herding share of a tenth or a fifth gives few days, so
  `K` is bound by days × `tend`, not the grass, and another grass tile adds nothing to `steady(K)`.
  Look at which binds on real herders before turning the radius or the walk.
- **A trickle of animals.** Taking every increase keeps a herd at its start (341 animals over the
  seven maps). Taking only past `K ÷ 2` grows herds three- to fourfold but doubles starving
  (p6, p7); a notch that only a fed clan may raise kills herding (p8, p9); breed 0.5 brings
  starving to 1.79 (p5). Something between — a clan that keeps part of the increase while its herd
  is young, or a lean that only a herd past `K ÷ 2` gives — is yours to find.
- **Herds everywhere.** If every plains gatherer herds and farmers or the starving bound suffer,
  the spec's first lever is grazing that takes game: a grazed tile's game cap falls by the herd's
  share of its room. It's a rule: record it.
- **Lean.** If short clans empty their herds and starve after, lower `HERD.lean`.

Settings are one draw each: run each as one `sweep --judge --json` (about 4 minutes), two at a time
where the runner allows, and the chosen one with `--floor` (it runs the nulls too: read its time on
the first). Expect eight to twelve settings.

## Done

- Every bound holds, or the nearest setting and an `ask` naming the bound and what no setting moved.
- `relay/specs/342/glaze.md`: one row a setting (the six bounds' numbers), what each change did, the
  chosen setting's herds group per civ, and the bench.
- `HERD.note` says where the numbers came from (`glaze.md`) in a line.
- `engine/check.sh` green.

Commit as `#342 build-4: herds that pay`.
