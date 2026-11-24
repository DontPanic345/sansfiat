# M6: Relay on two vendors

Relay runs Codex sessions as well as Claude ones. Each subscription is spent at the pace its
week allows, a review runs on the other vendor from the work it judges, and every step's cost
reaches the board on the same footing. It is built by hand in one PR before M6's first piece
opens, and `relay/README.md` is updated to match.

## 1. What the probe found

One `codex exec --json` session (codex-cli 0.171.0, `gpt-6.2-sol`, low effort, "reply ok"):

- **The event stream** carries `thread.started` with the thread id and a `turn.completed`
  per turn with `usage`: `input_tokens` 19,204 (cached included), `cached_input_tokens`
  15,232, `output_tokens` 5 (reasoning included). It carries no model, no cost and no usage
  windows.
- **The rollout file**, `$CODEX_HOME/sessions/YYYY/MM/DD/rollout-*-<thread id>.jsonl`, carries
  the rest. `turn_context` names the model and effort. Each `token_count` event has the
  running totals, the last turn's tokens, `model_context_window` (475,000) and `rate_limits`:
  `primary` is the 5-hour window (`used_percent` 3, `window_minutes` 300, `resets_at`),
  `secondary` the week (43, 10080), and `plan_type` "plus".
- **The cost** is tokens at list price: uncached input 3,972 × $2/M + cached 15,232 ×
  $0.10/M + output 5 × $10/M = **$0.0095**. A one-word probe costs about a cent and under
  a point of the 5-hour window, so it can pace like Claude's probe does.

List prices per million tokens, input / cached / output: Sol $2 / $0.10 / $10, Astra $8 /
$0.40 / $40, and more past 272K input (Sol $4 / $15, Astra $16 / $60). These are from
third-party pages; check them against OpenAI's pricing page when `prices.json` is written.

## 2. Vendors

The vendor-specific parts of the crew move behind one shape, `relay/vendors/<name>.mjs`:
the session's command line, a parser from its stream to one record (model, result, cost,
context, compactions, usage windows, whether a limit ended it), the usage reading, the CLI
install, and its models. `claude.mjs` is what `relay.mjs` does today; `codex.mjs` is new.
A step's block gains `vendor`; nothing else in the crew knows which vendor ran it.

A Codex session runs:

```
codex exec --json -m <model> -c model_reasoning_effort=<effort>
           --dangerously-bypass-approvals-and-sandbox -
```

The runner is a throwaway VM, so the sandbox is off, as Claude's `auto` mode is: the session
needs the network for `git push` and `gh`. The crew reads the stream for the thread id and
turns, and tails the rollout file for the model, the windows and the context. Prices live in
`relay/vendors/prices.json`, a line a model.

- **Instructions.** Codex reads `AGENTS.md`, not `CLAUDE.md`. Each `CLAUDE.md` (root and
  `engine/`) moves to `AGENTS.md` beside it, and the `CLAUDE.md` becomes `@AGENTS.md`, so
  there is one text.
- **No hook.** Codex has no `PostToolUse`. The prompt gives the windows at the start, and
  the crew, which sees them in the rollout, stops the session at 85% of the 5-hour window
  itself. It already commits what an unfinished session leaves and tells the next attempt.
- **Sign-in.** Relay has a Codex sign-in of its own, made once on the desktop under
  `~/.codex-relay` and never used there again, so the desktop and CI never refresh the same
  token. `CODEX_AUTH_JSON` in the `relay` environment holds its `auth.json`, and the crew
  writes it to `$CODEX_HOME/auth.json` before a session.
- **Refresh.** Codex replaces the token as it runs. After every Codex session or probe,
  crashed ones included, the crew compares `auth.json` with what it started from and, when it
  changed, seals it with the `relay` environment's public key and writes it back to
  `CODEX_AUTH_JSON`. That takes the App's **Environments: write**, which writes environment
  secrets and variables but reads no secret and changes no environment's rules. When a Codex
  session or probe still fails to sign in, the crew routes everything to Claude, says in the
  run log "Codex sign-in: make a new one for `CODEX_AUTH_JSON`", and carries on.
- **Two tokens.** A session gets an installation token narrowed to what sessions use:
  contents, issues and pull requests write, actions read. The App's full token, with
  Environments, stays with the crew, so no session can write a secret.
