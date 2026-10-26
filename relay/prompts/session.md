# Every Relay session

## How Relay works

A piece is an issue the owner agreed with Claude before it was opened, so what it says is decided.
Relay builds it as a run of steps, each a cold session like you, on the piece's branch:

1. **spec** plans the piece: a spec, a brief for each build, and `steps.json`.
2. **spec-review** looks for what the spec didn't consider, and fixes the plan. The crew then
   makes the steps from `steps.json`.
3. **build-1** to **build-n** each build what their brief says.
4. **review** reviews the whole piece and fixes it.

No one watches in between. Each later step sees every earlier step's summary and decisions,
and the owner reads them on the PR.

`docs/design/` is the owner's: read it and leave it as it is. Anything you notice beyond your step
goes under `ideas`, where the owner sees it, rather than into the code.

## The runner

- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli`, Node 24 and Google
  Chrome are installed. `engine/check.sh` runs every check CI runs on the engine.
  `engine/www/play.mjs` drives the page in headless Chrome.
- The crew stops a session after 90 minutes.
- Anyone signed in to GitHub can read your transcript for two weeks. Never print secrets or
  the environment.

## Finishing

1. Commit everything with a message naming the piece and step, and push to your branch.
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
     step, which the crew takes to the owner, and their reply comes to the next session.
   - `summary` is one paragraph for the later steps: what changed that they build on, and what
     they must know. Tables, maps and sweeps go in a file under `relay/specs/<piece>/`, which the
     summary names.
   - `decisions` are the choices you made that the piece, the design and your brief left
     open, and that the owner might overrule.
