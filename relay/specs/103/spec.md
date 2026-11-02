# #103 The page becomes the game: spec

## What it delivers

The page becomes a game you start, play and come back to, laid out as `page.md` says. A
**start screen** offers a new game (size, seed, a random seed), Continue (the autosave) and Load
(a file). The pre-game plays on the map, then **the pick** comes up over it as it does now. The
**play screen** has the map filling the window, a **top bar** with the turn, every civ's Standing
(yours marked), End turn, Play with speed and the Game, View and Debug menus, one **bottom
panel** in a single shape for whatever is selected, ending in one button row, and the **feed
and alerts** on the right. Space ends the turn, and **Move to…** arms a click. The game
**autosaves** to the browser after every turn. An **end screen** shows Standing over the game as
a line per civ. Until R adds a way to win, you open it from the Game menu, and it says the
game goes on.

Under it, the page's code takes the shape it grows in: the screens are modules, and `main.js`
only routes between them. A view model is built from `Sim.view()` once per change, and every
layer and panel reads it. The map draws in one layer per mechanic, and a single panel
component renders every selection. One atlas holds the placeholder art and one bus carries
sound. `play.mjs` is the only driver, with one script per touch point, and CI runs them all.

No rule changes. The engine gains only what the view needs: each civ's Standing in `Sim.view()`
and the save version in `rules()`.

**The question:** does the page put every touch point `page.md` lists in front of the player,
at phone, laptop and wide sizes, without changing a rule? It is a page piece, so it moves no
Standing, doubt or motion. Its scores are these:

- **Decisions** (the score this piece can move): the player levers that have a button or a key
  on the page. A lever is each `Order` kind (11), `setGift`, `setKeep`, `chooseCiv` and
  `endTurn`, so 15 in all. Main gives buttons for 14; `move` is right-click only, and no lever
  has a key. The target is all 15 with a button, plus space for `endTurn` and P/U for painting
  both a zone and a claim.
