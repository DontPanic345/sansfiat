# M1 retro: briefs and spec-review (#31, #32)

Sources: the spec commits (`a6d802a`, `2b41add`), the spec-review commits (`36c3011`,
`f196817`), the build and review commits on `refs/pull/39/head` and `refs/pull/48/head`, the
step issues' relay blocks (#38–#46, #47–#53), and the crew logs of runs 40100535820,
40131547026, 40189094743 and 40211144705.

## Verdict

- **#31: spec-review earned its place.** It cost $0.90 (6 min, 5.5% of the piece). Five of
  its fixes changed what happened: the builds used them, and three of them fired in the tests
  (findings 4–6).
- **#32: spec-review cost more than it saved.** Its real catches were the click cycle,
  `units::turns` and `cross-build.mjs` reading `sim.state()`. Against that, it caused a
  build step to vanish (finding 1). Most of its other edits were things the compiler or
  `check.mjs` would have told the build in seconds.
- Every miss on both pieces showed up only when the built code ran: a sweep seed, a
  flickering set, a neck radius that split the mainland. No one reading the briefs could
  have caught them. The builds fixed each one and corrected `spec.md` in their commits, as
  `build.md` asks. Nothing in the spec or spec-review prompts would have prevented them, so
  don't add anything for them.

## Findings, most severe first

### 1. Spec-review's edits to `steps.json` are never read. On #32 that dropped a whole build step (caused)

