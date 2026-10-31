# M2 retro: briefs, spec-review and the review (#54, #29, #55, #28, #56, #57)

Sources: the spec, spec-review and build commits on `refs/pull/{59,68,82,75,89,94}/head`; the
step issues' relay blocks (#58–#98); the `relay-runs-*` transcripts of runs 40691846214,
40781134477, 40825502720, 40857386196 and 40968311410 (turn counts and tool calls below are
measured from them); the piece #28 thread.

## Verdict

- **The shapes fit five pieces of six.** A's clay held for the whole milestone: no kiln
  recorded a clay fix, and B's camp and F's zone are built on `Actor`, `divide` and `Walks`
  with no new machinery. Every canvas held its piece's hard part. C, a Henry Ford, did not
  fit: its one unsettled item was proved on hand-built maps, and the review found the scout
  starves on real ones.
- **Design depth followed effort, not role.** The coin paired an Opus low spec with an Opus
  medium spec-review, or the other way round. Whichever session ran at medium did the design
  work: 18–37 turns, read the code, ran sweeps. At low it skimmed: 6–14 turns, under 3 minutes.
  Spec-review at medium paid for itself every time. At low it changed a few lines once.
- **The new spec-review rule didn't take as written.** No spec-review read the code before
  the spec. All six opened the spec in their first or second call. The medium ones read the
  code afterwards, and read it deeply.
- **The review's plan never reaches the review.** The designer writes in the review's `why`
  what the review must play, but the review prompt carries only the steps table. Reviews
  did what the piece's Decided lines asked and skipped what only the `why` asked.
- **Haiku did every settled step it was given**, plus one it wasn't: C's build-2, scored for
  Sonnet off the ladder, was built by Haiku medium in one session.

## 1. The pieces: settled, shape, parts, cut, and fit

| Piece | Suggested → chosen | Builds (part, model) | Unsettled, and who held it | Fit |
|---|---|---|---|---|
| A #54 | Clay and Kiln → **Clay, Kiln, then a Canvas** | b1 clay opus medium (off ladder up), b2 kiln haiku high, b3 kiln sonnet medium, b4 kiln haiku high, b5 canvas opus low | The actor's Rust (b1); every number, and any rule's form (b5) | Yes |
| B #29 | Easel and Canvas, taken | b1 easel sonnet medium (off up), b2 easel haiku medium, b3 canvas opus low | CIV/UNIT numbers, the AI's post, the reach (b3) | Yes |
| D #55 | Easel and Canvas, taken | b1 easel sonnet medium (off up), b2 easel haiku high, b3 canvas opus low | LEANING.*, the store, maybe a rule's form (b3) | Yes; the canvas was small |
| C #28 | Henry Ford, taken | b1 ford haiku medium, b2 ford sonnet medium (off up; run by Haiku), b3 ford haiku medium | "A few starting numbers checked by tests, not by feel" (b2) | **No** |
| F #56 | Magnum, taken | b1 magnum opus medium (off up) | Guard floor, guarding labour, the AI's zone (b1) | Yes |
| E #57 | Henry Ford, taken | b1 ford haiku high, b2 ford sonnet medium (off up) | "Nothing that matters to the goal" | Yes |

Only A changed the suggested shape. Its reason (spec decision 1): the clay is the core, but
whether the pre-game levels off depends on every rule's numbers together, and each kiln moves
them. So all the tuning went to one canvas at the end. That hybrid isn't in `spec.md`'s menu.
It was M2's best-fitting shape.

