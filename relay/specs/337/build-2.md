# build-2: the retro maps

Read `relay/specs/337/progress.md`: the rule, and what was measured on small 61288. This step
tunes `SCOUT.pack`, `SCOUT.spare` and `SCOUT.out` (`engine/core/src/data.rs`) on the retro maps and
writes the piece's evidence. The rules are settled; only those three numbers move, and the note
beside `SCOUT` says what settled them.

## 1. Errands: `relay/specs/337/errand.md`

`core/tests/scout_errand.rs` reports errands on the seven retro maps (build-1 keeps it running).
Report, on `main` (`git worktree add /tmp/main origin/main`) and on the branch: of the routed
errands, how many have their route built by +50, +100 and +200, turns off the errand, scouts
starved and scouts killed. Routes from each civ's capital to each rival capital, fight on and off.

Tune on that: the smallest pack that finishes the most routes, a spare that never leaves a scout
hungry on the walk home, and an `out` that doesn't keep a scout waiting at a poor store for most of
the game. Say what each setting you tried gave. If the store can't carry the pack (the scout waits
at `Filling` for tens of turns, or armies go unraised because the scout took the store), say so
with numbers: that is the piece's main risk.

## 2. The question: `relay/specs/337/scores.md`

- `sansfiat sweep --judge --json` on the branch (`branch-sweep.json`), and
  `sansfiat judge --compare` against main's (`relay/specs/354/branch-sweep.json` is main at
  4fea294: the engine has not changed since; copy it to `main-sweep.json`). Put the compare in
  `compare.txt` and each score's line in `scores.md`. Every AI civ's scout now eats from its store
  instead of foraging: name what that moved (stores, armies raised, Standing).
- The led game: small 61288, Red, one route order `{"type":"route","unit":232,"from":735,"to":109}`,
  through `sansfiat play`, with `--no-fight` (the control the retro had) and with fight on, where
  the script also raises an army and keeps it on the capital as long as it can. On main and on the
  branch: the turn the route is built, or where the scout stands at +200. Then, on the branch, with
  the route built: order an army along it (`move`) and post a signpost past the range, and report
  whether the army crosses and a clan settles by +200. Save the lines files under
  `relay/specs/337/game/`.

## 3. Open ideas

If a clan never takes a pass on the retro maps because `SETTLE.reach` or the draw's walk is too
short, say so in `scores.md` with the numbers; don't change `SETTLE`.

## Done when

`engine/check.sh` passes if you changed a number (the pins move with it), and `errand.md`,
`scores.md`, `compare.txt` and the game files are committed.
