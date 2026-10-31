# #101 build-3 (canvas): the bands, the prompts, one trial game

The easel is built: Standing, the orders counter and `core/src/judge.rs` (build-1); `sansfiat sweep
--judge` with `engine/judge.toml` in CI (build-1, health bands only); `sansfiat play`, the text view and
`sansfiat judge` over a game (build-2). Read both builds' summaries, `spec.md` (items 18–20, and Left
unsettled) and `probe.md` first. You paint what couldn't be settled before running, and you may change
the easel where your runs show it wrong: say what and why in your summary.

You have 90 minutes. The trial game takes most of it and runs in the background, so start it as soon as
the player prompt is written, and do the bands while it plays.

## 1. The player and judge prompts

- **`relay/players/player.md`**: a session that plays a civ for Standing through `sansfiat play` and
  nothing else. It says what Standing is and that the game has no money; how to start or resume the
  game, the line grammar, and that every call is one or more lines piped in; that before each
  decision it writes `note meant: … did: … surprised: …`; to play 100 turns from the pick, using
  `end N` where nothing calls for a decision, and to finish with a `note` on what it couldn't tell
  from the view. It must not read the engine's code or design (it plays from what a player sees), and
  must not print the environment: its transcript is public.
- **`relay/players/judge.md`**: a session that reads the lines file and `sansfiat judge`'s output,
  can resume the game file to look (`why`, `view`), and lists problems only, in the four kinds of spec
  item 18, each with its turn and evidence. Praise isn't a finding; nothing found is a finding of
  nothing. Its output is markdown, one finding per line.
- Both are written for any model; the trial runs Sonnet.

## 2. The trial

One game on **small seed 2** (the retro map where Standing moves most on main, `probe.md`), leading the
civ you choose from the cards and say why. Build the binary in release first.

Start the player with your **Agent tool**: `subagent_type: "general-purpose"`, `model: "sonnet"`,
in the background, with the text of `player.md` and the binary's path and the game file's path in a
scratch directory (not the repo) as its prompt. It is a fresh session: it knows only that prompt. Do
not use `claude -p`: inside a Relay session the nested CLI isn't logged in, and `node
relay/usage.mjs` can't read the plan's windows (spec-review tried both). The Agent tool's result
gives the subagent's tokens, tool calls and minutes: record those, and the list dollars the tokens
come to at Sonnet's price, marked as an estimate. The plan points are this step's own; the crew
records them. Give the player a time in its prompt (about 45 minutes); if it is still playing at
55, stop it and judge what there is. Then judge the game with `judge.md` the same way, `model:
"sonnet"`, `effort: "medium"` (a judge doesn't run at low), and run `sansfiat judge` yourself.

If the player stops short of +100 (time, a crash, it gives up), that is a result: record the turn it
reached and why, and judge what there is. Don't run a second game unless the first failed for a cause
in the easel that you fixed, and there is time.

Record **`relay/specs/101/play.md`**: the map and civ; tokens, tool calls, minutes and estimated list dollars for the player and for
the judge; the turns reached and the decisions made; `sansfiat judge`'s score lines and the led civ's
Standing against the control's; the judge's findings; the player's last note on what it couldn't tell;
and your answer to the piece's question in two or three sentences. Copy the game and lines files into
`relay/specs/101/trial/`.

## 3. The bands

From `build-1-judge.json` and your own runs on the branch, write every band into `engine/judge.toml`
with its `note`, and a table in **`relay/specs/101/bands.md`**: each metric, main's value, the band, and
why there. The aim: a piece that undoes another's work fails CI; a piece that moves a score it means to
move changes one band and says so. In practice, the health bands fail and the four scores warn (their
whole point is that later pieces move them), unless you find a score whose fall is always a bug.

Prove the bands catch something: perturb one number in `data.rs` that undoes a gain (halving the
default gift, as #75 halved the joins, or another you choose), run the judge, record what it said in
`bands.md`, and revert. Set `JUDGE.lead_margin` and `JUDGE.sample` from what the lead changes and
swings look like on the seven maps; say what you chose in `bands.md`.

## 4. The lines

Root `CLAUDE.md`, under Checks: one line for the judge (`sweep --judge`, the bands in
`engine/judge.toml`, a piece that moves a band says so in its PR) and one for play (`sansfiat play
--game`, the players' prompts in `relay/players/`, `sansfiat judge`).

## What proves the painting

- `play.md` answers the question with the trial: the turns reached, the findings, the cost.
- `./check.sh` passes with your bands; the perturbation fails or warns and is reverted.
- `bash relay/specs/100/baseline/compare.sh`: six `ok`.

The review reads `play.md`, `bands.md` and the trial files, plays twenty turns itself, and reruns the
judge.
