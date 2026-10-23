# review: Review the piece

The piece adds one file, `CLAUDE.md` at the repo root, so a session started at the root
learns from it alone what the repo holds and which checks to run where. It's a polish: no
code changes, no new rules. Read `relay/specs/2/spec.md` for the whole design.

The piece is done when "a new session finds the checks and the layout from `CLAUDE.md`
alone, without being told in its prompt."

## Look hardest at

1. **The diff is one file.** `git diff --stat origin/main...HEAD` shows `CLAUDE.md` and
   `relay/specs/2/` only. Any change to code, `docs/design/`, `engine/CLAUDE.md`,
   `poc/CLAUDE.md`, other prototype docs or `README.md` is out of scope: revert it and
   list it under `ideas`.
2. **Every command is right as written.** Run each line of the checks block from the root
   exactly as given: `cargo test` and `pwsh ./build-web.ps1` in `engine/`, `npm test` in
   each of `poc`, `hex-economy`, `hex4x`, `scout`, `supply-lines`, and
   `node --test relay/*.test.mjs` from the root. A bare `./build-web.ps1` fails from bash
   (the script is mode 644 with no shebang); the file must give a form that runs.
3. **The facts are true and complete.** Every top-level folder is in the layout; every
   path it names exists; it points at `engine/CLAUDE.md`, says the prototypes have their
   own `CLAUDE.md`, and says `docs/design/` is Alan's to edit, read-only for sessions.
   It matches `.github/workflows/ci.yml` (which prototypes have tests; `paint-army/` has
   none).
4. **It restates no design.** No game rules or numbers from `docs/design/`: it points
   there. A one-line description of the game is fine.
5. **It reads cold.** Read `CLAUDE.md` as a session with nothing else would: short, plain,
   in the style of `engine/CLAUDE.md`. Check the build step's before-and-after cold-session
   answers in its summary on the PR; if they're missing, run the check from
   `relay/specs/2/build-1.md` ("A cold session finds it") yourself.

Fix what's wrong in `CLAUDE.md` directly. There are no test files to add: a fix is proven
by the command or path it corrects running or existing. Leave wording nits.

Then wait for CI (`gh pr checks --watch`): the `js`, `relay` and `engine` jobs must be
green.
