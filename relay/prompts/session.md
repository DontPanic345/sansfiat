# Every Relay session

## How Relay works

A piece is an issue Alan agreed with Claude before it was opened, so what it says is decided.
Relay builds it as a run of steps, each a cold session like you, on the piece's branch:

1. **spec** plans the piece: a spec, a brief for each later step, and `steps.json`.
2. **spec-review** reads the plan the way the later sessions will, and fixes it.
3. **build-1** to **build-4** each build what their brief says.
4. **review** reviews the whole piece and gets CI green.

No one watches in between. What you write in `result.json` is how the later steps, Alan and
the retro after the milestone learn what happened, so write what they need. Each later step
sees every earlier step's summary and decisions.

`docs/design/` is Alan's: read it and leave it as it is. Anything you notice beyond your step
goes under `ideas`, where Alan sees it, rather than into the code.

## The runner

- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli` 0.2.128 and Node 24
  are installed. The checks are the ones CI runs: in `engine/`, those `engine/CLAUDE.md`
  lists and `./build-web.sh`; in a prototype, `npm test` in its folder.
- The crew stops a session after 90 minutes. Commit work that passes its checks as you go.
- Anyone signed in to GitHub can read your transcript for two weeks. Never print secrets or
  the environment.

## Finishing

1. Commit everything with a message naming the piece and step, and push to your branch.
2. Write `relay/runs/result.json` (not committed):

   ```json
   {
     "status": "done",
     "summary": "What you did, and what the later steps need to know.",
     "decisions": [{ "decision": "…", "why": "…" }],
     "ideas": [{ "title": "…", "body": "…" }]
   }
   ```

   - `status` is `done`; or `pending` with a `note` on where the next attempt picks up (two
     unfinished attempts block the step); or `blocked` with a `blocker` saying what stops the
     step, which the crew takes to Alan, and his reply comes to the next session.
   - `decisions` are the choices you made that the piece, the design and your brief left
     open, and that a player or a later piece would notice.
