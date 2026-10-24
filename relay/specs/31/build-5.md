# Build 5: the page

Fifth of five builds for #31. Read `spec.md`, `docs/design/map.md` ("On the page"),
`docs/design/scouting.md` ("What the player sees", for the sort of feed it will carry later),
the earlier steps' results, and the current page, `engine/www/index.html` + `engine/www/main.js`
(plain JS, an ES module, no framework, no bundler; `./build-web.sh` builds `www/pkg`).
Work in `engine/`; checks: `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D
warnings`, `./build-web.sh`, `./cross-build.sh`. Serve with
`python3 -m http.server -d www` and look at it with headless Chrome (below).

The wasm side (`Sim.events(from)`, `sizes()`, the extra tile fields in `state()`) exists from
build-4; if something the page needs isn't exposed, add it to `web/src/lib.rs`, additively.

## What to build

The page fills the window and the map takes the space. Rewrite `index.html`/`main.js` (split
`main.js` into a few ES modules if it gets long: `map-view.js`, `feed.js`; fine, same folder).

1. **Layout.** Full viewport, no page scroll: a **top bar** across the top, the **map** filling
   the rest (a `<canvas>` resized to its box on window resize, with device-pixel-ratio
   handling), the **event feed** as a column on the right (collapsible with a button), a small
   **legend** (collapsible) bottom-left of the map.
2. **Top bar.** Turn, phase (`Pre-game` while `end` is unset; `Pre-game over` when set, with
   the end turn and whether it was by centres or the limit, and `+N topped up`), people, clans,
   markets, hearths, centres (`n / 3`), the **End turn** button, **play/pause** and a **speed**
   control (slow 1/s, normal 4/s, fast 20/s, max = as fast as frames allow), a **Run to end**
   button, and the **new map control**: a size select (small / medium / large, from `sizes()`),
   a seed number input (with a dice button for a random seed) and **New map**. Save / Load
   stay (as they are today, in a menu or the bar).
3. **Pan and zoom.** Drag to pan, wheel to zoom about the cursor, `+`/`−`/`fit` buttons, and
   keys (arrows / `+` / `-` / `0`). Zoom range from "whole large map fits" to "a tile is ~60
   px". Pinch zoom on touch. Opening a new map fits it to the window. Only visible tiles are
   drawn; the static terrain layer is drawn to an offscreen canvas and redrawn only on pan,
   zoom, resize, new map or a view toggle; per-turn things (territory, rounds, markets,
   clans) draw on top. At the large size playing at "fast" must keep up (profile it; target a
   frame under ~30 ms with the large map on screen at default zoom; parse state JSON once per
   turn and reuse the typed layout).
4. **Reading the map at a glance.** Distinct treatments, each in the legend:
   - ocean vs **lake** (different blues); coast sand; plains, forest, hills as now; **mountain**
     grey with a peak mark; **desert** pale sand with a stipple; **marsh** (delta) teal-olive
     with reed marks;
   - **rivers**: a blue line from each river tile's centre to its `flow` tile's centre, width
     growing with `river` (the upstream count), drawn above the terrain so they join and run
     into the sea;
   - **floodplains**: a bright green wash with a thin darker border (and the delta's marsh);
   - **closed country**: a coloured outline around each zone's outer edge (valley gold, sealed
     red-brown dashed, neck purple); **pass** tiles marked with a notch/dot; and a sealed
     valley's bridgeable wall mountain is *not* shown (hidden stays hidden, except in the
     debug view).
   - Territories, rounds, trails, hearths, markets and centres as the page has today, restyled
     so they still read over the new terrain. Clans: circle at home sized by people, red ring if
     starving.
5. **Event feed.** Newest first, each line "turn · text" with a small icon/colour by kind
   (division, starving, moved, market formed/faded, hearth, centre, end). Keep a cursor into
   `sim.events(from)`: after every turn append only the new ones (cap the DOM at the latest
   300 entries). A **filter** by kind is welcome but optional. **Clicking an event centres the
   map on its tile** (smoothly or at once), zooms in if far out, and **selects what it's
   about** (the tile; if it's a clan or market event, shows that clan's / market's detail
   panel and highlights it). After `Load`, the feed is rebuilt from the replay (`events(0)`).
6. **Tooltip on hover.** After a short delay, a floating card near the cursor: the tile's
   terrain name (plus river / floodplain / delta / pass / lake as it applies, and zone name),
   the clan holding it (the clan whose round it's on, or whose territory it's in, else
   "nobody"), fertility, and market/hearth if any. It follows the cursor, stays on screen, and
   disappears while dragging. Click selects a tile as today and the detail panel (what the
   current side panel shows) opens in the feed column's top, with the feed below.
7. **Debug views stay**, in a "view" menu: fertility, territories on/off, rounds on/off, markets
   on/off, and a **hidden costs** view (shows `trail_cost` / `span` as small numbers on forest,
   hills and mountain tiles, and tints bridgeable mountains). They default to the play view.
8. **URL parameters** for tests: `?size=medium&seed=7` starts that map; `&play=1` starts
   playing; `&speed=fast|max`; `&run=1` runs to the end. The page sets `document.body.dataset.turn`,
   `dataset.phase` (`pregame` / `ended`) and `dataset.centres` on every refresh, so headless
   Chrome scripts can wait on them.
9. Keep it accessible enough: buttons are `<button>`, the select/inputs have labels, the
   feed is a list, focus rings visible.

## Checking it

There's no browser test harness. Do this yourself and record it in your result:

- Serve `www/`, then with headless Chrome take screenshots (`google-chrome --headless=new
  --no-sandbox --disable-gpu --window-size=1600,900 --screenshot=/tmp/x.png
  --virtual-time-budget=20000 'http://localhost:8000/?size=medium&seed=3&run=1'`; or drive it
  with a small Node script via the DevTools protocol if virtual time doesn't advance the wasm
  loop). Look at the screenshots with the Read tool: the three sizes at the start and at the
  end of the pre-game; zoomed on a river, a delta, a sealed valley, a pass. Fix what looks
  wrong.
- Confirm in the DOM dump (`--dump-dom`) that `body[data-phase="ended"]` and `data-centres="3"`
  appear after `run=1` for each size, and that the feed has entries including `centre` and the
  end event.
- A Node script (`engine/www/check.mjs`, committed, run with `node`) that loads the wasm
  build as `cross-build.mjs` does, makes a medium world, runs to the end, and checks the data
  the page relies on: `events(0)` parses, every event's `tile` is a valid index, `sizes()`
  has three entries, tiles carry `river/flow/zone/lake/flood/pass`. Add it to `ci.yml`'s
  engine job after `./cross-build.sh`.
- Update `engine/CLAUDE.md`'s `web/` row for the page.

Commit when checks pass: "#31 build-5: the full-screen page".
