# M1 retro: the prompts and steering files (#31, #32)

What each session is told, set against what it did. Sources: the saved prompts and stream-json
transcripts of all 14 sessions (the `relay-runs-*` artifacts of 24–25 Oct), the step summaries
and decisions as later prompts carry them, and `relay/specs/31/`, `relay/specs/32/` on main.

The files: `relay/prompts/session.md`, `spec.md`, `spec-review.md`, `build.md`, `review.md`,
the prompt text `sessionPrompt` builds in `relay.mjs`, and `engine/CLAUDE.md`. There is no root
`CLAUDE.md` or any `AGENTS.md`. Every session reads `engine/CLAUDE.md` with `cat` on its first or
second command, because `session.md` points at it for the checks.

## Verdict

Most of the steering text is short and was followed. The damage came from three places. A step's
summary has no reader named, so the summaries grew until they made up most of every later
prompt. "The checks" is a pointer to a file that disagrees with CI. And two lines invite a
session to change something the crew never reads again. Each fix below cuts or replaces a line;
only one adds a sentence that has no line to replace.

## Findings, most severe first

### 1. Summaries snowballed into every later prompt

`session.md` says: "What you write in `result.json` is how the later steps, the owner and the
retro after the milestone learn what happened, so write what they need." That gives three
readers, and the retro reads everything, so nothing is out of bounds. `spec.md` never says that a
result lands in every later step's prompt, so #31's briefs asked for evidence in the result:
build-2's brief said "`sansfiat map --size large --seed 3` (and a small one) shown in your result
as ASCII … put the table in your result"; build-1 and build-3 asked for tables and counts the same
way.

Size of "This piece so far" in each #31 prompt, against the steering text (`session.md` plus the
kind's prompt, about 2.5 KB):

| Prompt | Whole prompt | "This piece so far" |
|---|---|---|
| #31 build-1 | 18 KB | 6 KB |
| #31 build-3 | 41 KB | 31 KB |
| #31 build-5 | 68 KB | 58 KB |
| #31 review | 103 KB | 66 KB |
| #32 review | 56 KB | 18 KB |

The build-2 and build-3 summaries alone are 19.6 KB and 20.3 KB. They carry a 30-row per-seed
river table, a full ASCII map of small seed 2 and the per-seed sweep lines. Build-5's page work
read all of it. The counter-example is in the same piece: #31 build-4 wrote its 90-run sweep to
`relay/specs/31/evidence.md` and named it, and its summary is 7 KB. #32's briefs asked for no
tables, and its build summaries are 3.5–4.3 KB.

Fix: name the reader and where evidence goes (wording below). The crew could also refuse a
summary over a few thousand characters, the way it refuses a `steps.json` that doesn't parse.
That makes it a check rather than a request.

### 2. Two lines invite a `steps.json` change the crew drops

`session.md`: "**spec-review** reads the plan the way the later sessions will, and fixes it."
`spec-review.md`: "…or a model or effort whose `why` doesn't hold." The crew makes every step from
`steps.json` straight after the spec, though, and never reads the file again (`relay.mjs`
around line 751). #32's spec-review split build-3 in two and added build-4 to `steps.json`. The
crew had already made the steps, so build-4 never ran, and the #32 review wrote the drawing,
feed, tooltip and legend itself ("Build-4 … was never committed, so I'm adding it as part of this
review"). Nobody reviewed that code. A model or effort that spec-review changes is dropped in the
same way.

`relay/retro/m1/briefs.md` (finding 1) and `cost.md` (finding 2) cover this in full. I agree with
the `spec-review.md` cut in `briefs.md`. `session.md`'s "and fixes it" needs the same limit (wording
below).

### 3. "The checks" points at a file that disagrees with CI

`session.md`: "The checks are the ones CI runs: in `engine/`, those `engine/CLAUDE.md` lists and
`./build-web.sh`." `engine/CLAUDE.md` lists twelve commands. Several are tools, not checks
(`map`, `run`, `sweep`, `bench`, `replay`, `clear`, `digest`, the http server). `cross-build.sh` and
`check.mjs` are listed, and CI doesn't run `check.mjs`. Each session worked out its own set:

- #31 build-2: "I also ran `./cross-build.sh`, though this step didn't require it."
- #31 build-1 ran five checks; #32 build-1 to build-3 ran six, `check.mjs` included.
- #31 build-5 wrote "CI runs it" into `engine/CLAUDE.md`, tried to push `ci.yml`, and was refused.
  The #31 review rewrote the line as "(not in CI yet: the engine job needs a `node www/check.mjs`
  step after `./cross-build.sh`)", which is still in the file.
- The same CI gap was handed to the owner four times: #31 build-5's summary, an idea from the #31
  review, #32 spec's summary and an idea from the #32 review. #23 already filed "Give the Relay app
  the workflows permission".

