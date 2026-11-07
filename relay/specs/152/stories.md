# #152 build-5: does a story end a game before the limit?

**No. Not yet, on this economy.** No game on the seven retro maps ends before the limit with any
civ led, by any story: all 60 saves end `limit`, the judge's `early` is false on every one, and so is
the `--no-fight` control's (which can't fall). The best story, `conquest`, takes **one** capital in
3 of its 21 games (small 2 civ 3 at +76, small 3 civ 3 at +51, small 6 civ 2 at +14) and never the
second, which is what a conquest ending needs. Nothing about the fight, supply or economy numbers was
changed to get there (spec decision 12).

## The story

`conquest` is new in `cli/src/story.rs`; `stack`, `skirmish` and `surrender` are as they were (their
starts were done in build-1). It plays the strongest plan the player's levers allow:

- +0: gift 0, and **`SetKeep` 60**, the store's cap (`CIV.store_cap`). By default a civ posts to the market what its
  store holds past a pack, so a store doesn't bank; with the keep it banks to its cap. This is the lever
  the other stories don't use.
- Once the store fills four warriors' packs (or at +80), every warrior it fills is raised at once into one
  army, both switches on, and sent at the nearest standing rival's capital in **one move, with no stage
  and no wait**. After a fall the target is the other rival's. An army lost is followed by the next, banked
  and raised the same way.
- Guard kept by the AI's rule, as every story does.

What was tried (small and medium maps, each civ led; 144 + 108 runs of `conquest`, counting capitals taken):

| Variant | Capitals taken in 18 small/medium games |
|---|---|
| waves of 2 / 3 / 4 / 5 / 6 / 8 / 11 warriors (raise at that many packs, or at +60 / +80 / +110) | 2 / 3 / 3 / 2 / 1 / 2 / 0 to 1 |
| the store not kept (the default keep), raise at 9 packs or +60 | 0 of the 21 games; no army at all in 6 of them (the store never fills a pack) |
| `stack`'s stage and wait at the stage tile, 8 packs banked | none: the wait eats a pack the store can't refill (`probe.md`) |
| hopping a tile a turn, pillaging a rival's land when the pack is under 60% | none: the army stalled at 10 steps, a rival's tile held about one food |
| pushing straight, waves of 4, keep 60 (chosen) | 3 |

## Results

Each of the 60 saves: `story NAME --turns 0 [--no-fight]`, then `judge FILE --json`. `ended` is the
phase at the end; rank is the led civ's rank at the end, in the game, in `judge`'s AI control, and in
its idle control; steps are from the led civ's own capital, and the capitals are 10 to 14 steps away on
small 2, 3, 6 and 9 and medium 9 (nearest rival) and 23 to 26 for the other rival, 37 to 39 on large 8.

### Played (fight on)

