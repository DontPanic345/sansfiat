# #104 build-4 (kiln): the page shows armies and their fights

You put the piece in front of the player. The engine is done: armies are stacks of kinds
(`stack`, `still`, `bows` on a camp unit's JSON), a civ may have many, and fights, retreats,
surrenders and fallen capitals arrive as events and notes. Keep the page's shape from #103:
layers read the model, one panel component renders every selection, `feed.js`'s `KINDS` lists
every event kind, and `assets/sounds.json` maps each kind to a file. Read `spec.md` items 18
and 23, the earlier steps' summaries, `docs/design/page.md`, and `engine/CLAUDE.md`'s page
rows. Where the engine's shape doesn't hold for the page, make the smallest fix that works and
record it as a decision.

## What to build, and where

1. **Stacks on the map** (`www/map/layers/units.js`, `www/model.js`). Each army is drawn as
   its figure with its people as a number beside it (one decimal under ten, whole above), and
   archers marked (a small bow, or the number split as "6+2"). Several units on one tile stay a
   stack that a click cycles, as now. A fallen civ draws nothing. Add atlas cells only if you
   need them (`unit.archers` and its `.mask`, 64×64, in `atlas.json` and `placeholders.js`,
   redrawn with `draw-atlas.mjs`).

2. **The army's panel** (`www/panels.js`, `campShape`). The name is "Red's army 475".
   - Doing gains "Fighting at (x, y)" when its tile is contested this turn (from the view, or
     from the last `Skirmish` note), and "Making bows" while `bows` is set.
   - Numbers: warriors and archers separately; strength in attack and defence
     (`rules().fighters` from the engine, people × each, with hunger's factor when hungry);
     entrenched, as turns and the bonus.
   - Buttons: Raise… is a stepper whose options are warriors and archers, sending
     `{type:'raise', kind:'camp', fighter, into: u.id, n}` and greyed with `Sim.check`'s reason.
     Disband… has an option per kind present, plus all.

3. **The civ's panel** (`civShape`). Raise… offers warriors and archers (a new army at the
   capital), guard and scout. A fallen civ reads "Fallen to Blue on turn N" with no buttons.

4. **The scout's panel** (`scoutShape`). Auto-forage and Auto-supply toggles, Forage,
   Resupply and Disband…, as the camp has them.

5. **The engine's half** (`web/src/lib.rs`). `rules()` gives each fighter's name, attack and
   defence, and `FIGHT`'s `entrench`, `entrench_cap` and `hungry`, so the panel can show
   strength without a number in JS. `view()` gives each civ's `pressed`, `besieged` and
   `fallen`, and the contested tiles of the last turn if build-1 keeps them on `World`. Nothing
   else changes in the engine. `www/check.mjs` checks the new fields.

6. **The feed, alerts and sound** (`www/feed.js`, `assets/sounds.json`). Every new event kind
   (`skirmish`, `tileTaken`, `retreated`, `armySurrendered`, `capitalFell`, `archersRaised`,
   `scoutRaised`, and any other the engine added) gets an icon, a colour and a label in
   `KINDS`, and a file name in `sounds.json`. `play/art.mjs` already fails on a kind missing from
   `sounds.json`. A click on one selects its unit or civ, as now. Alerts come from the engine.

7. **Why.** Selecting an army, or the civ, after a fight shows the fight's note among its
   reasons ("fought Blue's guard at (−4, −10): strength 6.0 against 0.4; …"). Check that it
   does, and that a tile changed hands shows on the tile's panel ("Red's land, taken from Blue
   on turn N" if the engine gives it; else leave it).

8. **The script** (`www/play/army.mjs`, new; add its row to `engine/CLAUDE.md`'s table). On a
   retro map where the led civ holds forest (find one at run time; build-3's `stories.md` names
   the maps), from the pick it:
   - sets the gift to 0 and ends turns until the civ panel's Raise warriors isn't greyed: every
     store is 0 at the pick and a warrior's pack is 5 food, and medium seed 9's Gold has 5.6 by
     +10 (`spec-review.md`);
   - raises warriors as a new army through the civ's panel;
   - raises archers into it through the army's panel, or sees Raise archers greyed with "you
     hold no forest…";
   - sees the stack's number on the map;
   - marches the army with Move to… onto a rival's guarded tile;
   - ends turns until a `skirmish` is in the feed (or a `tileTaken`), clicks it, and reads the
     fight's note in the panel's reasons.

   It exits non-zero when any of these is missing, and writes `army-*.png` to `--out`.
   `play/all.mjs` runs it. If marching onto a rival's tile takes too long for CI (`all.mjs`'s
   whole run should stay under about three minutes), build a shorter way to the fight and say
   how.

## Done

- `./check.sh` passes with `army.mjs` in `play/all.mjs`, and every other play script still
  passes.
- Screenshots at laptop size of an army selected mid-fight, the civ panel's Raise…, and the
  feed with a skirmish, in `relay/specs/104/screens/`, under 1 MB in all.
- Your summary lists the new buttons and the levers they send. Build-5 and the review count the
  levers.
