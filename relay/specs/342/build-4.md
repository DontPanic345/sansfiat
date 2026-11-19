# #342 build-4 (glaze): herds that pay

Read `relay/specs/342/spec.md` whole, `probe.md` (nine settings the spec step ran on a sketch),
`spec-review.md` (seven more settings, and each civ's animal share), and the summaries and files of builds 1–3 (`clay.md`, `judge.md`). The rules are built; their numbers only work together. Tune them on the retro maps
against the no-herds control until every bound in the spec's table holds, read from
`sansfiat sweep --judge --json --floor` (its floor; one civ swings by up to half a point,
`main.md`). You may change the clay's and the kilns' rules where a bound needs it: record each change
as a decision with the sweep that shows why, and keep `core/tests/herds.rs`, the stage test and
`www/play/herds.mjs` passing (change what a test pins only with the rule it pins).

## What you tune

Every number in `HERD` (`data/herds.rs`), animals' `weight` in `GOOD`, and `herds::offtake`. The bounds that matter
most, in order:

1. **Herding civs gain Standing**: at least 4 civs herd at +200, a civ herding when animals are a
   tenth of its clans' food (the spec's animal share, not its days); they gain at least +0.01 on
   average past the floor, three quarters of them gain, and they beat the others by 0.01.
2. **Fields still pay**: farming civs' gain against no-fields not below +0.03 (main +0.049).
3. **Wide, thin land**: herders' tiles a person at least 1.5 times gatherers', over at least 40
   herder clans.
4. **Starving** at most 1.25 times the control's past its floor, no map over 1.6.
5. **Animals change hands** on at least 4 of 7 maps.
6. Clans, people and land changed hands no fewer than the control's, less the floor; the bench at
   most 1.25 times main's in one session (five runs each).

## What the spec expects you to meet

The probe (`probe.md`) and spec-review's runs (`spec-review.md`) start you at p4, and show where it
falls short:

- **Herds feed nobody yet.** In every setting run, animals are at most 4% of any civ's food; at p4,
  1–2%. p4's nine "herding civs" gave a tenth of their days to herds that barely fed them, and their
  +0.033 is not herds feeding them. Arithmetic says why: a herd yields at most `breed × K ÷ 4` a
  turn, so to feed one person at breed 0.3 a clan needs about 13 animals a person at `K ÷ 2`, and a
  herd that starts at `HERD.start` (0.5 a person) and gives up its whole increase stays near its
  start. Herds have to grow toward their land's `K`, and `K` has to be large beside the clan.
- **Wide land comes from `tend`.** When a herding day tends many animals, the grass binds and an
  added grass tile adds to `K`: herders' tiles a person go 0.66 of gatherers' at tend 3, 1.24 at
  12, 1.79 at 24 (`spec-review.md`). The radius and the walk don't bind before that. But herding
  then takes few days, so a days count of herders falls (12 herder clans at tend 24): watch
  `HERD.herder` with `tend`, and the 40-clan floor on the bound.
- **Growing herds starves clans.** Taking only past `K ÷ 2` grows herds three- to fourfold but
  starving goes to 1.8–2.0 (p6, p7, and tend 12 in `spec-review.md`), since a clan gives days to a
  herd that feeds it later and the notch reckons the herd's steady yield, not what it feeds now.
  A notch that only a fed clan may raise kills herding at tend 3 (p8, p9), where a herding day
  feeds a third of a gathering day; at tend 24 it leaves starving at 1.8 too (`spec-review.md`), so
  the starving is herding clans whose herd below `K ÷ 2` feeds them nothing while the notch counts
  its steady yield: the offtake and the reckoning are one rule. The nearest so far is tend 24 with
  the `K ÷ 2` offtake: 7 civs get 5% or more of their food from herds, 5 gain, starving 1.79.
  Something between — a herd built only from a clan's surplus, a
  reckoning that weighs how long a herd takes to grow, a lean only a herd past `K ÷ 2` gives — is
  yours to find. Expect a rule change here, recorded.
- **Herds everywhere.** If every plains gatherer herds and farmers or the starving bound suffer,
  the spec's first lever is grazing that takes game: a grazed tile's game cap falls by the herd's
  share of its room. It's a rule: record it.
- **Lean.** If short clans empty their herds and starve after, lower `HERD.lean`.

Settings are one draw each: run each as one `sweep --judge --json` (about 3 minutes alone; on the
spec-review's runner four at once took 12 minutes each, no faster than one after another), and the chosen one with `--floor` (it runs the nulls too: read its time on
the first). Expect eight to twelve settings.

## Done

- Every bound holds, or the nearest setting and an `ask` naming the bound and what no setting moved.
- `relay/specs/342/glaze.md`: one row a setting (the six bounds' numbers), what each change did, the
  chosen setting's herds group per civ, and the bench.
- `HERD.note` says where the numbers came from (`glaze.md`) in a line.
- `engine/check.sh` green.

Commit as `#342 build-4: herds that pay`.
