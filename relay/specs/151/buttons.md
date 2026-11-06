# #151 build-7: `www/play/buttons.mjs`, the control and the labels it covers

`node www/play/buttons.mjs [--out DIR] [--break forage]` stages one game (`game.stage`, a radius-3 map
written in the script: sea on the rim, a hill, a forest, the led civ's capital A and the rival's B
two apart, food beside A, B and on P, a rival plains tile beside its capital), reads the bottom
panel's shape from the page (`game.shape()`, the page's own `describe`), and presses every button
the led civ's panels show, once each, on a fresh stage, checking the world after each press.
A label with no case fails the script ("button X has no case"). Runtime: 12.8 s on the branch
(the engine check was running beside it); `all.mjs` runs it with the other page scripts.

Result on the branch: exit 0, 34 cases over 25 labels, 105 `ok` lines.

## The control: `--break forage`

`--break forage` makes Forage's case press Pillage (greyed on the led civ's own land) in its place.
Run once (exit 1):

```
buttons: camp Forage: Pillage is pressed on the camp (it is greyed on the led civ's own land)
buttons: scout Forage: Pillage is pressed on the scout (it is greyed on the led civ's own land)
```

## Red on main, held by `expectBug`

Each of these passes while its check fails with the stated message; T drops the wrapper.

| Case | Message on main | Why |
|---|---|---|
| Disband… all, the camp (4.6 people) | Disband all sends the whole army home: the army is still on the map | the stepper opens at 1 and sends that (build-4's page arm) |
| Disband… all, the scout | Disband all sends the whole scout home: the scout is still on the map | the same |
| Disband… all, the guard | Disband all sends the whole guard home: the guard is still on the map | the same |
| Keep −1 | Keep −1 moves the keep by one (30 → 32) | see below |
| Keep +1 | Keep +1 moves the keep by one (26 → 30) | see below |

**Keep ±1 (found by this script; not in the brief).** The page's keep is `civs::keep`: the set amount
plus the guard's food (3 on this map). `Keep +1` sets the keep to the shown keep + 1, so the shown
keep moves by 1 + 3 = 4; `Keep −1` sets the shown keep − 1, so the shown keep moves up by 2. The
spec's table says "moved by one". Red on main; the fix is in `panels.js`' `civShape` (set from the
keep the civ set, not the shown keep), and it is an idea for the fixes piece.

## Labels covered

Each row is one case: the panel it is pressed on, the button, and the check it makes on the world.

| Panel | Button | Press | Check |
|---|---|---|---|
| civ (led) | Raise…/warriors | warriors, 1, Go | refused for want of store; no army appears (see gaps) |
| civ (led) | Raise…/archers | archers, 1, Go | refused for want of store; no army appears (gaps) |
| civ (led) | Raise…/a guard | a guard, 1, Go | the guard gains one person |
| civ (led) | Raise…/a scout | a scout, 1, Go | refused for want of store; no scout appears (gaps) |
| civ (led) | Gift −1 | after Gift +1 | the gift moves down by one |
| civ (led) | Gift +1 | press | the gift moves up by one |
| civ (led) | Keep −1 | after Keep +1 | **expectBug**: moves by one, red on main |
| civ (led) | Keep +1 | press | **expectBug**: moves by one, red on main |
| civ (led) | Keep default | after Keep +1 | the civ's set keep is cleared |
| civ (led) | Claim (P) | P, click a tile | the civ's claim gains the tile |
| civ (led) | Unclaim (U) | claim, U, click | the civ's claim loses the tile |
| camp | Move to… | arm, click | the camp's dest is the tile |
| camp | Stop | right-click, Stop | dest is none or its own tile |
| camp | Paint (P) | P, click | the zone gains the tile |
| camp | Unpaint (U) | paint, U, click | the zone loses the tile |
| camp | Undeploy | paint, Undeploy | the zone is one tile |
| camp | Forage | Auto-forage off, Forage, a turn | the civ's foraged counter rises (the one-shot) |
| camp | Pillage | press on rival plains P, a turn | the tile is the rival's claim and the press is taken (gap: no food effect) |
| camp | Resupply | Auto-supply off, Resupply, a turn | the civ's resupplied counter rises |
| camp | Auto-forage | press | the switch flips and its label says so |
| camp | Auto-supply | press | the switch flips and its label says so |
| camp | Raise…/warriors | warriors, 1, Go | refused for want of store; the army does not grow (gaps) |
| camp | Raise…/archers | archers, 1, Go | refused for want of store; the army does not grow (gaps) |
| camp | Disband…/warriors | warriors, 1, Go | one warrior goes home; the clan gains one |
| camp | Disband…/all | all, Go at the opening count | **expectBug**: red on main (the army is still on the map) |
| scout | Move to… | arm, click | the route is [its tile, the tile] |
| scout | Route from… | arm start, click, Move to… | the route is [start, end] |
| scout | Clear route | route set, Clear route | the route is none |
| scout | Forage | Auto-forage off, Forage, a turn | the scout's pack rises |
| scout | Resupply | Auto-supply off, Resupply, a turn | the press is taken and the scout survives (gap: no pack rise) |
| scout | Auto-forage | press | the switch flips and its label says so |
| scout | Auto-supply | press | the switch flips and its label says so |
| scout | Disband…/all | all, Go at the opening count | **expectBug**: red on main (the scout is still on the map) |
| guard | Disband…/all | all, Go at the opening count | **expectBug**: red on main (the guard is still on the map) |

The inventory also asserts the panels with no buttons for the led player: the rival civ, the clan,
the market and the tile (all four: none).

Panels the inventory reads: the led civ; the led civ with a set keep; the camp; the camp marching
(right-click); the camp deployed (painted); the scout; the scout routed; the guard; the rival civ,
the clan, the market and the tile.

## Gaps the staged map leaves (each is a finding, not a fix)

- **No civ store.** The founding spends the civ's endowment (20 grain) on the first army's pack, so
  the led civ's store is 0 for the whole run, and its gift (2 a turn) and its clans keep it there.
  Raise… (warriors, archers, a scout) is refused by the store ("the store can't fill their packs"), so
  the cases check that refusal and that no unit appears. The positive path is in the Rust tests; a
  guard's raise needs no store and passes. A stage that could give the led civ a store would show it.
- **Pillage's food.** A rival tile is two steps from the led capital on this map (B's claim touches
  no tile next to A that is not B's capital, and B's capital holds B's units). A camp there is cut
  off (its line can't carry a pack's worth), so it has no labour to pillage with: the counter stays 0
  with any army size tried. The case checks the tile is the rival's and the press is taken.
- **Scout Resupply.** The scout starts on the capital, where a resupply draws on the civ's store (empty).
  The case checks the press is taken and the scout survives the turn.
- **Disband…/archers.** The staged army is warriors only; the archers option is not in the inventory.
## Changes this step made beside the script

- `www/screens/play.js`: `shape()` exports the panel's shape for the selection (`describe(ctx(), selected)`).
- `www/main.js`: `window.game.shape()`.
- The stage's own tile numbers are column by column (`map::index`), which the script's `indexOf` follows.