- **The limit.** A rollout with `rate_limit_reached_type` set, or a 429, is a limit end: no
  attempt counted, the next waits for that vendor's reset.

## 3. Balance

Each vendor's week is spent at the same pace, not each vendor given the same steps.

- The pace rule runs per vendor: the 5-hour window under 75% (50% for a whole step), the
  week under day / 7 × 95%, each on that vendor's own cycle.
- A step goes to the vendor with the most room under its weekly line; a tie goes to the one
  with fewer points this week. A piece's `**Vendor:**` line pins its steps, and
  `/relay vendor <step> <claude|codex>` changes an open step.
- The crew stops only when both vendors are over their lines. The `wait` run sleeps until
  the first of them lifts.
- For M6, spec, play and the retro stay on Claude; builds and whole steps balance. The M6
  retro decides whether the rest balance too.

## 4. Cross-vendor reviews

`spec-review` and `review` run on the other vendor from the one that ran most of the points
they judge; for a whole piece, the vendor of its whole step. When that vendor is over its
line, the review waits up to six hours for it, then runs on the same vendor, and the PR says
so. A piece's `**Review:** same` opts out.

## 5. The ladder

`prompts/spec.md` keeps the points ladder and gains a Codex column. The designer picks the
rung; the crew picks the vendor, and the step runs that vendor's model for the rung. The
starting map, for the M6 retro to correct:

| Rung | Claude | Codex |
|---|---|---|
| haiku low / medium | haiku | sol low |
| haiku high | haiku high | sol medium |
| sonnet medium | sonnet medium | sol high |
| sonnet high | sonnet high | astra low |
| opus medium | opus medium | astra medium |

## 6. The board

- **Vendor**, a new single select: claude, codex.
- **Model** gains sol and astra.
- **List $** is Claude's `total_cost_usd` or Codex's tokens at list price, so the two add up.
- **5-hour points** and **Weekly points** are the step's own vendor's windows.
- The run log gives both vendors' windows, each week beside its day's share.
- A third view, **Vendors**: the Steps table grouped by Vendor, summing List $ and points.

## 7. What changes in the words

- `relay/README.md`: a Vendors section (sessions, balance, cross-review, sign-in), and Spend,
  the board and Running it for two vendors.
- `prompts/session.md`: "agreed with Claude" becomes vendor-neutral, and no prompt names a
  vendor the session may not be.
- `prompts/spec.md`: the ladder's Codex column.
- `prompts/retro.md`: the ladder angle reads cost and verdicts per vendor and per part, and
  the relay angle reads the balance: each vendor's weekly share against its line, and the
  steps that waited for the other vendor.

## 8. Guidance for the owner

- **Your own sessions share both windows.** Codex's Plus week was already at 43% when the
  probe ran. Heavy desktop use on either vendor moves Relay to the other one; that is the
  balance working, not a fault.
- **Plus is the smaller budget.** Expect Codex to take the smaller share of builds until the
  retro shows its points per window. Pro is the lever if Codex earns more of the work.
- **Read List $ for comparison, the points for spend.** On a subscription neither vendor's
  dollars are money; the windows are what run out.
- **The cross-review is the quality check.** A vendor whose builds the other sends back more
  often costs more than its List $ says. The retro's ladder angle counts that.
- **Keep `~/.codex-relay` Relay's.** Running Codex from it on the desktop refreshes the
  token and retires the copy CI holds. When the log says Codex can't sign in, make a new
  sign-in there (step 1 below) and paste it in again.

## 9. Your steps

1. Make Relay's sign-in: `$env:CODEX_HOME = "$HOME\.codex-relay"; codex login`, in a fresh
   PowerShell window. Paste `~\.codex-relay\auth.json` whole into a new secret,
   `CODEX_AUTH_JSON`, in the `relay` environment. Close that window.
2. On the sansfiat-relay App's Permissions & events, set Repository → **Environments** to
   Read and write. Leave Secrets and Agent secrets at No access. Accept the new permission
   on the installation.
3. On "Sans Fiat Work": add a **Vendor** single select (claude, codex), add **sol** and
   **astra** to **Model**, and make the **Vendors** view.
4. Read and merge the PR that builds sections 2 to 7.
5. Open M6's first small piece with `**Vendor:** codex`, and watch its log line and board row.
6. Drop the pin. From then on the crew balances, and the M6 retro reads the numbers.