Spec-review split #32's page step into build-3 (logic) and build-4 (look). It added
`build-4.md`, rewrote `build-3.md`, and updated `steps.json`, `spec.md` and `review.md`
(`f196817`). But the crew makes every step from `steps.json` as soon as the spec step closes
(`plan()`, `relay.mjs:753`). The run 40211144705 crew log shows the steps were made at
16:30:44–16:31:04 and spec-review started at 16:31:11. So build-4 (#54) never existed. Build-3
(#52) still carries the spec's original title, "The page: pick, top bar, selection panel and
orders", and the piece went build-3 → review.

The review noticed only because its brief now said "as build-3 and build-4 did". It wrote
build-4 itself (`a7e451d`; decision: "build-4 had no commit"). So nobody reviewed the drawing,
feed kinds, tooltip or legend: the session that built them is the one that judged them.

Spec-review's `why` edits are dead too: #31's update to build-1's `why` (`36c3011`) and #32's
spec-review `why` never reached the steps. `spec-review.md` invites this with "or a model or
effort whose `why` doesn't hold". That is a fix it has no way to make.

**Take out:** the model/effort clause from `spec-review.md`, and tell spec-review that
`steps.json` is already in effect, so it fixes briefs and leaves the steps alone. The other
fix, making the crew plan after spec-review, would keep a power that finding 2 shows
spec-review used badly.

### 2. The split rested on a session-length fear the data contradicts (caused)

Spec-review's reason for the split: "One session is cut at 90 minutes and two unfinished
attempts block a step" (#49 decision). Measured from the transcripts' result frames, the
sessions on these pieces ran 4–23 minutes:

| Step | Time | Context |
|---|---|---|
| #31 build-3 (opus) | 23 min | 19% |
| #31 build-4 | 18 min | 18% |
| #31 build-5, the whole page | 14 min | 18% |
| #32 build-3, half the page | 9 min | 14% |

No session compacted. #31's build-5 did a wider page step than #32's unsplit build-3 in 14
minutes. The step blocks already record context and compactions, and they would have shown
this.

**What would have prevented it:** nothing new. Without the model/effort clause (finding 1),
the reworking of `steps.json` has no foothold in `spec-review.md`. If the spec splits steps,
it can read the last piece's context shares.

### 3. Splitting a brief put the same work in two briefs (caused)

After the split, `build-3.md` still says "the tooltip adds the tile's civ" (line 24) and
"Move here: cost 7, 2 turns" (lines 110–111), and `build-4.md` asks for both again
(`f196817`). Build-3 built the "Move here" line and left the civ line. Small, but it is the
kind of overlap a brief written in two passes produces. Finding 1's fix takes away the cause.

### 4. #31: the open-land guarantee had no way out when the base land failed (caught)

The spec's `build-3.md` said "the base land alone must satisfy it" with no fallback.
Spec-review added a whole-map re-roll, plus building the guarantee only at radius 16 and up
(`36c3011`, #40 decision). It fired: 5 of 40 small seeds needed a whole-map re-roll (#43
FALLBACKS). Without it, build-3 would have hit maps it couldn't make.

### 5. #31: a per-map closed-country floor couldn't pass on small maps (caught)

The spec asked for "at least `scaled(valleys)` valleys" on each map. Spec-review pooled it
over 15 seeds at 0.7 of the target and allowed maps that lack a kind (`36c3011`). Small came
in at 25/30 valleys and 24/30 necks against a floor of 21 (#43 COUNTS), so a per-map test
would have failed. Of all spec-review's checks, the "numbers that can't pass" one paid off
most.

### 6. #31: briefs blind to the test harness and test speed (caught)

- `flat()` builds `World::new` at radius 6 in hundreds of tests. Spec-review told build-1 to
  generate at any radius and to pin `rng` and `next_id` to the old values. Build-1: "no
  existing test needed a change" (#41).
- `cargo test` was unoptimised. Spec-review measured a 26 s small run and had build-1 set
  `opt-level = 3`. `cargo test` went from 1m50s to about 10 s (#41). Build-4's whole
  pre-games on medium and large still took 85 s with it (#44).
- Raised markets with activity 0 would have faded. Build-4 factored `markets::raise` from
  `form` as the fix asked (#44).

These came from checking the briefs against the code and running it, which is the job
`spec-review.md` describes.

### 7. #32: a design line turned into a rule that says something else (missed)

`founding.md` sends a disbanded unit's person to "the nearest civ clan". The spec and
`build-2.md` made that "the civ clan whose home is fewest hex steps from the unit's tile",
meaning any civ's clan. Spec-review's own `why` says it checks "that the decisions match
founding.md", and it missed this. The review fixed it (`a7e451d`, #53 decision 1). The review
is the backstop that works here, so don't add a prompt line for it.

### 8. #31: rules that broke only when run (missed, not catchable by reading)

Each of these was found by a build running the sweep or the tests. Each was fixed in the
build's commit with `spec.md` corrected:

- Build-2: hills-only river sources gave small maps no 5-tile river. Sources widened, with an
  adaptive `river_min`. `delta_river` went from 12 to 10. A source beside a river is skipped
  to stop braiding (#42, spec decisions 7–8). Spec-review had seen the symptom (little deep
  land on small maps) and gave leave to tune. The lever turned out to be the sources, not
  `river_min`.
- Build-3: necks of radius 3 split the mainland. Kinds placed one after another left necks no
  room. A neck cut could take a map's last lake. And without a site estimate per feature,
  small maps lost all their closed country (#43 decisions 1–3).
- Build-4: "the most populated clan's home region" left small seed 36 with two centres
  (#44 decision 1). Spec-review copied this rule from the brief into spec decision 13. "The
  previous turn's set" made "becomes a centre" repeat (#44 decision 2). Even then, the
  top-up path announced twice, which the review fixed (#46).

### 9. #31: build-5 was told to edit `ci.yml`, which the runner can't push (missed)

The GitHub App has no workflows permission. Build-5 committed the edit, the push was refused,
and it reverted (`a589cb3`). The review then corrected `engine/CLAUDE.md`'s claim (#46). This
is a fact about the runner, not about a brief, and #32's spec already carried it forward.
If it's worth a line anywhere, it goes under "The runner" in `session.md`, not in
spec-review.

### 10. #32: most of spec-review's edits were things the checks would have reported (noise)

Registering `pub mod civs;`, the derives, the `Event` literal in `events.rs`, and `civFounded`
in `check.mjs`'s `KINDS` would each fail compilation or `check.mjs` on the build's first
run. On #31, call-site line numbers (`markets.rs:35`, `round.rs:93, 124`) were noise of the
same kind: build-2 found every call site anyway (#42). These edits are cheap, but they make
up most of #32's diff and they bury the three fixes that mattered.

**Swap, don't add:** `spec-review.md`'s example "a brief missing what its build needs" could
become "a brief missing what its build needs and the checks won't tell it".

## Proposed prompt edits

`relay/prompts/spec-review.md`, all cuts:

```diff
 Read the spec, the briefs and `steps.json` the way the sessions after you will: each brief
 with only the code beside it. Fix whatever would trip them up, such as a brief missing what
-its build needs, a test whose numbers can't pass on the current map, or a model or effort whose `why` doesn't hold.
+its build needs and the checks won't tell it, or a test whose numbers can't pass on the current map.
+The steps are already made from `steps.json`, so leave it as it is.
```

The first sentence's "and `steps.json`" could go as well.

`relay/prompts/spec.md`: no change. The spec's misses were either caught by spec-review (#31)
or showed up only when the code ran.

`relay.mjs`: no change. Planning after spec-review is the other way to fix finding 1. It keeps
more machinery and a power spec-review misused.
