# #338 build-2 (canvas): play the page, find what's worst, fix the top three, say when nothing reaches

Read `relay/specs/338/spec.md` whole, `docs/design/page.md` and `docs/design/founding.md`, and
build-1's summary and `relay/specs/338/live.md`. You may change build-1's driver and readouts
where play shows they fall short; say what you changed.

Split your 90 minutes roughly: 35 playing, 5 writing the faults, 35 fixing and the Standing line,
15 for checks and the summary. Check `date -u` as you go.

## 1. Play the page (before your fixes)

- `engine/build-web.sh`, `cargo build --release -p sansfiat`. Start the driver in the background
  (`node engine/www/play/live.mjs start "?size=small&seed=61288&run=1" --dir /tmp/live` with
  `run_in_background`), and lead the second card (`lead` its name).
- Play 100 turns from the pick as `relay/prompts/play.md` says a player plays: a note before
  each decision, in a scratch file, and the page is all you read: the driver's printouts,
  tooltips and screenshots (read them as images; take one at the pick, at the first of each new
  kind of fault, and every 25 turns). No `game.view()`, no text view, no `why`.
- Play to win: raise, move, claim, gift, signpost, route, as the page lets you find them. When
  nothing seems worth doing, `end` several turns, and write down that nothing was.
- Save to `relay/specs/338/play/small-61288.json`, copy `live.lines.jsonl` beside it as
  `small-61288.lines.jsonl`, and run `engine/target/release/sansfiat judge` on the save into
  `small-61288.judge.txt`. Keep the screenshots you cite in `relay/specs/338/play/` (half size or
  smaller; no more than a dozen).

## 2. What's worst: `relay/specs/338/faults.md`

Each fault worst first, one paragraph: what the player wanted, what the page did or didn't
show, the turn and the screenshot or the lines-file line, and whether its fix needs a rule
change (a new or changed rule, order or number: then it is an idea, not yours). Worst means
what most cost the player a decision that would have moved Standing, or a turn of not knowing
what to do. Faults build-1 already fixed (the camera, the gift line, the words in `spec.md` item 4)
count only if its fix fell short. A fault that repeats an open idea the spec didn't take (no
pillage order, the scout eating while idle, the signpost that draws nobody) is noted, not filed.

## 3. Fix the top three that need no rule change

In the page (`engine/www/`), the text view (`engine/cli/src/view.rs`, or `view_civ.rs` if
build-1 split it) or the page scripts. Each fix has a play script under `www/play/` (in
`all.mjs`), or a `check.mjs`/`check-stage.mjs` case, that fails without it. A new button or
order the player meets is pressed in `www/play/buttons.mjs` with its effect on the world
checked. No `data.rs` number, rule, order kind or `SAVE_VERSION` change. Say in `faults.md`
which three you fixed and how.

## 4. When nothing reaches: the Standing line

Build the line `spec.md` settles (What build-2 leaves to play, the Standing readout): when none
of (a), (b), (c) holds for the led civ, the text view and the civ's panel say in one line that
nothing it can order now brings a clan or a rival's land to it, and name the nearest clan not
its own and the nearest rival tile, each with what would bring it nearer. The predicate is one
core function (in a module that fits: `core/src/claim.rs` is 212 lines, `civs.rs` larger; a new
`core/src/reach.rs` if neither fits) with stage tests for each of (a), (b), (c) holding and for
none holding. The wording and the helper for (c) with no army are yours; record them as
decisions.

Proof on the real map: on your saved game, the line appears on the turns it should (the owner's
small 61288 has an idle stretch with nothing in reach, `relay/retro/m4/game.md`), and on
`relay/specs/337/play/small-2.json` it doesn't appear while an army has a rival capital in
reach. Run your game file through `sansfiat play --game COPY` at a few turns, and the page
through the driver (`load`), and write the turns where it shows in `faults.md`.

## 5. The rest

The faults you didn't fix that the owner would have to decide go in `result.json` as ideas, at
most three, worst first; the others stay in `faults.md`. A fault that breaks a design line and
fits one Ford build is a `fix` with its repro.

## Done

`faults.md` written worst first with the three fixed marked; the played game, its lines and its
judge in `relay/specs/338/play/`; the Standing line built and seen on the real maps;
`engine/check.sh` passes; digest pins and `SAVE_VERSION` unchanged. Your summary gives the judge's
`player orders` line for your game beside the control's (11 in 103, 95 turns with none) and
says what the play step should watch for on small 2.

Size: unknown until play; expect 6 to 10 files, 300 to 500 lines with tests, and the play files.
