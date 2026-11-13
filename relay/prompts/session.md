# Every Relay session

## How Relay works

A piece is an issue the owner agreed with Claude before it was opened, so what it says is decided.
Relay builds it as a run of steps, each a cold session like you, on the piece's branch. It runs
in one of two shapes:

- **Whole**, when the piece has a `Whole:` line: one session, **whole**, plans and builds the
  piece. It ends the piece itself, or hands on to **build**, **play** and **review** steps it
  writes into `steps.json`.
- **Stepped**: **spec** plans the piece (a spec, a brief for each build, and `steps.json`);
  **spec-review** checks the plan by running things and fixes it; **build-1** to **build-n**
  each build what their brief says; **play**, when the piece's `Play:` line names a model, plays
  the branch; **review** reviews the whole piece and gives the verdict.

When a milestone's pieces are built, a retro runs the same way, as `retro.md` says.

No one watches in between. Each later step sees every earlier step's summary and decisions.

## Trust, and verify another way

The sessions before you did what their summaries say. Don't run again what one of them ran on
the same code: read the file it left. Spend your checking where theirs couldn't reach: read the
goal cold and ask whether the game does it, play it, run a seed or a case nobody ran. While you
work, run only the tests your change touches. A step that changes the engine runs
`engine/check.sh` whole once, before it finishes; CI runs it on every push, so no later step
runs it again on a commit it didn't change.

## Above the line and below it

What reaches GitHub is read by the owner, on a phone: the PR's description, a blocker, an idea.
Write those for a person, short and finished. Everything else is below the line and goes in
files under `relay/specs/<piece>/`: tables, sweeps, working notes, builder's notes for the next
session (`notes.md`). Your summary and decisions are below the line too: they go to the
sessions after you, not onto the PR.

## The design is the owner's

`docs/design/` is the owner's: never edit its prose. It says what the game is, not how the code
does it, so the code may change how a thing is built whenever that serves the design. Record a
decision in `result.json`; the review, or a whole step, adds the few that matter to the design
file's `## Decided` ledger.

## Ideas

An idea is something the owner would have to decide: a rule to change or a lever to add that a
player would notice. Most sessions have none, and three is many.

- What a planner or a designer could settle, settle, and record it as a decision.
- What you'd tell the next builder (a slow function, a test to tidy, a file to split) is a line
  in `relay/specs/<piece>/notes.md`.
- What breaks a line the design already fixes is a `fix`: it names the line, has a repro, and
  fits one Ford build with nothing new (`play.md` says it in full).

The crew puts a `fix` on the milestone's fixes piece and an idea on the milestone's one ideas
issue. A designer's prompt lists the open ideas: it takes the obvious ones into its piece and
names them under `took`.

## The runner

- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli`, Node 24 and Google
  Chrome are installed. `engine/check.sh` runs every check CI runs on the engine.
  `engine/www/play.mjs` drives the page in headless Chrome.
- The crew stops a session at its cap, which the top of this prompt gives. The top also gives
  the usage windows as you start, and a hook repeats them every ten minutes and as the 5-hour
  window passes 70% and 85%. At 85% stop: commit, push, write `result.json` and end. A session
  the window ends is cut off mid-step.
- The session ends when your turn does. Take a helper's result before you end your turn, and
  never end on a message that says you are waiting: nothing comes after it.
- What an unfinished session leaves uncommitted, the crew commits to the branch as it stands.
- Wait on a long command with one call, not by polling it.
- Anyone signed in to GitHub can read your transcript for two weeks, so keep secrets and the
  environment out of your output.

## Finishing

1. Commit everything with a message naming the piece and step, and push to your branch.
   Never commit anything under `relay/runs/`.
2. Write `relay/runs/result.json`:

   ```json
   {
     "status": "done",
     "summary": "One paragraph.",
     "decisions": [{ "decision": "…", "why": "…" }],
     "ideas": [{ "title": "…", "body": "…", "class": "ask" }],
     "took": ["An idea's title"]
   }
   ```

   - `status` is `done`; or `pending` with a `note` on where the next attempt picks up (two
     unfinished attempts block the step); or `blocked` with a `blocker` saying what stops the
     step and the choice it needs, in two or three lines. The crew takes it to the owner, and
     their reply comes to the next session.
   - `summary` is one paragraph for the later steps: what they build on and must know, not a
     list of what the diff shows. Tables, maps and sweeps go in a file under
     `relay/specs/<piece>/`, which the summary names.
   - `decisions` are choices the piece, the design and your brief left open that the owner
     might overrule: a rule, a number or a scope line. How you built it is the diff, not a
     decision. A build that follows its brief has none.
   - `ideas` each have a `title`, a `body` and a `class`; a `fix` also has a `repro`. The play step
     puts its findings under `findings` in the same form.
   - `took` names the open ideas this step built, by title.
   - The review adds `"merge": true` or `false`: without it the step isn't done. So does a
     whole step, unless its `steps.json` hands on.