This rule is mechanical. It belongs in one script, which CI runs and the prompt names, not in
prose that every session has to read and interpret. `build.md`'s "leave every check passing after
your last edit" is then something the crew can confirm. Every build said its checks passed, but
only the session itself said so.

Fix: add `engine/check.sh` (fmt, clippy, test, `build-web.sh`, `cross-build.sh`, `node
www/check.mjs`) and have CI run it. That needs the owner, since the runner can't push workflow
files. `session.md` then names the script (wording below).

### 4. The builds rewrite the spec, and the review checks against the rewrite

`build.md`: "When the brief or the spec turns out wrong, correct them in your commit and say so in
your summary." The builds did exactly that. #31 build-2 "corrected spec decisions 7–8 to match
what I tuned", build-3 rewrote `spec.md` and its own brief, and build-4 updated decisions 13 and
14. That part works: every later build reads `spec.md` first (it's the first `cat` in each #31
build transcript), so they all built on the corrected plan.

But `review.md` says to review "against the piece, its spec and its design files", and its prompt
carries "The whole spec" as the builds left it. So the review checks the code against a spec the
code rewrote. The departures that most need judging are a build's own reading of the design:

- #31 build-2 lowered `delta_river` from 12 to 10 so that deltas appear on "most" medium seeds.
  That came to 11 of 20.
- #31 build-1 set the land-share test bound after measuring: 0.35–0.75, then 0.25–0.85, then
  0.40–0.80 ("found 0.48..0.75"). Build-2 rewrote its "Found over seeds 1..=15" comment twice as
  its tuning moved the numbers.

Neither the #31 review's summary nor its PR description mentions deltas or these bounds. Whether
11 of 20 is "most", and whether a bound picked from the output tests the design or only the code,
are judgements for the review. A build has every reason to say yes to both.

Fix: keep `build.md` as it is, and point the review at the departures (wording below).

### 5. "Decisions" turned into a hand-off channel

`session.md`: "`decisions` are the choices you made that the piece, the design and your brief left
open, and that a player or a later piece would notice." Each decision goes to the owner on the
step and the PR, and into every later prompt. Many build decisions are implementation notes for the
next build:

- #31 build-1: "Zone lives in map.rs next to Tile", "generate_with(radius, rng, salt) returns
  Generated {tiles, wind}"
- #31 build-4: "Event::json adds a `text` field"
- #32 build-1: "Card fields are snake_case in JSON"
- #32 build-3: "view().civs and view().units are objects keyed by id … build-4 reads them with
  Object.values", "Mouse buttons other than left and right do nothing"

The decisions the owner needs sit in the same list: #32 spec's "Clans' stores aren't emptied at
founding", which reads `founding.md` against its own wording, and #31 build-5's grouped
starvation lines. "Or a later piece would notice" lets anything in. A hand-off belongs in the
summary.

Fix: test each decision by whether the owner might overrule it (wording below).

### 6. The review fixed a missing step instead of reporting it

`review.md`: "Fix what is substantive yourself and leave nits." On #32 the review treated a whole
missing step as something to fix, wrote it, and reviewed its own work before giving a merge
verdict. With finding 2 fixed this shouldn't come up again, so I propose no new line. If it does
come up, "a step's work missing" belongs in the verdict, not in the fixes.

### 7. `engine/CLAUDE.md` grows with every piece

No prompt tells a build to update it, but because it exists, every build did: +25 lines in #31
and +20 in #32. It's now 11 KB. The Layout rows restate function signatures and call order: the
`www/` row is 2,165 characters and the `units.rs` row 1,106. That copies the code, and sessions
read the code anyway. #31 build-2 read `CLAUDE.md`, then all of `map.rs`, `data.rs`, `land.rs`
and `round.rs`. #32 spec read every file in `core/src`. Each piece adds a layer, and each layer is
more for a build to keep true.

These parts are live, because nothing else tells a session what they say: the command block
(sessions ran `map`, `sweep` and `bench` from it), "Adding a mechanic", and "Invariants".

Fix: cut each Layout row to what the file owns, in one clause. Delete the CI parenthetical once
CI runs `check.mjs`, or once `check.sh` exists.

### 8. No-ops and stale lines

These made no visible difference:

- `session.md`: "Commit work that passes its checks as you go." Sessions ran 4–23 minutes of the
  90, and each committed 1–3 times near its end. Keep the 90-minute fact, which #32 spec-review
  reasoned from. Cut the instruction.
- `session.md`: "in a prototype, `npm test` in its folder." No M1 piece touched a prototype, and
  merge4 is frozen. A piece that does touch one can say so in its body, as next-iteration §4
  already does for archiving.
- `session.md`: "`wasm-bindgen-cli` 0.2.128". `engine/CLAUDE.md`'s invariants already pin it to
  `web/Cargo.toml`, and no session touched it. "wasm-bindgen-cli" without the version is enough.
