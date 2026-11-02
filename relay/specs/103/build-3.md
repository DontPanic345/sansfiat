# #103 build-3 (easel): the panel, Move to…, selection, the feed, and the touch-point scripts

You build:
- the one panel component at the bottom, and the rows `spec.md` settles for every kind;
- Move to…, Raise… and Disband…;
- clans and festivals as selections of their own;
- the feed on the right, with alerts that stay until seen.

You then write the scripts for those touch points, plus the three-size `layout` script, and
retire the old `play-*.mjs` scripts. Read `spec.md` items 15–20, 22 and 24 and the touch-point
table (rows 15–24 and 26). Build-1's summary covers the model, layers and driver; build-2's
covers the screens, the bar, keys and the bottom slot.

## What to build, and where

1. **The component** (`www/panel.js`, spec item 15). `render(el, shape, hooks)` draws spec item
   15's shape into the bottom slot, in order: name and doing, numbers (each a label–value pair
   with a tooltip), list, reasons, then one button row. The four kinds of button are action, arm,
   paint and stepper. An action button is greyed when `Sim.check` refuses it, with the reason as
   its tooltip and a line under the row. A stepper (Raise…, Disband…) opens a count in the row,
   with Go and Cancel. The panel hides when nothing is selected.

2. **The kinds** (`www/panels.js`, which `select.js` becomes, spec item 16). There is one
   function per kind, from the model and a selection to the shape: camp, scout, guard, civ,
   clan, festival and tile. Use spec item 16's rows exactly, and its "doing" lines. A rival's
   units and civ show their numbers with no buttons. Keep today's working parts: `whyHtml`'s
   reasons, `leaningList`, the festival's clans, the claim counts (now from the model), the
   camp's line and cut, and the scout's plan.

3. **Selecting** (spec item 17). `candidates` is extended so the cycle goes: the player's units,
   then the capital, then the clan whose home it is, then the festival, then other civs' units,
   then the tile. A selection is `{kind: 'unit'|'civ'|'clan'|'festival'|'tile', ...}`, and
   `keyOf` gives `unit:40`, `civ:3`, `clan:140`, `festival:12` and `tile:812` in
   `body.dataset.selected`. Update the map's focus (`setFocus`, `setFocusMarket`) to read these.
   A festival's walkers are drawn while it is selected, from the model's `walkers`.

4. **Move to…** (spec item 18) arms a left-click as `arm: 'to'` does:
   - a crosshair cursor;
   - the tooltip's "Move here: cost, turns";
   - the click sends `move` for a camp, or `route` from the scout's tile for a scout;
   - Escape or a second press disarms it.

   Right-click still moves a camp. **Route from…** is today's Start here.

5. **The feed** (`www/feed.js`, spec item 19). It stays on the right, newest first, collapsible,
   with its filter. A click selects the item's subject as a new-kind selection and centres the
   map. **Alerts stay until seen**: the alerts box holds every unseen alert, scrolling after
   five. An alert is seen when clicked or when its subject is selected. The Feed toggle counts
   the unseen ones.

6. **Scripts** in `www/play/`. Each exits non-zero when its touch point is missing, and each
   takes `--out`.
   - **`camp.mjs`** takes over `play-camp` and `play-hold`. It checks:
     - the camp's panel has a name, a doing line, numbers with tooltips, reasons, and one row
       holding Move to…, Paint, Unpaint, Forage, Pillage, Resupply, the two switches and
       Disband…;
     - Move to… then a click sets `dest`;
     - right-click moves;
     - P and drag paint a zone, and U unpaints;
     - each action either applies or is greyed with a reason;
     - Disband… opens a count;
     - the hold: a thicker guard marching on a rival camp's tile cuts its line, as `play-hold`
       checks.
   - **`scout.mjs`** takes over `play-scout`: Move to… sets a route from the scout's tile, Route
     from… then Move to… sets one from a picked start, and Clear route clears it.
   - **`civ.mjs`** takes over `play-lean` and `play-claim`, and checks:
     - the bar chip selects the civ;
     - the panel has Standing with its parts in the tooltip, and the claim counts;
     - Raise… for warriors and for a guard;
     - Gift + and −, and Keep;
     - P paints the claim, and the border and counts show;
     - an unaligned clan leans within 30 turns of a raised gift.
   - **`select.mjs`**:
     - a click on a clan's home selects the clan, with a panel and no buttons;
     - a festival selects, listing its clans, with walkers drawn;
     - a bare tile's panel names whose it is;
     - clicking again cycles a stack;
     - Escape steps back through an armed Move to…, painting, then the selection.
   - **`feed.mjs`** takes over `play-read`:
     - the feed lists newest first;
     - collapsing and opening works;
     - clicking an item selects it and centres the map;
     - an alert stays in the box after more turns until clicked;
     - the panel's reasons show for a clan and the camp.
   - **`layout.mjs`** opens each of `SIZES` and shoots start, the pick, play with the camp
     selected, play with the civ selected, and end, as `<size>-<screen>.png` (laptop and wide at
     half scale). It fails on:
     - a horizontal scroll;
     - a bar wrapping past one line at laptop and wide;
     - a button in the panel's row that is clipped and can't be scrolled to;
     - a map under 60% of the window at laptop and wide with the camp selected.

     Record the map's share per size in its output.

7. **Retire** `www/play-camp.mjs`, `play-hold.mjs`, `play-lean.mjs`, `play-read.mjs`,
   `play-scout.mjs` and `play-claim.mjs` once their successors pass and keep what each checked.

8. `engine/CLAUDE.md`'s `www` rows. Make them whole for the piece: every file in `www/`, its
   subfolders, and the scripts. Also the command block's `play` lines.

## Done

- `./check.sh` passes, with `play/all.mjs` reporting every script.
- `node www/play/layout.mjs --out relay/specs/103/screens` writes the shots. Look at them and say
  in your summary what reads badly at each size. That list is where the canvas starts.
- The lever count (spec, the question): list each of the 15 levers and the button or key that
  reaches it, in `relay/specs/103/levers.md`.

The canvas relies on:
- every script passing;
- `layout.mjs` to re-shoot;
- the panel's shapes in `panels.js` as the one place a row changes.
