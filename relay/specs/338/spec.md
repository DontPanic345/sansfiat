# #338 The page, played

**Question:** decisions. When a session plays the page as a player plays it, in a browser, with the
fixes in, does the player give more orders per hundred turns, with fewer turns that have none?
The measure is the judge's `player orders` line (`sansfiat judge FILE`): orders per 100 turns,
and turns with none out of the turns played. The control is the M4 check's game on main, small 2
as civ 1 (`relay/retro/m4/play.md`): **11 orders in 103 turns (10.7 per 100), 95 turns with
none.** The piece's played games:

- build-2's game on the page, small 61288 (the owner's map), played **before** its fixes: what it
  finds is the evidence for the fixes, and its count is the page as it stands.
- the play step's game on the page, small 2 as civ 1 (the control's map and civ), played **with**
  every fix in. This is the number the review sets beside the control.

Standing, doubt and motion are read off the same judge lines and reported, not targeted: the
piece adds no rule. Health: `engine/check.sh` passes and the digest doesn't move, except where a
build changes a refusal's text (no digest effect) — any `SAVE_VERSION` bump is a fault.

## Shape: Easel and Canvas (changed from Magnum)

The issue suggests Magnum: what is worst is known only by playing, and each fix is small. That
holds for the play, the finding and the fixing, and they stay in one step (build-2, the canvas).
But two things under it are settled and are not play: a way for a session to hold the page across
its tool calls (the page has none: `www/play.mjs` lives as long as one node process, `probe.md`),
and the two readouts the issue names exactly (the click's camera, the gift line). Building them in
the canvas spends the Opus session's minutes on settled work and leaves less for play. So
build-1 is an easel: the driver and the decided readouts, all with exact dones. build-2 is the
canvas: play, write the faults worst first, fix the top three, build the Standing readout, file
the rest. The canvas may change the easel.

## What build-1 settles (the easel)

1. **The live driver**, `engine/www/play/live.mjs`: one Chrome page held by a background process,
   driven one command at a time from separate shell calls, as `sansfiat play` is a line at a time.
   It prints what a player sees after each command (the top bar, the panel, new feed items, a
   refusal, page errors) and takes screenshots a session reads as images. It logs every command
   to a lines file and saves the game to a file `sansfiat judge` reads. Its commands are the
   player's touch points only (click, right-click, drag, key, button, stepper, tooltip, end
   turns, screenshot, save); no `game.view()` reads.
2. **A click centres the camera** on the clicked tile's middle (issue, Decided), so clicking a
   tile again to cycle what's there leaves the view still. In `onPick` (`www/screens/play.js`),
   a left click on a tile eases the view to that tile with the zoom unchanged. An armed click
   (Move to…, Route from…) centres too; a drag, a paint stroke, a right-click and a click off
   the map don't.
3. **The gift readout**, in the civ's panel and in the text view: the clans it reaches, the food
   given, the food left (issue, Decided).
   - *Reaches*: clans not the civ's own whose market is now the civ's capital market (what
     `markets.rs` gives to next time it meets), derived in the view, not saved.
   - *Given*: `Civ.given`, the food given last time its market met.
   - *Left*: `gift − given`, the part of the gift that went unspent (it stays in the store or is
     posted). When it reaches none, the line says so and why (no clan at your market is not
     already yours).
   This takes the idea **'gave 0.0' stays zero while gift is set and clans join**: the line now
   shows it gave nothing because it reached no one.
