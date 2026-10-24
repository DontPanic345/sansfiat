# Review

Review the whole of #31, "The map and the page", and fix what you find. Read `spec.md`,
`docs/design/map.md`, `docs/design/clans.md`, every earlier step's result, and the diff
(`git diff main...HEAD`). Checks, in `engine/`: `cargo test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `./build-web.sh`, `./cross-build.sh`.

## What to look at

1. **The design, line by line.** `map.md`: every bullet under *The map*, *Size*, *Always three
   centres*, *Rivers*, *Deserts and seas*, *Closed country*, *Also*, *Hidden trail costs*,
   *On the page* is either built and tested or you say it isn't. Take care with: rivers never
   double back and always reach water; crossing costs only off-course and a trail bridges it;
   a sealed valley has a bridgeable mountain and no pass and stays empty; hidden costs are rolled
   and read by nothing; floodplains are the richest land and deltas are slow marsh.
2. **Read the code the way a skeptic would**: float maths that isn't libm-safe (clippy covers
   the obvious), nondeterministic iteration, an rng draw that depends on something
   platform-dependent, `usize` arithmetic that differs on wasm, tests that pass vacuously
   (an assertion inside a loop over an empty set, a `>=` against zero). Check the new
   `step_cost(from, to)` is used by every path that moves (`cost_map`, rounds, reach, daughters,
   move-on).
3. **The rules** haven't changed beyond what the piece said: `git diff` of `CLAN`, `MARKET`,
   `TRAIL`, `TENDING`, `PREGAME` values should show only what build-4's evidence justified.
4. **The sweep, rerun.** `sansfiat sweep --size all` on seeds 1..=10 (background, output to a
   file; it may be slow). Compare with `evidence.md`. Check the top-up: find a seed that tops
   up and run it with `run --seed N --size S`; are the centres sensible (far apart, on fertile
   land, in the open)?
5. **Watch it through the page only.** Serve `engine/www` and use headless Chrome as build-5's
   brief describes (`?size=…&seed=…&run=1` and screenshots; Read the PNGs). Start a **new map** at
   each size, watch the pre-game (take screenshots at the start, partway, and the end), and
   judge what a player would: can you tell rivers, deserts, floodplains, deltas, ridges, passes
   and sealed valleys apart? Does the feed explain what is happening without clicking tiles?
   Does clicking an event centre the map on it? Does the tooltip name terrain and holder? Does
   pan/zoom work (drive it with the DevTools protocol or a Node + puppeteer-less script, or
   state plainly that you could only check the static render)? Do not read the engine's state
   to judge this part.
6. **Docs**: `engine/CLAUDE.md` matches the layout and commands.

## Fix, then answer

Fix real problems yourself, with tests. Anything outside the piece goes under `ideas`.

**Write the answer into the PR description** (`gh pr view`/`gh pr edit` on the piece's PR; if
there is none yet, put it at the end of your result's summary): the piece's question,
*does the pre-game still end, with three centres, on every size and seed, and how often on
its own versus needing the top-up?*: the sweep table (rerun, not copied), the rates per size,
what the top-up picked, the screenshots you checked (names and what they show), and every
known gap. Say plainly what you could not check.
