# #104 spec-review: what the runs showed

All on main's engine (`157a32e`), through `sansfiat play`, gift 0 unless said.

## The store is eaten by the camp at home, not empty

| Map, civ | Units at home | Store +10 | +20 | +30 | +60 | Guard |
|---|---|---|---|---|---|---|
| small 2 Red | camp 3 and guard 3 | 1.1 | 2.5 | 3.1 | 3.4 | starved by +10 |
| small 2 Red | camp disbanded at the pick | 4.0 | 12.7 | 14.7 | | 3.0, never hungry |
| medium 9 Gold | camp 3 and guard 3 | 5.6 | 14.0 | 13.9 | 12.2 | starved by +10 |
| medium 9 Gold | camp disbanded at the pick | 3.8 | 4.8 | | | kept |
| medium 9 Gold | camp marched out at +20 (guard already gone) | | 14.0 | 46.6 at +35 | | gone |

The levy brings a civ about 3–4 food a turn. A camp at the capital resupplies from the store
every turn, and the guard eats from it: the two together eat more than the levy brings, so the
led civ's guard starves within ten turns of the pick and the store sits at 3–14. With one of
them away the store fills: medium 9 Gold banks 46 by +35, nine warriors' packs. So "as many as
it can feed" is about four people of units kept at home, or a stack of eight or nine raised at
once from a banked store. The spec's "a handful" was measured with the camp eating at home.

## A march on a rival capital starves on the way

Medium 9 Gold's camp of 3, sent at Blue's capital (547) by `move` to 589:
- at the pick: at (3, −19) by +3, line cut by length (cost 18 carries 1.8); 2 warriors and
  hungry 3 turns at (−3, −17) by +6; gone by +9, two steps short of Blue's claim.
- at +20 with the store at 14: the same, gone by +28 at (−3, −17).

The path bends south round rough country. A line is cut past about cost 28, so the last
several steps are on the pack alone, and the pack is five turns. The stack story has to stage:
march to the last tile whose line isn't cut, refill, then push.

## Hunger beats surrender at 5

A camp loses a warrior every turn from its third hungry turn (`camp_turn`). A camp of three is
gone on its fifth hungry turn, so `FIGHT.surrender` 5 never fires on the starting camp, and a
stack of nine surrenders six. Surrender moves to `UNIT.hungry_turns` (3), checked before
hunger's take.

## The AI re-paints its claim every turn

`ai::land` sends `ai_claim` whenever it differs from the claim. A tile a stack takes is
re-claimed the next turn and re-held by the guard's share: the stack would fight and "take" its
own tile every turn it stands there, and `taken` would count it each time. And the tiles it
took walking in come back as Blue's land behind it, which is how Blue "closes round" it. So the
stand-in claim skips tiles a rival army holds, and only those.

## Surrender to whoever holds the ground

A stack's own tile is never a rival's land: it took it. So "its tile is a rival's land" never
holds for a surrendering army. It surrenders to the civ that cuts its line (`line::cutter`).

## Guards retreating would end every skirmish on its first turn

On an overlapped tile two guards' shares differ by their walks from two capitals. Blue's
shares on its claim run 0.18–0.46 at +1 on medium 9; a corridor's far end is thinner. Under
`retreat` × the other's (0.5), the thinner guard would leave on the first turn and the skirmish
would be decided at once. Guards don't retreat: a guard's tile changes hands only when its
people there are all lost, which a stack does in a turn (3 against 0.1) and a rival guard never
does (0.1 × 0.2 a turn against 0.2).
