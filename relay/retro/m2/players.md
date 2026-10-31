# M2 retro: automatic players and judges

Angle: what exists to play and judge the game without the owner, what to build, and what it can't tell him.
Measured = read from issue blocks, specs or code. Estimated = mine, marked.

## Verdict

**M2 already judges the game with numbers, and it works, but every judge is one-off and written by the step that needs it.** Nothing runs on every PR, nothing plays the game as a player with a goal, and nothing reads a play log and says "this is confusing". Build first the cheap, shared part: an invariant sweep in CI plus a turn-by-turn **session interface** on the CLI (observe as text/JSON, act as JSON, step, seed). That is the foundation for both a scripted bot and an LLM player. An LLM player-and-judge is worth a trial in M3 on top of it, as a `play` step in Relay after the review. It will catch broken and confusing things; it cannot say whether the game is fun, because the game has no win or lose yet (`docs/design/vision.md` lists "a way to win and a way to lose" as still to make).

## What exists

1. **The AI civs.** `units::ai` and the civ steps run when the player picks `ChooseCiv { civ: None }` ("watching"). All of `sweep` and `run` use it.
2. **`sansfiat sweep`** (`engine/cli/src/main.rs:486`). Default: seeds 1..=10, small, per-seed line, levelling-off check. `--after 200 --maps retro` runs the seven retro maps (21 AI civs) to the pick and 200 turns on, one line per civ plus totals (kept first camp, hungry turns, starved, tile-turns held, clans joined, clans' people), with controls `--no-army`, `--no-gifts`, `--no-zone`, `--lever G`. The default sweep takes about 14 s in release (relay/specs/55/sweep.md). Engine speed is 1.5 to 3.4 ms a turn (relay/specs/57/bench.md).
3. **`digest [--lead]`** with `lead_orders` (main.rs:263): a hard-coded script of a led civ (raise every 20 turns, switches, resupply, scout route at turn 2, camp moves, zone, disband, gift, keep). Mirrored in `cross-build.mjs`. This is the only scripted player, and it exists to compare native and wasm digests, not to play well.
4. **Pinned digests** (`cli/tests/save.rs`, `the_save_version_pins_the_rules`), `replay`, `cross-build.sh`, `check.mjs`: determinism judges. They catch "the rules moved", not "the game got worse".
5. **`www/play.mjs`** and `play-camp / -scout / -lean / -hold / -read.mjs`: headless Chrome scripts, each a fixed click path for one piece, screenshots out. `play-read.mjs` exits non-zero if a panel's reasons are missing and checks a versionless save errors. The rest only screenshot; a reviewer reads the pictures.
6. **The query surface the engine already has:** `run --json | --events-json | --why ID`, `Sim.view()` JSON, `Sim.check(action)` (validate without applying), `Sim.notes(id)`, `Action` as serde JSON, `Save` = config + action log, `rng` seeded. A player-facing "why" exists for clans, markets, civs, units.
7. **Not there:** a CLI mode that takes actions one at a time, a compact text view of "what the player sees", a goal/score for a player, any function that runs an AI-controlled *led* civ (the AI only plays watched civs; `--lead` is the fixed script), a win/lose.

## How M2 judged the game

| Piece | Judge | Result |
|---|---|---|
| #29 camp (review $0.56, 3 min, opus low) | `sweep.md`: 21/21 camps kept, clans' people 8% under control; `play-camp.mjs` | merge, no fix |
| #55 clans are the prize (review $0.45, 3 min, sonnet high) | sweep over gift levels: led civ 14.7 clans vs rivals' 8.1 at gift 15; `--no-gifts` control | a real answer to a game question ("is joining a lever?") from a sweep, not from play |
| #56 hold ground (review $2.28, 24 min, opus low) | sweep (`--no-zone` control), `play-hold.mjs` | **Caught a regression after merging main (#75 scouts):** AI camps zoned while cut off by distance and starved (39 hungry turns, 19 starved vs control 0). Review fixed `ai_zone`. Also found `select.js` with a missing brace that stopped the page loading, found by running the page. |
| #57 reading (review $0.31, 3 min, opus low) | `play-read.mjs`, digests unchanged, bench | merge, no fix |

What this shows:
- Numeric judges against a control (`--no-X`) caught the one substantive late bug of the four reviews, and cost seconds of CPU. They were run by an LLM review that had to know to look.
- The page scripts caught "the page doesn't load", which no Rust test would.
- The sweeps are the only evidence that the game works as *design*: M1's retro found 14 of 21 civs starved their warrior within 200 turns, by a scratch probe outside the repo (relay/retro/m1/game.md). M2's sweeps now include that as "kept first camp" and "hungry turns".
- No step ever played *as a player with a goal*; the play scripts follow a path the author already knew works. They prove the path exists, not that it is discoverable or interesting.

## Options, cheap to rich

Costs for LLM options are estimates (no M2 data on a player loop). Reference points from M2: an opus low review is $0.3 to $2.3 list and 3 to 24 minutes, 2 to 3 5-hour points and about 0 weekly points (#94, #89, #82, #68 blocks).

### (a) Scripted sweeps with numeric judges in CI

A `sweep --judge` (or a test) that runs the retro maps and fails on: every invariant `validate`/`check_*` already has; "every civ keeps its camp 200 turns" (21/21); no starvation beyond control; people not collapsing; no panic; digests stable across runs; speed within 10% of a stored baseline. Add balance bands as warnings, not failures (joins per civ, held tile-turns, share of unaligned clans left).
- Catches: crashes, starvation, runaway or collapsed economies, merge regressions like #56's, AI getting stuck, non-determinism.
- Misses: anything about a *player's* decisions, readability, whether numbers that pass the band are boring.
- Cost: 14 s to a minute of CI CPU per PR; no tokens. One-time build: a small step (Sonnet medium).
- Engine needs: a machine-readable sweep output (`--json`), thresholds in a file, seeds (has).

### (b) A scripted bot player through the CLI

Several named strategies (turtle, raider, giver, explorer) written as Rust or JSON order lists, run on the retro maps against the AI civs, scored on people, clans, store, army kept. Generalises `lead_orders`.
- Catches: one strategy dominating (balance), a lever that does nothing (like #55's saturation, found one-by-one by hand), orders that error, strategies that starve themselves. Cheap A/B for tuning: the sessions already do this by hand with `--lever`.
- Misses: discoverability, clarity, anything that needs reading the page; strategies are the author's.
- Cost: no tokens per run once written; build is a medium step (Sonnet high or Opus low). Run time seconds to minutes.
- Engine needs: a score or "standing" report for the led civ; an `Action` list that a bot reads and writes; **a CLI that applies orders**. Today `lead_orders` is code in `main.rs`; it should take a JSON strategy file.

### (c) An LLM player, and an LLM judge of its log

A `claude -p` session plays one led civ for N turns through a session interface, with a written strategy ("try to win clans by gifts", or no strategy: "explore and say what you do not understand"), writing a log of observation, intention, action, surprise per decision. A second session (or the same) reads the log and answers: which decisions had a real choice, what was unexplained (the "Why" text missing or wrong), what it wanted to do but could not, where it did nothing.
- Catches: unclear state and reasons, actions with no visible result, dead options, errors the UI gives, a lever with no visible feedback, "I could not tell why my camp was hungry". This is the nearest thing to the owner's first ten minutes with a build.
- Misses: fun, pacing, feel, anything seen only in a picture, long-run balance (too few games), the owner's taste. Its confusions are an LLM's, not a human's (it reads JSON patiently; a person scans a canvas). It is also liable to praise.
- Cost, estimated: one game of about 30 decisions (a decision every 5 to 10 turns of a 200-turn game) with a compact ~3k-token text view each = a few hundred thousand tokens in, plus the log. Sonnet: about $1 to $3 list, 10 to 25 minutes, a few 5-hour points. Opus: three to five times that. Judge pass over the log: $0.2 to $0.6, 3 minutes. Two to three seeds per check would be $3 to $10 list and under an hour. Weekly impact is small (reviews of $2 moved weekly by 0 points) but the 5-hour window is the pacing limit in README.
- Engine needs: the session interface (below), a **text view** of the world the way the page shows it (top bar, civ panel, the feed with alerts, selected-thing "Why"), seeds, `check` before `act`, save/load (has `Save` and `replay`) so a failed or interesting game can be reopened by the owner.

### (c2) Same, through the page

Drive `play.mjs` (headless Chrome) with screenshots to a vision model. It sees what the owner sees, and catches layout and legibility problems the JSON hides. Costs several times more per decision (images), is slower and flakier (pan, zoom, click by tile), and each piece's `play-*.mjs` shows how brittle it is. Do after (c) works, only for pieces that change the page.

### (d) A play-check step in Relay

A new step after `review`, or a **milestone play check** run by the owner or crew every N pieces on main. After-review: the play session runs on the PR's branch, files a short "play note" in `relay/specs/<n>/play.md`, and its verdict lands in the PR description beside the review's. The review could then keep its job (code vs design) and drop its play-scripts. Milestone: runs on main at the end of a milestone, for the long games a single piece can't show.
- Catches what (c) does, tied to the piece that caused it, before merge. The milestone version catches interactions between pieces.
- Costs one more session per piece, about $1 to $3 (estimated), but it can replace part of the opus low review's play work ($0.3 to $2.3 each).
- Misses: same as (c), and a play note nobody reads.

## What the engine must expose

| Need | Have | Gap |
|---|---|---|
| Seeds, deterministic runs, replay | yes | none |
| Action API as JSON, validate without applying | `Action` serde, `Sim.check` | not on the CLI |
| Step-wise session | no | `sansfiat play --seed N --size S --civ K [--save f]`: a loop reading one JSON action or `end N` per line from stdin, printing the result and a new observation. Or a stateless form: `act --save game.json '{...}'` that appends to the save and prints the observation |
| Text observation | `Sim.view()` JSON (large, built for the canvas), `run` text report | a compact text view for a led civ: turn, store/keep, army and pack and line, clans of the civ and nearby (people, leanings), new feed events since last call with alerts, legal actions with their `check` result |
| Reasons | `--why ID`, `Sim.notes` | expose in the session (`why ID`) |
| A score / standing | `Summary` counts | a few numbers for the led civ (people, clans joined, store, camp health) so a bot and a judge have something to compare. A real win/lose is a design decision, not a tooling one |
| Save/load | `Save`, `replay`, version pins | none; keep the log so the owner can open the exact game in the page |
| Led-civ AI | no | optional: let the AI play the led civ too (a "baseline player") so a judge can compare a human-like run to it |

## Recommendation

**M3 builds first: the session interface and the CI sweep judge as one piece, then an LLM play-check on top as a second piece.** Proposed piece for the owner:

```
**Goal:** Anyone, human or Claude, can play a game turn by turn from a terminal and read what a player sees; every PR runs a judge that fails on a broken world. The question it answers: can a Claude session play a led civ for 100 turns and say what it could not tell?
**Spec:** opus medium
**Spec-review:** opus low
**Shape:** Henry Ford: an interface and a judge built once, then a first player session to try them
**Decided:** `sansfiat play` reads one JSON action or `end N` a line and prints a compact text observation (turn, store, camp, civ's clans, feed since last call with alerts, `why ID`); `sweep --judge` runs the retro maps and fails CI on a civ losing its camp, starvation above the `--no-X` control, a collapse, or a panic; neither changes any rule or digest. The first player is a written strategy file for a `claude -p` session, run by hand, not yet a Relay step.
**In scope:** the play command, the observation text, `sweep --judge`, a score line for the led civ, one `relay/players/` prompt for a player and one for a judge, one trial game and its log in `relay/specs/<n>/`
**Out of scope:** a win or lose; driving the page; a Relay step; changing rules
```

Relay changes this implies (a proposal, for the owner):
1. **No new step kind in the first piece.** Prove the player on the trial game first, and see what its notes cost and say.
2. **If the trial pays, add a `play` step after `review`.** It is cut like a build (a model and effort from the ladder), not run by the coin. The piece line gets `**Play:** sonnet medium`, as `Spec` and `Spec-review` have. Pieces that change no player-visible thing set `**Play:** none`. The play session writes `play.md` (what it tried, what it could not tell, what failed); the review step reads it, or `play` runs after the review and its note goes into the PR description. A crash or a verdict of "confusing" doesn't block merge by default; the owner sees it. That keeps one stop for him, the merge.
3. **Who picks the model:** the spec designer sets it in `steps.json` with the other steps, like builds. Start on Sonnet (the point is breadth of play, not depth); use Opus for the judge pass only when the log is long. Haiku is worth a trial for the player on a compact text view: it is a game-playing loop with a small context, and a clear failure to read the view is itself a finding (a Haiku-sized confusion is a confusion).
4. **The coin's replacement:** the review has used the coin to decide whether to play. With a play step, "Miss: only play = 3" in the ladder stops pushing the review's cost up; the play step carries that job and its cost. This gives the owner a way to spend on a more expensive model where it pays: a judge on Opus reading play logs from several seeds.
5. **A milestone play check** is a manual or `/relay` command on main: N seeds, two strategies, one judge pass, one note for the owner. Cheap to start once the player exists.

## What an automatic judge can't tell the owner

- Whether the game is fun, or whether a decision is interesting *to him*. It can list where choices existed and where it did nothing. Interest needs stakes, and there is no win or lose yet.
- Feel: pace, the canvas's legibility at a glance, colour, sound of the feed. Text players do not see the map.
- Whether confusion is a human's. A model reads a compact view and a "Why" patiently; the owner will not.
- Balance across the strategies nobody wrote. A sweep and a bot test what the author thought of.
- Whether a passing band is the right band. Thresholds come from past sweeps, so they hold the game where it already is.
- It will agree with the design more than the owner would; judge prompts need "find five things wrong" and a rule that praise is not a finding.

So the owner still plays each milestone, but with a log of what a player stumbled on and a sweep that has already shown the world isn't broken, so his ten minutes go on taste.

## For other angles

- **cost:** reviews of the M2 pieces cost $0.31 to $2.28 and 3 to 24 minutes each on opus low and sonnet high, and found one substantive bug in four. A play step at an estimated $1 to $3 is the same order. #56's 24-minute review is the high one; it included rerunning `play-hold.mjs` after a merge fix.
- **prompts:** `session.md:24` tells sessions `play.mjs` exists; the review prompt (`relay/prompts/review.md`) names no way to play or sweep, and the ladder text puts "it must play a camp off its line" in each review's `why`. A shared player/judge prompt would replace that.
- **game:** `lead_orders` in `main.rs` is a fixed script of a led civ and the only one; the AI never plays a led civ. The play scripts per piece (`play-*.mjs`, 2.1k lines with `check.mjs`) repeat browser setup that `play.mjs` mostly provides.
- **architecture:** `main.rs` is 1010 lines with sweep, digest, lead orders and report in one file; the session interface and a judge are a reason to split a `cli/src/` module.
- **crew:** a `play` step needs headless Chrome only for (c2); (c) runs on the engine binary alone, so it needs no new environment.

## Ideas

| Idea | Value | Cost | Recommendation |
|---|---|---|---|
| `sweep --judge` in CI: invariants, camps kept, starvation vs control, collapse, speed | high | small | take |
| `sansfiat play` session interface (stdin JSON actions, text observation, `why`) | high | medium | take |
| Compact text view of a led civ's state and feed (shared by bot, LLM player, owner) | high | small | take |
| Score line for the led civ (people, clans joined, store, camp health) | medium | small | take |
| Strategy files (JSON) replacing hard-coded `lead_orders`; 3 to 4 named strategies | medium | medium | try |
| LLM player + judge prompts, one trial game, run by hand | high | medium | take (second half of the same piece) |
| `play` step after `review`, model from the ladder, `**Play:**` piece line | high | medium | try, after the trial |
| Let the AI play the led civ as a baseline player | medium | medium | later |
| Milestone play check on main: several seeds, strategies, one judge pass | medium | small | later |
| Vision model driving the page with screenshots (c2) | low | large | later |
| Haiku as the player on the text view | medium | small | try |
| Judge prompt that must list problems and may not praise; compare two builds' logs | medium | small | take |
| Balance bands as warnings in sweep output, thresholds from past sweeps | medium | small | try |
| Replace the per-piece `play-*.mjs` with one generic scripted page driver and a shared setup | low | medium | later |
| A win and a lose condition so a judge has something to measure | high | large | a design decision first; not for this piece |
