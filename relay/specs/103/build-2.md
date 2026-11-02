# #103 build-2 (easel): the screens, the bar, menus, keys, autosave, the end screen, the sound bus

You turn the page into an app of screens. You build:
- the start, pick, play and end screens, with `main.js` routing between them;
- the top bar with every civ's Standing;
- the Game, View and Debug menus;
- the keys;
- autosave, Continue and Load;
- the end screen as it reads before R;
- the sound bus.

You also write the touch-point scripts for them. The panel is build-3's: you give it its place
at the bottom, but leave its contents as they are. Read `spec.md` items 1–3, 6–14, 20, 22–23 and
the touch-point table, and build-1's summary. Its view model (`model.js`), layers
(`map/view.js`, `map/layers/`), atlas and driver are what you build on.

## What to build, and where

1. **Markup and styles.** `www/index.html` holds one `<section>` per screen (start, play, end;
   the pick sits inside play, over the map), and its styles move to `www/style.css`. The play
   screen's regions, top to bottom: the bar; the map with the feed column on its right; a bottom
   panel slot across the map's width.
   - Move today's `#detail` into the bottom slot, unchanged inside. Build-3 rebuilds it.
   - Phone width (under 760 px) is spec item 20, as far as the bar and feed go. The feed becomes
     a drawer over the map, closed by default.

2. **The router** (`main.js`, spec item 2). It loads the wasm, the atlas and the sounds, then
   routes. Each screen module exports `show(args)` and `hide()`.
   - `?size&seed` (with `run=1`, `play=1`, `speed`) goes straight to play.
   - A bare URL opens start.
   - Starting a game no longer rewrites the URL.
   - `window.game` keeps what the scripts read (`view`, `layout`, `map`, `feed`, `notes`, `save`,
     `border`) and gains `screen()`, `model()`, `sound` and `autosave()`.
   - `body.dataset.screen` names the screen.

3. **Start** (`screens/start.js`, spec item 7):
   - the title and the vision line;
   - New game: size, seed, 🎲 and Start;
   - Continue, with the autosave's line, greyed when there is none;
   - Load a file….

   An old-version save, autosaved or a file, shows the engine's error there. Start begins the
   pre-game at `fast` on the play screen (spec item 8). The bar there shows the pre-game counts
   and **Skip to the pick**.

4. **The pick** (`screens/pick.js`): today's `pickCard`, `showCiv` and `chooseCiv`, moved, over the
   map.

5. **The top bar** (`www/bar.js`, spec item 11).
   - Before the pick: the pre-game counts.
   - After it: the turn, then a chip per civ, then End turn, Play/Pause, speed, the menus, and
     the Feed toggle with its count.
     - Each chip shows the swatch, name and Standing %, with yours outlined and marked "you", and
       ▲ on the leader.
   - A chip click selects that civ's capital and centres the map on it.
   - Tooltips: every number has one, in the wording of spec item 11.
   - Today's civ bar (people, store, warriors) goes. Those numbers move to the civ's panel in
     build-3, and stay reachable meanwhile through today's capital panel.

6. **Menus** (spec item 12):
   - Game: New game, Save to file, Load a file, Standing so far, and the autosave line.
   - View: Territories, Festival routes, Hidden country (H), Legend.
   - Debug: Fertility, Hidden costs, Worn land.

   Each toggles a layer by name (`MapView.setLayer`). Defaults: festival routes and every Debug
   view off; territories on (build-4 may change that).

7. **Keys** (`screens/play.js`, spec item 13):
   - Space ends the turn, or pauses while playing, and does nothing at the pick;
   - P, U and H as now, with P/U also painting the civ's claim when the capital is selected
     (today's `setPainting` has the modes);
   - Escape steps back in spec item 13's order;
   - the arrows, + − and 0 as now.

   Ignore keys when an input has focus, and blur a button after its click.

8. **Autosave** (`www/save.js`, spec item 14).
   - Write `localStorage['sansfiat.autosave']` after each turn the player ends and each action.
   - While Play or Skip runs, write at most once a second, and once when it stops.
   - The entry holds `{version, saved, seed, size, turn, civ}`, the save, and the model's history.
   - A write that throws turns autosave off with a line in the Game menu.
   - Continue and Load a file go through `Sim.load`, rebuilding the model and the feed.
   - Save to file and Load a file work as today.

9. **The end screen** (`screens/end.js`, spec item 10).
   - It takes `{over, history, civs, player}`.
   - Before R the header reads "The game goes on: turn N".
   - Standing over the game is an inline SVG, a line per civ in its colour, the player's thicker,
     each line's last value labelled.
   - Play on and New game.
   - The Game menu's Standing so far opens it.

10. **The sound bus** (`www/sound.js`, `www/assets/sounds.json`, spec item 6).
    - `sounds.json` has every kind in `feed.js`'s `KINDS`.
    - The bus hears each new feed batch and tries each kind's file once, staying silent on a
      miss with no page error.
    - It counts `heard` by kind, and is muted until the first click.
    - Ship no sound files.

11. **Scripts** in `www/play/` (spec item 22, the touch-point table, rows 1–14 and the sound
    half of 25). Each exits non-zero when its touch point is missing:
    - **`start.mjs`**:
      - a bare URL shows the start screen;
      - New game with a seed starts the pre-game, and Skip reaches the pick;
      - 🎲 changes the seed;
      - Load a file (a save written by `sansfiat play` in a temp dir, or one the script saved
        from the page) lands in play at its turn;
      - a save with `version` edited to 2 shows the version error and stays on start.
    - **`pick.mjs`**: three cards; Show highlights a civ; Lead selects its capital; Watch leaves no civ led.
    - **`bar.mjs`**:
      - three Standing chips that sum to at most 1, yours marked;
      - a tooltip on every number in the bar;
      - End turn and space each end a turn;
      - Play runs turns, and space pauses it;
      - speed changes the rate;
      - each menu opens and Escape closes it.
    - **`save.mjs`**:
      - after five turns the autosave holds turn N;
      - `reload()`, then Continue, gives turn N with the same `Sim.digest()`;
      - Save to file, then Load a file, gives the same digest;
      - with `localStorage.setItem` made to throw, the game plays on and the Game menu says
        autosave is off.
    - **`end.mjs`**: Standing so far shows a line per civ and "The game goes on"; Play on returns to play at the same turn; New game reaches start.
    - **`views.mjs`** (build-1's): add the View and Debug menus. Debug is off by default and each
      item toggles its layer.
    - **`art.mjs`** (build-1's): add the sound bus. After ten turns `game.sound.heard` counts
      kinds, and every kind in `KINDS` is in `sounds.json`.

12. The old `www/play-*.mjs` scripts: keep them passing (change a label or selector where the bar
    moved). Build-3 retires them.

13. `engine/CLAUDE.md`: the `www` rows for your files.

## Done

- `./check.sh` passes, with every script in `play/all.mjs`.
- Shots of start, the pick, play and end at laptop size go in
  `relay/specs/103/screens/build-2-*.png`, at half scale. Look at them and say in your summary
  what reads badly; the canvas takes it from there.

Build-3 relies on:
- the bottom panel slot;
- `screens/play.js`'s selection state and `order()`;
- Escape's order, with the armed click and painting as steps;
- `bar.js`'s chip click selecting a civ;
- the feed column and drawer.
