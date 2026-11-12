# M4 retro: the owner's notes

As they came, on 12 November. They close the retro's wait (#311). The retro is finished in
local sessions, not by Relay: the `game`, `retro` and `proposal` steps read this file.

## Where each note stands

| Note | Stands |
|---|---|
| M5 runs pieces in the new shape; the twin and pick go; the log per milestone; draft PRs; above and below the line; fewer ideas, and designers take the obvious ones; trust and verify; a test can't run too long | Built in this PR's Relay change (`docs/proposals/m5-relay.md`) |
| Remove the militia and the guard | M5 piece L, on the retro's question 1: what holds land without a guard |
| Scouts go back to the capital for food; warriors kill scouts, and a warrior's painted tiles trap them | M5 piece C. The stalled errand is reproduced in `game/scout-61288.txt` |
| Play the page, name the worst aspects, propose fixes; a click centres the camera on the tile | M5 piece P. The retro played the text view only |
| The scout's test on a small known map, viewable and runnable on the page, and every such test | M5 piece V |
| Towns: one town on small 61288 | M5 piece N. As built, small maps form one town or none (`game.md` §1) |
| Strike implementation details from the design files; the ledger holds decisions of an ADR's weight | Done in this PR: 104 of 149 ledger lines struck, and `docs/design/README.md` says what the ledger is. The prose is untouched |
| Record the starving decision and its reasoning | `docs/design/vision.md`, Decided, last line; `docs/process.md`, Scores, line 5. What else isn't helping: `docs/proposals/m4-retro.md` |
| The scout issued its own order at the pick | Not reproduced in the text view |

## The notes

I want remaining steps of M4 to run with Fable.

M5 should run pieces in new shape.
1. Relay check for a full 5hour window
2. Fable starts the whole piece, planning and building
3. At the start, and Peridoically via some hook or other, the usage is inserted into the session
4. The Fable session can issue other build steps to follow the finish the work. e.g. the sonnet player and review. So its a bit like Clay and Kiln but the spec steps have been bundled into Clay as well.

Remove the milita and the guard - should be an easy way get more variation in the game. You can probably test that now actually...

Relay log should be per milestone
- So I dont have to scroll so far to see the current status.
- Updates should should link to sub-issue
- Add the weekly limit to the comment as well

Play the game on the page, identify the worst aspects and propose fixes

when clicking a tile, the camera should center on the tile, not where you clicked. currently cycling through units on the same tile causes the view to jump around

Scouts should go back to the capital to get food. forraging doesn't work, my scout was not able to finish its first errand.

If there is a dedicated separate test for the scout, on a smaller known map with a test. I asked for that at somepoint. If that exists it needs to be viewable and runable on the page. This goes for all such tests.

Warrors should be able to kill scouts, and scouts should be trapped by a warrors painted tiles.

I played small seed 61288 and only saw 1 town - they are not working correctly. The others were too far to bother sending a warror over so I just ran the game no-ops to the end. So still hardly a game. No matter, keep building.

Remove the twin and pick experiment, the experiment in M5 is using Fabel, and trying to maximis the value of free resets

Strike down implimentation details from Design documenation, the current state being 'as built' is good enough. I don't care if a later step rewrites a previous with new behavour, it's probably correct to change it. I would rather only pin implimentation details after after churn has been identified, not before.

The decisions being promoted to Design docs is an Architecure Decision Record. So should only record decisions of that magnitued and importance.

Above the line and below the line communications. Text that reaches GH issues and comments should be high quality and intended to be read by a human. Otherwise it should just be in files.

Why so many proposed ideas? Many of them seem to be implimentation details or builders notes. Or things that could be decided by planners or specifiers.

Whatever happened to the Ideas from previous milestones?

Specifiers (designers) should include the obvious-to-do ideas from the previous piece into their own.

Trust, and Verify - It is pointless to re-run a test that has just been ran by another session. Trust that sessions aren't lieing (they aren't) and verify via a different axises. A fresh context with a re-read of the goals or a different test.

First time hitting "deadline: no time for another step in this job". Is the testing excessive? There is no way that agents were doing meaningful thinking and dev work and ran into the 250 minute deadline.

Its possible the game has wandered into a pit of failure. The precedenct has become to run the game generate numbers, compare numbers with other numbers (tests), but all the while those numbers are totally meaningless. Tests noone asked for are driving and deciding factors.
I am cautious of this starving tests, its not something that needs to tested and adjusted for, its a consiquence of growth and not something to min/max tune against. A game where 'everyone is fed and happy' is not this game, and it would hardly be a game. This is a simulation and meaningful consiquences are the desired outcome, having more more people 'fed' is not.
- the above lead to changes mid #156, record the decision and reasoning somewhere.
- I delt with the starving tuning by removing the bands, but what else is going on that's not helpful?

How long does it take to run the tests? How long is the cache timeout when running relay? Can we set it to run on the 5min timeout for an easy win? Bake in a rule somewhere that a test cannot run on too long.

This was an early play tests, but my scout issued its own order at the pick. Or, there was AI running in the background that took over before I got in control.
- I didn't see that in my second play through so maybe fixed or non-issue

PR should be created as a draft till ready.

## Measured for the notes

- `engine/check.sh` took 12m24s on CI (main, 11 Nov). `cli/tests/judge.rs`
  ran the whole seven-map sweep to check JSON keys: 3m44s of it. With that test a report,
  `check.sh` takes 9m22s (CI on #335). The page scripts are 2m48s and the web builds 1m20s.
- "deadline: no time for another step in this job" is the job's 250-minute limit on starting a
  step, not a session running out. M4 had four, each with the next run already queued
  (`relay.md`, Failures).
- Sessions run on the 1-hour prompt cache. The CLI has `FORCE_PROMPT_CACHING_5M`; it is off,
  since a wait over five minutes rewrites the whole context and `check.sh` is still over that.
- Ideas: 85 asks against 6 fixes, 85 `relay:proposed` issues open, none promoted (`relay.md` §3).
