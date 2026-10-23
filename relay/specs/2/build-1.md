# build-1: Write the root CLAUDE.md

Add one file, `CLAUDE.md` at the repo root, so a Claude Code session started at the root
learns from it alone what the repo holds and which checks to run where. This piece is a
polish: no code changes, no new rules.

## Why

Today there is no root `CLAUDE.md`. A session at the root loads no project memory:
`engine/CLAUDE.md` and the prototypes' `CLAUDE.md` files load only once it reads files in
those folders, and `README.md` isn't loaded at all (and doesn't mention `cargo test`,
`build-web.ps1`, `docs/design/` or `relay/`). Relay sessions find the checks only because
their prompt lists them. The goal: a new session finds the checks and the layout from
`CLAUDE.md` alone.

## What to write

Match the style of `engine/CLAUDE.md` (read it first): a title, a short paragraph, a
commands block, a layout table, a few short rules. Plain, short sentences; about 30 to 50
lines. The wording is yours; these are the facts to carry.

1. **Title and paragraph.** `# Sans Fiat`. One or two sentences: a hex 4X with no money,
   built here as a Rust engine and a set of standalone JS prototypes. The game's design is
   in `docs/design/`; start at `docs/design/vision.md`. Don't restate any game rule or
   number from the design: point at it.

2. **Checks.** A fenced block with each check and the folder it runs in, each runnable as
   written from bash and from pwsh:

   ```
   cd engine && cargo test                 # the engine
   cd engine && pwsh ./build-web.ps1       # the wasm front end; needs the wasm32 target and wasm-bindgen-cli 0.2.128
   cd <prototype> && npm test              # poc, hex-economy, hex4x, scout, supply-lines
   node --test relay/*.test.mjs            # the crew, from the root
   ```

   - `engine/build-web.ps1` is mode 644 with no shebang, so a bare `./build-web.ps1` fails
     from bash. Write it as `pwsh ./build-web.ps1` in `engine/`, and say that's the
     script `./build-web.ps1` run through `pwsh`.
   - After the block, one line: run the checks for the folders you touched; CI
     (`.github/workflows/ci.yml`) runs all of them. The prototypes have no dependencies, so
     `npm test` needs no `npm install`. `paint-army/` has no tests.

3. **Layout.** A table of every top-level folder, carrying these facts:

   | Path | What |
   |---|---|
   | `engine/` | The Rust barter-market engine (core, CLI, wasm web front end). Read `engine/CLAUDE.md` before working in it. |
   | `poc/`, `hex-economy/`, `hex4x/`, `scout/`, `supply-lines/` | Standalone JS prototypes: plain ES modules, no build step, no dependencies. Each has its own `CLAUDE.md` with its commands and layout. |
   | `paint-army/` | A single-page territory-painting experiment. No tests. |
   | `docs/design/` | The game's design: `vision.md`, then one file per mechanic. Alan's: read it, never edit it. |
   | `relay/` | Relay, the crew that builds pieces in cold sessions: `relay.mjs`, its prompts, and `specs/<piece>/` on each piece's branch. `relay/README.md` explains it. |
   | `.github/` | CI (`workflows/ci.yml`), Relay's workflows, issue forms, labels. |

4. **Rules.** A few lines:
   - `docs/design/` is the design. The prototypes' own docs (such as `poc/docs/`) describe
     what was built, not what the game should be.
   - Each folder stands alone: a change in one doesn't need the others' checks.

## Tests that prove it

No code changes, so no test files. Prove the file instead:

1. **Every check runs as written.** From the root, run each line of the checks block
   exactly as `CLAUDE.md` gives it (for `<prototype>`, all five: `poc`, `hex-economy`,
   `hex4x`, `scout`, `supply-lines`). All pass.
2. **Every path exists.** Each file and folder `CLAUDE.md` names exists
   (`git ls-files`, `ls`).
3. **A cold session finds it.** Commit `CLAUDE.md` first. Then ask a cold session with its
   tools off, once before and once after:

   ```
   git worktree add /tmp/before origin/main
   Q='Without using any tools, from what you already know of this repo: which commands check it, in which folder does each run, and what are docs/design/ and relay/? If you do not know, say so.'
   (cd /tmp/before && claude -p --model haiku --tools "" "$Q")   # before
   claude -p --model haiku --tools "" "$Q"                       # after, from the root
   git worktree remove /tmp/before
   ```

   If `--tools ""` isn't accepted, see `claude --help` for how to turn the tools off.
   Before, it can't name the checks; after, it names all of them with their folders. Put
   both answers, trimmed, in your summary as the before and after. If `claude` won't run
   (no auth), say so in the summary and rely on 1 and 2.

## Out of scope

- Any code change, and any file but `CLAUDE.md` at the root.
- `docs/design/`, `engine/CLAUDE.md`, `poc/CLAUDE.md` and the other prototypes' docs:
  don't edit them, even where they're stale (the `scout/` and `supply-lines/` files are
  titled `hex4x`; `hex-economy/CLAUDE.md` points at a Windows path). Put those under
  `ideas`.
- `README.md`: leave it as it is.
- Game rules or design: `CLAUDE.md` points at `docs/design/`, it doesn't restate it.