- `session.md`: "(not committed)" after `relay/runs/result.json`. `.gitignore` already has
  `relay/runs/`.
- `relay.mjs`: the closing line "Do this one step, then finish as "Finishing" says." All 14
  sessions finished properly, the header names the one step, and `session.md` ends on
  "Finishing".
- `spec.md`: "; the retro reads it" after "say why in `why`". `cost.md` finding 3 shows the whys
  justify the plan to a reader ("as #31's review did well on sonnet high") rather than choose a
  model and effort.

Kept because they did change behaviour, or guard something:

- `spec-review.md`'s example "a test whose numbers can't pass on the current map". It sent #31's
  spec-review to measure: a 300-turn small run took 26 s unoptimised, which led to `opt-level =
  3`, and build-1 measured `cargo test` going from 1m50s to about 10 s. It also caught a topped-up
  market starting at activity 0, which would have faded.
- `session.md`'s `ideas` line. No session put out-of-scope work into code, and each step filed 0–5
  ideas.
- `session.md`'s "what it says is decided". #32 spec logged its reading of `founding.md` as a
  decision rather than quietly changing it.
- `review.md`'s mechanics: merge `origin/main`, rewrite the PR body, `gh pr checks`, `"merge":
  true`. Both reviews followed them.
- `session.md`'s "Never print secrets or the environment". It's a guardrail, and no session broke
  it.

## Proposed wording

The owner owns `relay/prompts/`, so these are proposals. Cuts first.

`relay/prompts/session.md`:

```diff
-2. **spec-review** reads the plan the way the later sessions will, and fixes it.
+2. **spec-review** reads the plan the way the later sessions will, and fixes the spec and briefs.
```

```diff
-No one watches in between. What you write in `result.json` is how the later steps, the owner and
-the retro after the milestone learn what happened, so write what they need. Each later step
-sees every earlier step's summary and decisions.
+No one watches in between. Your `summary` and `decisions` go into every later step's prompt
+and onto the PR. Keep the summary to what the later steps act on. Tables, maps and sweeps go in
+a file in `relay/specs/<piece>/`, and the summary names the file.
```

```diff
-- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli` 0.2.128 and Node 24
-  are installed. The checks are the ones CI runs: in `engine/`, those `engine/CLAUDE.md`
-  lists and `./build-web.sh`; in a prototype, `npm test` in its folder.
-- The crew stops a session after 90 minutes. Commit work that passes its checks as you go.
+- Stable Rust with the wasm target, rustfmt, clippy, `wasm-bindgen-cli`, Node 24 and Chrome
+  are installed. The checks are `./check.sh` in `engine/`, as CI runs them.
+- The crew stops a session after 90 minutes.
```

Chrome goes in because six of the fourteen sessions spent a call or two finding it with `which`.

```diff
-2. Write `relay/runs/result.json` (not committed):
+2. Write `relay/runs/result.json`:
```

```diff
-   - `decisions` are the choices you made that the piece, the design and your brief left
-     open, and that a player or a later piece would notice.
+   - `decisions` are the choices the piece, the design and your brief left open that the
+     owner might overrule.
```

`relay/prompts/review.md`:

```diff
-Review the whole piece as built, as a reader who didn't build it, against the piece, its spec
-and its design files. Read the code in the diff (`git diff origin/main...HEAD`), not only the
-tests. Your brief says where to look hardest. Fix what is substantive yourself and leave nits.
+Review the whole piece as built, as a reader who didn't build it, against the piece and its
+design files. Read the code in the diff (`git diff origin/main...HEAD`), not only the tests.
+The builds corrected the spec as they went: judge each departure, and each test bound a build
+picked from what it found, against the design. Your brief says where to look hardest. Fix what
+is substantive yourself and leave nits.
```

`relay/prompts/spec.md`:

```diff
-- `steps.json`: the spec-review, the build steps, then the review. Choose each step's
-  model and effort from its work, and say why in `why`; the retro reads it.
+- `steps.json`: the spec-review, the build steps, then the review. Choose each step's
+  model and effort from its work, and say why in `why`.
```

`cost.md` proposes a sentence for the same spot about Sonnet as the default. The two edits fit
together.

`relay/prompts/spec-review.md`: the edit in `briefs.md`.

`relay.mjs` `sessionPrompt`: delete `parts.push(`Do this one step, then finish as "Finishing" says.`);`.

`engine/CLAUDE.md`: cut each Layout row to one clause. For example, `units.rs` becomes "Units:
the warrior, making, feeding, marching, disbanding; `check_*` for `validate`." Then delete
"(not in CI yet: …)".

Outside the prompts:

- `engine/check.sh`, run by CI and named by `session.md` (finding 3). The CI step is the owner's.
- Optionally, the crew refuses an oversized `summary` as `pending` (finding 1).