| map | civ | story | ended | capitals fallen (first at) | led rank: game / AI / idle | largest army | farthest (steps, turn) | most at a rival capital (first there) |
|---|---|---|---|---|---|---|---|---|
| small 2 | 1 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 4 | 13 (+13) | 3.4 (+13) |
| small 2 | 2 | conquest | the limit +200 | 0 (never) | 3 / 2 / 3 | 1 | 11 (+85) | 0.0 (never) |
| small 2 | 3 | conquest | the limit +200 | 1 (+76) | 2 / 1 / 1 | 10 | 23 (+77) | 5.3 (+21) |
| small 3 | 1 | conquest | the limit +200 | 0 (never) | 2 / 2 / 2 | 2 | 12 (+85) | 0.0 (never) |
| small 3 | 2 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 3 | 3 | conquest | the limit +200 | 1 (+51) | 1 / 1 / 1 | 8 | 24 (+54) | 4.1 (+20) |
| small 6 | 1 | conquest | the limit +200 | 0 (never) | 2 / 1 / 2 | 5 | 16 (+32) | 0.0 (never) |
| small 6 | 2 | conquest | the limit +200 | 1 (+14) | 1 / 2 / 2 | 5 | 23 (+28) | 2.5 (+12) |
| small 6 | 3 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 2 | 16 (+86) | 0.0 (never) |
| small 9 | 1 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 9 | 2 | conquest | the limit +200 | 0 (never) | 2 / 1 / 1 | 7 | 13 (+18) | 0.0 (never) |
| small 9 | 3 | conquest | the limit +200 | 0 (never) | 2 / 2 / 2 | 10 | 14 (+66) | 0.0 (never) |
| medium 8 | 1 | conquest | the limit +250 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| medium 8 | 2 | conquest | the limit +250 | 0 (never) | 1 / 1 / 1 | 1 | 11 (+85) | 0.0 (never) |
| medium 8 | 3 | conquest | the limit +250 | 0 (never) | 2 / 2 / 2 | 5 | 25 (+74) | 0.0 (never) |
| medium 9 | 1 | conquest | the limit +250 | 0 (never) | 2 / 1 / 1 | 3 | 16 (+87) | 0.0 (never) |
| medium 9 | 2 | conquest | the limit +250 | 0 (never) | 3 / 2 / 2 | 6 | 10 (+11) | 0.0 (never) |
| medium 9 | 3 | conquest | the limit +250 | 0 (never) | 3 / 3 / 3 | 4 | 11 (+14) | 0.0 (never) |
| large 8 | 1 | conquest | the limit +300 | 0 (never) | 2 / 2 / 3 | 10 | 34 (+26) | 0.0 (never) |
| large 8 | 2 | conquest | the limit +300 | 0 (never) | 3 / 3 / 3 | 10 | 34 (+37) | 0.0 (never) |
| large 8 | 3 | conquest | the limit +300 | 0 (never) | 3 / 1 / 1 | 10 | 35 (+24) | 0.0 (never) |
| small 2 | 1 | skirmish | the limit +200 | 0 (never) | 2 / 3 / 3 | 3 | 9 (+18) | 0.0 (never) |
| small 2 | 2 | skirmish | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | skirmish | the limit +200 | 0 (never) | 1 / 1 / 1 | 4 | 20 (+18) | 0.0 (never) |
| small 2 | 1 | stack | the limit +200 | 0 (never) | 3 / 3 / 3 | 3 | 12 (+20) | 0.0 (never) |
| small 2 | 2 | stack | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | stack | the limit +200 | 0 (never) | 1 / 1 / 1 | 4 | 20 (+18) | 0.0 (never) |
| small 2 | 1 | surrender | the limit +200 | 0 (never) | 2 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 2 | surrender | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | surrender | the limit +200 | 0 (never) | 1 / 1 / 1 | 0 | 0 (+0) | 0.0 (never) |

### Control (`--no-fight`)

| map | civ | story | ended | capitals fallen (first at) | led rank: game / AI / idle | largest army | farthest (steps, turn) | most at a rival capital (first there) |
|---|---|---|---|---|---|---|---|---|
| small 2 | 1 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 4 | 13 (+13) | 3.0 (+13) |
| small 2 | 2 | conquest | the limit +200 | 0 (never) | 3 / 2 / 3 | 1 | 11 (+85) | 0.0 (never) |
| small 2 | 3 | conquest | the limit +200 | 0 (never) | 3 / 1 / 1 | 10 | 21 (+11) | 6.0 (+11) |
| small 3 | 1 | conquest | the limit +200 | 0 (never) | 2 / 2 / 2 | 2 | 13 (+85) | 2.0 (+85) |
| small 3 | 2 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 3 | 3 | conquest | the limit +200 | 0 (never) | 1 / 1 / 1 | 9 | 16 (+8) | 6.0 (+10) |
| small 6 | 1 | conquest | the limit +200 | 0 (never) | 2 / 1 / 2 | 5 | 17 (+33) | 2.0 (+33) |
| small 6 | 2 | conquest | the limit +200 | 0 (never) | 2 / 2 / 2 | 4 | 17 (+11) | 2.0 (+12) |
| small 6 | 3 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 2 | 16 (+86) | 0.0 (never) |
| small 9 | 1 | conquest | the limit +200 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 9 | 2 | conquest | the limit +200 | 0 (never) | 2 / 1 / 1 | 10 | 14 (+11) | 6.0 (+12) |
| small 9 | 3 | conquest | the limit +200 | 0 (never) | 2 / 2 / 2 | 10 | 14 (+20) | 6.0 (+20) |
| medium 8 | 1 | conquest | the limit +250 | 0 (never) | 3 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| medium 8 | 2 | conquest | the limit +250 | 0 (never) | 1 / 1 / 1 | 1 | 11 (+85) | 0.0 (never) |
| medium 8 | 3 | conquest | the limit +250 | 0 (never) | 2 / 2 / 2 | 5 | 25 (+74) | 0.0 (never) |
| medium 9 | 1 | conquest | the limit +250 | 0 (never) | 2 / 1 / 1 | 3 | 16 (+87) | 0.0 (never) |
| medium 9 | 2 | conquest | the limit +250 | 0 (never) | 3 / 2 / 2 | 7 | 10 (+11) | 3.0 (+12) |
| medium 9 | 3 | conquest | the limit +250 | 0 (never) | 3 / 3 / 3 | 4 | 11 (+14) | 0.0 (never) |
| large 8 | 1 | conquest | the limit +300 | 0 (never) | 2 / 2 / 3 | 10 | 34 (+26) | 0.0 (never) |
| large 8 | 2 | conquest | the limit +300 | 0 (never) | 3 / 3 / 3 | 10 | 34 (+37) | 0.0 (never) |
| large 8 | 3 | conquest | the limit +300 | 0 (never) | 3 / 1 / 1 | 10 | 35 (+24) | 0.0 (never) |
| small 2 | 1 | skirmish | the limit +200 | 0 (never) | 2 / 3 / 3 | 3 | 9 (+18) | 0.0 (never) |
| small 2 | 2 | skirmish | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | skirmish | the limit +200 | 0 (never) | 1 / 1 / 1 | 4 | 20 (+18) | 0.0 (never) |
| small 2 | 1 | stack | the limit +200 | 0 (never) | 2 / 3 / 3 | 3 | 13 (+21) | 1.0 (+21) |
| small 2 | 2 | stack | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | stack | the limit +200 | 0 (never) | 1 / 1 / 1 | 5 | 21 (+39) | 1.0 (+39) |
| small 2 | 1 | surrender | the limit +200 | 0 (never) | 2 / 3 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 2 | surrender | the limit +200 | 0 (never) | 2 / 2 / 3 | 0 | 0 (+0) | 0.0 (never) |
| small 2 | 3 | surrender | the limit +200 | 0 (never) | 1 / 1 / 1 | 0 | 0 (+0) | 0.0 (never) |

