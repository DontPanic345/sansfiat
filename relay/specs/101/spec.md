# #101 The game judges itself: spec

## What it delivers

The game gets a score and a judge. **Standing**, a civ's share of the world's people, is computed
every turn for every civ. `sansfiat sweep --judge` plays the seven retro maps with every civ on the AI
and reads the five scores of `docs/process.md`, against the `--no-army` control and a second run of
its own; `engine/judge.toml` holds the bands, and CI fails on a failing band. `sansfiat play` lets
anyone lead a civ from a terminal, a line at a time, and read what a player sees in text;
`sansfiat judge` reads a played game back in the same scores, beside the same civ under the AI. Two
prompts, `relay/players/player.md` and `judge.md`, make a model a player and a critic, and one trial
game on Sonnet says what that costs and what it finds.

The question: **can a session play a led civ for a hundred turns and say what it couldn't tell?**
It is answered by the trial: whether the player reaches +100, what Standing it reaches against the
AI's play of the same civ, and the judge's list of problems, with the cost.

No rule changes. Every digest stays what it is on main.

## Shape: Easel and Canvas

Taken as the piece suggests. The judge, the order loop, the text view and the score line can each
be said exactly, and they are the easel. What can't be said before running is what the canvas holds:
where each band sits, the two prompts, and whether a model can play from the text view at all. The
canvas may change the easel where play shows it wrong (a view too long, a missing line, a margin).

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | easel | Standing, the orders counter, `core/src/judge.rs` (what a game is scored on, turn by turn), `sweep --judge` with its control, the bands file and CI | Everything else reads Standing and the judge's measures |
| build-2 | easel | `sansfiat play`, the text view, `sansfiat judge` over a game | Uses build-1's Standing and its turn-by-turn scoring |
| build-3 | canvas | The bands from main, the player and judge prompts, one trial game on Sonnet and its record, the root `CLAUDE.md` lines | Needs the whole easel to play on |

## The digests (read before any step)

`stats::digest` is FNV-1a over the world's JSON. Everything this piece adds to `World` is
`#[serde(skip)]`, like `notes`, so no digest moves and `SAVE_VERSION` stays 2. `sweep --after` keeps its
output byte for byte. The proof is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines on main
today (`probe.md`), and six after every step.

## Settled (decisions the owner can overrule)

### Standing (build-1)

1. **Standing** of civ `c` = (people of the clans whose `civ` is `c` + people of `c`'s units) ÷ (people of
   every clan + people of every unit). Warriors and scouts are people the civ took from its clans, so
   raising a camp neither wins nor loses Standing. `stats::standing(w) -> BTreeMap<u32, f64>` (every
   civ, 0 when the world has nobody), and `Summary` gains `standing` with it (empty before founding).
   When L brings land, it adds the people on the civ's land; that is L's change, not this one. The
   page doesn't show it yet (P does). `report::line` doesn't print it, so `sweep` and `run` keep their
   output.
2. **The orders counter.** `World.orders: BTreeMap<u32, BTreeMap<String, u32>>`, `#[serde(skip)]`: per
   civ, accepted orders by kind. `World::order` adds one under `Order::kind()` (the wire name:
   `raise`, `move`, `zone`, `switches`, `resupply`, `forage`, `pillage`, `route`, `clearRoute`,
   `disband`) when it returns `Ok`; `act` adds one under `setKeep` / `setGift` for those. So the AI's
   orders and the player's both count, and a replay rebuilds the counts. The AI's are the counts of
   the civs the player doesn't lead.

### What a game is scored on (build-1, `core/src/judge.rs`)

