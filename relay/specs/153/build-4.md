# #153 build-4 (easel): the page

The scout's buttons each show what they did, the panel says why a mountain is left and why the
scout is hungry, the map warns past the line's reach, and every unit has Disband all. Read
`spec.md` items 23–28, and `relay/specs/151/cases.md`'s "Disband all, the page arm". Builds
1–3 are in: `Sim.view()`'s scout row has `dest`, `search` (`next`, `left`, `known`), `wander`
and `hunger`, `Sim.lineFrom(unit, tile)` gives `{cost, carry, eats}` or `null`, `Sim.route(id)`
works for a scout's `dest`, and `Order::Move` takes a scout (`search.md`, `feeding.md`). The
code: `www/panels.js` (`scoutShape`, `campShape`, `guardShape`, `disband`, `switchButtons`),
`www/screens/play.js` (`routing`, `arm`, `armedPick`, the right-click, the tile tooltip),
`www/map/layers/routes.js` and `lines.js`, `www/model.js`, `www/check-stage.mjs`
(`pressAll`, the `expectBug` wrappers), `www/play/scout.mjs`, `www/play/buttons.mjs`,
`www/play.mjs` (its helpers), `docs/design/page.md`.

## What to build

1. **The scout's row** (spec item 23), for the player's scout, in this order: **Move to…**
   (arms `'move'`), **Route from…** (arms `'start'`), **Route to…** (arms `'to'`), then
   **Clear route** while it has a route and **Stop** while it has a `dest`, then Forage,
   Resupply, the two switches, **Disband…** and **Disband all**. `armedPick` sends
   `{type: 'move', unit, to}` for `'move'` (for a camp too: the camp's Move to… arms
   `'move'`, so `'to'` means a route's end and nothing else). It sends `route` from
   `routing.start ?? u.tile` for `'to'`. The hints: "Click the map for where the scout goes;
   Esc cancels"; "Click the map for the route's start; Esc cancels"; "Click the map for the
   route's end (from (x, y)); Esc cancels". Right-click moves a selected scout of the
   player's as it moves a camp, and the tile tooltip's "Move here: cost, turns" line shows for
   a selected scout.
2. **The start mark.** While `routing.start` is set, the routes layer draws a ring on its
   tile in the scout's civ's colour, and the scout's hint reads "Route from (x, y): now Route
   to…". Sending the route or pressing Escape clears it (Escape's order of steps stays as
   `page.md` Decided has it: menu, armed click, painting, then the start mark, then the
   selection). Expose the start on `document.body.dataset.routeStart` so a script can check
   it.
3. **The doing line** (spec item 24): "Searching A → B: known …, possible …" (as now),
   "Going to X: N turns" (from `sim.route(u.id)`) with its march path drawn as a camp's is,
   "At X, foraging", "Foraging at X until fed, then back to its route" while `wander` is set
   and it has an errand, "Waiting on rich ground at X" while idle. A hungry scout's line leads
   with "Hungry N turns: loses a person at {hungry_turns}". A **Hunger** number row reads
   "foraging 0.4, line 0.3 of 2 it eats" from `hunger` (or "fed"), with the tooltip "What
   foraging and the line brought on its last hungry turn, against what it eats".
4. **The Unrevealed row** (spec item 25): value "N to check" from `search.next.length`, plus
   "; M left" when `search.left` has any. A **Left unrevealed** list, a row per mountain:
   "(x, y): 31.0 through it, can't beat the known 28.0". Clicking a row centres the map on it
   (the list rows' existing behaviour for units). The row's tooltip: "Mountains the search
   won't reveal: the cheapest route through each costs at least the route it knows".
5. **Disband all** (spec item 26): `disband(u, rules)` returns two buttons, **Disband…** with
   its counts (for a camp, one option per fighter kind it has; for a scout or guard, `people`
   with cap `floor(people)`, and the stepper only when that is 2 or more) and **Disband all**,
   sending `unitOrder(u, {type: 'disband'})` with no `n`. Its `all` option goes. The camp, the
   scout and the guard all show both. In `check-stage.mjs`, `pressAll` presses Disband all
   (its `count` argument goes). The four `expectBug` wrappers go, and their messages stay as
   the assertions' own.
6. **The warnings** (spec item 27): with `'move'` or `'to'` armed and a scout selected, the
   tile tooltip adds "Past the line's reach: its line there carries {carry} of the {eats} it
   eats" when `sim.lineFrom(u.id, i)` has a carry under `eats`, or "Past the line's reach: no
   line reaches it" for `null`. The engine's note shows in the reasons after the order.
7. **`www/play/scout.mjs`** (spec item 28), on small seed 2 as now. Press Move to… and click
   `TARGET`. The scout's `dest` is `TARGET`, its route is `null`, the doing line says "Going
   to", and the march path is drawn; End turn until it stands there (at most 15), and the line
   says "At". Press Route from… on the capital: `dataset.routeStart` is the capital and a
   screenshot shows the ring. Press Route to… on `TARGET`: the route is `[capital, TARGET]`
   and the known path is drawn. Press Clear route: the route is `null`. Hover a tile ten
   steps out with Move to… armed: the tooltip says "Past the line's reach". Save a screenshot
   at each step into `--out`.
8. **`www/play/buttons.mjs`**: the scout's cases press Move to… (the `dest` is set), Route
   from… then Route to… (the route is set from the start), Stop (the `dest` clears), Clear
   route, Disband all on the camp, the scout and the guard (each gone, its people in its
   clans), and Disband… `people` 1 on a scout of two (it keeps one). The existing
   "scout, routed" setup arms `'to'` (Route to…), not Move to….
9. `www/play/army.mjs`, `camp.mjs`, `game.mjs` and `select.mjs` press Move to… (on a camp, or
   the scout as a route): each still passes, with a scout's route pressed as Route to….

## Proof and record

`engine/check.sh` green (it runs `check.mjs` and `www/play/all.mjs`). Run `node
www/play/scout.mjs --out relay/specs/153/scout-page` and keep four screenshots: the start
ring, the march path, the Unrevealed list on an errand with a mountain left (small 9 to the
rival gives one), and a hungry scout's panel (send the scout ten steps out with its switches
off, End turn until it is hungry). Name them in `relay/specs/153/page.md`, with a line each on
what it shows.

## What later steps rely on

Every button the piece adds is pressed in a script that checks the world. The canvas changes
no page code unless a number's wording moves.

## Done

`engine/check.sh` green; `page.md` and its screenshots written; `engine/CLAUDE.md`'s `www/play/`
line says `scout.mjs` presses Move to…, Route from… and Route to….