Every piece was cut by area: engine, then page, then tuning. Builds ran 6–47 minutes, with
context peaking at 31% (#96). None compacted, and none was cut short.

**Did the hard part sit where the shape put it?**

- **A, clay (#61).** It held. The designer's invariants ("What the clay leaves the kiln",
  spec.md:62) were met. Kiln builds #62–#64 recorded no change to the clay. The clay itself
  designed: it found that "fed" read as `fed_ratio ≥ 1` left clans never growing, and it
  re-read "fed" as "at its harvest target" (#61 decision 1, spec corrected). It used 20 of its
  expected "most of 90" minutes.
- **A, canvas (#65).** It held the hard part. It changed a rule's form (the pre-game ends
  only once people level off) and tuned four numbers its brief didn't list. Its brief allowed
  both.
- **B, canvas (#72).** It held. It reversed spec-review's levy (`clans::surplus` was 0 at
  every capital), added `CIV.pull`, and set carry 3 and the reach (10.5 line cost).
- **D, canvas (#86).** It held the question, but the work was one number: `store_cap` 20 → 60,
  in 7 minutes for $0.58. The easel (#84) had already found that clans join. The canvas scored
  9 with Open 4; the work behind it was nearer Open 2.
- **C, ford.** A ford step ended up designing twice:
  - Build-1 (#77) blocked on a latent founding bug, a capital moved without a spacing check.
    The owner fixed it by redesigning centres ("a centre is a clan", `9a9eb81`, `2cfad7b`,
    with design docs edited).
  - Build-2 made eight decisions the spec hadn't covered: an idle scout walks home, a starving
    scout loses people for good, it walks on its whole labour against Settled 8, and others
    (#78). Spec-review had already re-scored build-2's Open from 0 to 2 (`dd6f666`).
  - The review (#80) then found the unsettled part nobody held: on seeds 1–8, 7 of 9 searched
    routes lost people and 5 starved out (`relay/specs/28/review.md`). The proofs, "a 6-tile
    route within 20 turns, a pass within 40", were on painted maps near the capital.
- **F, magnum (#91).** One Opus medium session took 20 minutes, in one attempt, as its `why`
  expected. The review still had work after merging main (section 3).
- **E, ford.** Settled, and it ran that way. Haiku high took the engine in 47 minutes, the
  longest step in M2. It made ten decisions, mostly about wording and dedupe, and corrected
  one spec item (`CampCutOff` is edge-triggered).

## 2. Spec-review under the new rule

| Piece | Spec | Spec-review | Turns | Min | $ | Read code first? | Ran something? |
|---|---|---|---|---|---|---|---|
| A #54 | opus medium | **opus low** | 6 | 1.0 | 0.34 | No: spec, then the sweep; never opened the briefs it didn't grep | `sweep --size all` |
| B #29 | opus low | **opus medium** | 24 | 4.5 | 1.52 | No: spec first, then about 10 code reads | timing only |
| D #55 | opus medium | **opus low** | 7 | 0.9 | 0.33 | No: spec, then 3 greps | no |
| C #28 | opus low | **opus medium** | 29 | 7.7 | 1.83 | No: spec first, then code and `scout/` | a scratch-worktree sweep with real trail costs |
| F #56 | opus low | **opus medium** | 18 | 3.7 | 1.01 | No: spec first, then all of `units.rs` and paint-army | `sweep --after 200 --maps retro` |
| E #57 | opus medium | **opus low** | 7 | 0.8 | 0.30 | No: spec, then greps | no |

The specs show the same split. At Opus low (B, C, F) they took 9–14 turns, 1.6–2.6 minutes
and $0.45–0.80. At Opus medium (A, D, E) they took 26–37 turns, 5–11 minutes and $1.79–2.80.

**What it caught, by piece:**

- **F (medium): the catch of the milestone.** The spec's first-holder-wins meant an AI's
  2-tile zone covered its whole 3-step line, so no line could ever be cut, and the review's
  required cut couldn't happen. It became a guard-share contest (#90 decision 1). It also made
  the zone `actor.territory`, which kept the clay's shape, limited held land to the line test,
  and made capitals unpaintable. The build and review both stood on these.
- **C (medium).**
  - It measured the pre-game with real trail costs before any build, which dropped the
    fallback cap and turned build-1's open question into a confirm.
  - It found the spec had misread `scout/`'s jobs: with no reveal reachable, the scout must
    build a trail.
  - It added a trail-reset bug fix, the range-widened outward order, feed before search, and
    the camp-only assumptions.
  - It re-scored build-2. This was **the only `steps.json` change in M2**: Open 0 → 2, kept off
    the ladder at Sonnet medium.
  - Missed: the founding bug that blocked build-1. Its sweep doesn't run `cargo test`. It
    also missed the real-map scout starvation, since the proofs it kept were on painted maps.
- **B (medium).** It made five changes the builds used:
  - a camp marches at one person's pace;
  - auto-supply refills the pack;
  - cut-off is judged on the whole labour;
  - the control has no levy;
  - "kept" means no camp lost and at least `UNIT.start` warriors (#71 used that definition).

  One fix was wrong: the levy as "a share of harvest beyond need, before growth". The canvas
  found that surplus is always 0 and moved the levy to the basket (#72 decision 1). That fix
  settled by reading something only running could settle. It's the very thing the prompt
  tells spec-review to look for.
- **A (low).** It ran the sweep on main and gave build-5 a per-size bar, adding medium and
  large. That bar is the one build-5 missed (large seed 3, 29.1 ms against 28), so the
  addition made the miss visible. Nothing else changed.
- **D (low).** It relaxed the easel's join requirement because the store "has at most 5
  spare". Build-1 found joins anyway, because the levy refills the store (#84). It was a false
  alarm, and it did no harm.
- **E (low).** It pinned a fourth digest, a led game, and made the bench a best of three.
  Build-1 and the review both relied on the four digests. It missed spec item 8 (`CampCutOff`
  is edge-triggered) and that `run --turns` stops at the pre-game's end. Build-1 (Haiku) found
  both in the code and fixed them.

**Worth its cost?** At medium, yes: $4.36 over B, C and F, and F's piece would have failed its
own goal without it. At low it cost $0.97 over A, D and E, for one useful bar, one useful
digest and one false alarm.

The pairing is inferred, not measured. The pieces with a low spec (B, C, F) had more
downstream trouble: B's canvas reversed a fix, C blocked and its review said don't merge, and
F's review had to fix code. The pieces with a medium spec (A, D, E) had reviews that changed
nothing. The pieces differ too much to pin that on the spec.

## 3. The review, with no brief

| Piece | Model | Turns | Min | Verdict | Fixed | Judged against the design |
|---|---|---|---|---|---|---|
| A #66 | sonnet high | 22 | 4.2 | merge | nothing | Departures "defensible": the settle rule, retuned numbers, a relaxed unit test. Re-ran the sweep and reproduced build-5. Played to the pick only |
| B #73 | opus low | 9 | 3.0 | merge | nothing | The levy on the basket and `CIV.pull` are "sound against the design". Re-ran `play-camp.mjs` (the piece asked) |
| D #87 | sonnet high | 13 | 3.1 | merge | nothing | Read the diff and looked at one screenshot. Didn't play a led civ raising its gift, which its `why` asked |
| C #80 | sonnet high | ? | 10.2 | **don't merge** | nothing (tried two fixes, no effect) | Departures sound. Measured scouts on real maps against the goal and found they starve. Owner merged after merging main |
| F #92 | opus low | ? | 23.8 | merge | AI zones only when its land-only line still feeds the camp; a missing brace in main's `select.js` that stopped the page loading; play-hold on seed 7 | Judged the AI zone against "the AI zones only as far as it can afford" |
| E #98 | opus low | 4 | 2.7 | merge | nothing | Read part of `world.rs`' diff and ran check.sh. Didn't rerun play-read, look at the screenshots or load an old save, all of which its `why` asked. Accepted three departures without the design open |

Unchecked: #80's and #92's turn counts (their transcripts weren't parsed); their minutes are
from the step blocks.

Two reviews found something. #80's finding came from building its own real-map test against
the piece's goal. #92's came from main moving under it. The other four took 3–4 minutes and
changed nothing.

The reviews' `why` lines list checks (play-lean, play-read, loading an old save) that the
review prompt never shows: it carries the steps table without the `why`. What the review did
tracks what the piece's own text asked. B's and F's pieces name a review play, and those
reviews did it. D's and E's don't, and those reviews skipped it.

## 4. Briefs

- **Left the method to the build (good):** every canvas brief: A b5, B b3, D b3. Each gave
  the deliverable, the proof, what may change and a file for the evidence, and said "Method:
  yours" (A b5). All three changed form or numbers beyond the obvious and recorded it. B b3
  also gave the starting arithmetic (cut off two plains steps out at carry 1), which saves a
  first sweep without fixing the answer.
- **Exact, and right to be (fords and kilns):**
  - A b2 gave every formula; Haiku high did it in 10 minutes for $0.50.
  - E b1 had a "Where things go" list and an invariant proof: the digests unchanged, taken
    before any edit. That proof gave Haiku a check it couldn't fool for a 47-minute step
    across six rule files.
- **Over-specified:**
  - A's briefs ran 63–180 lines. Build-1's 180 lines held Rust shapes the clay was free to
    change, and it changed `trip` to a method and flattened the clan.
  - B's spec-review fixed the levy by reading (section 2).
  - D b2 fixed a screenshot order ("end 30 turns, then shoot a leaning clan") that found no
    leaning clan. They had all joined by then (#85 decision 1).
- **Lacked what the build needed:**
  - C b2's proof stopped at painted maps (section 1).
  - E b1's done check needs `run --turns 400` past the pre-game's end, which the CLI didn't
    do. The build changed the CLI (#96 decision 5).
  - A b5's list of tunables left out `split_at`, `hungry_split`, `scout_range` and
    `surplus_target`. The build tuned them anyway and said so.
- **Rewrote the spec:** A b1 (holding land), A b5 (the end rule), B b3 (Settled 2 and the
  Unsettled table), E b1 (item 8). No build rewrote its brief. C b2 and F b1 departed from
  Settled lines (the scout's pace; F's guard rules) and left `spec.md` unchanged. They
  recorded decisions instead.

## 5. Guidance for designers (a draft for `relay/prompts/spec.md`)

### Paste-ready text

Under **Settle what you can**, after the two kinds:

> Running settles some of it now. A sweep or test that takes minutes is cheaper here than in a
> build: run it, and settle the number or drop the fallback.
>
> Prove on the maps the game is played on. A test on a built map shows a rule works; a run on
> the real maps (the sweep, or a play script) shows the piece does. Every unsettled item's proof
> has one, and so does any step whose rule the player meets.

Under **Choose a shape**, in Clay and Kiln's **Parts**:

> When the kiln's rules are settled but their numbers only work together, end with one
> `canvas` that tunes them all, and give the kiln starting numbers that run.

Under Henry Ford's **When**:

> A port is settled in its rules, not its numbers: the prototype's ran in another economy.
> Prove them on real maps, or give them a canvas.

Under **Choose each step's model and effort**, replacing the score table and ladder:

> Score the step on four things:
>
> | | Low | Middle | High |
> |---|---|---|---|
> | **Open**: can the brief say what done is? | 0: exactly | 2: in part | 4: judged by eye, or by searching for what works |
> | **New**: is there a pattern for it? | 0: in the code, or a port | 1: adapted from one | 3: nothing like it here |
> | **Miss**: what catches a mistake? | 0: the checks | 1: a test it has to write | 3: only play, or a run on real maps |
> | **Reach**: what does it change? | 0: a module on its own | 1: a few callers | 2: what every turn runs |
>
> | Points | 0–1 | 2–3 | 4–5 | 6–7 | 8–10 | 11–12 |
> |---|---|---|---|---|---|---|
> | Step | haiku low | haiku medium | haiku high | sonnet medium | sonnet high; opus low for `canvas` and `clay` | opus medium |
>
> How many files a step touches costs minutes, not a bigger model. Don't go off the ladder
> for it.
>
> Effort is for judging. A build has its checks to push back; spec-review and the review stop
> where their effort does. Neither runs at low.
>
> Don't score the review. It runs Sonnet high. What it must play or measure goes in
> `spec.md`, under what proves the piece: the review reads the spec, not `steps.json`.

In the part rules, "under 9" becomes "under 8".

For `relay/prompts/spec-review.md`, replacing "Read the piece, its design files and the code
it will touch first":

> Read the code each settled rule touches, and run the sweep if it's quick.

Add to its list:

> a proof that runs only on a built map

### Evidence for each change

- **Run it now.** Two spec-reviews ran a sweep in minutes and changed the plan. C's dropped
  a fallback and turned build-1's question into a confirm. A's added the per-size bar. B's
  fixed by reading what a run later disproved.
- **Real-map proofs.** C's painted-map proofs passed and real maps starved the scout (#80,
  `review.md`). Every piece whose proof ran on the retro maps or `--size all` (A, B, D, F) got
  through review on that point.
- **Clay, Kiln, Canvas.** A did it, and both parts held (section 1).
- **A port's numbers.** `scout/` had no food economy. Its numbers met the camp's store and
  starved (#80).
- **Drop Importance.** It was 1 on 12 of 17 builds and 0 on exactly the five page steps,
  which Reach already marks 1. Re-scored without it on the shifted ladder, **every M2 build
  lands on the same model and effort**. So it carried no information. A's clay (Importance
  3) still lands Opus low as a clay at 8.
- **Miss counts real maps.** Miss was 1 on 16 of 17 builds. C b2 scored 1, and only a
  real-map run would have caught its miss.
- **Breadth.** All six off-ladder steps went up, and every reason named breadth ("spans five
  page files", "across most of core", "a trader … across civs, markets, clans, world, the CLI
  and cross-build"). Haiku did two of the broadest steps in one session each: E b1, six rule
  files, 47 minutes; C b2, the scout across `units.rs`, `world.rs`, the CLI and cross-build,
  25 minutes. The `why` for C b2 had said Haiku "would likely miss a camp-only assumption and
  loop". Haiku fixed those assumptions itself (#77 decisions 6–8).
- **Effort for judging.** At low, spec-reviews and reviews took 4–9 turns. At medium or high
  they took 13–29. The only low session that went deep was #92, which was forced by a merge.
  Builds at Opus low (three canvases) all did their job, because the checks pull a build
  along and nothing pulls a reviewer.
- **The review.** All six designers scored it Open 2, New 1, Miss 3, Importance 1, Reach 2 =
  9, so the score decides nothing but the coin. The two reviews that found something were
  Sonnet high (#80) and an Opus low forced by a merge (#92). Their `why` checks went
  undelivered (section 3).
- **Spec-review's order.** No session followed "before the spec". Targeting the reading at
  the settled rules is what the medium sessions did anyway.

### Shapes and Ladders: keep, change, cut, add

- **Keep** the four shapes and the part rules: five of six fit, and the clay and every canvas
  held their part.
- **Add** Clay, Kiln, Canvas as a named arrangement (above). It needs no new shape.
- **Change** Henry Ford's When (ports) and Miss (real maps).
- **Cut** Importance, and shift the ladder down one (above).
- **Cut** scoring the review.
- **Keep** the off-ladder rule. For M3's challenge, the evidence asks for one step a piece
  *down* the ladder: every M2 off-ladder step went up, and the one accidental step down (C b2
  on Haiku) worked.
- **The design pair** (for the coin's replacement): one Opus medium pass does the design
  wherever it sits. Run the spec at Opus medium, and spec-review at medium or not at all. Its
  low-effort form isn't worth a session.

## Ideas

| Idea | Value | Cost | Rec |
|---|---|---|---|
| Real-map proof for every unsettled item and every rule the player meets (spec.md text above) | high | small | take |
| The review's must-plays go in spec.md, not its `why` (or the crew passes the review its `why`) | high | small | take |
| Judging sessions (spec-review, review) never at low effort | high | small | take |
| Spec at Opus medium; spec-review at medium or skipped, replacing the coin | high | small | take |
| Drop Importance and shift the ladder down one (same models on all of M2) | medium | small | take |
| Miss 3 includes "a run on real maps" | medium | small | take |
| Clay, Kiln, Canvas named in spec.md | medium | small | take |
| "Run it now" when a sweep takes minutes (spec.md and spec-review.md) | medium | small | take |
| Henry Ford: a port's numbers aren't settled | medium | small | take |
| Spec-review reads the code each settled rule touches, instead of "before the spec" | medium | small | take |
| Breadth isn't a reason to go off the ladder | medium | small | take |
| Review fixed at Sonnet high, no score | medium | small | try |
| M3 challenge: one step a piece off the ladder *down* | medium | small | try |
| Haiku high for builds up to 7 points (C b2 at 6 went fine on Haiku medium) | medium | small | try |
| Spec-review on Sonnet high instead of Opus | low | small | try |
| Cap brief length; leave Rust shapes to the clay (A b1 was 180 lines and changed them anyway) | low | small | later |
| Merge D-like small canvases into the easel's last step | low | small | drop (the canvas cost $0.58; the separation kept the easel from tuning) |

## For crew

- The review prompt carries the steps table without the `why`. The designer's review plan is
  lost (#98 skipped play-read, the screenshots and an old save).
- After the owner's reply on #28 ("Carry on with build-2 from the branch"), the build-1
  session on Haiku medium built build-2 (run 40857386196, 105 turns). Build-2's Sonnet session
  then found the work done and spent $0.07. The block records build-2's work under build-1's
  id and model.
- #28's build-1 ran `git add -f relay/runs/result.json`. The file rode on the branch into #57
  spec-review's commit `87cfd6c`, which overwrote #28's review result.
- #92 found a missing brace in main's `select.js` `scoutPanel` that stopped the page loading.
  It came in with #75's merge, so main was briefly broken.

## For cost

- The off-ladder steps all went up. C b2's Sonnet step was done by Haiku medium for $1.35 over
  25 minutes. Spec-review at low cost $0.30–0.34 and took about a minute; at medium,
  $1.01–1.83.

## For prompts

- spec-review.md's "first" was ignored by all six sessions.
- The owner's reply wording "carry on with build-2" was read as a step change.
