# #2 spec: a root CLAUDE.md for sessions

A polish. It adds one file, `CLAUDE.md` at the repo root, so that a Claude Code session
started at the root (every Relay session is) learns from it alone what the repo holds and
which checks to run where. No code changes, and no change to any other file.

It builds no game rules from `docs/design/vision.md`. It points at `docs/design/` as the
place the game's design lives, and doesn't restate it.

## Before and after

**Before.** There is no root `CLAUDE.md`. A session at the root loads no project memory.
`engine/CLAUDE.md` and the prototypes' own `CLAUDE.md` files load only once the session
reads files in those folders. The root `README.md` lists the folders and says the JS
prototypes run `npm start`, `npm test` and `npm run sim`, but it isn't loaded into a
session, and it says nothing of `cargo test`, `build-web.ps1`, `docs/design/` or `relay/`.
A Relay session finds the checks only because its prompt (`relay/prompts/session.md`)
lists them.

**After.** A session at the root loads `CLAUDE.md`, which names every top-level folder,
says where the design lives and that it's Alan's, points at `engine/CLAUDE.md` (and each
prototype's own `CLAUDE.md`), and gives each check with the folder it runs in.

The build step proves it with a cold `claude -p` asked, with its tools off, for the checks
and the layout: on `main` it can't answer; on `relay/2` it can.

## Scope map

| Step | In-scope line it serves |
|---|---|
| build-1: Write the root `CLAUDE.md` | A root `CLAUDE.md` that tells a session what the repo holds (the engine, the prototypes, `docs/design/`, `relay/`) and points it at `engine/CLAUDE.md` |
| build-1: Write the root `CLAUDE.md` | The build and test commands: `cargo test` in `engine/`, `./build-web.ps1`, and `npm test` in a prototype's folder |
| review | Both lines: checks the file against them and the out-of-scope list |

## Design questions

1. **Does the root `CLAUDE.md` list Relay's own check, `node --test relay/*.test.mjs`?**
   CI runs it (`ci.yml`, job `relay`), but the in-scope commands name only the engine's and
   the prototypes'.
   - (a) List it, on the `relay/` line or in the commands block, run from the root.
   - (b) Leave it out: `CLAUDE.md` gives only the three commands the piece names.

   **Recommend (a).** "How to tell it's better" asks that a session finds *the checks*,
   and a session that touches `relay.mjs` would otherwise miss the one CI runs on it. It's
   one line and describes `relay/`, which the first in-scope line covers. The build brief
   follows (a); if Alan picks (b), drop that line from the brief at sign-off.

## Design

### Files touched

- `CLAUDE.md` (new, repo root). Nothing else.

### What `CLAUDE.md` holds

It's written in the style of `engine/CLAUDE.md`: a title, a short paragraph, a commands
block, a layout table, and a few short rules. Plain, short sentences; about 30 to 50 lines.

1. **Title and paragraph.** `# Sans Fiat`. One or two sentences: a hex 4X with no money,
   built here as a Rust engine and a set of standalone JS prototypes. The game's design is
   in `docs/design/`: start at `docs/design/vision.md`. No rules or numbers restated.

2. **Checks.** A fenced block with each check and the folder it runs in. Every command
   is written so it runs as given from bash (the shell sessions use) and from pwsh:

   ```
   cd engine && cargo test                 # the engine
   cd engine && pwsh ./build-web.ps1       # the wasm front end; needs the wasm32 target and wasm-bindgen-cli 0.2.128
   cd <prototype> && npm test              # poc, hex-economy, hex4x, scout, supply-lines
   node --test relay/*.test.mjs            # the crew, from the root (design question 1)
   ```

   `build-web.ps1` is mode 644 with no shebang, so a bare `./build-web.ps1` fails from
   bash; `pwsh ./build-web.ps1` works from either shell. The file says the script is
   `./build-web.ps1` in `engine/`, run through `pwsh`. One line after the block: run the
   checks for the folders you touched; CI (`.github/workflows/ci.yml`) runs all of them.
   The prototypes have no dependencies, so `npm test` needs no `npm install`.
   `paint-army/` has no tests.

3. **Layout.** A table of every top-level folder:

   | Path | What |
   |---|---|
   | `engine/` | The Rust barter-market engine (core, CLI, wasm web front end). Read `engine/CLAUDE.md` before working in it. |
   | `poc/`, `hex-economy/`, `hex4x/`, `scout/`, `supply-lines/` | Standalone JS prototypes: plain ES modules, no build step, no dependencies. Each has its own `CLAUDE.md` with its commands and layout. |
   | `paint-army/` | A single-page territory-painting experiment. No tests. |
   | `docs/design/` | The game's design: `vision.md`, then one file per mechanic. Alan's: read it, never edit it. |
   | `relay/` | Relay, the crew that builds pieces in cold sessions: `relay.mjs`, its prompts, and `specs/<piece>/` on each piece's branch. `relay/README.md` explains it. |
   | `.github/` | CI (`workflows/ci.yml`), Relay's workflows, issue forms, labels. |

   Wording is the build session's; these are the facts to carry.

4. **Rules.** A few lines:
   - `docs/design/` is the design. The prototypes' own docs (such as `poc/docs/`) describe
     what was built, not what the game should be (as `docs/design/README.md` says).
   - Each folder stands alone: a change in one doesn't need the others' checks.

### Tests

No code changes, so no test files. The build step proves the file by:

- running every command in `CLAUDE.md`'s checks block exactly as written from the root,
  and seeing each pass;
- checking every path `CLAUDE.md` names exists (`git ls-files` / `ls`);
- the cold-session check: `claude -p --model haiku --tools ""` from the root, asked which
  commands check the repo and where each runs, and what `docs/design/` and `relay/` are,
  once in a worktree of `origin/main` (before) and once on `relay/2` (after). Its two
  answers go in the step's summary as the before and after.

CI's `js`, `relay` and `engine` jobs must stay green; nothing they run changes.