4. **Readout ideas that are only words**, taken because each is a line the player reads wrongly
   today and its fix touches no rule:
   - **A refused claim does not name the unjoined tile**: the refusal in `claim.rs` (`check`)
     names the first tile not joined to the capital or a settlement.
   - **Legal orders list only adjacent route targets but far tiles are accepted**: the text
     view's route line says any known tile is a target, with the near ones as examples.
   - **Keep label says default after gift change**: the keep's "(the default)" label says what
     the default is now (the camp's full pack), in the panel and the text view, so a figure
     that moves with the camp reads as the default moving, not as a set value.

## What build-2 leaves to play (the canvas)

- **Playing the page.** small 61288, the second card (piece 338 mod 3 + 1), 100 turns from the
  pick, through the driver, before any fix of build-2's own. The player reads the page only:
  printouts and screenshots. A note before each decision, as `relay/prompts/play.md` asks.
- **What's worst**, written worst first in `relay/specs/338/faults.md`, each with its turn, a
  screenshot or a lines-file repro, and whether it needs a rule change.
- **The top three that need no rule change, fixed**, beyond what build-1 built. A fault that is
  one of build-1's (the camera, the gift line) doesn't count toward the three. Each fix has a
  play script or a `check.mjs` case that fails without it.
- **The Standing readout** (issue, Decided): when nothing a player can order would change
  Standing, the view says so and names what would bring a clan or a rival in reach. Settled
  here: the predicate is "nothing in reach", where in reach is any of
  (a) a clan not the civ's trades at its capital market (the gift reaches it);
  (b) a clan not the civ's lives on a tile the civ may claim now (`claim::check` accepts it with
  the claim), since a claim covering its home brings it at once;
  (c) a rival's tile or capital lies inside an army's reach from one of the civ's stores, since
  an army ending its turn on it takes it. `units::past_reach(w, unit, tile)` answers it for an
  army the civ has; a civ with no army needs the same answer for an army it could raise at a
  store (`units/line.rs` `line_at` reads the unit only for its civ and people, so a helper over a
  civ and a head count is the likely shape).
  When none holds, the text view and the civ's panel say so in one line, and name the nearest
  clan not the civ's (steps from the claim, its market) and the nearest rival tile (steps from
  the nearest store), each with the order that would bring it nearer (a claim grown toward it, a
  signpost toward it, a scout's route). Unsettled, for the canvas: the wording, the helpers it
  reads, and the shape of (c) for a civ with no army. It is proved on
  small 61288 (the owner's idle civ is "second from the pick to the limit and nothing reaches
  it", `relay/retro/m4/game.md`), where the line must appear in the played game.
- **The rest as ideas** in `result.json`, at most three, the others in `faults.md`.

## Decisions I made

- Design says "one play driver for the scripts" (`page.md`, Built as). The live driver is a
  second entry beside `play.mjs`, built on its `open`, so there is still one driver of the page;
  `live.mjs` adds only a process that holds it.
- The shape is Easel and Canvas, not Magnum: the driver and the two decided readouts are settled
  and cheaper in a Sonnet session than in the Opus one.
- A click centres the camera without zooming; armed clicks centre too; drags, paint strokes and
  right-clicks don't. The ease is the page's existing third of a second.
- The gift's *left* is the gift less what it gave (the unspent gift), not the store's spare.
- The gift's *reaches* is derived from where clans trade now, so it says what the next gift
  will reach; no saved field, no `SAVE_VERSION` bump.
- The two named fixes (camera, gift) are build-1's, and build-2 fixes three more from play.
- "Nothing in reach" is (a) or (b) or (c) above. Raising, disbanding and signposts are not in
  it: they move people within the civ or move where its clans live, not whose they are.
- The play step plays small 2 as civ 1, not civ 2 as `play.md` computes, so its game sits on the
  control's map and civ. The canvas plays small 61288 as civ 2, so the piece meets both.

## Unsettled

All in build-2: which faults are worst and their fixes; the Standing line's wording and
helpers. Its Open score is 4.

## For the review and the play step

- **Play step:** build the page (`engine/build-web.sh`) and play small 2 as civ 1 on the page
  through `node engine/www/play/live.mjs` (its header says how), 100 turns from the pick; save
  to `relay/specs/338/play/small-2.json` and judge it. Not through `sansfiat play`. Run
  `node engine/www/play/all.mjs` and put its screenshots in `relay/specs/338/play/`.
- **Review:** read `faults.md` and check the three fixed faults are the top three that need no
  rule change. Put the two played games' `player orders` lines beside the control's (11 in 103,
  95 with none) in the PR, and say plainly if the played count didn't rise. Run
  `node engine/www/play/all.mjs`; open the page with the driver, click a tile, click it again,
  and confirm the view didn't move the second time; read the gift line in the civ's panel and
  in the text view on a game where the gift reaches no one; find the Standing line in
  build-2's game. Add the decisions that matter to `page.md`'s and `founding.md`'s ledgers.
