# #338 build-2: what's worst on the page, small 61288 as Gold (card 3, civ 202)

Played through `www/play/live.mjs`, the page alone (printouts, tooltips, screenshots), before any fix of
mine. Two games, both in `play/`:

- **Game 1** (`small-61288-first.json`): claimed 13 clans on the first turn (Standing 7.4% → 19.8%), sent
  its only army after a Blue army that had taken a tile, and **lost the capital to Blue on turn 268, +11**.
  Judge: 36 orders in 11 turns; the idle led civ falls at +3, so Blue rushes this capital on any play.
- **Game 2** (`small-61288.json`, the counted game): replayed from the pick with an army kept home. 101
  turns, Standing 16.5% (rank 3) against 29.7% (rank 1) for Gold on the AI.
  `player orders: 58 (claim 45, move 6, raise 3, setGift 4), 57.43 per 100 turns; turns with none 92 of 101, longest run 39`
  (control, small 2 civ 1 on main: 11 in 103, 95 with none).

The notes I took before each decision are `play/notes-small-61288.md`; the driver's lines are
`play/small-61288.live.jsonl` (game 2, both driver runs, joined) and `play/small-61288-first.live.jsonl`.
(The brief named them `.lines.jsonl`, but `sansfiat judge` reads `GAME.lines.jsonl` as `sansfiat play`
lines and fails on the driver's; they are `.live.jsonl`.)

## The faults, worst first

1. **FIXED. Nothing said the capital was open.** Game 1, turns 261–268 (`first-fall.png`): I sent my army 7
   steps out after Blue's; Blue's army walked onto my empty capital and the game ended. The page never said a
   capital with no army of yours on it falls to a rival army that ends its turn there, nor that Blue's army
   was a few steps off. It cost the game, the biggest decision there is. No rule change. **Fix:** the led
   civ's panel and the text view carry a Capital line, always: `held by your army, 6.0 warriors; Blue's army
   of 6.0 is 7 steps away`, or `no army of yours on it: a rival army that ends its turn there takes it, and
   the game; …` (`outlook::capital_line`). Check: `check-stage.mjs`, the capital and reach lines.

2. **FIXED. An army cut off by rival land surrendered when its panel said hunger would thin it.** Game 2,
   turns 312–314 (lines file, `end` at 314; `t315.png`): my army took three of Blue's tiles inside my area;
   Blue's army took the tile behind it, cutting its line. Its panel read `Hungry 2 turns: loses a warrior at
   3`; at 3 it surrendered whole to Blue (`FIGHT.surrender` is 3, and surrender comes before hunger's loss),
   five people to Blue, and left me with no army and an empty store for 40 turns. A truthful line would have
   sent it home a turn sooner. No rule change. **Fix:** the view carries `cut_by` (`units::cut_by_rival`),
   and the army's doing line says `Hungry 2 turns, cut off by Blue's land: surrenders to Blue at 3`, or, not
   yet hungry, `Cut off by Blue's land: hungry 3 turns standing here, it surrenders`; the text view's army
   line adds `cut off by civ N's land: surrenders hungry 3 turns in a row`. Check: `check-stage.mjs`.

3. **The reach line (the spec's build, not one of the three).** Turns 285–303 and 316–354: 20 and 39 turns
   with nothing done (`t285h.png`), because the page doesn't say where a claim would still bring a clan. On
   turn 258 I found clan homes by hovering tile after tile; later, with Blue's claim threading into my area,
   I could find none. The saved game at turn 359 had `claim 2 tiles to (9, 1)` in reach all along. **Built:**
   the Reach line (below).

4. **FIXED. Tiles are named (q, r) but nothing on the page shows a tile's (q, r).** Turn 258: the claim
   refusal `(12, 1) is not` and the feed's `Gold's land at (-1, 8) is taken` name tiles the player can't find:
   tooltips and panels gave terrain and clans, never the hex. No rule change. **Fix:** a tile's tooltip
   heads with `coast (9, 1)`. Check: `play/camera.mjs`, the tooltip names the tile.

5. **Alerts are flooded with rivals' gains.** By turn 316 the alert box held some 60 items, most `Clan N
   joins Red/Blue: its home is on Red's land` (lines file, any `end` after 290), burying the army's hunger
   and cut line; `end N` in the driver stops on each. `World::alert_key` marks a rival's `ClanJoined` as an
   alert on purpose. Whether a rival's gain is the led civ's trouble is the owner's call: an idea.

6. **The map moves when the panel changes height.** Turn 311 (`notes`): selecting the army (a taller panel
   than the civ's) shifts the whole map ~27 px; a right-click aimed from the screen before sent the army to
   (7, -7), 32 steps off, on turn 303. Fixable in the page (keep the view's centre in world terms when the
   map's box resizes) but not one of the top three.

7. **The gift's reason contradicted the numbers beside it.** Turn 271: `reaches 1 clan · gave 0.0 · left
   2.0`, tooltip `the store held less than keep plus gift`, with the store at 35.8 and keep 10. *Reaches* is
   now and *gave* is the last meeting, so the clan wasn't there then. build-1's tooltip falls short; it
   should say both possible reasons. Small.

8. **Rival land inside my claim is drawn faintly.** Turn 303–311: seven single Blue tiles holding clan homes
   inside my border were only findable by tooltip (`Blue's land`).

9. **The civ's Claim, Raise… and Signpost… buttons have no tooltip**; `P` with nothing selected does
   nothing and says nothing; a claim stroke over water skips the water silently.

10. **A claim has no size limit**: thirteen clans joined in one turn on turn 258 (Standing 7.4% → 18.7%).
    Noted, not filed: the probe already found it (`probe-review.md`), and it is a rule.

11. Noted: the army panel always shows the greyed Pillage's reason under Why, as if refused.

## The driver (build-1's), changed where play showed it short

- `@x,y` pixels were read as the canvas's, not the window's: every pixel landed one top bar (40 px) low.
- It printed before the view's ease ended, so the next pixel aimed at a moving map; it now waits for rest.
- `tip LABEL` reads a button's or a panel number's tooltip; a tile's tooltip prints its lines split by ` / `.
- `button Legend` presses a fold's heading (the legend covered the bottom-left of the map).

## The reach line, read on the real maps

- **My game** (`small-61288.json`, turn 359), text view (`sansfiat play --game COPY`, `view`; one line, at the end of the gift line):
  `reach: in reach: claim 2 tiles to (9, 1) ((8, 2), (9, 1)): brings clan 3, 5.0 people; army 273 can take Blue's land at (-3, 6): march it there`
  `; capital: held by your army, 6.0 warriors; Blue's army of 6.0 is 7 steps away`
- **The page** (`live.mjs load`), the civ panel's Reach row the same; then `Claim (P)`, `click 8,2`,
  `click 9,1`: Clans 17 → 18, feed `359 ·⇄Clan 3 leaves Red for Gold: its home is on Gold's land`, and the
  line moved on to `claim 4 tiles to (6, 9) (…): brings clan 181, 2.8 people` (`play/reach-claim.png`).
- **small 2**: `relay/specs/337/play/small-2.json` is save version 25 and this build reads 26, so it can't be
  opened. In its place, small 2 on the AI for 110 turns (`play/reach-small-2-ai.txt`, civ 467 every 10
  turns): a claim is in reach at every sample; from +60 the army's reach shows (`army 477 can take Gold's
  land at (4, -9): march it there`); no rival capital came into any civ's reach.
- The nothing-in-reach form is proved on a stage (`core/tests/outlook.rs`).
