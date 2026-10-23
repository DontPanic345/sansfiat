# Every Relay session

## The runner

- Stable Rust with the wasm target, `wasm-bindgen-cli` 0.2.128, Node 24 and PowerShell are
  installed. The engine's checks are `cargo test` in `engine/` and `./build-web.ps1`
  (`shell: pwsh`). A prototype's checks are `npm test` in its folder.
- The crew stops a session after 90 minutes. Commit work that passes its checks as you go.
- Read what the step needs and no more: a session that fills its context window is a sign
  the step was too big.
- Anyone signed in to GitHub can read your transcript for seven days. Never print secrets
  or the environment.

## Scope and design

- Build only what your step says. Anything else you notice, however small or tempting,
  goes under `ideas` in your result, never into the code.
- `docs/design/` is Alan's. Read it; never edit it.
- A design question the design and the spec don't answer: settle it the way they point,
  and say what you chose and why in your result's `summary`. Design questions never block.

## Blockers

A **blocker** makes the step impossible without Alan: infrastructure or an account only he
can supply. Push what you have and finish with a `blocked` result. The crew asks Alan, and
his reply comes to the next session.

## Finishing

1. Commit everything with a message naming the piece and step, and push to your branch.
2. Write `relay/runs/result.json` (not committed):

   ```json
   {
     "status": "done",
     "summary": "Two or three sentences on what you did, and any design choice you made.",
     "ideas": [{ "title": "…", "body": "…" }]
   }
   ```

   `status` is one of:
   - `done`: the step is finished and pushed. You marked your own homework, so be sure.
   - `pending`, with `"note": "<where the next session picks up>"`: you ran out of room.
     Push what passes its checks first. Two unfinished attempts block the step.
   - `blocked`, with `"blocker": "<what Alan must supply, in one or two sentences>"`.

   `ideas` may be empty. The crew opens each as an idea issue for Alan.
