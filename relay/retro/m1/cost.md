# M1 retro: models, effort and cost

This covers #31 (The map and the page) and #32 (Founding). Sources: the `<!-- relay -->` block
on every step issue (#38–#53), the step comments, and the `crew.log` and transcripts of relay
runs 40100535820 to 40232705905 (24–25 Oct). "5h" is the 5-hour usage the crew probed before
each step, so the gap to the next probe is roughly what the step took from the subscription.
It's in whole points, and the owner's own use shares the window.

## The steps

| Piece | Step | Model / effort | List $ | Out tokens | Peak context | Minutes | 5h | How it went |
|---|---|---|---|---|---|---|---|---|
| #31 | spec | sonnet high | 0.93 | 43k | 116k (12%) | 6.1 | +7 | Builds later corrected three of its numbers or rules |
| #31 | spec-review | sonnet high | 0.90 | 31k | 119k | 5.8 | +7 | Six groups of fixes (raised markets that would fade, test radius, opt-level) |
| #31 | build-1 | sonnet high | 0.79 | 32k | 96k | 9.9 | +6 | Clean; no test moved |
| #31 | build-2 | **opus** high | 3.53 | 60k | 159k | 14.5 | +18 | Rivers. Fixed the brief's sources and delta threshold by measurement; anti-braid rule |
| #31 | build-3 | **opus** high | 4.74 | 83k | 185k | 23.1 | weekly +2 | Closed country. Added a site estimate and turn-taking (necks 6/30 → 24/30) |
| #31 | build-4 | sonnet high | 1.89 | 64k | 179k | 17.7 | +13 | Found the brief's top-up rule left seed 36 on two centres; fixed with a test |
| #31 | build-5 | sonnet high | 1.96 | 73k | 180k | 14.1 | +15 | The page, checked by headless Chrome screenshots |
| #31 | review | sonnet high | 1.65 | 28k | 177k | 11.9 | +10 | Fixed a doubled centre event and centre rings drawn under clans |
| #32 | spec | **opus** high | 3.38 | 58k | 192k (19%) | 10.2 | +18 | 12 decisions. Spec-review still made six fixes |
| #32 | spec-review | sonnet high | 0.84 | 24k | 120k | 4.1 | +6 | Split build-3 into 3 and 4 "to fit 90 minutes"; fixed a check.mjs assert that would have failed |
| #32 | build-1 | sonnet high | 1.02 | 33k | 125k | 7.8 | +8 | Clean |
| #32 | build-2 | sonnet high | 1.30 | 48k | 133k | 12.0 | +9 | Sent a disbanded unit's person to a rival civ's clan (the review caught it) |
| #32 | build-3 | sonnet high | 1.24 | 46k | 136k | 8.6 | +9 | Clean |
| #32 | review | sonnet high | 1.51 | 35k | 173k | 10.4 | ? | Also wrote build-4, which never ran as a step |

Totals: #31 $16.39 in 105 session-minutes, #32 $9.29 in 53, so $25.68 in all. Weekly usage
went from 9% to 25%. Every step finished in one session, and nothing crashed, timed out or
compacted. Every step ran at high effort, and Haiku was never chosen.

## Findings

### 1. Opus cost 45% and set the wall clock, and no step shows it was needed

The three Opus sessions (#31 build-2 and build-3, #32 spec) cost $11.65 of $25.68. Each took
about 18 points of the 5-hour window, more than a third of the 50% gate:

- After #31 build-2 the run stopped at 62% (14:47). The next run, at 15:58, stopped too.
- #31 build-3 then started at 20:50 and pushed weekly usage to 14%, past day 1's 13.6%. Its
  run stopped, and so did the one at 04:42.
- #31's sessions ran for 105 minutes, but the piece took 22 hours. Most of the gap is those
  two waits.

**Did it earn it?**

- **#31 build-2 and build-3:** plausibly. Both hit cases where the brief's numbers failed on
  small maps, and both fixed them by measuring, first time, with no regressions. But #31
  build-4 on Sonnet did the same kind of work for $1.89: it found that the brief's top-up rule
  failed on seed 36, fixed it and added a test. Nothing in #31 or #32 shows Sonnet failing
  where Opus then succeeded. Opus bought insurance, not a result Sonnet had missed.
- **#32 spec:** not shown. It cost 3.6× #31's Sonnet spec ($3.38 against $0.93) for a smaller
  piece. Sonnet spec-review still made six fixes to it, as it did to #31's. These included a
  build step judged too wide and a check.mjs assert that would have failed. #32's builds
  corrected less of their briefs than #31's did. That fits plumbing over fixed shapes as well
  as it fits a better spec.

**What the 5-hour usage showed:** Opus took about 5 points of the 5-hour window per list
dollar (18/3.53 and 18/3.38). Sonnet's heavy steps took about 7 (13/1.89, 15/1.96, 8/1.02,
9/1.30, 9/1.24). So list $ overstates Opus's share of the subscription by about 1.4×. Even
so, each Opus session was still the one that ran into the gate.

### 2. Steps are cut far smaller than any limit, and the one cut made for the limit was lost

- The longest session ran 23 minutes against the 90-minute cap.
- The largest peak context was 192k, 19% of the 1M window.
- Nothing compacted.
- #32 spec-review split build-3 "so one session fits the 90 minutes" (`relay/specs/32/steps.json`).
  The unsplit-sized work then took 8.6 minutes as build-3, plus 10.4 minutes inside the
  review, which wrote build-4 itself (#53). The crew had made the steps from the spec's
  `steps.json` before spec-review changed it. That's a flow bug for another angle. For cost,
  it shows that one session could have done all of it.

Every session pays to load the code again: 100–180k tokens of cache creation in each. #31 ran
eight sessions and #32 six. Fewer, larger build steps would pay that cost fewer times.
Grouping one step per area is reasonable, as #31 did (generator, rivers, closed country, rules,
page). Splitting a step for time or context isn't supported by this evidence.

### 3. Effort wasn't chosen: all 14 steps ran at high

No step tried medium, so these two pieces can't say whether high mattered anywhere.

#32's whys copy rather than choose. Its review's why reads "as #31's review did well on sonnet
high", and build-3's reads "as #31's page step was". The #32 spec summary calls the plan "all
sonnet high after this opus spec".

### 4. Sonnet was never under-powered

- No Sonnet step crashed, timed out, compacted or needed a second session.
- The bugs that got past Sonnet builds were small, and Sonnet reviews caught all of them:
  - a doubled centre event (#46),
  - a draw order (#46),
  - a disband going to a rival civ's clan (#53).
- The #31 spec's three wrong numbers or rules showed up only on running the generator, and
  builds measured and fixed them.

### 5. The spec step's model fitted #31 and cost too much on #32

- **#31, Sonnet high ($0.93):** adequate. Its errors were ones only measurement finds.
- **#32, Opus high ($3.38):** see finding 1. Its `why` names the stakes ("sets the shape every
  player mechanic builds on") but not what Sonnet would have got wrong.

### 6. The spend reporting misreads or drops several figures

- **The board's numbers mostly fail.** `couldn't update the board: Number values cannot exceed
  8 decimal places` or `Column value must be a valid value for number column` appears after 11
  of the 14 steps. The cause is unrounded cost such as `0.9322537999999999` (crew.log of runs
  40100535820, 40189094743, 40211144705 and 40232162428). The step block and `/relay status`
  already carry the same figures.
- **The spec step's `why` stops at the first line break.** `pieceField` reads to the end of the
  line, and both pieces' `Spec:` lines are hard-wrapped. #38 and #47 keep "sonnet high. The
  generator exists in merge4 and the work is porting it, so the" and "opus high. Founding sets
  the shape every player mechanic builds on: what a civ is,".
- **`tokens` is mostly cache reads.** Cache reads are 87–97% of every session's tokens, so
  5,991,919 tokens in a session that peaked at 159k reads as size but isn't. Cost and output
  tokens are the size signals.
- **`context` and `compactions` never moved.** With a 1M window and sessions under 25 minutes,
  `context` (the last turn) equals the peak, and `compactions` was always 0.
- **The usage signal expires.** The 5-hour and weekly percentages are the real subscription
  signal, and they live only in `crew.log` inside 14-day artifacts. #31's first run expires
  around 7 Nov, so a retro that runs more than two weeks after a milestone's first piece can't
  read them.

## Guidance for the next milestone's proposals and specs

- **Default to Sonnet,** for specs and for builds. Pick Opus only when the `why` says what
  Sonnet would get wrong, not how much rides on the step. Neither piece has a case where it did.
- **Try medium.** Run spec-review and plumbing builds at medium, and keep high for steps that
  must find what the brief got wrong (tuning against a sweep, generative maths). That gives the
  next retro a comparison it doesn't have now.
- **Cut steps by area, not by time or context.** A session has room for several times what it
  got here. Prefer three builds to five.
- **Keep `**Spec:**` on one line** in the piece body until the crew reads past a line break.

## Changes the evidence supports

**`relay/prompts/spec.md`:** after "Choose each step's model and effort from its work", add:
"Sonnet is the default. Opus needs a `why` that says what Sonnet would get wrong."

**The crew's spend reporting** (`relay.mjs`), taking out first:
- **Drop the board's number fields:** Attempts, Tokens, List $, Context and Compactions. They
  fail on most steps, and the block and `/relay status` already hold them. If they stay, round
  cost to cents before writing, in the block too.
- **Drop `tokens`** from the step comment and the status table, or replace it with output tokens.
- **Record the 5-hour and weekly usage** before and after each step in its block. The crew
  already probes them, and it's the only spend figure that outlives the artifacts.
- **Read the `Spec:` line to the next field** (or the next blank line), not the end of the line.
