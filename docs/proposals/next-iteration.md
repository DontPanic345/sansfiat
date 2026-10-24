# Next iteration

The plan for M1: a lighter Relay that runs without a babysitter, every branch playable in the
browser, the pre-game merged, and the four prototypes rebuilt on the engine.

## 1. How a piece starts and ends

Alan doesn't write piece issues or sign off specs. Claude brings a proposal like this one; once
Alan is happy with it, Claude opens the piece's issue from it, labels it `relay:ready` and starts a
run. Every decision Alan cares about is made in the proposal, so it's in the issue body.

The issue body has what the spec needs, in Claude's words:

- **Goal**: what the piece delivers, or the one question a proof answers.
- **Ends**: on main (it changes the game), or on its branch (a proof).
- **Design**: the `docs/design/` files it builds.
- **Decided**: the decisions from the proposal.
- **In scope** and **Out of scope**.
- **Spec**: the model and effort for the spec step, and why. The crew reads it from the body;
  no model is fixed in code. The spec then chooses every later step's.
- Anything the work needs that a session wouldn't know to do, such as merging `main` first or
  archiving a prototype folder it replaces.

There's one kind of piece. A proof stays on its branch, tagged `proof/<n>` when it closes, and
section 3 makes it playable there. A proof that turns out well becomes a piece that ends on main.
Merging proofs into a `proofs/<n>/` folder was considered and dropped: proofs change shared code
(`engine/core`, a prototype's `src`), so a copy forks it.

All issue forms go, and `config.yml` turns blank issues on. The crew makes its own step and log
issues from code, so it needs no templates.

Removes: `type:*` labels, the seven forms, `type-*.md` prompts (done), the "no type label" hold,
merge/milestone gating and `milestoneRank`, and `parseForm` (the spec reads the body as prose).
Pieces that end on main still queue behind each other; a proof never waits.

## 2. Spec, spec-review and decisions

A piece runs **spec → spec-review → build-1…4 → review**, with no stop between them.

- **Spec** makes the decisions the piece leaves open, the way the piece and the design point,
  and records the noteworthy ones in `spec.md`'s `## Decisions`: anything a player would notice
  or a later piece will build on. No design questions, no sign-off.
- **Spec-review** stays. One run per piece (it ran once on each of #2 and #13), now sized by the
  spec: `steps.json` starts with it, so its model and effort come from the spec like every other
  step. Its job: read each brief cold, check the numbers work, cut work outside the scope, and
  catch decisions a brief settled quietly. The retro can remove it later if the evidence says so.
- **Build and review** sessions record their own decisions in `result.json`'s `decisions`. The
  crew shows them to the review step and lists them on the PR, so the retro can read them.

**"The numbers work"** is about numbers the spec picks, not ones Alan writes. On #13 the spec chose
when a clan is fed, when it divides, and when a centre forms, and the end test checked for centres.
Spec-review found clans on two-food land could never be fully fed, so they'd never grow, and the
test could never pass; the build would have burned a session finding that out. Checking a test can
pass on the current map, by arithmetic or a quick run, is cheap before building and expensive
after.

**Models.** No defaults and no rules: the spec chooses each step's model and effort from the work
and explains it in `why`, and every choice needs one, Sonnet as much as Opus. On #13 the old
rule ("Opus: rules that are subtle or interact with other mechanics") was followed to the letter,
which put build-2 and build-3 on Opus high; build-3 cost $2.14 against $1.17 for Sonnet's build-4.
A rule gets followed; a choice gets thought about. The retro looks at the choices against what
they cost and how the steps went, and gives guidance for the next milestone's proposals. The
spec step's own model, fixed on Opus high in `relay.mjs:773` today, comes from the piece.

**What later steps see.** Every step's prompt carries every earlier step's summary and
decisions under "This piece so far", as Jogging carried the piece's log entries. So an earlier
step writes down what later ones need in its summary, with no length cap, and a build that
finds its brief wrong fixes the brief too.

Crew changes: the spec step's model from the piece; the spec-review step from `steps.json`;
`pieceAction` goes straight from spec-review to the build steps; `signOff`, the `waiting` label on
sign-off and `/relay go` go; `result.decisions` kept on the step and shown with the summaries.

## 3. Every branch in the browser

Pages is on at https://dontpanic345.github.io/sansfiat/. The `pages` job in `ci.yml` runs only on
main: it copies the prototypes into `_site/`, adds the wasm engine under `engine/`, and writes an
index. Branches never appear. Every prototype is plain static files loaded by relative path, so
any of them works under a subfolder as-is; only the engine needs a build.

GitHub Pages is one site per repo and each deploy replaces all of it, so "every branch playable"
means one deploy with every live branch built into a subfolder:

```
/                     main, as today
/b/relay/13/engine/   a piece's branch
/b/proof/7/engine/    a closed proof, kept as a tag
```

1. Move the site assembly out of `ci.yml` into `tools/site.sh <out>` on each branch, so a branch
   that adds a prototype publishes it without a workflow change. Branches without it use main's.
2. A new `site.yml`, started by `workflow_run` on CI (which runs in main's context, so the
   `github-pages` environment allows the deploy) and on branch delete. It builds main, every
   `relay/*` and `claude/*` branch and every `proof/*` tag, and deploys once. `concurrency` keeps
   only the latest rebuild.
3. The index lists main's prototypes, then a row per branch (piece, PR, CI status, last commit,
   a link to each folder it built), then closed proofs. Branches that failed to build are listed.
4. The review step puts the play link in the PR body. When a proof closes, the crew tags its
   head `proof/<n>`, so the branch can go and the proof stays playable.

Engine rebuilds per branch are the cost; if they get slow, cache `www/pkg` per branch keyed on the
`engine/` tree hash. Cloudflare Pages or Netlify would give real per-branch URLs but need an
outside account and token, so they're the fallback.

## 4. Prompts

Rewritten to explain the flow and leave the judgment to the sessions, using `writing-for-agents`,
on top of Alan's changes. 96 lines across the prompts, down from 171. Not committed: they have to
land with the crew changes in sections 1, 2 and 5, because the new `steps.json` lists spec-review
and today's crew would make it twice.

- `session.md` opens with how Relay works: the four kinds of step, that no one watches between
  them, and that `result.json` is how later steps, Alan and the retro learn what happened. Then
  the runner (checks point at `engine/CLAUDE.md`) and finishing.
- `spec.md` says what the spec, the briefs and `steps.json` are for and who reads them. Every
  step's model and effort is the spec's choice, with a `why`. A proof answers its question and
  writes its findings into the PR.
- `spec-review.md` is one paragraph: read the plan as the later sessions will, and fix what would
  trip them up.
- `build.md` and `review.md` are a paragraph each.
- `type-*.md` are deleted. What a piece needs beyond the flow (merging `main` first, archiving a
  prototype) goes in its issue body.

## 5. Upkeep

Evidence is one day of Relay (2026-10-23 to 24): 2 pieces, 14 step issues, 3 PRs, 4 crew runs. The
core loop works; most of the rest hasn't run yet, and Relay plumbing commits matched game commits
one for one.

| Part | Change |
|---|---|
| Issue forms, 7 files | Cut (section 1) |
| Labels, 23 | Keep `kind:piece`, `kind:step`, `kind:idea`, `kind:chore`, `relay:ready`, `relay:paused`, `waiting`, `scope-cut`. 15 go, with `labels.yml` kept to sync the rest |
| Slash commands | `go` goes with sign-off. `run`, `retry`, `skip`, `model`, `pause`/`resume`, `status`, `scope` stay. Replying to a blocker and `/relay retry` start a run, as `/relay run` does, so nothing waits for a babysitter. `relay-events.yml` only starts on `/relay` comments or a blocked piece, ending the ~40 skipped runs from bot comments |
| Closing a piece | Nothing closes its steps today. `relay-events.yml` also runs on `issues: closed` for a `kind:piece`: the crew closes its open steps as not planned and, for a proof, tags the branch |
| Projects board | Kept (section 5a) |
| CODEOWNERS | Cut. It does nothing in a solo repo without branch protection |
| Secret scrubbing | One gap to close (section 5b) |
| Sub-issues with hidden JSON | Keep for now |

Tidy now: #2's draft PR #9 and steps #10 and #11 are still open with #2 closed.

### 5a. The board

What it needs, all on Alan's side:

1. A project called exactly **Sans Fiat Work** on the DontPanic345 account, with the fields in
   `relay/README.md` (only the ones that exist get set).
2. A classic personal access token with the `project` scope, stored as `RELAY_BOARD_TOKEN` in the
   repo's `relay` environment.

The secret is set: runs show it masked. No run has logged a board error, but the last one that ran
steps (17:12 on 2026-10-23) left no board line either way, and `gh` here can't read projects, so
whether it's writing is unconfirmed. Open the project after the next run that starts a step. If
it's empty, the run log's board line says why. The crew already adds both pieces and steps; with
piece types gone, the "Piece type" field goes and an "Ends" field (main, branch) can replace it.

### 5b. Secrets

What's right:

- A session's environment loses `RELAY_APP_KEY`, `DISPATCH_TOKEN` and `RELAY_BOARD_TOKEN`
  (`relay.mjs:666-668`). It keeps `CLAUDE_CODE_OAUTH_TOKEN`, which `claude` needs.
- Git and `gh` get the App's installation token from a file, through a credential helper and a
  wrapper, so it's never in the environment, and it's renewed every five minutes.
- Transcripts, stderr and crash reports go through `redact` for all four secrets before they're
  written (`relay.mjs:701, 720, 724, 1109`), and Actions masks them in the job log.
- Scanning #13's downloaded transcripts found no installation token and no OAuth token.

What's missing:

- **The installation token isn't redacted.** `SECRETS` lists environment variables, and the
  token lives in a file, so a session that ran `cat` on the token file or `git config -l` with it
  expanded would write it into the transcript in the clear. It lasts an hour and can write to
  the repo. Fix: redact the current token too, or every `ghs_…` pattern.
- Redaction covers what the crew writes, not what a session pushes or posts itself: a token in a
  commit, a PR comment or a file under `relay/specs/` gets through. Pieces come only from Alan,
  so this would take a session going badly wrong; it's worth knowing, not worth code.
- `session.md` told sessions transcripts are public for seven days; the artifact is kept for 14.
  Fixed in the prompt.

## 6. Cost and usage

### Cost

The transcripts settle both questions:

- **Cache tier: 1 hour.** Every cache write in #13's build-3, build-4 and review was
  `ephemeral_1h`; none were 5-minute.
- **`total_cost_usd` is enough.** The `result` frame carries it, priced by the CLI with the
  right tier. For Opus it matches the list price to the cent ($2.143). For Sonnet it's about 24%
  under the price table the subagent used ($1.17 against $1.54 for build-4), so one of the two
  Sonnet prices is off; worth checking once, not worth code.

So no price table in Relay. The crew records `total_cost_usd` on the step (summed across
attempts), and shows it in the step comment ("1.2M tokens, 41% context, 0 compactions, $3.10 at
list"), a "List $" board field and `/relay status`. A crashed or timed-out attempt has no `result`
frame and records nothing, so the retro counts those attempts separately.

A pure `rollup(steps)` totals a milestone by piece, step kind and model: list price, tokens,
attempts, crashed attempts, compactions and peak context.

### `usage.py` to `usage.mjs`

`usage.py` reads the rate-limit windows, from `/api/oauth/usage` or, failing that, a one-word
Haiku probe. Its only caller is `readUsage` (`relay.mjs:907-909`), which shells out to the
runner's `python3`, and `parseUsage` already repeats its payload handling. It moves to
`relay/usage.mjs` beside `token.mjs`, in the same style: about 70 lines and 40 of tests, with
`parseUsage` moving too and `usage.py`'s 174 lines going. One logged CI run confirms which route
answers there before the Python goes.

## 7. The retro

When a milestone's last piece closes, the crew opens a `retro` step on the milestone. One session
reads every piece's spec, decisions, step comments, ideas, blockers and cost, and Alan's play
notes, and opens a PR with two short lists:

- **Process:** edits to `relay/prompts/` and `relay.mjs`, each tied to its evidence.
- **Game:** proposed edits to `docs/design/`, and candidate pieces for the next milestone.

Alan owns both folders, so the PR is a proposal. What the retro reads for:

- each step's model and effort against its `why`, its cost and how it went, ending in guidance
  for the next proposals' and specs' choices,
- brief errors builds had to correct, and whether spec-review caught or missed them,
- decisions that should have been in the proposal,
- numbers that didn't work,
- ideas that show pressure on the scope.

A panel would be the same step run as several cold sessions, each with one of those lenses, and
one more that merges them. One session first; a panel if the first retros miss things.

## 8. Player layer

What the prototypes show today:

| Prototype | Status | Turn controls | Detail panel | Alerts | Help |
|---|---|---|---|---|---|
| hex4x, scout, supply-lines | Turn, player, winner | End turn | Tile and unit | Plain log | None |
| hex-economy | Turn, phase | Step, play, fast | Clan and tile | Log | None |
| merge4 | Turn, phase, win/lose | Step, play, fast-forward, keys | Tile, clan, market with a "why" log, module panels | Alerts, log with "mine only" | How-to-play panel and guide |
| engine (`relay/13`) | Turn, end state | Reset, end, ×10, play, run to end | Tile, clan, market | Error line | None |

Missing everywhere: a persistent top bar, a "what changed this turn" report, alerts that jump to
where they happened, tile tooltips, pan and zoom, a minimap, pending decisions before end turn, a
rivals overview, graphs over time, a start screen, in-game save, a tutorial.

**First piece: a "what happened" feed and a top bar, in the engine.** merge4 is a frozen as-built
reference, so anything added there is thrown away.

- `World` records structured events each turn, `{turn, kind, entity, tile, text}`: a clan
  divided, starved or moved, a market formed or faded, a hearth tamed, a centre formed. They come
  out through `Sim::events()` in `engine/web`, and the CLI prints them.
- `engine/www` gets a top bar (turn, phase, people, clans, markets, hearths, centres), end turn
  and speed, an event feed that centres the map on the event and selects it, and a tile tooltip
  naming the clan holding it.

It needs the pre-game on main, and no player mechanics. The resource bar, pan and zoom and the
minimap wait for founding.

## 9. Interviews

The four mechanic prototypes first, one interview each, each ending in a design file the proof
builds. `hex4x/` is the base 4X the others grew from, with no mechanic of its own; the military
and territory prototype is `paint-army/` (its paint-the-territory page was removed on 2026-10-23,
and `index.html`, "Paint Army", is what's left):

| Prototype | Mechanic | Opening questions |
|---|---|---|
| hex-economy | Barter market and trade | What makes trade pay: diets or specialisation? Does the festival bonus stay? Kin terms? When do traders and bead tokens come in? Much of it is already in the pre-game. |
| paint-army | Captains, territory and armies | Territory from troops only, or clans and cities too? What happens to cut-off detachments? Is 3 captains real? Entrenchment and retreat? |
| scout | Scouting, trails and fog | What does a scout cost? Are clan trails and scout trails one thing? Opaque or soft fog? Is a map tradeable? |
| supply-lines | Supply lines | Do enemy units cut lines, or only territory? Do lines carry goods for projects? Do pack animals or rivers extend reach? |

The rest later, one at a time: the boon year and founding, the civ economy (merge4's pooled store
is close to a treasury), expansion and joining, discoveries and years, map generation, AI rivals
and winning.

### How to talk it

**Pick: the Claude Code CLI with voice in, an interviewer style, and replies read aloud.**

- [`/voice tap`](https://code.claude.com/docs/en/voice-dictation) in Windows Terminal: tap Space,
  talk, tap again and it sends. Needs the claude.ai sign-in; doesn't count against usage.
- An [output style](https://code.claude.com/docs/en/output-styles),
  `.claude/output-styles/interviewer.md`: one question at a time, short, no headings or bullets,
  push on vague answers, and append each settled point to `docs/design/<mechanic>.md` as it goes.
- An async [`Stop` hook](https://code.claude.com/docs/en/hooks) that passes
  `last_assistant_message` to Windows' built-in speech (`System.Speech`). Robotic, but hands-free.
- [`/rc`](https://code.claude.com/docs/en/remote-control) to carry on from the phone.
- `/export` the transcript at the end, beside the design file.

The desktop Code tab has a mic button but no `/voice` and no hotkey. Win+H dictation works
anywhere as a fallback. **Runner-up:** claude.ai voice mode on the phone, which talks best but
can't write to the repo, so it ends with a Markdown summary pasted into Claude Code.

## 10. M1

1. **Tidy.** Close #2's leftovers (PR #9, steps #10 and #11). `poc/` is now `merge4/`.
2. **Relay changes, by hand**, in one Claude Code session: sections 1, 2, 5 and the cost half of
   6, with the prompts from section 4, as one PR.
3. **`usage.mjs`** (section 6).
4. **`site.yml`** (section 3).
5. **Interviews** on the four prototypes (section 9), Alan's time, alongside 2 to 4.
6. **Merge the pre-game.** A piece that ends on main, bringing `relay/13` over: it merges `main`
   first, then takes what PR #16 found (centres formed on 3 seeds of 10) as its first job.
7. **Rebuild the four prototypes on the engine** as four proofs, opened together, each from its
   interview's design file.
8. **Event feed and top bar** (section 8), ending on main.
9. **Retro** on M1 (section 7). The first one by hand from a Claude Code session; the crew step
   once its shape is clear.

Steps 2, 3 and 4 are independent and can run side by side in separate sessions.

**Opening the four proofs together** works: proofs never wait on each other. But the crew runs one
session at a time (`concurrency: relay`) and goes through pieces oldest first, so they build one
after another, each paced by the 50% five-hour budget. #13 took about seven sessions and a day;
four proofs is a few days. Running them truly in parallel would need a job per piece, and the
budget is the real limit, not the queue. They need the pre-game on main first, since each builds
on its clans and markets.