3. **`judge::Watch`**, in core so later pieces (R's end screen keeps Standing per turn) and the CLI
   share it: `Watch::new(&World)` at the pick, `watch.turn(&World)` after each turn, and the
   measures read off it. It keeps, per turn from the pick: every civ's Standing, the leader, the
   spread; and it counts what changes hands. It draws nothing from `World.rng` and changes nothing.
   Its numbers live in `data.rs` as `JUDGE`, each with a note: `lead_margin` (0.005) and `sample` (10).
4. **The leader and lead changes.** The leader at the pick is the civ with the most Standing (lowest
   id on a tie). It changes when another civ's Standing is more than the leader's by more than
   `JUDGE.lead_margin`; each change counts one. Without a margin, medium 8 and 9 count 7 and 8 changes
   between civs within half a point of each other (`probe.md`).
5. **The spread** is the most Standing minus the least, each turn. It is reported at the pick, +100
   and +200, with its least and most over the run. **Swings** count how often it opens and closes:
   sampled every `JUDGE.sample` turns, each difference between samples of more than
   `JUDGE.lead_margin` is up or down, and a swing is a difference whose direction is the opposite of
   the last such one.
6. **Motion**, counted turn by turn after the pick: `clans_changed`, clans whose `civ` differs from
   the turn before (a clan gone or new doesn't count); `people_changed`, the people of those clans at
   the change; `land_changed`, land tiles whose `civs::tile_civ` differs from the turn before. Before
   L, land changes only with a clan's claims; L makes it the land a civ holds.

### `sansfiat sweep --judge` (build-1, `cli/src/judge.rs`)

7. **The runs.** For each retro map (`retro::RETRO`), in parallel: the army run twice (`army`, `gifts`,
   `zone` on) and the control once (`army` off), each to the pick with every civ on the AI, then 200
   turns, with a `Watch`. A map's run that panics is caught (`std::panic::catch_unwind` or the
   thread's `join` error), counted under `health.panics` with its map and message, and left out of
   the other totals; the sweep goes on.
8. **What it reports**, per map and in total, in five groups. `--json` prints
   `{"maps": [{"size", "seed", "pick", <groups>}...], "total": {<groups>}, "bands": [...]}`; the text
   prints one line per map, then the total, then one line per band. The metric names are the band
   names (item 10):
   - `standing`: per map, each civ's Standing at +100 and +200 (`civs: [{id, at_100, at_200}]`); in
     total `top_200` (mean over maps of the most Standing at +200) and `aligned_200` (mean over maps of
     the civs' Standing summed: the share of the world in any civ).
   - `doubt`: `lead_changes` (sum), `maps_with_lead_change`, `spread_100` and `spread_200` (means),
     `swings` (sum). Games ending before the limit come with R.
   - `decisions`: `ai_orders_per_100` (accepted AI orders per civ per hundred turns, the mean), with
     `by_kind` the same per kind, and `levers_repeated` (the mean per civ of order kinds given at
     least twice).
   - `motion`: `clans_changed`, `people_changed`, `land_changed` (sums).
   - `health`: `pregame_by_centres` (maps whose pre-game ended without the limit, of 7),
     `camps_kept` (civs that kept their first camp, as `sweep --after` counts it, of 21),
     `warriors_starved` and `camps_lost` (sums, from `Civ.army`), `people_vs_control` (the world's
     people at +200 in the army run over the control's: the army doesn't starve the world),
     `panics`, and `digest_mismatches` (maps whose two army runs end on different `stats::digest`).
   `seconds`, the wall clock, is printed and not banded.
9. **"Units starved against the `--no-army` control"** (the piece's line) is read as two measures:
   warriors starved, which the control has none of, and the world's people against the control's.
   Design says the control is `--no-army`; the scores' control for every later piece is main, which
   the bands hold.
10. **Bands: `engine/judge.toml`.** One table per band, named `group.metric` as the JSON's `total`
    names it, with any of `fail_min`, `fail_max`, `warn_min`, `warn_max` (inclusive bounds) and a `note`
    saying what the band protects:
    ```toml
    [health.panics]
    fail_max = 0
    note = "a map that panics is a game that can't be played"
    ```
    `sweep --judge` reads `judge.toml` from the working directory, or `--bands FILE`. A value outside a
    fail bound prints `FAIL group.metric value (bound)` and the command exits 1 after printing
    everything; outside a warn bound prints `warn …` and doesn't change the exit; inside prints `ok …`. A
    band naming a metric the judge doesn't report fails. The `toml` crate parses it.
11. **The easel's `judge.toml`** holds the health bands only, as fail bands at what main does:
    `panics` 0, `digest_mismatches` 0, `pregame_by_centres` at least 7. The canvas adds the rest.
12. **CI.** `check.sh` runs `cargo run -p sansfiat -- sweep --judge` after `cargo test` (the dev profile is
    optimised, so it reuses the test build): about a minute.

### `sansfiat play` (build-2, `cli/src/play.rs`, the view in `cli/src/view.rs`)

13. **A game is a file, and every call resumes it.** A model's shell calls are separate processes, so
    `play` can't hold a game in memory between decisions. `sansfiat play --game g.json --seed N --size S
    --civ K` makes the game: runs the pre-game to the pick, leads the civ `K`-th in id order (1 to 3;
    the ids are not known before the pick), saves `g.json`, prints the view. Without `--civ` it prints
    the pick's cards (`civs::cards`) and makes nothing. `sansfiat play --game g.json` with the file there
    replays it (`World::replay`, which rebuilds notes and events too) and goes on. The file is a plain
    `Save`, so `sansfiat replay g.json` reads it. After each line it is written again, so a game killed
    mid-call loses at most one line. A small retro map replays to +100 in under 2 s (`probe.md`).
14. **Lines**, one a line from stdin, until EOF:
    - an `Order` as JSON, `{"type":"move","unit":12,"to":5}`: sent as `Action::Order` for the led civ;
    - an `Action` as JSON, `{"type":"setGift","food":2}` (the two parse apart by `type`);
    - `end` or `end N`: N turns, 1 by default, stopping early if the game leaves `Main`;
    - `why ID`: the subject's reasons, as `run --why` prints them;
    - `view`: the view again; `map`: the map as `sansfiat map` prints it, with the led civ's capital
      and units marked;
    - `note TEXT`: the player's words, kept with the turn and printed back as `noted`;
    - an empty line or `#…` is skipped.
    Every order and action goes through `World::act`, so through `validate`, the page's check. A
    refusal prints `refused: <reason>` and changes nothing. After an accepted order or action, and
    after `end`, the view prints. Anything else prints `unknown: <line>` and the grammar.
15. **The lines file**, `g.lines.jsonl` beside the game: every line read, as
    `{"turn", "line", "result"}` with `result` `ok`, `refused: …`, `noted` or `unknown`. It is what the
    judge prompt reads for what the player meant (its notes) and what it tried (its refusals), which
    the save doesn't keep.
16. **The view** (`view::text(&World, since)`): what a player of the page can see, in text, every tile
    named by its id and `(q, r)`:
    - the turn, the turns since the pick, and the phase;
    - every civ's Standing, its rank and the leader, the led civ marked;
    - the led civ's store, keep (in force and as set) and gift, and what it gave and was given last turn;
    - each of its units: kind, id, tile, people, pack (and pack when full, turns it feeds), dest,
      line (cost and carry, or none, and cut off or cut at), hunger, zone and held tiles; a scout's route;
    - its clans: how many, their people, and the five largest with home and fed share;
    - the unaligned clans in its market's reach with a leaning to any civ: id, home, people, and their
      leaning to each civ against the join threshold, the strongest first, at most eight, with a count
      of the rest;
    - places worth naming: each capital, the AI's post for its camp (`units::post`), its clans' homes;
    - the feed since the last call: the events about the led civ, its units and its clans, alerts
      marked `!`, with a count of the rest;
    - the legal orders: each order and action as its JSON wire form with what it needs filled in,
      shown only where `validate` passes it now (or for a tile order, where some tile would), and
      the gift's and keep's ranges.
    It is under 80 lines on small seed 2 at the pick and at +100: a player reads it every decision.
17. **`sansfiat judge g.json`** (`cli/src/judge.rs`) replays the game a turn at a time with a `Watch`,
    and plays the same map with every civ on the AI to the same turn as the control. It prints the
    score line every `JUDGE.sample` turns (turn, every civ's Standing, the leader), then the led civ's
    Standing at the end beside the control's for the same civ, its rank in each, the doubt and motion
    measures of both games, the player's orders by kind (from the log) and per hundred turns, the
    turns it ended with no order or action since the last end, its longest stretch without one, and its
    refusals (from the lines file, if there). `--json` as for the sweep.

### The prompts and the trial (build-3)

18. **`relay/players/player.md`** makes a session a player: it plays for Standing through
    `sansfiat play` only, and before each decision writes `note meant: … did: … surprised: …` as
    lines in the game. **`relay/players/judge.md`** makes a session a critic: it reads the lines file
    and `sansfiat judge`'s output and lists problems only, in four kinds: a decision with no real
    choice, a reason missing or wrong, an order with no visible result, a turn the player did nothing.
    Each finding names its turn and its evidence. Praise isn't a finding.
19. **The trial** plays one game on a retro map, 100 turns from the pick, with the player on Sonnet,
    then judges it on Sonnet at medium effort (a judge, so not at low), each a fresh subagent the
    canvas starts with its Agent tool (`subagent_type: "general-purpose"`, `model: "sonnet"`), given
    the prompt file's text and nothing of the canvas's context. A nested `claude -p` can't: inside a
    Relay session it isn't logged in, and `node relay/usage.mjs` can't read the plan's windows either
    (spec-review ran both; `probe.md`). So it records in `relay/specs/101/play.md`, for the player and
    for the judge: the tokens, tool calls and minutes the Agent tool reports, and the list dollars
    those tokens come to at Sonnet's price, said as an estimate; the plan points are build-3's own,
    which the crew records on its step. Then the turns reached, the decisions, `sansfiat judge`'s
    score lines, the judge's findings, and what the player said it couldn't tell.
20. **Root `CLAUDE.md`** gains, under Checks, the judge line (`sweep --judge`, the bands in
    `engine/judge.toml`, how a piece moves a band) and the play line (`sansfiat play`, the game file,
    `sansfiat judge`). `engine/CLAUDE.md` gains a row for each new file and the commands.

## Left unsettled: the canvas (build-3)

- **The bands.** Where each score's warn and fail bounds sit, read from the judge on main and from a
  known regression the judge must catch. A band that held main to its value exactly would fail every
  piece that means to move it; a band too wide catches nothing. Proof: the judge passes on the branch,
  and a perturbation that undoes something (the canvas picks one, such as halving the default gift as
  #75's joins fell from 183 to 84) fails or warns it.
- **The prompts**, judged by the trial: the player reaches +100 within the time the canvas gives it,
  writes notes at each decision, and the judge returns findings with turns and evidence.
- **Whether the view is enough to play.** The player's notes and the judge's findings say; the canvas
  fixes the easel where they show a gap it can fix in the time.
- **`JUDGE.lead_margin` and `JUDGE.sample`**, from the sweep's lead changes on main.

## What proves the piece

The piece's question is asked of a played game, and its control is **the same civ on the same map
played by the AI**, which `sansfiat judge` plays beside it. The judge itself is held to main: no digest
moves.

| Score | How it is measured | Passes when |
|---|---|---|
| Standing | `sansfiat judge` on the trial game: the led civ's Standing at +100 against the AI's for the same civ | Reported in `play.md`; either way is an answer |
| Doubt | The same: lead changes and spread in the trial and in its control | Reported |
| Decisions | The player's orders per hundred turns and levers pulled more than once, against the AI's `ai_orders_per_100` | Reported; the player gave at least one order in each 20 turns, or the judge lists the stretch |
| Motion | `motion` in the trial and the control | Reported |
| Health: the judge | `sweep --judge` on the branch against `judge.toml` | No fail band; a perturbation the canvas names fails or warns, and is reverted |
| Health: the digests | `bash relay/specs/100/baseline/compare.sh` | Six `ok` lines |
| Health: the builds | `./check.sh`, the judge included | Passes, in about a minute more than main |

### What the review must run

1. `./check.sh`, and how long the judge's step took in it.
2. `bash relay/specs/100/baseline/compare.sh`: six `ok`.
3. `cargo run --release -p sansfiat -- sweep --judge` and `--json`: every band `ok` or `warn`, and read
   the totals against `relay/specs/101/bands.md`.
4. Play twenty turns itself: `sansfiat play --game /tmp/r.json --seed 2 --size small --civ 1`, then a
   move, a zone, a gift, `end 5`, `why` on its camp, a refused order; then `sansfiat judge /tmp/r.json`.
   Is the view under 80 lines, and does it say what the page says?
5. Read `play.md`: the cost (with build-3's points from its step), the turns reached, the findings;
   the PR description carries them and answers the question.
6. The decisions below go into the design's Decided ledgers: `vision.md` takes items 1, 4 and 9;
   nothing else in `docs/design/` is touched by this piece. `process.md` isn't a design file: the review
   notes in an idea anything it should say now.

## Tests the piece must have

- build-1: Standing sums to the civs' share and is 0 for a civ with no people; raising a camp doesn't
  change it; the orders counter counts an AI move and a player's order and isn't in the JSON; `Watch`
  counts a lead change only past the margin, a clan joining under `clans_changed`, and is the same on
  two equal worlds; `judge.toml` parsing and the three band outcomes, and a band naming no metric fails.
- build-2: a scripted session through `play`'s stdin (in `cli/tests/`): make a game, a refused order,
  an accepted move, `end 3`, `why`, `note`, and a second call that resumes at the same turn with the
  lines file holding every line; the view of small seed 2 at the pick is under 80 lines; `judge` on that
  game prints the score lines and the control.

## Not in this piece

A Relay `play` step (m3-relay.md §5, once the trial's cost is known). Strategy files. Driving the page.
Any rule or number of the game. Standing on the page (P) and on land (L).
