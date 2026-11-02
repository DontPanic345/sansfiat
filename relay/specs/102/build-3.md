# #102 build-3 (easel): the judge's land checks, the text view, the page

You build what reads build-2's land: the judge's controls and measures, the text view and `play`, and
the page's border, tints, claim painting, guard and festival walkers. `spec.md` items 26–29 are the
rules. Read `spec.md` and the summaries of build-1 and build-2 first. No rule of the game changes here:
`stats::digest` stays as build-2 left it.

## What to build, and where

1. **The judge** (items 26–27), `cli/src/judge.rs` with the measures in `core/src/judge.rs`'s `Watch`
   and `Scores`: the no-claims control per map (`Config { claims: none, .. }`), `--solo` adding the
   three `only(k)` runs, `standing.on_land_200`, `standing.solo_gain` per map and in total (mean, count
   above +0.005 of 21) with its two parts (clans and units; unaligned people on the land), doubt beside the no-claims control, `motion.clans_left`, `clans_joined`,
   `claim_taken` (from the events, as `Watch` reads the world each turn), `land.claimed`, `land.held`,
   `land.guard`. If `judge.rs` in the CLI passes ~800 lines, move the runs into `cli/src/judge_runs.rs`
   and keep the printing where it is. `sansfiat judge FILE` prints the led civ's claim, held land and
   guard beside the control's. USAGE and `engine/CLAUDE.md`'s command line gain `--solo`.
2. **The text view** (item 28), `cli/src/view.rs`; `map`'s marks in `cli/src/play.rs` or `cli/src/map.rs`
   where the view's map is made. The view stays under 80 lines on small seed 2 at the pick and +100; the
   existing test of that keeps passing.
3. **The page** (item 29):
   - `web/src/lib.rs`: `view()` per civ `claim`, `guard`, `land`; per clan its leanings (if not there);
     the clans going to each festival; the `claim` order and `raise` with `kind` through the order form.
   - `www/map-view.js`: a land layer (a border per civ in its colour: solid round held land, dashed round
     paper), drawn in place of the ring and work circles, which go; the market reach ring goes; the
     unaligned clans' tint; a selected festival's walkers' routes.
   - `www/select.js`: the civ's panel gains the claim's counts (claimed, held, paper, thinnest held
     guard), Claim / Unclaim paint buttons, Raise guard / Disband guard; the guard as a unit gets a
     short panel (people, hunger, its claim's counts); a festival's panel lists its clans.
   - `www/main.js`: painting a claim reuses the zone's painting (`painting`, `pruneZone`) with the
     capital as the fixed tile, sending `{"type":"claim"}` as tiles change, as the zone sends `zone`.
   - `www/feed.js`: `ClaimTaken` and `ClanLeft` in the feed and alerts.
   - `www/check.mjs`: the claim order and the guard's raise through `Sim`, a refused claim.
   - `www/play-claim.mjs` (new): spec item 29's script; screenshots to the folder it's given; exits
     non-zero if the civ's land border or the claim's counts aren't there.
4. `engine/CLAUDE.md`: the rows for the files you change, the new script.

## Tests

- `cli/tests/`: `sweep --judge --json` has the new groups and keys; `judge FILE` on a played game with
  a claim prints the claim lines; the view of small seed 2 at the pick lists the claim and the two new
  legal orders and stays under 80 lines.
- `www/check.mjs` as above.

## Done

- `./check.sh` passes.
- `cargo run --release -p sansfiat -- sweep --judge --solo` saved as `relay/specs/102/build-3-judge.txt`
  and `--json` as `build-3-judge.json`: the canvas starts from these. Its wall-clock time in your
  summary.
- `node www/play-claim.mjs relay/specs/102/screens` passes, its screenshots committed (a few, under
  1 MB in all), and one `sansfiat play` view at the pick saved as `relay/specs/102/build-3-view.txt`.

Build-4 relies on: `sweep --judge --solo` reporting the canvas's measures, `play`'s view showing the
claim, and `play-claim.mjs` to re-shoot the page.
