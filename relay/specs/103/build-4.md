# #103 build-4 (canvas): how the page reads, at three sizes

The easel is built:
- the view model, the layers and the atlas (build-1);
- the screens, the bar, menus, keys, autosave and the end screen (build-2);
- the panel, Move to…, selection, the feed and every touch-point script (build-3).

You make it read well. Play every touch point, read your own screenshots at phone, laptop and
wide, and fix what reads badly, until the page is a game a player can start, play and come back
to. The map should be the thing you look at.

Read:
- `spec.md`, the whole of it, and "Left unsettled" closely;
- `docs/design/page.md`;
- the summaries of builds 1–3, and build-3's list of what reads badly;
- main's shots in `relay/specs/103/screens/main-*.png`.

## What you settle

- **The bar's density.** One line at laptop and wide; a short line with a ☰ at phone.
- **The panel.**
  - Its height, and how the numbers lay out: in columns, in chips, or wrapping.
  - Whether the reasons fit beside the numbers.
  - How the button row reads, with the greyed reasons.
- **The map's clutter**, under "No ring, radius or circle that isn't a rule" and "stays out of
  the map's way":
  - every clan's territory tint;
  - the centre rings after founding;
  - festival circles;
  - the many small routes.

  Choose what is off by default, what is drawn quieter, and what goes into View.
- **The phone layout:**
  - the drawer;
  - the bottom sheet;
  - the bar;
  - whether anything needs touch-specific handling to reach a touch point, given that
    right-click doesn't exist there, which Move to… covers.
- **The end screen's chart**, and the start screen's look.
- **The placeholder art's legibility** at low and high zoom: low is a large map at fit, which
  `layout.mjs` shoots. Change `placeholders.js` and re-run
  `draw-atlas.mjs`. Keep `atlas.json`'s cell names and sizes unless one is wrong, which would
  be a decision.

You may change any easel choice the shots show wrong: a row, a default, a breakpoint, an item of
`spec.md`. Make each one a decision with the shot that showed it. Keep the shape: the screens
as modules, one model, layers, one panel component, one atlas, one driver. No rule changes:
`stats::digest` stays.

## What proves it

- `./check.sh` passes, with `play/all.mjs`, 26 of 26 touch points.
- `layout.mjs` passes at all three sizes. The map is at least 60% of the window at laptop and
  wide with a camp selected and the feed as it opens by default, counting only what nothing
  covers. Main's is 67% and 79%. At laptop, a 340 px feed and a 40 px bar leave the panel 106 px
  at 60%, so the feed's width, whether it opens by default at laptop, and the panel's height
  are yours to trade. Turn the share bar on in `all.mjs` if build-3 left it off.
- **A played game on the page** (spec, Left unsettled):
  1. Through `play.mjs`, lead a civ on small seed 2 and play 30 turns with only buttons, keys and
     clicks.
  2. Use Move to…, paint a zone and a claim, raise a guard, change the gift, and follow an alert.
  3. Reload at turn 15 and Continue.
  4. Save to a file.

  `cargo run --release -p sansfiat -- digest --game FILE` and the page's `Sim.digest()` match, and
  `sansfiat judge FILE` reads the game. Write the script you play it with as
  `www/play/game.mjs`, so the review can rerun it on another seed. `all.mjs` may skip it for
  time if it runs past a minute; say so.
- **The screenshots.** `layout.mjs` at the end, into `relay/specs/103/screens/`, under 2 MB in all.

## Record

- `relay/specs/103/canvas.md` holds:
  - what you changed and why, each with the shot (before and after) that showed it;
  - the played game's notes: what the page said and failed to say at each step, the digests, and
    the judge's score lines;
  - the map's share per size;
  - what you still think reads badly.
- Your summary gives the touch points (26 of 26), the lever count, and one sentence on how the
  page reads at each size.