- **Touch points**: the list in "Touch points" below, each reached by its script on the page.
  Main reaches 15 of 26 (counted from main's code in that table). The target is 26 of 26, every
  script passing in CI.
- **Health**: `stats::digest` unchanged for every seed. That means no `SAVE_VERSION` bump,
  `the_save_version_pins_the_rules` untouched, `cross-build.sh` passing, and `sweep --judge`
  identical to main's (its score lines, not its timings).
- **By eye**: screenshots of start, pick, play with a selection, and end, at phone (390×844),
  laptop (1280×800) and wide (1920×1080). Each set sits beside main's control shots, which this
  spec commits in `screens/main-*.png` (small seed 2, at the pick and five turns after leading
  Red).

The control is main's page: its code for the counts, its shots for the eye, and its digests
and judge for health.

## Shape: Easel and Canvas

I took the shape the piece suggests. `page.md`, the piece's Decided list and this spec settle
the structure, screens, panel rows, keys, autosave, atlas layout and scripts, so those are the
easel. What can't be settled before it exists is layout and feel: whether the bar reads at a
glance, whether the panel stays out of the map's way, and how the phone reads. Only playing it
and reading the screenshots settles those, and that is the canvas's work.

I cut the easel by area into three steps. The map's side and the page's side touch different
files, and the panel is the one component everything selected passes through.

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | easel | The map side: `Sim.view()`'s Standing, the view model, the map split into layers, the atlas and its placeholders; the driver's new powers, `play/all.mjs` in `check.sh`, and the `art` and `views` scripts | Everything after reads the model and draws through the layers |
| build-2 | easel | The page's side: screens and the router, the top bar, the menus (Game, View, Debug), keys, autosave, Continue and Load, the end screen, the sound bus; the `start`, `pick`, `bar`, `save` and `end` scripts | Needs the model; makes the regions the panel sits in |
| build-3 | easel | The panel: one component, the rows per kind, Move to…, Raise… and Disband…, selection on the map, the feed on the right with alerts until seen; the `camp`, `scout`, `civ`, `select`, `feed` and `layout` scripts; the old `play-*.mjs` scripts go | Needs the play screen's regions |
| build-4 | canvas | Layout and feel at three sizes: play every touch point, read the shots, fix what reads badly; the final screenshots | Needs everything under it |

## Touch points

Each has a script in `www/play/` that exits non-zero when the touch point is missing or the page
reports an error. The table says which script covers each, which build writes it, and whether
main has it.

| # | Touch point | Script | Build | Main |
|---|---|---|---|---|
| 1 | Start: new game with size, seed and a random seed | `start` | 2 | in the bar, no screen |
| 2 | Start: Continue the autosaved game | `save` | 2 | no |
| 3 | Start: Load a file; an old save fails with its version | `start` | 2 | Load in the Game menu |
| 4 | The pre-game plays on the map, with Skip to the pick | `start` | 2 | yes (Run to end) |
| 5 | The pick: three cards, Show, Lead, Watch | `pick` | 2 | yes |
| 6 | Map: pan, zoom, fit | `views` | 1 | yes |
| 7 | Top bar: every civ's Standing, yours marked, a tooltip on each number | `bar` | 2 | no |
| 8 | End turn by button and by space | `bar` | 2 | button only |
| 9 | Play, pause and speed | `bar` | 2 | yes |
| 10 | Game menu: new game, save to file, load a file, Standing so far | `save` | 2 | save and load |
| 11 | View menu: territories, festival routes, hidden country (H), legend | `views` | 1 (map), 2 (menu) | yes, mixed with debug |
| 12 | Debug menu: fertility, hidden costs, worn land; off by default | `views` | 2 | no |
| 13 | Autosave each turn, guarded when storage is refused | `save` | 2 | no |
| 14 | End screen: Standing over the game, play on, new game | `end` | 2 | no |
| 15 | Bottom panel, one shape, for a camp | `camp` | 3 | right-side panel |
| 16 | Move to… arms a click; right-click still moves | `camp` | 3 | right-click only |
| 17 | Paint and Unpaint a zone by button, by P and U, and by dragging | `camp` | 3 | yes |
| 18 | Forage, Pillage, Resupply, the switches, Disband… | `camp` | 3 | yes |
| 19 | The scout's route: Move to… and Route from…, Clear | `scout` | 3 | yes |
| 20 | The civ's row: Raise… (warriors, guard), Gift, Keep, Claim and Unclaim (P, U) | `civ` | 3 | yes, P/U for the camp only |
| 21 | A clan's, a festival's and a tile's panel; clicking cycles a stack; Escape clears | `select` | 3 | yes |
| 22 | The feed on the right, collapsible; clicking an item goes there | `feed` | 3 | yes |
| 23 | Alerts stay until seen | `feed` | 3 | no (the last five show) |
| 24 | The latest reasons in the panel | `feed` | 3 | yes |
| 25 | Art from one atlas; sounds mapped by event kind, silent | `art` | 1 (atlas), 2 (sound) | no |
| 26 | Three sizes: nothing clipped or overflowing, and the screenshots | `layout` | 3, then 4 | no |

Main reaches 1, 3, 4, 5, 6, 8, 9, 11, 15, 17, 18, 19, 21, 22 and 24 in some form. Counting the
mixed ones, that is 15 of 26.

## Settled: the code's shape

1. **Files.** The page's code is laid out as follows. A file past about 800 lines splits (`docs/architecture.md`).

   ```
   www/index.html          one <section> per screen; no inline styles
   www/style.css           the styles, with the size breakpoints
   www/main.js             loads the wasm, the atlas and the sounds, then routes: start, play, pick, end
   www/screens/start.js    new game, Continue, Load
   www/screens/pick.js     the cards over the map
   www/screens/play.js     the play screen: wires the bar, the map, the panel and the feed; the turn loop; keys
   www/screens/end.js      Standing over the game, play on, new game
   www/model.js            the view model
   www/bar.js              the top bar
   www/panel.js            the panel component
   www/panels.js           a selection to the panel's shape, per kind (select.js becomes this)
   www/feed.js             the feed and alerts
   www/save.js             autosave, Continue, save and load files
   www/sound.js            the sound bus
   www/atlas.js            the atlas loader
   www/map/view.js         MapView: pan, zoom, picking, the layer order (map-view.js becomes this)
   www/map/layers/*.js     one file per layer (item 4)
   www/assets/atlas.json   every cell, its place and size: the artist's brief
   www/assets/atlas.png    the cells; placeholders until the paid set
   www/assets/placeholders.js   draws each placeholder cell on a canvas
   www/assets/draw-atlas.mjs    renders placeholders.js in headless Chrome into atlas.png
   www/assets/sounds.json  each feed event kind to a file name
   www/play.mjs            the one driver
   www/play/<touch>.mjs    one script per touch point; www/play/all.mjs runs them all
   ```

2. **The router.** `main.js` loads the wasm, `atlas.json` with its image, and `sounds.json`, and
   shows one screen at a time, calling each screen module's `show(args)` and `hide()`.
   `?size=…&seed=…` in the URL goes straight to the play screen with that map, and `run=1` runs
   the pre-game at max speed, so the scripts' URLs keep working. A bare URL opens the start
   screen. Starting a game no longer writes the seed into the URL, so a reload lands on the
   start screen, where Continue is.

3. **The view model** (`model.js`). `build(sim, layout, rules, prev)` is called after every change:
   a turn, an order, a load. It returns `v` (the parsed `Sim.view()`) plus everything the layers
   and panels used to work out for themselves:
   - each clan's civ;
   - units by tile, as stacks;
   - each tile's holders;
   - each civ's claim counts, Standing, rank, and whether it is the player's or leads;
   - each festival's walkers;
   - the player's units;
   - the Standing history: `[turn, {civ: standing}]` for every turn the page has seen since the pick.

   Layers and panels read only the model. The `Sim` is still called for orders, `check`,
   `notes`, `route`, `path` and `tile` (per selection or hover, not per frame).

4. **Layers.** `MapView` keeps pan, zoom, picking and the offscreen terrain layer, and draws an
   ordered list of layers. Each layer is a module exporting
   `{ name, static?, draw(ctx, model, view) }`. `drawCivs` splits as the piece says, and the rest
   splits by mechanic, in this order:
   - terrain (atlas);
   - rivers;
   - hidden country;
   - debug fertility, debug hidden costs and debug worn land;
   - territories;
   - trails;
   - festivals and hearths;
   - festival routes;
   - land (the civ's border, held and paper);
   - held ground (a camp's held tiles);
   - lines;
   - leanings (the tint on unaligned clans);
   - clans;
   - units;
   - routes (a unit's route and the scout's plan);
   - the overlay (the selection, painting, highlight).

   A View or Debug toggle switches a layer on or off by name.

5. **The atlas** (`atlas.js`, `assets/`). This is the cell layout I settled, in `atlas.json` as
   `{ image, cells: { name: { x, y, w, h, note } } }`:
   - **Terrain**, `terrain.<name>.<0-2>` for sea, lake, coast, plains, forest, hills, mountain,
     desert and marsh: three variants each, 128×148 px. Each cell is a pointy-top hex of
     circumradius 74, centred, transparent outside it. The variant is picked by the tile's index, so
     a map always looks the same.
   - **Overlays**, `overlay.floodplain` and `overlay.desert-dots`, 128×148, translucent.
   - **Units**, `unit.camp`, `unit.scout`, `unit.guard`, 64×64. Each has a `.mask` twin, `unit.camp.mask`
     and so on, which the loader tints in the civ's colour (once per colour, into an offscreen
     canvas) and draws under the figure. The stack's count is drawn by code.
   - **Icons**, 64×64: `icon.capital` (also masked), `icon.festival`, `icon.centre`, `icon.hearth` and `icon.pass`.

   Rivers, trails, borders, routes, lines and clans stay vector, because their shape is the
   rule. `placeholders.js` draws each cell from today's flat colours and shapes.
   `draw-atlas.mjs` renders those cells into `atlas.png` through `play.mjs`'s Chrome, and the
   result is committed. That is "placeholder cells drawn by the build": paid art replaces
   `atlas.png`, and `atlas.json` is the brief. The atlas is laid out in rows by kind, 1024 px
   wide. Each cell's `note` says what it is ("plains, variant 1: pointy-top hex, circumradius 74,
   transparent outside").

6. **The sound bus** (`sound.js`, `assets/sounds.json`). `sounds.json` maps every feed event kind
   in `feed.js`'s `KINDS` to a file name, such as `"clanJoined": "clan-joined.ogg"`. The bus hears
   each new feed batch, and for each kind tries its file once. A missing file makes that kind
   silent from then on, with no page error. No sound files ship. The bus counts what it heard
   (`game.sound.heard`) so a script can see it ran. Sound is muted until the player's first click,
   as browsers require.

## Settled: the screens

7. **Start.**
   - The title, and the vision in a line: "A hex 4X with no money, set among the first
     civilisations."
   - **New game**: size, seed, a 🎲 for a random seed, and Start.
   - **Continue**: the autosave's line, such as "Red, turn 312, small seed 2". It is greyed with
     "No saved game" when there is none.
   - **Load a file…**: any save, from the page or `sansfiat play`.

   A save from another version shows the engine's error on the start screen ("this save is
   version 2; this build reads version 3"), and the page stays on the start screen.

8. **The pre-game** plays on the play screen. Start begins it at `fast`, and the bar has
   **Skip to the pick** (today's Run to end). The bar shows the pre-game's counts, as `map.md`
   asks: turn, phase, people, clans, festivals, hearths, centres. **The pick** comes up over the
   map when the phase is `choose`, as now.

9. **Play** (`screens/play.js`). Top to bottom: the bar, then the map with the feed on its right,
   then the bottom panel across the map's width, when something is selected. The map takes all
   space the others don't. The zoom buttons and the legend stay on the map. The new-map controls
   leave the bar for the start screen.

10. **The end screen** (`screens/end.js`). Its header reads one of two ways. Once R adds a
    finished phase with a winner and why, it says won or lost and how. Until then it reads "The
    game goes on: turn N". Under the header:
    - Standing over the game, as a line per civ in its colour from the pick to now (an inline SVG
      from the model's history), with the player's line thicker and each civ's last value labelled;
    - **Play on**, back to the play screen;
    - **New game**, to the start screen.

    The Game menu's **Standing so far** opens it. `end.js` takes
    `{ over: null | { winner, why }, history, civs, player }`, so R only fills `over`.

## Settled: the bar, menus and keys

11. **The top bar**, after the pick:
    - the turn;
    - one chip per civ: its swatch, name and Standing as a percentage. Yours is outlined and marked
      "you". The civ that leads has a ▲. When watching, no chip is marked.
    - End turn;
    - Play/Pause and the speed;
    - the Game, View and Debug menus;
    - the Feed toggle, with the unread-alert count.

    Clicking a chip selects that civ (its capital) and centres the map on it. Each chip's tooltip
    reads like "Standing 9.1%: Red's clans and units 8.0%, unaligned clans on its land 1.1%; 2nd
    of 3". The turn's tooltip holds the world's counts: people, clans, festivals, hearths.
    Before the pick, the bar shows item 8's counts instead of the chips. Every number in the bar
    has a tooltip saying what it is.

12. **Menus.**
    - **Game**: New game (to the start screen), Save to file, Load a file, Standing so far.
      Autosave's state also shows here ("Autosaved turn 312", or "Autosave off: this browser
      refuses storage").
    - **View**: Territories, Festival routes, Hidden country (H), Legend.
    - **Debug**: Fertility, Hidden costs, Worn land, all off by default.

    The "Civs" toggle and "Markets" go. A civ's land, its units and the festivals are the game,
    not options.

13. **Keys.**
    - **Space** ends the turn. While playing, it pauses instead. It does nothing at the pick or
      on another screen.
    - **P** and **U** paint and unpaint whatever the selection paints: a camp's zone or the civ's
      claim.
    - **H** shows hidden country.
    - **Escape**, in order: closes an open menu, then disarms an armed click, then stops painting,
      then clears the selection.
    - The arrows, + and − and 0 still pan, zoom and fit.

    Keys are ignored while an input has focus. A button blurs after a click, so space never
    re-presses it.

## Settled: autosave

14. After every turn the player ends and every action they take, the page writes `localStorage`
    key `sansfiat.autosave`, which holds:
    - `{ version, saved, seed, size, turn, civ }`;
    - the save (`Sim.save()`: config and log);
    - the Standing history (item 3).

    While Play or Skip runs, it writes at most once a second and once when the run stops. A
    write that throws (storage refused or full) turns autosave off for the session and says so
    in the Game menu, and the game plays on. Continue loads it with `Sim.load` and puts back the
    history. A file load starts with an empty history from its turn. Save to file and Load a file
    work as now.

## Settled: the panel

15. **One component** (`panel.js`). `render(el, shape)` draws a shape
    `{ name, swatch?, doing, numbers: [{label, value, tip}], list?: {label, rows}, reasons:
    [{turn, text}], buttons: [...] }` in this order:
    - the name with its civ's swatch, and the line of what it is doing;
    - the numbers as label–value pairs, each with a tooltip;
    - an optional short list (units here, leaning clans, a festival's clans), each row selecting
      its subject;
    - the latest reasons (`rules.notes_shown`), newest first;
    - **one button row**.

    A button is one of:
    - `{label, action}`: sent through `order`, greyed with `Sim.check`'s reason as its tooltip and
      shown in a line under the row when refused;
    - `{label, arm}`: arms a map click;
    - `{label, paint}`: P or U;
    - `{label, stepper}`: Raise… and Disband… open a count in the row, from 1 to the most allowed,
      with Go and Cancel.

    A button the kind can never take is left out. One it can't take now is greyed with why.
    The panel sits at the bottom, below the map, and is gone when nothing is selected.

16. **The rows per kind** (the designer's to settle; settled here). "Name: doing" are the first
    two lines, then the numbers, the list if any, the reasons, then the buttons.

    - **Camp**
      - **Name and doing:** "Red's camp". Doing is one of:
        - "Marching to 3, −4: 2 turns";
        - "Holding 5 tiles round 3, −4";
        - "At the capital";
        - "Cut off: its line can't feed it";
        - "Hungry 2 turns: loses a warrior at 3".
      - **Numbers:**
        - warriors;
        - pack and its turns of food;
        - line: steps, cost, carry a turn;
        - zone: tiles of cap, held, thinnest guard.
      - **List:** units here.
      - **Buttons:**
        - Move to… (and Stop while marching);
        - Paint (P) and Unpaint (U);
        - Forage, Pillage, Resupply;
        - Auto-forage and Auto-supply, as toggles showing on or off;
        - Undeploy, while deployed;
        - Disband….
    - **Scout**
      - **Name and doing:** "Red's scout". Doing is "Searching 2, 3 → 9, −1: known cost 14, possible 9"
        or "Idle at 2, 3".
      - **Numbers:**
        - people;
        - pack and its turns of food;
        - known and possible cost;
        - unrevealed tiles on the possible route.
      - **Buttons:**
        - Move to…, which arms the route's end from the scout's tile, as Route to here does now;
        - Route from…, which arms the start (today's Start here);
        - Clear route;
        - Disband…, greyed when refused.
    - **Guard**
      - **Name and doing:** "Red's guard", "Guarding 24 claimed tiles, holding 8".
      - **Numbers:**
        - people;
        - hunger;
        - claimed, held and paper tiles;
        - thinnest held guard.
      - **Buttons:** Disband….
    - **Civ** (the capital, or its bar chip)
      - **Name and doing:** "Red". Doing is "You lead Red: Standing 9.1%, 2nd of 3", or for a rival
        "Standing 9.1%, 2nd of 3".
      - **Numbers:**
        - Standing, with its two parts in the tooltip;
        - people;
        - clans;
        - store of cap, with its goods in the tooltip;
        - kept, posted and given last turn;
        - joined since founding;
        - claimed, held and paper tiles;
        - gift and keep.
      - **List:** leaning clans, then its units.
      - **Buttons:**
        - Raise…, a stepper with a choice of warriors or guard;
        - Gift − and Gift +;
        - Keep −, Keep + and Keep default;
        - Claim (P) and Unclaim (U).

      Rivals' civs have no buttons.
    - **Clan**
      - **Name and doing:** "Clan 140", with its civ's swatch. Doing is "Walks to festival 12",
        "No festival in reach" or "Starving".
      - **Numbers:**
        - people;
        - fed;
        - territory tiles;
        - the route's cost a year;
        - its leanings, as shares of the joining line.
      - **Buttons:** none, since a clan is told nothing.
    - **Festival**
      - **Name and doing:** "Festival 12" (and "centre"), "14 clans come".
      - **Numbers:**
        - age;
        - traders;
        - activity;
        - prices, in the tooltip.
      - **List:** the clans going.
      - **Buttons:** none. Its walkers' routes are drawn on the map while it is selected.
    - **Tile**
      - **Name and doing:** the terrain, with river, floodplain and pass. Doing is whose it is:
        "Red's land", "Paper of Red", "Clan 140's territory" or "Nobody's".
      - **Numbers:**
        - fertility;
        - each food's stock of cap;
        - trail;
        - tame share, on wild grain.
      - **Buttons:** none.

17. **Selecting.** A left-click picks the first of these on the tile, and the next one on another
    click: the player's units, the capital, the clan whose home it is, the festival there, other
    civs' units, then the bare tile. Selecting a clan or festival is its own kind now, no longer
    a tile with extras.

18. **Move to…** arms a left-click, as Route to here does. The cursor turns to a crosshair, the
    tooltip shows "Move here: cost, turns", and the click sends `move`. Escape or a second press
    disarms it. Right-click still moves a camp.

19. **The feed and alerts** stay on the right, newest first, collapsible, with the filter.
    Clicking an item selects its subject (item 17) and centres the map on it. **Alerts stay until
    seen**: an alert is listed in the alerts box until the player clicks it or selects its
    subject. The box holds every unseen one, scrolling after five, and the Feed toggle counts them.

20. **At phone width** (under 760 px), the feed goes **over the map**, as a drawer from the right
    that is closed by default. The Feed button carries the unseen-alert count. The panel is a
    bottom sheet of at most 45% of the height. Its numbers wrap, and its button row scrolls
    sideways. The bar is one line: the turn, the Standing chips as swatch and percentage, End
    turn, and a ☰ menu holding Play, speed, Game, View and Debug.

## Settled: the engine

21. `Sim.view()` gives each civ `standing`, `standing_clans` and `standing_land`, from
    `stats::standing_parts`. `rules()` gives `save_version`. Nothing else in the engine changes.
    `stats::digest` is unchanged for every seed. `www/check.mjs` checks the Standing: each in [0, 1],
    their sum at most 1, and parts that add up.

## Settled: the scripts and CI

22. **The driver** (`play.mjs`) gains:
    - `SIZES` (`phone` 390×844, `laptop` 1280×800, `wide` 1920×1080) and `open(query, {size})`;
    - `page.reload()`, which keeps `localStorage`;
    - `page.key(name)`, a real key press;
    - `page.file(selector, path)`, through `DOM.setFileInputFiles`;
    - `page.drag(fromTile, toTile)`;
    - `page.tip(selectorOrTile)`, the tooltip's text;
    - `lead(page, {seed, civ})`: to the main game, leading a civ, on small seed 2 by default.

    Each script in `www/play/` takes `--out DIR` for its screenshots (a temp dir by default) and
    exits non-zero when its touch point is missing or the page logged an error. `www/play/all.mjs`
    runs every script and prints a table: the script, pass or fail, its time.

23. **CI.** `check.sh` runs `node www/play/all.mjs` after `check.mjs`. Without Chrome it fails
    under `CI` and skips with a warning otherwise. `ci.yml`'s syntax check covers
    `engine/www/**/*.js` and `*.mjs`. Today the scripts run in about 2–8 s each (measured: the
    pre-game to the pick on small seed 2 takes under 2 s in headless Chrome), so all of them
    should take under three minutes.

24. **The old scripts** (`play-camp`, `play-hold`, `play-lean`, `play-read`, `play-scout`,
    `play-claim`) become `play/camp.mjs` (with the hold), `play/civ.mjs` (lean and claim),
    `play/feed.mjs` (read) and `play/scout.mjs`, keeping what each checked. Build-3 removes them
    once its scripts pass. Until then they keep passing.

## Left unsettled: the canvas (build-4)

How it all reads at three sizes. That covers:
- the bar's density;
- the panel's height and how its numbers lay out;
- whether the map stays the thing you look at;
- what of today's clutter (every clan's territory tint, the centre rings, the festival circles)
  should be off by default or drawn quieter, under "No ring, radius or circle that isn't a rule";
- the phone layout of item 20;
- the end screen's chart;
- the placeholder art's legibility.

The canvas plays every touch point, reads its own screenshots, and fixes what reads badly. It
may change the easel: a row, a default, a breakpoint, an item above. Each such change is a
decision with the shot that showed it. What proves it:

- `node www/play/all.mjs` passes, 26 of 26 touch points.
- The `layout` script passes at all three sizes, with no horizontal scroll, no bar wrapping
  past one line at laptop and wide, no button row clipped, and the map at least 60% of the
  window at laptop and wide with a camp selected.
- A **played game on the page**:
  - lead a civ on small seed 2 (or another retro map, saying why) and play 30 turns using only
    the page's buttons, keys and clicks through `play.mjs`;
  - move the camp with Move to…, paint a zone and a claim, raise a guard, gift, and follow an
    alert;
  - reload mid-way and Continue;
  - save to a file.

  `sansfiat replay` of that file gives the same `digest` as the page's `Sim.digest()`, and
  `sansfiat judge` reads it. The notes go in `canvas.md`.
- The screenshots: start, pick, play with the camp selected, play with the civ selected, and
  end, at phone, laptop and wide, in `relay/specs/103/screens/`, beside main's. Under 2 MB in
  all: the laptop and wide shots at half scale.

## Decisions (the owner can overrule)

- The piece's question is asked in Decisions and touch points, with Health as the band. A page
  piece moves no Standing, so it claims none.
- Design says the top bar shows Standing (`page.md`), and `map.md` says it shows turn, phase,
  people, clans, markets, hearths and centres. Before the pick, the bar shows `map.md`'s counts.
  After it, the bar shows Standing per civ, and the world's counts move into the turn's tooltip.
- The pre-game plays on the play screen from Start at `fast`, with Skip to the pick, rather
  than as a screen of its own.
- Until R, the end screen opens from Game → Standing so far and says the game goes on. Its
  Standing lines come from the page's own history, kept in the autosave, because the engine
  keeps no history until R.
- Design says "Markets, festivals and camps are called by their names". The page calls every
  market a festival, and units the camp, the scout and the guard. Festivals keep their numbers,
  since nothing names them yet.
- Design says "routes on demand". Festival routes are off by default in View, and drawn for a
  selected clan or festival.
- The View menu's Civs and Markets toggles go. A civ's land, units and festivals are the game,
  not an option.
- A clan and a festival are selections of their own, with a panel each and no buttons.
- Escape steps back one thing at a time: a menu, an armed click, painting, then the selection.
- At phone width, the feed is a drawer over the map, closed by default, with the unseen-alert
  count on its button.
- Alerts stay until clicked or their subject is selected, so none is lost to the last-five box.
- The atlas: three variants of every terrain at 128×148, units and icons at 64×64, with masks
  tinted per civ. Rivers, trails, borders, routes, lines and clans stay vector.
- Placeholders are drawn by `placeholders.js` and rendered into a committed `atlas.png` by
  `draw-atlas.mjs`, so paid art replaces one file.
- `sounds.json` names a file for every feed kind, and a missing file is silent. No sound ships.
- A bare URL opens the start screen. `?size&seed` still goes straight to a game, for the
  scripts and for shared links, and starting a game no longer rewrites the URL.

## What the review plays and measures

1. `./check.sh` passes, with `play/all.mjs` in it: 26 of 26.
2. Health: `cargo run --release -p sansfiat -- digest` on small seeds 1–3 matches main's, the
   pins in `cli/tests/save.rs` are untouched, and `SAVE_VERSION` is still 3.
3. The lever count: every one of the 15 has a button (list them), space ends the turn, and P/U
   paint a zone and a claim.
4. Play the page through `play.mjs` for 30 turns as the canvas did, but on another map (small seed
   4), and from the start screen: New game, the pick, Move to…, a claim, a guard, an alert
   followed, Standing so far, a reload and Continue. Then `sansfiat replay` the saved file and
   match its digest. Does the page say what happened, and why?
5. Look at the screenshots at all three sizes beside `screens/main-*.png`. Does the map stay the
   thing you look at? Is there a ring or circle that isn't a rule?
6. The PR description carries the touch-point table (main and now), the lever count, the
   digests, and the screenshots at three sizes.
7. Decided ledgers:
   - `page.md`: the screens, the bar's chips, the panel's rows, Escape's order, alerts until seen,
     the phone drawer, the atlas and sound manifests, the end screen before R;
   - `map.md`: the bar before and after the pick, festival routes on demand.

## Not in this piece

New rules; sound files; first-time hints (M10); paid art; the end rules and Standing history in
the engine (R); anything the AI does.