early: []
Counter({'games with a fall': 3})

## Why not: what runs out

- **Food, on the march.** An army's pack holds five turns of its warriors' food, and a march leaves
  the army no labour to forage or to carry from the store (`camp_march` returns 0 labour left until the
  army has arrived), so a march longer than the pack is a starving one. A nearest capital 13 or 14 steps away is
  reached on turn 7 to 10 of the march with the pack gone on turn 4 or 5: the army is hungry (half
  strength) for the last three turns and loses a warrior a turn from the third. Pushing at the capital
  with eight to ten warriors, the army that arrives is one to five (largest figures at a rival capital: 5.3,
  4.1, 3.4, 2.5). The second capital is 23 to 26 steps out: the last wave on small 6 civ 2 got there
  with 2 of 5 warriors, hungry, and the militia of 8.8 people held.
- **The store can't feed a bigger army.** With the keep the store banks to 60, but the levy brings 2 to
  4 food a turn: waves of four beat waves of eight to eleven (a bigger wave waits longer and starves as
  long). A raised army also costs its people: the led civ's rank at the end is lower than the AI control's in
  7 of the 21 games, and better in 1.
- **The fight at the capital** needs the warriors the march doesn't leave. A capital falls when no part
  of its civ is left there: no guard, and under one person of militia (the clans whose home it is,
  strength `FIGHT.militia` 0.3 a person, 6 to 9 people on these maps). Where the rival's guard is thin
  (the AI raises one to three) 2 to 4 warriors can clear it and the militia in several turns; a hungry
  warrior is half a warrior. In the three games where a capital fell, the rival's guard was down to 0.1 or 0.2 of a person
  (the waves had worn it) and the fall came to the 6th, 4th and 1st wave (small 2 civ 3, small 3 civ 3,
  small 6 civ 2).
- **Time is not what runs out.** The first army is at the nearest capital by +12 to +21, the first
  fall is at +14 to +76, and waves keep coming every 10 to 13 turns to the limit; a longer limit would
  give more waves, not a better wave.

So the piece's answer is "not yet, on this economy": F (the store worth taking: a pack that lasts, a
store that banks) and R (a rival that plays for the end) are the pieces that move it. The end exists,
the judge reads it on every run, and a staged game ends by conquest, by the led civ's fall and by the
limit (`core/tests/over.rs`, `www/play/end.mjs`).

`conquest` in `cli/tests/story.rs` saves, replays to its digest, and `judge` prints the end line.
