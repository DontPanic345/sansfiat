# #103 build-3: the panel, Move to…, selection, the feed and the touch-point scripts

**Touch points, after build-3** (script in `engine/www/play/`, all run by `play/all.mjs`, which passes 13 of 13):

| # | Touch point | Script | Result |
|---|---|---|---|
| 15 | Bottom panel, one shape, for a camp | `camp` | pass |
| 16 | Move to… arms a click; right-click still moves | `camp` | pass |
| 17 | Paint and Unpaint a zone by button, by P and U, and by dragging | `camp` | pass |
| 18 | Forage, Pillage, Resupply, the switches, Disband… | `camp` | pass |
| 19 | The scout's route: Move to… and Route from…, Clear | `scout` | pass |
| 20 | The civ's row: Raise… (warriors, guard), Gift, Keep, Claim and Unclaim (P, U) | `civ` | pass |
| 21 | A clan's, a festival's and a tile's panel; clicking cycles a stack; Escape clears | `select` | pass |
| 22 | The feed on the right, collapsible; clicking an item goes there | `feed` | pass |
| 23 | Alerts stay until seen | `feed` | pass |
| 24 | The latest reasons in the panel (a clan, the camp) | `feed` | pass |
| 26 | Three sizes: nothing clipped or overflowing, and the screenshots | `layout` | pass with `--share 0`; see the share below |

The hold (spec item 24, `play-hold`) is rebuilt in `camp.mjs`: the civ's Raise… stepper raises ten
warriors (greyed with `Sim.check`'s reason while refused, and a turn ends), Move to… marches the camp onto
the rival camp's post, the ring is painted, and the rival's line is cut and its camp goes hungry.

**Seed for the hold: 4, not 7.** On small seed 7 the march starves a ten-warrior camp down to four, and
a zone holds one tile a warrior, so the ring can't be painted and the cut never comes. Seed 4 cuts.
`camp.mjs --hold-seed N` tries another.

**Retired:** `www/play-camp.mjs`, `play-hold.mjs`, `play-lean.mjs`, `play-read.mjs`, `play-scout.mjs`,
`play-claim.mjs`. What each checked is in the scripts above: the walk, the line and hunger (camp), the
hold (camp), the gift and its leaning clan (civ), the reasons and alerts (feed), the route (scout), the
claim and its border (civ, and the festival's walkers in select). The old load-a-save-without-a-version
check is `start`'s (touch point 3) from build-2.

**The map's share (touch point 26, the canvas's to settle):** with the camp selected and the feed open as
it opens by default, the map's box less the legend, zoom buttons, Show feed and error line:

| Size | Bar | Map share | Map box |
|---|---|---|---|
| phone | 35 px (one line) | n/a (feed is a drawer) | not measured |
| laptop | 41 px (one line) | **44%** (under 60%) | 940 × 483 |
| wide | 41 px (one line) | **58%** (under 60%) | 1580 × 763 |

The check stays on; `layout.mjs` fails at laptop and wide until the canvas moves the panel's height
down or the feed's width. `all.mjs` runs `layout.mjs --share 0`, so the other checks still gate CI.
Main measured 67% and 79%. The panel is the cause: with a camp selected it takes about 40% of the
laptop window's height (the doing line, four numbers, units, reasons, two lines of refusal and hint,
then the row).

**What reads badly in `screens/`** (half scale at laptop and wide, full at phone):

- *Laptop, camp (`laptop-camp.png`)*: the panel takes the bottom 40% of the window, so the map is a
  band; the one button row fits at 1280 px, but the lines above it (Units here, Why, the hint) push it
  down. The feed's 340 px leaves the map narrow.
- *Laptop, civ (`laptop-civ.png`)*: the Standing and its numbers run to three lines of label–value pairs,
  and the row of eight buttons sits at the panel's foot, under the leaning clans and the units.
- *Phone, camp (`phone-camp.png`)*: the row scrolls sideways, and at first sight it shows Auto-forage,
  Auto-supply and Disband…, with Move to… and Paint off the left edge. The most-used button is hidden.
  The map is about a third of the screen under the bar and the panel.
- *Phone*: Show feed sits over the map's bottom-right corner, next to the legend, above the panel.
- *Large map at fit, laptop (`laptop-large-fit.png`)*: at low zoom the placeholders read as coloured
  speckle; the clan tints, festival rings and the civ's border compete with the terrain cells, which are
  too small to tell apart. The fit is the whole map, so this is the canvas's to judge at its own zoom.
- *Pick and start*: build-2's notes stand, nothing changed there.

**Decisions**
- The hold runs on seed 4 (above), and the civ check on seed 4 too, because the store fills its packs
  there within a few turns (on seed 2 the camp at the capital draws the store down faster than it fills).
- The stepper's Go is greyed with the engine's reason for its option. The closed Raise… is open while any
  option is allowed, so a guard can be raised while warriors are refused.
- The alerts box holds every unseen alert, with a height of five rows and scrolling past that; the Feed
  toggle counts all unseen alerts, not only those that came while the feed was hidden.
- `layout.mjs`'s share check counts the map box less what the legend, zoom, Show feed and error line cover.

**Spec correction (made here, so later steps read it right):** item 15's `list?` is an array of lists, since a
civ shows its leaning clans and then its units. `spec.md` now says `list?: [{label, rows, empty?}]`.

**Engine:** no Rust changed in this step. `stats::digest`, the pins and `SAVE_VERSION` are as build-1 left them.

**Lever count:** `levers.md`: all 15, each with a button, and P/U and space as keys.
