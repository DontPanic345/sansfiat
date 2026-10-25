# Review

Review the whole of #32, "Founding", and fix what you find. Read `spec.md` (its decisions
especially), `docs/design/founding.md`, `docs/design/clans.md`, every earlier step's result,
and the diff. Checks, in `engine/`: `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `./build-web.sh`, `./cross-build.sh`,
`node www/check.mjs`.

## What to look at

1. **The design, line by line.** Every bullet of `founding.md` under *Founding*, *The store*
   and *Units*, and every line of the piece's *Decided*, is built and tested, or you say it
   isn't. Take care with: clans joining the *nearest* founding market and the fallback for a
   civ with none; daughters keeping their civ; the levy coming only from fed civ clans; the
   cap; a unit's person coming from a clan near the capital and going back to the nearest
   civ clan; hungry for three turns, it disbands; a route that is the cheapest by the costs
   as they stand and never reads hidden trail costs; a turn's labour at a time.
2. **The pre-game hasn't changed.** No new random draws before the end, no change to existing
   rules' values or arithmetic (`git diff` of `data.rs`, `clans.rs`, `markets.rs`,
   `pregame.rs`). Existing tests that changed: is each change only plumbing (struct literals,
   running past the end now watches)?
3. **Read the code as a skeptic**: nondeterministic order, float maths that isn't libm-safe,
   validation that lets an order through for the wrong civ or phase, an order that changes
   the world when refused, a refused `EndTurn` that `World::run` could loop on, tests that
   pass vacuously (an assertion in a loop over nothing; a unit test whose unit never moves).
   Check `replay` rebuilds a led game with orders, and `./cross-build.sh`'s `lead` line
   really gives orders (its digests change after the pick, units move).
4. **Play it through the page only.** Serve `engine/www` (`python3 -m http.server -d www`)
   and drive `/usr/bin/google-chrome --headless=new` over the DevTools protocol with a small
   Node script under `/tmp`, as build-3 and build-4 did. Order only by clicking: mouse events on buttons
   and the canvas (`Input.dispatchMouseEvent`, `button: "right"` for moves). You may read
   `window.game.map.screenOf(i)`, the page's text and `document.body.dataset` to find where
   to click and to know when a turn is done; never order through a script call or read the
   engine's state to judge. On small seed 2 (`?size=small&seed=2`), and one medium seed (8):
   - click **Run to end**; the pick appears: are each civ's clans, people, hearths, terrain
     and rival distance there and sensible? Does **Show** find the capital?
   - **Lead** a civ: the top bar shows the civ, people, store, units; the capital is selected
     with **Make warrior**.
   - make a warrior; select it (from the feed or the map); right-click a tile several steps
     away; the route shows with its cost and turns; **End turn** until it arrives, checking
     it moves about a turn's labour a turn; the tooltip names tiles' civs and previews the
     move.
   - **Disband** it: the feed says which clan its person rejoined; units and people in the
     top bar change by one.
   - Watch a second game with **Watch all three**: turns run, nothing can be ordered.
   Take screenshots at each point and Read them: can a player find their capital, their
   units, the route, the store? Do Gold civ marks read apart from hearths and centres?
5. **Docs**: `engine/CLAUDE.md` matches the code, commands and page.

## Fix, then report

Fix real problems yourself, with tests. Anything outside the piece goes under `ideas`.

The PR description says what the piece delivers, what you checked and fixed, the screenshots
you read (names and what they show), and every known gap. Fold in #31's still-open gap: CI
doesn't run `node www/check.mjs` (the owner adds `- run: node www/check.mjs` after
`./cross-build.sh` in the engine job of `.github/workflows/ci.yml`). Say plainly what you
couldn't check (touch, other browsers, narrow windows).
