# #152 spec probe: can a story take a capital?

Run in the spec step on `main` (99a6904), with the start this piece decides: `UNIT.start` and
`GUARD.start` set to 0 in `data.rs` (founding raises the scout alone; the AI's
`Raise { n: UNIT.start }` is then refused, so it raises no army). A throwaway edit, not
committed.

## The stories as they are, 200 turns past the pick

`sansfiat story {skirmish,stack} --size S --seed N --civ K --turns 200` on small 2, 3, 6, 9 and
medium 8, 9, each civ led in turn (36 runs): **every run ends with three civs alive.** No capital
falls. `surrender` does nothing at all now, since it marches the starting army, which is gone.

On small 2 with the old start the stories also end with three civs alive (`main`'s binary,
default turns): no story has ever taken a capital (`relay/specs/104/glaze.md` says the same of
nine map/civ pairs over 120 turns).

## A bigger, later stack

`stack` with `STACK_WANT` 8 (raise when the store fills eight packs) and `STACK_BY` 150, 250
turns, each civ led on the six maps. The table: the largest army raised, the farthest it got
(steps from its own capital), the store's highest, and the civs alive at the end.

| Map, civ | Largest army | Farthest | Store max | Alive |
|---|---|---|---|---|
| small 2, 1 | 3 | 12 | 15.6 | 3 |
| small 2, 2 | none | | 8.1 | 3 |
| small 2, 3 | 1 | 4 | 26.9 | 3 |
| small 3, 1 | none | | 10.2 | 3 |
| small 3, 2 | none | | 8.3 | 3 |
| small 3, 3 | 4 | 16 | 26.6 | 3 |
| small 6, 1 | none | | 8.4 | 3 |
| small 6, 2 | 2 | 12 | 14.4 | 3 |
| small 6, 3 | none | | 10.4 | 3 |
| small 9, 1 | 2 | 9 | 22.4 | 3 |
| small 9, 2 | 9 | 14 | 47.2 | 3 |
| small 9, 3 | 9 | 14 | 46.7 | 3 |
| medium 8, 1 | 2 | 12 | 16.5 | 3 |
| medium 8, 2 | none | | 8.1 | 3 |
| medium 8, 3 | 1 | 4 | 10.9 | 3 |
| medium 9, 1 | 1 | 7 | 15.7 | 3 |
| medium 9, 2 | 8 | 10 | 41.4 | 3 |
| medium 9, 3 | 2 | 10 | 13.2 | 3 |

Small 9, Blue (civ 2): eight warriors raised at +8 with a pack of 28, staged four steps out,
where they wait ten turns (`WAIT`) while the store, empty, can't top the pack up. They push at
+19 with 8 food for 8 people, are cut and hungry from +20, and reach 13 of the 14 steps with
two left (+26). Later armies reach the rival's capital (14 steps) with one to three warriors,
starving, against a guard of 3–5.

## What it says

- With this economy a story's army reaches a rival capital only starving and with a few
  people left. The stack's wait at the stage burns its pack, since the store is empty by then:
  a script that pushes as soon as its pack is full, or banks more before it raises, may do
  better. That is the canvas's search.
- Capitals are 10 to 39 steps apart (the pick's cards): medium 9 has 10 and 13, small 2, 3
  and medium 9 have 13, large 8 has 37 to 39.
- A game ends by conquest only when one capital stands, which for a story is **two** capitals
  taken, or when the led civ's falls, which needs a rival army the AI no longer raises. So an
  early end on the stories' maps is a long way off; the canvas measures how near it gets,
  counting single capitals taken as well.

## The test fallout of the start

`cargo test` on `main` with both starts at 0 and nothing else changed: **94 tests fail in 15
files** (units 24, fight 18, orders 7, claim 7, civs 6, play 4, alerts 2, judge 2, stage 2,
save 2 (the pins), and one each in cases, events, leaning, notes, scout). 32 fail at
`common::camp_id` ("a camp", `core/tests/common/mod.rs:169`), 7 at `common::led_camp`
(line 22) and 3 at `Stage::army` (`stage.rs:284`): fixtures that expect founding's camp. The
rest assert founding's camp or guard directly. build-1 owns them.
