# Every Relay session

## How Relay works

A piece is an issue the owner agreed with Claude before it was opened, so what it says is decided.
Relay builds it as a run of steps, each a cold session like you, on the piece's branch:

1. **spec**: the designer plans the piece: a spec, a brief for each build, and `steps.json`.
2. **spec-review** checks the plan against the code and the maps by running things, and fixes
   it. The crew then makes the steps from `steps.json`.
3. **build-1** to **build-n** each build what their brief says.
4. **review** reviews the whole piece, fixes it, and adds its decisions to the design's ledger.

No one watches in between. Each later step sees every earlier step's summary and decisions,
and the owner reads them on the PR.

`docs/design/` is the owner's: never edit its prose. Record your decisions in `result.json`,
and the review adds them to the design file's `## Decided` ledger. Anything you notice beyond
your step goes under `ideas`, where the owner sees it.

## The runner

- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli`, Node 24 and Google
  Chrome are installed. `engine/check.sh` runs every check CI runs on the engine.
  `engine/www/play.mjs` drives the page in headless Chrome.
- The crew stops a session after 90 minutes.
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
     "ideas": [{ "title": "…", "body": "…" }]
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
     might overrule. How you built it is the diff, not a decision. Under five is usual.
