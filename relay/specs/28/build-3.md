# #28 build-3: the route view, the page, archiving (ford)

Read `relay/specs/28/spec.md`, Settled 10, 11, 13, and build-2's result.

Deliver:
- `web/src/lib.rs`: the view gives the led civ's revealed tiles, tiles with trail work, and for
  the selected scout its route, known and possible paths and costs (from `units::route_plan`).
- `www/select.js`: a selected scout shows people, pack, its route and both costs, and buttons
  "start here", "route to here" (each arms a map click; the second sends `SetRoute`) and
  "clear route". `main.js` sets `body.dataset` fields scripts can wait on (e.g. `routeState`).
- `www/map-view.js`: revealed marks, known route solid, possible route dashed (as `setLine`
  draws the camp's line), trails as now. The feed shows the scout's events.
- `www/check.mjs` checks the new view data; `www/play-scout.mjs` (like `play-camp.mjs`) leads a
  civ, selects its scout, picks start and target only by clicking the page, ends turns until
  route found or no route, and screenshots into `relay/specs/28/screens/`.
- `engine/CLAUDE.md`: layout rows and commands.
- Archive `scout/`: delete it, leave a `README.md` pointing at tag `prototype/scout`, drop
  `scout` from `.github/workflows/ci.yml`'s `js` matrix (as #29 did for `supply-lines/`).

Done: `engine/check.sh` passes, and `node engine/www/play-scout.mjs` reaches a found route with
screenshots showing both routes and a built trail.
